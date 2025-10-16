//! Message signing module
//!
//! Provides typed data builders for EIP-712 style message signing on Starknet.

pub mod auth;
pub mod onboarding;
pub mod order;
pub mod typed_data;

pub use auth::{build_auth_message, build_fullnode_message};
pub use onboarding::build_onboarding_message;
pub use order::{build_modify_order_message, build_order_message};
pub use typed_data::TypedData;
