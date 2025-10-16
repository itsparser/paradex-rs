use crate::{
    account::ParadexAccount,
    error::Result,
    message::{
        build_auth_message, build_modify_order_message, build_onboarding_message,
        build_order_message,
    },
    types::Order,
};
use chrono::Utc;

impl ParadexAccount {
    /// Sign an order for submission
    pub fn sign_order(&self, order: &mut Order) -> Result<String> {
        // Set signature timestamp if not already set
        if order.signature_timestamp.is_none() {
            order.signature_timestamp = Some(Utc::now().timestamp_millis());
        }

        // Build the appropriate message based on whether it's a modification
        let typed_data = if order.id.is_some() {
            build_modify_order_message(self.chain_id(), order)
        } else {
            build_order_message(self.chain_id(), order)
        };

        // Compute message hash
        let message_hash = typed_data.message_hash()?;

        // Sign the hash
        let (r, s) = self.sign_hash(message_hash)?;

        // Flatten signature
        let signature = Self::flatten_signature(r, s);

        // Set signature on order
        order.signature = Some(signature.clone());

        Ok(signature)
    }

    /// Generate authentication headers for onboarding
    pub fn onboarding_headers(&self) -> Result<Vec<(String, String)>> {
        let typed_data = build_onboarding_message(self.chain_id());
        let message_hash = typed_data.message_hash()?;
        let (r, s) = self.sign_hash(message_hash)?;
        let signature = Self::flatten_signature(r, s);

        Ok(vec![
            (
                "PARADEX-ETHEREUM-ACCOUNT".to_string(),
                self.l1_address.clone(),
            ),
            (
                "PARADEX-STARKNET-ACCOUNT".to_string(),
                self.l2_address_hex(),
            ),
            ("PARADEX-STARKNET-SIGNATURE".to_string(), signature),
        ])
    }

    /// Generate authentication headers for JWT request
    pub fn auth_headers(&self) -> Result<Vec<(String, String)>> {
        let timestamp = Utc::now().timestamp();
        let expiry = timestamp + 24 * 60 * 60; // 24 hours

        let typed_data = build_auth_message(self.chain_id(), timestamp, expiry);
        let message_hash = typed_data.message_hash()?;
        let (r, s) = self.sign_hash(message_hash)?;
        let signature = Self::flatten_signature(r, s);

        Ok(vec![
            (
                "PARADEX-STARKNET-ACCOUNT".to_string(),
                self.l2_address_hex(),
            ),
            ("PARADEX-STARKNET-SIGNATURE".to_string(), signature),
            ("PARADEX-TIMESTAMP".to_string(), timestamp.to_string()),
            (
                "PARADEX-SIGNATURE-EXPIRATION".to_string(),
                expiry.to_string(),
            ),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{OrderSide, OrderType, SystemConfig};
    use starknet_types_core::felt::Felt;

    fn mock_config() -> SystemConfig {
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
    fn test_sign_order() {
        let config = mock_config();
        let private_key =
            Felt::from_hex("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
                .unwrap();

        let account = ParadexAccount::from_l2_private_key(
            &config,
            "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
            private_key,
        )
        .unwrap();

        let mut order = Order {
            market: "BTC-USD-PERP".to_string(),
            order_side: OrderSide::Buy,
            order_type: OrderType::Limit,
            size: "1.0".to_string(),
            price: Some("50000".to_string()),
            signature_timestamp: None,
            client_id: None,
            instruction: None,
            reduce_only: None,
            trigger_price: None,
            signature: None,
            id: None,
            flags: None,
            recv_window: None,
            stp: None,
        };

        let result = account.sign_order(&mut order);
        if let Err(e) = &result {
            eprintln!("Signing error: {:?}", e);
        }
        assert!(result.is_ok(), "Failed to sign order: {:?}", result);
        assert!(order.signature.is_some());
        assert!(order.signature_timestamp.is_some());
    }

    #[test]
    fn test_onboarding_headers() {
        let config = mock_config();
        let private_key =
            Felt::from_hex("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
                .unwrap();

        let account = ParadexAccount::from_l2_private_key(
            &config,
            "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
            private_key,
        )
        .unwrap();

        let headers = account.onboarding_headers();
        assert!(headers.is_ok());
        let headers = headers.unwrap();
        assert_eq!(headers.len(), 3);
    }
}
