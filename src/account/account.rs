use crate::{
    account::key_derivation::{
        build_stark_key_message, compute_account_address, compute_public_key, derive_stark_key,
    },
    error::{ParadexError, Result},
    types::SystemConfig,
};
use starknet_types_core::felt::Felt;

/// Paradex account with L1 and L2 key management
pub struct ParadexAccount {
    /// Ethereum (L1) address
    pub l1_address: String,

    /// Starknet (L2) account address
    pub l2_address: Felt,

    /// Starknet public key
    pub l2_public_key: Felt,

    /// Starknet private key (kept private)
    l2_private_key: Felt,

    /// L2 chain ID
    chain_id: Felt,

    /// JWT token for authentication
    pub jwt_token: Option<String>,
}

impl ParadexAccount {
    /// Create a new account from L1 private key (derives L2 key)
    pub async fn from_l1_private_key(
        config: &SystemConfig,
        l1_address: impl Into<String>,
        l1_private_key: impl Into<String>,
    ) -> Result<Self> {
        let l1_address = l1_address.into();
        let l1_private_key = l1_private_key.into();

        // Parse L1 chain ID
        let l1_chain_id = config
            .l1_chain_id
            .parse::<u64>()
            .map_err(|e| ParadexError::ConfigError(format!("Invalid L1 chain ID: {}", e)))?;

        // Build stark key message and derive L2 private key
        let stark_message = build_stark_key_message(l1_chain_id);
        let l2_private_key = derive_stark_key(&l1_private_key, &stark_message).await?;

        Self::from_l2_private_key(config, l1_address, l2_private_key)
    }

    /// Create a new account from L2 private key directly
    pub fn from_l2_private_key(
        config: &SystemConfig,
        l1_address: impl Into<String>,
        l2_private_key: Felt,
    ) -> Result<Self> {
        // Compute public key from private key
        let l2_public_key = compute_public_key(l2_private_key)?;

        // Parse system config hashes
        let account_class_hash = Felt::from_hex(&config.paraclear_account_hash)
            .map_err(|e| ParadexError::ConfigError(format!("Invalid account hash: {}", e)))?;

        let proxy_class_hash = Felt::from_hex(&config.paraclear_account_proxy_hash)
            .map_err(|e| ParadexError::ConfigError(format!("Invalid proxy hash: {}", e)))?;

        // Compute L2 account address
        let l2_address =
            compute_account_address(l2_public_key, account_class_hash, proxy_class_hash)?;

        // Parse L2 chain ID from string (e.g., "SN_MAIN")
        // For now, use a simple hash of the chain ID string
        let mut chain_bytes = [0u8; 32];
        let id_bytes = config.starknet_chain_id.as_bytes();
        let copy_len = id_bytes.len().min(32);
        chain_bytes[32 - copy_len..].copy_from_slice(&id_bytes[..copy_len]);
        let chain_id = Felt::from_bytes_be(&chain_bytes);

        Ok(Self {
            l1_address: l1_address.into(),
            l2_address,
            l2_public_key,
            l2_private_key,
            chain_id,
            jwt_token: None,
        })
    }

    /// Get L2 address as hex string
    pub fn l2_address_hex(&self) -> String {
        format!("{:#x}", self.l2_address)
    }

    /// Get L2 public key as hex string
    pub fn l2_public_key_hex(&self) -> String {
        format!("{:#x}", self.l2_public_key)
    }

    /// Get L2 private key (for signing)
    #[allow(dead_code)]
    pub(crate) fn l2_private_key(&self) -> Felt {
        self.l2_private_key
    }

    /// Get chain ID
    pub fn chain_id(&self) -> Felt {
        self.chain_id
    }

    /// Set JWT token
    pub fn set_jwt_token(&mut self, token: impl Into<String>) {
        self.jwt_token = Some(token.into());
    }

    /// Get JWT token
    pub fn get_jwt_token(&self) -> Option<&str> {
        self.jwt_token.as_deref()
    }

    /// Sign a message hash with the L2 private key
    pub fn sign_hash(&self, hash: Felt) -> Result<(Felt, Felt)> {
        let signature = starknet_crypto::sign(&self.l2_private_key, &hash, &self.l2_public_key)
            .map_err(|e| ParadexError::SigningError(format!("Signing failed: {}", e)))?;

        Ok((signature.r, signature.s))
    }

    /// Flatten signature to hex string format
    pub fn flatten_signature(r: Felt, s: Felt) -> String {
        format!("[{:#x},{:#x}]", r, s)
    }
}

impl std::fmt::Debug for ParadexAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParadexAccount")
            .field("l1_address", &self.l1_address)
            .field("l2_address", &self.l2_address_hex())
            .field("l2_public_key", &self.l2_public_key_hex())
            .field("has_jwt_token", &self.jwt_token.is_some())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_system_config() -> SystemConfig {
        SystemConfig {
            l1_chain_id: "1".to_string(),
            starknet_chain_id: "SN_MAIN".to_string(),
            starknet_fullnode_rpc_url: "http://localhost".to_string(),
            paraclear_address: "0x123".to_string(),
            paraclear_account_proxy_hash:
                "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
            paraclear_account_hash:
                "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890".to_string(),
            paraclear_decimals: 8,
            bridged_tokens: vec![],
        }
    }

    #[test]
    fn test_account_from_l2_key() {
        let config = mock_system_config();
        let l2_key =
            Felt::from_hex("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
                .unwrap();

        let account = ParadexAccount::from_l2_private_key(
            &config,
            "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
            l2_key,
        );

        assert!(account.is_ok());
        let account = account.unwrap();
        assert_eq!(
            account.l1_address,
            "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
        );
    }

    #[test]
    fn test_flatten_signature() {
        let r = Felt::from_hex("0x123").unwrap();
        let s = Felt::from_hex("0x456").unwrap();
        let flattened = ParadexAccount::flatten_signature(r, s);
        assert!(flattened.starts_with("["));
        assert!(flattened.ends_with("]"));
    }
}
