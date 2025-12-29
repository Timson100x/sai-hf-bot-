# Build stage
FROM rust:1.92-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1000 botuser

# Create app directory
WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/sai-hf-bot /app/sai-hf-bot

# Copy dashboard files
COPY dashboard /app/dashboard

# Change ownership
RUN chown -R botuser:botuser /app

# Switch to app user
USER botuser

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/api/health || exit 1

# Run the application
CMD ["/app/sai-hf-bot"]
