//! Network Analysis System
//!
//! This module provides network analysis capabilities for graph metrics and analysis.

use super::{CentralityMeasures, GraphEdge, GraphNode, NetworkMetrics, VisualizationMetrics};
use std::collections::{HashMap, HashSet};

/// Network analysis system for graph metrics and analysis
#[derive(Debug, Clone)]
pub struct NetworkAnalyzer {
    pub metrics: HashMap<String, f64>,
    pub is_analyzed: bool,
    pub centrality_cache: HashMap<String, CentralityMeasures>,
    pub path_cache: HashMap<(String, String), Vec<String>>,
}

impl NetworkAnalyzer {
    /// Create a new network analyzer
    pub fn new() -> Self {
        Self {
            metrics: HashMap::new(),
            is_analyzed: false,
            centrality_cache: HashMap::new(),
            path_cache: HashMap::new(),
        }
    }

    /// Analyze the network and calculate metrics
    pub fn analyze_network(&mut self, nodes: &[GraphNode], edges: &[GraphEdge]) {
        self.metrics.clear();
        self.centrality_cache.clear();
        self.path_cache.clear();

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

        // Calculate clustering coefficient
        let clustering_coefficient = self.calculate_clustering_coefficient(nodes, edges);
        self.metrics
            .insert("clustering_coefficient".to_string(), clustering_coefficient);

        // Calculate average path length
        let avg_path_length = self.calculate_average_path_length(nodes, edges);
        self.metrics
            .insert("average_path_length".to_string(), avg_path_length);

        self.is_analyzed = true;
    }

    /// Get centrality measures for all nodes
    pub fn get_centrality_measures(&self) -> &HashMap<String, CentralityMeasures> {
        &self.centrality_cache
    }

    /// Calculate centrality measures for a specific node
    pub fn calculate_node_centrality(
        &mut self,
        node_id: &str,
        nodes: &[GraphNode],
        edges: &[GraphEdge],
    ) -> CentralityMeasures {
        let degree_centrality = self.calculate_degree_centrality(node_id, edges);
        let betweenness_centrality = self.calculate_betweenness_centrality(node_id, nodes, edges);
        let closeness_centrality = self.calculate_closeness_centrality(node_id, nodes, edges);

        let measures = CentralityMeasures {
            degree_centrality,
            betweenness_centrality,
            closeness_centrality,
        };

        self.centrality_cache
            .insert(node_id.to_string(), measures.clone());
        measures
    }

    /// Calculate degree centrality
    fn calculate_degree_centrality(&self, node_id: &str, edges: &[GraphEdge]) -> f64 {
        let degree = edges
            .iter()
            .filter(|e| e.source == node_id || e.target == node_id)
            .count();

        if edges.is_empty() {
            0.0
        } else {
            degree as f64 / edges.len() as f64
        }
    }

    /// Calculate betweenness centrality
    fn calculate_betweenness_centrality(
        &mut self,
        node_id: &str,
        nodes: &[GraphNode],
        edges: &[GraphEdge],
    ) -> f64 {
        let mut betweenness = 0.0;

        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                if let Some(path) = self.shortest_path(&nodes[i].id, &nodes[j].id, nodes, edges) {
                    if path.contains(&node_id.to_string()) && path.len() > 2 {
                        betweenness += 1.0;
                    }
                }
            }
        }

        let total_pairs = nodes.len() * (nodes.len() - 1) / 2;
        if total_pairs > 0 {
            betweenness / total_pairs as f64
        } else {
            0.0
        }
    }

    /// Calculate closeness centrality
    fn calculate_closeness_centrality(
        &mut self,
        node_id: &str,
        nodes: &[GraphNode],
        edges: &[GraphEdge],
    ) -> f64 {
        let mut total_distance = 0.0;
        let mut reachable_nodes = 0;

        for node in nodes {
            if node.id != node_id {
                if let Some(distance) = self.path_length(node_id, &node.id, nodes, edges) {
                    total_distance += distance;
                    reachable_nodes += 1;
                }
            }
        }

        if reachable_nodes > 0 {
            reachable_nodes as f64 / total_distance
        } else {
            0.0
        }
    }

    /// Find shortest path between two nodes
    pub fn shortest_path(
        &mut self,
        source: &str,
        target: &str,
        _nodes: &[GraphNode],
        edges: &[GraphEdge],
    ) -> Option<Vec<String>> {
        let cache_key = (source.to_string(), target.to_string());

        if let Some(cached_path) = self.path_cache.get(&cache_key) {
            return Some(cached_path.clone());
        }

        // Use BFS to find shortest path
        let mut queue = vec![(source.to_string(), vec![source.to_string()])];
        let mut visited = HashSet::new();

        while let Some((current, path)) = queue.pop() {
            if current == target {
                self.path_cache.insert(cache_key, path.clone());
                return Some(path);
            }

            if visited.insert(current.clone()) {
                for edge in edges {
                    let next = if edge.source == current {
                        &edge.target
                    } else if edge.target == current {
                        &edge.source
                    } else {
                        continue;
                    };

                    if !visited.contains(next) {
                        let mut new_path = path.clone();
                        new_path.push(next.clone());
                        queue.insert(0, (next.clone(), new_path));
                    }
                }
            }
        }

        None
    }

    /// Calculate path length between two nodes
    pub fn path_length(
        &mut self,
        source: &str,
        target: &str,
        nodes: &[GraphNode],
        edges: &[GraphEdge],
    ) -> Option<f64> {
        if let Some(path) = self.shortest_path(source, target, nodes, edges) {
            Some((path.len() - 1) as f64)
        } else {
            None
        }
    }

    /// Get network-level metrics
    pub fn get_network_metrics(&self) -> NetworkMetrics {
        NetworkMetrics {
            node_count: *self.metrics.get("node_count").unwrap_or(&0.0) as usize,
            edge_count: *self.metrics.get("edge_count").unwrap_or(&0.0) as usize,
            density: *self.metrics.get("density").unwrap_or(&0.0),
            clustering_coefficient: *self.metrics.get("clustering_coefficient").unwrap_or(&0.0),
            average_path_length: *self.metrics.get("average_path_length").unwrap_or(&0.0),
        }
    }

    /// Get visualization-specific metrics
    pub fn get_visualization_metrics(
        &self,
        nodes: &[GraphNode],
        edges: &[GraphEdge],
    ) -> VisualizationMetrics {
        let edge_crossings = self.count_edge_crossings(edges);
        let node_overlaps = self.count_node_overlaps(nodes);
        let layout_quality = self.calculate_layout_quality(nodes, edges);

        VisualizationMetrics {
            edge_crossings,
            node_overlaps,
            layout_quality,
        }
    }

    /// Calculate clustering coefficient
    fn calculate_clustering_coefficient(&self, nodes: &[GraphNode], edges: &[GraphEdge]) -> f64 {
        let mut total_coefficient = 0.0;
        let mut valid_nodes = 0;

        for node in nodes {
            let neighbors = self.get_neighbors(&node.id, edges);
            if neighbors.len() >= 2 {
                let mut connections = 0;
                for i in 0..neighbors.len() {
                    for j in (i + 1)..neighbors.len() {
                        if edges.iter().any(|e| {
                            (e.source == neighbors[i] && e.target == neighbors[j])
                                || (e.source == neighbors[j] && e.target == neighbors[i])
                        }) {
                            connections += 1;
                        }
                    }
                }
                let possible_connections = neighbors.len() * (neighbors.len() - 1) / 2;
                total_coefficient += connections as f64 / possible_connections as f64;
                valid_nodes += 1;
            }
        }

        if valid_nodes > 0 {
            total_coefficient / valid_nodes as f64
        } else {
            0.0
        }
    }

    /// Calculate average path length
    fn calculate_average_path_length(&mut self, nodes: &[GraphNode], edges: &[GraphEdge]) -> f64 {
        let mut total_length = 0.0;
        let mut path_count = 0;

        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                if let Some(length) = self.path_length(&nodes[i].id, &nodes[j].id, nodes, edges) {
                    total_length += length;
                    path_count += 1;
                }
            }
        }

        if path_count > 0 {
            total_length / path_count as f64
        } else {
            0.0
        }
    }

    /// Get neighbors of a node
    fn get_neighbors(&self, node_id: &str, edges: &[GraphEdge]) -> Vec<String> {
        let mut neighbors = Vec::new();

        for edge in edges {
            if edge.source == node_id {
                neighbors.push(edge.target.clone());
            } else if edge.target == node_id {
                neighbors.push(edge.source.clone());
            }
        }

        neighbors
    }

    /// Count edge crossings
    fn count_edge_crossings(&self, edges: &[GraphEdge]) -> usize {
        let mut crossings = 0;

        for i in 0..edges.len() {
            for _j in (i + 1)..edges.len() {
                // Simplified edge crossing detection
                // In practice, this would check if line segments intersect
                crossings += 1; // Placeholder
            }
        }

        crossings
    }

    /// Count node overlaps
    fn count_node_overlaps(&self, nodes: &[GraphNode]) -> usize {
        let mut overlaps = 0;
        let threshold = 20.0; // Minimum distance between nodes

        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let dx = nodes[i].x - nodes[j].x;
                let dy = nodes[i].y - nodes[j].y;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance < threshold {
                    overlaps += 1;
                }
            }
        }

        overlaps
    }

    /// Calculate layout quality score
    fn calculate_layout_quality(&self, nodes: &[GraphNode], edges: &[GraphEdge]) -> f64 {
        let edge_crossings = self.count_edge_crossings(edges);
        let node_overlaps = self.count_node_overlaps(nodes);

        // Quality score based on minimizing crossings and overlaps
        let max_crossings = edges.len() * (edges.len() - 1) / 2;
        let max_overlaps = nodes.len() * (nodes.len() - 1) / 2;

        let crossing_score = if max_crossings > 0 {
            1.0 - (edge_crossings as f64 / max_crossings as f64)
        } else {
            1.0
        };
        let overlap_score = if max_overlaps > 0 {
            1.0 - (node_overlaps as f64 / max_overlaps as f64)
        } else {
            1.0
        };

        (crossing_score + overlap_score) / 2.0
    }
}
