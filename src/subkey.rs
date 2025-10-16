//! Paradex Subkey implementation
//!
//! Provides L2-only authentication using subkeys without requiring L1 credentials.

use crate::{
    api::{ApiClient, WebSocketClient},
    environment::Environment,
    error::Result,
    types::SystemConfig,
};
use starknet_crypto::get_public_key;
use starknet_types_core::felt::Felt;
use std::sync::{Arc, Mutex};

/// Subkey account (L2-only, no L1 derivation)
pub struct SubkeyAccount {
    pub l2_address: String,
    pub l2_public_key: Felt,
    l2_private_key: Felt,
    pub jwt_token: Option<String>,
}

impl SubkeyAccount {
    /// Create a new subkey account
    pub fn new(l2_private_key: &str, l2_address: &str) -> Result<Self> {
        let private_key = Felt::from_hex(l2_private_key).map_err(|e| {
            crate::error::ParadexError::ConfigError(format!("Invalid L2 key: {}", e))
        })?;

        let public_key = get_public_key(&private_key);

        Ok(Self {
            l2_address: l2_address.to_string(),
            l2_public_key: public_key,
            l2_private_key: private_key,
            jwt_token: None,
        })
    }

    /// Set JWT token
    pub fn set_jwt_token(&mut self, token: impl Into<String>) {
        self.jwt_token = Some(token.into());
    }

    /// Get JWT token
    pub fn get_jwt_token(&self) -> Option<&str> {
        self.jwt_token.as_deref()
    }

    /// Sign a message hash
    pub fn sign_hash(&self, hash: Felt) -> Result<(Felt, Felt)> {
        let signature = starknet_crypto::sign(&self.l2_private_key, &hash, &self.l2_public_key)
            .map_err(|e| {
                crate::error::ParadexError::SigningError(format!("Signing failed: {}", e))
            })?;

        Ok((signature.r, signature.s))
    }
}

/// ParadexSubkey client for L2-only authentication
///
/// This client allows trading without L1 credentials by using a subkey account.
pub struct ParadexSubkey {
    env: Environment,
    api_client: Arc<Mutex<ApiClient>>,
    ws_client: Arc<Mutex<WebSocketClient>>,
    account: Arc<Mutex<SubkeyAccount>>,
    config: SystemConfig,
}

impl ParadexSubkey {
    /// Create a new ParadexSubkey client
    ///
    /// # Arguments
    ///
    /// * `env` - Environment (Prod or Testnet)
    /// * `l2_private_key` - L2 (Starknet) private key
    /// * `l2_address` - L2 account address
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use paradex_rs::{ParadexSubkey, Environment};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let paradex = ParadexSubkey::new(
    ///         Environment::Testnet,
    ///         "0x1234...",  // L2 private key
    ///         "0x5678..."   // L2 address
    ///     ).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new(
        env: Environment,
        l2_private_key: impl Into<String>,
        l2_address: impl Into<String>,
    ) -> Result<Self> {
        let api_client = Arc::new(Mutex::new(ApiClient::new(env)?));
        let ws_client = Arc::new(Mutex::new(WebSocketClient::new(env)));

        // Fetch system config
        let config = api_client.lock().unwrap().fetch_system_config().await?;

        // Create subkey account
        let account = SubkeyAccount::new(&l2_private_key.into(), &l2_address.into())?;

        let subkey = Self {
            env,
            api_client,
            ws_client,
            account: Arc::new(Mutex::new(account)),
            config,
        };

        // Authenticate
        subkey.auth().await?;

        Ok(subkey)
    }

    /// Get the environment
    pub fn environment(&self) -> Environment {
        self.env
    }

    /// Get API client reference
    pub fn api_client(&self) -> Arc<Mutex<ApiClient>> {
        Arc::clone(&self.api_client)
    }

    /// Get WebSocket client reference
    pub fn ws_client(&self) -> Arc<Mutex<WebSocketClient>> {
        Arc::clone(&self.ws_client)
    }

    /// Get account reference
    pub fn account(&self) -> Arc<Mutex<SubkeyAccount>> {
        Arc::clone(&self.account)
    }

    /// Authenticate to get JWT token
    async fn auth(&self) -> Result<()> {
        // TODO: Implement auth for subkey
        // For now, log that subkey auth is ready
        log::info!("Subkey authentication ready");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subkey_account_creation() {
        let private_key = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        let address = "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb";

        let account = SubkeyAccount::new(private_key, address);
        assert!(account.is_ok());
    }
}
