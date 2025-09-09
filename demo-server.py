#!/usr/bin/env python3
"""
Simple HTTP server for serving the Canvas2D TDD Demo
Run with: python3 demo-server.py
"""

import http.server
import socketserver
import webbrowser
import os
import sys
from pathlib import Path

PORT = 8081

class DemoHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        # Add CORS headers for development
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        super().end_headers()

    def do_GET(self):
        # Serve the demo index page for root path
        if self.path == '/':
            self.path = '/demo-index.html'
        elif self.path == '/webgpu':
            self.path = '/webgpu-demo.html'
        elif self.path == '/webgpu-charts':
            self.path = '/webgpu-charts-demo.html'
        elif self.path == '/example':
            self.path = '/example.html'
        elif self.path == '/webgpu-test':
            self.path = '/webgpu-test.html'
        return super().do_GET()

def main():
    # Change to the directory containing this script
    script_dir = Path(__file__).parent
    os.chdir(script_dir)

    # Check if demo file exists
    demo_file = script_dir / 'canvas2d-demo.html'
    if not demo_file.exists():
        print("❌ Error: canvas2d-demo.html not found!")
        print("Please make sure the demo file is in the same directory as this script.")
        sys.exit(1)

    # Start the server
    with socketserver.TCPServer(("", PORT), DemoHTTPRequestHandler) as httpd:
        print("🚀 Helios Visualization Demo Server")
        print("=" * 50)
    print(f"📡 Server running at: http://localhost:{PORT}")
    print("=" * 50)
    print("🌐 Available Demos:")
    print(f"  • Demo Index: http://localhost:{PORT}/")
    print(f"  • Phase 4 Showcase: http://localhost:{PORT}/phase4-showcase.html")
    print(f"  • WebGPU Charts Demo: http://localhost:{PORT}/webgpu-charts-demo.html")
    print(f"  • Canvas2D Demo: http://localhost:{PORT}/canvas2d-demo.html")
    print(f"  • WebGPU Test: http://localhost:{PORT}/webgpu-test.html")
    print("=" * 50)
    print("🎯 Canvas2D Features:")
    print("  • Line Chart Rendering with TDD Tests")
    print("  • Bar Chart Rendering with Performance Metrics")
    print("  • Scatter Plot Rendering with Interactive Hover")
    print("  • Performance Benchmarks (100K points in <3ms)")
    print("  • Interactive Zoom, Pan, and Hover Detection")
    print("  • Real-time TDD Test Results")
    print("=" * 50)
    print("⚡ WebGPU Features:")
    print("  • WebGPU Support Detection")
    print("  • Shader Compilation and Caching")
    print("  • Render Pipeline Creation")
    print("  • Vertex Buffer Management")
    print("  • Performance Benchmarking")
    print("  • GPU Acceleration Testing")
    print("=" * 50)
    print("💡 Press Ctrl+C to stop the server")
    print()

    # Try to open the browser automatically
    try:
        webbrowser.open(f'http://localhost:{PORT}')
        print("🌐 Opening demo in your default browser...")
    except Exception as e:
        print(f"⚠️  Could not open browser automatically: {e}")
        print("   Please manually open: http://localhost:8081")

    print()

    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        print("\n🛑 Server stopped by user")
        print("👋 Thanks for trying the Canvas2D TDD Demo!")

if __name__ == "__main__":
    main()
