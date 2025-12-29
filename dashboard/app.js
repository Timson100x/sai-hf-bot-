// API base URL
const API_BASE = '/api';

// Update interval (milliseconds)
const UPDATE_INTERVAL = 5000;

// State
let updateTimer = null;

// Initialize dashboard
document.addEventListener('DOMContentLoaded', () => {
    console.log('Initializing dashboard...');
    checkHealth();
    updateDashboard();
    startAutoUpdate();
});

// Check API health
async function checkHealth() {
    try {
        const response = await fetch(`${API_BASE}/health`);
        const data = await response.json();
        
        if (data.status === 'healthy') {
            setStatus('connected', 'Connected');
        } else {
            setStatus('warning', 'Degraded');
        }
    } catch (error) {
        console.error('Health check failed:', error);
        setStatus('error', 'Disconnected');
    }
}

// Set connection status
function setStatus(status, text) {
    const statusDot = document.getElementById('statusDot');
    const statusText = document.getElementById('statusText');
    
    statusDot.className = `status-dot ${status}`;
    statusText.textContent = text;
}

// Update all dashboard data
async function updateDashboard() {
    await Promise.all([
        updateBotStatus(),
        updateOpportunities(),
        updatePools(),
        updateTrades()
    ]);
    
    updateLastUpdateTime();
}

// Update bot status
async function updateBotStatus() {
    try {
        const response = await fetch(`${API_BASE}/status`);
        const data = await response.json();
        
        document.getElementById('botStatus').textContent = data.bot_status || 'Unknown';
        document.getElementById('slippage').textContent = `${data.slippage_bps} bps`;
        document.getElementById('minProfit').textContent = `${data.min_profit_threshold} SOL`;
        document.getElementById('maxPosition').textContent = `${data.max_position_size_sol} SOL`;
    } catch (error) {
        console.error('Failed to update bot status:', error);
    }
}

// Update opportunities list
async function updateOpportunities() {
    try {
        const response = await fetch(`${API_BASE}/opportunities`);
        const opportunities = await response.json();
        
        const container = document.getElementById('opportunitiesList');
        
        if (opportunities.length === 0) {
            container.innerHTML = '<p class="placeholder">No opportunities detected</p>';
            return;
        }
        
        container.innerHTML = opportunities.map(opp => `
            <div class="opportunity-item">
                <div class="opp-header">
                    <strong>${opp.token_in} → ${opp.token_out}</strong>
                    <span class="profit ${opp.expected_profit > 0 ? 'positive' : 'negative'}">
                        ${opp.expected_profit > 0 ? '+' : ''}${opp.expected_profit.toFixed(4)} SOL
                    </span>
                </div>
                <div class="opp-details">
                    <span>Amount: ${opp.amount_in.toFixed(4)} SOL</span>
                    <span>Expected: ${opp.expected_amount_out.toFixed(4)} SOL</span>
                    <span class="pool-address">${truncateAddress(opp.pool_address)}</span>
                </div>
            </div>
        `).join('');
    } catch (error) {
        console.error('Failed to update opportunities:', error);
    }
}

// Update pools list
async function updatePools() {
    try {
        const response = await fetch(`${API_BASE}/pools`);
        const pools = await response.json();
        
        document.getElementById('poolCount').textContent = pools.length;
        
        const container = document.getElementById('poolsList');
        
        if (pools.length === 0) {
            container.innerHTML = '<p class="placeholder">No pools monitored yet</p>';
            return;
        }
        
        container.innerHTML = pools.slice(0, 10).map(pool => `
            <div class="pool-item">
                <div class="pool-header">
                    <strong>${pool.token_a} / ${pool.token_b}</strong>
                    <span class="pool-price">${pool.price.toFixed(6)}</span>
                </div>
                <div class="pool-details">
                    <span>Liquidity: ${pool.liquidity_a.toFixed(2)} / ${pool.liquidity_b.toFixed(2)}</span>
                    <span class="pool-address">${truncateAddress(pool.pool_address)}</span>
                </div>
            </div>
        `).join('');
    } catch (error) {
        console.error('Failed to update pools:', error);
    }
}

// Update trades table
async function updateTrades() {
    try {
        const response = await fetch(`${API_BASE}/trades`);
        const trades = await response.json();
        
        const tbody = document.getElementById('tradesBody');
        
        if (trades.length === 0) {
            tbody.innerHTML = '<tr><td colspan="6" class="placeholder">No trades yet</td></tr>';
            return;
        }
        
        tbody.innerHTML = trades.slice(-10).reverse().map(trade => `
            <tr class="${trade.success ? 'success' : 'failed'}">
                <td>${new Date().toLocaleTimeString()}</td>
                <td>${trade.success ? '✅ Success' : '❌ Failed'}</td>
                <td>${trade.amount_in.toFixed(4)}</td>
                <td>${trade.amount_out.toFixed(4)}</td>
                <td class="${trade.actual_profit > 0 ? 'positive' : 'negative'}">
                    ${trade.actual_profit > 0 ? '+' : ''}${trade.actual_profit.toFixed(4)}
                </td>
                <td class="signature">${truncateAddress(trade.signature || 'N/A')}</td>
            </tr>
        `).join('');
    } catch (error) {
        console.error('Failed to update trades:', error);
    }
}

// Start auto-update
function startAutoUpdate() {
    if (updateTimer) {
        clearInterval(updateTimer);
    }
    
    updateTimer = setInterval(() => {
        updateDashboard();
    }, UPDATE_INTERVAL);
}

// Update last update time
function updateLastUpdateTime() {
    const now = new Date();
    document.getElementById('lastUpdate').textContent = now.toLocaleTimeString();
}

// Utility: Truncate Solana address
function truncateAddress(address) {
    if (!address || address === 'N/A') return 'N/A';
    if (address.length <= 12) return address;
    return `${address.substring(0, 6)}...${address.substring(address.length - 4)}`;
}

// Handle page visibility
document.addEventListener('visibilitychange', () => {
    if (document.hidden) {
        if (updateTimer) {
            clearInterval(updateTimer);
            updateTimer = null;
        }
    } else {
        updateDashboard();
        startAutoUpdate();
    }
});
