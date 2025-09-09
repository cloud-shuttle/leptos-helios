#!/usr/bin/env python3
"""
Comprehensive Demo Server for leptos-helios
Serves all demo pages and showcases for E2E testing
"""

import http.server
import socketserver
import os
import sys
import webbrowser
import threading
import time
from pathlib import Path

class DemoHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    """Custom request handler with CORS support and proper MIME types"""

    def end_headers(self):
        # Add CORS headers for cross-origin requests
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        self.send_header('Cache-Control', 'no-cache, no-store, must-revalidate')
        self.send_header('Pragma', 'no-cache')
        self.send_header('Expires', '0')
        super().end_headers()

    def guess_type(self, path):
        """Override to set correct MIME types"""
        mimetype, encoding = super().guess_type(path)

        # Ensure proper MIME types for our demo files
        if path.endswith('.html'):
            return 'text/html'
        elif path.endswith('.js'):
            return 'application/javascript'
        elif path.endswith('.css'):
            return 'text/css'
        elif path.endswith('.json'):
            return 'application/json'
        elif path.endswith('.wasm'):
            return 'application/wasm'

        return mimetype

    def log_message(self, format, *args):
        """Custom log format"""
        print(f"[{time.strftime('%H:%M:%S')}] {format % args}")

def find_available_port(start_port=8082, max_attempts=10):
    """Find an available port starting from start_port"""
    import socket

    for port in range(start_port, start_port + max_attempts):
        try:
            with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
                s.bind(('localhost', port))
                return port
        except OSError:
            continue

    raise RuntimeError(f"Could not find available port in range {start_port}-{start_port + max_attempts}")

def create_demo_index():
    """Create an index page listing all available demos"""
    demos = [
        {
            'name': 'Comprehensive Showcase',
            'file': 'comprehensive-showcase.html',
            'description': 'Complete showcase of all leptos-helios features'
        },
        {
            'name': 'Phase 4 Showcase',
            'file': 'phase4-showcase.html',
            'description': 'Phase 4 features: Security, Accessibility, Performance'
        },
        {
            'name': 'Phase 5 Showcase',
            'file': 'phase5-showcase.html',
            'description': 'Phase 5 features: Theming and Analytics'
        },
        {
            'name': 'Canvas2D Demo',
            'file': 'canvas2d-demo.html',
            'description': 'Canvas2D rendering capabilities'
        },
        {
            'name': 'WebGPU Demo',
            'file': 'webgpu-demo.html',
            'description': 'WebGPU rendering capabilities'
        },
        {
            'name': 'WebGPU Charts Demo',
            'file': 'webgpu-charts-demo.html',
            'description': 'WebGPU chart rendering'
        },
        {
            'name': 'Streaming Demo',
            'file': 'simple-streaming-demo.html',
            'description': 'Real-time streaming visualizations'
        }
    ]

    html_content = f"""<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>leptos-helios Demo Index</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}

        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: #333;
            line-height: 1.6;
            min-height: 100vh;
        }}

        .container {{
            max-width: 1200px;
            margin: 0 auto;
            padding: 40px 20px;
        }}

        .header {{
            text-align: center;
            margin-bottom: 50px;
            color: white;
        }}

        .header h1 {{
            font-size: 3rem;
            margin-bottom: 10px;
            text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
        }}

        .header p {{
            font-size: 1.2rem;
            opacity: 0.9;
        }}

        .demo-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
            gap: 30px;
        }}

        .demo-card {{
            background: white;
            border-radius: 15px;
            padding: 30px;
            box-shadow: 0 10px 30px rgba(0,0,0,0.1);
            transition: transform 0.3s ease, box-shadow 0.3s ease;
            text-decoration: none;
            color: inherit;
        }}

        .demo-card:hover {{
            transform: translateY(-5px);
            box-shadow: 0 15px 40px rgba(0,0,0,0.15);
        }}

        .demo-card h3 {{
            color: #667eea;
            margin-bottom: 15px;
            font-size: 1.5rem;
        }}

        .demo-card p {{
            color: #666;
            margin-bottom: 20px;
        }}

        .demo-link {{
            display: inline-block;
            background: #667eea;
            color: white;
            padding: 10px 20px;
            border-radius: 5px;
            text-decoration: none;
            transition: background 0.3s ease;
        }}

        .demo-link:hover {{
            background: #5a6fd8;
        }}

        .status {{
            text-align: center;
            margin-top: 30px;
            color: white;
            font-size: 0.9rem;
            opacity: 0.8;
        }}

        @media (max-width: 768px) {{
            .demo-grid {{
                grid-template-columns: 1fr;
            }}

            .header h1 {{
                font-size: 2rem;
            }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🚀 leptos-helios</h1>
            <p>Demo Index - Choose a showcase to explore</p>
        </div>

        <div class="demo-grid">
"""

    for demo in demos:
        if os.path.exists(demo['file']):
            html_content += f"""
            <a href="{demo['file']}" class="demo-card">
                <h3>{demo['name']}</h3>
                <p>{demo['description']}</p>
                <span class="demo-link">Launch Demo →</span>
            </a>
"""
        else:
            html_content += f"""
            <div class="demo-card" style="opacity: 0.6;">
                <h3>{demo['name']}</h3>
                <p>{demo['description']}</p>
                <span class="demo-link" style="background: #ccc;">Demo Not Available</span>
            </div>
"""

    html_content += f"""
        </div>

        <div class="status">
            <p>Server running on port {PORT} | Last updated: {time.strftime('%Y-%m-%d %H:%M:%S')}</p>
        </div>
    </div>
</body>
</html>"""

    with open('demo-index.html', 'w') as f:
        f.write(html_content)

def main():
    global PORT

    # Find available port
    PORT = find_available_port()

    # Change to the directory containing this script
    script_dir = Path(__file__).parent
    os.chdir(script_dir)

    # Create demo index
    create_demo_index()

    # Create server
    with socketserver.TCPServer(("", PORT), DemoHTTPRequestHandler) as httpd:
        print(f"🚀 leptos-helios Comprehensive Demo Server")
        print(f"📡 Server running on http://localhost:{PORT}")
        print(f"🌐 Demo Index: http://localhost:{PORT}/demo-index.html")
        print(f"🎯 Comprehensive Showcase: http://localhost:{PORT}/comprehensive-showcase.html")
        print(f"📊 Available demos:")

        # List available demos
        demo_files = [
            'comprehensive-showcase.html',
            'phase4-showcase.html',
            'phase5-showcase.html',
            'canvas2d-demo.html',
            'webgpu-demo.html',
            'webgpu-charts-demo.html',
            'simple-streaming-demo.html'
        ]

        for demo_file in demo_files:
            if os.path.exists(demo_file):
                print(f"   ✅ {demo_file}")
            else:
                print(f"   ❌ {demo_file} (not found)")

        print(f"\n🔧 Server Features:")
        print(f"   • CORS enabled for cross-origin requests")
        print(f"   • Proper MIME types for all file types")
        print(f"   • No-cache headers for development")
        print(f"   • Custom logging with timestamps")

        print(f"\n🧪 E2E Testing:")
        print(f"   • Playwright tests: npx playwright test")
        print(f"   • Test URL: http://localhost:{PORT}")
        print(f"   • Comprehensive showcase: /comprehensive-showcase.html")

        print(f"\n⏹️  Press Ctrl+C to stop the server")
        print("=" * 60)

        try:
            # Open browser automatically
            def open_browser():
                time.sleep(1)
                webbrowser.open(f'http://localhost:{PORT}/demo-index.html')

            browser_thread = threading.Thread(target=open_browser)
            browser_thread.daemon = True
            browser_thread.start()

            httpd.serve_forever()
        except KeyboardInterrupt:
            print(f"\n🛑 Server stopped by user")
            sys.exit(0)

if __name__ == "__main__":
    main()
