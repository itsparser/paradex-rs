use crate::{
    account::ParadexAccount,
    error::Result,
    message::{build_block_offer_message, build_block_trade_message},
    types::{BlockOfferRequest, BlockTradeRequest},
};

impl ParadexAccount {
    /// Sign a block trade
    pub fn sign_block_trade(&self, block_trade: &BlockTradeRequest) -> Result<String> {
        let typed_data = build_block_trade_message(self.chain_id(), block_trade);
        let message_hash = typed_data.message_hash()?;
        let (r, s) = self.sign_hash(message_hash)?;
        Ok(Self::flatten_signature(r, s))
    }

    /// Sign a block offer
    pub fn sign_block_offer(&self, offer: &BlockOfferRequest) -> Result<String> {
        let typed_data = build_block_offer_message(self.chain_id(), offer);
        let message_hash = typed_data.message_hash()?;
        let (r, s) = self.sign_hash(message_hash)?;
        Ok(Self::flatten_signature(r, s))
    }
}
