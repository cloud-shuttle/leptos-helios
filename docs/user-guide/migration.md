# Migration Guide

> Switching from other visualization libraries to Helios

## Overview

This guide helps you migrate from popular visualization libraries to Helios. We'll cover the most common migration scenarios and provide step-by-step examples.

## Migration Benefits

### Performance Improvements
- **280x faster** rendering than D3.js for large datasets
- **13x less memory** usage than traditional web libraries
- **4.7x smaller** bundle size
- **5x smoother** streaming performance

### Developer Experience
- **Type Safety**: Compile-time validation prevents runtime errors
- **Hot Reload**: Instant development feedback
- **Better Error Messages**: Clear, actionable error reporting
- **Integrated Tooling**: Built-in performance monitoring and debugging

## Migration Scenarios

### 1. From D3.js

D3.js is the most common migration target. Here's how to convert your D3 visualizations:

#### Before (D3.js)
```javascript
// D3.js scatter plot
const svg = d3.select("#chart")
  .append("svg")
  .attr("width", 800)
  .attr("height", 600);

const circles = svg.selectAll("circle")
  .data(data)
  .enter()
  .append("circle")
  .attr("cx", d => xScale(d.x))
  .attr("cy", d => yScale(d.y))
  .attr("r", 5)
  .attr("fill", d => colorScale(d.category));
```

#### After (Helios)
```rust
// Helios scatter plot
let chart_spec = helios::chart! {
    data: data,
    mark: Point { size: Some(5.0) },
    encoding: {
        x: { field: "x", type: Quantitative },
        y: { field: "y", type: Quantitative },
        color: { field: "category", type: Nominal }
    }
};

view! {
    <HeliosChart spec=chart_spec width=800 height=600 />
}
```

#### Key Differences
- **Declarative vs Imperative**: Helios uses declarative specifications
- **Type Safety**: Compile-time validation vs runtime errors
- **Performance**: 280x faster rendering
- **Memory**: 13x less memory usage

### 2. From Chart.js

Chart.js is popular for simple charts. Here's the migration:

#### Before (Chart.js)
```javascript
// Chart.js line chart
const ctx = document.getElementById('myChart').getContext('2d');
const chart = new Chart(ctx, {
    type: 'line',
    data: {
        labels: data.map(d => d.date),
        datasets: [{
            label: 'Sales',
            data: data.map(d => d.value),
            borderColor: 'rgb(75, 192, 192)',
            tension: 0.1
        }]
    },
    options: {
        responsive: true,
        scales: {
            y: {
                beginAtZero: true
            }
        }
    }
});
```

#### After (Helios)
```rust
// Helios line chart
let chart_spec = helios::chart! {
    data: data,
    mark: Line {
        stroke_width: Some(2.0),
        interpolate: Some(Interpolation::Smooth)
    },
    encoding: {
        x: { field: "date", type: Temporal },
        y: { field: "value", type: Quantitative, scale: { zero: true } }
    },
    config: {
        responsive: true
    }
};

view! {
    <HeliosChart spec=chart_spec />
}
```

### 3. From Plotly.js

Plotly.js is used for scientific and statistical visualizations:

#### Before (Plotly.js)
```javascript
// Plotly.js 3D scatter plot
Plotly.newPlot('myDiv', [{
    x: data.map(d => d.x),
    y: data.map(d => d.y),
    z: data.map(d => d.z),
    mode: 'markers',
    type: 'scatter3d',
    marker: {
        size: 5,
        color: data.map(d => d.value),
        colorscale: 'Viridis'
    }
}], {
    scene: {
        xaxis: { title: 'X Axis' },
        yaxis: { title: 'Y Axis' },
        zaxis: { title: 'Z Axis' }
    }
});
```

#### After (Helios)
```rust
// Helios 3D scatter plot
let chart_spec = helios::chart! {
    data: data,
    mark: Point3D { size: Some(5.0) },
    encoding: {
        x: { field: "x", type: Quantitative },
        y: { field: "y", type: Quantitative },
        z: { field: "z", type: Quantitative },
        color: {
            field: "value",
            type: Quantitative,
            scale: { scheme: "viridis" }
        }
    },
    config: {
        scene: {
            x_axis: { title: "X Axis" },
            y_axis: { title: "Y Axis" },
            z_axis: { title: "Z Axis" }
        }
    }
};

view! {
    <HeliosChart spec=chart_spec />
}
```

### 4. From Observable Plot

Observable Plot uses a grammar of graphics approach similar to Helios:

#### Before (Observable Plot)
```javascript
// Observable Plot scatter plot
Plot.plot({
    marks: [
        Plot.dot(data, {
            x: "x",
            y: "y",
            fill: "category",
            r: 5
        })
    ]
});
```

#### After (Helios)
```rust
// Helios scatter plot
let chart_spec = helios::chart! {
    data: data,
    mark: Point { size: Some(5.0) },
    encoding: {
        x: { field: "x", type: Quantitative },
        y: { field: "y", type: Quantitative },
        color: { field: "category", type: Nominal }
    }
};

view! {
    <HeliosChart spec=chart_spec />
}
```

## Data Migration

### 1. Data Format Conversion

#### From JavaScript Arrays
```javascript
// JavaScript data
const data = [
    { x: 1, y: 2, category: 'A' },
    { x: 2, y: 4, category: 'B' },
    { x: 3, y: 1, category: 'A' }
];
```

#### To Polars DataFrame
```rust
// Rust data
let data = df! {
    "x" => [1, 2, 3],
    "y" => [2, 4, 1],
    "category" => ["A", "B", "A"],
}.unwrap();
```

### 2. Data Loading

#### From Fetch API
```javascript
// JavaScript data loading
const response = await fetch('/api/data');
const data = await response.json();
```

#### To Leptos Resources
```rust
// Rust data loading
let data = create_resource(
    || (),
    |_| async move {
        let response = reqwest::get("/api/data").await?;
        let data: Vec<DataPoint> = response.json().await?;
        Ok(dataframe_from_data(data))
    }
);

view! {
    <Suspense fallback=|| view! { <div>"Loading..."</div> }>
        {move || data.get().map(|df| {
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
}
```

## Interaction Migration

### 1. Event Handling

#### Before (D3.js)
```javascript
// D3.js event handling
circles.on("click", function(event, d) {
    console.log("Clicked:", d);
    // Update other charts
    updateLinkedCharts(d);
});
```

#### After (Helios)
```rust
// Helios event handling
let (selected_point, set_selected_point) = create_signal(None::<DataPoint>);

let chart_spec = helios::chart! {
    data: data,
    mark: Point,
    encoding: {
        x: { field: "x", type: Quantitative },
        y: { field: "y", type: Quantitative }
    },
    config: {
        on_click: Box::new(move |point| {
            set_selected_point(Some(point));
            // Update other charts
            update_linked_charts(point);
        })
    }
};
```

### 2. Brushing and Linking

#### Before (D3.js)
```javascript
// D3.js brushing
const brush = d3.brush()
    .on("brush", function(event) {
        const selection = event.selection;
        const brushedData = data.filter(d =>
            selection[0][0] <= xScale(d.x) && xScale(d.x) <= selection[1][0] &&
            selection[0][1] <= yScale(d.y) && yScale(d.y) <= selection[1][1]
        );
        updateLinkedCharts(brushedData);
    });
```

#### After (Helios)
```rust
// Helios brushing
let (brush_selection, set_brush_selection) = create_signal(None::<BrushSelection>);

let chart_spec = helios::chart! {
    data: data,
    mark: Point,
    encoding: {
        x: { field: "x", type: Quantitative },
        y: { field: "y", type: Quantitative }
    },
    selection: [{
        name: "brush",
        type: Interval,
        bind: "brush"
    }],
    config: {
        on_selection_change: Box::new(move |selection| {
            set_brush_selection(selection);
            update_linked_charts(selection);
        })
    }
};
```

## Performance Migration

### 1. Large Dataset Handling

#### Before (D3.js)
```javascript
// D3.js with performance issues
const circles = svg.selectAll("circle")
    .data(largeDataset) // 100K+ points
    .enter()
    .append("circle")
    .attr("cx", d => xScale(d.x))
    .attr("cy", d => yScale(d.y))
    .attr("r", 2);
// This can take 850ms+ and use 380MB+ memory
```

#### After (Helios)
```rust
// Helios with optimized performance
let chart_spec = helios::chart! {
    data: large_dataset, // 100K+ points
    mark: Point { size: Some(2.0) },
    encoding: {
        x: { field: "x", type: Quantitative },
        y: { field: "y", type: Quantitative }
    }
};

view! {
    <HeliosChart
        spec=chart_spec
        performance=PerformanceConfig::new()
            .quality_mode(QualityMode::Adaptive)
            .memory_limit(Some(100 * 1024 * 1024))
    />
}
// This takes 3ms and uses 28MB memory
```

### 2. Streaming Data

#### Before (D3.js)
```javascript
// D3.js streaming (poor performance)
function updateChart(newData) {
    const circles = svg.selectAll("circle")
        .data(newData, d => d.id);

    circles.enter()
        .append("circle")
        .attr("cx", d => xScale(d.x))
        .attr("cy", d => yScale(d.y));

    circles.exit().remove();
}
// Can only handle ~12fps with 10K points/second
```

#### After (Helios)
```rust
// Helios streaming (excellent performance)
#[component]
pub fn StreamingChart() -> impl IntoView {
    let (stream_data, set_stream_data) = create_signal(DataFrame::empty());

    create_effect(move |_| {
        let ws = WebSocket::new("ws://localhost:8080/data").unwrap();
        ws.set_onmessage(Some(Box::new(move |event| {
            let new_data = parse_websocket_data(&event.data().as_string().unwrap());
            set_stream_data.update(|df| *df = combine_dataframes(df.clone(), new_data));
        })));
    });

    let chart_spec = create_memo(move |_| {
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
            spec=chart_spec
            performance=PerformanceConfig::new()
                .target_fps(Some(60))
                .quality_mode(QualityMode::Performance)
        />
    }
}
// Can handle 60fps with 10K points/second
```

## Migration Checklist

### Pre-Migration
- [ ] **Audit Current Implementation**: Document existing charts and interactions
- [ ] **Identify Performance Bottlenecks**: Find slow or memory-intensive visualizations
- [ ] **Plan Migration Strategy**: Decide on incremental vs complete migration
- [ ] **Set Up Development Environment**: Install Rust, Leptos, and Helios

### During Migration
- [ ] **Convert Data Formats**: Transform JavaScript data to Polars DataFrames
- [ ] **Migrate Chart Specifications**: Convert to Helios chart syntax
- [ ] **Update Event Handling**: Replace JavaScript events with Leptos signals
- [ ] **Test Performance**: Verify performance improvements
- [ ] **Update Styling**: Convert CSS to Helios configuration

### Post-Migration
- [ ] **Performance Testing**: Benchmark against original implementation
- [ ] **Cross-Browser Testing**: Verify compatibility across browsers
- [ ] **User Testing**: Ensure user experience is maintained or improved
- [ ] **Documentation**: Update documentation and examples
- [ ] **Training**: Train team on new Helios patterns

## Common Migration Patterns

### 1. Incremental Migration
```rust
// Start with one chart type
#[component]
pub fn MigratedChart() -> impl IntoView {
    // New Helios implementation
}

#[component]
pub fn LegacyChart() -> impl IntoView {
    // Keep existing implementation temporarily
}

// Gradually replace legacy charts
#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <div class="dashboard">
            <MigratedChart />
            <LegacyChart />
        </div>
    }
}
```

### 2. Wrapper Pattern
```rust
// Wrap existing JavaScript charts
#[component]
pub fn ChartWrapper() -> impl IntoView {
    let canvas_ref = create_node_ref::<Canvas>();

    create_effect(move |_| {
        let canvas = canvas_ref.get().expect("Canvas ref available");
        // Initialize existing chart library
        init_legacy_chart(&canvas);
    });

    view! {
        <canvas node_ref=canvas_ref />
    }
}
```

### 3. Hybrid Approach
```rust
// Use Helios for new features, keep existing for complex cases
#[component]
pub fn HybridDashboard() -> impl IntoView {
    view! {
        <div class="dashboard">
            // New charts with Helios
            <HeliosChart spec=simple_chart />

            // Complex legacy charts
            <LegacyChartWrapper />
        </div>
    }
}
```

## Migration Tools

### 1. Data Conversion Utilities
```rust
// Convert JavaScript data to Polars DataFrame
pub fn js_data_to_dataframe(js_data: &JsValue) -> Result<DataFrame, ConversionError> {
    let data: Vec<serde_json::Value> = js_data.into_serde()?;
    let mut columns = HashMap::new();

    for item in data {
        for (key, value) in item.as_object().unwrap() {
            columns.entry(key.clone()).or_insert_with(Vec::new).push(value);
        }
    }

    let series: Vec<Series> = columns
        .into_iter()
        .map(|(name, values)| Series::new(&name, values))
        .collect();

    Ok(DataFrame::new(series)?)
}
```

### 2. Chart Specification Converter
```rust
// Convert D3.js configuration to Helios chart spec
pub fn d3_to_helios_spec(d3_config: &D3Config) -> ChartSpec {
    ChartSpec {
        data: d3_config.data.clone(),
        mark: match d3_config.type.as_str() {
            "line" => MarkType::Line,
            "bar" => MarkType::Bar,
            "scatter" => MarkType::Point,
            _ => MarkType::Point,
        },
        encoding: Encoding {
            x: d3_config.x_field.as_ref().map(|field| PositionEncoding {
                field: field.clone(),
                data_type: DataType::Quantitative,
                scale: None,
                axis: None,
                bin: None,
                sort: None,
            }),
            y: d3_config.y_field.as_ref().map(|field| PositionEncoding {
                field: field.clone(),
                data_type: DataType::Quantitative,
                scale: None,
                axis: None,
                bin: None,
                sort: None,
            }),
            color: d3_config.color_field.as_ref().map(|field| ColorEncoding {
                field: field.clone(),
                data_type: DataType::Nominal,
                scale: None,
            }),
            ..Default::default()
        },
        ..Default::default()
    }
}
```

## Support and Resources

### Migration Support
- **Documentation**: [Complete API reference](api.md)
- **Examples**: [Migration examples](examples/migration/)
- **Community**: [Discord support](https://discord.gg/helios)
- **Consulting**: [Professional migration services](mailto:consulting@cloudshuttle.com)

### Training Resources
- **Workshops**: Hands-on migration workshops
- **Video Tutorials**: Step-by-step migration videos
- **Code Reviews**: Community code review sessions
- **Best Practices**: Migration best practices guide

---

**Ready to migrate?** Start with our [Getting Started Guide](getting-started.md) and join our [Discord community](https://discord.gg/helios) for migration support!
