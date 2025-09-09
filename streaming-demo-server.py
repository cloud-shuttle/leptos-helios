#!/usr/bin/env python3
"""
Helios Streaming Demo Server
Combines HTTP server for demo pages with WebSocket streaming capabilities
"""

import asyncio
import json
import random
import time
import websockets
from datetime import datetime
from typing import Dict, List, Any
import argparse
import signal
import sys
from http.server import HTTPServer, SimpleHTTPRequestHandler
import threading
import os

class DataGenerator:
    """Generates realistic streaming data for different sources"""

    def __init__(self, source_type: str = "stock"):
        self.source_type = source_type
        self.base_values = self._init_base_values()
        self.trend = random.uniform(-0.001, 0.001)
        self.volatility = 0.02

    def _init_base_values(self) -> Dict[str, float]:
        """Initialize base values for different data sources"""
        if self.source_type == "stock":
            return {
                "price": 100.0 + random.uniform(-20, 20),
                "volume": 1000000.0,
                "market_cap": 1000000000.0
            }
        elif self.source_type == "sensor":
            return {
                "temperature": 20.0 + random.uniform(-5, 15),
                "humidity": 50.0 + random.uniform(-20, 20),
                "pressure": 1013.25 + random.uniform(-50, 50),
                "light": random.uniform(0, 1000)
            }
        elif self.source_type == "network":
            return {
                "bandwidth": random.uniform(100, 1000),
                "latency": 10.0 + random.uniform(0, 50),
                "packets": random.uniform(1000, 10000),
                "errors": random.uniform(0, 10)
            }
        elif self.source_type == "crypto":
            return {
                "price": 50000.0 + random.uniform(-10000, 20000),
                "volume": random.uniform(100000000, 1000000000),
                "market_cap": 1000000000000.0,
                "dominance": random.uniform(40, 60)
            }
        elif self.source_type == "weather":
            return {
                "temperature": 15.0 + random.uniform(-10, 25),
                "humidity": 40.0 + random.uniform(-20, 40),
                "wind_speed": random.uniform(0, 30),
                "pressure": 1013.25 + random.uniform(-40, 40),
                "precipitation": random.uniform(0, 10)
            }
        else:
            return {"value": 50.0}

    def generate_data_point(self) -> Dict[str, Any]:
        """Generate a new data point with realistic variations"""
        current_time = datetime.now()
        timestamp = current_time.isoformat()

        # Add some trend and volatility
        self.trend += random.uniform(-0.0001, 0.0001)
        self.trend = max(-0.005, min(0.005, self.trend))  # Clamp trend

        data = {
            "timestamp": timestamp,
            "source": self.source_type,
            "data": {}
        }

        # Generate data based on source type
        for key, base_value in self.base_values.items():
            # Add trend, volatility, and some seasonal patterns
            seasonal = 0.1 * random.sin(time.time() / 3600)  # Hourly seasonality
            noise = random.gauss(0, self.volatility)
            new_value = base_value * (1 + self.trend + seasonal + noise)

            # Keep values within reasonable bounds
            if key == "price" and new_value < 0:
                new_value = base_value * 0.1
            elif key in ["humidity", "dominance"] and (new_value < 0 or new_value > 100):
                new_value = max(0, min(100, new_value))
            elif key == "temperature" and (new_value < -50 or new_value > 60):
                new_value = max(-50, min(60, new_value))

            data["data"][key] = round(new_value, 2)
            self.base_values[key] = new_value

        # Add some metadata
        data["metadata"] = {
            "sequence": int(time.time() * 1000) % 1000000,
            "quality": random.uniform(0.95, 1.0),
            "anomaly_score": random.uniform(0, 0.1)
        }

        return data

class StreamingWebSocketServer:
    """WebSocket server for streaming data"""

    def __init__(self, host: str = "localhost", port: int = 8083):
        self.host = host
        self.port = port
        self.clients = set()
        self.data_generators = {}
        self.running = False

    async def register_client(self, websocket, path):
        """Register a new client connection"""
        self.clients.add(websocket)
        client_id = f"client_{len(self.clients)}"
        print(f"📡 WebSocket client connected: {client_id} (Total: {len(self.clients)})")

        try:
            # Send welcome message
            welcome_msg = {
                "type": "welcome",
                "client_id": client_id,
                "timestamp": datetime.now().isoformat(),
                "available_sources": ["stock", "sensor", "network", "crypto", "weather"],
                "server_info": {
                    "version": "1.0.0",
                    "uptime": time.time(),
                    "clients_connected": len(self.clients)
                }
            }
            await websocket.send(json.dumps(welcome_msg))

            # Handle client messages
            async for message in websocket:
                await self.handle_client_message(websocket, message)

        except websockets.exceptions.ConnectionClosed:
            pass
        finally:
            self.clients.remove(websocket)
            print(f"📡 WebSocket client disconnected: {client_id} (Total: {len(self.clients)})")

    async def handle_client_message(self, websocket, message):
        """Handle incoming client messages"""
        try:
            data = json.loads(message)
            msg_type = data.get("type")

            if msg_type == "subscribe":
                source = data.get("source", "stock")
                frequency = data.get("frequency", 500)  # ms

                # Create or update data generator
                if source not in self.data_generators:
                    self.data_generators[source] = DataGenerator(source)

                # Start streaming for this client
                await self.start_streaming(websocket, source, frequency)

            elif msg_type == "unsubscribe":
                # Stop streaming for this client
                await websocket.send(json.dumps({
                    "type": "unsubscribed",
                    "timestamp": datetime.now().isoformat()
                }))

            elif msg_type == "ping":
                # Respond to ping
                await websocket.send(json.dumps({
                    "type": "pong",
                    "timestamp": datetime.now().isoformat()
                }))

        except json.JSONDecodeError:
            await websocket.send(json.dumps({
                "type": "error",
                "message": "Invalid JSON message",
                "timestamp": datetime.now().isoformat()
            }))
        except Exception as e:
            print(f"❌ Error handling WebSocket message: {e}")

    async def start_streaming(self, websocket, source: str, frequency: int):
        """Start streaming data to a specific client"""
        generator = self.data_generators[source]

        # Send subscription confirmation
        await websocket.send(json.dumps({
            "type": "subscribed",
            "source": source,
            "frequency": frequency,
            "timestamp": datetime.now().isoformat()
        }))

        # Start streaming loop
        try:
            while websocket in self.clients:
                data_point = generator.generate_data_point()
                message = {
                    "type": "data",
                    "source": source,
                    "data": data_point,
                    "timestamp": datetime.now().isoformat()
                }

                await websocket.send(json.dumps(message))
                await asyncio.sleep(frequency / 1000.0)

        except websockets.exceptions.ConnectionClosed:
            pass

    async def start_server(self):
        """Start the WebSocket server"""
        self.running = True
        print(f"📡 WebSocket server starting on {self.host}:{self.port}")

        # Start server
        server = await websockets.serve(
            self.register_client,
            self.host,
            self.port,
            ping_interval=20,
            ping_timeout=10
        )

        try:
            await server.wait_closed()
        except KeyboardInterrupt:
            print("\n🛑 WebSocket server shutting down...")
            self.running = False
            server.close()
            await server.wait_closed()

class HeliosDemoHTTPHandler(SimpleHTTPRequestHandler):
    """Custom HTTP handler for Helios demo pages"""

    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=os.getcwd(), **kwargs)

    def end_headers(self):
        # Add CORS headers for WebSocket connections
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        super().end_headers()

    def do_GET(self):
        # Handle root path redirect
        if self.path == '/':
            self.path = '/streaming-demo.html'
        return super().do_GET()

class StreamingDemoServer:
    """Combined HTTP and WebSocket server for streaming demos"""

    def __init__(self, http_port: int = 8084, ws_port: int = 8083):
        self.http_port = http_port
        self.ws_port = ws_port
        self.http_server = None
        self.ws_server = None
        self.running = False

    def start_http_server(self):
        """Start HTTP server in a separate thread"""
        def run_server():
            self.http_server = HTTPServer(('localhost', self.http_port), HeliosDemoHTTPHandler)
            print(f"🌐 HTTP server started on http://localhost:{self.http_port}")
            self.http_server.serve_forever()

        http_thread = threading.Thread(target=run_server, daemon=True)
        http_thread.start()
        return http_thread

    async def start_websocket_server(self):
        """Start WebSocket server"""
        self.ws_server = StreamingWebSocketServer(port=self.ws_port)
        await self.ws_server.start_server()

    async def start(self):
        """Start both HTTP and WebSocket servers"""
        self.running = True

        print("🚀 Helios Live Streaming Demo Server")
        print("=" * 50)

        # Start HTTP server
        self.start_http_server()

        # Start WebSocket server
        await self.start_websocket_server()

    def stop(self):
        """Stop all servers"""
        self.running = False
        if self.http_server:
            self.http_server.shutdown()
        print("✅ All servers stopped")

def signal_handler(signum, frame):
    """Handle shutdown signals"""
    print(f"\n🛑 Received signal {signum}, shutting down...")
    sys.exit(0)

async def main():
    """Main server function"""
    parser = argparse.ArgumentParser(description="Helios Live Streaming Demo Server")
    parser.add_argument("--http-port", type=int, default=8084, help="HTTP server port")
    parser.add_argument("--ws-port", type=int, default=8083, help="WebSocket server port")
    args = parser.parse_args()

    # Set up signal handlers
    signal.signal(signal.SIGINT, signal_handler)
    signal.signal(signal.SIGTERM, signal_handler)

    # Create and start server
    server = StreamingDemoServer(args.http_port, args.ws_port)

    try:
        await server.start()
    except KeyboardInterrupt:
        print("\n👋 Goodbye!")
    except Exception as e:
        print(f"❌ Server error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("\n👋 Goodbye!")
    except Exception as e:
        print(f"❌ Server error: {e}")
        sys.exit(1)
