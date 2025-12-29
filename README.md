# 🚀 Solana High-Frequency Trading Bot

A high-performance, production-ready Solana trading bot built in Rust. This bot monitors liquidity pools in real-time, detects arbitrage opportunities, and executes trades with minimal slippage using Jupiter aggregator.

## ✨ Features

### Core Trading Features
- **High-Frequency Trading Module**: Advanced sniping logic to track price differences and execute trades with minimal slippage
- **Real-time Pool Monitoring**: Continuous liquidity pool monitoring using Moralis APIs
- **Webhook Integration**: Real-time updates via Helius webhooks for instant pool change detection
- **Jupiter Integration**: Optimized trading routes for cost-efficient swaps
- **AI-Powered Analysis**: Gemini AI Studio integration for intelligent trade execution decisions

### Technical Features
- **Async Architecture**: Built on Tokio for high-performance concurrent operations
- **RESTful API**: Full-featured API for monitoring and control
- **Web Dashboard**: Real-time monitoring dashboard with live updates
- **Docker Support**: Production-ready containerization for easy VPS deployment
- **Comprehensive Logging**: Structured logging with tracing for debugging and monitoring

## 📋 Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Docker (optional, for containerized deployment)
- API Keys:
  - Helius API key ([helius.dev](https://helius.dev))
  - Moralis API key ([moralis.io](https://moralis.io))
  - Gemini AI API key ([ai.google.dev](https://ai.google.dev))
- Solana wallet with SOL for trading

## 🚀 Quick Start

### 1. Clone the Repository

```bash
git clone https://github.com/Timson100x/sai-hf-bot-.git
cd sai-hf-bot-
```

### 2. Configure Environment Variables

```bash
cp .env.example .env
```

Edit `.env` and add your API keys and configuration:

```env
HELIUS_API_KEY=your_helius_api_key_here
MORALIS_API_KEY=your_moralis_api_key_here
JUPITER_API_URL=https://quote-api.jup.ag/v6
GEMINI_API_KEY=your_gemini_api_key_here
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
WALLET_PRIVATE_KEY=your_wallet_private_key_here
SLIPPAGE_BPS=50
MIN_PROFIT_THRESHOLD=0.01
MAX_POSITION_SIZE_SOL=1.0
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
```

⚠️ **Security Warning**: Never commit your `.env` file or share your private keys!

### 3. Build the Project

```bash
cargo build --release
```

### 4. Run the Bot

```bash
cargo run --release
```

The bot will start and be accessible at:
- Dashboard: `http://localhost:8080`
- API: `http://localhost:8080/api`

## 🐳 Docker Deployment

### Build Docker Image

```bash
docker build -t sai-hf-bot .
```

### Run with Docker

```bash
docker run -d \
  --name sai-hf-bot \
  --env-file .env \
  -p 8080:8080 \
  sai-hf-bot
```

### Run with Docker Compose

Create a `docker-compose.yml`:

```yaml
version: '3.8'

services:
  bot:
    build: .
    container_name: sai-hf-bot
    env_file:
      - .env
    ports:
      - "8080:8080"
    restart: unless-stopped
    volumes:
      - ./logs:/app/logs
```

Then run:

```bash
docker-compose up -d
```

## 🖥️ VPS Deployment (Contabo)

### 1. Connect to Your VPS

```bash
ssh root@your-vps-ip
```

### 2. Install Dependencies

```bash
# Update system
apt update && apt upgrade -y

# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh

# Install Docker Compose
apt install docker-compose -y
```

### 3. Deploy the Bot

```bash
# Clone repository
git clone https://github.com/Timson100x/sai-hf-bot-.git
cd sai-hf-bot-

# Configure environment
cp .env.example .env
nano .env  # Edit with your configuration

# Build and run
docker-compose up -d

# View logs
docker-compose logs -f
```

### 4. Setup Firewall

```bash
ufw allow 22/tcp   # SSH
ufw allow 8080/tcp # Bot dashboard
ufw enable
```

### 5. Monitor the Bot

```bash
# Check status
docker-compose ps

# View logs
docker-compose logs -f bot

# Restart if needed
docker-compose restart bot
```

## 📊 API Endpoints

### Health Check
```bash
GET /api/health
```

### Bot Status
```bash
GET /api/status
```

### Get Monitored Pools
```bash
GET /api/pools
```

### Get Trading Opportunities
```bash
GET /api/opportunities
```

### Get Trade History
```bash
GET /api/trades
```

### Execute Manual Trade
```bash
POST /api/execute
Content-Type: application/json

{
  "pool_address": "...",
  "token_in": "...",
  "token_out": "...",
  "amount_in": 1.0,
  "expected_amount_out": 1.05,
  "expected_profit": 0.05,
  "timestamp": 1234567890
}
```

## 📁 Project Structure

```
sai-hf-bot/
├── Cargo.toml              # Rust dependencies
├── .env.example            # Environment configuration template
├── README.md               # This file
├── Dockerfile              # Docker configuration
├── dashboard/              # Web dashboard
│   ├── index.html         # Dashboard HTML
│   ├── app.js             # Dashboard JavaScript
│   └── styles.css         # Dashboard styles
├── src/                   # Source code
│   ├── main.rs           # Application entry point
│   ├── config.rs         # Configuration management
│   ├── event_loop.rs     # Pool monitoring & event handling
│   ├── sniper.rs         # Trade execution logic
│   └── utils.rs          # Utility functions
└── tests/                # Integration tests
    └── integration_tests.rs
```

## 🔧 Configuration

### Trading Parameters

- **SLIPPAGE_BPS**: Maximum acceptable slippage in basis points (50 = 0.5%)
- **MIN_PROFIT_THRESHOLD**: Minimum profit in SOL to execute a trade
- **MAX_POSITION_SIZE_SOL**: Maximum amount of SOL to use per trade

### API Configuration

- **HELIUS_API_KEY**: For webhook-based pool updates
- **MORALIS_API_KEY**: For liquidity pool data
- **JUPITER_API_URL**: Jupiter aggregator endpoint
- **GEMINI_API_KEY**: For AI-powered trade analysis

### Network Configuration

- **SOLANA_RPC_URL**: Solana RPC endpoint (use private RPC for production)
- **SOLANA_WS_URL**: Solana WebSocket endpoint

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

## 📝 Development

### Build for Development

```bash
cargo build
cargo run
```

### Enable Debug Logging

```bash
RUST_LOG=debug cargo run
```

### Format Code

```bash
cargo fmt
```

### Lint Code

```bash
cargo clippy
```

## ⚠️ Important Warnings

1. **Security**: 
   - Never commit your `.env` file
   - Keep your private keys secure
   - Use a dedicated trading wallet with limited funds for testing

2. **Testing**:
   - Always test thoroughly on devnet/testnet first
   - Start with small amounts on mainnet
   - Monitor the bot closely during initial runs

3. **RPC Limits**:
   - Free RPC endpoints have rate limits
   - Consider using paid RPC services (Helius, QuickNode) for production
   - Monitor your RPC usage to avoid throttling

4. **Market Risks**:
   - HFT bots can lose money
   - Market conditions change rapidly
   - Always set appropriate risk limits

## 🛠️ Troubleshooting

### Bot Won't Start
- Check that all API keys are correctly set in `.env`
- Verify Rust is installed: `cargo --version`
- Check logs for specific error messages

### No Opportunities Detected
- Verify API keys are valid
- Check RPC endpoint is responsive
- Adjust `MIN_PROFIT_THRESHOLD` if needed

### Trades Failing
- Check wallet has sufficient SOL
- Verify slippage settings
- Check Jupiter API is accessible

### High CPU Usage
- This is normal for HFT operations
- Consider reducing monitoring frequency
- Optimize pool filtering

## 📚 Additional Resources

- [Solana Documentation](https://docs.solana.com/)
- [Jupiter Aggregator](https://jup.ag/)
- [Helius Developer Docs](https://docs.helius.dev/)
- [Moralis Solana API](https://docs.moralis.io/web3-data-api/solana)

## 📄 License

MIT License - see LICENSE file for details

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## ⚖️ Disclaimer

This software is for educational purposes only. Use at your own risk. The authors are not responsible for any financial losses incurred while using this bot. Always do your own research and never invest more than you can afford to lose.

---

**Built with ❤️ for the Solana ecosystem**
