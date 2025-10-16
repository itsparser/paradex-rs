use crate::{
    message::typed_data::{Domain, TypeMember, TypedData},
    types::Order,
};
use serde_json::Value;
use starknet_types_core::felt::Felt;
use std::collections::HashMap;

/// Build order message for signing
pub fn build_order_message(chain_id: Felt, order: &Order) -> TypedData {
    let mut types = HashMap::new();

    // Define StarkNetDomain type
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

    // Define Order type
    types.insert(
        "Order".to_string(),
        vec![
            TypeMember {
                name: "timestamp".to_string(),
                type_name: "felt".to_string(),
            },
            TypeMember {
                name: "market".to_string(),
                type_name: "felt".to_string(),
            },
            TypeMember {
                name: "side".to_string(),
                type_name: "felt".to_string(),
            },
            TypeMember {
                name: "orderType".to_string(),
                type_name: "felt".to_string(),
            },
            TypeMember {
                name: "size".to_string(),
                type_name: "felt".to_string(),
            },
            TypeMember {
                name: "price".to_string(),
                type_name: "felt".to_string(),
            },
        ],
    );

    let mut message = HashMap::new();
    message.insert(
        "timestamp".to_string(),
        Value::String(order.signature_timestamp.unwrap_or(0).to_string()),
    );
    message.insert("market".to_string(), Value::String(order.market.clone()));
    message.insert(
        "side".to_string(),
        Value::String(order.order_side.chain_side().to_string()),
    );
    message.insert(
        "orderType".to_string(),
        Value::String(order.order_type.to_string()),
    );
    message.insert("size".to_string(), Value::String(order.chain_size()));
    message.insert("price".to_string(), Value::String(order.chain_price()));

    TypedData {
        domain: Domain {
            name: "Paradex".to_string(),
            chain_id: format!("{:#x}", chain_id),
            version: "1".to_string(),
        },
        primary_type: "Order".to_string(),
        types,
        message,
    }
}

/// Build modify order message for signing
pub fn build_modify_order_message(chain_id: Felt, order: &Order) -> TypedData {
    let mut types = HashMap::new();

    // Define StarkNetDomain type
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

    // Define ModifyOrder type (includes id field)
    types.insert(
        "ModifyOrder".to_string(),
        vec![
            TypeMember {
                name: "timestamp".to_string(),
                type_name: "felt".to_string(),
            },
            TypeMember {
                name: "market".to_string(),
                type_name: "felt".to_string(),
            },
            TypeMember {
                name: "side".to_string(),
                type_name: "felt".to_string(),
            },
            TypeMember {
                name: "orderType".to_string(),
                type_name: "felt".to_string(),
            },
            TypeMember {
                name: "size".to_string(),
                type_name: "felt".to_string(),
            },
            TypeMember {
                name: "price".to_string(),
                type_name: "felt".to_string(),
            },
            TypeMember {
                name: "id".to_string(),
                type_name: "felt".to_string(),
            },
        ],
    );

    let mut message = HashMap::new();
    message.insert(
        "timestamp".to_string(),
        Value::String(order.signature_timestamp.unwrap_or(0).to_string()),
    );
    message.insert("market".to_string(), Value::String(order.market.clone()));
    message.insert(
        "side".to_string(),
        Value::String(order.order_side.chain_side().to_string()),
    );
    message.insert(
        "orderType".to_string(),
        Value::String(order.order_type.to_string()),
    );
    message.insert("size".to_string(), Value::String(order.chain_size()));
    message.insert("price".to_string(), Value::String(order.chain_price()));
    message.insert(
        "id".to_string(),
        Value::String(order.id.clone().unwrap_or_default()),
    );

    TypedData {
        domain: Domain {
            name: "Paradex".to_string(),
            chain_id: format!("{:#x}", chain_id),
            version: "1".to_string(),
        },
        primary_type: "ModifyOrder".to_string(),
        types,
        message,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{OrderSide, OrderType};

    #[test]
    fn test_build_order_message() {
        let order = Order {
            market: "BTC-USD-PERP".to_string(),
            order_side: OrderSide::Buy,
            order_type: OrderType::Limit,
            size: "1.0".to_string(),
            price: Some("50000".to_string()),
            signature_timestamp: Some(1234567890),
            client_id: None,
            instruction: None,
            reduce_only: None,
            trigger_price: None,
            signature: None,
            id: None,
            flags: None,
            recv_window: None,
            stp: None,
        };

        let chain_id = Felt::from_hex("0x1").unwrap();
        let typed_data = build_order_message(chain_id, &order);

        assert_eq!(typed_data.primary_type, "Order");
        assert_eq!(typed_data.domain.name, "Paradex");
    }
}
