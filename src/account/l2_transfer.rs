use crate::{account::ParadexAccount, error::Result};
use rust_decimal::Decimal;

impl ParadexAccount {
    /// Transfer USDC on L2 (Starknet)
    ///
    /// # Arguments
    ///
    /// * `target_l2_address` - Target L2 address
    /// * `amount_decimal` - Amount to transfer (in USDC decimals)
    ///
    /// # Note
    ///
    /// This requires full Starknet contract integration and is marked as TODO.
    /// The Python SDK uses starknet.py for contract calls.
    pub async fn transfer_on_l2(
        &self,
        target_l2_address: &str,
        amount_decimal: Decimal,
    ) -> Result<()> {
        // TODO: Implement L2 transfer using starknet-rs
        // This requires:
        // 1. Loading Paraclear contract
        // 2. Loading account contract
        // 3. Checking multisig requirements
        // 4. Preparing invoke transaction
        // 5. Signing and submitting
        log::warn!("transfer_on_l2: Not yet implemented");
        log::info!("Would transfer {} to {}", amount_decimal, target_l2_address);
        Err(crate::error::ParadexError::GenericError(
            "L2 transfer not yet implemented - requires full Starknet contract integration"
                .to_string(),
        ))
    }
}
