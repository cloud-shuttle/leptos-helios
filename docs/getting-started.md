# Getting Started with Helios

> Your guide to building high-performance visualizations with Rust and Leptos v0.8

## Quick Start

### Prerequisites

- **Rust**: Latest stable (1.79+) with `wasm32-unknown-unknown` target
- **Node.js**: 18+ for development tools
- **Browser**: Modern browser with WebGPU support (Chrome 113+, Safari 17+, Firefox 115+)

```bash
# Install Rust and WebAssembly target
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown

# Install trunk for WASM development
cargo install trunk

# Install additional tools
cargo install wasm-pack wasm-opt
```

### Installation

Add Helios to your Leptos project:

```toml
[dependencies]
helios = "0.1"
leptos = { version = "0.8", features = ["csr", "hydrate"] }
polars = { version = "1.30", features = ["lazy"] }
```

For server-side features, also add:

```toml
[dependencies]
helios = { version = "0.1", features = ["ssr"] }
leptos = { version = "0.8", features = ["ssr", "hydrate"] }
datafusion = "43"
tokio = { version = "1", features = ["full"] }
```

### Your First Chart

Create a simple line chart in just a few lines:

```rust
use helios::prelude::*;
use leptos::*;
use polars::prelude::*;

#[component]
pub fn MyFirstChart() -> impl IntoView {
    // Create sample data
    let data = df! {
        "x" => [1, 2, 3, 4, 5],
        "y" => [2, 5, 3, 8, 7],
    }.unwrap();
    
    // Define chart specification
    let chart_spec = helios::chart! {
        data: data,
        mark: Line,
        encoding: {
            x: { field: "x", type: Quantitative },
            y: { field: "y", type: Quantitative }
        }
    };
    
    view! {
        <div class="chart-container">
            <h2>"My First Helios Chart"</h2>
            <HeliosChart spec=chart_spec width=600 height=400 />
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <MyFirstChart />
        </main>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> })
}
```

### Project Structure

A typical Helios project structure:

```
my-viz-app/
├── Cargo.toml
├── Trunk.toml           # WASM build configuration
├── index.html           # HTML template
├── src/
│   ├── main.rs          # Application entry point
│   ├── app.rs           # Root app component
│   ├── components/      # Reusable components
│   │   ├── mod.rs
│   │   ├── charts/      # Chart components
│   │   └── controls/    # UI controls
│   ├── data/            # Data processing
│   │   ├── mod.rs
│   │   ├── loaders.rs   # Data loading utilities
│   │   └── transforms.rs # Data transformations
│   └── utils/           # Utility functions
├── static/              # Static assets
├── examples/            # Example charts
└── benches/            # Performance benchmarks
```

## Core Concepts

### Reactive Data Flow

Helios integrates deeply with Leptos's fine-grained reactivity:

```rust
#[component]
pub fn ReactiveChart() -> impl IntoView {
    // Create reactive data source
    let (raw_data, set_raw_data) = create_signal(DataFrame::empty());
    let (filter_value, set_filter_value) = create_signal(0.0);
    
    // Processed data updates automatically when dependencies change
    let filtered_data = create_memo(move |_| {
        raw_data.with(|df| {
            df.clone()
                .lazy()
                .filter(col("value").gt(lit(filter_value.get())))
                .collect()
                .unwrap()
        })
    });
    
    // Chart spec recomputes only when filtered_data changes
    let chart_spec = create_memo(move |_| {
        helios::chart! {
            data: filtered_data.get(),
            mark: Point { size: Some(5.0) },
            encoding: {
                x: { field: "x", type: Quantitative },
                y: { field: "value", type: Quantitative },
                color: { 
                    field: "value", 
                    type: Quantitative,
                    scale: { scheme: "viridis" }
                }
            }
        }
    });
    
    view! {
        <div>
            <input 
                type="range" 
                min="0" max="100" step="1"
                on:input=move |ev| {
                    set_filter_value(event_target_value(&ev).parse().unwrap_or(0.0));
                }
            />
            <HeliosChart spec=chart_spec />
        </div>
    }
}
```

### Data Integration

Helios works seamlessly with Polars DataFrames:

```rust
use polars::prelude::*;

// Create data directly
let df = df! {
    "date" => ["2024-01-01", "2024-01-02", "2024-01-03"],
    "value" => [100, 120, 90],
    "category" => ["A", "B", "A"],
}.unwrap();

// Load from file
let df = LazyFrame::scan_csv("data.csv", ScanArgsIo::default())
    .filter(col("value").gt(0))
    .select([col("*")])
    .collect()
    .unwrap();

// Complex transformations
let processed = df
    .lazy()
    .group_by([col("category")])
    .agg([
        col("value").mean().alias("avg_value"),
        col("value").std(1).alias("std_value"),
    ])
    .sort("avg_value", SortMultipleOptions::default())
    .collect()
    .unwrap();
```

### Chart Types

Helios supports a wide range of visualization types:

```rust
// Scatter plot
let scatter = helios::chart! {
    data: df,
    mark: Point { size: Some(8.0) },
    encoding: {
        x: { field: "height", type: Quantitative },
        y: { field: "weight", type: Quantitative },
        color: { field: "species", type: Nominal }
    }
};

// Bar chart
let bars = helios::chart! {
    data: df,
    mark: Bar,
    encoding: {
        x: { field: "category", type: Ordinal },
        y: { field: "value", type: Quantitative }
    }
};

// Area chart with stacking
let area = helios::chart! {
    data: df,
    mark: Area { opacity: Some(0.7) },
    encoding: {
        x: { field: "date", type: Temporal },
        y: { field: "value", type: Quantitative },
        color: { field: "series", type: Nominal }
    }
};

// Multi-layered chart
let combo = helios::chart! {
    data: df,
    layer: [
        {
            mark: Line,
            encoding: {
                x: { field: "date", type: Temporal },
                y: { field: "value", type: Quantitative }
            }
        },
        {
            mark: Point { size: Some(6.0) },
            encoding: {
                x: { field: "date", type: Temporal },
                y: { field: "value", type: Quantitative },
                color: { field: "anomaly", type: Nominal }
            }
        }
    ]
};
```

## Data Loading Patterns

### Static Data

```rust
// Compile-time data inclusion
const DATA_CSV: &str = include_str!("../data/sample.csv");

#[component]
pub fn StaticChart() -> impl IntoView {
    let df = CsvReader::new(std::io::Cursor::new(DATA_CSV))
        .finish()
        .unwrap();
        
    let chart = helios::chart! {
        data: df,
        mark: Line,
        encoding: {
            x: { field: "x", type: Quantitative },
            y: { field: "y", type: Quantitative }
        }
    };
    
    view! { <HeliosChart spec=chart /> }
}
```

### Dynamic Data Loading

```rust
#[component] 
pub fn DynamicChart() -> impl IntoView {
    // Resource automatically manages loading/error states
    let data = create_resource(
        || (), // No dependencies, load once
        |_| async move {
            // This runs in a web worker to avoid blocking
            LazyFrame::scan_csv("large_dataset.csv", ScanArgsIo::default())
                .select([col("*")])
                .collect()
        }
    );
    
    view! {
        <Suspense fallback=move || view! { <div>"Loading chart data..."</div> }>
            {move || {
                data.get().map(|result| match result {
                    Ok(df) => {
                        let chart = helios::chart! {
                            data: df,
                            mark: Point,
                            encoding: {
                                x: { field: "x", type: Quantitative },
                                y: { field: "y", type: Quantitative }
                            }
                        };
                        view! { <HeliosChart spec=chart /> }
                    },
                    Err(e) => view! { <div class="error">{e.to_string()}</div> }
                })
            }}
        </Suspense>
    }
}
```

### Server-Side Data Processing

```rust
// Server function for heavy processing
#[server(LoadLargeDataset, "/api")]
pub async fn load_large_dataset(
    file_path: String,
    filter_query: String,
) -> Result<DataFrame, ServerFnError> {
    use datafusion::prelude::*;
    
    // Use DataFusion for complex queries
    let ctx = SessionContext::new();
    ctx.register_csv("data", &file_path, CsvReadOptions::new()).await?;
    
    let df = ctx.sql(&filter_query).await?
        .collect().await?;
        
    // Convert Arrow to Polars
    Ok(arrow_to_polars(df)?)
}

#[component]
pub fn ServerDataChart() -> impl IntoView {
    let load_data = create_server_action::<LoadLargeDataset>();
    
    let chart_data = create_resource(
        move || load_data.value().get(),
        |data| async move { data.transpose()? }
    );
    
    view! {
        <div>
            <button on:click=move |_| {
                load_data.dispatch(LoadLargeDataset {
                    file_path: "big_data.csv".to_string(),
                    filter_query: "SELECT * FROM data WHERE value > 100".to_string(),
                });
            }>
                "Load Data"
            </button>
            
            <Suspense fallback=|| view! { <div>"Processing..."</div> }>
                {move || chart_data.get().map(|df| {
                    let chart = helios::chart! {
                        data: df,
                        mark: Point,
                        encoding: {
                            x: { field: "x", type: Quantitative },
                            y: { field: "y", type: Quantitative }
                        }
                    };
                    view! { <HeliosChart spec=chart /> }
                })}
            </Suspense>
        </div>
    }
}
```

## Interactive Features

### Pan and Zoom

```rust
let chart = helios::chart! {
    data: df,
    mark: Point,
    encoding: {
        x: { field: "x", type: Quantitative },
        y: { field: "y", type: Quantitative }
    }
};

view! {
    <HeliosChart 
        spec=chart
        interactions=InteractionConfig::new()
            .enable_pan()
            .enable_zoom()
            .zoom_extent([0.5, 10.0]) // Min and max zoom levels
    />
}
```

### Brushing and Selection

```rust
#[component]
pub fn InteractiveScatterPlot() -> impl IntoView {
    let (selected_points, set_selected_points) = create_signal(Vec::<usize>::new());
    
    let chart = helios::chart! {
        data: df,
        mark: Point { 
            size: Some(6.0),
            opacity: Some(0.8) 
        },
        encoding: {
            x: { field: "x", type: Quantitative },
            y: { field: "y", type: Quantitative },
            color: { 
                condition: {
                    selection: "brush",
                    value: "red"
                },
                value: "blue"
            }
        },
        selection: [{
            name: "brush",
            type: Interval,
            bind: "brush"
        }]
    };
    
    view! {
        <div>
            <HeliosChart 
                spec=chart 
                on:selection_changed=move |selection| {
                    set_selected_points(selection.point_indices);
                }
            />
            <div>"Selected points: " {move || selected_points.get().len()}</div>
        </div>
    }
}
```

### Tooltips

```rust
let chart = helios::chart! {
    data: df,
    mark: Point { 
        size: Some(8.0),
        tooltip: true 
    },
    encoding: {
        x: { field: "x", type: Quantitative },
        y: { field: "y", type: Quantitative },
        color: { field: "category", type: Nominal },
        tooltip: [
            { field: "x", type: Quantitative },
            { field: "y", type: Quantitative },
            { field: "category", type: Nominal },
            { field: "description", type: Nominal }
        ]
    }
};
```

## Performance Optimization

### Large Datasets

```rust
// For datasets > 100K points
let chart = helios::chart! {
    data: large_df,
    mark: Point { size: Some(2.0) },
    encoding: {
        x: { field: "x", type: Quantitative },
        y: { field: "y", type: Quantitative }
    }
};

view! {
    <HeliosChart 
        spec=chart
        performance=PerformanceConfig::new()
            .quality_mode(QualityMode::Adaptive {
                target_frame_time: Duration::from_millis(16), // 60fps
                quality_range: (0.5, 1.0)
            })
            .memory_limit(Some(100 * 1024 * 1024)) // 100MB
    />
}
```

### Streaming Data

```rust
#[component]
pub fn StreamingChart() -> impl IntoView {
    let (stream_data, set_stream_data) = create_signal(DataFrame::empty());
    
    // Simulate streaming data
    create_effect(move |_| {
        set_interval(
            move || {
                // Add new data points
                let new_points = generate_new_data_points(10);
                set_stream_data.update(|df| {
                    *df = concat_dataframes(df.clone(), new_points);
                    // Keep only recent data to manage memory
                    if df.height() > 10000 {
                        *df = df.slice(df.height() - 10000, 10000);
                    }
                });
            },
            Duration::from_millis(100) // 10fps updates
        );
    });
    
    let chart = create_memo(move |_| {
        helios::chart! {
            data: stream_data.get(),
            mark: Line,
            encoding: {
                x: { field: "timestamp", type: Temporal },
                y: { field: "value", type: Quantitative }
            }
        }
    });
    
    view! {
        <HeliosChart 
            spec=chart 
            performance=PerformanceConfig::new()
                .target_fps(Some(30)) // Lower FPS for streaming
                .quality_mode(QualityMode::Performance)
        />
    }
}
```

## Styling and Themes

### Custom Styling

```rust
let chart = helios::chart! {
    data: df,
    mark: Bar { corner_radius: Some(4.0) },
    encoding: {
        x: { field: "category", type: Ordinal },
        y: { field: "value", type: Quantitative }
    },
    config: {
        background: "transparent",
        padding: { top: 20, right: 40, bottom: 40, left: 60 },
        axis: {
            domain_color: "#666",
            tick_color: "#999",
            label_font_size: 12,
            title_font_size: 14
        },
        scale: {
            band_padding_inner: 0.1,
            band_padding_outer: 0.05
        }
    }
};
```

### CSS Integration

```css
/* styles.css */
.helios-chart {
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.helios-tooltip {
    background: rgba(0, 0, 0, 0.8);
    color: white;
    padding: 8px 12px;
    border-radius: 4px;
    font-size: 14px;
}

.chart-container {
    padding: 20px;
    background: #f9f9f9;
}
```

## Development Tips

### Debugging

```rust
// Enable debug mode for development
view! {
    <HeliosChart 
        spec=chart_spec 
        debug=true  // Shows render stats and debug info
    />
}

// Performance monitoring
let (render_stats, set_render_stats) = create_signal(None);

view! {
    <HeliosChart 
        spec=chart_spec
        on:render_complete=move |stats| set_render_stats(Some(stats))
    />
    {move || render_stats.get().map(|stats| view! {
        <div class="debug-info">
            <div>"Frame time: " {stats.frame_time.as_millis()} "ms"</div>
            <div>"Memory: " {stats.memory_used / 1024 / 1024} "MB"</div>
        </div>
    })}
}
```

### Hot Reloading

```bash
# Start development server with hot reloading
trunk serve --open --port 3000

# The chart will automatically update when you change:
# - Data transformations
# - Chart specifications  
# - Styling
# - Component logic
```

### Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    async fn test_chart_rendering() {
        let df = df! {
            "x" => [1, 2, 3],
            "y" => [1, 4, 2],
        }.unwrap();
        
        let chart_spec = helios::chart! {
            data: df,
            mark: Line,
            encoding: {
                x: { field: "x", type: Quantitative },
                y: { field: "y", type: Quantitative }
            }
        };
        
        // Test chart specification validation
        assert!(chart_spec.validate().is_ok());
        
        // Test rendering performance
        let stats = render_chart_spec(&chart_spec).await;
        assert!(stats.frame_time < Duration::from_millis(10));
    }
}
```

## Next Steps

- **Explore Examples**: Check out `/examples` for more complex visualizations
- **Read the API Docs**: Complete reference at `/docs/api.md`
- **Performance Guide**: Optimization tips at `/docs/performance.md`
- **Advanced Features**: AI/ML integration, custom marks, and more
- **Community**: Join discussions on GitHub and Discord

## Common Patterns

### Dashboard Layout

```rust
#[component]
pub fn Dashboard() -> impl IntoView {
    let (data, set_data) = create_signal(load_dashboard_data());
    
    view! {
        <div class="dashboard-grid">
            <div class="chart-panel">
                <HeliosChart spec=create_revenue_chart(data.get()) />
            </div>
            <div class="chart-panel">
                <HeliosChart spec=create_user_chart(data.get()) />
            </div>
            <div class="chart-panel">
                <HeliosChart spec=create_performance_chart(data.get()) />
            </div>
            <div class="controls-panel">
                <DataControls on_data_change=set_data />
            </div>
        </div>
    }
}
```

### Real-time Monitoring

```rust
#[component]
pub fn MonitoringDashboard() -> impl IntoView {
    let ws_context = expect_context::<WebSocketContext>();
    let (metrics, set_metrics) = create_signal(Vec::<MetricPoint>::new());
    
    // Listen to WebSocket updates
    create_effect(move |_| {
        let handler = move |data: MetricPoint| {
            set_metrics.update(|m| {
                m.push(data);
                if m.len() > 1000 { m.remove(0); } // Rolling window
            });
        };
        ws_context.on_message(handler);
    });
    
    let chart_spec = create_memo(move |_| {
        helios::chart! {
            data: metrics_to_dataframe(metrics.get()),
            mark: Line,
            encoding: {
                x: { field: "timestamp", type: Temporal },
                y: { field: "value", type: Quantitative },
                color: { field: "metric_type", type: Nominal }
            }
        }
    });
    
    view! {
        <HeliosChart 
            spec=chart_spec 
            performance=PerformanceConfig::new()
                .target_fps(Some(10)) // Smooth but not excessive
                .quality_mode(QualityMode::Performance)
        />
    }
}
```

Ready to create amazing visualizations? Start with these examples and build up to more complex use cases. The Helios ecosystem provides everything you need for world-class data visualization in Rust!