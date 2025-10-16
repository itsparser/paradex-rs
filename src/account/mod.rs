//! Account management module
//!
//! Handles L1/L2 key derivation, account address computation, and message signing.

mod account;
mod block_trades_signing;
mod key_derivation;
mod l2_transfer;
mod signing;

pub use account::ParadexAccount;
pub use key_derivation::{
    build_stark_key_message, compute_account_address, compute_public_key, derive_stark_key,
};
