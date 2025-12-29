# Contributing to Solana HFT Bot

Thank you for your interest in contributing to the Solana HFT Bot! This document provides guidelines for contributing to the project.

## Development Setup

1. **Install Rust**: Get the latest stable Rust toolchain from [rustup.rs](https://rustup.rs)

2. **Clone the repository**:
   ```bash
   git clone https://github.com/Timson100x/sai-hf-bot-.git
   cd sai-hf-bot-
   ```

3. **Set up environment**:
   ```bash
   cp .env.example .env
   # Edit .env with your test API keys
   ```

4. **Build and test**:
   ```bash
   cargo build
   cargo test
   ```

## Code Style

- Follow Rust's official style guide
- Run `cargo fmt` before committing
- Run `cargo clippy` to catch common mistakes
- Add tests for new functionality
- Document public APIs with doc comments

## Commit Messages

- Use clear, descriptive commit messages
- Start with a verb in present tense (e.g., "Add", "Fix", "Update")
- Keep first line under 72 characters
- Add detailed description if needed

Example:
```
Add retry logic for Jupiter API calls

Implements exponential backoff for failed API calls to improve
reliability during network issues or API rate limiting.
```

## Pull Request Process

1. **Fork the repository** and create a feature branch
2. **Make your changes** with clear commits
3. **Test thoroughly**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```
4. **Update documentation** if needed
5. **Submit a pull request** with:
   - Clear description of changes
   - Links to related issues
   - Screenshots for UI changes

## Areas for Contribution

### High Priority
- [ ] Implement actual Moralis API integration
- [ ] Complete Helius webhook listener
- [ ] Add comprehensive integration tests
- [ ] Improve error handling and recovery
- [ ] Add metrics and monitoring

### Medium Priority
- [ ] Add support for multiple trading pairs
- [ ] Implement advanced risk management
- [ ] Add backtesting capabilities
- [ ] Improve dashboard with more metrics
- [ ] Add WebSocket support for real-time updates

### Low Priority
- [ ] Add support for other DEXes
- [ ] Mobile-responsive dashboard
- [ ] CLI interface
- [ ] Performance optimizations

## Testing Guidelines

- Write unit tests for utility functions
- Add integration tests for API endpoints
- Mock external services in tests
- Aim for >80% code coverage for critical paths

Example test:
```rust
#[tokio::test]
async fn test_config_validation() {
    let config = Config {
        slippage_bps: 50,
        min_profit_threshold: 0.01,
        max_position_size_sol: 1.0,
        // ... other fields
    };
    assert!(config.validate().is_ok());
}
```

## Security

- **Never commit** API keys or private keys
- Use `.env` for sensitive configuration
- Report security issues privately to maintainers
- Follow secure coding practices
- Validate all user inputs

## Code Review

All submissions require code review:
- Address review comments promptly
- Be open to feedback and suggestions
- Discuss design decisions when needed

## Questions?

- Open an issue for bugs or feature requests
- Start a discussion for general questions
- Check existing issues before creating new ones

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing! 🚀
