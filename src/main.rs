use anyhow::Result;
use log::info;

mod config;
mod event_loop;
mod sniper;
mod ai_model;
mod utils;

use config::Config;
use event_loop::EventLoop;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();
    
    info!("🚀 Starting SAI-HF Bot...");
    
    // Load configuration
    let config = Config::load()?;
    info!("✓ Configuration loaded successfully");
    info!("  - RPC URL: {}", config.solana_rpc_url);
    info!("  - Trade Amount: {} SOL", config.trade_sol_amount);
    info!("  - Slippage: {} bps", config.slippage_bps);
    
    // Initialize the event loop
    let mut event_loop = EventLoop::new(config);
    
    // Start monitoring
    info!("👀 Starting liquidity pool monitoring...");
    event_loop.start().await?;
    
    info!("Bot stopped");
    Ok(())
}
