//! Comprehensive TDD Tests for Advanced Chart Types
//!
//! This module implements comprehensive Test-Driven Development tests for advanced chart types,
//! including heatmaps, treemaps, and Sankey diagrams.
//!
//! ## Test Coverage Goals
//!
//! - **Heatmap Charts**: Data handling, color mapping, clustering, cell selection
//! - **Treemap Charts**: Hierarchical data, rectangle packing, size calculations
//! - **Sankey Diagrams**: Flow visualization, node connections, path calculations
//! - **Color Schemes**: Color mapping, validation, scheme switching
//! - **Performance**: Large dataset handling, rendering optimization
//!
//! ## TDD Methodology
//!
//! 1. **RED**: Write failing tests first
//! 2. **GREEN**: Implement minimal code to pass tests
//! 3. **REFACTOR**: Improve code quality while maintaining test coverage

use leptos_helios::advanced_chart_types::*;
use std::collections::HashMap;

/// Test suite for Heatmap Chart functionality
mod heatmap_tests {
    use super::*;

    #[test]
    fn test_heatmap_creation() {
        // RED: Test heatmap chart creation
        let data = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];

        let heatmap = HeatmapChart::new(data.clone());

        // GREEN: Verify basic properties
        assert_eq!(heatmap.rows(), 3);
        assert_eq!(heatmap.cols(), 3);
        assert_eq!(heatmap.get_value(0, 0), 1.0);
        assert_eq!(heatmap.get_value(2, 2), 9.0);
        assert_eq!(heatmap.min_value, 1.0);
        assert_eq!(heatmap.max_value, 9.0);
    }

    #[test]
    fn test_heatmap_empty_data() {
        // RED: Test heatmap with empty data
        let data = vec![];
        let heatmap = HeatmapChart::new(data);

        // GREEN: Verify empty data handling
        assert_eq!(heatmap.rows(), 0);
        assert_eq!(heatmap.cols(), 0);
    }

    #[test]
    fn test_heatmap_single_cell() {
        // RED: Test heatmap with single cell
        let data = vec![vec![42.0]];
        let heatmap = HeatmapChart::new(data);

        // GREEN: Verify single cell handling
        assert_eq!(heatmap.rows(), 1);
        assert_eq!(heatmap.cols(), 1);
        assert_eq!(heatmap.get_value(0, 0), 42.0);
        assert_eq!(heatmap.min_value, 42.0);
        assert_eq!(heatmap.max_value, 42.0);
    }

    #[test]
    fn test_heatmap_color_scheme_switching() {
        // RED: Test color scheme switching
        let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let mut heatmap = HeatmapChart::new(data);

        // GREEN: Test default color scheme
        assert_eq!(heatmap.color_scheme, ColorScheme::Viridis);

        // Test switching to different schemes
        heatmap.set_color_scheme(ColorScheme::Plasma);
        assert_eq!(heatmap.color_scheme, ColorScheme::Plasma);

        heatmap.set_color_scheme(ColorScheme::Inferno);
        assert_eq!(heatmap.color_scheme, ColorScheme::Inferno);

        heatmap.set_color_scheme(ColorScheme::Magma);
        assert_eq!(heatmap.color_scheme, ColorScheme::Magma);
    }

    #[test]
    fn test_heatmap_color_generation() {
        // RED: Test color generation for cells
        let data = vec![vec![0.0, 0.5], vec![1.0, 0.75]];
        let heatmap = HeatmapChart::new(data);

        // GREEN: Test color generation
        let color_min = heatmap.get_color(0, 0);
        let color_max = heatmap.get_color(1, 0);

        // Colors should be different for different values
        assert_ne!(color_min, color_max);

        // Colors should be valid hex colors
        assert!(heatmap.is_valid_color(color_min));
        assert!(heatmap.is_valid_color(color_max));
    }

    #[test]
    fn test_heatmap_color_validation() {
        // RED: Test color validation
        let data = vec![vec![1.0, 2.0]];
        let heatmap = HeatmapChart::new(data);

        // GREEN: Test valid and invalid colors
        assert!(heatmap.is_valid_color("#FF0000".to_string()));
        assert!(heatmap.is_valid_color("#00FF00".to_string()));
        assert!(heatmap.is_valid_color("#0000FF".to_string()));

        assert!(!heatmap.is_valid_color("FF0000".to_string())); // Missing #
        assert!(!heatmap.is_valid_color("#FF00".to_string())); // Too short
        assert!(!heatmap.is_valid_color("#FF00000".to_string())); // Too long
        assert!(!heatmap.is_valid_color("red".to_string())); // Named color
    }

    #[test]
    fn test_heatmap_clustering() {
        // RED: Test clustering functionality
        let data = vec![
            vec![1.0, 1.1, 1.2],
            vec![5.0, 5.1, 5.2],
            vec![9.0, 9.1, 9.2],
        ];
        let mut heatmap = HeatmapChart::new(data);

        // GREEN: Test clustering
        heatmap.enable_clustering(3);
        let clusters = heatmap.get_clusters();

        assert_eq!(clusters.len(), 3);

        // Each cluster should have at least one cell
        for cluster in clusters {
            assert!(!cluster.cells.is_empty());
        }
    }

    #[test]
    fn test_heatmap_cell_selection() {
        // RED: Test cell selection functionality
        let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let mut heatmap = HeatmapChart::new(data);

        // GREEN: Test cell selection
        heatmap.select_cell(0, 0);
        heatmap.select_cell(1, 1);

        assert!(heatmap.is_cell_selected(0, 0));
        assert!(heatmap.is_cell_selected(1, 1));
        assert!(!heatmap.is_cell_selected(0, 1));
        assert!(!heatmap.is_cell_selected(1, 0));

        // Test deselecting
        heatmap.deselect_cell(0, 0);
        assert!(!heatmap.is_cell_selected(0, 0));
        assert!(heatmap.is_cell_selected(1, 1));
    }

    #[test]
    fn test_heatmap_cell_position_mapping() {
        // RED: Test cell position mapping
        let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let mut heatmap = HeatmapChart::new(data);

        // Set cell dimensions
        heatmap.set_cell_dimensions(10.0, 10.0);

        // GREEN: Test position to cell mapping
        let cell_00 = heatmap.get_cell_at_position(5.0, 5.0);
        assert_eq!(cell_00, Some((0, 0)));

        let cell_11 = heatmap.get_cell_at_position(15.0, 15.0);
        assert_eq!(cell_11, Some((1, 1)));

        // Test out of bounds
        let out_of_bounds = heatmap.get_cell_at_position(100.0, 100.0);
        assert_eq!(out_of_bounds, None);
    }

    #[test]
    fn test_heatmap_performance_large_dataset() {
        // RED: Test performance with large dataset
        let mut data = Vec::new();
        for i in 0..100 {
            let mut row = Vec::new();
            for j in 0..100 {
                row.push((i * j) as f64);
            }
            data.push(row);
        }

        let start = std::time::Instant::now();
        let heatmap = HeatmapChart::new(data);
        let creation_time = start.elapsed();

        // GREEN: Test performance requirements
        assert!(creation_time < std::time::Duration::from_millis(100));
        assert_eq!(heatmap.rows(), 100);
        assert_eq!(heatmap.cols(), 100);

        // Test color generation performance
        let start = std::time::Instant::now();
        for i in 0..10 {
            for j in 0..10 {
                let _color = heatmap.get_color(i, j);
            }
        }
        let color_time = start.elapsed();

        assert!(color_time < std::time::Duration::from_millis(10));
    }
}

/// Test suite for Treemap Chart functionality
mod treemap_tests {
    use super::*;

    #[test]
    fn test_treemap_creation() {
        // RED: Test treemap chart creation
        let data = vec![TreeNode::new("Root".to_string(), 100.0)];

        let treemap = TreemapChart::new(data);

        // GREEN: Verify basic properties
        assert_eq!(treemap.get_root().name, "Root");
        assert_eq!(treemap.get_root().value, 100.0);
    }

    #[test]
    fn test_treemap_hierarchical_data() {
        // RED: Test hierarchical data structure
        let mut root = TreeNode::new("Root".to_string(), 100.0);
        let mut child1 = TreeNode::new("Child1".to_string(), 60.0);
        let mut child2 = TreeNode::new("Child2".to_string(), 40.0);

        child1.add_child(TreeNode::new("Grandchild1".to_string(), 30.0));
        child1.add_child(TreeNode::new("Grandchild2".to_string(), 30.0));

        root.add_child(child1);
        root.add_child(child2);

        let treemap = TreemapChart::new(vec![root]);

        // GREEN: Verify hierarchical structure
        let root_node = treemap.get_root();
        assert_eq!(root_node.children.len(), 2);
        assert_eq!(root_node.children[0].name, "Child1");
        assert_eq!(root_node.children[1].name, "Child2");
        assert_eq!(root_node.children[0].children.len(), 2);
    }

    #[test]
    fn test_treemap_rectangle_packing() {
        // RED: Test rectangle packing algorithm
        let data = vec![
            TreeNode::new("A".to_string(), 40.0),
            TreeNode::new("B".to_string(), 30.0),
            TreeNode::new("C".to_string(), 20.0),
            TreeNode::new("D".to_string(), 10.0),
        ];

        let mut treemap = TreemapChart::new(data);
        treemap.set_dimensions(100.0, 100.0);
        treemap.pack_rectangles();

        // GREEN: Verify rectangle packing
        let rectangles = treemap.get_rectangles();
        assert_eq!(rectangles.len(), 4);

        // All rectangles should fit within bounds
        for rect in rectangles {
            assert!(rect.x >= 0.0);
            assert!(rect.y >= 0.0);
            assert!(rect.x + rect.width <= 100.0);
            assert!(rect.y + rect.height <= 100.0);
        }
    }

    #[test]
    fn test_treemap_size_calculations() {
        // RED: Test size calculations
        let data = vec![
            TreeNode::new("A".to_string(), 100.0),
            TreeNode::new("B".to_string(), 200.0),
            TreeNode::new("C".to_string(), 300.0),
        ];

        let treemap = TreemapChart::new(data);

        // GREEN: Verify size calculations
        let total_value = treemap.calculate_total_value();
        assert_eq!(total_value, 600.0);

        let percentages = treemap.calculate_percentages();
        assert_eq!(percentages.len(), 3);
        assert!((percentages[0] - 16.67).abs() < 0.01); // 100/600 * 100
        assert!((percentages[1] - 33.33).abs() < 0.01); // 200/600 * 100
        assert!((percentages[2] - 50.0).abs() < 0.01); // 300/600 * 100
    }

    #[test]
    fn test_treemap_interaction() {
        // RED: Test treemap interaction
        let data = vec![
            TreeNode::new("A".to_string(), 100.0),
            TreeNode::new("B".to_string(), 200.0),
        ];

        let mut treemap = TreemapChart::new(data);
        treemap.set_dimensions(100.0, 100.0);
        treemap.pack_rectangles();

        // GREEN: Test node selection
        let node = treemap.get_node_at_position(50.0, 50.0);
        assert!(node.is_some());

        if let Some(selected_node) = node {
            treemap.select_node(&selected_node.name);
            assert!(treemap.is_node_selected(&selected_node.name));
        }
    }
}

/// Test suite for Sankey Diagram functionality
mod sankey_tests {
    use super::*;

    #[test]
    fn test_sankey_creation() {
        // RED: Test Sankey diagram creation
        let nodes = vec![
            SankeyNode::new("Source".to_string(), 100.0),
            SankeyNode::new("Target".to_string(), 100.0),
        ];

        let links = vec![SankeyLink::new(
            "Source".to_string(),
            "Target".to_string(),
            100.0,
        )];

        let sankey = SankeyDiagram::new(nodes, links);

        // GREEN: Verify basic properties
        assert_eq!(sankey.get_nodes().len(), 2);
        assert_eq!(sankey.get_links().len(), 1);
    }

    #[test]
    fn test_sankey_flow_calculation() {
        // RED: Test flow calculation
        let nodes = vec![
            SankeyNode::new("A".to_string(), 100.0),
            SankeyNode::new("B".to_string(), 60.0),
            SankeyNode::new("C".to_string(), 40.0),
        ];

        let links = vec![
            SankeyLink::new("A".to_string(), "B".to_string(), 60.0),
            SankeyLink::new("A".to_string(), "C".to_string(), 40.0),
        ];

        let mut sankey = SankeyDiagram::new(nodes, links);
        sankey.calculate_flows();

        // GREEN: Verify flow calculations
        let flows = sankey.get_flows();
        assert_eq!(flows.len(), 2);

        // Total flow should equal source node value
        let total_flow: f64 = flows.iter().map(|f| f.value).sum();
        assert!((total_flow - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_sankey_path_calculation() {
        // RED: Test path calculation
        let nodes = vec![
            SankeyNode::new("A".to_string(), 100.0),
            SankeyNode::new("B".to_string(), 100.0),
            SankeyNode::new("C".to_string(), 100.0),
        ];

        let links = vec![
            SankeyLink::new("A".to_string(), "B".to_string(), 50.0),
            SankeyLink::new("B".to_string(), "C".to_string(), 50.0),
        ];

        let mut sankey = SankeyDiagram::new(nodes, links);
        sankey.calculate_flows();

        // GREEN: Verify path calculation
        let paths = sankey.calculate_paths("A", "C");
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].len(), 3); // A -> B -> C
        assert_eq!(paths[0][0], "A");
        assert_eq!(paths[0][1], "B");
        assert_eq!(paths[0][2], "C");
    }

    #[test]
    fn test_sankey_node_positioning() {
        // RED: Test node positioning
        let nodes = vec![
            SankeyNode::new("A".to_string(), 100.0),
            SankeyNode::new("B".to_string(), 100.0),
        ];

        let links = vec![SankeyLink::new("A".to_string(), "B".to_string(), 100.0)];

        let mut sankey = SankeyDiagram::new(nodes, links);
        sankey.set_dimensions(200.0, 100.0);
        sankey.calculate_flows();
        sankey.position_nodes();

        // GREEN: Verify node positioning
        let positioned_nodes = sankey.get_positioned_nodes();
        assert_eq!(positioned_nodes.len(), 2);

        // Nodes should be positioned within bounds
        for node in positioned_nodes {
            assert!(node.x >= 0.0);
            assert!(node.y >= 0.0);
            assert!(node.x + node.width <= 200.0);
            assert!(node.y + node.height <= 100.0);
        }
    }

    #[test]
    fn test_sankey_link_curvature() {
        // RED: Test link curvature calculation
        let nodes = vec![
            SankeyNode::new("A".to_string(), 100.0),
            SankeyNode::new("B".to_string(), 100.0),
        ];

        let links = vec![SankeyLink::new("A".to_string(), "B".to_string(), 100.0)];

        let mut sankey = SankeyDiagram::new(nodes, links);
        sankey.set_dimensions(200.0, 100.0);
        sankey.calculate_flows();
        sankey.position_nodes();
        sankey.calculate_link_curves();

        // GREEN: Verify link curvature
        let curved_links = sankey.get_curved_links();
        assert_eq!(curved_links.len(), 1);

        let link = &curved_links[0];
        assert!(link.control_points.len() >= 2); // At least start and end points
    }
}

/// Test suite for Color Scheme functionality
mod color_scheme_tests {
    use super::*;

    #[test]
    fn test_color_scheme_viridis() {
        // RED: Test Viridis color scheme
        let scheme = ColorScheme::Viridis;

        // GREEN: Test color generation
        let color_min = scheme.get_color(0.0);
        let color_max = scheme.get_color(1.0);
        let color_mid = scheme.get_color(0.5);

        assert_ne!(color_min, color_max);
        assert_ne!(color_min, color_mid);
        assert_ne!(color_max, color_mid);

        // All colors should be valid hex colors
        assert!(color_min.starts_with('#') && color_min.len() == 7);
        assert!(color_max.starts_with('#') && color_max.len() == 7);
        assert!(color_mid.starts_with('#') && color_mid.len() == 7);
    }

    #[test]
    fn test_color_scheme_plasma() {
        // RED: Test Plasma color scheme
        let scheme = ColorScheme::Plasma;

        // GREEN: Test color generation
        let color_min = scheme.get_color(0.0);
        let color_max = scheme.get_color(1.0);

        assert_ne!(color_min, color_max);
        assert!(color_min.starts_with('#') && color_min.len() == 7);
        assert!(color_max.starts_with('#') && color_max.len() == 7);
    }

    #[test]
    fn test_color_scheme_inferno() {
        // RED: Test Inferno color scheme
        let scheme = ColorScheme::Inferno;

        // GREEN: Test color generation
        let color_min = scheme.get_color(0.0);
        let color_max = scheme.get_color(1.0);

        assert_ne!(color_min, color_max);
        assert!(color_min.starts_with('#') && color_min.len() == 7);
        assert!(color_max.starts_with('#') && color_max.len() == 7);
    }

    #[test]
    fn test_color_scheme_magma() {
        // RED: Test Magma color scheme
        let scheme = ColorScheme::Magma;

        // GREEN: Test color generation
        let color_min = scheme.get_color(0.0);
        let color_max = scheme.get_color(1.0);

        assert_ne!(color_min, color_max);
        assert!(color_min.starts_with('#') && color_min.len() == 7);
        assert!(color_max.starts_with('#') && color_max.len() == 7);
    }

    #[test]
    fn test_color_scheme_boundary_values() {
        // RED: Test color scheme boundary values
        let scheme = ColorScheme::Viridis;

        // GREEN: Test boundary values
        let color_zero = scheme.get_color(0.0);
        let color_one = scheme.get_color(1.0);

        // Test values outside [0,1] range
        let color_negative = scheme.get_color(-0.5);
        let color_above_one = scheme.get_color(1.5);

        // Should clamp to valid range
        assert_eq!(color_negative, color_zero);
        assert_eq!(color_above_one, color_one);
    }
}

/// Integration tests for advanced chart types
mod integration_tests {
    use super::*;

    #[test]
    fn test_heatmap_treemap_integration() {
        // RED: Test integration between heatmap and treemap
        let heatmap_data = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let heatmap = HeatmapChart::new(heatmap_data);

        let treemap_data = vec![TreeNode::new(
            "Heatmap".to_string(),
            heatmap.calculate_total_value(),
        )];
        let treemap = TreemapChart::new(treemap_data);

        // GREEN: Verify integration
        assert_eq!(treemap.get_root().value, 21.0); // Sum of heatmap values
    }

    #[test]
    fn test_sankey_heatmap_integration() {
        // RED: Test integration between Sankey and heatmap
        let heatmap_data = vec![vec![10.0, 20.0], vec![30.0, 40.0]];
        let heatmap = HeatmapChart::new(heatmap_data);

        let nodes = vec![
            SankeyNode::new("Source".to_string(), heatmap.calculate_total_value()),
            SankeyNode::new("Target".to_string(), heatmap.calculate_total_value()),
        ];

        let links = vec![SankeyLink::new(
            "Source".to_string(),
            "Target".to_string(),
            heatmap.calculate_total_value(),
        )];

        let sankey = SankeyDiagram::new(nodes, links);

        // GREEN: Verify integration
        assert_eq!(sankey.get_nodes()[0].value, 100.0); // Sum of heatmap values
    }

    #[test]
    fn test_performance_comprehensive() {
        // RED: Test comprehensive performance
        let start = std::time::Instant::now();

        // Create large heatmap
        let mut heatmap_data = Vec::new();
        for i in 0..50 {
            let mut row = Vec::new();
            for j in 0..50 {
                row.push((i * j) as f64);
            }
            heatmap_data.push(row);
        }
        let heatmap = HeatmapChart::new(heatmap_data);

        // Create treemap with multiple levels
        let mut root = TreeNode::new("Root".to_string(), 1000.0);
        for i in 0..10 {
            let mut child = TreeNode::new(format!("Child{}", i), 100.0);
            for j in 0..5 {
                child.add_child(TreeNode::new(format!("Grandchild{}{}", i, j), 20.0));
            }
            root.add_child(child);
        }
        let mut treemap = TreemapChart::new(vec![root]);
        treemap.set_dimensions(100.0, 100.0);
        treemap.pack_rectangles();

        // Create complex Sankey diagram
        let mut nodes = Vec::new();
        let mut links = Vec::new();

        for i in 0..10 {
            nodes.push(SankeyNode::new(format!("Node{}", i), 100.0));
        }

        for i in 0..9 {
            links.push(SankeyLink::new(
                format!("Node{}", i),
                format!("Node{}", i + 1),
                50.0,
            ));
        }

        let mut sankey = SankeyDiagram::new(nodes, links);
        sankey.calculate_flows();
        sankey.position_nodes();

        let total_time = start.elapsed();

        // GREEN: Verify performance requirements
        assert!(total_time < std::time::Duration::from_millis(500));

        // Verify all components work
        assert_eq!(heatmap.rows(), 50);
        assert_eq!(treemap.get_rectangles().len(), 51); // 1 root + 10 children + 40 grandchildren
        assert_eq!(sankey.get_nodes().len(), 10);
        assert_eq!(sankey.get_links().len(), 9);
    }
}
