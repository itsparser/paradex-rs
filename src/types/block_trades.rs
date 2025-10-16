use serde::{Deserialize, Serialize};

/// Block trade request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockTradeRequest {
    pub markets: Vec<String>,
    pub required_signers: Vec<String>,
    pub signature: String,
    pub signature_timestamp: i64,
}

/// Block offer request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockOfferRequest {
    pub orders: Vec<BlockOfferOrder>,
    pub signature: String,
    pub signature_timestamp: i64,
}

/// Block offer order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockOfferOrder {
    pub market: String,
    pub side: String,
    pub size: String,
    pub price: String,
}

/// Block execute request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockExecuteRequest {
    pub offer_ids: Vec<String>,
}

/// Block trade detail response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockTradeDetail {
    pub block_id: String,
    pub status: String,
    pub markets: Vec<String>,
    pub initiator: String,
    pub required_signers: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Block trade offer detail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockOfferDetail {
    pub offer_id: String,
    pub block_id: String,
    pub account: String,
    pub orders: Vec<BlockOfferOrder>,
    pub status: String,
    pub created_at: i64,
}
