# AGENTS.md - Leptos Helios Development Guide

## Build/Test/Lint Commands

- **Build**: `make build` or `cargo build --workspace` or `pnpm run build`
- **Test**: `cargo test --workspace` or `pnpm test` - Run all tests
- **Test Single**: `cargo test test_name` - Run specific test by name
- **Test WASM**: `pnpm run test:wasm` or `wasm-pack test --headless --firefox`
- **Test E2E**: `pnpm run test:e2e` (Playwright tests)
- **Lint**: `cargo clippy --all-targets --all-features -- -D warnings`
- **Format**: `cargo fmt --all`
- **Check**: `cargo check --workspace`
- **Coverage**: `cargo tarpaulin --all-features --out Html` (requires `tarpaulin` installed)
- **TDD Cycle**: `make tdd-cycle` (RED-GREEN-REFACTOR workflow)
- **Dev Server**: `make dev` or `python3 demo-server.py` (serves on localhost:8080)

## Architecture & Codebase Structure

**Multi-Crate Workspace** (Cargo workspace with 8 crates):
- `helios-core` - Core visualization engine, WebGPU/Canvas2D rendering, chart specs
- `helios-leptos` - Leptos v0.8 components and integration
- `helios-macros` - Procedural macros for chart generation
- `helios-wasm` - WebAssembly bindings and exports
- `helios-wasm-core` - Core WASM functionality
- `helios-examples` - Usage examples and demos
- `helios-benchmarks` - Performance benchmarking
- `helios-app` - Demo application

**Key Technologies**: Rust + WebAssembly, Leptos v0.8, WebGPU, Polars (DataFrames), wasm-pack, Trunk

**Core Modules** (helios-core/src/):
- `chart/` - Chart specification system with compile-time validation
- `rendering/` - WebGPU device management and rendering pipeline
- `accessibility/` - WCAG 2.1 AA compliance, screen reader support
- `data_sources/` - PostgreSQL, ClickHouse, JSON/CSV adapters
- `security/` - OAuth2, SAML, RBAC, audit logging
- `export_system/` - PNG, SVG, PDF, HTML export formats

## Code Style & Conventions

**Imports**: Use `pub use` for re-exports, group std/external/internal imports, prefer full module paths
**Error Handling**: Use `thiserror::Error` for custom errors, `anyhow` for application errors
**Async**: Tokio runtime, `async_trait` for async traits
**Types**: Strong typing with serde serialization, prefer `Arc<RwLock<T>>` for shared state
**Naming**: snake_case for functions/variables, PascalCase for types, UPPER_SNAKE_CASE for constants
**Testing**: Unit tests in each module, integration tests in `/tests`, use `rstest` for parameterized tests
**Documentation**: Use `//!` for module docs, `///` for public APIs with examples
**WebAssembly**: Use `wasm-bindgen` for JS interop, `console_error_panic_hook` for debugging

**Performance**: SIMD optimization preferred, memory pooling for large datasets, WebGPU for >10K points
**Security**: Never log secrets, use data classification, audit all data access operations
