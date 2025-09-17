//! Graph Algorithms
//!
//! This module provides various graph algorithms for analysis and manipulation.

use super::{GraphEdge, GraphNode};
use std::collections::{HashMap, HashSet, VecDeque};

/// Graph algorithms for various operations
pub struct GraphAlgorithms;

impl GraphAlgorithms {
    /// Find all connected components in the graph
    pub fn find_connected_components(nodes: &[GraphNode], edges: &[GraphEdge]) -> Vec<Vec<String>> {
        let mut visited = HashSet::new();
        let mut components = Vec::new();

        for node in nodes {
            if !visited.contains(&node.id) {
                let mut component = Vec::new();
                Self::dfs_component(&node.id, nodes, edges, &mut visited, &mut component);
                components.push(component);
            }
        }

        components
    }

    /// Depth-first search for connected components
    fn dfs_component(
        node_id: &str,
        nodes: &[GraphNode],
        edges: &[GraphEdge],
        visited: &mut HashSet<String>,
        component: &mut Vec<String>,
    ) {
        visited.insert(node_id.to_string());
        component.push(node_id.to_string());

        for edge in edges {
            let neighbor = if edge.source == node_id {
                Some(&edge.target)
            } else if edge.target == node_id {
                Some(&edge.source)
            } else {
                None
            };

            if let Some(neighbor_id) = neighbor {
                if !visited.contains(neighbor_id) {
                    Self::dfs_component(neighbor_id, nodes, edges, visited, component);
                }
            }
        }
    }

    /// Find shortest path using Dijkstra's algorithm
    pub fn dijkstra_shortest_path(
        source: &str,
        target: &str,
        nodes: &[GraphNode],
        edges: &[GraphEdge],
    ) -> Option<Vec<String>> {
        let mut distances: HashMap<String, f64> = nodes
            .iter()
            .map(|n| (n.id.clone(), f64::INFINITY))
            .collect();
        let mut previous: HashMap<String, Option<String>> =
            nodes.iter().map(|n| (n.id.clone(), None)).collect();
        let mut unvisited: HashSet<String> = nodes.iter().map(|n| n.id.clone()).collect();

        distances.insert(source.to_string(), 0.0);

        while !unvisited.is_empty() {
            // Find unvisited node with minimum distance
            let current = unvisited
                .iter()
                .min_by(|a, b| {
                    distances
                        .get(*a)
                        .unwrap_or(&f64::INFINITY)
                        .partial_cmp(distances.get(*b).unwrap_or(&f64::INFINITY))
                        .unwrap()
                })?
                .clone();

            if current == target {
                break;
            }

            unvisited.remove(&current);

            // Update distances to neighbors
            for edge in edges {
                let neighbor = if edge.source == current {
                    Some(&edge.target)
                } else if edge.target == current {
                    Some(&edge.source)
                } else {
                    None
                };

                if let Some(neighbor_id) = neighbor {
                    if unvisited.contains(neighbor_id) {
                        let alt = distances.get(&current).unwrap_or(&f64::INFINITY) + edge.weight;
                        if alt < *distances.get(neighbor_id).unwrap_or(&f64::INFINITY) {
                            distances.insert(neighbor_id.clone(), alt);
                            previous.insert(neighbor_id.clone(), Some(current.clone()));
                        }
                    }
                }
            }
        }

        // Reconstruct path
        let mut path = Vec::new();
        let mut current = target.to_string();

        while let Some(prev) = previous.get(&current)? {
            path.push(current);
            current = prev.clone();
        }
        path.push(source.to_string());
        path.reverse();

        Some(path)
    }

    /// Find minimum spanning tree using Kruskal's algorithm
    pub fn kruskal_mst(nodes: &[GraphNode], edges: &[GraphEdge]) -> Vec<GraphEdge> {
        let mut sorted_edges = edges.to_vec();
        sorted_edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());

        let mut mst = Vec::new();
        let mut parent: HashMap<String, String> =
            nodes.iter().map(|n| (n.id.clone(), n.id.clone())).collect();

        for edge in sorted_edges {
            let root_source = Self::find_root(&edge.source, &parent);
            let root_target = Self::find_root(&edge.target, &parent);

            if root_source != root_target {
                mst.push(edge.clone());
                parent.insert(root_source, root_target);
            }
        }

        mst
    }

    /// Find root of a node in union-find structure
    fn find_root(node: &str, parent: &HashMap<String, String>) -> String {
        let mut current = node.to_string();
        while parent.get(&current) != Some(&current) {
            current = parent.get(&current).unwrap().clone();
        }
        current
    }

    /// Detect cycles in the graph using DFS
    pub fn detect_cycle(nodes: &[GraphNode], edges: &[GraphEdge]) -> bool {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for node in nodes {
            if !visited.contains(&node.id) {
                if Self::dfs_cycle(&node.id, nodes, edges, &mut visited, &mut rec_stack) {
                    return true;
                }
            }
        }

        false
    }

    /// DFS helper for cycle detection
    fn dfs_cycle(
        node_id: &str,
        _nodes: &[GraphNode],
        edges: &[GraphEdge],
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
    ) -> bool {
        visited.insert(node_id.to_string());
        rec_stack.insert(node_id.to_string());

        for edge in edges {
            let neighbor = if edge.source == node_id {
                Some(&edge.target)
            } else if edge.target == node_id {
                Some(&edge.source)
            } else {
                None
            };

            if let Some(neighbor_id) = neighbor {
                if !visited.contains(neighbor_id) {
                    if Self::dfs_cycle(neighbor_id, _nodes, edges, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(neighbor_id) {
                    return true;
                }
            }
        }

        rec_stack.remove(node_id);
        false
    }

    /// Topological sort using Kahn's algorithm
    pub fn topological_sort(nodes: &[GraphNode], edges: &[GraphEdge]) -> Option<Vec<String>> {
        let mut in_degree: HashMap<String, usize> =
            nodes.iter().map(|n| (n.id.clone(), 0)).collect();

        // Calculate in-degrees
        for edge in edges {
            *in_degree.get_mut(&edge.target).unwrap() += 1;
        }

        // Find nodes with no incoming edges
        let mut queue: VecDeque<String> = in_degree
            .iter()
            .filter(|(_, &degree)| degree == 0)
            .map(|(node, _)| node.clone())
            .collect();

        let mut result = Vec::new();

        while let Some(current) = queue.pop_front() {
            result.push(current.clone());

            // Update in-degrees of neighbors
            for edge in edges {
                if edge.source == current {
                    let neighbor = &edge.target;
                    if let Some(degree) = in_degree.get_mut(neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
        }

        // Check if all nodes were processed
        if result.len() == nodes.len() {
            Some(result)
        } else {
            None // Cycle detected
        }
    }

    /// Find strongly connected components using Tarjan's algorithm
    pub fn tarjan_scc(nodes: &[GraphNode], edges: &[GraphEdge]) -> Vec<Vec<String>> {
        let mut index = 0;
        let mut stack = Vec::new();
        let mut indices: HashMap<String, usize> = HashMap::new();
        let mut lowlinks: HashMap<String, usize> = HashMap::new();
        let mut on_stack: HashSet<String> = HashSet::new();
        let mut sccs = Vec::new();

        for node in nodes {
            if !indices.contains_key(&node.id) {
                Self::tarjan_visit(
                    &node.id,
                    nodes,
                    edges,
                    &mut index,
                    &mut stack,
                    &mut indices,
                    &mut lowlinks,
                    &mut on_stack,
                    &mut sccs,
                );
            }
        }

        sccs
    }

    /// Tarjan's algorithm helper
    fn tarjan_visit(
        node_id: &str,
        nodes: &[GraphNode],
        edges: &[GraphEdge],
        index: &mut usize,
        stack: &mut Vec<String>,
        indices: &mut HashMap<String, usize>,
        lowlinks: &mut HashMap<String, usize>,
        on_stack: &mut HashSet<String>,
        sccs: &mut Vec<Vec<String>>,
    ) {
        indices.insert(node_id.to_string(), *index);
        lowlinks.insert(node_id.to_string(), *index);
        *index += 1;
        stack.push(node_id.to_string());
        on_stack.insert(node_id.to_string());

        // Visit neighbors
        for edge in edges {
            let neighbor = if edge.source == node_id {
                Some(&edge.target)
            } else if edge.target == node_id {
                Some(&edge.source)
            } else {
                None
            };

            if let Some(neighbor_id) = neighbor {
                if !indices.contains_key(neighbor_id) {
                    Self::tarjan_visit(
                        neighbor_id,
                        nodes,
                        edges,
                        index,
                        stack,
                        indices,
                        lowlinks,
                        on_stack,
                        sccs,
                    );
                    let lowlink = *lowlinks
                        .get(node_id)
                        .unwrap()
                        .min(lowlinks.get(neighbor_id).unwrap());
                    lowlinks.insert(node_id.to_string(), lowlink);
                } else if on_stack.contains(neighbor_id) {
                    let lowlink = *lowlinks
                        .get(node_id)
                        .unwrap()
                        .min(indices.get(neighbor_id).unwrap());
                    lowlinks.insert(node_id.to_string(), lowlink);
                }
            }
        }

        // If node is a root node, pop the stack and create an SCC
        if lowlinks.get(node_id) == indices.get(node_id) {
            let mut scc = Vec::new();
            loop {
                let w = stack.pop().unwrap();
                on_stack.remove(&w);
                scc.push(w.clone());
                if w == *node_id {
                    break;
                }
            }
            sccs.push(scc);
        }
    }
}
