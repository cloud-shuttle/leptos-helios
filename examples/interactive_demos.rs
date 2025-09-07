//! Interactive Demo Examples
//! 
//! Examples demonstrating interactive features in Helios

use leptos::*;
use leptos_helios::*;

/// Zoom and Pan Example
#[component]
pub fn ZoomPanExample() -> impl IntoView {
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

/// Hover Tooltips Example
#[component]
pub fn HoverTooltipsExample() -> impl IntoView {
    let (hover_info, set_hover_info) = create_signal(None);

    let config = LineChartConfig {
        base_config: BaseChartConfig {
            width: 800,
            height: 600,
            title: "Hover Tooltips Demo".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        data: vec![
            DataPoint { x: 1.0, y: 2.0 },
            DataPoint { x: 2.0, y: 4.0 },
            DataPoint { x: 3.0, y: 6.0 },
            DataPoint { x: 4.0, y: 8.0 },
            DataPoint { x: 5.0, y: 10.0 },
        ],
        color: "#3b82f6".to_string(),
        show_legend: true,
    };

    let handle_hover = move |info: HoverInfo| {
        set_hover_info.set(Some(info));
    };

    view! {
        <div>
            <HeliosChart config=config on:hover=handle_hover />
            {move || {
                if let Some(info) = hover_info() {
                    view! {
                        <div class="tooltip">
                            <p>"X: " {info.x}</p>
                            <p>"Y: " {info.y}</p>
                            <p>"Value: " {info.value}</p>
                        </div>
                    }
                } else {
                    view! { <div></div> }
                }
            }}
        </div>
    }
}

/// Brush Selection Example
#[component]
pub fn BrushSelectionExample() -> impl IntoView {
    let (selected_range, set_selected_range) = create_signal(None);
    let (data, _) = create_signal(generate_interactive_data());

    let config = ScatterPlotConfig {
        base_config: BaseChartConfig {
            width: 800,
            height: 600,
            title: "Brush Selection Demo".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        data: data(),
        point_shape: PointShape::Circle,
        color: "#10b981".to_string(),
        show_legend: true,
    };

    let handle_brush = move |range: Rect| {
        set_selected_range.set(Some(range));
    };

    view! {
        <div>
            <HeliosChart config=config on:brush=handle_brush />
            {move || {
                if let Some(range) = selected_range() {
                    view! {
                        <div>
                            <p>"Selected Range:"</p>
                            <p>"X: " {range[0]} " to " {range[0] + range[2]}</p>
                            <p>"Y: " {range[1]} " to " {range[1] + range[3]}</p>
                        </div>
                    }
                } else {
                    view! { <div></div> }
                }
            }}
        </div>
    }
}

/// Crossfilter Example
#[component]
pub fn CrossfilterExample() -> impl IntoView {
    let (filtered_data, set_filtered_data) = create_signal(Vec::<DataPoint>::new());
    let (filters, set_filters) = create_signal(HashMap::<String, f64>::new());

    let config = ScatterPlotConfig {
        base_config: BaseChartConfig {
            width: 800,
            height: 600,
            title: "Crossfilter Demo".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        data: filtered_data(),
        point_shape: PointShape::Circle,
        color: "#ef4444".to_string(),
        show_legend: true,
    };

    let apply_filter = move |filter_name: String, value: f64| {
        set_filters.update(|filters| {
            filters.insert(filter_name, value);
        });
        
        // Apply filters to data
        let original_data = generate_interactive_data();
        let filtered: Vec<DataPoint> = original_data
            .into_iter()
            .filter(|point| {
                filters().iter().all(|(_, &filter_value)| {
                    // Simple filtering logic
                    point.x <= filter_value && point.y <= filter_value
                })
            })
            .collect();
        
        set_filtered_data.set(filtered);
    };

    view! {
        <div>
            <HeliosChart config=config />
            <div>
                <label>
                    "X Filter: "
                    <input
                        type="range"
                        min="0"
                        max="100"
                        step="1"
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<f64>().unwrap();
                            apply_filter("x".to_string(), value);
                        }
                    />
                </label>
                <label>
                    "Y Filter: "
                    <input
                        type="range"
                        min="0"
                        max="100"
                        step="1"
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<f64>().unwrap();
                            apply_filter("y".to_string(), value);
                        }
                    />
                </label>
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
