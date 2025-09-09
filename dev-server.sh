#!/bin/bash

# Helios Development Server Script
# This script provides easy access to different development modes

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Helios Development Server${NC}"
echo "=================================="

# Function to show usage
show_usage() {
    echo "Usage: $0 [OPTION]"
    echo ""
    echo "Options:"
    echo "  trunk-app     Start Trunk development server for helios-app (port 8081)"
    echo "  trunk-root    Start Trunk development server for root project (port 8080)"
    echo "  python        Start Python HTTP server for static demos (port 8081)"
    echo "  build         Build the project with Trunk"
    echo "  clean         Clean build artifacts"
    echo "  help          Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 trunk-app    # Start Trunk dev server for helios-app"
    echo "  $0 python       # Start Python server for static demos"
    echo "  $0 build        # Build the project"
}

# Function to start Trunk for helios-app
start_trunk_app() {
    echo -e "${GREEN}🎯 Starting Trunk development server for helios-app...${NC}"
    echo "📍 Server will be available at: http://localhost:8081"
    echo "🔄 Hot reloading enabled"
    echo "⚡ Press Ctrl+C to stop"
    echo ""

    cd helios-app
    trunk serve --port 8081 --open
}

# Function to start Trunk for root project
start_trunk_root() {
    echo -e "${GREEN}🎯 Starting Trunk development server for root project...${NC}"
    echo "📍 Server will be available at: http://localhost:8080"
    echo "🔄 Hot reloading enabled"
    echo "⚡ Press Ctrl+C to stop"
    echo ""

    trunk serve --port 8080 --open
}

# Function to start Python server
start_python_server() {
    echo -e "${GREEN}🐍 Starting Python HTTP server...${NC}"
    echo "📍 Server will be available at: http://localhost:8081"
    echo "📁 Serving static demo files"
    echo "⚡ Press Ctrl+C to stop"
    echo ""

    python3 demo-server.py
}

# Function to build with Trunk
build_project() {
    echo -e "${GREEN}🔨 Building project with Trunk...${NC}"

    echo "Building helios-app..."
    cd helios-app
    trunk build
    cd ..

    echo "Building root project..."
    trunk build

    echo -e "${GREEN}✅ Build complete!${NC}"
}

# Function to clean build artifacts
clean_project() {
    echo -e "${YELLOW}🧹 Cleaning build artifacts...${NC}"

    # Clean Rust target directories
    cargo clean

    # Clean Trunk dist directories
    rm -rf helios-app/dist
    rm -rf dist

    # Clean node_modules if they exist
    if [ -d "node_modules" ]; then
        rm -rf node_modules
    fi

    echo -e "${GREEN}✅ Clean complete!${NC}"
}

# Main script logic
case "${1:-help}" in
    "trunk-app")
        start_trunk_app
        ;;
    "trunk-root")
        start_trunk_root
        ;;
    "python")
        start_python_server
        ;;
    "build")
        build_project
        ;;
    "clean")
        clean_project
        ;;
    "help"|*)
        show_usage
        ;;
esac
