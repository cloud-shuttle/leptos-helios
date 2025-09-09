#!/usr/bin/env python3
"""
Phase 5B Demo Server
Serves the Helios Phase 5B showcase with advanced theming and analytics capabilities.
"""

import http.server
import socketserver
import webbrowser
import os
import sys
import time
import json
from pathlib import Path

class HeliosDemoHandler(http.server.SimpleHTTPRequestHandler):
    """Custom handler for Helios Phase 5B demo server"""

    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=os.getcwd(), **kwargs)

    def end_headers(self):
        # Add CORS headers for development
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        super().end_headers()

    def do_GET(self):
        """Handle GET requests with custom routing"""
        if self.path == '/':
            # Redirect to Phase 5B showcase
            self.send_response(302)
            self.send_header('Location', '/phase5-showcase.html')
            self.end_headers()
            return

        elif self.path == '/api/status':
            # API endpoint for system status
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.end_headers()

            status = {
                "system": "Helios Phase 5B Demo Server",
                "version": "0.6.0",
                "status": "running",
                "timestamp": time.time(),
                "features": {
                    "theming": {
                        "enabled": True,
                        "presets": 6,
                        "custom_themes": True,
                        "css_in_rust": True,
                        "animation_engine": True,
                        "responsive_design": True
                    },
                    "analytics": {
                        "enabled": True,
                        "ml_pipeline": True,
                        "statistical_analysis": True,
                        "anomaly_detection": True,
                        "forecasting": True,
                        "algorithm_registry": True
                    },
                    "performance": {
                        "memory_usage": "45MB",
                        "cpu_usage": "12%",
                        "gpu_usage": "8%",
                        "processing_time": "<10ms"
                    }
                }
            }

            self.wfile.write(json.dumps(status, indent=2).encode())
            return

        elif self.path == '/api/analytics/demo':
            # API endpoint for analytics demo data
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.end_headers()

            # Generate demo analytics data
            import random
            import math

            data_size = int(self.headers.get('X-Data-Size', 1000))
            analysis_type = self.headers.get('X-Analysis-Type', 'descriptive')

            # Generate time series data
            data = []
            for i in range(data_size):
                trend = i * 0.1
                seasonal = math.sin(i * 0.2) * 0.5
                noise = (random.random() - 0.5) * 0.2
                anomaly = random.random() < 0.05
                value = trend + seasonal + noise + (5 if anomaly else 0)

                data.append({
                    "x": i,
                    "y": value,
                    "timestamp": time.time() + i,
                    "is_anomaly": anomaly
                })

            # Calculate analytics results
            values = [d["y"] for d in data]
            mean = sum(values) / len(values)
            variance = sum((x - mean) ** 2 for x in values) / len(values)
            std_dev = math.sqrt(variance)

            anomalies = [d for d in data if d["is_anomaly"]]

            result = {
                "data": data,
                "statistics": {
                    "mean": mean,
                    "std_dev": std_dev,
                    "variance": variance,
                    "min": min(values),
                    "max": max(values),
                    "count": len(values),
                    "anomalies": len(anomalies)
                },
                "analysis_type": analysis_type,
                "processing_time": random.uniform(5, 50),
                "timestamp": time.time()
            }

            self.wfile.write(json.dumps(result, indent=2).encode())
            return

        elif self.path == '/api/themes':
            # API endpoint for theme information
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.end_headers()

            themes = {
                "default": {
                    "name": "Default",
                    "primary": "#667eea",
                    "secondary": "#764ba2",
                    "accent": "#f093fb",
                    "warning": "#f5576c",
                    "info": "#4facfe",
                    "success": "#00f2fe",
                    "description": "Clean and modern default theme"
                },
                "dark": {
                    "name": "Dark Mode",
                    "primary": "#2d3748",
                    "secondary": "#4a5568",
                    "accent": "#805ad5",
                    "warning": "#dd6b20",
                    "info": "#3182ce",
                    "success": "#38a169",
                    "description": "Dark theme for low-light environments"
                },
                "ocean": {
                    "name": "Ocean Blue",
                    "primary": "#0ea5e9",
                    "secondary": "#0284c7",
                    "accent": "#06b6d4",
                    "warning": "#f59e0b",
                    "info": "#3b82f6",
                    "success": "#10b981",
                    "description": "Calming ocean-inspired theme"
                },
                "sunset": {
                    "name": "Sunset Orange",
                    "primary": "#f97316",
                    "secondary": "#ea580c",
                    "accent": "#f59e0b",
                    "warning": "#ef4444",
                    "info": "#8b5cf6",
                    "success": "#22c55e",
                    "description": "Warm sunset color palette"
                },
                "forest": {
                    "name": "Forest Green",
                    "primary": "#16a34a",
                    "secondary": "#15803d",
                    "accent": "#84cc16",
                    "warning": "#eab308",
                    "info": "#0ea5e9",
                    "success": "#22c55e",
                    "description": "Natural forest green theme"
                }
            }

            self.wfile.write(json.dumps(themes, indent=2).encode())
            return

        else:
            # Default file serving
            super().do_GET()

    def log_message(self, format, *args):
        """Custom log format"""
        timestamp = time.strftime("%Y-%m-%d %H:%M:%S")
        print(f"[{timestamp}] {format % args}")

def find_free_port(start_port=8082, max_attempts=10):
    """Find a free port starting from start_port"""
    import socket

    for port in range(start_port, start_port + max_attempts):
        try:
            with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
                s.bind(('localhost', port))
                return port
        except OSError:
            continue

    raise RuntimeError(f"Could not find a free port in range {start_port}-{start_port + max_attempts}")

def main():
    """Main function to start the demo server"""
    print("🚀 Helios Phase 5B Demo Server")
    print("=" * 50)

    # Find a free port
    try:
        port = find_free_port(8082)
    except RuntimeError as e:
        print(f"❌ Error: {e}")
        sys.exit(1)

    # Check if showcase file exists
    showcase_file = Path("phase5-showcase.html")
    if not showcase_file.exists():
        print("❌ Error: phase5-showcase.html not found!")
        print("   Make sure you're running this from the project root directory.")
        sys.exit(1)

    # Start server
    try:
        with socketserver.TCPServer(("localhost", port), HeliosDemoHandler) as httpd:
            print(f"✅ Server started on http://localhost:{port}")
            print(f"📊 Phase 5B Showcase: http://localhost:{port}/phase5-showcase.html")
            print(f"🔧 API Status: http://localhost:{port}/api/status")
            print(f"🎨 Themes API: http://localhost:{port}/api/themes")
            print(f"📈 Analytics API: http://localhost:{port}/api/analytics/demo")
            print()
            print("Features available:")
            print("  🎨 Advanced Theming System")
            print("    - 6 theme presets")
            print("    - Custom color picker")
            print("    - CSS-in-Rust compilation")
            print("    - Animation engine")
            print("    - Responsive design")
            print()
            print("  📊 Advanced Analytics Engine")
            print("    - ML pipeline")
            print("    - Statistical analysis")
            print("    - Anomaly detection")
            print("    - Forecasting")
            print("    - Algorithm registry")
            print()
            print("Press Ctrl+C to stop the server")
            print("=" * 50)

            # Open browser automatically
            try:
                webbrowser.open(f"http://localhost:{port}/phase5-showcase.html")
                print("🌐 Opening browser...")
            except Exception as e:
                print(f"⚠️  Could not open browser automatically: {e}")
                print(f"   Please open http://localhost:{port}/phase5-showcase.html manually")

            # Start serving
            httpd.serve_forever()

    except KeyboardInterrupt:
        print("\n🛑 Server stopped by user")
    except Exception as e:
        print(f"❌ Server error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
