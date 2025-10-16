use crate::{environment::Environment, error::Result};

#[path = "ws_client_impl.rs"]
mod ws_impl;
pub use ws_impl::WebSocketClientImpl;

/// WebSocket channels available in Paradex
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebSocketChannel {
    /// Private channel for account updates
    Account,
    /// Private channel for balance event updates
    BalanceEvents,
    /// Public channel for best bid/offer updates
    BBO,
    /// Private channel for fill updates
    Fills,
    /// Public channel for funding data updates
    FundingData,
    /// Private channel for funding payment updates
    FundingPayments,
    /// Public channel for funding rate comparison
    FundingRateComparison,
    /// Public channel for market summary updates
    MarketsSummary,
    /// Private channel for order updates
    Orders,
    /// Public channel for orderbook snapshots
    OrderBook,
    /// Private channel for position updates
    Positions,
    /// Public channel for trade updates
    Trades,
    /// Private channel for tradebust notifications
    Tradebusts,
    /// Private channel for transaction updates
    Transactions,
    /// Private channel for transfer updates
    Transfers,
}

impl WebSocketChannel {
    /// Get the channel name as string
    pub fn as_str(&self) -> &'static str {
        match self {
            WebSocketChannel::Account => "account",
            WebSocketChannel::BalanceEvents => "balance_events",
            WebSocketChannel::BBO => "bbo",
            WebSocketChannel::Fills => "fills",
            WebSocketChannel::FundingData => "funding_data",
            WebSocketChannel::FundingPayments => "funding_payments",
            WebSocketChannel::FundingRateComparison => "funding_rate_comparison",
            WebSocketChannel::MarketsSummary => "markets_summary",
            WebSocketChannel::Orders => "orders",
            WebSocketChannel::OrderBook => "order_book",
            WebSocketChannel::Positions => "positions",
            WebSocketChannel::Trades => "trades",
            WebSocketChannel::Tradebusts => "tradebusts",
            WebSocketChannel::Transactions => "transaction",
            WebSocketChannel::Transfers => "transfers",
        }
    }

    /// Check if this channel requires authentication
    pub fn requires_auth(&self) -> bool {
        matches!(
            self,
            WebSocketChannel::Account
                | WebSocketChannel::BalanceEvents
                | WebSocketChannel::Fills
                | WebSocketChannel::FundingPayments
                | WebSocketChannel::Orders
                | WebSocketChannel::Positions
                | WebSocketChannel::Tradebusts
                | WebSocketChannel::Transactions
                | WebSocketChannel::Transfers
        )
    }

    /// Format channel name with market parameter (e.g., "bbo.BTC-USD-PERP")
    pub fn with_market(&self, market: &str) -> String {
        format!("{}.{}", self.as_str(), market)
    }

    /// Format channel with multiple parameters
    /// For example, OrderBook needs: market, depth, refresh_rate, price_tick
    pub fn with_params(&self, params: &[&str]) -> String {
        let mut channel = self.as_str().to_string();
        for param in params {
            channel.push('.');
            channel.push_str(param);
        }
        channel
    }
}

/// WebSocket client facade (wraps implementation)
pub struct WebSocketClient {
    inner: WebSocketClientImpl,
}

impl WebSocketClient {
    /// Create a new WebSocket client
    pub fn new(env: Environment) -> Self {
        Self {
            inner: WebSocketClientImpl::new(env),
        }
    }

    /// Set JWT token for authenticated channels
    pub fn set_token(&mut self, token: impl Into<String>) {
        self.inner.set_token(token);
    }

    /// Connect to WebSocket
    pub async fn connect(&self) -> Result<()> {
        self.inner.connect().await
    }

    /// Subscribe to a channel with a callback
    pub async fn subscribe<F>(
        &self,
        channel: WebSocketChannel,
        market: Option<&str>,
        callback: F,
    ) -> Result<()>
    where
        F: Fn(serde_json::Value) -> futures::future::BoxFuture<'static, ()> + Send + Sync + 'static,
    {
        let channel_name = if let Some(m) = market {
            channel.with_market(m)
        } else {
            channel.as_str().to_string()
        };

        self.inner
            .subscribe(&channel_name, std::sync::Arc::new(callback))
            .await
    }

    /// Subscribe to a channel by exact name
    pub async fn subscribe_by_name<F>(&self, channel_name: &str, callback: F) -> Result<()>
    where
        F: Fn(serde_json::Value) -> futures::future::BoxFuture<'static, ()> + Send + Sync + 'static,
    {
        self.inner
            .subscribe(channel_name, std::sync::Arc::new(callback))
            .await
    }

    /// Unsubscribe from a channel
    pub async fn unsubscribe(&self, channel: WebSocketChannel, market: Option<&str>) -> Result<()> {
        let channel_name = if let Some(m) = market {
            channel.with_market(m)
        } else {
            channel.as_str().to_string()
        };

        self.inner.unsubscribe(&channel_name).await
    }

    /// Check if connected
    pub async fn is_connected(&self) -> bool {
        self.inner.is_connected().await
    }

    /// Close the connection
    pub async fn close(&self) -> Result<()> {
        self.inner.close().await
    }

    /// Get current subscriptions
    pub async fn get_subscriptions(&self) -> std::collections::HashMap<String, bool> {
        self.inner.get_subscriptions().await
    }

    /// Manually pump one message (for deterministic consumption)
    pub async fn pump_once(&self) -> Result<bool> {
        self.inner.pump_once().await
    }

    /// Pump messages until predicate is satisfied
    pub async fn pump_until<F>(&self, predicate: F, timeout_secs: f64) -> Result<u32>
    where
        F: Fn(&serde_json::Value) -> bool,
    {
        self.inner.pump_until(predicate, timeout_secs).await
    }

    /// Inject a raw message for testing/simulation
    pub async fn inject(&self, message: &str) -> Result<()> {
        self.inner.inject(message).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_names() {
        assert_eq!(WebSocketChannel::BBO.as_str(), "bbo");
        assert_eq!(WebSocketChannel::Account.as_str(), "account");
        assert_eq!(WebSocketChannel::Trades.as_str(), "trades");
    }

    #[test]
    fn test_channel_with_market() {
        let channel = WebSocketChannel::BBO.with_market("BTC-USD-PERP");
        assert_eq!(channel, "bbo.BTC-USD-PERP");
    }

    #[test]
    fn test_channel_requires_auth() {
        assert!(WebSocketChannel::Account.requires_auth());
        assert!(WebSocketChannel::Orders.requires_auth());
        assert!(!WebSocketChannel::BBO.requires_auth());
        assert!(!WebSocketChannel::Trades.requires_auth());
    }
}
