//! Basic Chart Examples
//! 
//! Examples demonstrating basic chart types in Helios

use leptos::*;
use leptos_helios::*;

/// Line Chart Example
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

/// Bar Chart Example
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

/// Scatter Plot Example
#[component]
pub fn ScatterPlotExample() -> impl IntoView {
    let config = ScatterPlotConfig {
        base_config: BaseChartConfig {
            width: 800,
            height: 600,
            title: "Correlation Analysis".to_string(),
            x_label: "X Value".to_string(),
            y_label: "Y Value".to_string(),
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
        point_shape: PointShape::Circle,
        color: "#8b5cf6".to_string(),
        show_legend: true,
    };

    view! {
        <HeliosChart config=config />
    }
}

/// Heatmap Example
#[component]
pub fn HeatmapExample() -> impl IntoView {
    let config = HeatmapConfig {
        base_config: BaseChartConfig {
            width: 800,
            height: 600,
            title: "Data Heatmap".to_string(),
            x_label: "X Axis".to_string(),
            y_label: "Y Axis".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        data: vec![
            HeatmapData { x: 0, y: 0, value: 1.0 },
            HeatmapData { x: 1, y: 0, value: 2.0 },
            HeatmapData { x: 0, y: 1, value: 3.0 },
            HeatmapData { x: 1, y: 1, value: 4.0 },
        ],
        color_scheme: ColorScheme::Viridis,
        show_legend: true,
    };

    view! {
        <HeliosChart config=config />
    }
}
