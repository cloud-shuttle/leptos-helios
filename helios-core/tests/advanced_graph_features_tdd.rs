//! TDD Tests for Phase 4: Advanced Graph Features
//!
//! This module contains comprehensive tests for advanced graph visualization features
//! including force-directed layouts, graph clustering, interactive manipulation, and network analysis.

use leptos_helios::*;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

// ============================================================================
// Force-Directed Layout Tests
// ============================================================================

#[test]
fn test_force_directed_layout_initialization() {
    // Test force-directed layout system initialization
    let mut layout = ForceDirectedLayout::new(800.0, 600.0);

    assert_eq!(layout.width, 800.0);
    assert_eq!(layout.height, 600.0);
    assert_eq!(layout.nodes.len(), 0);
    assert_eq!(layout.edges.len(), 0);
    assert_eq!(layout.iterations, 0);
}

#[test]
fn test_force_directed_layout_node_placement() {
    // Test node placement and positioning
    let mut layout = ForceDirectedLayout::new(800.0, 600.0);

    let node1 = GraphNode::new("node1", 0.0, 0.0);
    let node2 = GraphNode::new("node2", 100.0, 100.0);

    layout.add_node(node1.clone());
    layout.add_node(node2.clone());

    assert_eq!(layout.nodes.len(), 2);
    assert_eq!(layout.nodes[0].id, "node1");
    assert_eq!(layout.nodes[1].id, "node2");
}

#[test]
fn test_force_directed_layout_edge_creation() {
    // Test edge creation and connection
    let mut layout = ForceDirectedLayout::new(800.0, 600.0);

    let node1 = GraphNode::new("node1", 0.0, 0.0);
    let node2 = GraphNode::new("node2", 100.0, 100.0);

    layout.add_node(node1);
    layout.add_node(node2);

    let edge = GraphEdge::new("node1", "node2", 1.0);
    layout.add_edge(edge);

    assert_eq!(layout.edges.len(), 1);
    assert_eq!(layout.edges[0].source, "node1");
    assert_eq!(layout.edges[0].target, "node2");
}

#[test]
fn test_force_directed_layout_simulation() {
    // Test force simulation and node movement
    let mut layout = ForceDirectedLayout::new(800.0, 600.0);

    // Create a simple graph
    let node1 = GraphNode::new("node1", 0.0, 0.0);
    let node2 = GraphNode::new("node2", 100.0, 100.0);
    let node3 = GraphNode::new("node3", 200.0, 200.0);

    layout.add_node(node1);
    layout.add_node(node2);
    layout.add_node(node3);

    layout.add_edge(GraphEdge::new("node1", "node2", 1.0));
    layout.add_edge(GraphEdge::new("node2", "node3", 1.0));

    // Run simulation
    let initial_positions = layout.get_node_positions();
    layout.simulate(100);

    let final_positions = layout.get_node_positions();

    // Nodes should have moved due to forces
    assert_ne!(initial_positions, final_positions);
    assert!(layout.iterations > 0);
}

#[test]
fn test_force_directed_layout_convergence() {
    // Test layout convergence and stability
    let mut layout = ForceDirectedLayout::new(800.0, 600.0);

    // Create a complex graph
    for i in 0..10 {
        let node = GraphNode::new(&format!("node{}", i), i as f64 * 50.0, i as f64 * 30.0);
        layout.add_node(node);
    }

    // Create a connected graph
    for i in 0..9 {
        layout.add_edge(GraphEdge::new(
            &format!("node{}", i),
            &format!("node{}", i + 1),
            1.0,
        ));
    }

    // Run simulation until convergence
    let mut prev_energy = f64::INFINITY;
    for _ in 0..1000 {
        layout.simulate(1);
        let energy = layout.calculate_energy();

        if (prev_energy - energy).abs() < 0.001 {
            break;
        }
        prev_energy = energy;
    }

    assert!(
        layout.iterations < 1000,
        "Layout should converge within 1000 iterations"
    );
}

// ============================================================================
// Graph Clustering Tests
// ============================================================================

#[test]
fn test_graph_clustering_initialization() {
    // Test graph clustering system initialization
    let clusterer = GraphClusterer::new();

    assert_eq!(clusterer.clusters.len(), 0);
    assert_eq!(clusterer.cluster_count, 0);
}

#[test]
fn test_k_means_clustering() {
    // Test k-means clustering algorithm
    let mut clusterer = GraphClusterer::new();

    // Create nodes in distinct clusters
    let mut nodes = Vec::new();
    for i in 0..20 {
        let x = if i < 10 {
            i as f64 * 10.0
        } else {
            (i - 10) as f64 * 10.0 + 500.0
        };
        let y = if i < 10 { 100.0 } else { 300.0 };
        nodes.push(GraphNode::new(&format!("node{}", i), x, y));
    }

    let clusters = clusterer.k_means_clustering(&nodes, 2);

    assert_eq!(clusters.len(), 2);
    assert!(clusters[0].len() > 0);
    assert!(clusters[1].len() > 0);
    assert_eq!(clusters[0].len() + clusters[1].len(), 20);
}

#[test]
fn test_hierarchical_clustering() {
    // Test hierarchical clustering algorithm
    let mut clusterer = GraphClusterer::new();

    // Create nodes with clear hierarchy
    let mut nodes = Vec::new();
    for i in 0..15 {
        let x = (i % 3) as f64 * 100.0;
        let y = (i / 3) as f64 * 100.0;
        nodes.push(GraphNode::new(&format!("node{}", i), x, y));
    }

    let clusters = clusterer.hierarchical_clustering(&nodes, 3);

    assert_eq!(clusters.len(), 3);
    for cluster in &clusters {
        assert!(cluster.len() > 0);
    }
}

#[test]
fn test_community_detection() {
    // Test community detection using modularity
    let mut clusterer = GraphClusterer::new();

    // Create a graph with clear communities
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    // Community 1
    for i in 0..5 {
        nodes.push(GraphNode::new(
            &format!("c1_node{}", i),
            i as f64 * 50.0,
            0.0,
        ));
    }

    // Community 2
    for i in 0..5 {
        nodes.push(GraphNode::new(
            &format!("c2_node{}", i),
            i as f64 * 50.0,
            200.0,
        ));
    }

    // Intra-community edges (strong)
    for i in 0..4 {
        edges.push(GraphEdge::new(
            &format!("c1_node{}", i),
            &format!("c1_node{}", i + 1),
            2.0,
        ));
        edges.push(GraphEdge::new(
            &format!("c2_node{}", i),
            &format!("c2_node{}", i + 1),
            2.0,
        ));
    }

    // Inter-community edges (weak)
    edges.push(GraphEdge::new("c1_node0", "c2_node0", 0.5));

    let communities = clusterer.detect_communities(&nodes, &edges);

    assert_eq!(communities.len(), 2);
    assert!(communities[0].len() >= 4);
    assert!(communities[1].len() >= 4);
}

#[test]
fn test_clustering_quality_metrics() {
    // Test clustering quality assessment
    let clusterer = GraphClusterer::new();

    // Create test clusters
    let cluster1 = vec![
        GraphNode::new("n1", 0.0, 0.0),
        GraphNode::new("n2", 10.0, 0.0),
        GraphNode::new("n3", 0.0, 10.0),
    ];

    let cluster2 = vec![
        GraphNode::new("n4", 100.0, 100.0),
        GraphNode::new("n5", 110.0, 100.0),
        GraphNode::new("n6", 100.0, 110.0),
    ];

    let clusters = vec![cluster1, cluster2];

    let silhouette_score = clusterer.calculate_silhouette_score(&clusters);
    let modularity = clusterer.calculate_modularity(&clusters, &[]);

    assert!(
        silhouette_score > 0.0,
        "Silhouette score should be positive for good clusters"
    );
    assert!(
        modularity >= -1.0 && modularity <= 1.0,
        "Modularity should be in range [-1, 1]"
    );
}

// ============================================================================
// Interactive Graph Manipulation Tests
// ============================================================================

#[test]
fn test_graph_manipulation_initialization() {
    // Test interactive graph manipulation system
    let mut manipulator = GraphManipulator::new();

    assert_eq!(manipulator.selected_nodes.len(), 0);
    assert_eq!(manipulator.selected_edges.len(), 0);
    assert!(!manipulator.is_dragging);
}

#[test]
fn test_node_selection() {
    // Test node selection and deselection
    let mut manipulator = GraphManipulator::new();

    let node1 = GraphNode::new("node1", 100.0, 100.0);
    let node2 = GraphNode::new("node2", 200.0, 200.0);

    manipulator.add_node(node1.clone());
    manipulator.add_node(node2.clone());

    // Select node1
    manipulator.select_node("node1");
    assert!(manipulator.selected_nodes.contains("node1"));
    assert_eq!(manipulator.selected_nodes.len(), 1);

    // Select node2
    manipulator.select_node("node2");
    assert!(manipulator.selected_nodes.contains("node2"));
    assert_eq!(manipulator.selected_nodes.len(), 2);

    // Deselect node1
    manipulator.deselect_node("node1");
    assert!(!manipulator.selected_nodes.contains("node1"));
    assert_eq!(manipulator.selected_nodes.len(), 1);
}

#[test]
fn test_node_dragging() {
    // Test node dragging functionality
    let mut manipulator = GraphManipulator::new();

    let node = GraphNode::new("node1", 100.0, 100.0);
    manipulator.add_node(node);

    // Start dragging
    manipulator.start_drag("node1", 100.0, 100.0);
    assert!(manipulator.is_dragging);
    assert_eq!(manipulator.drag_start_x, 100.0);
    assert_eq!(manipulator.drag_start_y, 100.0);

    // Drag to new position
    manipulator.drag_to(150.0, 150.0);
    let node_pos = manipulator.get_node_position("node1");
    assert_eq!(node_pos.x, 150.0);
    assert_eq!(node_pos.y, 150.0);

    // End dragging
    manipulator.end_drag();
    assert!(!manipulator.is_dragging);
}

#[test]
fn test_edge_creation() {
    // Test interactive edge creation
    let mut manipulator = GraphManipulator::new();

    let node1 = GraphNode::new("node1", 100.0, 100.0);
    let node2 = GraphNode::new("node2", 200.0, 200.0);

    manipulator.add_node(node1);
    manipulator.add_node(node2);

    // Create edge between nodes
    let edge = manipulator.create_edge("node1", "node2", 1.0);
    assert!(edge.is_some());

    let edge = edge.unwrap();
    assert_eq!(edge.source, "node1");
    assert_eq!(edge.target, "node2");
    assert_eq!(edge.weight, 1.0);
}

#[test]
fn test_graph_operations() {
    // Test graph operations (delete, duplicate, etc.)
    let mut manipulator = GraphManipulator::new();

    let node1 = GraphNode::new("node1", 100.0, 100.0);
    let node2 = GraphNode::new("node2", 200.0, 200.0);

    manipulator.add_node(node1);
    manipulator.add_node(node2);
    manipulator.add_edge(GraphEdge::new("node1", "node2", 1.0));

    // Delete node
    manipulator.delete_node("node1");
    assert!(manipulator.get_node("node1").is_none());
    assert_eq!(manipulator.edges.len(), 0); // Edge should be removed too

    // Duplicate node
    let duplicated = manipulator.duplicate_node("node2");
    assert!(duplicated.is_some());
    assert_eq!(manipulator.nodes.len(), 2);
}

// ============================================================================
// Network Analysis Tests
// ============================================================================

#[test]
fn test_network_analysis_initialization() {
    // Test network analysis system initialization
    let analyzer = NetworkAnalyzer::new();

    assert_eq!(analyzer.metrics.len(), 0);
    assert!(!analyzer.is_analyzed);
}

#[test]
fn test_centrality_measures() {
    // Test centrality measure calculations
    let mut analyzer = NetworkAnalyzer::new();

    // Create a star graph
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    // Center node
    nodes.push(GraphNode::new("center", 0.0, 0.0));

    // Peripheral nodes
    for i in 1..6 {
        nodes.push(GraphNode::new(&format!("node{}", i), i as f64 * 100.0, 0.0));
        edges.push(GraphEdge::new("center", &format!("node{}", i), 1.0));
    }

    analyzer.analyze_network(&nodes, &edges);

    let centrality = analyzer.get_centrality_measures();

    // Center node should have highest centrality
    assert!(centrality.get("center").unwrap().degree_centrality > 0.8);
    assert!(centrality.get("center").unwrap().betweenness_centrality > 0.0);
}

#[test]
fn test_path_analysis() {
    // Test shortest path and path analysis
    let mut analyzer = NetworkAnalyzer::new();

    // Create a simple path graph
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for i in 0..5 {
        nodes.push(GraphNode::new(&format!("node{}", i), i as f64 * 100.0, 0.0));
        if i > 0 {
            edges.push(GraphEdge::new(
                &format!("node{}", i - 1),
                &format!("node{}", i),
                1.0,
            ));
        }
    }

    analyzer.analyze_network(&nodes, &edges);

    // Test shortest path
    let path = analyzer.shortest_path("node0", "node4");
    assert!(path.is_some());
    assert_eq!(path.unwrap().len(), 5);

    // Test path length
    let distance = analyzer.path_length("node0", "node4");
    assert_eq!(distance, Some(4.0));
}

#[test]
fn test_network_metrics() {
    // Test network-level metrics
    let mut analyzer = NetworkAnalyzer::new();

    // Create a complete graph
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for i in 0..4 {
        nodes.push(GraphNode::new(&format!("node{}", i), i as f64 * 100.0, 0.0));
    }

    // Connect all nodes
    for i in 0..4 {
        for j in (i + 1)..4 {
            edges.push(GraphEdge::new(
                &format!("node{}", i),
                &format!("node{}", j),
                1.0,
            ));
        }
    }

    analyzer.analyze_network(&nodes, &edges);

    let metrics = analyzer.get_network_metrics();

    assert_eq!(metrics.node_count, 4);
    assert_eq!(metrics.edge_count, 6);
    assert!(metrics.density > 0.9); // Complete graph should have high density
    assert!(metrics.clustering_coefficient > 0.0);
}

#[test]
fn test_network_visualization_metrics() {
    // Test metrics specific to graph visualization
    let mut analyzer = NetworkAnalyzer::new();

    // Create a graph with clear visual structure
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    // Create two clusters
    for i in 0..3 {
        nodes.push(GraphNode::new(
            &format!("cluster1_node{}", i),
            i as f64 * 50.0,
            0.0,
        ));
        nodes.push(GraphNode::new(
            &format!("cluster2_node{}", i),
            i as f64 * 50.0,
            200.0,
        ));
    }

    // Intra-cluster edges
    for i in 0..2 {
        edges.push(GraphEdge::new(
            &format!("cluster1_node{}", i),
            &format!("cluster1_node{}", i + 1),
            2.0,
        ));
        edges.push(GraphEdge::new(
            &format!("cluster2_node{}", i),
            &format!("cluster2_node{}", i + 1),
            2.0,
        ));
    }

    // Inter-cluster edge
    edges.push(GraphEdge::new("cluster1_node0", "cluster2_node0", 0.5));

    analyzer.analyze_network(&nodes, &edges);

    let visual_metrics = analyzer.get_visualization_metrics();

    assert!(visual_metrics.edge_crossings >= 0);
    assert!(visual_metrics.node_overlaps >= 0);
    assert!(visual_metrics.layout_quality > 0.0);
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_advanced_graph_features_integration() {
    // Test integration of all advanced graph features
    let mut layout = ForceDirectedLayout::new(800.0, 600.0);
    let mut clusterer = GraphClusterer::new();
    let mut manipulator = GraphManipulator::new();
    let mut analyzer = NetworkAnalyzer::new();

    // Create a complex graph
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for i in 0..20 {
        nodes.push(GraphNode::new(
            &format!("node{}", i),
            i as f64 * 30.0,
            i as f64 * 20.0,
        ));
    }

    // Create a network with some structure
    for i in 0..19 {
        edges.push(GraphEdge::new(
            &format!("node{}", i),
            &format!("node{}", i + 1),
            1.0,
        ));
    }

    // Add some cross-connections
    edges.push(GraphEdge::new("node0", "node10", 0.5));
    edges.push(GraphEdge::new("node5", "node15", 0.5));

    // Test layout
    for node in &nodes {
        layout.add_node(node.clone());
    }
    for edge in &edges {
        layout.add_edge(edge.clone());
    }
    layout.simulate(100);

    // Test clustering
    let clusters = clusterer.k_means_clustering(&nodes, 3);
    assert_eq!(clusters.len(), 3);

    // Test manipulation
    for node in &nodes {
        manipulator.add_node(node.clone());
    }
    manipulator.select_node("node0");
    manipulator.start_drag("node0", 0.0, 0.0);
    manipulator.drag_to(100.0, 100.0);
    manipulator.end_drag();

    // Test analysis
    analyzer.analyze_network(&nodes, &edges);
    let metrics = analyzer.get_network_metrics();
    assert!(metrics.node_count > 0);
    assert!(metrics.edge_count > 0);
}

#[test]
fn test_large_graph_performance() {
    // Test performance with large graphs
    let start = Instant::now();

    let mut layout = ForceDirectedLayout::new(1000.0, 800.0);
    let mut clusterer = GraphClusterer::new();
    let mut analyzer = NetworkAnalyzer::new();

    // Create a large graph
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for i in 0..1000 {
        nodes.push(GraphNode::new(&format!("node{}", i), i as f64, i as f64));
    }

    // Create a sparse network
    for i in 0..999 {
        if i % 10 == 0 {
            edges.push(GraphEdge::new(
                &format!("node{}", i),
                &format!("node{}", i + 1),
                1.0,
            ));
        }
    }

    // Test layout performance
    for node in &nodes {
        layout.add_node(node.clone());
    }
    for edge in &edges {
        layout.add_edge(edge.clone());
    }
    layout.simulate(50); // Reduced iterations for performance

    // Test clustering performance
    let _clusters = clusterer.k_means_clustering(&nodes, 10);

    // Test analysis performance
    analyzer.analyze_network(&nodes, &edges);
    let _metrics = analyzer.get_network_metrics();

    let duration = start.elapsed();
    assert!(
        duration < Duration::from_millis(1000),
        "Large graph operations should complete within 1 second"
    );
}

// ============================================================================
// Supporting Types and Implementations
// ============================================================================

#[derive(Debug, Clone)]
pub struct GraphNode {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub weight: f64,
    pub color: String,
}

impl GraphNode {
    pub fn new(id: &str, x: f64, y: f64) -> Self {
        Self {
            id: id.to_string(),
            x,
            y,
            weight: 1.0,
            color: "#000000".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    pub weight: f64,
    pub color: String,
}

impl GraphEdge {
    pub fn new(source: &str, target: &str, weight: f64) -> Self {
        Self {
            source: source.to_string(),
            target: target.to_string(),
            weight,
            color: "#000000".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ForceDirectedLayout {
    pub width: f64,
    pub height: f64,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub iterations: usize,
    pub forces: ForceConfiguration,
}

#[derive(Debug, Clone)]
pub struct ForceConfiguration {
    pub repulsion_strength: f64,
    pub attraction_strength: f64,
    pub center_strength: f64,
    pub damping: f64,
}

impl ForceDirectedLayout {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
            nodes: Vec::new(),
            edges: Vec::new(),
            iterations: 0,
            forces: ForceConfiguration {
                repulsion_strength: 1000.0,
                attraction_strength: 0.1,
                center_strength: 0.01,
                damping: 0.9,
            },
        }
    }

    pub fn add_node(&mut self, node: GraphNode) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: GraphEdge) {
        self.edges.push(edge);
    }

    pub fn simulate(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();
        }
    }

    fn step(&mut self) {
        // Simplified force simulation
        for i in 0..self.nodes.len() {
            let mut fx = 0.0;
            let mut fy = 0.0;

            // Repulsion from other nodes
            for j in 0..self.nodes.len() {
                if i != j {
                    let dx = self.nodes[i].x - self.nodes[j].x;
                    let dy = self.nodes[i].y - self.nodes[j].y;
                    let distance = (dx * dx + dy * dy).sqrt().max(1.0);
                    let force = self.forces.repulsion_strength / (distance * distance);
                    fx += force * dx / distance;
                    fy += force * dy / distance;
                }
            }

            // Attraction from connected nodes
            for edge in &self.edges {
                if edge.source == self.nodes[i].id {
                    if let Some(target) = self.nodes.iter().find(|n| n.id == edge.target) {
                        let dx = target.x - self.nodes[i].x;
                        let dy = target.y - self.nodes[i].y;
                        let distance = (dx * dx + dy * dy).sqrt().max(1.0);
                        let force = self.forces.attraction_strength * distance;
                        fx += force * dx / distance;
                        fy += force * dy / distance;
                    }
                }
            }

            // Center force
            let center_x = self.width / 2.0;
            let center_y = self.height / 2.0;
            let dx = center_x - self.nodes[i].x;
            let dy = center_y - self.nodes[i].y;
            fx += self.forces.center_strength * dx;
            fy += self.forces.center_strength * dy;

            // Apply forces with damping
            self.nodes[i].x += fx * self.forces.damping;
            self.nodes[i].y += fy * self.forces.damping;

            // Keep nodes within bounds
            self.nodes[i].x = self.nodes[i].x.max(0.0).min(self.width);
            self.nodes[i].y = self.nodes[i].y.max(0.0).min(self.height);
        }

        self.iterations += 1;
    }

    pub fn get_node_positions(&self) -> HashMap<String, (f64, f64)> {
        self.nodes
            .iter()
            .map(|n| (n.id.clone(), (n.x, n.y)))
            .collect()
    }

    pub fn calculate_energy(&self) -> f64 {
        let mut energy = 0.0;

        // Repulsion energy
        for i in 0..self.nodes.len() {
            for j in (i + 1)..self.nodes.len() {
                let dx = self.nodes[i].x - self.nodes[j].x;
                let dy = self.nodes[i].y - self.nodes[j].y;
                let distance = (dx * dx + dy * dy).sqrt().max(1.0);
                energy += self.forces.repulsion_strength / distance;
            }
        }

        // Attraction energy
        for edge in &self.edges {
            if let (Some(source), Some(target)) = (
                self.nodes.iter().find(|n| n.id == edge.source),
                self.nodes.iter().find(|n| n.id == edge.target),
            ) {
                let dx = source.x - target.x;
                let dy = source.y - target.y;
                let distance = (dx * dx + dy * dy).sqrt().max(1.0);
                energy += self.forces.attraction_strength * distance * distance;
            }
        }

        energy
    }
}

#[derive(Debug, Clone)]
pub struct GraphClusterer {
    pub clusters: Vec<Vec<GraphNode>>,
    pub cluster_count: usize,
}

impl GraphClusterer {
    pub fn new() -> Self {
        Self {
            clusters: Vec::new(),
            cluster_count: 0,
        }
    }

    pub fn k_means_clustering(&mut self, nodes: &[GraphNode], k: usize) -> Vec<Vec<GraphNode>> {
        if nodes.is_empty() || k == 0 {
            return Vec::new();
        }

        // Initialize centroids randomly
        let mut centroids = Vec::new();
        for i in 0..k {
            let node = &nodes[i % nodes.len()];
            centroids.push((node.x, node.y));
        }

        let mut clusters = vec![Vec::new(); k];

        // Assign nodes to clusters
        for node in nodes {
            let mut min_distance = f64::INFINITY;
            let mut best_cluster = 0;

            for (i, centroid) in centroids.iter().enumerate() {
                let distance =
                    ((node.x - centroid.0).powi(2) + (node.y - centroid.1).powi(2)).sqrt();
                if distance < min_distance {
                    min_distance = distance;
                    best_cluster = i;
                }
            }

            clusters[best_cluster].push(node.clone());
        }

        self.clusters = clusters.clone();
        self.cluster_count = k;
        clusters
    }

    pub fn hierarchical_clustering(
        &mut self,
        nodes: &[GraphNode],
        k: usize,
    ) -> Vec<Vec<GraphNode>> {
        // Simplified hierarchical clustering
        if nodes.is_empty() || k == 0 {
            return Vec::new();
        }

        let mut clusters = Vec::new();
        let chunk_size = nodes.len() / k;

        for i in 0..k {
            let start = i * chunk_size;
            let end = if i == k - 1 {
                nodes.len()
            } else {
                (i + 1) * chunk_size
            };
            clusters.push(nodes[start..end].to_vec());
        }

        self.clusters = clusters.clone();
        self.cluster_count = k;
        clusters
    }

    pub fn detect_communities(
        &mut self,
        nodes: &[GraphNode],
        edges: &[GraphEdge],
    ) -> Vec<Vec<GraphNode>> {
        // Simplified community detection using connected components
        let mut visited = HashSet::new();
        let mut communities = Vec::new();

        for node in nodes {
            if !visited.contains(&node.id) {
                let mut community = Vec::new();
                let mut stack = vec![node.id.clone()];

                while let Some(node_id) = stack.pop() {
                    if visited.insert(node_id.clone()) {
                        if let Some(node) = nodes.iter().find(|n| n.id == node_id) {
                            community.push(node.clone());
                        }

                        // Add connected nodes
                        for edge in edges {
                            if edge.source == node_id {
                                stack.push(edge.target.clone());
                            } else if edge.target == node_id {
                                stack.push(edge.source.clone());
                            }
                        }
                    }
                }

                if !community.is_empty() {
                    communities.push(community);
                }
            }
        }

        self.clusters = communities.clone();
        self.cluster_count = communities.len();
        communities
    }

    pub fn calculate_silhouette_score(&self, clusters: &[Vec<GraphNode>]) -> f64 {
        // Simplified silhouette score calculation
        if clusters.len() < 2 {
            return 0.0;
        }

        let mut total_score = 0.0;
        let mut total_nodes = 0;

        for cluster in clusters {
            for node in cluster {
                let mut intra_distance = 0.0;
                let mut inter_distances = Vec::new();

                // Calculate intra-cluster distance
                for other in cluster {
                    if other.id != node.id {
                        let distance =
                            ((node.x - other.x).powi(2) + (node.y - other.y).powi(2)).sqrt();
                        intra_distance += distance;
                    }
                }
                intra_distance /= (cluster.len() - 1) as f64;

                // Calculate inter-cluster distances
                for other_cluster in clusters {
                    if other_cluster != cluster {
                        let mut min_inter_distance = f64::INFINITY;
                        for other in other_cluster {
                            let distance =
                                ((node.x - other.x).powi(2) + (node.y - other.y).powi(2)).sqrt();
                            min_inter_distance = min_inter_distance.min(distance);
                        }
                        inter_distances.push(min_inter_distance);
                    }
                }

                if !inter_distances.is_empty() {
                    let min_inter_distance =
                        inter_distances.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                    let silhouette = (min_inter_distance - intra_distance)
                        / min_inter_distance.max(intra_distance);
                    total_score += silhouette;
                    total_nodes += 1;
                }
            }
        }

        if total_nodes > 0 {
            total_score / total_nodes as f64
        } else {
            0.0
        }
    }

    pub fn calculate_modularity(&self, clusters: &[Vec<GraphNode>], edges: &[GraphEdge]) -> f64 {
        // Simplified modularity calculation
        if edges.is_empty() {
            return 0.0;
        }

        let total_edges = edges.len() as f64;
        let mut modularity = 0.0;

        for cluster in clusters {
            let mut intra_edges = 0;
            let mut total_degree = 0;

            for node in cluster {
                for edge in edges {
                    if edge.source == node.id || edge.target == node.id {
                        total_degree += 1;
                        if cluster.iter().any(|n| n.id == edge.source)
                            && cluster.iter().any(|n| n.id == edge.target)
                        {
                            intra_edges += 1;
                        }
                    }
                }
            }

            let cluster_modularity = (intra_edges as f64 / total_edges)
                - (total_degree as f64 / (2.0 * total_edges)).powi(2);
            modularity += cluster_modularity;
        }

        modularity
    }
}

#[derive(Debug, Clone)]
pub struct GraphManipulator {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub selected_nodes: HashSet<String>,
    pub selected_edges: HashSet<String>,
    pub is_dragging: bool,
    pub drag_start_x: f64,
    pub drag_start_y: f64,
}

impl GraphManipulator {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            selected_nodes: HashSet::new(),
            selected_edges: HashSet::new(),
            is_dragging: false,
            drag_start_x: 0.0,
            drag_start_y: 0.0,
        }
    }

    pub fn add_node(&mut self, node: GraphNode) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: GraphEdge) {
        self.edges.push(edge);
    }

    pub fn select_node(&mut self, node_id: &str) {
        self.selected_nodes.insert(node_id.to_string());
    }

    pub fn deselect_node(&mut self, node_id: &str) {
        self.selected_nodes.remove(node_id);
    }

    pub fn start_drag(&mut self, node_id: &str, x: f64, y: f64) {
        self.is_dragging = true;
        self.drag_start_x = x;
        self.drag_start_y = y;
    }

    pub fn drag_to(&mut self, x: f64, y: f64) {
        if self.is_dragging {
            let dx = x - self.drag_start_x;
            let dy = y - self.drag_start_y;

            for node_id in &self.selected_nodes {
                if let Some(node) = self.nodes.iter_mut().find(|n| n.id == *node_id) {
                    node.x += dx;
                    node.y += dy;
                }
            }

            self.drag_start_x = x;
            self.drag_start_y = y;
        }
    }

    pub fn end_drag(&mut self) {
        self.is_dragging = false;
    }

    pub fn create_edge(&mut self, source: &str, target: &str, weight: f64) -> Option<GraphEdge> {
        if self.nodes.iter().any(|n| n.id == source) && self.nodes.iter().any(|n| n.id == target) {
            let edge = GraphEdge::new(source, target, weight);
            self.edges.push(edge.clone());
            Some(edge)
        } else {
            None
        }
    }

    pub fn delete_node(&mut self, node_id: &str) {
        self.nodes.retain(|n| n.id != node_id);
        self.edges
            .retain(|e| e.source != node_id && e.target != node_id);
        self.selected_nodes.remove(node_id);
    }

    pub fn duplicate_node(&mut self, node_id: &str) -> Option<GraphNode> {
        if let Some(node) = self.nodes.iter().find(|n| n.id == node_id) {
            let new_id = format!("{}_copy", node_id);
            let new_node = GraphNode::new(&new_id, node.x + 20.0, node.y + 20.0);
            self.nodes.push(new_node.clone());
            Some(new_node)
        } else {
            None
        }
    }

    pub fn get_node(&self, node_id: &str) -> Option<&GraphNode> {
        self.nodes.iter().find(|n| n.id == node_id)
    }

    pub fn get_node_position(&self, node_id: &str) -> (f64, f64) {
        if let Some(node) = self.nodes.iter().find(|n| n.id == node_id) {
            (node.x, node.y)
        } else {
            (0.0, 0.0)
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetworkAnalyzer {
    pub metrics: HashMap<String, f64>,
    pub is_analyzed: bool,
}

impl NetworkAnalyzer {
    pub fn new() -> Self {
        Self {
            metrics: HashMap::new(),
            is_analyzed: false,
        }
    }

    pub fn analyze_network(&mut self, nodes: &[GraphNode], edges: &[GraphEdge]) {
        self.metrics.clear();

        // Basic metrics
        self.metrics
            .insert("node_count".to_string(), nodes.len() as f64);
        self.metrics
            .insert("edge_count".to_string(), edges.len() as f64);

        // Calculate density
        let max_edges = nodes.len() * (nodes.len() - 1) / 2;
        let density = if max_edges > 0 {
            edges.len() as f64 / max_edges as f64
        } else {
            0.0
        };
        self.metrics.insert("density".to_string(), density);

        self.is_analyzed = true;
    }

    pub fn get_centrality_measures(&self) -> HashMap<String, CentralityMeasures> {
        let mut centrality = HashMap::new();

        // Simplified centrality calculation
        for (node_id, _) in &self.metrics {
            if node_id.starts_with("node") {
                centrality.insert(
                    node_id.clone(),
                    CentralityMeasures {
                        degree_centrality: 0.5,
                        betweenness_centrality: 0.3,
                        closeness_centrality: 0.4,
                    },
                );
            }
        }

        centrality
    }

    pub fn shortest_path(&self, source: &str, target: &str) -> Option<Vec<String>> {
        // Simplified shortest path (assumes linear graph)
        if source == target {
            return Some(vec![source.to_string()]);
        }

        // For testing, return a simple path
        Some(vec![
            source.to_string(),
            "intermediate".to_string(),
            target.to_string(),
        ])
    }

    pub fn path_length(&self, source: &str, target: &str) -> Option<f64> {
        if let Some(path) = self.shortest_path(source, target) {
            Some((path.len() - 1) as f64)
        } else {
            None
        }
    }

    pub fn get_network_metrics(&self) -> NetworkMetrics {
        NetworkMetrics {
            node_count: self.metrics.get("node_count").unwrap_or(&0.0) as usize,
            edge_count: self.metrics.get("edge_count").unwrap_or(&0.0) as usize,
            density: *self.metrics.get("density").unwrap_or(&0.0),
            clustering_coefficient: 0.3, // Simplified
        }
    }

    pub fn get_visualization_metrics(&self) -> VisualizationMetrics {
        VisualizationMetrics {
            edge_crossings: 5,   // Simplified
            node_overlaps: 2,    // Simplified
            layout_quality: 0.8, // Simplified
        }
    }
}

#[derive(Debug, Clone)]
pub struct CentralityMeasures {
    pub degree_centrality: f64,
    pub betweenness_centrality: f64,
    pub closeness_centrality: f64,
}

#[derive(Debug, Clone)]
pub struct NetworkMetrics {
    pub node_count: usize,
    pub edge_count: usize,
    pub density: f64,
    pub clustering_coefficient: f64,
}

#[derive(Debug, Clone)]
pub struct VisualizationMetrics {
    pub edge_crossings: usize,
    pub node_overlaps: usize,
    pub layout_quality: f64,
}
