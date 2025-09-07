# Getting Started with Helios

Helios is a high-performance Rust visualization library for Leptos v0.8, designed for creating interactive data visualizations with WebGPU acceleration.

## Quick Start

### Installation

Add Helios to your `Cargo.toml`:

```toml
[dependencies]
leptos-helios = "0.1.0-beta.1"
leptos = "0.8"
```

### Basic Usage

```rust
use leptos::*;
use leptos_helios::*;

#[component]
pub fn MyChart() -> impl IntoView {
    let config = LineChartConfig {
        base_config: BaseChartConfig {
            width: 800,
            height: 600,
            title: "My First Chart".to_string(),
            x_label: "X Axis".to_string(),
            y_label: "Y Axis".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        data: vec![
            DataPoint { x: 1.0, y: 2.0 },
            DataPoint { x: 2.0, y: 4.0 },
            DataPoint { x: 3.0, y: 6.0 },
        ],
        color: "#3b82f6".to_string(),
        show_legend: true,
    };

    view! {
        <HeliosChart config=config />
    }
}
```

## Chart Types

Helios supports a wide range of chart types:

### Basic Charts
- **Line Charts**: Perfect for time series data
- **Bar Charts**: Great for categorical comparisons
- **Scatter Plots**: Ideal for correlation analysis
- **Heatmaps**: Excellent for matrix data visualization

### Advanced Charts
- **Radar Charts**: Multi-dimensional data visualization
- **Sankey Diagrams**: Flow visualization for hierarchical data
- **Treemaps**: Hierarchical data with nested rectangles
- **Violin Plots**: Distribution visualization with density

## Performance Features

Helios is optimized for high-performance visualization:

- **WebGPU Acceleration**: Hardware-accelerated rendering
- **SIMD Optimization**: Vectorized data processing
- **Level of Detail (LOD)**: Adaptive detail for large datasets
- **Memory Pooling**: Efficient buffer management
- **Background Processing**: Web Workers for data processing

## Next Steps

- Check out the [API Reference](api-reference.md)
- Explore [Examples](examples.md)
- Learn about [Performance Optimization](performance-guide.md)
- Troubleshoot issues in [Troubleshooting](troubleshooting.md)