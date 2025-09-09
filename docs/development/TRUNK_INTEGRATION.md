# 🚀 Trunk Integration for Helios

This document describes the Trunk integration setup for the Helios visualization library, providing enhanced developer experience with hot reloading, optimized builds, and streamlined development workflow.

## 📋 Overview

Trunk is a modern build tool for Rust web applications that provides:
- **Hot Reloading**: Automatic rebuilds and browser refresh on code changes
- **WASM Optimization**: Optimized WebAssembly builds for production
- **Asset Bundling**: Automatic bundling of CSS, JS, and other assets
- **Development Server**: Built-in HTTP server with live reloading
- **Source Maps**: Full debugging support with source maps

## 🛠️ Setup

### Prerequisites

1. **Install Trunk** (if not already installed):
   ```bash
   cargo install trunk
   ```

2. **Verify Installation**:
   ```bash
   trunk --version
   ```

### Configuration Files

#### Root Project (`Trunk.toml`)
```toml
[build]
target = "dist"
index = "index.html"
wasm_file = "helios"
bin = "helios-app"

[tools]
wasm_opt = "false"

[serve]
address = "127.0.0.1"
port = 8080
open = true
watch = true

[watch]
watch = ["src", "assets", "index.html"]
ignore = ["target", "node_modules", ".git"]
```

#### Helios App (`helios-app/Trunk.toml`)
```toml
[build]
target = "dist"
index = "index.html"
wasm_file = "leptos_helios_app"
bin = "helios-app"

[tools]
wasm_opt = "false"

[serve]
address = "127.0.0.1"
port = 8081
open = true
watch = true

[watch]
watch = ["src", "assets", "index.html"]
ignore = ["target", "node_modules", ".git"]
```

## 🚀 Development Workflow

### Quick Start

Use the provided development script for easy access to different modes:

```bash
# Start Trunk development server for helios-app
./dev-server.sh trunk-app

# Start Trunk development server for root project
./dev-server.sh trunk-root

# Start Python HTTP server for static demos
./dev-server.sh python

# Build the project
./dev-server.sh build

# Clean build artifacts
./dev-server.sh clean
```

### Manual Commands

#### Development Server
```bash
# Start development server for helios-app (port 8081)
cd helios-app
trunk serve --port 8081 --open

# Start development server for root project (port 8080)
trunk serve --port 8080 --open
```

#### Building
```bash
# Build for development (faster, larger WASM)
trunk build

# Build for production (optimized, smaller WASM)
trunk build --release
```

#### Cleaning
```bash
# Clean Trunk build artifacts
rm -rf dist/
rm -rf helios-app/dist/

# Clean Rust build artifacts
cargo clean
```

## 🎯 Development Features

### Hot Reloading
- **Automatic Rebuilds**: Changes to Rust code trigger automatic WASM rebuilds
- **Live Browser Refresh**: Browser automatically refreshes when rebuilds complete
- **Fast Iteration**: Development cycle is significantly faster than manual builds

### File Watching
- **Source Files**: Watches `src/` directories for changes
- **Assets**: Watches `assets/` and `index.html` for changes
- **Ignored Files**: Automatically ignores `target/`, `node_modules/`, `.git/`

### Development Server
- **Built-in HTTP Server**: No need for external server setup
- **CORS Handling**: Proper CORS headers for WASM loading
- **Error Reporting**: Clear error messages in browser console

## 📁 Project Structure

```
leptos-helios/
├── Trunk.toml                 # Root Trunk configuration
├── helios-app/
│   ├── Trunk.toml            # Helios app Trunk configuration
│   ├── index.html            # Development index page
│   ├── src/
│   │   └── lib.rs            # Main WASM entry point
│   └── dist/                 # Trunk build output
├── dev-server.sh             # Development server script
└── dist/                     # Root Trunk build output
```

## 🔧 Configuration Options

### Build Configuration
- **`target`**: Output directory for built files
- **`index`**: Main HTML file to serve
- **`wasm_file`**: Name of the generated WASM file
- **`bin`**: Rust binary to build

### Tools Configuration
- **`wasm_opt`**: Enable/disable WASM optimization (disabled for faster dev builds)

### Serve Configuration
- **`address`**: Server bind address
- **`port`**: Server port
- **`open`**: Automatically open browser
- **`watch`**: Enable file watching

### Watch Configuration
- **`watch`**: Directories to watch for changes
- **`ignore`**: Directories to ignore

## 🚨 Troubleshooting

### Common Issues

#### Port Already in Use
```bash
# Kill existing processes on port 8080/8081
lsof -ti:8080 | xargs kill -9
lsof -ti:8081 | xargs kill -9
```

#### WASM Loading Issues
- Ensure `wasm_file` in `Trunk.toml` matches the actual WASM file name
- Check browser console for CORS or loading errors
- Verify the `index.html` file exists and is properly configured

#### Build Failures
- Run `cargo clean` to clear any corrupted build artifacts
- Check that all dependencies are properly installed
- Verify Rust toolchain is up to date

### Debug Mode
```bash
# Enable debug logging
RUST_LOG=debug trunk serve
```

## 🎨 Integration with Existing Demos

The Trunk integration works alongside the existing demo system:

1. **Static Demos**: Use `./dev-server.sh python` for static HTML demos
2. **WASM Demos**: Use `./dev-server.sh trunk-app` for interactive WASM demos
3. **Development**: Use Trunk for active development with hot reloading

## 📈 Performance Benefits

### Development
- **Faster Builds**: Trunk optimizes the build process for development
- **Hot Reloading**: Instant feedback on code changes
- **Source Maps**: Full debugging support

### Production
- **WASM Optimization**: Smaller, faster WASM files
- **Asset Bundling**: Optimized CSS and JS delivery
- **Tree Shaking**: Dead code elimination

## 🔄 Migration from Manual Builds

The Trunk integration is designed to work alongside existing build processes:

1. **Existing Demos**: Continue to work with Python server
2. **New Development**: Use Trunk for enhanced developer experience
3. **Production Builds**: Use Trunk for optimized production builds

## 📚 Additional Resources

- [Trunk Documentation](https://trunkrs.dev/)
- [Leptos Documentation](https://leptos.dev/)
- [WebAssembly Guide](https://developer.mozilla.org/en-US/docs/WebAssembly)

## 🤝 Contributing

When contributing to the project:

1. Use `./dev-server.sh trunk-app` for development
2. Test changes with hot reloading
3. Run `./dev-server.sh build` before committing
4. Update this documentation if configuration changes

---

**Happy coding with Trunk! 🚀**
