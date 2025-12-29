use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Format a number as SOL with proper decimals
pub fn format_sol(lamports: u64) -> String {
    format!("{:.4} SOL", lamports as f64 / 1e9)
}

/// Calculate slippage amount
pub fn calculate_slippage(amount: f64, slippage_bps: u16) -> f64 {
    amount * (slippage_bps as f64 / 10000.0)
}

/// Validate Solana address format
pub fn is_valid_solana_address(address: &str) -> bool {
    // Basic validation - addresses should be 32-44 characters base58
    address.len() >= 32 && address.len() <= 44 && address.chars().all(|c| c.is_alphanumeric())
}

/// Calculate price impact percentage
pub fn calculate_price_impact(amount_in: f64, liquidity: f64) -> f64 {
    if liquidity <= 0.0 {
        return 100.0;
    }
    (amount_in / liquidity) * 100.0
}

/// Format timestamp to human readable string
pub fn format_timestamp(timestamp: i64) -> String {
    match chrono::DateTime::from_timestamp(timestamp, 0) {
        Some(dt) => dt.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
        None => "Invalid timestamp".to_string(),
    }
}

/// Calculate profit percentage
pub fn calculate_profit_percentage(amount_in: f64, amount_out: f64) -> f64 {
    if amount_in <= 0.0 {
        return 0.0;
    }
    ((amount_out - amount_in) / amount_in) * 100.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub status: String,
    pub timestamp: i64,
    pub version: String,
}

impl HealthCheck {
    pub fn new() -> Self {
        Self {
            status: "healthy".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

impl Default for HealthCheck {
    fn default() -> Self {
        Self::new()
    }
}

/// Retry an async operation with exponential backoff
pub async fn retry_with_backoff<F, Fut, T>(
    mut operation: F,
    max_attempts: u32,
    initial_delay_ms: u64,
) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let mut attempts = 0;
    let mut delay = initial_delay_ms;

    loop {
        attempts += 1;
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if attempts >= max_attempts {
                    return Err(e);
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
                delay *= 2; // Exponential backoff
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_sol() {
        assert_eq!(format_sol(1_000_000_000), "1.0000 SOL");
        assert_eq!(format_sol(500_000_000), "0.5000 SOL");
    }

    #[test]
    fn test_calculate_slippage() {
        assert_eq!(calculate_slippage(100.0, 50), 0.5); // 0.5%
        assert_eq!(calculate_slippage(100.0, 100), 1.0); // 1%
    }

    #[test]
    fn test_is_valid_solana_address() {
        assert!(is_valid_solana_address("4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R"));
        assert!(!is_valid_solana_address("invalid"));
        assert!(!is_valid_solana_address(""));
    }

    #[test]
    fn test_calculate_price_impact() {
        assert_eq!(calculate_price_impact(10.0, 100.0), 10.0);
        assert_eq!(calculate_price_impact(1.0, 100.0), 1.0);
    }

    #[test]
    fn test_calculate_profit_percentage() {
        assert_eq!(calculate_profit_percentage(100.0, 110.0), 10.0);
        assert_eq!(calculate_profit_percentage(100.0, 90.0), -10.0);
    }
}
