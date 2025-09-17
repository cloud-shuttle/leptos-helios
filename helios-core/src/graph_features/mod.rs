//! Graph Features Module
//!
//! This module provides comprehensive advanced graph visualization features including
//! force-directed layouts, graph clustering, interactive manipulation, and network analysis.

pub mod algorithms;
pub mod analysis;
pub mod clustering;
pub mod layouts;
pub mod visualization;

// Re-export main types and functions
pub use algorithms::*;
pub use analysis::*;
pub use clustering::*;
pub use layouts::*;
pub use visualization::*;

// Common types used across all graph features
use std::collections::{HashMap, HashSet};

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
