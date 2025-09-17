//! Force-Directed Layout System
//!
//! This module provides force-directed layout algorithms for graph visualization.

use super::{GraphEdge, GraphNode};
use std::collections::HashMap;

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
