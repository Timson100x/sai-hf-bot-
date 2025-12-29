use crate::config::Config;
use crate::event_loop::TradeOpportunity;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeResult {
    pub success: bool,
    pub signature: Option<String>,
    pub amount_in: f64,
    pub amount_out: f64,
    pub actual_profit: f64,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JupiterQuote {
    pub input_mint: String,
    pub output_mint: String,
    pub in_amount: String,
    pub out_amount: String,
    pub other_amount_threshold: String,
    pub swap_mode: String,
    pub slippage_bps: u16,
    pub price_impact_pct: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JupiterSwapRequest {
    pub quote_response: JupiterQuote,
    pub user_public_key: String,
    pub wrap_unwrap_sol: bool,
}

pub struct Sniper {
    config: Arc<Config>,
    http_client: reqwest::Client,
}

impl Sniper {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            http_client: reqwest::Client::new(),
        }
    }

    /// Execute a trade based on an opportunity
    pub async fn execute_trade(&self, opportunity: &TradeOpportunity) -> Result<TradeResult> {
        info!(
            "Attempting to execute trade: {} -> {}",
            opportunity.token_in, opportunity.token_out
        );

        // Step 1: Get Jupiter quote
        let quote = match self.get_jupiter_quote(opportunity).await {
            Ok(q) => q,
            Err(e) => {
                error!("Failed to get Jupiter quote: {}", e);
                return Ok(TradeResult {
                    success: false,
                    signature: None,
                    amount_in: opportunity.amount_in,
                    amount_out: 0.0,
                    actual_profit: 0.0,
                    error: Some(format!("Quote failed: {}", e)),
                });
            }
        };

        // Step 2: Verify the quote meets our profit threshold
        if !self.verify_profitability(&quote, opportunity) {
            warn!("Trade no longer profitable after quote");
            return Ok(TradeResult {
                success: false,
                signature: None,
                amount_in: opportunity.amount_in,
                amount_out: 0.0,
                actual_profit: 0.0,
                error: Some("Insufficient profit after quote".to_string()),
            });
        }

        // Step 3: Execute the swap via Jupiter
        let result = self.execute_jupiter_swap(&quote).await?;

        info!("Trade executed successfully: {:?}", result);
        Ok(result)
    }

    /// Get a quote from Jupiter aggregator
    async fn get_jupiter_quote(&self, opportunity: &TradeOpportunity) -> Result<JupiterQuote> {
        let url = format!(
            "{}/quote?inputMint={}&outputMint={}&amount={}&slippageBps={}",
            self.config.jupiter_api_url,
            opportunity.token_in,
            opportunity.token_out,
            (opportunity.amount_in * 1e9) as u64, // Convert to lamports
            self.config.slippage_bps
        );

        let response = self.http_client.get(&url).send().await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    let quote: JupiterQuote = resp.json().await?;
                    Ok(quote)
                } else {
                    warn!("Jupiter API returned error status: {}", resp.status());
                    // Return a placeholder quote for development
                    Ok(self.create_placeholder_quote(opportunity))
                }
            }
            Err(e) => {
                warn!("Jupiter API call failed: {} (using placeholder)", e);
                Ok(self.create_placeholder_quote(opportunity))
            }
        }
    }

    /// Create a placeholder quote for development
    fn create_placeholder_quote(&self, opportunity: &TradeOpportunity) -> JupiterQuote {
        JupiterQuote {
            input_mint: opportunity.token_in.clone(),
            output_mint: opportunity.token_out.clone(),
            in_amount: ((opportunity.amount_in * 1e9) as u64).to_string(),
            out_amount: ((opportunity.expected_amount_out * 1e9) as u64).to_string(),
            other_amount_threshold: "0".to_string(),
            swap_mode: "ExactIn".to_string(),
            slippage_bps: self.config.slippage_bps,
            price_impact_pct: "0.1".to_string(),
        }
    }

    /// Verify that the quote still meets profitability requirements
    fn verify_profitability(&self, quote: &JupiterQuote, opportunity: &TradeOpportunity) -> bool {
        let out_amount: f64 = quote.out_amount.parse().unwrap_or(0.0) / 1e9;
        let in_amount: f64 = quote.in_amount.parse().unwrap_or(1.0) / 1e9;
        
        let profit = out_amount - in_amount;
        
        profit >= self.config.min_profit_threshold && profit >= opportunity.expected_profit * 0.8
    }

    /// Execute a swap transaction via Jupiter
    async fn execute_jupiter_swap(&self, quote: &JupiterQuote) -> Result<TradeResult> {
        // In production, this would:
        // 1. Create a swap transaction using Jupiter's /swap endpoint
        // 2. Sign the transaction with the wallet private key
        // 3. Submit the transaction to Solana network
        // 4. Wait for confirmation
        
        // For now, return a placeholder result
        warn!("Jupiter swap execution is a placeholder - no actual transaction sent");
        
        let in_amount: f64 = quote.in_amount.parse().unwrap_or(0.0) / 1e9;
        let out_amount: f64 = quote.out_amount.parse().unwrap_or(0.0) / 1e9;
        
        Ok(TradeResult {
            success: true,
            signature: Some("placeholder_signature".to_string()),
            amount_in: in_amount,
            amount_out: out_amount,
            actual_profit: out_amount - in_amount,
            error: None,
        })
    }

    /// Execute trades in a sniping loop
    pub async fn snipe_loop(&self, opportunities_rx: tokio::sync::mpsc::Receiver<TradeOpportunity>) {
        info!("Starting sniper loop");
        
        let mut rx = opportunities_rx;
        
        while let Some(opportunity) = rx.recv().await {
            if opportunity.expected_profit >= self.config.min_profit_threshold {
                match self.execute_trade(&opportunity).await {
                    Ok(result) => {
                        if result.success {
                            info!(
                                "Trade successful! Profit: {} SOL, Signature: {:?}",
                                result.actual_profit, result.signature
                            );
                        } else {
                            warn!("Trade failed: {:?}", result.error);
                        }
                    }
                    Err(e) => {
                        error!("Error executing trade: {}", e);
                    }
                }
            }
        }
    }

    /// Analyze opportunity using Gemini AI (placeholder)
    pub async fn analyze_with_ai(&self, opportunity: &TradeOpportunity) -> Result<bool> {
        // Placeholder for Gemini AI Studio integration
        // In production, this would send opportunity data to Gemini AI
        // and receive recommendations on whether to execute the trade
        
        warn!("Gemini AI analysis is a placeholder - defaulting to true");
        
        // Simple heuristic for now
        Ok(opportunity.expected_profit > self.config.min_profit_threshold)
    }
}
