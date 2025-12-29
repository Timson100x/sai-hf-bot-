use anyhow::{Context, Result};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

/// Configuration structure for the SAI-HF Bot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    // Solana Configuration
    pub solana_rpc_url: String,
    pub solana_ws_url: String,
    
    // Trading Configuration
    pub trade_sol_amount: f64,
    pub slippage_bps: u16,
    pub max_slippage_bps: u16,
    
    // Bot Configuration
    pub pool_check_interval_ms: u64,
    pub max_retries: u32,
    
    // Wallet Configuration
    pub wallet_private_key: String,
    
    // Dashboard Configuration
    pub dashboard_port: u16,
    pub enable_dashboard: bool,
    
    // Logging
    pub log_level: String,
}

impl Config {
    /// Load configuration from environment variables
    pub fn load() -> Result<Self> {
        // Load .env file if it exists
        dotenv().ok();
        
        let config = Config {
            solana_rpc_url: env::var("SOLANA_RPC_URL")
                .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string()),
            solana_ws_url: env::var("SOLANA_WS_URL")
                .unwrap_or_else(|_| "wss://api.mainnet-beta.solana.com".to_string()),
            
            trade_sol_amount: env::var("TRADE_SOL_AMOUNT")
                .unwrap_or_else(|_| "0.1".to_string())
                .parse()
                .context("Failed to parse TRADE_SOL_AMOUNT")?,
            
            slippage_bps: env::var("SLIPPAGE_BPS")
                .unwrap_or_else(|_| "50".to_string())
                .parse()
                .context("Failed to parse SLIPPAGE_BPS")?,
            
            max_slippage_bps: env::var("MAX_SLIPPAGE_BPS")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .context("Failed to parse MAX_SLIPPAGE_BPS")?,
            
            pool_check_interval_ms: env::var("POOL_CHECK_INTERVAL_MS")
                .unwrap_or_else(|_| "1000".to_string())
                .parse()
                .context("Failed to parse POOL_CHECK_INTERVAL_MS")?,
            
            max_retries: env::var("MAX_RETRIES")
                .unwrap_or_else(|_| "3".to_string())
                .parse()
                .context("Failed to parse MAX_RETRIES")?,
            
            wallet_private_key: env::var("WALLET_PRIVATE_KEY")
                .unwrap_or_else(|_| "your_private_key_here".to_string()),
            
            dashboard_port: env::var("DASHBOARD_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .context("Failed to parse DASHBOARD_PORT")?,
            
            enable_dashboard: env::var("ENABLE_DASHBOARD")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .context("Failed to parse ENABLE_DASHBOARD")?,
            
            log_level: env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "info".to_string()),
        };
        
        // Validate configuration
        config.validate()?;
        
        Ok(config)
    }
    
    /// Validate configuration values
    fn validate(&self) -> Result<()> {
        if self.trade_sol_amount <= 0.0 {
            anyhow::bail!("TRADE_SOL_AMOUNT must be positive");
        }
        
        if self.slippage_bps > self.max_slippage_bps {
            anyhow::bail!("SLIPPAGE_BPS cannot exceed MAX_SLIPPAGE_BPS");
        }
        
        if self.pool_check_interval_ms == 0 {
            anyhow::bail!("POOL_CHECK_INTERVAL_MS must be greater than 0");
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_validation() {
        let mut config = Config {
            solana_rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
            solana_ws_url: "wss://api.mainnet-beta.solana.com".to_string(),
            trade_sol_amount: 0.1,
            slippage_bps: 50,
            max_slippage_bps: 100,
            pool_check_interval_ms: 1000,
            max_retries: 3,
            wallet_private_key: "test".to_string(),
            dashboard_port: 8080,
            enable_dashboard: true,
            log_level: "info".to_string(),
        };
        
        assert!(config.validate().is_ok());
        
        // Test invalid trade amount
        config.trade_sol_amount = -0.1;
        assert!(config.validate().is_err());
        
        // Reset and test invalid slippage
        config.trade_sol_amount = 0.1;
        config.slippage_bps = 150;
        assert!(config.validate().is_err());
    }
}
