# Troubleshooting Guide

> Common issues and solutions for Helios development and deployment

## Quick Diagnosis

### Performance Issues
- **Slow Rendering**: Check WebGPU support and fallback status
- **High Memory Usage**: Verify data size and streaming configuration
- **Poor Responsiveness**: Check interaction latency and frame timing

### Build Issues
- **WASM Compilation Errors**: Verify Rust toolchain and target installation
- **Bundle Size Too Large**: Check optimization flags and dependencies
- **Runtime Errors**: Verify browser compatibility and feature support

### Integration Issues
- **Leptos Integration**: Check version compatibility and feature flags
- **Data Processing**: Verify Polars version and DataFrame format
- **Server Functions**: Check network configuration and CORS settings

## Common Issues and Solutions

### 1. WebGPU Not Available

**Symptoms:**
- Charts fall back to WebGL2 or Canvas rendering
- Performance is slower than expected
- Console warnings about WebGPU support

**Solutions:**

```rust
// Check WebGPU support
if !webgpu::is_supported().await {
    console::warn_1(&"WebGPU not supported, falling back to WebGL2".into());
}

// Force WebGL2 fallback for testing
#[cfg(debug_assertions)]
std::env::set_var("HELIOS_FORCE_WEBGL2", "true");
```

**Browser Requirements:**
- Chrome 113+ (stable WebGPU support)
- Safari 17+ (WebGPU support)
- Firefox 115+ (WebGPU support in development)

### 2. WASM Compilation Errors

**Symptoms:**
- `cargo build` fails with WASM target
- Missing `wasm32-unknown-unknown` target
- Linker errors during WASM compilation

**Solutions:**

```bash
# Install WASM target
rustup target add wasm32-unknown-unknown

# Install WASM tools
cargo install trunk wasm-pack wasm-opt

# Clean and rebuild
cargo clean
trunk build --release
```

**Common Fixes:**
```toml
# Cargo.toml - ensure proper configuration
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1

[dependencies]
# Use compatible versions
leptos = { version = "0.8", features = ["csr", "hydrate"] }
polars = { version = "1.30", features = ["lazy"] }
```

### 3. Memory Issues with Large Datasets

**Symptoms:**
- Browser crashes or becomes unresponsive
- High memory usage in task manager
- "Out of memory" errors

**Solutions:**

```rust
// Enable streaming for large datasets
let chart = helios::chart! {
    data: large_dataset,
    mark: Point { size: Some(1.0) },
    encoding: {
        x: { field: "x", type: Quantitative },
        y: { field: "y", type: Quantitative }
    }
};

view! {
    <HeliosChart
        spec=chart
        performance=PerformanceConfig::new()
            .memory_limit(Some(100 * 1024 * 1024)) // 100MB limit
            .quality_mode(QualityMode::Performance)
    />
}
```

**Data Processing Optimization:**
```rust
// Use lazy evaluation and limit data
let processed_data = df
    .lazy()
    .filter(col("value").gt(0))
    .limit(Some(100_000)) // Limit to 100K rows
    .collect()
    .unwrap();
```

### 4. Leptos Integration Issues

**Symptoms:**
- Components not rendering
- Reactive updates not working
- Server function errors

**Solutions:**

```rust
// Ensure proper Leptos setup
use leptos::*;

#[component]
pub fn MyChart() -> impl IntoView {
    let (data, set_data) = create_signal(DataFrame::empty());

    // Use create_memo for derived state
    let chart_spec = create_memo(move |_| {
        helios::chart! {
            data: data.get(),
            mark: Line,
            encoding: {
                x: { field: "x", type: Quantitative },
                y: { field: "y", type: Quantitative }
            }
        }
    });

    view! {
        <HeliosChart spec=chart_spec />
    }
}
```

**Server Function Issues:**
```rust
// Ensure proper server function configuration
#[server(LoadData, "/api")]
pub async fn load_data() -> Result<DataFrame, ServerFnError> {
    // Implementation
}

// Check CORS and network configuration
#[cfg(feature = "ssr")]
pub fn configure_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
}
```

### 5. Data Processing Errors

**Symptoms:**
- DataFrame operations fail
- Type mismatches in data
- Performance issues with data transformations

**Solutions:**

```rust
// Validate data before processing
fn validate_dataframe(df: &DataFrame) -> Result<(), DataError> {
    if df.is_empty() {
        return Err(DataError::EmptyData);
    }

    // Check required columns
    let required_columns = ["x", "y"];
    for col in required_columns {
        if !df.get_column_names().contains(&col) {
            return Err(DataError::MissingColumn(col.to_string()));
        }
    }

    Ok(())
}

// Handle data type conversions
let processed_df = df
    .lazy()
    .with_columns([
        col("x").cast(DataType::Float64),
        col("y").cast(DataType::Float64),
    ])
    .filter(col("x").is_not_null())
    .filter(col("y").is_not_null())
    .collect()
    .unwrap();
```

### 6. Performance Issues

**Symptoms:**
- Slow chart rendering
- Poor frame rates
- High CPU usage

**Solutions:**

```rust
// Enable performance monitoring
view! {
    <HeliosChart
        spec=chart_spec
        debug=true  // Shows performance metrics
        performance=PerformanceConfig::new()
            .target_fps(Some(60))
            .quality_mode(QualityMode::Adaptive {
                target_frame_time: Duration::from_millis(16),
                quality_range: (0.5, 1.0)
            })
    />
}
```

**Optimization Strategies:**
```rust
// Use appropriate chart types for data size
let chart_type = match data_size {
    size if size < 1_000 => MarkType::Point { size: Some(8.0) },
    size if size < 10_000 => MarkType::Point { size: Some(4.0) },
    size if size < 100_000 => MarkType::Point { size: Some(2.0) },
    _ => MarkType::Point { size: Some(1.0) },
};

// Enable LOD for large datasets
let chart = helios::chart! {
    data: large_dataset,
    mark: chart_type,
    encoding: {
        x: { field: "x", type: Quantitative },
        y: { field: "y", type: Quantitative }
    },
    config: {
        lod: LODConfig {
            enabled: true,
            thresholds: vec![1000, 10000, 100000],
        }
    }
};
```

### 7. Browser Compatibility Issues

**Symptoms:**
- Charts not rendering in certain browsers
- Different behavior across browsers
- Console errors about unsupported features

**Solutions:**

```rust
// Check browser capabilities
pub async fn check_browser_support() -> BrowserSupport {
    let webgpu_supported = webgpu::is_supported().await;
    let webgl2_supported = webgl2::is_supported();
    let canvas_supported = canvas2d::is_supported();

    BrowserSupport {
        webgpu: webgpu_supported,
        webgl2: webgl2_supported,
        canvas2d: canvas_supported,
        recommended_backend: if webgpu_supported {
            RenderBackend::WebGPU
        } else if webgl2_supported {
            RenderBackend::WebGL2
        } else {
            RenderBackend::Canvas2D
        }
    }
}

// Graceful degradation
let render_config = match browser_support.recommended_backend {
    RenderBackend::WebGPU => RenderConfig::high_performance(),
    RenderBackend::WebGL2 => RenderConfig::medium_performance(),
    RenderBackend::Canvas2D => RenderConfig::compatibility(),
};
```

### 8. Build and Deployment Issues

**Symptoms:**
- Build failures in CI/CD
- Deployment errors
- Asset loading issues

**Solutions:**

```bash
# CI/CD configuration
name: Build and Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown
          override: true

      - name: Install trunk
        run: cargo install trunk

      - name: Build
        run: trunk build --release

      - name: Test
        run: cargo test
```

**Docker Configuration:**
```dockerfile
FROM rust:1.79-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev curl \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js for WASM tools
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
    && apt-get install -y nodejs

# Install Rust tools
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-opt

WORKDIR /app
COPY . .

# Build WASM
RUN trunk build --release

# Build server
RUN cargo build --release --bin server
```

## Debugging Tools

### 1. Performance Profiler

```rust
// Enable detailed performance monitoring
#[cfg(debug_assertions)]
pub fn enable_performance_debugging() {
    std::env::set_var("HELIOS_DEBUG_PERFORMANCE", "true");
    std::env::set_var("HELIOS_DEBUG_MEMORY", "true");
    std::env::set_var("HELIOS_DEBUG_GPU", "true");
}

// Performance metrics component
#[component]
pub fn PerformanceMonitor() -> impl IntoView {
    let (metrics, set_metrics) = create_signal(PerformanceMetrics::default());

    use_interval_fn(
        move || {
            if let Some(renderer) = get_renderer_instance() {
                set_metrics.set(renderer.collect_metrics());
            }
        },
        Duration::from_secs(1)
    );

    view! {
        <div class="performance-monitor">
            <div>"FPS: " {move || format!("{:.1}", metrics.get().fps())}</div>
            <div>"Memory: " {move || format!("{:.1}MB", metrics.get().memory_usage().used as f64 / 1024.0 / 1024.0)}</div>
            <div>"Frame Time: " {move || format!("{:.1}ms", metrics.get().frame_time().as_millis())}</div>
        </div>
    }
}
```

### 2. Data Inspector

```rust
// Debug data processing
pub fn debug_dataframe(df: &DataFrame, name: &str) {
    #[cfg(debug_assertions)]
    {
        console::log_1(&format!("DataFrame '{}': {} rows, {} columns",
            name, df.height(), df.width()).into());

        for col_name in df.get_column_names() {
            let series = df.column(col_name).unwrap();
            console::log_1(&format!("  {}: {:?} ({} nulls)",
                col_name, series.dtype(), series.null_count()).into());
        }
    }
}
```

### 3. Error Boundary

```rust
// Catch and display errors gracefully
#[component]
pub fn ErrorBoundary<F, IV>(children: F) -> impl IntoView
where
    F: Fn() -> IV + 'static,
    IV: IntoView,
{
    let (error, set_error) = create_signal(None::<String>);

    view! {
        <div class="error-boundary">
            <Show
                when=move || error.get().is_some()
                fallback=move || children()
            >
                <div class="error-display">
                    <h3>"Something went wrong"</h3>
                    <p>{move || error.get().unwrap_or_default()}</p>
                    <button on:click=move |_| set_error.set(None)>
                        "Try Again"
                    </button>
                </div>
            </Show>
        </div>
    }
}
```

## Getting Help

### 1. Check Documentation
- [Getting Started Guide](getting-started.md)
- [API Reference](api.md)
- [Performance Guide](performance.md)
- [Architecture Overview](architecture.md)

### 2. Search Issues
- [GitHub Issues](https://github.com/cloudshuttle/helios/issues)
- [GitHub Discussions](https://github.com/cloudshuttle/helios/discussions)

### 3. Community Support
- **Discord**: [Real-time chat](https://discord.gg/helios)
- **GitHub Discussions**: [General questions](https://github.com/cloudshuttle/helios/discussions)
- **Stack Overflow**: Tag questions with `helios` and `rust`

### 4. Report Issues
When reporting issues, please include:

1. **Environment**: OS, browser, Rust version
2. **Reproduction Steps**: Clear steps to reproduce the issue
3. **Expected Behavior**: What you expected to happen
4. **Actual Behavior**: What actually happened
5. **Error Messages**: Full error messages and stack traces
6. **Code Sample**: Minimal code that reproduces the issue

### 5. Performance Issues
For performance issues, include:

1. **Performance Metrics**: FPS, memory usage, frame times
2. **Data Size**: Number of data points, columns, rows
3. **Chart Configuration**: Chart type, encoding, interactions
4. **Browser Information**: Version, WebGPU support, hardware
5. **Performance Profile**: Screenshots of performance tools

## Prevention Tips

### 1. Development Best Practices
- Use TypeScript/strong typing where possible
- Test with different data sizes and types
- Monitor performance during development
- Use debug mode for development

### 2. Production Considerations
- Enable performance monitoring
- Set appropriate memory limits
- Use streaming for large datasets
- Implement error boundaries

### 3. Testing Strategy
- Test across different browsers
- Test with various data sizes
- Test performance under load
- Test error conditions

---

**Still having issues?** Join our [Discord community](https://discord.gg/helios) for real-time help or create a [GitHub issue](https://github.com/cloudshuttle/helios/issues) with detailed information about your problem.
