//! Advanced Chart Examples
//!
//! Examples demonstrating advanced chart types in Helios

use leptos::*;
use leptos_helios::*;

/// Radar Chart Example
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
        RadarDataPoint {
            category: "Performance".to_string(),
            value: 85.0,
            max_value: 100.0,
        },
        RadarDataPoint {
            category: "Usability".to_string(),
            value: 92.0,
            max_value: 100.0,
        },
        RadarDataPoint {
            category: "Features".to_string(),
            value: 78.0,
            max_value: 100.0,
        },
        RadarDataPoint {
            category: "Support".to_string(),
            value: 88.0,
            max_value: 100.0,
        },
        RadarDataPoint {
            category: "Price".to_string(),
            value: 95.0,
            max_value: 100.0,
        },
    ];

    view! {
        <HeliosChart config=config data=data />
    }
}

/// Sankey Diagram Example
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
            SankeyNode {
                id: "source1".to_string(),
                name: "Source 1".to_string(),
                value: 100.0,
            },
            SankeyNode {
                id: "source2".to_string(),
                name: "Source 2".to_string(),
                value: 80.0,
            },
            SankeyNode {
                id: "target1".to_string(),
                name: "Target 1".to_string(),
                value: 0.0,
            },
            SankeyNode {
                id: "target2".to_string(),
                name: "Target 2".to_string(),
                value: 0.0,
            },
        ],
        links: vec![
            SankeyLink {
                source: "source1".to_string(),
                target: "target1".to_string(),
                value: 60.0,
            },
            SankeyLink {
                source: "source1".to_string(),
                target: "target2".to_string(),
                value: 40.0,
            },
            SankeyLink {
                source: "source2".to_string(),
                target: "target1".to_string(),
                value: 30.0,
            },
            SankeyLink {
                source: "source2".to_string(),
                target: "target2".to_string(),
                value: 50.0,
            },
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

/// Treemap Example
#[component]
pub fn TreemapExample() -> impl IntoView {
    let config = TreemapConfig {
        base_config: BaseChartConfig {
            width: 800,
            height: 600,
            title: "Hierarchical Data".to_string(),
            x_label: "".to_string(),
            y_label: "".to_string(),
            show_grid: false,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        root_node: TreemapNode {
            id: "root".to_string(),
            name: "Root".to_string(),
            value: 100.0,
            children: vec![
                TreemapNode {
                    id: "category1".to_string(),
                    name: "Category 1".to_string(),
                    value: 40.0,
                    children: vec![
                        TreemapNode {
                            id: "item1".to_string(),
                            name: "Item 1".to_string(),
                            value: 25.0,
                            children: vec![],
                        },
                        TreemapNode {
                            id: "item2".to_string(),
                            name: "Item 2".to_string(),
                            value: 15.0,
                            children: vec![],
                        },
                    ],
                },
                TreemapNode {
                    id: "category2".to_string(),
                    name: "Category 2".to_string(),
                    value: 35.0,
                    children: vec![
                        TreemapNode {
                            id: "item3".to_string(),
                            name: "Item 3".to_string(),
                            value: 20.0,
                            children: vec![],
                        },
                        TreemapNode {
                            id: "item4".to_string(),
                            name: "Item 4".to_string(),
                            value: 15.0,
                            children: vec![],
                        },
                    ],
                },
                TreemapNode {
                    id: "category3".to_string(),
                    name: "Category 3".to_string(),
                    value: 25.0,
                    children: vec![],
                },
            ],
        },
        padding: 2.0,
        border_width: 1.0,
        border_color: "#ffffff".to_string(),
        color_scheme: ColorScheme::Viridis,
        show_labels: true,
        label_threshold: 0.05,
    };

    view! {
        <HeliosChart config=config />
    }
}

/// Violin Plot Example
#[component]
pub fn ViolinPlotExample() -> impl IntoView {
    let config = ViolinConfig {
        base_config: BaseChartConfig {
            width: 800,
            height: 600,
            title: "Distribution Analysis".to_string(),
            x_label: "Category".to_string(),
            y_label: "Value".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        data: vec![
            ViolinDataPoint {
                category: "Group A".to_string(),
                values: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
            },
            ViolinDataPoint {
                category: "Group B".to_string(),
                values: vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0],
            },
            ViolinDataPoint {
                category: "Group C".to_string(),
                values: vec![3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0],
            },
        ],
        bandwidth: 0.5,
        show_box_plot: true,
        show_points: false,
        point_size: 3.0,
        fill_opacity: 0.7,
        stroke_width: 1.0,
        color_scheme: ColorScheme::Viridis,
    };

    view! {
        <HeliosChart config=config />
    }
}
