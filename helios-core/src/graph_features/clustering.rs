//! Graph Clustering System
//!
//! This module provides graph clustering algorithms for community detection and analysis.

use super::{GraphEdge, GraphNode};

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
