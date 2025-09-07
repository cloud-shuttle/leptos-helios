# Examples

This document provides comprehensive examples for all Helios chart types and features.

## 🚀 WebGPU Demo

The WebGPU demo showcases the real WebGPU implementation with performance benchmarking:

```bash
cargo run --example webgpu_demo
```

This example demonstrates:
- WebGPU device initialization
- Shader compilation and caching
- Render pipeline creation
- Vertex buffer management
- Performance benchmarking (291.79 MB/s throughput)

## v0.1.0-alpha Features

The current alpha release includes:
- ✅ **Canvas Surface Integration**: WebGPU canvas connection
- ✅ **Line Chart Rendering**: Data processing and coordinate mapping
- ✅ **Leptos Component**: Working `HeliosChart` component
- ✅ **Fallback System**: WebGL2/Canvas2D automatic detection
- ✅ **48 Tests Passing**: Comprehensive test coverage

## Basic Charts

### Line Chart

```rust
use leptos::*;
use leptos_helios::*;

#[component]
pub fn LineChartExample() -> impl IntoView {
    let config = LineChartConfig {
        base_config: BaseChartConfig {
            width: 800,
            height: 600,
            title: "Sales Over Time".to_string(),
            x_label: "Month".to_string(),
            y_label: "Sales ($)".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        data: vec![
            DataPoint { x: 1.0, y: 1000.0 },
            DataPoint { x: 2.0, y: 1200.0 },
            DataPoint { x: 3.0, y: 1100.0 },
            DataPoint { x: 4.0, y: 1400.0 },
            DataPoint { x: 5.0, y: 1600.0 },
        ],
        color: "#3b82f6".to_string(),
        show_legend: true,
    };

    view! {
        <HeliosChart config=config />
    }
}
```

### Bar Chart

```rust
#[component]
pub fn BarChartExample() -> impl IntoView {
    let config = BarChartConfig {
        base_config: BaseChartConfig {
            width: 800,
            height: 600,
            title: "Product Sales".to_string(),
            x_label: "Product".to_string(),
            y_label: "Sales ($)".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        data: vec![
            BarData { label: "Product A".to_string(), value: 1000.0 },
            BarData { label: "Product B".to_string(), value: 1200.0 },
            BarData { label: "Product C".to_string(), value: 800.0 },
            BarData { label: "Product D".to_string(), value: 1500.0 },
        ],
        color: "#10b981".to_string(),
        show_legend: true,
    };

    view! {
        <HeliosChart config=config />
    }
}
```

## Advanced Charts

### Radar Chart

```rust
#[component]
pub fn RadarChartExample() -> impl IntoView {
    let config = RadarChartConfig {
        base_config: BaseChartConfig {
            width: 800,
            height: 600,
            title: "Performance Metrics".to_string(),
            x_label: "".to_string(),
            y_label: "".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        categories: vec![
            "Performance".to_string(),
            "Usability".to_string(),
            "Features".to_string(),
            "Support".to_string(),
            "Price".to_string(),
        ],
        max_value: 100.0,
        show_grid: true,
        show_labels: true,
        fill_area: true,
        stroke_width: 2.0,
        color: "#8b5cf6".to_string(),
    };

    let data = vec![
        RadarDataPoint { category: "Performance".to_string(), value: 85.0, max_value: 100.0 },
        RadarDataPoint { category: "Usability".to_string(), value: 92.0, max_value: 100.0 },
        RadarDataPoint { category: "Features".to_string(), value: 78.0, max_value: 100.0 },
        RadarDataPoint { category: "Support".to_string(), value: 88.0, max_value: 100.0 },
        RadarDataPoint { category: "Price".to_string(), value: 95.0, max_value: 100.0 },
    ];

    view! {
        <HeliosChart config=config data=data />
    }
}
```

### Sankey Diagram

```rust
#[component]
pub fn SankeyExample() -> impl IntoView {
    let config = SankeyConfig {
        base_config: BaseChartConfig {
            width: 800,
            height: 600,
            title: "Data Flow".to_string(),
            x_label: "".to_string(),
            y_label: "".to_string(),
            show_grid: false,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        nodes: vec![
            SankeyNode { id: "source1".to_string(), name: "Source 1".to_string(), value: 100.0 },
            SankeyNode { id: "source2".to_string(), name: "Source 2".to_string(), value: 80.0 },
            SankeyNode { id: "target1".to_string(), name: "Target 1".to_string(), value: 0.0 },
            SankeyNode { id: "target2".to_string(), name: "Target 2".to_string(), value: 0.0 },
        ],
        links: vec![
            SankeyLink { source: "source1".to_string(), target: "target1".to_string(), value: 60.0 },
            SankeyLink { source: "source1".to_string(), target: "target2".to_string(), value: 40.0 },
            SankeyLink { source: "source2".to_string(), target: "target1".to_string(), value: 30.0 },
            SankeyLink { source: "source2".to_string(), target: "target2".to_string(), value: 50.0 },
        ],
        node_width: 20.0,
        node_padding: 10.0,
        link_opacity: 0.6,
        color_scheme: ColorScheme::Viridis,
    };

    view! {
        <HeliosChart config=config />
    }
}
```

## Performance Examples

### Large Dataset Rendering

```rust
#[component]
pub fn LargeDatasetExample() -> impl IntoView {
    let (data, set_data) = create_signal(generate_large_dataset(100000));
    let (performance_metrics, set_performance_metrics) = create_signal(None);

    let config = LineChartConfig {
        base_config: BaseChartConfig {
            width: 1200,
            height: 800,
            title: "Large Dataset (100K points)".to_string(),
            x_label: "Index".to_string(),
            y_label: "Value".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        data: data(),
        color: "#3b82f6".to_string(),
        show_legend: true,
    };

    // Performance monitoring
    let update_performance = move || {
        let mut performance_manager = PerformanceManager::new(PerformanceConfig::default());
        let metrics = performance_manager.process_data(&data(), 1.0).unwrap();
        set_performance_metrics.set(Some(metrics));
    };

    view! {
        <div>
            <HeliosChart config=config />
            <div>
                <button on:click=move |_| update_performance()>
                    "Check Performance"
                </button>
                {move || {
                    if let Some(metrics) = performance_metrics() {
                        view! {
                            <div>
                                <p>"FPS: " {metrics.fps}</p>
                                <p>"Frame Time: " {metrics.frame_time_ms} "ms"</p>
                                <p>"Memory Usage: " {metrics.memory_usage_bytes} " bytes"</p>
                            </div>
                        }
                    } else {
                        view! { <div></div> }
                    }
                }}
            </div>
        </div>
    }
}

fn generate_large_dataset(size: usize) -> Vec<DataPoint> {
    (0..size)
        .map(|i| DataPoint {
            x: i as f64,
            y: (i as f64 * 0.1).sin() * 100.0 + (i as f64 * 0.05).cos() * 50.0,
        })
        .collect()
}
```

### Real-time Streaming

```rust
#[component]
pub fn StreamingExample() -> impl IntoView {
    let (data, set_data) = create_signal(Vec::<DataPoint>::new());
    let (is_streaming, set_is_streaming) = create_signal(false);

    let config = LineChartConfig {
        base_config: BaseChartConfig {
            width: 800,
            height: 600,
            title: "Real-time Data Stream".to_string(),
            x_label: "Time".to_string(),
            y_label: "Value".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        data: data(),
        color: "#ef4444".to_string(),
        show_legend: true,
    };

    let start_streaming = move || {
        set_is_streaming.set(true);
        // In real implementation, this would start a WebSocket or similar
        spawn_local(async move {
            let mut counter = 0.0;
            loop {
                if !is_streaming() {
                    break;
                }

                let new_point = DataPoint {
                    x: counter,
                    y: (counter * 0.1).sin() * 100.0,
                };

                set_data.update(|data| {
                    data.push(new_point);
                    if data.len() > 1000 {
                        data.remove(0);
                    }
                });

                counter += 1.0;
                gloo_timers::future::TimeoutFuture::new(100).await;
            }
        });
    };

    let stop_streaming = move || {
        set_is_streaming.set(false);
    };

    view! {
        <div>
            <HeliosChart config=config />
            <div>
                <button on:click=move |_| start_streaming() disabled=is_streaming>
                    "Start Streaming"
                </button>
                <button on:click=move |_| stop_streaming() disabled=!is_streaming>
                    "Stop Streaming"
                </button>
            </div>
        </div>
    }
}
```

## Interactive Examples

### Zoom and Pan

```rust
#[component]
pub fn InteractiveExample() -> impl IntoView {
    let (viewport, set_viewport) = create_signal(Viewport::new(0.0, 0.0, 1.0, 1.0));
    let (data, _) = create_signal(generate_interactive_data());

    let config = ScatterPlotConfig {
        base_config: BaseChartConfig {
            width: 800,
            height: 600,
            title: "Interactive Scatter Plot".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        data: data(),
        point_shape: PointShape::Circle,
        color: "#8b5cf6".to_string(),
        show_legend: true,
    };

    let handle_zoom = move |delta: f64| {
        set_viewport.update(|v| {
            let scale_factor = if delta > 0.0 { 1.1 } else { 0.9 };
            v.scale_x *= scale_factor;
            v.scale_y *= scale_factor;
        });
    };

    let handle_pan = move |dx: f64, dy: f64| {
        set_viewport.update(|v| {
            v.x += dx;
            v.y += dy;
        });
    };

    view! {
        <div>
            <HeliosChart config=config viewport=viewport />
            <div>
                <button on:click=move |_| handle_zoom(1.0)>"Zoom In"</button>
                <button on:click=move |_| handle_zoom(-1.0)>"Zoom Out"</button>
                <button on:click=move |_| handle_pan(-10.0, 0.0)>"Pan Left"</button>
                <button on:click=move |_| handle_pan(10.0, 0.0)>"Pan Right"</button>
            </div>
        </div>
    }
}

fn generate_interactive_data() -> Vec<DataPoint> {
    (0..1000)
        .map(|i| DataPoint {
            x: (i as f64 * 0.1).sin() * 100.0,
            y: (i as f64 * 0.1).cos() * 100.0,
        })
        .collect()
}
```

## Next Steps

- Explore the [Performance Guide](performance-guide.md) for optimization tips
- Check the [Troubleshooting](troubleshooting.md) guide for common issues
- Visit the [API Reference](api-reference.md) for detailed documentation
