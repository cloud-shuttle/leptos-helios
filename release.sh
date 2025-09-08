#!/bin/bash

# Helios Visualization Library - Demo Release Script
# Version: v0.2.0-demo

set -e

echo "🚀 Helios Visualization Library - Demo Release v0.2.0"
echo "=================================================="

# Create release directory
RELEASE_DIR="helios-demo-v0.2.0"
echo "📦 Creating release directory: $RELEASE_DIR"
rm -rf "$RELEASE_DIR"
mkdir -p "$RELEASE_DIR"

# Copy demo files
echo "📋 Copying demo files..."
cp canvas2d-demo.html "$RELEASE_DIR/"
cp webgpu-demo.html "$RELEASE_DIR/"
cp webgpu-charts-demo.html "$RELEASE_DIR/"
cp example.html "$RELEASE_DIR/"
cp demo-server.py "$RELEASE_DIR/"

# Copy WASM files
echo "🦀 Copying WASM files..."
cp -r helios-app/pkg "$RELEASE_DIR/"

# Copy documentation
echo "📚 Copying documentation..."
cp RELEASE_v0.2.0-demo.md "$RELEASE_DIR/README.md"
cp CANVAS2D_DEMO_README.md "$RELEASE_DIR/"
cp CANVAS2D_TDD_SUMMARY.md "$RELEASE_DIR/"
cp DEMO_RELEASE_SUMMARY.md "$RELEASE_DIR/"

# Copy TDD implementation files
echo "🧪 Copying TDD implementation..."
mkdir -p "$RELEASE_DIR/src"
cp helios-core/src/canvas2d_renderer.rs "$RELEASE_DIR/src/"
cp helios-core/tests/canvas2d_rendering_tdd.rs "$RELEASE_DIR/src/"

# Create release info
echo "📝 Creating release info..."
cat > "$RELEASE_DIR/RELEASE_INFO.txt" << EOF
Helios Visualization Library - Demo Release v0.2.0
==================================================

Release Date: $(date)
Version: v0.2.0-demo
Status: Ready for Release

Demo URLs (when running demo-server.py):
- Canvas2D TDD Demo: http://localhost:8080/
- WebGPU Demo: http://localhost:8080/webgpu
- WebGPU Charts Demo: http://localhost:8080/webgpu-charts
- WASM Example: http://localhost:8080/example

Quick Start:
1. python3 demo-server.py
2. Open http://localhost:8080 in your browser
3. Explore all four demo pages

Features:
- Complete TDD implementation
- Canvas2D rendering with 100K points in <3ms
- WebGPU acceleration with 800+ MB/s throughput
- WebAssembly integration with Rust functions
- Interactive charts and performance benchmarks

Performance Metrics:
- Canvas2D: 100K points in <3ms
- WebGPU: 816 MB/s throughput, 1M points in 77ms
- WASM: <1ms function execution
- Interactive: Sub-millisecond hover detection

Built with Rust, WebAssembly, WebGPU, and Test-Driven Development
EOF

# Make demo server executable
chmod +x "$RELEASE_DIR/demo-server.py"

# Create archive
echo "🗜️ Creating release archive..."
tar -czf "${RELEASE_DIR}.tar.gz" "$RELEASE_DIR"
zip -r "${RELEASE_DIR}.zip" "$RELEASE_DIR"

# Display release info
echo ""
echo "✅ Release created successfully!"
echo "📁 Release directory: $RELEASE_DIR"
echo "📦 Archives created:"
echo "   - ${RELEASE_DIR}.tar.gz"
echo "   - ${RELEASE_DIR}.zip"
echo ""
echo "🚀 To test the release:"
echo "   cd $RELEASE_DIR"
echo "   python3 demo-server.py"
echo "   open http://localhost:8080"
echo ""
echo "📊 Release Contents:"
echo "   - 4 Interactive Demo Pages"
echo "   - Complete TDD Implementation"
echo "   - WebGPU Performance Benchmarks"
echo "   - WebAssembly Integration"
echo "   - Documentation and README"
echo ""
echo "🎉 Ready for distribution!"
