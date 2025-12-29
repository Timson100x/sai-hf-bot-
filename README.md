# SAI-HF Bot - Solana AI High-Frequency Trading Bot

A modular, scalable Rust-based high-frequency trading bot for Solana blockchain with AI integration capabilities.

## 🚀 Overview

SAI-HF Bot is designed to monitor liquidity pools on Solana, execute trades with minimal latency, and integrate AI-powered decision making for optimal trading strategies.

## 📁 Project Structure

```
sai-hf-bot/
├── Cargo.toml                # Rust Project Configuration
├── .env.example              # Example environment configuration
├── README.md                 # This file
├── dashboard/                # Web-based monitoring dashboard
│   ├── index.html            # Dashboard UI
│   ├── app.js                # WebSocket client for real-time updates
│   ├── styles.css            # Dashboard styles
├── src/                      # Core Rust modules
│   ├── main.rs               # Application entry point
│   ├── config.rs             # Configuration management
│   ├── event_loop.rs         # Async event loop for pool monitoring
│   ├── sniper.rs             # Trade execution logic
│   ├── ai_model.rs           # AI integration placeholder
│   ├── utils.rs              # Utility functions
└── tests/                    # Test suite
    ├── integration_tests.rs  # Integration tests
```

## ✨ Features

### Current Features (v0.1.0)
- **Liquidity Pool Monitoring**: Simulated monitoring of new pools on Solana
- **Transaction Execution**: Core trading logic with configurable slippage
- **Modular Architecture**: Clean separation of concerns for easy extension
- **Configuration Management**: Environment-based configuration
- **Dashboard Placeholder**: Ready for real-time monitoring integration

### Planned Features
- AI-powered trade decision making
- Machine learning model integration
- Advanced risk management
- Multi-pool parallel monitoring
- WebSocket dashboard with real-time updates

## 🔧 Setup

### Prerequisites
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- A Solana wallet with private key
- Access to Solana RPC endpoint

### Installation

1. Clone the repository:
```bash
git clone https://github.com/Timson100x/sai-hf-bot-.git
cd sai-hf-bot-
```

2. Copy the example environment file:
```bash
cp .env.example .env
```

3. Edit `.env` and configure your settings:
```bash
nano .env  # or use your preferred editor
```

4. Build the project:
```bash
cargo build --release
```

## 🚦 Usage

### Running the Bot

```bash
cargo run --release
```

### Running Tests

```bash
cargo test
```

### Running Integration Tests

```bash
cargo test --test integration_tests
```

## ⚙️ Configuration

Edit the `.env` file to customize bot behavior:

| Variable | Description | Default |
|----------|-------------|---------|
| `SOLANA_RPC_URL` | Solana RPC endpoint | https://api.mainnet-beta.solana.com |
| `TRADE_SOL_AMOUNT` | Amount of SOL per trade | 0.1 |
| `SLIPPAGE_BPS` | Slippage tolerance in basis points | 50 (0.5%) |
| `POOL_CHECK_INTERVAL_MS` | Pool check frequency in milliseconds | 1000 |
| `LOG_LEVEL` | Logging level (trace, debug, info, warn, error) | info |

## 🏗️ Architecture

### Core Modules

- **config.rs**: Loads and manages environment variables and configuration
- **event_loop.rs**: Async event loop that continuously monitors liquidity pools
- **sniper.rs**: Executes trades with configurable parameters
- **ai_model.rs**: Placeholder for future AI/ML integrations
- **utils.rs**: Helper functions used across the application

### Event Flow

```
main.rs → config.rs (load config)
       → event_loop.rs (start monitoring)
       → sniper.rs (execute trades when opportunities detected)
       → ai_model.rs (future: AI decision making)
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## ⚠️ Disclaimer

This bot is for educational purposes only. Trading cryptocurrencies carries risk. Always test thoroughly on devnet/testnet before using on mainnet. Never commit private keys to version control.

## 📄 License

This project is licensed under the MIT License.

## 🔗 Links

- [Solana Documentation](https://docs.solana.com/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Documentation](https://tokio.rs/)

