use crate::error::{ParadexError, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use starknet_core::utils::starknet_keccak;
use starknet_types_core::felt::Felt;
use std::collections::HashMap;

/// EIP-712 style typed data for Starknet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypedData {
    pub domain: Domain,
    #[serde(rename = "primaryType")]
    pub primary_type: String,
    pub types: HashMap<String, Vec<TypeMember>>,
    pub message: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    pub name: String,
    #[serde(rename = "chainId")]
    pub chain_id: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeMember {
    pub name: String,
    #[serde(rename = "type")]
    pub type_name: String,
}

impl TypedData {
    /// Compute the message hash for signing
    pub fn message_hash(&self) -> Result<Felt> {
        // Encode domain
        let domain_hash = self.encode_type("StarkNetDomain")?;

        // Encode message
        let message_hash = self.encode_message(&self.primary_type)?;

        // Compute final hash: hash("StarkNet Message", domain_hash, account_address, message_hash)
        let prefix = starknet_keccak(b"StarkNet Message");

        // For now, we'll return the message hash
        // Full implementation would combine with domain and account
        Ok(message_hash)
    }

    fn encode_type(&self, type_name: &str) -> Result<Felt> {
        let type_def = self
            .types
            .get(type_name)
            .ok_or_else(|| ParadexError::SigningError(format!("Type not found: {}", type_name)))?;

        let mut encoding = type_name.to_string();
        encoding.push('(');

        for (i, member) in type_def.iter().enumerate() {
            if i > 0 {
                encoding.push(',');
            }
            encoding.push_str(&format!("{} {}", member.type_name, member.name));
        }
        encoding.push(')');

        let hash = starknet_keccak(encoding.as_bytes());
        Ok(hash)
    }

    fn encode_message(&self, type_name: &str) -> Result<Felt> {
        let type_hash = self.encode_type(type_name)?;

        let type_def = self
            .types
            .get(type_name)
            .ok_or_else(|| ParadexError::SigningError(format!("Type not found: {}", type_name)))?;

        let mut values = vec![type_hash];

        for member in type_def {
            let value = self.message.get(&member.name).ok_or_else(|| {
                ParadexError::SigningError(format!("Missing field: {}", member.name))
            })?;

            let encoded_value = self.encode_value(&member.type_name, value)?;
            values.push(encoded_value);
        }

        // Hash all values together
        let mut hash_input = Vec::new();
        for value in values {
            hash_input.extend_from_slice(&value.to_bytes_be());
        }

        let hash = starknet_keccak(&hash_input);
        Ok(hash)
    }

    fn encode_value(&self, type_name: &str, value: &Value) -> Result<Felt> {
        match type_name {
            "felt" => {
                if let Value::String(s) = value {
                    // Try to parse as hex or decimal
                    if s.starts_with("0x") {
                        Felt::from_hex(s)
                            .map_err(|e| ParadexError::SigningError(format!("Invalid felt: {}", e)))
                    } else {
                        Felt::from_dec_str(s)
                            .map_err(|e| ParadexError::SigningError(format!("Invalid felt: {}", e)))
                    }
                } else {
                    Err(ParadexError::SigningError(
                        "Expected string for felt".to_string(),
                    ))
                }
            }
            _ => {
                // For other types, try to encode as felt
                if let Value::String(s) = value {
                    if s.starts_with("0x") {
                        Felt::from_hex(s).map_err(|e| {
                            ParadexError::SigningError(format!("Invalid value: {}", e))
                        })
                    } else {
                        Felt::from_dec_str(s).map_err(|e| {
                            ParadexError::SigningError(format!("Invalid value: {}", e))
                        })
                    }
                } else {
                    Err(ParadexError::SigningError(
                        "Expected string value".to_string(),
                    ))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typed_data_structure() {
        let typed_data = TypedData {
            domain: Domain {
                name: "Paradex".to_string(),
                chain_id: "0x1".to_string(),
                version: "1".to_string(),
            },
            primary_type: "Order".to_string(),
            types: HashMap::new(),
            message: HashMap::new(),
        };

        assert_eq!(typed_data.domain.name, "Paradex");
        assert_eq!(typed_data.primary_type, "Order");
    }
}
