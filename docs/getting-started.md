# Getting Started with Helios

Helios is a high-performance Rust visualization library designed for Leptos v0.8. It provides WebGPU-accelerated charting capabilities with machine learning integration.

## Quick Start

### Prerequisites

- Rust 1.70+ with `wasm32-unknown-unknown` target
- Node.js 16+ (for development server)
- Modern browser with WebGPU support

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/cloudshuttle/helios.git
   cd helios
   ```

2. **Install Rust toolchain:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add wasm32-unknown-unknown
   ```

3. **Install build tools:**
   ```bash
   make install-tools
   # or manually:
   cargo install wasm-pack trunk
   ```

### Basic Usage

#### 1. Create a Simple Chart

```rust
use helios_core::{DataFrame, Chart, ChartType};
use helios_leptos::HeliosChart;

// Create sample data
let data = DataFrame::new(vec![
    Series::new("x", &[1, 2, 3, 4, 5]),
    Series::new("y", &[2, 4, 6, 8, 10]),
]);

// Create chart configuration
let chart = Chart::new()
    .chart_type(ChartType::Line)
    .data(data)
    .title("Simple Line Chart");

// Use in Leptos component
view! {
    <HeliosChart chart=chart />
}
```

#### 2. Advanced Visualization with ML

```rust
use helios_core::{DataFrame, Chart, ChartType, MLPipeline};
use helios_leptos::VisualizationDashboard;

// Load your data
let data = DataFrame::from_csv("data.csv")?;

// Create ML pipeline for anomaly detection
let ml_pipeline = MLPipeline::new()
    .add_anomaly_detection("value", AnomalyMethod::Statistical)
    .add_forecasting("value", 10);

// Create dashboard
let dashboard = VisualizationDashboard::new()
    .add_chart(Chart::new()
        .chart_type(ChartType::Line)
        .data(data.clone())
        .title("Time Series Data"))
    .add_chart(Chart::new()
        .chart_type(ChartType::Scatter)
        .data(ml_pipeline.detect_anomalies(&data)?)
        .title("Anomaly Detection"));

view! {
    <VisualizationDashboard dashboard=dashboard />
}
```

### Development Workflow

#### 1. Start Development Server

```bash
make dev
# or
trunk serve --open
```

#### 2. Run Tests

```bash
make test
# or
cargo test --workspace
```

#### 3. Build for Production

```bash
make build
# or
./build.sh build
```

### Chart Types

Helios supports a wide variety of chart types:

- **Line Charts** - Time series and continuous data
- **Bar Charts** - Categorical comparisons
- **Scatter Plots** - Correlation analysis
- **Heatmaps** - 2D data visualization
- **Area Charts** - Filled line charts
- **Box Plots** - Statistical distributions
- **Violin Plots** - Distribution shapes
- **Radar Charts** - Multi-dimensional data
- **Sankey Diagrams** - Flow visualization
- **Treemaps** - Hierarchical data

### Machine Learning Features

#### Anomaly Detection

```rust
use helios_core::{MLPipeline, AnomalyMethod};

let pipeline = MLPipeline::new()
    .add_anomaly_detection("temperature", AnomalyMethod::Statistical)
    .add_anomaly_detection("pressure", AnomalyMethod::IsolationForest);

let anomalies = pipeline.detect_anomalies(&data)?;
```

#### Time Series Forecasting

```rust
use helios_core::{MLPipeline, ModelType};

let pipeline = MLPipeline::new()
    .add_forecasting("sales", 30)
    .model_type(ModelType::LinearRegression);

let forecast = pipeline.forecast(&data)?;
```

#### Clustering

```rust
use helios_core::{MLPipeline, ClusterAnalyzer};

let pipeline = MLPipeline::new()
    .add_clustering(&["feature1", "feature2"], 3);

let clusters = pipeline.cluster(&data)?;
```

### Performance Optimization

#### WebGPU Configuration

```rust
use helios_core::{PerformanceConfig, PerformanceManager};

let config = PerformanceConfig::new()
    .enable_webgpu(true)
    .enable_simd(true)
    .batch_size(1000)
    .max_memory_mb(512);

let manager = PerformanceManager::new(config);
```

#### Data Processing

```rust
use helios_core::{DataProcessor, ProcessingConfig};

let processor = DataProcessor::new()
    .config(ProcessingConfig::new()
        .enable_parallel(true)
        .chunk_size(10000)
        .enable_caching(true));

let processed_data = processor.process(&raw_data)?;
```

### Integration with Leptos

#### Component Props

```rust
use helios_leptos::{HeliosChart, DataLoader};

#[component]
pub fn MyDashboard() -> impl IntoView {
    let (data, set_data) = create_signal(DataFrame::empty());

    view! {
        <DataLoader
            on_load=move |df| set_data.set(df)
            format="csv"
        />
        <HeliosChart
            chart=move || Chart::new()
                .data(data.get())
                .chart_type(ChartType::Line)
        />
    }
}
```

#### Reactive Updates

```rust
use helios_leptos::HeliosChart;

#[component]
pub fn ReactiveChart() -> impl IntoView {
    let (data, set_data) = create_signal(DataFrame::empty());

    // Update data reactively
    create_effect(move |_| {
        // Your data update logic here
        let new_data = fetch_data();
        set_data.set(new_data);
    });

    view! {
        <HeliosChart
            chart=move || Chart::new()
                .data(data.get())
                .chart_type(ChartType::Bar)
        />
    }
}
```

### Configuration

#### Trunk Configuration

```toml
# Trunk.toml
[build]
target = "dist"
index = "index.html"

[serve]
host = "127.0.0.1"
port = 8080
watch = true
open = true

[tools]
wasm_opt = "false"  # Disable for compatibility

[features]
webgpu = true
webgl2 = true
simd = true
```

#### Cargo Configuration

```toml
# Cargo.toml
[package.metadata.wasm-pack.profile.release]
wasm-opt = false  # Disable for compatibility

[features]
default = ["webgpu"]
webgpu = []
```

### Troubleshooting

#### Common Issues

1. **WebGPU not supported**: Enable WebGL2 fallback
2. **WASM optimization errors**: Disable `wasm-opt` in configuration
3. **Memory issues**: Reduce batch size or enable chunking
4. **Performance issues**: Enable SIMD and parallel processing

#### Debug Mode

```rust
use helios_core::{PerformanceConfig, DebugLevel};

let config = PerformanceConfig::new()
    .debug_level(DebugLevel::Verbose)
    .enable_profiling(true);
```

### Examples

Check the `helios-examples` crate for comprehensive examples:

- Basic chart creation
- Interactive dashboards
- ML integration
- Performance optimization
- Custom styling

### API Reference

For detailed API documentation, see:
- [Core API](api.md)
- [Leptos Components](components.md)
- [Performance Guide](performance.md)
- [Architecture Overview](architecture.md)

### Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for development guidelines.

### License

MIT License - see [LICENSE](../LICENSE) for details.
