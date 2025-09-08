# Helios Documentation

Welcome to the Helios visualization library documentation! This comprehensive guide covers all aspects of the library, from basic usage to advanced performance optimization.

## 📚 Documentation Index

### Getting Started
- **[Getting Started](getting-started.md)** - Quick start guide and basic setup
- **[Examples](examples.md)** - Comprehensive examples and demo suite
- **[Architecture](architecture.md)** - System architecture and design principles

### Demo Suite
- **[Canvas2D TDD Demo](canvas2d-tdd-demo.md)** - Test-Driven Development methodology demonstration
- **[WebGPU Demo](webgpu-demo.md)** - High-performance GPU acceleration showcase
- **[WebAssembly Integration](wasm-integration.md)** - Rust-WebAssembly integration guide

### Core Features
- **[API Reference](api-reference.md)** - Complete API documentation
- **[Performance Guide](performance-guide.md)** - Performance optimization techniques
- **[Chart Types](api.md)** - Available chart types and configurations

### Development
- **[Contributing](contributing.md)** - Development guidelines and contribution process
- **[Migration Guide](migration.md)** - Version migration instructions
- **[Troubleshooting](troubleshooting.md)** - Common issues and solutions

### Advanced Topics
- **[Ecosystem](ecosystem.md)** - Integration with other libraries and frameworks
- **[Performance](performance.md)** - Detailed performance analysis and benchmarks
- **[Roadmap](roadmap.md)** - Future development plans and features

## 🎯 Quick Start

### Demo Suite
Experience Helios in action with our comprehensive demo suite:

```bash
# Clone the repository
git clone <repository-url>
cd leptos-helios

# Start the demo server
python3 demo-server.py

# Open http://localhost:8080 in your browser
```

### Available Demos
- **Canvas2D TDD Demo** (`/`) - Interactive TDD methodology demonstration
- **WebGPU Demo** (`/webgpu`) - GPU acceleration testing and benchmarking
- **WebGPU Charts Demo** (`/webgpu-charts`) - Visual GPU-rendered charts
- **WASM Example** (`/example`) - Rust-WebAssembly integration showcase

## 🚀 Key Features

### Performance Excellence
- **Canvas2D**: 100K points in <3ms render time
- **WebGPU**: 816+ MB/s throughput, 1M points in 77ms
- **WASM**: <1ms function execution
- **Interactive**: Sub-millisecond hover detection

### Modern Technologies
- **WebGPU**: Next-generation GPU acceleration
- **WebAssembly**: Near-native performance in browsers
- **Canvas2D**: Universal browser compatibility
- **Rust**: Type safety and memory safety

### Development Methodology
- **Test-Driven Development**: Complete TDD methodology
- **100% Test Coverage**: Comprehensive test suites
- **Performance Validation**: Automated benchmarking
- **Quality Gates**: Continuous integration and testing

## 📊 Performance Metrics

### Canvas2D Performance
- **100K Points**: <3ms render time ✅
- **Interactive Response**: <1ms hover detection ✅
- **Memory Efficiency**: <50MB for 1M points ✅
- **Frame Rate**: 60fps sustained rendering ✅

### WebGPU Performance
- **Shader Compilation**: 0.00-0.10ms ✅
- **Pipeline Creation**: 0.10-1.10ms ✅
- **Vertex Buffer Throughput**: 40-816 MB/s ✅
- **1M Points Rendering**: 77.40ms (12.9 FPS) ✅
- **Peak Throughput**: 816.33 MB/s ✅

### WebAssembly Performance
- **Function Execution**: <1ms ✅
- **String Processing**: Real-time ✅
- **JSON Generation**: Instant ✅
- **Memory Usage**: Efficient WebAssembly management ✅

## 🧪 Test-Driven Development

Helios demonstrates the complete TDD methodology:

### RED-GREEN-REFACTOR Cycle
1. **RED Phase**: Tests written first with expected failures
2. **GREEN Phase**: Implementation makes tests pass
3. **REFACTOR Phase**: Code improved while maintaining coverage

### Quality Assurance
- **Test Coverage**: 100% for all core functionality
- **Performance Tests**: All benchmarks meet or exceed targets
- **Integration Tests**: Cross-browser compatibility validation
- **Error Handling**: Comprehensive error scenario testing

## 🌐 Browser Compatibility

### WebGPU Support
- **Chrome 113+**: Full WebGPU support
- **Firefox 110+**: WebGPU support (experimental)
- **Safari**: WebGPU support (experimental)
- **Edge 113+**: Full WebGPU support

### WebAssembly Support
- **Chrome 57+**: Full WebAssembly support
- **Firefox 52+**: Full WebAssembly support
- **Safari 11+**: Full WebAssembly support
- **Edge 16+**: Full WebAssembly support

### Canvas2D Support
- **Universal**: All modern browsers support Canvas2D
- **Fallback**: Automatic fallback for unsupported features
- **Performance**: Optimized for all platforms

## 🛠️ Development Tools

### Testing Framework
- **Unit Tests**: Individual function testing
- **Integration Tests**: Component interaction testing
- **Performance Tests**: Benchmark validation
- **Property-Based Tests**: Edge case coverage with `proptest`
- **Mutation Tests**: Test quality validation with `cargo-mutants`

### Performance Monitoring
- **Real-time Metrics**: Live performance monitoring
- **Benchmarking**: Automated performance testing
- **Memory Profiling**: Memory usage tracking
- **Error Reporting**: Comprehensive error handling

## 📦 Installation

### From Crates.io
```toml
[dependencies]
leptos-helios = "0.2.0"
```

### From Source
```bash
git clone <repository-url>
cd leptos-helios
cargo build --release
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](contributing.md) for details on:

- Development setup
- Code style guidelines
- Testing requirements
- Pull request process

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Rust Community**: For the amazing Rust ecosystem
- **WebGPU Working Group**: For the WebGPU specification
- **Leptos Team**: For the excellent reactive framework
- **WebAssembly Community**: For the WASM standard

## 📞 Support

- **Documentation**: [docs.rs/leptos-helios](https://docs.rs/leptos-helios)
- **Issues**: [GitHub Issues](https://github.com/cloud-shuttle/leptos-helios/issues)
- **Discussions**: [GitHub Discussions](https://github.com/cloud-shuttle/leptos-helios/discussions)
- **Discord**: [Join our Discord](https://discord.gg/helios)

---

**Built with ❤️ using Rust, WebAssembly, WebGPU, and Test-Driven Development**
