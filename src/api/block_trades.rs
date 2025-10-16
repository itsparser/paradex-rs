use crate::{
    api::http_client::HttpClient,
    error::Result,
    types::{
        BlockExecuteRequest, BlockOfferDetail, BlockOfferRequest, BlockTradeDetail,
        BlockTradeRequest, PaginatedResponse,
    },
};
use serde_json::Value;

/// Block trades API mixin
pub struct BlockTradesApi<'a> {
    http_client: &'a HttpClient,
}

impl<'a> BlockTradesApi<'a> {
    pub fn new(http_client: &'a HttpClient) -> Self {
        Self { http_client }
    }

    /// List block trades with optional filtering
    pub async fn list_block_trades(
        &self,
        status: Option<&str>,
        market: Option<&str>,
    ) -> Result<PaginatedResponse<BlockTradeDetail>> {
        let mut params = vec![];
        if let Some(s) = status {
            params.push(("status", s));
        }
        if let Some(m) = market {
            params.push(("market", m));
        }

        if params.is_empty() {
            self.http_client.get("block-trades").await
        } else {
            self.http_client
                .get_with_params("block-trades", &params)
                .await
        }
    }

    /// Create a new block trade
    pub async fn create_block_trade(
        &self,
        block_trade: &BlockTradeRequest,
    ) -> Result<BlockTradeDetail> {
        self.http_client.post("block-trades", block_trade).await
    }

    /// Get a specific block trade
    pub async fn get_block_trade(&self, block_trade_id: &str) -> Result<BlockTradeDetail> {
        let path = format!("block-trades/{}", block_trade_id);
        self.http_client.get(&path).await
    }

    /// Cancel a block trade
    pub async fn cancel_block_trade(&self, block_trade_id: &str) -> Result<Value> {
        let path = format!("block-trades/{}", block_trade_id);
        self.http_client.delete(&path).await
    }

    /// Execute a block trade
    pub async fn execute_block_trade(
        &self,
        block_trade_id: &str,
        execution: &BlockExecuteRequest,
    ) -> Result<BlockTradeDetail> {
        let path = format!("block-trades/{}/execute", block_trade_id);
        self.http_client.post(&path, execution).await
    }

    /// Get all offers for a block trade
    pub async fn get_block_trade_offers(
        &self,
        block_trade_id: &str,
    ) -> Result<PaginatedResponse<BlockOfferDetail>> {
        let path = format!("block-trades/{}/offers", block_trade_id);
        self.http_client.get(&path).await
    }

    /// Create an offer for a block trade
    pub async fn create_block_trade_offer(
        &self,
        block_trade_id: &str,
        offer: &BlockOfferRequest,
    ) -> Result<BlockOfferDetail> {
        let path = format!("block-trades/{}/offers", block_trade_id);
        self.http_client.post(&path, offer).await
    }

    /// Get a specific offer
    pub async fn get_block_trade_offer(
        &self,
        block_trade_id: &str,
        offer_id: &str,
    ) -> Result<BlockOfferDetail> {
        let path = format!("block-trades/{}/offers/{}", block_trade_id, offer_id);
        self.http_client.get(&path).await
    }

    /// Cancel an offer
    pub async fn cancel_block_trade_offer(
        &self,
        block_trade_id: &str,
        offer_id: &str,
    ) -> Result<Value> {
        let path = format!("block-trades/{}/offers/{}", block_trade_id, offer_id);
        self.http_client.delete(&path).await
    }

    /// Execute a specific offer
    pub async fn execute_block_trade_offer(
        &self,
        block_trade_id: &str,
        offer_id: &str,
        execution: &BlockExecuteRequest,
    ) -> Result<BlockOfferDetail> {
        let path = format!(
            "block-trades/{}/offers/{}/execute",
            block_trade_id, offer_id
        );
        self.http_client.post(&path, execution).await
    }
}
