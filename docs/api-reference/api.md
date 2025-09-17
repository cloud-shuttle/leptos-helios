# Helios API Reference

This document provides comprehensive API documentation for the Helios visualization library.

## Core API

### DataFrame

The `DataFrame` type is the primary data structure for storing and manipulating data.

```rust
use helios_core::DataFrame;

// Create empty DataFrame
let df = DataFrame::empty();

// Create DataFrame with data
let df = DataFrame::new(vec![
    Series::new("x", &[1, 2, 3, 4, 5]),
    Series::new("y", &[2, 4, 6, 8, 10]),
]);

// Load from CSV
let df = DataFrame::from_csv("data.csv")?;

// Load from JSON
let df = DataFrame::from_json("data.json")?;
```

#### Methods

- `new(columns: Vec<Series>) -> DataFrame` - Create new DataFrame
- `empty() -> DataFrame` - Create empty DataFrame
- `from_csv(path: &str) -> Result<DataFrame>` - Load from CSV file
- `from_json(path: &str) -> Result<DataFrame>` - Load from JSON file
- `column(&self, name: &str) -> Result<&Column>` - Get column by name
- `add_column(&mut self, series: Series) -> Result<()>` - Add new column
- `len(&self) -> usize` - Get number of rows
- `width(&self) -> usize` - Get number of columns

### Chart

The `Chart` type represents a visualization configuration.

```rust
use helios_core::{Chart, ChartType, TitleConfig, ScaleConfig};

let chart = Chart::new()
    .chart_type(ChartType::Line)
    .data(data)
    .title(TitleConfig::new("My Chart"))
    .x_scale(ScaleConfig::new().domain([0.0, 10.0]))
    .y_scale(ScaleConfig::new().domain([0.0, 20.0]))
    .width(800)
    .height(600);
```

#### Chart Types

```rust
pub enum ChartType {
    Line,
    Bar,
    Scatter,
    Area,
    Heatmap,
    Boxplot,
    Violin,
    Radar,
    Sankey,
    Treemap,
}
```

#### Configuration

```rust
// Title configuration
let title = TitleConfig::new("Chart Title")
    .font_size(16.0)
    .color("#333333")
    .position(TitlePosition::Top);

// Scale configuration
let scale = ScaleConfig::new()
    .domain([0.0, 100.0])
    .range([0.0, 800.0])
    .type_(ScaleType::Linear);

// Color configuration
let colors = ColorConfig::new()
    .palette(ColorPalette::Viridis)
    .opacity(0.8);
```

### MLPipeline

Machine learning pipeline for data analysis and visualization.

```rust
use helios_core::{MLPipeline, AnomalyMethod, ModelType};

let pipeline = MLPipeline::new()
    .add_anomaly_detection("value", AnomalyMethod::Statistical)
    .add_forecasting("value", 30)
    .add_clustering(&["feature1", "feature2"], 3)
    .model_type(ModelType::LinearRegression);
```

#### Methods

- `new() -> MLPipeline` - Create new pipeline
- `add_anomaly_detection(column: &str, method: AnomalyMethod) -> &mut Self` - Add anomaly detection
- `add_forecasting(column: &str, steps: usize) -> &mut Self` - Add forecasting
- `add_clustering(columns: &[&str], clusters: usize) -> &mut Self` - Add clustering
- `model_type(model: ModelType) -> &mut Self` - Set model type
- `detect_anomalies(&self, data: &DataFrame) -> Result<DataFrame>` - Detect anomalies
- `forecast(&self, data: &DataFrame) -> Result<DataFrame>` - Generate forecast
- `cluster(&self, data: &DataFrame) -> Result<DataFrame>` - Perform clustering

#### Anomaly Detection Methods

```rust
pub enum AnomalyMethod {
    Statistical,      // Z-score based
    IsolationForest,  // Isolation Forest algorithm
    LocalOutlier,     // Local Outlier Factor
    OneClassSVM,      // One-class SVM
}
```

#### Model Types

```rust
pub enum ModelType {
    LinearRegression,
    PolynomialRegression,
    ARIMA,
    LSTM,
    RandomForest,
    XGBoost,
}
```

### PerformanceManager

Manages performance optimization and resource allocation.

```rust
use helios_core::{PerformanceManager, PerformanceConfig};

let config = PerformanceConfig::new()
    .enable_webgpu(true)
    .enable_simd(true)
    .batch_size(1000)
    .max_memory_mb(512)
    .debug_level(DebugLevel::Info);

let manager = PerformanceManager::new(config);
```

#### Configuration Options

```rust
pub struct PerformanceConfig {
    pub enable_webgpu: bool,
    pub enable_simd: bool,
    pub batch_size: usize,
    pub max_memory_mb: usize,
    pub debug_level: DebugLevel,
    pub enable_profiling: bool,
    pub enable_caching: bool,
}
```

#### Debug Levels

```rust
pub enum DebugLevel {
    Silent,
    Error,
    Warning,
    Info,
    Debug,
    Verbose,
}
```

### DataProcessor

Handles data processing and transformation.

```rust
use helios_core::{DataProcessor, ProcessingConfig};

let processor = DataProcessor::new()
    .config(ProcessingConfig::new()
        .enable_parallel(true)
        .chunk_size(10000)
        .enable_caching(true));

let processed_data = processor.process(&raw_data)?;
```

#### Processing Methods

- `new() -> DataProcessor` - Create new processor
- `config(config: ProcessingConfig) -> &mut Self` - Set configuration
- `process(&self, data: &DataFrame) -> Result<DataFrame>` - Process data
- `filter(&self, predicate: &str) -> &mut Self` - Add filter
- `aggregate(&self, column: &str, operation: AggregationOp) -> &mut Self` - Add aggregation
- `transform(&self, column: &str, transform: TransformOp) -> &mut Self` - Add transformation

## Leptos Components

### HeliosChart

Main chart component for Leptos applications.

```rust
use helios_leptos::HeliosChart;

view! {
    <HeliosChart
        chart=chart_signal
        on_click=move |event| handle_click(event)
        on_hover=move |event| handle_hover(event)
    />
}
```

#### Props

- `chart: Signal<Chart>` - Chart configuration signal
- `on_click: Option<Callback<ChartEvent>>` - Click event handler
- `on_hover: Option<Callback<ChartEvent>>` - Hover event handler
- `class: Option<String>` - CSS class
- `style: Option<String>` - Inline styles

### DataLoader

Component for loading data from various sources.

```rust
use helios_leptos::DataLoader;

view! {
    <DataLoader
        on_load=move |df| set_data.set(df)
        format="csv"
        url="/api/data"
        auto_load=true
    />
}
```

#### Props

- `on_load: Callback<DataFrame>` - Data loaded callback
- `format: &str` - Data format (csv, json, parquet)
- `url: Option<&str>` - Data source URL
- `auto_load: bool` - Auto-load on mount
- `refresh_interval: Option<Duration>` - Auto-refresh interval

### VisualizationDashboard

Dashboard component for multiple visualizations.

```rust
use helios_leptos::VisualizationDashboard;

view! {
    <VisualizationDashboard
        dashboard=dashboard_signal
        layout=DashboardLayout::Grid
        responsive=true
    />
}
```

#### Props

- `dashboard: Signal<Dashboard>` - Dashboard configuration
- `layout: DashboardLayout` - Layout type
- `responsive: bool` - Enable responsive design
- `theme: Option<Theme>` - Theme configuration

## WebGPU Shaders

### Shader Types

Helios includes optimized WebGPU shaders for different chart types:

- `bar.wgsl` - Bar chart vertex shader
- `bar_fragment.wgsl` - Bar chart fragment shader
- `line.wgsl` - Line chart shader
- `scatter.wgsl` - Scatter plot shader
- `heatmap.wgsl` - Heatmap shader
- `area.wgsl` - Area chart shader

### Shader Usage

```rust
use helios_core::gpu::ShaderManager;

let shader_manager = ShaderManager::new();
let shader = shader_manager.load_shader("bar.wgsl")?;
```

## Error Handling

### HeliosError

Main error type for the library.

```rust
use helios_core::HeliosError;

match result {
    Ok(data) => println!("Success: {:?}", data),
    Err(HeliosError::DataProcessing(msg)) => println!("Data error: {}", msg),
    Err(HeliosError::MachineLearning(msg)) => println!("ML error: {}", msg),
    Err(HeliosError::WebGPU(msg)) => println!("WebGPU error: {}", msg),
    Err(e) => println!("Other error: {}", e),
}
```

#### Error Types

```rust
pub enum HeliosError {
    DataProcessing(String),
    MachineLearning(String),
    WebGPU(String),
    IO(String),
    Serialization(String),
    Validation(String),
}
```

## Utilities

### Test Utilities

```rust
use helios_core::test_utils::*;

// Create test data
let df = create_test_dataframe(1000);
let large_df = create_large_test_dataframe(10000);

// Performance testing
let config = create_test_performance_config();
let results = benchmark_data_processing(&df, &config);
```

### Data Validation

```rust
use helios_core::utils::validate_dataframe;

let validation_result = validate_dataframe(&df);
if !validation_result.is_valid {
    println!("Validation errors: {:?}", validation_result.errors);
}
```

## Configuration

### Build Configuration

```toml
# Cargo.toml
[package.metadata.wasm-pack.profile.release]
wasm-opt = false

[features]
default = ["webgpu"]
webgpu = []
webgl2 = []
canvas2d = []
simd = []
```

### Runtime Configuration

```rust
use helios_core::Config;

let config = Config::new()
    .enable_webgpu(true)
    .enable_webgl2(true)
    .enable_canvas2d(false)
    .enable_simd(true)
    .debug_mode(false);
```

## Performance Tips

1. **Use appropriate chart types** for your data size
2. **Enable SIMD** for numerical computations
3. **Use WebGPU** when available, fallback to WebGL2
4. **Batch data processing** for large datasets
5. **Enable caching** for repeated operations
6. **Use parallel processing** for data transformations

## Browser Compatibility

- **WebGPU**: Chrome 113+, Edge 113+
- **WebGL2**: All modern browsers
- **Canvas2D**: All browsers (fallback)
- **SIMD**: Chrome 37+, Firefox 28+, Safari 10.1+

## Examples

See the `helios-examples` crate for comprehensive usage examples covering all major features and use cases.
