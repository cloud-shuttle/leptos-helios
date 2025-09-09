#!/bin/bash

# Helios Phase 5B Demo Launcher
# Launches the Phase 5B showcase with advanced theming and analytics

echo "🚀 Launching Helios Phase 5B Demo..."
echo "=================================="

# Check if Python 3 is available
if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 is required but not installed."
    echo "   Please install Python 3 and try again."
    exit 1
fi

# Check if showcase file exists
if [ ! -f "phase5-showcase.html" ]; then
    echo "❌ phase5-showcase.html not found!"
    echo "   Make sure you're running this from the project root directory."
    exit 1
fi

# Launch the demo server
echo "🎨 Starting Phase 5B showcase server..."
echo "   Features: Advanced Theming + Analytics"
echo ""

python3 phase5-demo-server.py
