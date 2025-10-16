use crate::message::typed_data::{Domain, TypeMember, TypedData};
use serde_json::Value;
use starknet_types_core::felt::Felt;
use std::collections::HashMap;

/// Build authentication message for signing
pub fn build_auth_message(chain_id: Felt, timestamp: i64, expiry: i64) -> TypedData {
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

    // Define Auth type
    types.insert(
        "Auth".to_string(),
        vec![
            TypeMember {
                name: "timestamp".to_string(),
                type_name: "felt".to_string(),
            },
            TypeMember {
                name: "expiry".to_string(),
                type_name: "felt".to_string(),
            },
        ],
    );

    let mut message = HashMap::new();
    message.insert(
        "timestamp".to_string(),
        Value::String(timestamp.to_string()),
    );
    message.insert("expiry".to_string(), Value::String(expiry.to_string()));

    TypedData {
        domain: Domain {
            name: "Paradex".to_string(),
            chain_id: format!("{:#x}", chain_id),
            version: "1".to_string(),
        },
        primary_type: "Auth".to_string(),
        types,
        message,
    }
}

/// Build fullnode RPC message for signing
pub fn build_fullnode_message(
    chain_id: Felt,
    account_address: &str,
    json_payload: &str,
    signature_timestamp: i64,
    signature_version: &str,
) -> TypedData {
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
        "FullnodeRequest".to_string(),
        vec![
            TypeMember {
                name: "account".to_string(),
                type_name: "felt".to_string(),
            },
            TypeMember {
                name: "payload".to_string(),
                type_name: "felt".to_string(),
            },
            TypeMember {
                name: "timestamp".to_string(),
                type_name: "felt".to_string(),
            },
            TypeMember {
                name: "version".to_string(),
                type_name: "felt".to_string(),
            },
        ],
    );

    let mut message = HashMap::new();
    message.insert(
        "account".to_string(),
        Value::String(account_address.to_string()),
    );
    message.insert(
        "payload".to_string(),
        Value::String(json_payload.to_string()),
    );
    message.insert(
        "timestamp".to_string(),
        Value::String(signature_timestamp.to_string()),
    );
    message.insert(
        "version".to_string(),
        Value::String(signature_version.to_string()),
    );

    TypedData {
        domain: Domain {
            name: "Paradex".to_string(),
            chain_id: format!("{:#x}", chain_id),
            version: "1".to_string(),
        },
        primary_type: "FullnodeRequest".to_string(),
        types,
        message,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_auth_message() {
        let chain_id = Felt::from_hex("0x1").unwrap();
        let timestamp = 1234567890;
        let expiry = timestamp + 86400;

        let typed_data = build_auth_message(chain_id, timestamp, expiry);

        assert_eq!(typed_data.primary_type, "Auth");
        assert_eq!(typed_data.domain.name, "Paradex");
    }

    #[test]
    fn test_build_fullnode_message() {
        let chain_id = Felt::from_hex("0x1").unwrap();
        let typed_data = build_fullnode_message(
            chain_id,
            "0x123",
            r#"{"method":"test"}"#,
            1234567890,
            "1.0.0",
        );

        assert_eq!(typed_data.primary_type, "FullnodeRequest");
    }
}
