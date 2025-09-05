#!/bin/bash

# Helios Build Script
# Comprehensive build pipeline for Rust to WASM compilation

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROJECT_NAME="helios"
WASM_TARGET="wasm32-unknown-unknown"
RELEASE_DIR="dist"
PKG_DIR="pkg"
OPTIMIZATION_LEVEL=4

# Functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Install missing tools
install_tools() {
    log_info "Checking build tools..."
    
    # Check for Rust
    if ! command_exists cargo; then
        log_error "Rust/Cargo not found. Please install Rust: https://rustup.rs/"
        exit 1
    fi
    
    # Check for wasm-pack
    if ! command_exists wasm-pack; then
        log_warning "wasm-pack not found. Installing..."
        cargo install wasm-pack
    fi
    
    # Check for trunk
    if ! command_exists trunk; then
        log_warning "trunk not found. Installing..."
        cargo install trunk
    fi
    
    # Check for wasm-opt
    if ! command_exists wasm-opt; then
        log_warning "wasm-opt not found. Installing binaryen..."
        if [[ "$OSTYPE" == "darwin"* ]]; then
            brew install binaryen
        elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
            sudo apt-get update && sudo apt-get install -y binaryen
        else
            log_error "Please install binaryen manually for your OS"
            exit 1
        fi
    fi
    
    log_success "All build tools are available"
}

# Clean previous builds
clean_build() {
    log_info "Cleaning previous builds..."
    rm -rf $RELEASE_DIR
    rm -rf $PKG_DIR
    cargo clean
    log_success "Clean completed"
}

# Build Rust to WASM
build_wasm() {
    log_info "Building Rust to WASM..."
    
    # Build with wasm-pack
    wasm-pack build \
        --target web \
        --out-dir $PKG_DIR \
        --out-name helios \
        --release \
        --no-typescript \
        --no-pack \
        --no-default-features \
        --features webgpu,simd
    
    log_success "WASM build completed"
}

# Optimize WASM
optimize_wasm() {
    log_info "Optimizing WASM..."
    
    if command_exists wasm-opt; then
        wasm-opt \
            -O$OPTIMIZATION_LEVEL \
            --enable-simd \
            --enable-threads \
            --enable-bulk-memory \
            --enable-mutable-globals \
            --enable-nontrapping-float-to-int \
            --enable-sign-ext \
            --enable-saturating-float-to-int \
            --enable-tail-call \
            --strip-debug \
            --strip-producers \
            $PKG_DIR/helios_bg.wasm \
            -o $PKG_DIR/helios_optimized.wasm
        
        # Replace original with optimized
        mv $PKG_DIR/helios_optimized.wasm $PKG_DIR/helios_bg.wasm
        log_success "WASM optimization completed"
    else
        log_warning "wasm-opt not available, skipping optimization"
    fi
}

# Build with Trunk
build_trunk() {
    log_info "Building with Trunk..."
    
    # Create index.html if it doesn't exist
    if [ ! -f "index.html" ]; then
        cat > index.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Helios Visualization Library</title>
    <style>
        body {
            margin: 0;
            padding: 0;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: #1a1a1a;
            color: #ffffff;
        }
        #app {
            width: 100vw;
            height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
        }
        .loading {
            text-align: center;
        }
        .spinner {
            border: 4px solid #333;
            border-top: 4px solid #00d4ff;
            border-radius: 50%;
            width: 40px;
            height: 40px;
            animation: spin 1s linear infinite;
            margin: 0 auto 20px;
        }
        @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
        }
    </style>
</head>
<body>
    <div id="app">
        <div class="loading">
            <div class="spinner"></div>
            <p>Loading Helios Visualization Library...</p>
        </div>
    </div>
    <script type="module">
        import init from './pkg/helios.js';
        init().then(() => {
            console.log('Helios loaded successfully');
        }).catch(console.error);
    </script>
</body>
</html>
EOF
    fi
    
    # Build with Trunk
    trunk build --release
    
    log_success "Trunk build completed"
}

# Run tests
run_tests() {
    log_info "Running tests..."
    
    # Run Rust tests
    cargo test --workspace
    
    # Run WASM tests if available
    if [ -f "tests/wasm_tests.js" ]; then
        node tests/wasm_tests.js
    fi
    
    log_success "All tests passed"
}

# Development server
dev_server() {
    log_info "Starting development server..."
    trunk serve --open
}

# Main build function
build() {
    log_info "Starting Helios build process..."
    
    install_tools
    clean_build
    build_wasm
    optimize_wasm
    build_trunk
    
    log_success "Build completed successfully!"
    log_info "Output directory: $RELEASE_DIR"
    log_info "Package directory: $PKG_DIR"
}

# Parse command line arguments
case "${1:-build}" in
    "build")
        build
        ;;
    "dev"|"serve")
        dev_server
        ;;
    "test")
        run_tests
        ;;
    "clean")
        clean_build
        ;;
    "install-tools")
        install_tools
        ;;
    "wasm")
        build_wasm
        optimize_wasm
        ;;
    "trunk")
        build_trunk
        ;;
    *)
        echo "Usage: $0 {build|dev|test|clean|install-tools|wasm|trunk}"
        echo ""
        echo "Commands:"
        echo "  build        - Full build process (default)"
        echo "  dev          - Start development server"
        echo "  test         - Run all tests"
        echo "  clean        - Clean build artifacts"
        echo "  install-tools - Install required build tools"
        echo "  wasm         - Build and optimize WASM only"
        echo "  trunk        - Build with Trunk only"
        exit 1
        ;;
esac
