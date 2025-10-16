use crate::{
    api::http_client::HttpClient,
    environment::Environment,
    error::Result,
    types::*,
};
use std::collections::HashMap;

/// API client for interacting with Paradex REST API
///
/// This client provides access to all Paradex API endpoints including:
/// - Market data (public)
/// - Account data (private)
/// - Orders (private)
/// - Positions (private)
/// - Fills and trades
/// - Funding and liquidations
pub struct ApiClient {
    http_client: HttpClient,
}

impl ApiClient {
    /// Create a new API client
    pub fn new(env: Environment) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new(env)?,
        })
    }

    /// Set JWT token for authenticated requests
    pub fn set_token(&mut self, token: impl Into<String>) {
        self.http_client.set_token(token);
    }

    /// Get the underlying HTTP client (for auth operations)
    pub(crate) fn get_http_client(&self) -> reqwest::Client {
        self.http_client.get_client()
    }

    // PUBLIC ENDPOINTS

    /// Fetch system configuration
    pub async fn fetch_system_config(&self) -> Result<SystemConfig> {
        self.http_client.get("system/config").await
    }

    /// Fetch system state
    pub async fn fetch_system_state(&self) -> Result<serde_json::Value> {
        self.http_client.get("system/state").await
    }

    /// Fetch system time
    pub async fn fetch_system_time(&self) -> Result<serde_json::Value> {
        self.http_client.get("system/time").await
    }

    /// Fetch all markets
    pub async fn fetch_markets(&self) -> Result<PaginatedResponse<Market>> {
        self.http_client.get("markets").await
    }

    /// Fetch markets summary
    pub async fn fetch_markets_summary(
        &self,
        market: Option<&str>,
    ) -> Result<PaginatedResponse<MarketSummary>> {
        match market {
            Some(m) => self.http_client.get_with_params("markets/summary", &[("market", m)]).await,
            None => self.http_client.get("markets/summary").await,
        }
    }

    /// Fetch orderbook for a specific market
    pub async fn fetch_orderbook(&self, market: &str, depth: Option<u32>) -> Result<OrderBook> {
        let path = format!("orderbook/{}", market);
        match depth {
            Some(d) => {
                let depth_str = d.to_string();
                self.http_client
                    .get_with_params(&path, &[("depth", &depth_str)])
                    .await
            }
            None => self.http_client.get(&path).await,
        }
    }

    /// Fetch best bid/offer for a specific market
    pub async fn fetch_bbo(&self, market: &str) -> Result<BBO> {
        let path = format!("bbo/{}", market);
        self.http_client.get(&path).await
    }

    /// Fetch trades for a specific market
    pub async fn fetch_trades(&self, market: &str) -> Result<PaginatedResponse<serde_json::Value>> {
        self.http_client
            .get_with_params("trades", &[("market", market)])
            .await
    }

    /// Fetch funding data
    pub async fn fetch_funding_data(
        &self,
        market: Option<&str>,
    ) -> Result<PaginatedResponse<serde_json::Value>> {
        match market {
            Some(m) => self.http_client.get_with_params("funding/data", &[("market", m)]).await,
            None => self.http_client.get("funding/data").await,
        }
    }

    /// Fetch insurance fund information
    pub async fn fetch_insurance_fund(&self) -> Result<serde_json::Value> {
        self.http_client.get("insurance").await
    }

    // PRIVATE ENDPOINTS (require authentication)

    /// Fetch account summary
    pub async fn fetch_account_summary(&self) -> Result<AccountSummary> {
        self.http_client.get("account").await
    }

    /// Fetch account profile
    pub async fn fetch_account_profile(&self) -> Result<serde_json::Value> {
        self.http_client.get("account/profile").await
    }

    /// Fetch account info
    pub async fn fetch_account_info(&self) -> Result<serde_json::Value> {
        self.http_client.get("account/info").await
    }

    /// Fetch sub-accounts
    pub async fn fetch_subaccounts(&self) -> Result<PaginatedResponse<serde_json::Value>> {
        self.http_client.get("account/subaccounts").await
    }

    /// Fetch balances
    pub async fn fetch_balances(&self) -> Result<PaginatedResponse<Balance>> {
        self.http_client.get("balance").await
    }

    /// Fetch positions
    pub async fn fetch_positions(&self) -> Result<PaginatedResponse<Position>> {
        self.http_client.get("positions").await
    }

    /// Fetch open orders
    pub async fn fetch_orders(&self, market: Option<&str>) -> Result<PaginatedResponse<OrderResponse>> {
        match market {
            Some(m) => self.http_client.get_with_params("orders", &[("market", m)]).await,
            None => self.http_client.get("orders").await,
        }
    }

    /// Fetch order history
    pub async fn fetch_orders_history(&self) -> Result<PaginatedResponse<OrderResponse>> {
        self.http_client.get("orders-history").await
    }

    /// Fetch specific order by ID
    pub async fn fetch_order(&self, order_id: &str) -> Result<OrderResponse> {
        let path = format!("orders/{}", order_id);
        self.http_client.get(&path).await
    }

    /// Fetch order by client ID
    pub async fn fetch_order_by_client_id(&self, client_id: &str) -> Result<OrderResponse> {
        let path = format!("orders/by_client_id/{}", client_id);
        self.http_client.get(&path).await
    }

    /// Submit a new order
    pub async fn submit_order(&self, order: &Order) -> Result<OrderResponse> {
        self.http_client.post("orders", order).await
    }

    /// Submit batch of orders
    pub async fn submit_orders_batch(&self, orders: &[Order]) -> Result<BatchOrderResponse> {
        self.http_client.post("orders/batch", &orders).await
    }

    /// Modify an existing order
    pub async fn modify_order(&self, order_id: &str, order: &Order) -> Result<OrderResponse> {
        let path = format!("orders/{}", order_id);
        self.http_client.put(&path, order).await
    }

    /// Cancel an order
    pub async fn cancel_order(&self, order_id: &str) -> Result<serde_json::Value> {
        let path = format!("orders/{}", order_id);
        self.http_client.delete(&path).await
    }

    /// Cancel order by client ID
    pub async fn cancel_order_by_client_id(&self, client_id: &str) -> Result<serde_json::Value> {
        let path = format!("orders/by_client_id/{}", client_id);
        self.http_client.delete(&path).await
    }

    /// Cancel all orders
    pub async fn cancel_all_orders(&self, market: Option<&str>) -> Result<serde_json::Value> {
        match market {
            Some(m) => {
                self.http_client
                    .get_with_params("orders", &[("market", m)])
                    .await
            }
            None => self.http_client.delete("orders").await,
        }
    }

    /// Cancel batch of orders
    pub async fn cancel_orders_batch(
        &self,
        order_ids: Option<&[String]>,
        client_order_ids: Option<&[String]>,
    ) -> Result<serde_json::Value> {
        let mut body = HashMap::new();
        if let Some(ids) = order_ids {
            body.insert("order_ids", ids);
        }
        if let Some(ids) = client_order_ids {
            body.insert("client_order_ids", ids);
        }
        self.http_client.delete_with_body("orders/batch", &body).await
    }

    /// Fetch fills
    pub async fn fetch_fills(&self, market: Option<&str>) -> Result<PaginatedResponse<Fill>> {
        match market {
            Some(m) => self.http_client.get_with_params("fills", &[("market", m)]).await,
            None => self.http_client.get("fills").await,
        }
    }

    /// Fetch tradebusts
    pub async fn fetch_tradebusts(&self) -> Result<PaginatedResponse<serde_json::Value>> {
        self.http_client.get("tradebusts").await
    }

    /// Fetch funding payments
    pub async fn fetch_funding_payments(
        &self,
        market: Option<&str>,
    ) -> Result<PaginatedResponse<FundingPayment>> {
        match market {
            Some(m) => {
                self.http_client
                    .get_with_params("funding/payments", &[("market", m)])
                    .await
            }
            None => self.http_client.get("funding/payments").await,
        }
    }

    /// Fetch transactions
    pub async fn fetch_transactions(&self) -> Result<PaginatedResponse<Transaction>> {
        self.http_client.get("transactions").await
    }

    /// Fetch transfers
    pub async fn fetch_transfers(&self) -> Result<PaginatedResponse<Transfer>> {
        self.http_client.get("transfers").await
    }

    /// Fetch liquidations
    pub async fn fetch_liquidations(&self) -> Result<PaginatedResponse<serde_json::Value>> {
        self.http_client.get("liquidations").await
    }

    /// Fetch points data
    pub async fn fetch_points_data(&self, market: &str, program: &str) -> Result<PointsData> {
        let path = format!("points_data/{}/{}", market, program);
        self.http_client.get(&path).await
    }
}
