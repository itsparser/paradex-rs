use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// System configuration from Paradex API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub l1_chain_id: String,
    pub starknet_chain_id: String,
    pub starknet_fullnode_rpc_url: String,
    pub paraclear_address: String,
    pub paraclear_account_proxy_hash: String,
    pub paraclear_account_hash: String,
    pub paraclear_decimals: u32,
    pub bridged_tokens: Vec<BridgedToken>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgedToken {
    pub l1_token_address: String,
    pub l2_token_address: String,
    pub l1_bridge_address: String,
    pub l2_bridge_address: String,
    pub decimals: u32,
    pub symbol: String,
}

/// Account summary response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSummary {
    pub account: String,
    pub equity_usd: String,
    pub notional_usd: String,
    pub total_pnl_usd: String,
    pub total_upnl_usd: String,
    pub total_rpnl_usd: String,
    pub margin_balance_usd: String,
    pub portfolio_initial_margin_requirement_usd: String,
    pub portfolio_maintenance_margin_requirement_usd: String,
    pub leverage: String,
    pub available_balance_usd: String,
    pub withdrawable_balance_usd: String,
    pub buying_power_usd: String,
}

/// Authentication response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub jwt_token: String,
}

/// Market information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub symbol: String,
    pub base_currency: String,
    pub quote_currency: String,
    pub price_tick_size: String,
    pub quantity_tick_size: String,
    pub min_quantity: String,
    pub max_quantity: String,
    pub max_market_order_size: String,
    pub max_leverage: String,
    pub status: String,
}

/// Market summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketSummary {
    pub symbol: String,
    pub last_price: Option<String>,
    pub index_price: Option<String>,
    pub mark_price: Option<String>,
    pub high_24h: Option<String>,
    pub low_24h: Option<String>,
    pub volume_24h: Option<String>,
    pub open_interest: Option<String>,
    pub funding_rate: Option<String>,
    pub next_funding_at: Option<i64>,
}

/// Order book entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookEntry {
    pub price: String,
    pub size: String,
}

/// Order book
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub bids: Vec<OrderBookEntry>,
    pub asks: Vec<OrderBookEntry>,
    pub seq_no: i64,
    pub timestamp: i64,
}

/// Best bid/offer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BBO {
    pub bid: Option<String>,
    pub bid_size: Option<String>,
    pub ask: Option<String>,
    pub ask_size: Option<String>,
    pub timestamp: i64,
}

/// Fill information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fill {
    pub id: String,
    pub account: String,
    pub market: String,
    pub order_id: String,
    pub client_id: Option<String>,
    pub side: String,
    pub price: String,
    pub size: String,
    pub fee: String,
    pub trade_id: String,
    pub liquidity_role: String,
    pub created_at: i64,
}

/// Position information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub account: String,
    pub market: String,
    pub side: String,
    pub size: String,
    pub entry_price: String,
    pub mark_price: String,
    pub liquidation_price: Option<String>,
    pub unrealized_pnl: String,
    pub realized_pnl: String,
    pub margin: String,
    pub leverage: String,
}

/// Balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub token: String,
    pub available: String,
    pub locked: String,
}

/// Transaction information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub account: String,
    pub r#type: String,
    pub amount: String,
    pub status: String,
    pub created_at: i64,
}

/// Transfer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transfer {
    pub id: String,
    pub account: String,
    pub r#type: String,
    pub amount: String,
    pub token: String,
    pub status: String,
    pub created_at: i64,
}

/// Funding payment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingPayment {
    pub id: String,
    pub account: String,
    pub market: String,
    pub payment: String,
    pub position_size: String,
    pub rate: String,
    pub created_at: i64,
}

/// Points data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointsData {
    pub market: String,
    pub program: String,
    pub points: String,
}

/// Paginated response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub results: Vec<T>,
    pub next: Option<String>,
    pub prev: Option<String>,
}

/// Generic API response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    #[serde(flatten)]
    pub data: T,
}

/// Order response from API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub id: String,
    pub client_id: Option<String>,
    pub account: String,
    pub market: String,
    pub side: String,
    pub r#type: String,
    pub price: Option<String>,
    pub size: String,
    pub filled_size: String,
    pub remaining_size: String,
    pub status: String,
    pub signature: String,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Batch order response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOrderResponse {
    pub orders: Vec<OrderResponse>,
    pub errors: Vec<OrderError>,
}

/// Order error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderError {
    pub client_id: Option<String>,
    pub error: String,
}
