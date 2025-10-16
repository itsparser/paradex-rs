use crate::error::{ParadexError, Result};
use ethers::signers::{LocalWallet, Signer};
use futures::TryFutureExt;
use starknet_types_core::felt::Felt;
use tiny_keccak::{Hasher, Keccak};

/// Derive Stark key from Ethereum private key
/// This matches the Python SDK's stark key derivation logic
pub async fn derive_stark_key(eth_private_key: &str, message: &str) -> Result<Felt> {
    // Parse the Ethereum private key
    let wallet: LocalWallet = eth_private_key
        .parse()
        .map_err(|e| ParadexError::EthereumError(format!("Invalid private key: {e}")))?;

    // Sign the message with the Ethereum key
    let signature = wallet
        .sign_message(message.as_bytes())
        .map_err(|e| ParadexError::EthereumError(format!("Signing failed: {e}")))
        .await?;

    // Convert signature to bytes (r + s, 64 bytes)
    let sig_bytes = signature.to_vec();

    // Hash the signature to get a 256-bit value for Stark key
    let mut hasher = Keccak::v256();
    hasher.update(&sig_bytes);
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);

    // Convert to Felt, ensuring it's within the field
    let stark_key = Felt::from_bytes_be(&output);

    Ok(stark_key)
}

/// Build the stark key derivation message for signing
pub fn build_stark_key_message(chain_id: u64) -> String {
    format!("Paradex Stark Key Derivation: {chain_id}")
}

/// Compute Starknet public key from private key
pub fn compute_public_key(private_key: Felt) -> Result<Felt> {
    let public_key = starknet_crypto::get_public_key(&private_key);
    Ok(public_key)
}

/// Compute Paradex account address from public key and system config
pub fn compute_account_address(
    public_key: Felt,
    account_class_hash: Felt,
    proxy_class_hash: Felt,
) -> Result<Felt> {
    use starknet_core::utils::get_selector_from_name;

    // Build constructor calldata
    // [account_class_hash, initialize_selector, 2, public_key, 0]
    let initialize_selector = get_selector_from_name("initialize")
        .map_err(|e| ParadexError::StarknetError(format!("Selector error: {e}")))?;

    let calldata = vec![
        account_class_hash,
        initialize_selector,
        Felt::TWO,
        public_key,
        Felt::ZERO,
    ];

    // Compute contract address
    let address = starknet_core::utils::get_contract_address(
        public_key, // salt
        proxy_class_hash,
        &calldata,
        Felt::ZERO, // deployer_address
    );

    Ok(address)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_stark_key_message() {
        let message = build_stark_key_message(1);
        assert_eq!(message, "Paradex Stark Key Derivation: 1");
    }

    #[test]
    fn test_compute_public_key() {
        // Test with a known private key
        let private_key =
            Felt::from_hex("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
                .unwrap();

        let public_key = compute_public_key(private_key);
        assert!(public_key.is_ok());
    }
}
