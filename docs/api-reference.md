# API Reference

This document provides comprehensive API documentation for Helios.

## Core Types

### BaseChartConfig

Configuration for all chart types.

```rust
pub struct BaseChartConfig {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub x_label: String,
    pub y_label: String,
    pub show_grid: bool,
    pub background_color: String,
    pub text_color: String,
}
```

### Chart Types

#### LineChartConfig

```rust
pub struct LineChartConfig {
    pub base_config: BaseChartConfig,
    pub data: Vec<DataPoint>,
    pub color: String,
    pub show_legend: bool,
}
```

#### BarChartConfig

```rust
pub struct BarChartConfig {
    pub base_config: BaseChartConfig,
    pub data: Vec<BarData>,
    pub color: String,
    pub show_legend: bool,
}
```

#### ScatterPlotConfig

```rust
pub struct ScatterPlotConfig {
    pub base_config: BaseChartConfig,
    pub data: Vec<DataPoint>,
    pub point_shape: PointShape,
    pub color: String,
    pub show_legend: bool,
}
```

### Advanced Chart Types

#### RadarChartConfig

```rust
pub struct RadarChartConfig {
    pub base_config: BaseChartConfig,
    pub categories: Vec<String>,
    pub max_value: f64,
    pub show_grid: bool,
    pub show_labels: bool,
    pub fill_area: bool,
    pub stroke_width: f32,
    pub color: String,
}
```

#### SankeyConfig

```rust
pub struct SankeyConfig {
    pub base_config: BaseChartConfig,
    pub nodes: Vec<SankeyNode>,
    pub links: Vec<SankeyLink>,
    pub node_width: f32,
    pub node_padding: f32,
    pub link_opacity: f32,
    pub color_scheme: ColorScheme,
}
```

## Performance API

### PerformanceManager

```rust
pub struct PerformanceManager {
    // ... fields
}

impl PerformanceManager {
    pub fn new(config: PerformanceConfig) -> Self;
    pub fn process_data(&mut self, data: &[f64], viewport_scale: f64) -> Result<PerformanceMetrics, WebGpuError>;
    pub fn get_average_fps(&self) -> f64;
    pub fn is_performance_target_met(&self) -> bool;
}
```

### LodSystem

```rust
pub struct LodSystem {
    // ... fields
}

impl LodSystem {
    pub fn new() -> Self;
    pub fn update_lod(&mut self, viewport_scale: f64, data_size: usize);
    pub fn get_current_lod(&self) -> &LodLevel;
    pub fn sample_data(&self, data: &[f64]) -> Vec<f64>;
}
```

## Error Types

### WebGpuError

```rust
#[derive(Error, Debug)]
pub enum WebGpuError {
    #[error("WebGPU device initialization failed: {0}")]
    DeviceInit(String),

    #[error("Surface creation failed: {0}")]
    SurfaceCreation(String),

    #[error("Shader compilation failed: {0}")]
    ShaderCompilation(String),

    #[error("Render pipeline creation failed: {0}")]
    PipelineCreation(String),

    #[error("Buffer allocation failed: {0}")]
    BufferAllocation(String),
}
```

## Utility Functions

### Data Processing

```rust
pub fn process_data_points(data: &[f64]) -> Result<Vec<f64>, WebGpuError>;
pub fn normalize_data(data: &[f64]) -> Vec<f64>;
pub fn interpolate_data(data: &[f64], target_size: usize) -> Vec<f64>;
```

### Color Utilities

```rust
pub fn hex_to_rgba(hex: &str) -> Result<[f32; 4], String>;
pub fn rgba_to_hex(rgba: [f32; 4]) -> String;
pub fn generate_color_palette(count: usize) -> Vec<String>;
```

## Examples

See the [Examples](examples.md) section for complete usage examples.
