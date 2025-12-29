mod config;
mod event_loop;
mod sniper;
mod utils;

use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};
use tracing::{error, info};

use config::Config;
use event_loop::EventLoop;
use sniper::Sniper;
use utils::HealthCheck;

#[derive(Clone)]
struct AppState {
    config: Arc<Config>,
    event_loop: Arc<EventLoop>,
    sniper: Arc<Sniper>,
    trade_history: Arc<RwLock<Vec<sniper::TradeResult>>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("Starting Solana High-Frequency Trading Bot");

    // Load configuration
    let config = Arc::new(Config::from_env()?);
    config.validate()?;

    info!("Configuration loaded successfully");
    info!("Server will start on {}:{}", config.server_host, config.server_port);

    // Initialize components
    let event_loop = Arc::new(EventLoop::new(config.clone()));
    let sniper = Arc::new(Sniper::new(config.clone()));

    // Create app state
    let state = AppState {
        config: config.clone(),
        event_loop: event_loop.clone(),
        sniper: sniper.clone(),
        trade_history: Arc::new(RwLock::new(Vec::new())),
    };

    // Start background tasks
    let event_loop_handle = {
        let event_loop = event_loop.clone();
        tokio::spawn(async move {
            if let Err(e) = event_loop.run().await {
                error!("Event loop error: {}", e);
            }
        })
    };

    info!("Event loop started");

    // Build API routes
    let api_routes = Router::new()
        .route("/health", get(health_check))
        .route("/status", get(get_status))
        .route("/pools", get(get_pools))
        .route("/opportunities", get(get_opportunities))
        .route("/trades", get(get_trades))
        .route("/execute", post(execute_trade))
        .with_state(state.clone());

    // Serve dashboard static files
    let dashboard_service = ServeDir::new("dashboard")
        .not_found_service(ServeFile::new("dashboard/index.html"));

    // Build application
    let app = Router::new()
        .nest("/api", api_routes)
        .fallback_service(dashboard_service)
        .layer(CorsLayer::permissive());

    // Start server
    let addr = format!("{}:{}", config.server_host, config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    info!("Server listening on {}", addr);
    info!("Dashboard available at http://{}", addr);
    info!("API available at http://{}/api", addr);

    // Serve the application
    tokio::select! {
        result = axum::serve(listener, app) => {
            result?;
        }
        result = event_loop_handle => {
            let _ = result.map_err(|e| anyhow::anyhow!("Event loop task error: {}", e))?;
        }
    }

    Ok(())
}

// API Handlers

async fn health_check() -> Json<HealthCheck> {
    Json(HealthCheck::new())
}

async fn get_status(State(state): State<AppState>) -> impl IntoResponse {
    let status = serde_json::json!({
        "bot_status": "running",
        "slippage_bps": state.config.slippage_bps,
        "min_profit_threshold": state.config.min_profit_threshold,
        "max_position_size_sol": state.config.max_position_size_sol,
    });

    Json(status)
}

async fn get_pools(State(state): State<AppState>) -> impl IntoResponse {
    let pools = state.event_loop.get_pools().await;
    Json(pools)
}

async fn get_opportunities(State(state): State<AppState>) -> impl IntoResponse {
    let opportunities = state.event_loop.get_opportunities().await;
    Json(opportunities)
}

async fn get_trades(State(state): State<AppState>) -> impl IntoResponse {
    let trades = state.trade_history.read().await;
    Json(trades.clone())
}

async fn execute_trade(
    State(state): State<AppState>,
    Json(opportunity): Json<event_loop::TradeOpportunity>,
) -> impl IntoResponse {
    info!("Received manual trade execution request");

    match state.sniper.execute_trade(&opportunity).await {
        Ok(result) => {
            if result.success {
                // Store in trade history
                let mut history = state.trade_history.write().await;
                history.push(result.clone());
                
                (StatusCode::OK, Json(result))
            } else {
                (StatusCode::BAD_REQUEST, Json(result))
            }
        }
        Err(e) => {
            error!("Trade execution error: {}", e);
            let error_result = sniper::TradeResult {
                success: false,
                signature: None,
                amount_in: opportunity.amount_in,
                amount_out: 0.0,
                actual_profit: 0.0,
                error: Some(e.to_string()),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_result))
        }
    }
}
