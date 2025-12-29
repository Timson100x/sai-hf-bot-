// SAI-HF Bot Dashboard WebSocket Client
// Placeholder for real-time communication with the bot

class DashboardClient {
    constructor() {
        this.ws = null;
        this.reconnectAttempts = 0;
        this.maxReconnectAttempts = 5;
        this.reconnectDelay = 3000;
        this.stats = {
            totalTrades: 0,
            successfulTrades: 0,
            failedTrades: 0,
            activePools: 0
        };
    }

    // Initialize dashboard
    init() {
        this.updateTime();
        setInterval(() => this.updateTime(), 1000);
        
        // Placeholder: Connect to WebSocket server when implemented
        // this.connect();
        
        // For now, simulate some activity
        this.simulateActivity();
    }

    // Connect to WebSocket server
    connect() {
        const wsUrl = 'ws://localhost:8080/ws';
        
        try {
            this.ws = new WebSocket(wsUrl);
            
            this.ws.onopen = () => {
                console.log('Connected to bot');
                this.updateConnectionStatus(true);
                this.reconnectAttempts = 0;
            };
            
            this.ws.onmessage = (event) => {
                this.handleMessage(JSON.parse(event.data));
            };
            
            this.ws.onerror = (error) => {
                console.error('WebSocket error:', error);
                this.updateConnectionStatus(false);
            };
            
            this.ws.onclose = () => {
                console.log('Disconnected from bot');
                this.updateConnectionStatus(false);
                this.attemptReconnect();
            };
        } catch (error) {
            console.error('Failed to connect:', error);
            this.updateConnectionStatus(false);
        }
    }

    // Handle incoming messages
    handleMessage(data) {
        switch (data.type) {
            case 'stats':
                this.updateStats(data.stats);
                break;
            case 'activity':
                this.addActivity(data.activity);
                break;
            case 'pool':
                this.updatePool(data.pool);
                break;
            case 'config':
                this.updateConfig(data.config);
                break;
            default:
                console.log('Unknown message type:', data.type);
        }
    }

    // Update connection status
    updateConnectionStatus(connected) {
        const statusDot = document.getElementById('connection-status');
        const statusText = document.getElementById('connection-text');
        
        if (connected) {
            statusDot.className = 'status-dot connected';
            statusText.textContent = 'Connected';
        } else {
            statusDot.className = 'status-dot disconnected';
            statusText.textContent = 'Disconnected';
        }
    }

    // Attempt to reconnect
    attemptReconnect() {
        if (this.reconnectAttempts < this.maxReconnectAttempts) {
            this.reconnectAttempts++;
            console.log(`Reconnecting... Attempt ${this.reconnectAttempts}`);
            setTimeout(() => this.connect(), this.reconnectDelay);
        }
    }

    // Update statistics
    updateStats(stats) {
        this.stats = { ...this.stats, ...stats };
        document.getElementById('total-trades').textContent = this.stats.totalTrades;
        document.getElementById('successful-trades').textContent = this.stats.successfulTrades;
        document.getElementById('failed-trades').textContent = this.stats.failedTrades;
        document.getElementById('active-pools').textContent = this.stats.activePools;
    }

    // Add activity log entry
    addActivity(activity) {
        const activityLog = document.getElementById('activity-log');
        
        // Remove placeholder if exists
        const placeholder = activityLog.querySelector('.placeholder');
        if (placeholder) {
            placeholder.remove();
        }
        
        const entry = document.createElement('div');
        entry.className = `activity-entry ${activity.type}`;
        entry.innerHTML = `
            <span class="timestamp">${new Date(activity.timestamp).toLocaleTimeString()}</span>
            <span class="message">${activity.message}</span>
        `;
        
        activityLog.insertBefore(entry, activityLog.firstChild);
        
        // Keep only last 50 entries
        while (activityLog.children.length > 50) {
            activityLog.removeChild(activityLog.lastChild);
        }
    }

    // Update pool information
    updatePool(pool) {
        const poolsList = document.getElementById('pools-list');
        
        // Remove placeholder if exists
        const placeholder = poolsList.querySelector('.placeholder');
        if (placeholder) {
            placeholder.remove();
        }
        
        let poolElement = document.getElementById(`pool-${pool.id}`);
        
        if (!poolElement) {
            poolElement = document.createElement('div');
            poolElement.id = `pool-${pool.id}`;
            poolElement.className = 'pool-item';
            poolsList.appendChild(poolElement);
        }
        
        poolElement.innerHTML = `
            <div class="pool-header">
                <strong>${pool.name || pool.id}</strong>
                <span class="pool-status ${pool.active ? 'active' : 'inactive'}">
                    ${pool.active ? 'Active' : 'Inactive'}
                </span>
            </div>
            <div class="pool-details">
                <span>Liquidity: ${pool.liquidity || 'N/A'}</span>
                <span>Volume: ${pool.volume || 'N/A'}</span>
            </div>
        `;
    }

    // Update configuration display
    updateConfig(config) {
        document.getElementById('config-sol-amount').textContent = `${config.solAmount || 0} SOL`;
        document.getElementById('config-slippage').textContent = `${config.slippage || 0} bps`;
        document.getElementById('config-interval').textContent = `${config.interval || 0}ms`;
    }

    // Update current time
    updateTime() {
        const now = new Date();
        document.getElementById('current-time').textContent = now.toLocaleTimeString();
    }

    // Simulate activity for demonstration
    simulateActivity() {
        // Simulate initial config
        this.updateConfig({
            solAmount: 0.1,
            slippage: 50,
            interval: 1000
        });

        // Add sample activity
        setTimeout(() => {
            this.addActivity({
                type: 'info',
                timestamp: new Date().toISOString(),
                message: 'Bot initialized successfully'
            });
        }, 1000);

        setTimeout(() => {
            this.addActivity({
                type: 'info',
                timestamp: new Date().toISOString(),
                message: 'Started monitoring liquidity pools'
            });
        }, 2000);

        setTimeout(() => {
            this.updateStats({
                totalTrades: 0,
                successfulTrades: 0,
                failedTrades: 0,
                activePools: 3
            });
            
            this.addActivity({
                type: 'success',
                timestamp: new Date().toISOString(),
                message: 'Found 3 active liquidity pools'
            });
        }, 3000);
    }

    // Send message to bot
    send(message) {
        if (this.ws && this.ws.readyState === WebSocket.OPEN) {
            this.ws.send(JSON.stringify(message));
        }
    }
}

// Initialize dashboard on page load
document.addEventListener('DOMContentLoaded', () => {
    const dashboard = new DashboardClient();
    dashboard.init();
    
    // Make dashboard globally accessible for debugging
    window.dashboard = dashboard;
});
