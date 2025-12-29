use anyhow::Result;
use log::{info, warn};
use std::time::Duration;
use tokio::time;

use crate::config::Config;
use crate::event_loop::LiquidityPool;
use crate::utils;

/// Sniper module for executing trades on new liquidity pools
pub struct Sniper {
    config: Config,
    retry_count: u32,
}

impl Sniper {
    /// Create a new Sniper instance
    pub fn new(config: Config) -> Self {
        Self {
            config,
            retry_count: 0,
        }
    }
    
    /// Execute a trade on the given liquidity pool
    pub async fn execute_trade(&mut self, pool: &LiquidityPool) -> Result<String> {
        info!("Preparing trade for pool: {}", pool.address);
        
        // Validate trade parameters
        self.validate_trade(pool)?;
        
        // Calculate optimal trade parameters
        let trade_params = self.calculate_trade_params(pool)?;
        info!("Trade params: amount={} SOL, slippage={} bps", 
              trade_params.amount, trade_params.slippage);
        
        // Execute trade with retries
        for attempt in 1..=self.config.max_retries {
            match self.attempt_trade(pool, &trade_params).await {
                Ok(signature) => {
                    self.retry_count = 0;
                    return Ok(signature);
                },
                Err(e) if attempt < self.config.max_retries => {
                    warn!("Trade attempt {} failed: {}, retrying...", attempt, e);
                    time::sleep(Duration::from_millis(100 * attempt as u64)).await;
                    continue;
                },
                Err(e) => {
                    anyhow::bail!("Trade failed after {} attempts: {}", self.config.max_retries, e);
                }
            }
        }
        
        anyhow::bail!("Trade execution failed")
    }
    
    /// Validate trade parameters
    fn validate_trade(&self, pool: &LiquidityPool) -> Result<()> {
        if pool.liquidity < 1000.0 {
            anyhow::bail!("Liquidity too low: {}", pool.liquidity);
        }
        
        if self.config.trade_sol_amount <= 0.0 {
            anyhow::bail!("Invalid trade amount: {}", self.config.trade_sol_amount);
        }
        
        Ok(())
    }
    
    /// Calculate trade parameters based on pool and config
    fn calculate_trade_params(&self, pool: &LiquidityPool) -> Result<TradeParams> {
        let amount = self.config.trade_sol_amount;
        let mut slippage = self.config.slippage_bps;
        
        // Adjust slippage based on pool liquidity
        if pool.liquidity < 5000.0 {
            slippage = slippage.min(self.config.max_slippage_bps);
            warn!("Adjusted slippage to {} bps for low liquidity pool", slippage);
        }
        
        Ok(TradeParams {
            amount,
            slippage,
            min_output: self.calculate_min_output(amount, slippage),
        })
    }
    
    /// Calculate minimum output tokens based on slippage
    fn calculate_min_output(&self, amount: f64, slippage_bps: u16) -> f64 {
        let slippage_factor = 1.0 - (slippage_bps as f64 / 10000.0);
        amount * slippage_factor
    }
    
    /// Attempt to execute the trade
    async fn attempt_trade(&self, pool: &LiquidityPool, params: &TradeParams) -> Result<String> {
        // TODO: Implement actual Solana transaction execution
        // For now, simulate trade execution
        
        info!("Simulating trade execution...");
        info!("  Pool: {}", pool.address);
        info!("  Amount: {} SOL", params.amount);
        info!("  Slippage: {} bps", params.slippage);
        info!("  Min output: {}", params.min_output);
        
        // Simulate network delay
        time::sleep(Duration::from_millis(100)).await;
        
        // Simulate success/failure (90% success rate)
        if utils::random_bool(0.9) {
            let signature = utils::generate_random_signature();
            Ok(signature)
        } else {
            anyhow::bail!("Simulated transaction failure")
        }
    }
}

/// Trade parameters structure
#[derive(Debug, Clone)]
struct TradeParams {
    amount: f64,
    slippage: u16,
    min_output: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_config() -> Config {
        Config {
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
        }
    }
    
    fn create_test_pool() -> LiquidityPool {
        LiquidityPool {
            address: "test_pool".to_string(),
            token_a: "token_a".to_string(),
            token_b: "token_b".to_string(),
            liquidity: 10000.0,
            volume_24h: 5000.0,
            is_new: true,
        }
    }
    
    #[test]
    fn test_sniper_creation() {
        let config = create_test_config();
        let sniper = Sniper::new(config);
        assert_eq!(sniper.retry_count, 0);
    }
    
    #[test]
    fn test_validate_trade() {
        let config = create_test_config();
        let sniper = Sniper::new(config);
        let pool = create_test_pool();
        
        assert!(sniper.validate_trade(&pool).is_ok());
        
        let low_liquidity_pool = LiquidityPool {
            liquidity: 500.0,
            ..pool
        };
        assert!(sniper.validate_trade(&low_liquidity_pool).is_err());
    }
    
    #[test]
    fn test_calculate_min_output() {
        let config = create_test_config();
        let sniper = Sniper::new(config);
        
        let min_output = sniper.calculate_min_output(1.0, 50); // 50 bps = 0.5%
        assert!((min_output - 0.995).abs() < 0.001);
    }
}
