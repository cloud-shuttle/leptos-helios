//! Advanced Graph Features for leptos-helios
//!
//! This module provides comprehensive advanced graph visualization features including
//! force-directed layouts, graph clustering, interactive manipulation, and network analysis.

use std::collections::{HashMap, HashSet};

// ============================================================================
// Force-Directed Layout
// ============================================================================

/// Force-directed layout system for graph visualization
#[derive(Debug, Clone)]
pub struct ForceDirectedLayout {
    pub width: f64,
    pub height: f64,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub iterations: usize,
    pub forces: ForceConfiguration,
    pub velocities: HashMap<String, (f64, f64)>,
}

/// Configuration for force-directed layout forces
#[derive(Debug, Clone)]
pub struct ForceConfiguration {
    pub repulsion_strength: f64,
    pub attraction_strength: f64,
    pub center_strength: f64,
    pub damping: f64,
    pub max_velocity: f64,
    pub min_distance: f64,
}

impl ForceDirectedLayout {
    /// Create a new force-directed layout
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
                max_velocity: 10.0,
                min_distance: 1.0,
            },
            velocities: HashMap::new(),
        }
    }

    /// Add a node to the layout
    pub fn add_node(&mut self, node: GraphNode) {
        self.velocities.insert(node.id.clone(), (0.0, 0.0));
        self.nodes.push(node);
    }

    /// Add an edge to the layout
    pub fn add_edge(&mut self, edge: GraphEdge) {
        self.edges.push(edge);
    }

    /// Run the force simulation for a specified number of steps
    pub fn simulate(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();
        }
    }

    /// Perform one simulation step
    fn step(&mut self) {
        let mut forces = HashMap::new();

        // Calculate forces for each node
        for i in 0..self.nodes.len() {
            let mut fx = 0.0;
            let mut fy = 0.0;

            // Repulsion from other nodes
            for j in 0..self.nodes.len() {
                if i != j {
                    let dx = self.nodes[i].x - self.nodes[j].x;
                    let dy = self.nodes[i].y - self.nodes[j].y;
                    let distance = (dx * dx + dy * dy).sqrt().max(self.forces.min_distance);
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
                        let distance = (dx * dx + dy * dy).sqrt().max(self.forces.min_distance);
                        let force = self.forces.attraction_strength * distance * edge.weight;
                        fx += force * dx / distance;
                        fy += force * dy / distance;
                    }
                } else if edge.target == self.nodes[i].id {
                    if let Some(source) = self.nodes.iter().find(|n| n.id == edge.source) {
                        let dx = source.x - self.nodes[i].x;
                        let dy = source.y - self.nodes[i].y;
                        let distance = (dx * dx + dy * dy).sqrt().max(self.forces.min_distance);
                        let force = self.forces.attraction_strength * distance * edge.weight;
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

            forces.insert(self.nodes[i].id.clone(), (fx, fy));
        }

        // Apply forces and update positions
        for i in 0..self.nodes.len() {
            let node_id = &self.nodes[i].id;
            let (fx, fy) = forces.get(node_id).unwrap_or(&(0.0, 0.0));

            // Update velocity
            let (vx, vy) = self.velocities.get(node_id).unwrap_or(&(0.0, 0.0));
            let mut new_vx = (vx + fx) * self.forces.damping;
            let mut new_vy = (vy + fy) * self.forces.damping;

            // Limit velocity
            let velocity_magnitude = (new_vx * new_vx + new_vy * new_vy).sqrt();
            if velocity_magnitude > self.forces.max_velocity {
                new_vx = new_vx / velocity_magnitude * self.forces.max_velocity;
                new_vy = new_vy / velocity_magnitude * self.forces.max_velocity;
            }

            self.velocities.insert(node_id.clone(), (new_vx, new_vy));

            // Update position
            self.nodes[i].x += new_vx;
            self.nodes[i].y += new_vy;

            // Keep nodes within bounds
            self.nodes[i].x = self.nodes[i].x.max(0.0).min(self.width);
            self.nodes[i].y = self.nodes[i].y.max(0.0).min(self.height);
        }

        self.iterations += 1;
    }

    /// Get current node positions
    pub fn get_node_positions(&self) -> HashMap<String, (f64, f64)> {
        self.nodes
            .iter()
            .map(|n| (n.id.clone(), (n.x, n.y)))
            .collect()
    }

    /// Calculate the total energy of the system
    pub fn calculate_energy(&self) -> f64 {
        let mut energy = 0.0;

        // Repulsion energy
        for i in 0..self.nodes.len() {
            for j in (i + 1)..self.nodes.len() {
                let dx = self.nodes[i].x - self.nodes[j].x;
                let dy = self.nodes[i].y - self.nodes[j].y;
                let distance = (dx * dx + dy * dy).sqrt().max(self.forces.min_distance);
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
                let distance = (dx * dx + dy * dy).sqrt().max(self.forces.min_distance);
                energy += self.forces.attraction_strength * distance * distance * edge.weight;
            }
        }

        energy
    }

    /// Check if the layout has converged
    pub fn has_converged(&self, threshold: f64) -> bool {
        let energy = self.calculate_energy();
        energy < threshold
    }

    /// Reset the layout
    pub fn reset(&mut self) {
        self.iterations = 0;
        self.velocities.clear();
        for node in &mut self.nodes {
            node.x = self.width / 2.0 + (rand::random::<f64>() - 0.5) * self.width * 0.1;
            node.y = self.height / 2.0 + (rand::random::<f64>() - 0.5) * self.height * 0.1;
        }
    }
}

// ============================================================================
// Graph Clustering
// ============================================================================

/// Graph clustering system for community detection and analysis
#[derive(Debug, Clone)]
pub struct GraphClusterer {
    pub clusters: Vec<Vec<GraphNode>>,
    pub cluster_count: usize,
    pub algorithm: ClusteringAlgorithm,
}

/// Available clustering algorithms
#[derive(Debug, Clone, PartialEq)]
pub enum ClusteringAlgorithm {
    KMeans,
    Hierarchical,
    CommunityDetection,
    Spectral,
}

impl GraphClusterer {
    /// Create a new graph clusterer
    pub fn new() -> Self {
        Self {
            clusters: Vec::new(),
            cluster_count: 0,
            algorithm: ClusteringAlgorithm::KMeans,
        }
    }

    /// Perform k-means clustering
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
        let mut converged = false;
        let mut iterations = 0;
        let max_iterations = 100;

        while !converged && iterations < max_iterations {
            // Assign nodes to clusters
            clusters = vec![Vec::new(); k];
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

            // Update centroids
            let mut new_centroids = Vec::new();
            for cluster in &clusters {
                if cluster.is_empty() {
                    new_centroids.push((0.0, 0.0));
                } else {
                    let avg_x = cluster.iter().map(|n| n.x).sum::<f64>() / cluster.len() as f64;
                    let avg_y = cluster.iter().map(|n| n.y).sum::<f64>() / cluster.len() as f64;
                    new_centroids.push((avg_x, avg_y));
                }
            }

            // Check for convergence
            converged = true;
            for (old, new) in centroids.iter().zip(new_centroids.iter()) {
                let distance = ((old.0 - new.0).powi(2) + (old.1 - new.1).powi(2)).sqrt();
                if distance > 0.001 {
                    converged = false;
                    break;
                }
            }

            centroids = new_centroids;
            iterations += 1;
        }

        self.clusters = clusters.clone();
        self.cluster_count = k;
        self.algorithm = ClusteringAlgorithm::KMeans;
        clusters
    }

    /// Perform hierarchical clustering
    pub fn hierarchical_clustering(
        &mut self,
        nodes: &[GraphNode],
        k: usize,
    ) -> Vec<Vec<GraphNode>> {
        if nodes.is_empty() || k == 0 {
            return Vec::new();
        }

        // Start with each node as its own cluster
        let mut clusters: Vec<Vec<GraphNode>> = nodes.iter().map(|n| vec![n.clone()]).collect();

        // Merge clusters until we have k clusters
        while clusters.len() > k {
            let mut min_distance = f64::INFINITY;
            let mut best_pair = (0, 1);

            // Find the closest pair of clusters
            for i in 0..clusters.len() {
                for j in (i + 1)..clusters.len() {
                    let distance = self.calculate_cluster_distance(&clusters[i], &clusters[j]);
                    if distance < min_distance {
                        min_distance = distance;
                        best_pair = (i, j);
                    }
                }
            }

            // Merge the closest clusters
            let cluster2 = clusters.remove(best_pair.1);
            clusters[best_pair.0].extend(cluster2);
        }

        self.clusters = clusters.clone();
        self.cluster_count = k;
        self.algorithm = ClusteringAlgorithm::Hierarchical;
        clusters
    }

    /// Detect communities using modularity optimization
    pub fn detect_communities(
        &mut self,
        nodes: &[GraphNode],
        edges: &[GraphEdge],
    ) -> Vec<Vec<GraphNode>> {
        // Start with each node in its own community
        let mut communities: Vec<Vec<GraphNode>> = nodes.iter().map(|n| vec![n.clone()]).collect();
        let mut improved = true;

        while improved {
            improved = false;

            for node in nodes {
                let mut best_community = 0;
                let mut best_modularity_gain = 0.0;

                // Try moving the node to each community
                for (i, _community) in communities.iter().enumerate() {
                    let modularity_gain =
                        self.calculate_modularity_gain(node, &communities, edges, i);
                    if modularity_gain > best_modularity_gain {
                        best_modularity_gain = modularity_gain;
                        best_community = i;
                    }
                }

                // Move node to best community if it improves modularity
                if best_modularity_gain > 0.0 {
                    // Remove from current community
                    for community in &mut communities {
                        community.retain(|n| n.id != node.id);
                    }

                    // Add to best community
                    communities[best_community].push(node.clone());
                    improved = true;
                }
            }
        }

        // Remove empty communities
        communities.retain(|c| !c.is_empty());

        self.clusters = communities.clone();
        self.cluster_count = communities.len();
        self.algorithm = ClusteringAlgorithm::CommunityDetection;
        communities
    }

    /// Calculate distance between two clusters
    fn calculate_cluster_distance(&self, cluster1: &[GraphNode], cluster2: &[GraphNode]) -> f64 {
        let mut total_distance = 0.0;
        let mut count = 0;

        for node1 in cluster1 {
            for node2 in cluster2 {
                let distance = ((node1.x - node2.x).powi(2) + (node1.y - node2.y).powi(2)).sqrt();
                total_distance += distance;
                count += 1;
            }
        }

        if count > 0 {
            total_distance / count as f64
        } else {
            f64::INFINITY
        }
    }

    /// Calculate modularity gain for moving a node to a community
    fn calculate_modularity_gain(
        &self,
        node: &GraphNode,
        communities: &[Vec<GraphNode>],
        edges: &[GraphEdge],
        community_index: usize,
    ) -> f64 {
        // Simplified modularity gain calculation
        let mut gain = 0.0;

        // Count edges within the target community
        let community = &communities[community_index];
        let mut internal_edges = 0;
        let mut total_edges = 0;

        for edge in edges {
            if edge.source == node.id || edge.target == node.id {
                total_edges += 1;
                if community.iter().any(|n| n.id == edge.source)
                    && community.iter().any(|n| n.id == edge.target)
                {
                    internal_edges += 1;
                }
            }
        }

        if total_edges > 0 {
            gain = (internal_edges as f64 / total_edges as f64) - 0.5;
        }

        gain
    }

    /// Calculate silhouette score for clustering quality
    pub fn calculate_silhouette_score(&self, clusters: &[Vec<GraphNode>]) -> f64 {
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
                if cluster.len() > 1 {
                    intra_distance /= (cluster.len() - 1) as f64;
                }

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

    /// Calculate modularity of the clustering
    pub fn calculate_modularity(&self, clusters: &[Vec<GraphNode>], edges: &[GraphEdge]) -> f64 {
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

// ============================================================================
// Interactive Graph Manipulation
// ============================================================================

/// Interactive graph manipulation system
#[derive(Debug, Clone)]
pub struct GraphManipulator {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub selected_nodes: HashSet<String>,
    pub selected_edges: HashSet<String>,
    pub is_dragging: bool,
    pub drag_start_x: f64,
    pub drag_start_y: f64,
    pub drag_offset_x: f64,
    pub drag_offset_y: f64,
}

impl GraphManipulator {
    /// Create a new graph manipulator
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            selected_nodes: HashSet::new(),
            selected_edges: HashSet::new(),
            is_dragging: false,
            drag_start_x: 0.0,
            drag_start_y: 0.0,
            drag_offset_x: 0.0,
            drag_offset_y: 0.0,
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: GraphNode) {
        self.nodes.push(node);
    }

    /// Add an edge to the graph
    pub fn add_edge(&mut self, edge: GraphEdge) {
        self.edges.push(edge);
    }

    /// Select a node
    pub fn select_node(&mut self, node_id: &str) {
        self.selected_nodes.insert(node_id.to_string());
    }

    /// Deselect a node
    pub fn deselect_node(&mut self, node_id: &str) {
        self.selected_nodes.remove(node_id);
    }

    /// Select multiple nodes
    pub fn select_nodes(&mut self, node_ids: &[String]) {
        for node_id in node_ids {
            self.selected_nodes.insert(node_id.clone());
        }
    }

    /// Clear all selections
    pub fn clear_selection(&mut self) {
        self.selected_nodes.clear();
        self.selected_edges.clear();
    }

    /// Start dragging a node
    pub fn start_drag(&mut self, node_id: &str, x: f64, y: f64) {
        if let Some(node) = self.nodes.iter().find(|n| n.id == node_id) {
            self.is_dragging = true;
            self.drag_start_x = x;
            self.drag_start_y = y;
            self.drag_offset_x = x - node.x;
            self.drag_offset_y = y - node.y;
        }
    }

    /// Drag to a new position
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

    /// End dragging
    pub fn end_drag(&mut self) {
        self.is_dragging = false;
        self.drag_offset_x = 0.0;
        self.drag_offset_y = 0.0;
    }

    /// Create an edge between two nodes
    pub fn create_edge(&mut self, source: &str, target: &str, weight: f64) -> Option<GraphEdge> {
        if self.nodes.iter().any(|n| n.id == source)
            && self.nodes.iter().any(|n| n.id == target)
            && source != target
        {
            let edge = GraphEdge::new(source, target, weight);
            self.edges.push(edge.clone());
            Some(edge)
        } else {
            None
        }
    }

    /// Delete a node and its associated edges
    pub fn delete_node(&mut self, node_id: &str) {
        self.nodes.retain(|n| n.id != node_id);
        self.edges
            .retain(|e| e.source != node_id && e.target != node_id);
        self.selected_nodes.remove(node_id);
    }

    /// Delete an edge
    pub fn delete_edge(&mut self, source: &str, target: &str) {
        self.edges
            .retain(|e| !(e.source == source && e.target == target));
    }

    /// Duplicate a node
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

    /// Get a node by ID
    pub fn get_node(&self, node_id: &str) -> Option<&GraphNode> {
        self.nodes.iter().find(|n| n.id == node_id)
    }

    /// Get a mutable reference to a node by ID
    pub fn get_node_mut(&mut self, node_id: &str) -> Option<&mut GraphNode> {
        self.nodes.iter_mut().find(|n| n.id == node_id)
    }

    /// Get node position
    pub fn get_node_position(&self, node_id: &str) -> (f64, f64) {
        if let Some(node) = self.nodes.iter().find(|n| n.id == node_id) {
            (node.x, node.y)
        } else {
            (0.0, 0.0)
        }
    }

    /// Set node position
    pub fn set_node_position(&mut self, node_id: &str, x: f64, y: f64) {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == node_id) {
            node.x = x;
            node.y = y;
        }
    }

    /// Find nodes within a radius of a point
    pub fn find_nodes_in_radius(&self, x: f64, y: f64, radius: f64) -> Vec<String> {
        self.nodes
            .iter()
            .filter(|node| {
                let dx = node.x - x;
                let dy = node.y - y;
                (dx * dx + dy * dy).sqrt() <= radius
            })
            .map(|node| node.id.clone())
            .collect()
    }

    /// Get all connected nodes
    pub fn get_connected_nodes(&self, node_id: &str) -> Vec<String> {
        let mut connected = Vec::new();

        for edge in &self.edges {
            if edge.source == node_id {
                connected.push(edge.target.clone());
            } else if edge.target == node_id {
                connected.push(edge.source.clone());
            }
        }

        connected
    }
}

// ============================================================================
// Network Analysis
// ============================================================================

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

// ============================================================================
// Supporting Types
// ============================================================================

/// Graph node with position and properties
#[derive(Debug, Clone, PartialEq)]
pub struct GraphNode {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub weight: f64,
    pub color: String,
    pub size: f64,
    pub label: String,
}

impl GraphNode {
    /// Create a new graph node
    pub fn new(id: &str, x: f64, y: f64) -> Self {
        Self {
            id: id.to_string(),
            x,
            y,
            weight: 1.0,
            color: "#000000".to_string(),
            size: 10.0,
            label: id.to_string(),
        }
    }

    /// Set node color
    pub fn set_color(&mut self, color: &str) {
        self.color = color.to_string();
    }

    /// Set node size
    pub fn set_size(&mut self, size: f64) {
        self.size = size;
    }

    /// Set node label
    pub fn set_label(&mut self, label: &str) {
        self.label = label.to_string();
    }
}

/// Graph edge with connection and properties
#[derive(Debug, Clone)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    pub weight: f64,
    pub color: String,
    pub width: f64,
    pub label: String,
}

impl GraphEdge {
    /// Create a new graph edge
    pub fn new(source: &str, target: &str, weight: f64) -> Self {
        Self {
            source: source.to_string(),
            target: target.to_string(),
            weight,
            color: "#000000".to_string(),
            width: 1.0,
            label: String::new(),
        }
    }

    /// Set edge color
    pub fn set_color(&mut self, color: &str) {
        self.color = color.to_string();
    }

    /// Set edge width
    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    /// Set edge label
    pub fn set_label(&mut self, label: &str) {
        self.label = label.to_string();
    }
}

/// Centrality measures for a node
#[derive(Debug, Clone)]
pub struct CentralityMeasures {
    pub degree_centrality: f64,
    pub betweenness_centrality: f64,
    pub closeness_centrality: f64,
}

/// Network-level metrics
#[derive(Debug, Clone)]
pub struct NetworkMetrics {
    pub node_count: usize,
    pub edge_count: usize,
    pub density: f64,
    pub clustering_coefficient: f64,
    pub average_path_length: f64,
}

/// Visualization-specific metrics
#[derive(Debug, Clone)]
pub struct VisualizationMetrics {
    pub edge_crossings: usize,
    pub node_overlaps: usize,
    pub layout_quality: f64,
}
