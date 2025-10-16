use serde::{Deserialize, Serialize};
use std::fmt;

/// Order side (Buy/Sell)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
}

impl OrderSide {
    /// Convert to chain-compatible integer representation
    pub fn chain_side(&self) -> u8 {
        match self {
            OrderSide::Buy => 1,
            OrderSide::Sell => 2,
        }
    }
}

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderSide::Buy => write!(f, "BUY"),
            OrderSide::Sell => write!(f, "SELL"),
        }
    }
}

/// Order type (Limit/Market/Stop/etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename = "MARKET")]
    Market,
    #[serde(rename = "LIMIT")]
    Limit,
    #[serde(rename = "STOP_LIMIT")]
    StopLimit,
    #[serde(rename = "STOP_MARKET")]
    StopMarket,
    #[serde(rename = "TAKE_PROFIT_LIMIT")]
    TakeProfitLimit,
    #[serde(rename = "TAKE_PROFIT_MARKET")]
    TakeProfitMarket,
    #[serde(rename = "STOP_LOSS_MARKET")]
    StopLossMarket,
    #[serde(rename = "STOP_LOSS_LIMIT")]
    StopLossLimit,
}

impl OrderType {
    /// Check if this is a limit order type
    pub fn is_limit_type(&self) -> bool {
        matches!(
            self,
            OrderType::Limit
                | OrderType::StopLimit
                | OrderType::TakeProfitLimit
                | OrderType::StopLossLimit
        )
    }
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderType::Market => write!(f, "MARKET"),
            OrderType::Limit => write!(f, "LIMIT"),
            OrderType::StopLimit => write!(f, "STOP_LIMIT"),
            OrderType::StopMarket => write!(f, "STOP_MARKET"),
            OrderType::TakeProfitLimit => write!(f, "TAKE_PROFIT_LIMIT"),
            OrderType::TakeProfitMarket => write!(f, "TAKE_PROFIT_MARKET"),
            OrderType::StopLossMarket => write!(f, "STOP_LOSS_MARKET"),
            OrderType::StopLossLimit => write!(f, "STOP_LOSS_LIMIT"),
        }
    }
}

/// Time in force
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeInForce {
    #[serde(rename = "GTC")]
    Gtc, // Good Till Cancel
    #[serde(rename = "IOC")]
    Ioc, // Immediate Or Cancel
    #[serde(rename = "FOK")]
    Fok, // Fill Or Kill
}

/// Order instruction type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderInstruction {
    #[serde(rename = "GTC")]
    Gtc,
    #[serde(rename = "POST_ONLY")]
    PostOnly,
    #[serde(rename = "IOC")]
    Ioc,
    #[serde(rename = "FOK")]
    Fok,
}

impl fmt::Display for OrderInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderInstruction::Gtc => write!(f, "GTC"),
            OrderInstruction::PostOnly => write!(f, "POST_ONLY"),
            OrderInstruction::Ioc => write!(f, "IOC"),
            OrderInstruction::Fok => write!(f, "FOK"),
        }
    }
}

/// Order structure for submitting to Paradex
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// Market symbol (e.g., "BTC-USD-PERP")
    pub market: String,

    /// Order side (Buy/Sell)
    #[serde(rename = "side")]
    pub order_side: OrderSide,

    /// Order type (Limit/Market/Stop/etc.)
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Order size (as string with proper decimal places)
    pub size: String,

    /// Order price (required for limit orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Client-assigned order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,

    /// Order instruction (GTC, POST_ONLY, IOC, FOK)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction: Option<OrderInstruction>,

    /// Reduce-only flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    /// Trigger price for conditional orders (Stop/TakeProfit orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,

    /// Receive window for order validity (milliseconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Self-trade prevention (EXPIRE_MAKER, EXPIRE_TAKER, EXPIRE_BOTH)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp: Option<String>,

    /// Order signature (set when signing)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,

    /// Signature timestamp (milliseconds since epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_timestamp: Option<i64>,

    /// Order ID (for modify operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Order flags (e.g., ["REDUCE_ONLY"])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
}

impl Order {
    /// Create a new order builder
    pub fn builder() -> OrderBuilder {
        OrderBuilder::default()
    }

    /// Convert size to chain-compatible format (quantum with 8 decimals)
    pub fn chain_size(&self) -> String {
        self.size.clone()
    }

    /// Convert price to chain-compatible format (quantum with 8 decimals)
    pub fn chain_price(&self) -> String {
        self.price.clone().unwrap_or_else(|| "0".to_string())
    }
}

/// Order builder for fluent API
#[derive(Debug, Default)]
pub struct OrderBuilder {
    market: Option<String>,
    order_side: Option<OrderSide>,
    order_type: Option<OrderType>,
    size: Option<String>,
    price: Option<String>,
    client_id: Option<String>,
    instruction: Option<OrderInstruction>,
    reduce_only: Option<bool>,
    trigger_price: Option<String>,
    recv_window: Option<i64>,
    stp: Option<String>,
}

impl OrderBuilder {
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.market = Some(market.into());
        self
    }

    pub fn side(mut self, side: OrderSide) -> Self {
        self.order_side = Some(side);
        self
    }

    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = Some(order_type);
        self
    }

    pub fn size(mut self, size: impl Into<String>) -> Self {
        self.size = Some(size.into());
        self
    }

    pub fn price(mut self, price: impl Into<String>) -> Self {
        self.price = Some(price.into());
        self
    }

    pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    pub fn instruction(mut self, instruction: OrderInstruction) -> Self {
        self.instruction = Some(instruction);
        self
    }

    pub fn reduce_only(mut self, reduce_only: bool) -> Self {
        self.reduce_only = Some(reduce_only);
        self
    }

    pub fn trigger_price(mut self, trigger_price: impl Into<String>) -> Self {
        self.trigger_price = Some(trigger_price.into());
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }

    pub fn stp(mut self, stp: impl Into<String>) -> Self {
        self.stp = Some(stp.into());
        self
    }

    pub fn build(self) -> Result<Order, String> {
        Ok(Order {
            market: self.market.ok_or("market is required")?,
            order_side: self.order_side.ok_or("order_side is required")?,
            order_type: self.order_type.ok_or("order_type is required")?,
            size: self.size.ok_or("size is required")?,
            price: self.price,
            client_id: self.client_id,
            instruction: self.instruction,
            reduce_only: self.reduce_only,
            trigger_price: self.trigger_price,
            recv_window: self.recv_window,
            stp: self.stp,
            signature: None,
            signature_timestamp: None,
            id: None,
            flags: None,
        })
    }
}
