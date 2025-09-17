//! TDD Implementation of Core Interactivity Features
//!
//! This module implements the core interactivity features identified in the feature gap analysis:
//! - Zoom & Pan functionality
//! - Rich tooltips with contextual information
//! - Brush selection for data filtering
//! - Cross-filtering between charts
//!
//! Following TDD methodology: Red -> Green -> Refactor

use leptos_helios::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Test suite for Viewport and Zoom/Pan functionality
#[cfg(test)]
mod viewport_tests {
    use super::*;

    #[test]
    fn test_viewport_initialization() {
        // Given: A new viewport
        // When: Creating a viewport with default values
        let viewport = Viewport::new(800.0, 600.0);

        // Then: It should have correct initial values
        assert_eq!(viewport.x, 0.0);
        assert_eq!(viewport.y, 0.0);
        assert_eq!(viewport.scale, 1.0);
        assert_eq!(viewport.width, 800.0);
        assert_eq!(viewport.height, 600.0);
    }

    #[test]
    fn test_viewport_zoom_in() {
        // Given: A viewport at scale 1.0
        let mut viewport = Viewport::new(800.0, 600.0);

        // When: Zooming in by factor 2.0 at center
        viewport.zoom(2.0, 400.0, 300.0);

        // Then: Scale should increase and position should adjust
        assert_eq!(viewport.scale, 2.0);
        // Position should be adjusted to keep zoom center fixed
        assert_eq!(viewport.x, 400.0);
        assert_eq!(viewport.y, 300.0);
    }

    #[test]
    fn test_viewport_zoom_out() {
        // Given: A viewport at scale 2.0
        let mut viewport = Viewport::new(800.0, 600.0);
        viewport.scale = 2.0;
        viewport.x = 100.0;
        viewport.y = 100.0;

        // When: Zooming out by factor 0.5
        viewport.zoom(0.5, 400.0, 300.0);

        // Then: Scale should decrease
        assert_eq!(viewport.scale, 1.0);
    }

    #[test]
    fn test_viewport_pan() {
        // Given: A viewport at origin
        let mut viewport = Viewport::new(800.0, 600.0);

        // When: Panning by (50, 30)
        viewport.pan(50.0, 30.0);

        // Then: Position should update
        assert_eq!(viewport.x, 50.0);
        assert_eq!(viewport.y, 30.0);
    }

    #[test]
    fn test_viewport_bounds_checking() {
        // Given: A viewport with bounds
        let mut viewport = Viewport::new(800.0, 600.0);
        viewport.set_bounds(0.0, 0.0, 1600.0, 1200.0);

        // When: Panning beyond bounds
        viewport.pan(-100.0, -100.0);

        // Then: Position should be clamped to bounds
        assert_eq!(viewport.x, 0.0);
        assert_eq!(viewport.y, 0.0);
    }

    #[test]
    fn test_viewport_coordinate_transformation() {
        // Given: A viewport with offset and scale
        let mut viewport = Viewport::new(800.0, 600.0);
        viewport.x = 100.0;
        viewport.y = 50.0;
        viewport.scale = 2.0;

        // When: Transforming screen coordinates to world coordinates
        let world_pos = viewport.screen_to_world(200.0, 150.0);

        // Then: Should return correct world coordinates
        assert_eq!(world_pos.0, 50.0); // (200 - 100) / 2
        assert_eq!(world_pos.1, 50.0); // (150 - 50) / 2
    }

    #[test]
    fn test_viewport_world_to_screen_transformation() {
        // Given: A viewport with offset and scale
        let mut viewport = Viewport::new(800.0, 600.0);
        viewport.x = 100.0;
        viewport.y = 50.0;
        viewport.scale = 2.0;

        // When: Transforming world coordinates to screen coordinates
        let screen_pos = viewport.world_to_screen(50.0, 25.0);

        // Then: Should return correct screen coordinates
        assert_eq!(screen_pos.0, 200.0); // 50 * 2 + 100
        assert_eq!(screen_pos.1, 100.0); // 25 * 2 + 50
    }
}

/// Test suite for Tooltip functionality
#[cfg(test)]
mod tooltip_tests {
    use super::*;

    #[test]
    fn test_tooltip_creation() {
        // Given: Tooltip data
        let content = "Value: 42.5\nCategory: A\nTimestamp: 2025-01-01";
        let position = (100.0, 200.0);

        // When: Creating a tooltip
        let tooltip = Tooltip::new(content.to_string(), position);

        // Then: Should have correct properties
        assert_eq!(tooltip.content, content);
        assert_eq!(tooltip.position, position);
        assert_eq!(tooltip.visible, false);
    }

    #[test]
    fn test_tooltip_show_hide() {
        // Given: A tooltip
        let mut tooltip = Tooltip::new("Test".to_string(), (0.0, 0.0));

        // When: Showing the tooltip
        tooltip.show();

        // Then: Should be visible
        assert_eq!(tooltip.visible, true);

        // When: Hiding the tooltip
        tooltip.hide();

        // Then: Should be hidden
        assert_eq!(tooltip.visible, false);
    }

    #[test]
    fn test_tooltip_position_update() {
        // Given: A tooltip
        let mut tooltip = Tooltip::new("Test".to_string(), (0.0, 0.0));

        // When: Updating position
        tooltip.update_position(150.0, 250.0);

        // Then: Position should be updated
        assert_eq!(tooltip.position, (150.0, 250.0));
    }

    #[test]
    fn test_tooltip_auto_positioning() {
        // Given: A tooltip and viewport bounds
        let mut tooltip = Tooltip::new("Test".to_string(), (100.0, 100.0));
        let viewport = Viewport::new(800.0, 600.0);

        // When: Auto-positioning near edge
        tooltip.auto_position(&viewport);

        // Then: Should adjust position to stay within bounds
        assert!(tooltip.position.0 >= 0.0);
        assert!(tooltip.position.1 >= 0.0);
        assert!(tooltip.position.0 <= viewport.width);
        assert!(tooltip.position.1 <= viewport.height);
    }

    #[test]
    fn test_tooltip_rich_content() {
        // Given: Rich data for tooltip
        let data = TooltipData {
            title: "Stock Price".to_string(),
            values: vec![
                ("Price".to_string(), "125.50".to_string()),
                ("Volume".to_string(), "1,250,000".to_string()),
                ("Change".to_string(), "+2.5%".to_string()),
            ],
            timestamp: "2025-01-01 12:00:00".to_string(),
        };

        // When: Creating rich tooltip
        let tooltip = Tooltip::from_data(data, (100.0, 100.0));

        // Then: Should format content correctly
        assert!(tooltip.content.contains("Stock Price"));
        assert!(tooltip.content.contains("Price: 125.50"));
        assert!(tooltip.content.contains("Volume: 1,250,000"));
        assert!(tooltip.content.contains("Change: +2.5%"));
    }
}

/// Test suite for Brush Selection functionality
#[cfg(test)]
mod brush_tests {
    use super::*;

    #[test]
    fn test_brush_creation() {
        // Given: Brush coordinates
        let start = (100.0, 100.0);
        let end = (200.0, 200.0);

        // When: Creating a brush selection
        let brush = BrushSelection::new(start, end);

        // Then: Should have correct bounds
        assert_eq!(brush.x1, 100.0);
        assert_eq!(brush.y1, 100.0);
        assert_eq!(brush.x2, 200.0);
        assert_eq!(brush.y2, 200.0);
    }

    #[test]
    fn test_brush_normalization() {
        // Given: A brush with reversed coordinates
        let brush = BrushSelection::new((200.0, 200.0), (100.0, 100.0));

        // When: Normalizing the brush
        let normalized = brush.normalized();

        // Then: Should have correct min/max values
        assert_eq!(normalized.x1, 100.0);
        assert_eq!(normalized.y1, 100.0);
        assert_eq!(normalized.x2, 200.0);
        assert_eq!(normalized.y2, 200.0);
    }

    #[test]
    fn test_brush_contains_point() {
        // Given: A brush selection
        let brush = BrushSelection::new((100.0, 100.0), (200.0, 200.0));

        // When: Checking if points are contained
        let inside = brush.contains_point(150.0, 150.0);
        let outside = brush.contains_point(50.0, 50.0);

        // Then: Should return correct results
        assert_eq!(inside, true);
        assert_eq!(outside, false);
    }

    #[test]
    fn test_brush_intersects_rect() {
        // Given: A brush and rectangle
        let brush = BrushSelection::new((100.0, 100.0), (200.0, 200.0));
        let rect = (150.0, 150.0, 250.0, 250.0); // x, y, width, height

        // When: Checking intersection
        let intersects = brush.intersects_rect(rect);

        // Then: Should intersect
        assert_eq!(intersects, true);
    }

    #[test]
    fn test_brush_data_filtering() {
        // Given: Data points and brush selection
        let data_points = vec![
            DataPoint {
                x: 50.0,
                y: 50.0,
                value: 10.0,
            },
            DataPoint {
                x: 150.0,
                y: 150.0,
                value: 20.0,
            },
            DataPoint {
                x: 250.0,
                y: 250.0,
                value: 30.0,
            },
        ];
        let brush = BrushSelection::new((100.0, 100.0), (200.0, 200.0));

        // When: Filtering data with brush
        let filtered = brush.filter_data(&data_points);

        // Then: Should return only points within brush
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].value, 20.0);
    }

    #[test]
    fn test_brush_clear() {
        // Given: A brush selection
        let mut brush = BrushSelection::new((100.0, 100.0), (200.0, 200.0));

        // When: Clearing the brush
        brush.clear();

        // Then: Should be empty
        assert_eq!(brush.is_empty(), true);
    }
}

/// Test suite for Cross-filtering functionality
#[cfg(test)]
mod cross_filter_tests {
    use super::*;

    #[test]
    fn test_cross_filter_initialization() {
        // Given: Chart IDs
        let chart_ids = vec!["chart1".to_string(), "chart2".to_string()];

        // When: Creating cross-filter
        let cross_filter = CrossFilter::new(chart_ids);

        // Then: Should have correct state
        assert_eq!(cross_filter.charts.len(), 2);
        assert_eq!(cross_filter.active_filters.len(), 0);
    }

    #[test]
    fn test_cross_filter_add_filter() {
        // Given: A cross-filter and brush selection
        let mut cross_filter = CrossFilter::new(vec!["chart1".to_string()]);
        let brush = BrushSelection::new((100.0, 100.0), (200.0, 200.0));

        // When: Adding a filter
        cross_filter.add_filter("chart1", brush);

        // Then: Should have active filter
        assert_eq!(cross_filter.active_filters.len(), 1);
        assert!(cross_filter.has_filter("chart1"));
    }

    #[test]
    fn test_cross_filter_remove_filter() {
        // Given: A cross-filter with active filter
        let mut cross_filter = CrossFilter::new(vec!["chart1".to_string()]);
        let brush = BrushSelection::new((100.0, 100.0), (200.0, 200.0));
        cross_filter.add_filter("chart1", brush);

        // When: Removing the filter
        cross_filter.remove_filter("chart1");

        // Then: Should have no active filters
        assert_eq!(cross_filter.active_filters.len(), 0);
        assert!(!cross_filter.has_filter("chart1"));
    }

    #[test]
    fn test_cross_filter_propagate_filter() {
        // Given: A cross-filter with multiple charts
        let mut cross_filter = CrossFilter::new(vec!["chart1".to_string(), "chart2".to_string()]);
        let brush = BrushSelection::new((100.0, 100.0), (200.0, 200.0));

        // When: Adding filter to one chart
        cross_filter.add_filter("chart1", brush);

        // Then: Should propagate to other charts
        let events = cross_filter.get_filter_events();
        assert_eq!(events.len(), 1); // One event for chart2
        assert_eq!(events[0].target_chart, "chart2");
    }

    #[test]
    fn test_cross_filter_clear_all() {
        // Given: A cross-filter with multiple active filters
        let mut cross_filter = CrossFilter::new(vec!["chart1".to_string(), "chart2".to_string()]);
        let brush1 = BrushSelection::new((100.0, 100.0), (200.0, 200.0));
        let brush2 = BrushSelection::new((300.0, 300.0), (400.0, 400.0));
        cross_filter.add_filter("chart1", brush1);
        cross_filter.add_filter("chart2", brush2);

        // When: Clearing all filters
        cross_filter.clear_all();

        // Then: Should have no active filters
        assert_eq!(cross_filter.active_filters.len(), 0);
    }

    #[test]
    fn test_cross_filter_data_synchronization() {
        // Given: A cross-filter and shared dataset
        let mut cross_filter = CrossFilter::new(vec!["chart1".to_string(), "chart2".to_string()]);
        let shared_data = Arc::new(Mutex::new(vec![
            DataPoint {
                x: 50.0,
                y: 50.0,
                value: 10.0,
            },
            DataPoint {
                x: 150.0,
                y: 150.0,
                value: 20.0,
            },
            DataPoint {
                x: 250.0,
                y: 250.0,
                value: 30.0,
            },
        ]));

        // When: Applying filter
        let brush = BrushSelection::new((100.0, 100.0), (200.0, 200.0));
        cross_filter.add_filter("chart1", brush);

        // Then: Should update shared data
        let filtered_data = cross_filter.get_filtered_data(&shared_data);
        assert_eq!(filtered_data.len(), 1);
        assert_eq!(filtered_data[0].value, 20.0);
    }
}

/// Test suite for Integration tests
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_interactive_chart_workflow() {
        // Given: An interactive chart with all features
        let mut chart = InteractiveChart::new(800.0, 600.0);
        let data = vec![
            DataPoint {
                x: 100.0,
                y: 100.0,
                value: 10.0,
            },
            DataPoint {
                x: 200.0,
                y: 200.0,
                value: 20.0,
            },
            DataPoint {
                x: 300.0,
                y: 300.0,
                value: 30.0,
            },
        ];
        chart.set_data(data);

        // When: Performing interactive operations
        chart.zoom(2.0, 200.0, 200.0);
        chart.pan(50.0, 50.0);
        chart.show_tooltip(200.0, 200.0);
        chart.start_brush_selection(150.0, 150.0);
        chart.update_brush_selection(250.0, 250.0);
        chart.finish_brush_selection();

        // Then: All operations should work together
        assert_eq!(chart.viewport.scale, 2.0);
        assert_eq!(chart.viewport.x, 250.0); // 200 + 50
        assert_eq!(chart.viewport.y, 250.0); // 200 + 50
        assert!(chart.tooltip.visible);
        assert!(!chart.brush.is_empty());
    }

    #[test]
    fn test_multi_chart_cross_filtering() {
        // Given: Multiple charts with shared data
        let mut chart1 = InteractiveChart::new(400.0, 300.0);
        let mut chart2 = InteractiveChart::new(400.0, 300.0);
        let shared_data = Arc::new(Mutex::new(vec![
            DataPoint {
                x: 100.0,
                y: 100.0,
                value: 10.0,
            },
            DataPoint {
                x: 200.0,
                y: 200.0,
                value: 20.0,
            },
            DataPoint {
                x: 300.0,
                y: 300.0,
                value: 30.0,
            },
        ]));

        chart1.set_shared_data(shared_data.clone());
        chart2.set_shared_data(shared_data.clone());

        // When: Applying brush selection on chart1
        let brush = BrushSelection::new((150.0, 150.0), (250.0, 250.0));
        chart1.apply_brush_filter(brush);

        // Then: Chart2 should update automatically
        let chart2_data = chart2.get_filtered_data();
        assert_eq!(chart2_data.len(), 1);
        assert_eq!(chart2_data[0].value, 20.0);
    }

    #[test]
    fn test_performance_with_large_dataset() {
        // Given: A large dataset
        let mut data = Vec::new();
        for i in 0..10000 {
            data.push(DataPoint {
                x: (i as f64) * 0.1,
                y: (i as f64) * 0.1,
                value: (i as f64) * 0.01,
            });
        }

        let mut chart = InteractiveChart::new(800.0, 600.0);
        chart.set_data(data);

        // When: Performing operations on large dataset
        let start = std::time::Instant::now();
        chart.zoom(2.0, 400.0, 300.0);
        chart.pan(100.0, 100.0);
        let brush = BrushSelection::new((200.0, 200.0), (400.0, 400.0));
        let filtered = chart.apply_brush_filter(brush);
        let duration = start.elapsed();

        // Then: Should complete within reasonable time
        assert!(duration.as_millis() < 100); // Less than 100ms
        assert!(filtered.len() > 0);
    }
}

// Data structures for testing
#[derive(Debug, Clone, PartialEq)]
struct DataPoint {
    x: f64,
    y: f64,
    value: f64,
}

#[derive(Debug, Clone)]
struct TooltipData {
    title: String,
    values: Vec<(String, String)>,
    timestamp: String,
}

// Implementation will be added in the next step to make tests pass
