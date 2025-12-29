use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub helius_api_key: String,
    pub moralis_api_key: String,
    pub jupiter_api_url: String,
    pub gemini_api_key: String,
    pub solana_rpc_url: String,
    pub solana_ws_url: String,
    pub wallet_private_key: String,
    pub slippage_bps: u16,
    pub min_profit_threshold: f64,
    pub max_position_size_sol: f64,
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        Ok(Config {
            helius_api_key: env::var("HELIUS_API_KEY")
                .context("HELIUS_API_KEY must be set")?,
            moralis_api_key: env::var("MORALIS_API_KEY")
                .context("MORALIS_API_KEY must be set")?,
            jupiter_api_url: env::var("JUPITER_API_URL")
                .unwrap_or_else(|_| "https://quote-api.jup.ag/v6".to_string()),
            gemini_api_key: env::var("GEMINI_API_KEY")
                .context("GEMINI_API_KEY must be set")?,
            solana_rpc_url: env::var("SOLANA_RPC_URL")
                .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string()),
            solana_ws_url: env::var("SOLANA_WS_URL")
                .unwrap_or_else(|_| "wss://api.mainnet-beta.solana.com".to_string()),
            wallet_private_key: env::var("WALLET_PRIVATE_KEY")
                .context("WALLET_PRIVATE_KEY must be set")?,
            slippage_bps: env::var("SLIPPAGE_BPS")
                .unwrap_or_else(|_| "50".to_string())
                .parse()
                .context("SLIPPAGE_BPS must be a valid u16")?,
            min_profit_threshold: env::var("MIN_PROFIT_THRESHOLD")
                .unwrap_or_else(|_| "0.01".to_string())
                .parse()
                .context("MIN_PROFIT_THRESHOLD must be a valid f64")?,
            max_position_size_sol: env::var("MAX_POSITION_SIZE_SOL")
                .unwrap_or_else(|_| "1.0".to_string())
                .parse()
                .context("MAX_POSITION_SIZE_SOL must be a valid f64")?,
            server_host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .context("SERVER_PORT must be a valid u16")?,
        })
    }

    /// Validate configuration values
    pub fn validate(&self) -> Result<()> {
        if self.slippage_bps > 10000 {
            anyhow::bail!("SLIPPAGE_BPS must be <= 10000 (100%)");
        }
        if self.min_profit_threshold <= 0.0 {
            anyhow::bail!("MIN_PROFIT_THRESHOLD must be positive");
        }
        if self.max_position_size_sol <= 0.0 {
            anyhow::bail!("MAX_POSITION_SIZE_SOL must be positive");
        }
        Ok(())
    }
}
