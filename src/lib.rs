//! # Paradex Rust SDK
//!
//! Rust implementation of the Paradex SDK for interacting with the Paradex decentralized derivatives exchange.
//!
//! ## Features
//!
//! - Complete REST API client with all endpoints
//! - WebSocket client for real-time data streams
//! - Account management and key derivation
//! - Order creation, signing, and submission
//! - Authentication and JWT token management
//! - Full Starknet integration
//!
//! ## Example
//!
//! ```rust,no_run
//! use paradex_rs::{Paradex, Environment};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize Paradex client
//!     let paradex = Paradex::new(Environment::Testnet)?;
//!
//!     // Fetch system configuration
//!     let config = paradex.api_client().fetch_system_config().await?;
//!     println!("System config: {:?}", config);
//!
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod account;
pub mod constants;
pub mod environment;
pub mod error;
pub mod message;
pub mod types;
pub mod utils;

pub use environment::Environment;
pub use error::{ParadexError, Result};
pub use types::*;

use account::ParadexAccount;
use api::{authenticate, needs_refresh, onboard, ApiClient, WebSocketClient};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

/// Main Paradex client for interacting with the Paradex API
///
/// This is the primary entry point for using the Paradex SDK.
pub struct Paradex {
    env: Environment,
    api_client: Arc<Mutex<ApiClient>>,
    ws_client: Arc<Mutex<WebSocketClient>>,
    account: Option<Arc<Mutex<ParadexAccount>>>,
    config: Option<SystemConfig>,
    auth_timestamp: Arc<Mutex<Option<SystemTime>>>,
}

impl Paradex {
    /// Create a new Paradex client without authentication
    ///
    /// # Arguments
    ///
    /// * `env` - Environment (Prod or Testnet)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use paradex_rs::{Paradex, Environment};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let paradex = Paradex::new(Environment::Testnet)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new(env: Environment) -> Result<Self> {
        let api_client = Arc::new(Mutex::new(ApiClient::new(env)?));
        let ws_client = Arc::new(Mutex::new(WebSocketClient::new(env)));

        Ok(Self {
            env,
            api_client,
            ws_client,
            account: None,
            config: None,
            auth_timestamp: Arc::new(Mutex::new(None)),
        })
    }

    /// Initialize Paradex client with L1 credentials (derives L2 key)
    ///
    /// # Arguments
    ///
    /// * `env` - Environment (Prod or Testnet)
    /// * `l1_address` - Ethereum address
    /// * `l1_private_key` - Ethereum private key
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use paradex_rs::{Paradex, Environment};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let paradex = Paradex::with_l1_credentials(
    ///         Environment::Testnet,
    ///         "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
    ///         "0x1234..."
    ///     ).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn with_l1_credentials(
        env: Environment,
        l1_address: impl Into<String>,
        l1_private_key: impl Into<String>,
    ) -> Result<Self> {
        let mut paradex = Self::new(env)?;

        // Fetch system config first
        let config = paradex.fetch_and_store_config().await?;

        // Create account from L1 credentials
        let account = ParadexAccount::from_l1_private_key(&config, l1_address, l1_private_key)?;
        paradex.account = Some(Arc::new(Mutex::new(account)));

        // Perform authentication flow
        paradex.authenticate().await?;

        Ok(paradex)
    }

    /// Initialize with L2 credentials directly
    ///
    /// # Arguments
    ///
    /// * `env` - Environment
    /// * `l1_address` - Ethereum address
    /// * `l2_private_key` - L2 (Starknet) private key as hex string
    pub async fn with_l2_credentials(
        env: Environment,
        l1_address: impl Into<String>,
        l2_private_key: impl Into<String>,
    ) -> Result<Self> {
        let mut paradex = Self::new(env)?;

        // Fetch system config first
        let config = paradex.fetch_and_store_config().await?;

        // Parse L2 private key
        let l2_key = starknet_crypto::FieldElement::from_hex_be(&l2_private_key.into())
            .map_err(|e| ParadexError::ConfigError(format!("Invalid L2 key: {}", e)))?;

        // Create account from L2 credentials
        let account = ParadexAccount::from_l2_private_key(&config, l1_address, l2_key)?;
        paradex.account = Some(Arc::new(Mutex::new(account)));

        // Perform authentication flow
        paradex.authenticate().await?;

        Ok(paradex)
    }

    /// Get the environment this client is using
    pub fn environment(&self) -> Environment {
        self.env
    }

    /// Get a reference to the API client (for public endpoints)
    pub fn api_client(&self) -> Arc<Mutex<ApiClient>> {
        Arc::clone(&self.api_client)
    }

    /// Get a reference to the WebSocket client
    pub fn ws_client(&self) -> Arc<Mutex<WebSocketClient>> {
        Arc::clone(&self.ws_client)
    }

    /// Get account reference
    pub fn account(&self) -> Option<Arc<Mutex<ParadexAccount>>> {
        self.account.as_ref().map(Arc::clone)
    }

    /// Fetch and store system configuration
    async fn fetch_and_store_config(&mut self) -> Result<SystemConfig> {
        let config = self.api_client.lock().unwrap().fetch_system_config().await?;
        self.config = Some(config.clone());
        Ok(config)
    }

    /// Perform onboarding and authentication
    async fn authenticate(&self) -> Result<()> {
        let account = self.account.as_ref()
            .ok_or_else(|| ParadexError::AuthError("No account initialized".to_string()))?;

        // Step 1: Onboarding (may fail if already onboarded, that's ok)
        let _ = self.onboard().await;

        // Step 2: Authentication to get JWT
        self.auth().await?;

        Ok(())
    }

    /// Perform onboarding
    async fn onboard(&self) -> Result<()> {
        let account = self.account.as_ref()
            .ok_or_else(|| ParadexError::AuthError("No account initialized".to_string()))?;

        let account_guard = account.lock().unwrap();
        let headers = account_guard.onboarding_headers()?;
        let public_key_hex = account_guard.l2_public_key_hex();
        drop(account_guard);

        // Get HTTP client
        let client = {
            let api_client = self.api_client.lock().unwrap();
            api_client.get_http_client()
        };

        let api_url = self.env.api_url();

        // Call onboarding API
        onboard(&client, &api_url, headers, &public_key_hex).await?;
        log::info!("Onboarding successful for: {}", public_key_hex);

        Ok(())
    }

    /// Authenticate to get JWT token
    async fn auth(&self) -> Result<()> {
        let account = self.account.as_ref()
            .ok_or_else(|| ParadexError::AuthError("No account initialized".to_string()))?;

        let account_guard = account.lock().unwrap();
        let headers = account_guard.auth_headers()?;
        let public_key_hex = account_guard.l2_public_key_hex();
        drop(account_guard);

        // Get HTTP client
        let client = {
            let api_client = self.api_client.lock().unwrap();
            api_client.get_http_client()
        };

        let api_url = self.env.api_url();

        // Call auth API and get JWT
        let jwt_token = authenticate(&client, &api_url, headers, &public_key_hex).await?;
        log::info!("Authentication successful for: {}", public_key_hex);

        // Store JWT in account
        let mut account_guard = account.lock().unwrap();
        account_guard.set_jwt_token(&jwt_token);
        drop(account_guard);

        // Store JWT in API client
        let mut api_client = self.api_client.lock().unwrap();
        api_client.set_token(&jwt_token);

        // Update auth timestamp
        *self.auth_timestamp.lock().unwrap() = Some(SystemTime::now());

        Ok(())
    }

    /// Refresh JWT token if needed
    pub async fn refresh_auth_if_needed(&self) -> Result<()> {
        if self.account.is_none() {
            return Ok(());
        }

        let auth_time = *self.auth_timestamp.lock().unwrap();

        if let Some(timestamp) = auth_time {
            if needs_refresh(timestamp) {
                log::info!("JWT token expired, refreshing...");
                self.auth().await?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_paradex_client() {
        let paradex = Paradex::new(Environment::Testnet);
        assert!(paradex.is_ok());
    }

    #[test]
    fn test_environment() {
        let paradex = Paradex::new(Environment::Testnet).unwrap();
        assert_eq!(paradex.environment(), Environment::Testnet);
    }
}
