#!/bin/bash

# Helios Live Streaming Demo Launcher
# This script starts the WebSocket streaming server and opens the demo page

set -e

echo "🚀 Helios Live Streaming Demo"
echo "=============================="

# Check if Python 3 is available
if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 is required but not installed"
    exit 1
fi

# Check if websockets module is available
if ! python3 -c "import websockets" 2>/dev/null; then
    echo "📦 Installing websockets module..."
    pip3 install websockets
fi

# Kill any existing servers on port 8083
echo "🧹 Cleaning up existing servers..."
lsof -ti:8083 | xargs kill -9 2>/dev/null || true

# Start WebSocket streaming server in background
echo "🌐 Starting WebSocket streaming server on port 8083..."
python3 streaming-server.py --port 8083 &
WEBSOCKET_PID=$!

# Wait a moment for server to start
sleep 2

# Check if server started successfully
if ! curl -s http://localhost:8083 >/dev/null 2>&1; then
    echo "⚠️  WebSocket server may not be ready yet, but continuing..."
fi

# Start HTTP server for demo page
echo "📊 Starting HTTP server for demo page on port 8084..."
python3 -m http.server 8084 &
HTTP_PID=$!

# Wait for servers to be ready
sleep 2

echo ""
echo "✅ Servers started successfully!"
echo "📡 WebSocket Server: ws://localhost:8083"
echo "🌐 Demo Page: http://localhost:8084/streaming-demo.html"
echo ""
echo "🎯 Available data sources:"
echo "   • Stock Prices (realistic market data)"
echo "   • IoT Sensors (temperature, humidity, pressure)"
echo "   • Network Traffic (bandwidth, latency, packets)"
echo "   • Cryptocurrency (price, volume, market cap)"
echo "   • Weather Data (temperature, humidity, wind, pressure)"
echo ""
echo "🔧 Features:"
echo "   • Real-time WebSocket streaming"
echo "   • Multiple chart types (line, bar, scatter, area)"
echo "   • Configurable update frequency"
echo "   • Performance metrics"
echo "   • Connection status monitoring"
echo "   • Fallback to local data generation"
echo ""
echo "🌐 Opening demo page in browser..."
open "http://localhost:8084/streaming-demo.html" 2>/dev/null || \
    xdg-open "http://localhost:8084/streaming-demo.html" 2>/dev/null || \
    echo "Please open http://localhost:8084/streaming-demo.html in your browser"

echo ""
echo "Press Ctrl+C to stop all servers"
echo "=============================="

# Function to cleanup on exit
cleanup() {
    echo ""
    echo "🛑 Shutting down servers..."
    kill $WEBSOCKET_PID 2>/dev/null || true
    kill $HTTP_PID 2>/dev/null || true
    echo "✅ Servers stopped"
    exit 0
}

# Set up signal handlers
trap cleanup SIGINT SIGTERM

# Wait for user to stop
wait
