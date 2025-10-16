use thiserror::Error;

/// Result type for Paradex operations
pub type Result<T> = std::result::Result<T, ParadexError>;

/// Paradex SDK error types
#[derive(Error, Debug)]
pub enum ParadexError {
    /// HTTP request error
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// WebSocket error
    #[error("WebSocket error: {0}")]
    WebSocketError(String),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Authentication error
    #[error("Authentication error: {0}")]
    AuthError(String),

    /// Account error
    #[error("Account error: {0}")]
    AccountError(String),

    /// Signing error
    #[error("Signing error: {0}")]
    SigningError(String),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    /// API error with HTTP status code
    #[error("API error (status {status}): {message}")]
    ApiError { status: u16, message: String },

    /// Generic error
    #[error("{0}")]
    GenericError(String),

    /// Starknet error
    #[error("Starknet error: {0}")]
    StarknetError(String),

    /// Ethereum error
    #[error("Ethereum error: {0}")]
    EthereumError(String),
}
