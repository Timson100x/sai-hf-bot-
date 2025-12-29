# Quick Start Guide

This guide will help you get the Solana HFT Bot running on your local machine in under 5 minutes.

## Prerequisites

- Rust 1.70 or higher (install from https://rustup.rs)
- Git

## Quick Setup

### 1. Clone the Repository

```bash
git clone https://github.com/Timson100x/sai-hf-bot-.git
cd sai-hf-bot-
```

### 2. Configure Environment Variables

Create a `.env` file from the template:

```bash
cp .env.example .env
```

Edit `.env` and add your API keys (minimum required for testing):

```bash
# Required API Keys
HELIUS_API_KEY=your_helius_api_key_here
MORALIS_API_KEY=your_moralis_api_key_here
GEMINI_API_KEY=your_gemini_api_key_here
WALLET_PRIVATE_KEY=your_wallet_private_key_here

# Optional - defaults are provided
JUPITER_API_URL=https://quote-api.jup.ag/v6
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
SLIPPAGE_BPS=50
MIN_PROFIT_THRESHOLD=0.01
MAX_POSITION_SIZE_SOL=1.0
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
```

### 3. Build and Run

```bash
# Build in release mode for optimal performance
cargo build --release

# Run the bot
cargo run --release
```

The bot will start and be available at:
- **Dashboard**: http://localhost:8080
- **API**: http://localhost:8080/api

## Testing the Bot

### Run Tests

```bash
cargo test
```

### Check Build

```bash
cargo build
```

### Format Code

```bash
cargo fmt
```

### Lint Code

```bash
cargo clippy
```

## API Endpoints

Once running, you can test the API:

### Health Check
```bash
curl http://localhost:8080/api/health
```

### Get Bot Status
```bash
curl http://localhost:8080/api/status
```

### Get Monitored Pools
```bash
curl http://localhost:8080/api/pools
```

### Get Trading Opportunities
```bash
curl http://localhost:8080/api/opportunities
```

### Get Trade History
```bash
curl http://localhost:8080/api/trades
```

## Development Mode

For development with auto-reload, you can use:

```bash
# Install cargo-watch (one-time)
cargo install cargo-watch

# Run with auto-reload
cargo watch -x run
```

## Troubleshooting

### "HELIUS_API_KEY must be set" error
- Make sure you created the `.env` file
- Verify all required API keys are set in `.env`

### Bot won't start
- Check Rust is installed: `cargo --version`
- Try rebuilding: `cargo clean && cargo build --release`

### Port already in use
- Change `SERVER_PORT` in `.env` to a different port (e.g., 8081)

## Next Steps

1. **Get API Keys**: Sign up for:
   - [Helius](https://helius.dev) - Solana RPC and webhooks
   - [Moralis](https://moralis.io) - Blockchain data
   - [Gemini AI](https://ai.google.dev) - AI analysis

2. **Configure Trading Parameters**: Adjust in `.env`:
   - `SLIPPAGE_BPS`: Slippage tolerance (50 = 0.5%)
   - `MIN_PROFIT_THRESHOLD`: Minimum profit to execute trades
   - `MAX_POSITION_SIZE_SOL`: Maximum SOL per trade

3. **Test on Devnet**: Before using mainnet:
   - Change `SOLANA_RPC_URL` to devnet
   - Use a devnet wallet for testing
   - Monitor the dashboard for activity

4. **Deploy to VPS**: See [README.md](README.md) for VPS deployment instructions

## Security Notes

⚠️ **Important**:
- Never commit your `.env` file
- Keep your private keys secure
- Start with small amounts for testing
- Use a dedicated trading wallet

## Support

For issues or questions:
- Check the [README.md](README.md) for detailed documentation
- Review the code comments for implementation details
- Open an issue on GitHub

---

Happy Trading! 🚀
