use std::fmt;

/// Paradex environment configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    /// Production environment
    Prod,
    /// Testnet environment
    Testnet,
}

impl Environment {
    /// Get the API base URL for this environment
    pub fn api_url(&self) -> String {
        match self {
            Environment::Prod => "https://api.prod.paradex.trade/v1".to_string(),
            Environment::Testnet => "https://api.testnet.paradex.trade/v1".to_string(),
        }
    }

    /// Get the WebSocket URL for this environment
    pub fn ws_url(&self) -> String {
        match self {
            Environment::Prod => "wss://ws.api.prod.paradex.trade/v1".to_string(),
            Environment::Testnet => "wss://ws.api.testnet.paradex.trade/v1".to_string(),
        }
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Environment::Prod => write!(f, "prod"),
            Environment::Testnet => write!(f, "testnet"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_urls() {
        assert_eq!(
            Environment::Testnet.api_url(),
            "https://api.testnet.paradex.trade/v1"
        );
        assert_eq!(
            Environment::Prod.api_url(),
            "https://api.prod.paradex.trade/v1"
        );
    }

    #[test]
    fn test_environment_display() {
        assert_eq!(Environment::Testnet.to_string(), "testnet");
        assert_eq!(Environment::Prod.to_string(), "prod");
    }
}
