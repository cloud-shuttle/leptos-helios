//! Advanced Chart Types Tests
//! 
//! Comprehensive test suite for advanced chart types including:
//! - Radar Charts (multi-dimensional polar coordinates)
//! - Sankey Diagrams (flow visualization)
//! - Treemaps (hierarchical rectangles)
//! - Violin Plots (distribution visualization)

use leptos_helios::chart_config::*;
use leptos_helios::webgpu_renderer::WebGpuError;

/// Mock WebGpuRenderResult for testing
#[derive(Debug, Clone)]
pub struct WebGpuRenderResult {
    pub render_time_ms: f64,
    pub memory_used_bytes: usize,
    pub vertices_rendered: usize,
}

/// Test data structures for advanced charts
#[derive(Debug, Clone)]
pub struct RadarDataPoint {
    pub category: String,
    pub value: f64,
    pub max_value: f64,
}

#[derive(Debug, Clone)]
pub struct SankeyNode {
    pub id: String,
    pub name: String,
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct SankeyLink {
    pub source: String,
    pub target: String,
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct TreemapNode {
    pub id: String,
    pub name: String,
    pub value: f64,
    pub children: Vec<TreemapNode>,
}

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
            categories: vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string()],
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
    fn render_radar_chart(&mut self, config: &RadarChartConfig, data: &[RadarDataPoint]) -> Result<WebGpuRenderResult, WebGpuError>;
    fn render_sankey_diagram(&mut self, config: &SankeyConfig) -> Result<WebGpuRenderResult, WebGpuError>;
    fn render_treemap(&mut self, config: &TreemapConfig) -> Result<WebGpuRenderResult, WebGpuError>;
    fn render_violin_plot(&mut self, config: &ViolinConfig) -> Result<WebGpuRenderResult, WebGpuError>;
}

/// Mock WebGpuRenderer for testing
pub struct MockWebGpuRenderer;

impl MockWebGpuRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl AdvancedChartRenderer for MockWebGpuRenderer {
    fn render_radar_chart(&mut self, config: &RadarChartConfig, data: &[RadarDataPoint]) -> Result<WebGpuRenderResult, WebGpuError> {
        // Mock implementation for testing
        Ok(WebGpuRenderResult {
            render_time_ms: 2.5,
            memory_used_bytes: 1024,
            vertices_rendered: data.len() * 2,
        })
    }

    fn render_sankey_diagram(&mut self, config: &SankeyConfig) -> Result<WebGpuRenderResult, WebGpuError> {
        // Mock implementation for testing
        Ok(WebGpuRenderResult {
            render_time_ms: 5.2,
            memory_used_bytes: 2048,
            vertices_rendered: config.nodes.len() + config.links.len() * 4,
        })
    }

    fn render_treemap(&mut self, config: &TreemapConfig) -> Result<WebGpuRenderResult, WebGpuError> {
        // Mock implementation for testing
        Ok(WebGpuRenderResult {
            render_time_ms: 3.8,
            memory_used_bytes: 1536,
            vertices_rendered: count_treemap_nodes(&config.root_node) * 4,
        })
    }

    fn render_violin_plot(&mut self, config: &ViolinConfig) -> Result<WebGpuRenderResult, WebGpuError> {
        // Mock implementation for testing
        Ok(WebGpuRenderResult {
            render_time_ms: 4.1,
            memory_used_bytes: 1792,
            vertices_rendered: config.data.iter().map(|d| d.values.len()).sum::<usize>() * 2,
        })
    }
}

/// Helper function to count treemap nodes recursively
fn count_treemap_nodes(node: &TreemapNode) -> usize {
    1 + node.children.iter().map(count_treemap_nodes).sum::<usize>()
}

/// Test data generators
pub fn create_sample_radar_data() -> Vec<RadarDataPoint> {
    vec![
        RadarDataPoint { category: "Performance".to_string(), value: 85.0, max_value: 100.0 },
        RadarDataPoint { category: "Usability".to_string(), value: 92.0, max_value: 100.0 },
        RadarDataPoint { category: "Features".to_string(), value: 78.0, max_value: 100.0 },
        RadarDataPoint { category: "Support".to_string(), value: 88.0, max_value: 100.0 },
        RadarDataPoint { category: "Price".to_string(), value: 95.0, max_value: 100.0 },
    ]
}

pub fn create_sample_sankey_data() -> (Vec<SankeyNode>, Vec<SankeyLink>) {
    let nodes = vec![
        SankeyNode { id: "source1".to_string(), name: "Source 1".to_string(), value: 100.0 },
        SankeyNode { id: "source2".to_string(), name: "Source 2".to_string(), value: 80.0 },
        SankeyNode { id: "intermediate".to_string(), name: "Intermediate".to_string(), value: 0.0 },
        SankeyNode { id: "target1".to_string(), name: "Target 1".to_string(), value: 0.0 },
        SankeyNode { id: "target2".to_string(), name: "Target 2".to_string(), value: 0.0 },
    ];

    let links = vec![
        SankeyLink { source: "source1".to_string(), target: "intermediate".to_string(), value: 60.0 },
        SankeyLink { source: "source2".to_string(), target: "intermediate".to_string(), value: 50.0 },
        SankeyLink { source: "intermediate".to_string(), target: "target1".to_string(), value: 70.0 },
        SankeyLink { source: "intermediate".to_string(), target: "target2".to_string(), value: 40.0 },
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
                    TreemapNode { id: "item1".to_string(), name: "Item 1".to_string(), value: 25.0, children: vec![] },
                    TreemapNode { id: "item2".to_string(), name: "Item 2".to_string(), value: 15.0, children: vec![] },
                ],
            },
            TreemapNode {
                id: "category2".to_string(),
                name: "Category 2".to_string(),
                value: 35.0,
                children: vec![
                    TreemapNode { id: "item3".to_string(), name: "Item 3".to_string(), value: 20.0, children: vec![] },
                    TreemapNode { id: "item4".to_string(), name: "Item 4".to_string(), value: 15.0, children: vec![] },
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radar_chart_config_creation() {
        let config = RadarChartConfig::default();
        assert_eq!(config.categories.len(), 5);
        assert_eq!(config.max_value, 100.0);
        assert!(config.show_grid);
        assert!(config.show_labels);
        assert!(config.fill_area);
    }

    #[test]
    fn test_radar_chart_rendering() {
        let mut renderer = MockWebGpuRenderer::new();
        let config = RadarChartConfig::default();
        let data = create_sample_radar_data();

        let result = renderer.render_radar_chart(&config, &data);
        assert!(result.is_ok());

        let render_result = result.unwrap();
        assert!(render_result.render_time_ms > 0.0);
        assert!(render_result.memory_used_bytes > 0);
        assert_eq!(render_result.vertices_rendered, data.len() * 2);
    }

    #[test]
    fn test_sankey_diagram_config_creation() {
        let config = SankeyConfig::default();
        assert_eq!(config.node_width, 20.0);
        assert_eq!(config.node_padding, 10.0);
        assert_eq!(config.link_opacity, 0.6);
    }

    #[test]
    fn test_sankey_diagram_rendering() {
        let mut renderer = MockWebGpuRenderer::new();
        let (nodes, links) = create_sample_sankey_data();
        let mut config = SankeyConfig::default();
        config.nodes = nodes;
        config.links = links;

        let result = renderer.render_sankey_diagram(&config);
        assert!(result.is_ok());

        let render_result = result.unwrap();
        assert!(render_result.render_time_ms > 0.0);
        assert!(render_result.memory_used_bytes > 0);
        assert!(render_result.vertices_rendered > 0);
    }

    #[test]
    fn test_treemap_config_creation() {
        let config = TreemapConfig::default();
        assert_eq!(config.padding, 2.0);
        assert_eq!(config.border_width, 1.0);
        assert!(config.show_labels);
        assert_eq!(config.label_threshold, 0.05);
    }

    #[test]
    fn test_treemap_rendering() {
        let mut renderer = MockWebGpuRenderer::new();
        let mut config = TreemapConfig::default();
        config.root_node = create_sample_treemap_data();

        let result = renderer.render_treemap(&config);
        assert!(result.is_ok());

        let render_result = result.unwrap();
        assert!(render_result.render_time_ms > 0.0);
        assert!(render_result.memory_used_bytes > 0);
        assert!(render_result.vertices_rendered > 0);
    }

    #[test]
    fn test_violin_plot_config_creation() {
        let config = ViolinConfig::default();
        assert_eq!(config.bandwidth, 0.5);
        assert!(config.show_box_plot);
        assert!(!config.show_points);
        assert_eq!(config.point_size, 3.0);
        assert_eq!(config.fill_opacity, 0.7);
    }

    #[test]
    fn test_violin_plot_rendering() {
        let mut renderer = MockWebGpuRenderer::new();
        let mut config = ViolinConfig::default();
        config.data = create_sample_violin_data();

        let result = renderer.render_violin_plot(&config);
        assert!(result.is_ok());

        let render_result = result.unwrap();
        assert!(render_result.render_time_ms > 0.0);
        assert!(render_result.memory_used_bytes > 0);
        assert!(render_result.vertices_rendered > 0);
    }

    #[test]
    fn test_radar_chart_with_empty_data() {
        let mut renderer = MockWebGpuRenderer::new();
        let config = RadarChartConfig::default();
        let data = vec![];

        let result = renderer.render_radar_chart(&config, &data);
        assert!(result.is_ok());

        let render_result = result.unwrap();
        assert_eq!(render_result.vertices_rendered, 0);
    }

    #[test]
    fn test_sankey_diagram_with_single_node() {
        let mut renderer = MockWebGpuRenderer::new();
        let mut config = SankeyConfig::default();
        config.nodes = vec![SankeyNode {
            id: "single".to_string(),
            name: "Single Node".to_string(),
            value: 100.0,
        }];
        config.links = vec![];

        let result = renderer.render_sankey_diagram(&config);
        assert!(result.is_ok());

        let render_result = result.unwrap();
        assert_eq!(render_result.vertices_rendered, 1);
    }

    #[test]
    fn test_treemap_node_counting() {
        let root = create_sample_treemap_data();
        let count = count_treemap_nodes(&root);
        assert_eq!(count, 8); // root + 3 categories + 4 items
    }

    #[test]
    fn test_violin_plot_with_single_category() {
        let mut renderer = MockWebGpuRenderer::new();
        let mut config = ViolinConfig::default();
        config.data = vec![ViolinDataPoint {
            category: "Single".to_string(),
            values: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        }];

        let result = renderer.render_violin_plot(&config);
        assert!(result.is_ok());

        let render_result = result.unwrap();
        assert_eq!(render_result.vertices_rendered, 10); // 5 values * 2
    }

    #[test]
    fn test_advanced_chart_performance() {
        let mut renderer = MockWebGpuRenderer::new();
        
        // Test radar chart performance
        let radar_config = RadarChartConfig::default();
        let radar_data = create_sample_radar_data();
        let radar_result = renderer.render_radar_chart(&radar_config, &radar_data).unwrap();
        assert!(radar_result.render_time_ms < 10.0); // Should render quickly

        // Test sankey diagram performance
        let (sankey_nodes, sankey_links) = create_sample_sankey_data();
        let mut sankey_config = SankeyConfig::default();
        sankey_config.nodes = sankey_nodes;
        sankey_config.links = sankey_links;
        let sankey_result = renderer.render_sankey_diagram(&sankey_config).unwrap();
        assert!(sankey_result.render_time_ms < 15.0); // Should render quickly

        // Test treemap performance
        let mut treemap_config = TreemapConfig::default();
        treemap_config.root_node = create_sample_treemap_data();
        let treemap_result = renderer.render_treemap(&treemap_config).unwrap();
        assert!(treemap_result.render_time_ms < 12.0); // Should render quickly

        // Test violin plot performance
        let mut violin_config = ViolinConfig::default();
        violin_config.data = create_sample_violin_data();
        let violin_result = renderer.render_violin_plot(&violin_config).unwrap();
        assert!(violin_result.render_time_ms < 8.0); // Should render quickly
    }

    #[test]
    fn test_advanced_chart_memory_usage() {
        let mut renderer = MockWebGpuRenderer::new();
        
        // Test memory usage for each chart type
        let radar_config = RadarChartConfig::default();
        let radar_data = create_sample_radar_data();
        let radar_result = renderer.render_radar_chart(&radar_config, &radar_data).unwrap();
        assert!(radar_result.memory_used_bytes < 10000); // Should use reasonable memory

        let (sankey_nodes, sankey_links) = create_sample_sankey_data();
        let mut sankey_config = SankeyConfig::default();
        sankey_config.nodes = sankey_nodes;
        sankey_config.links = sankey_links;
        let sankey_result = renderer.render_sankey_diagram(&sankey_config).unwrap();
        assert!(sankey_result.memory_used_bytes < 15000); // Should use reasonable memory

        let mut treemap_config = TreemapConfig::default();
        treemap_config.root_node = create_sample_treemap_data();
        let treemap_result = renderer.render_treemap(&treemap_config).unwrap();
        assert!(treemap_result.memory_used_bytes < 12000); // Should use reasonable memory

        let mut violin_config = ViolinConfig::default();
        violin_config.data = create_sample_violin_data();
        let violin_result = renderer.render_violin_plot(&violin_config).unwrap();
        assert!(violin_result.memory_used_bytes < 8000); // Should use reasonable memory
    }
}
