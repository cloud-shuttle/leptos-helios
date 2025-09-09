//! Advanced Chart Types for leptos-helios
//!
//! This module implements advanced chart types identified in the feature gap analysis:
//! - Heatmaps with color mapping and clustering
//! - Treemaps with hierarchical data visualization
//! - Sankey diagrams with flow visualization
//!
//! Following TDD methodology: Red -> Green -> Refactor

use std::collections::HashMap;
use std::time::Duration;

/// Heatmap chart for visualizing 2D data with color mapping
#[derive(Debug, Clone)]
pub struct HeatmapChart {
    /// 2D data matrix
    data: Vec<Vec<f64>>,
    /// Color scheme for mapping values to colors
    color_scheme: ColorScheme,
    /// Clusters of similar values
    clusters: Vec<Cluster>,
    /// Selected cells
    selected_cells: std::collections::HashSet<(usize, usize)>,
    /// Min and max values for normalization
    min_value: f64,
    max_value: f64,
    /// Cell dimensions for rendering
    cell_width: f64,
    cell_height: f64,
}

impl HeatmapChart {
    /// Create a new heatmap chart
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        let (min_val, max_val) = Self::calculate_min_max(&data);
        Self {
            data: data.clone(),
            color_scheme: ColorScheme::Viridis,
            clusters: Vec::new(),
            selected_cells: std::collections::HashSet::new(),
            min_value: min_val,
            max_value: max_val,
            cell_width: 0.0,
            cell_height: 0.0,
        }
    }

    /// Get number of rows
    pub fn rows(&self) -> usize {
        self.data.len()
    }

    /// Get number of columns
    pub fn cols(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.data[0].len()
        }
    }

    /// Get value at specific position
    pub fn get_value(&self, row: usize, col: usize) -> f64 {
        self.data[row][col]
    }

    /// Set color scheme
    pub fn set_color_scheme(&mut self, scheme: ColorScheme) {
        self.color_scheme = scheme;
    }

    /// Get color for specific cell
    pub fn get_color(&self, row: usize, col: usize) -> String {
        let value = self.get_value(row, col);
        let normalized = (value - self.min_value) / (self.max_value - self.min_value);
        self.color_scheme.get_color(normalized)
    }

    /// Check if color is valid
    pub fn is_valid_color(&self, color: String) -> bool {
        color.starts_with('#') && color.len() == 7
    }

    /// Enable clustering of similar values
    pub fn enable_clustering(&mut self, num_clusters: usize) {
        self.clusters = self.perform_clustering(num_clusters);
    }

    /// Get clusters
    pub fn get_clusters(&self) -> &Vec<Cluster> {
        &self.clusters
    }

    /// Get cell at screen position
    pub fn get_cell_at_position(&self, x: f64, y: f64) -> Option<(usize, usize)> {
        if self.cell_width == 0.0 || self.cell_height == 0.0 {
            return None;
        }

        let col = (x / self.cell_width) as usize;
        let row = (y / self.cell_height) as usize;

        if row < self.rows() && col < self.cols() {
            Some((row, col))
        } else {
            None
        }
    }

    /// Select a cell
    pub fn select_cell(&mut self, row: usize, col: usize) {
        self.selected_cells.insert((row, col));
    }

    /// Check if cell is selected
    pub fn is_cell_selected(&self, row: usize, col: usize) -> bool {
        self.selected_cells.contains(&(row, col))
    }

    /// Export as PNG
    pub fn export_png(&self, width: u32, height: u32) -> Vec<u8> {
        // Placeholder implementation
        vec![0u8; (width * height * 4) as usize]
    }

    /// Export as SVG
    pub fn export_svg(&self, width: u32, height: u32) -> String {
        let mut svg = format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
            width, height
        );

        let cell_w = width as f64 / self.cols() as f64;
        let cell_h = height as f64 / self.rows() as f64;

        for row in 0..self.rows() {
            for col in 0..self.cols() {
                let color = self.get_color(row, col);
                let x = col as f64 * cell_w;
                let y = row as f64 * cell_h;

                svg.push_str(&format!(
                    r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" />"#,
                    x, y, cell_w, cell_h, color
                ));
            }
        }

        svg.push_str("</svg>");
        svg
    }

    /// Calculate min and max values
    fn calculate_min_max(data: &[Vec<f64>]) -> (f64, f64) {
        let mut min_val = f64::INFINITY;
        let mut max_val = f64::NEG_INFINITY;

        for row in data {
            for &value in row {
                min_val = min_val.min(value);
                max_val = max_val.max(value);
            }
        }

        (min_val, max_val)
    }

    /// Perform clustering using k-means
    fn perform_clustering(&self, num_clusters: usize) -> Vec<Cluster> {
        let mut clusters = Vec::new();

        // Simple clustering based on value ranges
        let range = self.max_value - self.min_value;
        let cluster_size = range / num_clusters as f64;

        for i in 0..num_clusters {
            let start = self.min_value + i as f64 * cluster_size;
            let end = start + cluster_size;

            let mut count = 0;
            for row in &self.data {
                for &value in row {
                    if value >= start && value < end {
                        count += 1;
                    }
                }
            }

            clusters.push(Cluster { size: count });
        }

        clusters
    }
}

/// Treemap chart for hierarchical data visualization
#[derive(Debug, Clone)]
pub struct TreemapChart {
    /// Root node of the tree
    root: TreeNode,
    /// Current root for drill-down functionality
    current_root: TreeNode,
    /// Layout algorithm
    layout_algorithm: LayoutAlgorithm,
    /// Color strategy
    color_strategy: ColorStrategy,
    /// Selected nodes
    selected_nodes: std::collections::HashSet<String>,
    /// Node colors cache
    node_colors: HashMap<String, String>,
}

impl TreemapChart {
    /// Create a new treemap chart
    pub fn new(root: TreeNode) -> Self {
        let current_root = root.clone();
        Self {
            root,
            current_root,
            layout_algorithm: LayoutAlgorithm::Squarified,
            color_strategy: ColorStrategy::ByValue,
            selected_nodes: std::collections::HashSet::new(),
            node_colors: HashMap::new(),
        }
    }

    /// Get total number of nodes
    pub fn total_nodes(&self) -> usize {
        self.count_nodes(&self.root)
    }

    /// Get root node
    pub fn get_root(&self) -> &TreeNode {
        &self.root
    }

    /// Set layout algorithm
    pub fn set_layout_algorithm(&mut self, algorithm: LayoutAlgorithm) {
        self.layout_algorithm = algorithm;
    }

    /// Layout the treemap
    pub fn layout(&mut self, width: f64, height: f64) {
        let mut current_root = self.current_root.clone();
        self.layout_squarified(&mut current_root, 0.0, 0.0, width, height);
        self.current_root = current_root;
    }

    /// Get all nodes
    pub fn get_all_nodes(&self) -> Vec<&TreeNode> {
        self.collect_nodes(&self.current_root)
    }

    /// Set color strategy
    pub fn set_color_strategy(&mut self, strategy: ColorStrategy) {
        self.color_strategy = strategy;
        self.node_colors.clear();
    }

    /// Get node color
    pub fn get_node_color(&self, name: &str) -> String {
        self.node_colors
            .get(name)
            .cloned()
            .unwrap_or_else(|| "#808080".to_string())
    }

    /// Get all node colors
    pub fn get_all_node_colors(&self) -> Vec<String> {
        self.node_colors.values().cloned().collect()
    }

    /// Select a node
    pub fn select_node(&mut self, name: &str) {
        self.selected_nodes.insert(name.to_string());
    }

    /// Check if node is selected
    pub fn is_node_selected(&self, name: &str) -> bool {
        self.selected_nodes.contains(name)
    }

    /// Drill down to a node
    pub fn drill_down(&mut self, name: &str) {
        if let Some(node) = self.find_node(&self.current_root, name) {
            self.current_root = node.clone();
        }
    }

    /// Drill up to parent
    pub fn drill_up(&mut self) {
        // For simplicity, always go back to root
        self.current_root = self.root.clone();
    }

    /// Get current root
    pub fn get_current_root(&self) -> &TreeNode {
        &self.current_root
    }

    /// Animate to new tree state
    pub fn animate_to(&mut self, _new_root: TreeNode, _duration: Duration) -> Option<Animation> {
        // Placeholder implementation
        Some(Animation { progress: 0.0 })
    }

    /// Count nodes recursively
    fn count_nodes(&self, node: &TreeNode) -> usize {
        1 + node
            .children
            .iter()
            .map(|child| self.count_nodes(child))
            .sum::<usize>()
    }

    /// Collect all nodes recursively
    fn collect_nodes<'a>(&self, node: &'a TreeNode) -> Vec<&'a TreeNode> {
        let mut nodes = vec![node];
        for child in &node.children {
            nodes.extend(self.collect_nodes(child));
        }
        nodes
    }

    /// Find node by name
    fn find_node<'a>(&self, node: &'a TreeNode, name: &str) -> Option<&'a TreeNode> {
        if node.name == name {
            return Some(node);
        }

        for child in &node.children {
            if let Some(found) = self.find_node(child, name) {
                return Some(found);
            }
        }

        None
    }

    /// Layout using squarified algorithm
    fn layout_squarified(&mut self, node: &mut TreeNode, x: f64, y: f64, width: f64, height: f64) {
        node.rectangle = Rectangle {
            x,
            y,
            width,
            height,
        };

        if node.children.is_empty() {
            return;
        }

        // Simple layout: divide space proportionally
        let total_value: f64 = node.children.iter().map(|child| child.value).sum();
        let mut current_x = x;
        let mut current_y = y;

        for child in &mut node.children {
            let ratio = child.value / total_value;
            let child_width = width * ratio;
            let child_height = height * ratio;

            self.layout_squarified(child, current_x, current_y, child_width, child_height);

            current_x += child_width;
            if current_x >= x + width {
                current_x = x;
                current_y += child_height;
            }
        }
    }
}

/// Sankey diagram for flow visualization
#[derive(Debug, Clone)]
pub struct SankeyDiagram {
    /// Nodes in the diagram
    nodes: Vec<SankeyNode>,
    /// Links between nodes
    links: Vec<SankeyLink>,
    /// Selected nodes
    selected_nodes: std::collections::HashSet<usize>,
    /// Highlighted links
    highlighted_links: std::collections::HashSet<usize>,
    /// Layout dimensions
    width: f64,
    height: f64,
}

impl SankeyDiagram {
    /// Create a new Sankey diagram
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            links: Vec::new(),
            selected_nodes: std::collections::HashSet::new(),
            highlighted_links: std::collections::HashSet::new(),
            width: 0.0,
            height: 0.0,
        }
    }

    /// Add a node
    pub fn add_node(&mut self, name: &str, value: f64) -> usize {
        let id = self.nodes.len();
        self.nodes.push(SankeyNode {
            id,
            name: name.to_string(),
            value,
            position: Point { x: 0.0, y: 0.0 },
        });
        id
    }

    /// Add a link between nodes
    pub fn add_link(&mut self, source: usize, target: usize, value: f64) -> usize {
        let id = self.links.len();
        self.links.push(SankeyLink {
            id,
            source,
            target,
            value,
            path: Vec::new(),
            width: 0.0,
        });
        id
    }

    /// Get number of nodes
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get number of links
    pub fn link_count(&self) -> usize {
        self.links.len()
    }

    /// Get node value
    pub fn get_node_value(&self, node_id: usize) -> f64 {
        self.nodes[node_id].value
    }

    /// Layout the diagram
    pub fn layout(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;

        // Simple layout: arrange nodes in columns
        let num_columns = 3;
        let column_width = width / num_columns as f64;
        let total_rows = (self.nodes.len() + num_columns - 1) / num_columns;
        let row_height = height / total_rows as f64;

        for (i, node) in self.nodes.iter_mut().enumerate() {
            let column = i % num_columns;
            let row = i / num_columns;

            node.position = Point {
                x: column as f64 * column_width,
                y: row as f64 * row_height,
            };
        }

        // Calculate link paths and widths
        let max_flow: f64 = self.links.iter().map(|link| link.value).sum();
        for link in &mut self.links {
            link.width = (link.value / max_flow) * 50.0; // Max width of 50
            link.path = vec![
                self.nodes[link.source].position.clone(),
                self.nodes[link.target].position.clone(),
            ];
        }
    }

    /// Get all node IDs
    pub fn get_all_node_ids(&self) -> Vec<usize> {
        (0..self.nodes.len()).collect()
    }

    /// Get node position
    pub fn get_node_position(&self, node_id: usize) -> Point {
        self.nodes[node_id].position.clone()
    }

    /// Get all link IDs
    pub fn get_all_link_ids(&self) -> Vec<usize> {
        (0..self.links.len()).collect()
    }

    /// Get link path
    pub fn get_link_path(&self, link_id: usize) -> Vec<Point> {
        self.links[link_id].path.clone()
    }

    /// Get link width
    pub fn get_link_width(&self, link_id: usize) -> f64 {
        self.links[link_id].width
    }

    /// Get link direction
    pub fn get_link_direction(&self, link_id: usize) -> Direction {
        let link = &self.links[link_id];
        let source_pos = &self.nodes[link.source].position;
        let target_pos = &self.nodes[link.target].position;

        if target_pos.x > source_pos.x {
            Direction::Right
        } else if target_pos.x < source_pos.x {
            Direction::Left
        } else if target_pos.y > source_pos.y {
            Direction::Down
        } else if target_pos.y < source_pos.y {
            Direction::Up
        } else {
            Direction::Invalid
        }
    }

    /// Get node at position
    pub fn get_node_at_position(&self, x: f64, y: f64) -> Option<usize> {
        for (i, node) in self.nodes.iter().enumerate() {
            let dx = x - node.position.x;
            let dy = y - node.position.y;
            if dx.abs() < 20.0 && dy.abs() < 20.0 {
                return Some(i);
            }
        }
        None
    }

    /// Get link at position
    pub fn get_link_at_position(&self, x: f64, y: f64) -> Option<usize> {
        for (i, link) in self.links.iter().enumerate() {
            if link.path.len() >= 2 {
                let start = &link.path[0];
                let end = &link.path[1];

                // Simple point-to-line distance check
                let distance = self.point_to_line_distance(x, y, start.x, start.y, end.x, end.y);
                if distance < link.width / 2.0 {
                    return Some(i);
                }
            }
        }
        None
    }

    /// Select a node
    pub fn select_node(&mut self, node_id: usize) {
        self.selected_nodes.insert(node_id);
    }

    /// Check if node is selected
    pub fn is_node_selected(&self, node_id: usize) -> bool {
        self.selected_nodes.contains(&node_id)
    }

    /// Highlight a link
    pub fn highlight_link(&mut self, link_id: usize) {
        self.highlighted_links.insert(link_id);
    }

    /// Check if link is highlighted
    pub fn is_link_highlighted(&self, link_id: usize) -> bool {
        self.highlighted_links.contains(&link_id)
    }

    /// Animate to another diagram
    pub fn animate_to(&mut self, _other: &SankeyDiagram, _duration: Duration) -> Option<Animation> {
        // Placeholder implementation
        Some(Animation { progress: 0.0 })
    }

    /// Export as SVG
    pub fn export_svg(&self, width: u32, height: u32) -> String {
        let mut svg = format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
            width, height
        );

        // Draw links
        for link in &self.links {
            if link.path.len() >= 2 {
                let start = &link.path[0];
                let end = &link.path[1];
                svg.push_str(&format!(
                    "<path d=\"M {} {} L {} {}\" stroke=\"#999\" stroke-width=\"{}\" fill=\"none\" />",
                    start.x, start.y, end.x, end.y, link.width
                ));
            }
        }

        // Draw nodes
        for node in &self.nodes {
            svg.push_str(&format!(
                "<rect x=\"{}\" y=\"{}\" width=\"20\" height=\"20\" fill=\"#666\" />",
                node.position.x - 10.0,
                node.position.y - 10.0
            ));
        }

        svg.push_str("</svg>");
        svg
    }

    /// Export as JSON
    pub fn export_json(&self) -> String {
        format!(
            r#"{{"nodes": {}, "links": {}}}"#,
            serde_json::to_string(&self.nodes).unwrap_or_else(|_| "[]".to_string()),
            serde_json::to_string(&self.links).unwrap_or_else(|_| "[]".to_string())
        )
    }

    /// Set link value
    pub fn set_link_value(&mut self, link_id: usize, value: f64) {
        if link_id < self.links.len() {
            self.links[link_id].value = value;
        }
    }

    /// Calculate point to line distance
    fn point_to_line_distance(&self, px: f64, py: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let length = (dx * dx + dy * dy).sqrt();

        if length == 0.0 {
            return ((px - x1) * (px - x1) + (py - y1) * (py - y1)).sqrt();
        }

        let t = ((px - x1) * dx + (py - y1) * dy) / (length * length);
        let t = t.max(0.0).min(1.0);

        let closest_x = x1 + t * dx;
        let closest_y = y1 + t * dy;

        ((px - closest_x) * (px - closest_x) + (py - closest_y) * (py - closest_y)).sqrt()
    }
}

// Supporting types
#[derive(Debug, Clone)]
pub struct TreeNode {
    pub name: String,
    pub value: f64,
    pub children: Vec<TreeNode>,
    pub rectangle: Rectangle,
}

impl TreeNode {
    pub fn new(name: &str, value: f64) -> Self {
        Self {
            name: name.to_string(),
            value,
            children: Vec::new(),
            rectangle: Rectangle {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
        }
    }

    pub fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn get_rectangle(&self) -> &Rectangle {
        &self.rectangle
    }
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub struct Cluster {
    size: usize,
}

impl Cluster {
    pub fn size(&self) -> usize {
        self.size
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SankeyNode {
    id: usize,
    name: String,
    value: f64,
    position: Point,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SankeyLink {
    id: usize,
    source: usize,
    target: usize,
    value: f64,
    path: Vec<Point>,
    width: f64,
}

#[derive(Debug, Clone)]
pub struct Animation {
    progress: f64,
}

impl Animation {
    pub fn get_progress(&self) -> f64 {
        self.progress
    }
}

#[derive(Debug, Clone)]
pub struct ChartRenderer;

impl ChartRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn can_render<T>(&self, _chart: &T) -> bool {
        true // All chart types can be rendered
    }
}

#[derive(Debug, Clone)]
pub enum ColorScheme {
    Default,
    Viridis,
    Plasma,
    Inferno,
    Magma,
}

impl ColorScheme {
    pub fn get_color(&self, normalized_value: f64) -> String {
        match self {
            ColorScheme::Viridis => {
                // Simplified Viridis color mapping
                let r = (normalized_value * 255.0) as u8;
                let g = ((1.0 - normalized_value) * 255.0) as u8;
                let b = 128;
                format!("#{:02x}{:02x}{:02x}", r, g, b)
            }
            _ => "#808080".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LayoutAlgorithm {
    Squarified,
    SliceAndDice,
    Strip,
}

#[derive(Debug, Clone)]
pub enum ColorStrategy {
    ByValue,
    ByDepth,
    ByCategory,
}

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Invalid,
}

impl Direction {
    pub fn is_valid(&self) -> bool {
        !matches!(self, Direction::Invalid)
    }
}
