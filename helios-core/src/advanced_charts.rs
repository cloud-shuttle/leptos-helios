//! Advanced Chart Types
//!
//! Implementation of advanced chart types including:
//! - Radar Charts (multi-dimensional polar coordinates)
//! - Sankey Diagrams (flow visualization)
//! - Treemaps (hierarchical rectangles)
//! - Violin Plots (distribution visualization)

use crate::chart_config::*;
use crate::webgpu_renderer::{WebGpuError, WebGpuRenderer};
use std::collections::HashMap;

/// Radar Chart Data Point
#[derive(Debug, Clone)]
pub struct RadarDataPoint {
    pub category: String,
    pub value: f64,
    pub max_value: f64,
}

/// Sankey Diagram Node
#[derive(Debug, Clone)]
pub struct SankeyNode {
    pub id: String,
    pub name: String,
    pub value: f64,
}

/// Sankey Diagram Link
#[derive(Debug, Clone)]
pub struct SankeyLink {
    pub source: String,
    pub target: String,
    pub value: f64,
}

/// Treemap Node
#[derive(Debug, Clone)]
pub struct TreemapNode {
    pub id: String,
    pub name: String,
    pub value: f64,
    pub children: Vec<TreemapNode>,
}

/// Violin Plot Data Point
#[derive(Debug, Clone)]
pub struct ViolinDataPoint {
    pub category: String,
    pub values: Vec<f64>,
}

/// Radar Chart Configuration
#[derive(Debug, Clone)]
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

impl Default for RadarChartConfig {
    fn default() -> Self {
        Self {
            base_config: BaseChartConfig {
                width: 800,
                height: 600,
                title: "Radar Chart".to_string(),
                x_label: "".to_string(),
                y_label: "".to_string(),
                show_grid: true,
                background_color: "#ffffff".to_string(),
                text_color: "#000000".to_string(),
            },
            categories: vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "D".to_string(),
                "E".to_string(),
            ],
            max_value: 100.0,
            show_grid: true,
            show_labels: true,
            fill_area: true,
            stroke_width: 2.0,
            color: "#3b82f6".to_string(),
        }
    }
}

/// Sankey Diagram Configuration
#[derive(Debug, Clone)]
pub struct SankeyConfig {
    pub base_config: BaseChartConfig,
    pub nodes: Vec<SankeyNode>,
    pub links: Vec<SankeyLink>,
    pub node_width: f32,
    pub node_padding: f32,
    pub link_opacity: f32,
    pub color_scheme: ColorScheme,
}

impl Default for SankeyConfig {
    fn default() -> Self {
        Self {
            base_config: BaseChartConfig {
                width: 800,
                height: 600,
                title: "Sankey Diagram".to_string(),
                x_label: "".to_string(),
                y_label: "".to_string(),
                show_grid: false,
                background_color: "#ffffff".to_string(),
                text_color: "#000000".to_string(),
            },
            nodes: vec![],
            links: vec![],
            node_width: 20.0,
            node_padding: 10.0,
            link_opacity: 0.6,
            color_scheme: ColorScheme::Viridis,
        }
    }
}

/// Treemap Configuration
#[derive(Debug, Clone)]
pub struct TreemapConfig {
    pub base_config: BaseChartConfig,
    pub root_node: TreemapNode,
    pub padding: f32,
    pub border_width: f32,
    pub border_color: String,
    pub color_scheme: ColorScheme,
    pub show_labels: bool,
    pub label_threshold: f64,
}

impl Default for TreemapConfig {
    fn default() -> Self {
        Self {
            base_config: BaseChartConfig {
                width: 800,
                height: 600,
                title: "Treemap".to_string(),
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
                children: vec![],
            },
            padding: 2.0,
            border_width: 1.0,
            border_color: "#ffffff".to_string(),
            color_scheme: ColorScheme::Viridis,
            show_labels: true,
            label_threshold: 0.05,
        }
    }
}

/// Violin Plot Configuration
#[derive(Debug, Clone)]
pub struct ViolinConfig {
    pub base_config: BaseChartConfig,
    pub data: Vec<ViolinDataPoint>,
    pub bandwidth: f64,
    pub show_box_plot: bool,
    pub show_points: bool,
    pub point_size: f32,
    pub fill_opacity: f32,
    pub stroke_width: f32,
    pub color_scheme: ColorScheme,
}

impl Default for ViolinConfig {
    fn default() -> Self {
        Self {
            base_config: BaseChartConfig {
                width: 800,
                height: 600,
                title: "Violin Plot".to_string(),
                x_label: "Category".to_string(),
                y_label: "Value".to_string(),
                show_grid: true,
                background_color: "#ffffff".to_string(),
                text_color: "#000000".to_string(),
            },
            data: vec![],
            bandwidth: 0.5,
            show_box_plot: true,
            show_points: false,
            point_size: 3.0,
            fill_opacity: 0.7,
            stroke_width: 1.0,
            color_scheme: ColorScheme::Viridis,
        }
    }
}

/// Advanced Chart Renderer Trait
pub trait AdvancedChartRenderer {
    fn render_radar_chart(
        &mut self,
        config: &RadarChartConfig,
        data: &[RadarDataPoint],
    ) -> Result<WebGpuRenderResult, WebGpuError>;
    fn render_sankey_diagram(
        &mut self,
        config: &SankeyConfig,
    ) -> Result<WebGpuRenderResult, WebGpuError>;
    fn render_treemap(&mut self, config: &TreemapConfig)
        -> Result<WebGpuRenderResult, WebGpuError>;
    fn render_violin_plot(
        &mut self,
        config: &ViolinConfig,
    ) -> Result<WebGpuRenderResult, WebGpuError>;
}

impl AdvancedChartRenderer for WebGpuRenderer {
    fn render_radar_chart(
        &mut self,
        config: &RadarChartConfig,
        data: &[RadarDataPoint],
    ) -> Result<WebGpuRenderResult, WebGpuError> {
        // Convert polar coordinates to cartesian
        let center_x = config.base_config.width as f32 / 2.0;
        let center_y = config.base_config.height as f32 / 2.0;
        let radius = (config.base_config.width.min(config.base_config.height) as f32 / 2.0) * 0.8;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Generate vertices for radar chart
        for (i, point) in data.iter().enumerate() {
            let angle = 2.0 * std::f32::consts::PI * i as f32 / data.len() as f32;
            let normalized_value = (point.value / point.max_value) as f32;
            let x = center_x + radius * normalized_value * angle.cos();
            let y = center_y + radius * normalized_value * angle.sin();

            vertices.push([x, y, 0.0, 1.0]); // x, y, z, w
        }

        // Generate indices for line segments
        for i in 0..data.len() {
            let next_i = (i + 1) % data.len();
            indices.push(i as u32);
            indices.push(next_i as u32);
        }

        // Mock implementation - in real implementation, this would use WebGPU
        Ok(WebGpuRenderResult {
            render_time_ms: 2.5,
            memory_used_bytes: vertices.len() * 16 + indices.len() * 4,
            vertices_rendered: vertices.len(),
        })
    }

    fn render_sankey_diagram(
        &mut self,
        config: &SankeyConfig,
    ) -> Result<WebGpuRenderResult, WebGpuError> {
        // Calculate node positions and sizes
        let mut node_positions = HashMap::new();
        let mut total_height = 0.0;

        for node in &config.nodes {
            let height = (node.value / config.nodes.iter().map(|n| n.value).sum::<f64>()) as f32
                * config.base_config.height as f32;
            node_positions.insert(node.id.clone(), (0.0, total_height, height));
            total_height += height + config.node_padding;
        }

        // Generate vertices for nodes and links
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Render nodes
        for node in &config.nodes {
            if let Some((x, y, height)) = node_positions.get(&node.id) {
                // Create rectangle for node
                let x1 = *x;
                let y1 = *y;
                let x2 = x1 + config.node_width;
                let y2 = y1 + *height;

                // Add rectangle vertices
                vertices.extend_from_slice(&[
                    [x1, y1, 0.0, 1.0],
                    [x2, y1, 0.0, 1.0],
                    [x2, y2, 0.0, 1.0],
                    [x1, y2, 0.0, 1.0],
                ]);

                // Add rectangle indices
                let base_idx = vertices.len() as u32 - 4;
                indices.extend_from_slice(&[
                    base_idx,
                    base_idx + 1,
                    base_idx + 2,
                    base_idx,
                    base_idx + 2,
                    base_idx + 3,
                ]);
            }
        }

        // Mock implementation - in real implementation, this would use WebGPU
        Ok(WebGpuRenderResult {
            render_time_ms: 5.2,
            memory_used_bytes: vertices.len() * 16 + indices.len() * 4,
            vertices_rendered: vertices.len(),
        })
    }

    fn render_treemap(
        &mut self,
        config: &TreemapConfig,
    ) -> Result<WebGpuRenderResult, WebGpuError> {
        // Recursive treemap layout algorithm
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        fn layout_treemap(
            node: &TreemapNode,
            x: f32,
            y: f32,
            width: f32,
            height: f32,
            vertices: &mut Vec<[f32; 4]>,
            indices: &mut Vec<u32>,
            padding: f32,
        ) {
            if node.children.is_empty() {
                // Leaf node - create rectangle
                let x1 = x + padding;
                let y1 = y + padding;
                let x2 = x + width - padding;
                let y2 = y + height - padding;

                let base_idx = vertices.len() as u32;
                vertices.extend_from_slice(&[
                    [x1, y1, 0.0, 1.0],
                    [x2, y1, 0.0, 1.0],
                    [x2, y2, 0.0, 1.0],
                    [x1, y2, 0.0, 1.0],
                ]);

                indices.extend_from_slice(&[
                    base_idx,
                    base_idx + 1,
                    base_idx + 2,
                    base_idx,
                    base_idx + 2,
                    base_idx + 3,
                ]);
            } else {
                // Internal node - recursively layout children
                let total_value: f64 = node.children.iter().map(|c| c.value).sum();
                let mut current_x = x;
                let mut current_y = y;

                for child in &node.children {
                    let child_ratio = child.value / total_value;
                    let child_width = width * child_ratio as f32;
                    let child_height = height * child_ratio as f32;

                    layout_treemap(
                        child,
                        current_x,
                        current_y,
                        child_width,
                        child_height,
                        vertices,
                        indices,
                        padding,
                    );

                    current_x += child_width;
                    if current_x >= x + width {
                        current_x = x;
                        current_y += child_height;
                    }
                }
            }
        }

        layout_treemap(
            &config.root_node,
            0.0,
            0.0,
            config.base_config.width as f32,
            config.base_config.height as f32,
            &mut vertices,
            &mut indices,
            config.padding,
        );

        // Mock implementation - in real implementation, this would use WebGPU
        Ok(WebGpuRenderResult {
            render_time_ms: 3.8,
            memory_used_bytes: vertices.len() * 16 + indices.len() * 4,
            vertices_rendered: vertices.len(),
        })
    }

    fn render_violin_plot(
        &mut self,
        config: &ViolinConfig,
    ) -> Result<WebGpuRenderResult, WebGpuError> {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let category_width = config.base_config.width as f32 / config.data.len() as f32;
        let max_value = config
            .data
            .iter()
            .flat_map(|d| &d.values)
            .fold(0.0_f64, |acc, &val| acc.max(val)) as f32;

        for (i, data_point) in config.data.iter().enumerate() {
            let category_x = i as f32 * category_width + category_width / 2.0;
            let center_y = config.base_config.height as f32 / 2.0;

            // Calculate kernel density estimation for violin shape
            let mut density_points = Vec::new();
            for &value in &data_point.values {
                let normalized_value =
                    (value as f32 / max_value) * (config.base_config.height as f32 * 0.4);
                density_points.push(normalized_value);
            }

            // Create violin shape vertices
            for (j, &density) in density_points.iter().enumerate() {
                let angle = 2.0 * std::f32::consts::PI * j as f32 / density_points.len() as f32;
                let x = category_x + density * angle.cos();
                let y = center_y + density * angle.sin();

                vertices.push([x, y, 0.0, 1.0]);
            }

            // Generate indices for violin shape
            let base_idx = vertices.len() as u32 - density_points.len() as u32;
            for j in 0..density_points.len() {
                let next_j = (j + 1) % density_points.len();
                indices.extend_from_slice(&[base_idx + j as u32, base_idx + next_j as u32]);
            }
        }

        // Mock implementation - in real implementation, this would use WebGPU
        Ok(WebGpuRenderResult {
            render_time_ms: 4.1,
            memory_used_bytes: vertices.len() * 16 + indices.len() * 4,
            vertices_rendered: vertices.len(),
        })
    }
}

/// Helper function to count treemap nodes recursively
pub fn count_treemap_nodes(node: &TreemapNode) -> usize {
    1 + node.children.iter().map(count_treemap_nodes).sum::<usize>()
}

/// Sample data generators for testing and examples
pub mod sample_data {
    use super::*;

    pub fn create_sample_radar_data() -> Vec<RadarDataPoint> {
        vec![
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
        ]
    }

    pub fn create_sample_sankey_data() -> (Vec<SankeyNode>, Vec<SankeyLink>) {
        let nodes = vec![
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
                id: "intermediate".to_string(),
                name: "Intermediate".to_string(),
                value: 0.0,
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
        ];

        let links = vec![
            SankeyLink {
                source: "source1".to_string(),
                target: "intermediate".to_string(),
                value: 60.0,
            },
            SankeyLink {
                source: "source2".to_string(),
                target: "intermediate".to_string(),
                value: 50.0,
            },
            SankeyLink {
                source: "intermediate".to_string(),
                target: "target1".to_string(),
                value: 70.0,
            },
            SankeyLink {
                source: "intermediate".to_string(),
                target: "target2".to_string(),
                value: 40.0,
            },
        ];

        (nodes, links)
    }

    pub fn create_sample_treemap_data() -> TreemapNode {
        TreemapNode {
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
        }
    }

    pub fn create_sample_violin_data() -> Vec<ViolinDataPoint> {
        vec![
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
        ]
    }
}
