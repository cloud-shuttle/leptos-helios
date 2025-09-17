//! Interactive Graph Manipulation
//!
//! This module provides interactive graph manipulation capabilities for visualization.

use super::{GraphEdge, GraphNode};
use std::collections::HashSet;

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
