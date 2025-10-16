//! Utility functions for Paradex SDK

use rust_decimal::Decimal;

/// Convert decimal to quantum (8 decimal places)
pub fn to_quantum(value: Decimal, decimals: u32) -> String {
    let multiplier = Decimal::from(10u64.pow(decimals));
    let quantum = value * multiplier;
    quantum.normalize().to_string()
}

/// Convert quantum to decimal
pub fn from_quantum(quantum: &str, decimals: u32) -> Result<Decimal, String> {
    let quantum_value =
        Decimal::from_str_exact(quantum).map_err(|e| format!("Invalid quantum value: {}", e))?;

    let divisor = Decimal::from(10u64.pow(decimals));
    Ok(quantum_value / divisor)
}

/// Format price with proper decimals
pub fn format_price(price: Decimal) -> String {
    price.to_string()
}

/// Parse price from string
pub fn parse_price(price_str: &str) -> Result<Decimal, String> {
    Decimal::from_str_exact(price_str).map_err(|e| format!("Invalid price: {}", e))
}

/// Generate random resource bounds for Starknet transactions
pub fn random_resource_bounds() -> starknet_core::types::ResourceBoundsMapping {
    use starknet_core::types::{ResourceBounds, ResourceBoundsMapping};

    ResourceBoundsMapping {
        l1_gas: ResourceBounds {
            max_amount: 50000,
            max_price_per_unit: 100000000000,
        },
        l2_gas: ResourceBounds {
            max_amount: 0,
            max_price_per_unit: 0,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_quantum() {
        let value = Decimal::from_str_exact("1.5").unwrap();
        let quantum = to_quantum(value, 8);
        assert_eq!(quantum, "150000000");
    }

    #[test]
    fn test_from_quantum() {
        let quantum = "150000000";
        let value = from_quantum(quantum, 8).unwrap();
        assert_eq!(value, Decimal::from_str_exact("1.5").unwrap());
    }
}
