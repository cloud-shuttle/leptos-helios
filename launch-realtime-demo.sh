#!/bin/bash

# Helios Real-Time Data Visualization Demo Launcher
# This script launches the real-time streaming demo with WebSocket support

echo "🚀 Helios Real-Time Data Visualization Demo"
echo "============================================="

# Check if Python 3 is available
if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 is required but not installed."
    exit 1
fi

# Check if websockets module is available
if ! python3 -c "import websockets" &> /dev/null; then
    echo "⚠️  WebSocket support not available. Installing websockets..."
    pip3 install websockets
fi

# Function to start the streaming server
start_streaming_server() {
    echo "🌐 Starting WebSocket streaming server on port 8083..."
    python3 streaming-demo-server.py --http-port 8084 --ws-port 8083 &
    STREAMING_PID=$!
    echo "✅ Streaming server started (PID: $STREAMING_PID)"
}

# Function to start the HTTP server
start_http_server() {
    echo "📡 Starting HTTP server on port 8080..."
    python3 -m http.server 8080 --bind 127.0.0.1 &
    HTTP_PID=$!
    echo "✅ HTTP server started (PID: $HTTP_PID)"
}

# Function to cleanup on exit
cleanup() {
    echo ""
    echo "🛑 Shutting down servers..."
    if [ ! -z "$STREAMING_PID" ]; then
        kill $STREAMING_PID 2>/dev/null
        echo "✅ Streaming server stopped"
    fi
    if [ ! -z "$HTTP_PID" ]; then
        kill $HTTP_PID 2>/dev/null
        echo "✅ HTTP server stopped"
    fi
    echo "👋 Demo stopped. Thanks for using Helios!"
    exit 0
}

# Set up signal handlers
trap cleanup SIGINT SIGTERM

# Start servers
start_streaming_server
sleep 2
start_http_server
sleep 2

echo ""
echo "🎉 Demo is ready!"
echo ""
echo "📊 Available demos:"
echo "   • Real-Time Visualization: http://localhost:8080/realtime-demo.html"
echo "   • Basic Example:          http://localhost:8080/example.html"
echo "   • WebSocket Streaming:    http://localhost:8084/streaming-demo.html"
echo ""
echo "🎯 Features:"
echo "   • 4 chart types (Line, Bar, Scatter, Area)"
echo "   • 5 data sources (Stock, IoT, Network, Crypto, Weather)"
echo "   • Real-time WebSocket streaming"
echo "   • Local data generation fallback"
echo "   • Tailwind CSS styling"
echo "   • Responsive design"
echo ""
echo "💡 Tips:"
echo "   • Click 'Start Streaming' to begin"
echo "   • Try different data sources and frequencies"
echo "   • Watch the real-time metrics update"
echo "   • Check the console for detailed logs"
echo ""
echo "Press Ctrl+C to stop the demo"

# Wait for user to stop
wait
