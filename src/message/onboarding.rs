use crate::message::typed_data::{Domain, TypeMember, TypedData};
use starknet_types_core::felt::Felt;
use std::collections::HashMap;

/// Build onboarding message for signing
pub fn build_onboarding_message(chain_id: Felt) -> TypedData {
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

    // Define Onboarding type (empty message)
    types.insert("Onboarding".to_string(), vec![]);

    let message = HashMap::new(); // Empty message for onboarding

    TypedData {
        domain: Domain {
            name: "Paradex".to_string(),
            chain_id: format!("{:#x}", chain_id),
            version: "1".to_string(),
        },
        primary_type: "Onboarding".to_string(),
        types,
        message,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_onboarding_message() {
        let chain_id = Felt::from_hex("0x1").unwrap();
        let typed_data = build_onboarding_message(chain_id);

        assert_eq!(typed_data.primary_type, "Onboarding");
        assert_eq!(typed_data.domain.name, "Paradex");
        assert!(typed_data.message.is_empty());
    }
}
