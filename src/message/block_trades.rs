use crate::message::typed_data::{Domain, TypeMember, TypedData};
use crate::types::{BlockOfferRequest, BlockTradeRequest};
use serde_json::Value;
use starknet_types_core::felt::Felt;
use std::collections::HashMap;

/// Build block trade message for signing
pub fn build_block_trade_message(chain_id: Felt, block_trade: &BlockTradeRequest) -> TypedData {
    let mut types = HashMap::new();

    types.insert(
        "StarkNetDomain".to_string(),
        vec![
            TypeMember {
                name: "name".to_string(),
                type_name: "felt".to_string(),
            },
            TypeMember {
                name: "chainId".to_string(),
                type_name: "felt".to_string(),
            },
            TypeMember {
                name: "version".to_string(),
                type_name: "felt".to_string(),
            },
        ],
    );

    types.insert(
        "BlockTrade".to_string(),
        vec![
            TypeMember {
                name: "timestamp".to_string(),
                type_name: "felt".to_string(),
            },
            TypeMember {
                name: "markets".to_string(),
                type_name: "felt".to_string(),
            },
            TypeMember {
                name: "required_signers".to_string(),
                type_name: "felt".to_string(),
            },
        ],
    );

    let mut message = HashMap::new();
    message.insert(
        "timestamp".to_string(),
        Value::String(block_trade.signature_timestamp.to_string()),
    );
    message.insert(
        "markets".to_string(),
        Value::String(block_trade.markets.join(",")),
    );
    message.insert(
        "required_signers".to_string(),
        Value::String(block_trade.required_signers.join(",")),
    );

    TypedData {
        domain: Domain {
            name: "Paradex".to_string(),
            chain_id: format!("{chain_id:#x}"),
            version: "1".to_string(),
        },
        primary_type: "BlockTrade".to_string(),
        types,
        message,
    }
}

/// Build block offer message for signing
pub fn build_block_offer_message(chain_id: Felt, offer: &BlockOfferRequest) -> TypedData {
    let mut types = HashMap::new();

    types.insert(
        "StarkNetDomain".to_string(),
        vec![
            TypeMember {
                name: "name".to_string(),
                type_name: "felt".to_string(),
            },
            TypeMember {
                name: "chainId".to_string(),
                type_name: "felt".to_string(),
            },
            TypeMember {
                name: "version".to_string(),
                type_name: "felt".to_string(),
            },
        ],
    );

    types.insert(
        "BlockOffer".to_string(),
        vec![TypeMember {
            name: "timestamp".to_string(),
            type_name: "felt".to_string(),
        }],
    );

    let mut message = HashMap::new();
    message.insert(
        "timestamp".to_string(),
        Value::String(offer.signature_timestamp.to_string()),
    );

    TypedData {
        domain: Domain {
            name: "Paradex".to_string(),
            chain_id: format!("{chain_id:#x}"),
            version: "1".to_string(),
        },
        primary_type: "BlockOffer".to_string(),
        types,
        message,
    }
}
