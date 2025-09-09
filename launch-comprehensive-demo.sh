#!/bin/bash

# Launch Comprehensive Demo Server for leptos-helios
# This script starts the demo server and provides information about available demos

echo "🚀 Starting leptos-helios Comprehensive Demo Server..."
echo ""

# Check if Python is available
if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 is required but not installed."
    echo "   Please install Python 3 and try again."
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "comprehensive-demo-server.py" ]; then
    echo "❌ comprehensive-demo-server.py not found."
    echo "   Please run this script from the leptos-helios root directory."
    exit 1
fi

# Make the server script executable
chmod +x comprehensive-demo-server.py

# Check for available demo files
echo "📊 Checking available demo files..."
demo_files=(
    "comprehensive-showcase.html"
    "phase4-showcase.html"
    "phase5-showcase.html"
    "canvas2d-demo.html"
    "webgpu-demo.html"
    "webgpu-charts-demo.html"
    "simple-streaming-demo.html"
)

available_demos=0
for demo in "${demo_files[@]}"; do
    if [ -f "$demo" ]; then
        echo "   ✅ $demo"
        ((available_demos++))
    else
        echo "   ❌ $demo (not found)"
    fi
done

echo ""
echo "📈 Demo Status: $available_demos/${#demo_files[@]} demos available"
echo ""

# Check if Playwright is available for testing
if command -v npx &> /dev/null; then
    echo "🧪 Playwright testing available:"
    echo "   • Run E2E tests: npx playwright test"
    echo "   • Run specific tests: npx playwright test comprehensive-showcase"
    echo "   • Run with UI: npx playwright test --ui"
    echo ""
fi

# Start the server
echo "🌐 Starting demo server..."
echo "   • Server will open automatically in your browser"
echo "   • Press Ctrl+C to stop the server"
echo "   • Server logs will show below"
echo ""

python3 comprehensive-demo-server.py
