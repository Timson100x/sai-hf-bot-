#!/bin/bash

# Solana HFT Bot VPS Deployment Script
# This script helps you deploy the bot on a fresh VPS

set -e

echo "================================================"
echo "Solana HFT Bot - VPS Deployment Script"
echo "================================================"
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
    echo "Please run as root or with sudo"
    exit 1
fi

echo "Step 1: Updating system packages..."
apt-get update && apt-get upgrade -y

echo ""
echo "Step 2: Installing Docker..."
if ! command -v docker &> /dev/null; then
    curl -fsSL https://get.docker.com -o get-docker.sh
    sh get-docker.sh
    rm get-docker.sh
    echo "Docker installed successfully"
else
    echo "Docker is already installed"
fi

echo ""
echo "Step 3: Installing Docker Compose..."
if ! command -v docker-compose &> /dev/null; then
    apt-get install -y docker-compose
    echo "Docker Compose installed successfully"
else
    echo "Docker Compose is already installed"
fi

echo ""
echo "Step 4: Setting up firewall..."
apt-get install -y ufw
ufw --force enable
ufw allow 22/tcp   # SSH
ufw allow 8080/tcp # Bot dashboard
echo "Firewall configured"

echo ""
echo "Step 5: Checking if .env file exists..."
if [ ! -f ".env" ]; then
    if [ -f ".env.example" ]; then
        echo "Creating .env from .env.example..."
        cp .env.example .env
        echo ""
        echo "⚠️  IMPORTANT: Please edit the .env file with your API keys:"
        echo "    nano .env"
        echo ""
        echo "After editing .env, run this script again or run:"
        echo "    docker-compose up -d"
        exit 0
    else
        echo "Error: .env.example not found. Please ensure you're in the project directory."
        exit 1
    fi
else
    echo ".env file already exists"
fi

echo ""
echo "Step 6: Building and starting the bot..."
docker-compose down 2>/dev/null || true
docker-compose build --no-cache
docker-compose up -d

echo ""
echo "================================================"
echo "✅ Deployment complete!"
echo "================================================"
echo ""
echo "The bot is now running!"
echo ""
echo "Dashboard: http://$(hostname -I | awk '{print $1}'):8080"
echo ""
echo "Useful commands:"
echo "  - View logs:        docker-compose logs -f"
echo "  - Stop bot:         docker-compose stop"
echo "  - Start bot:        docker-compose start"
echo "  - Restart bot:      docker-compose restart"
echo "  - View status:      docker-compose ps"
echo "  - Update and restart:"
echo "      git pull"
echo "      docker-compose down"
echo "      docker-compose up -d --build"
echo ""
