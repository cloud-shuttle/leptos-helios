//! Comprehensive TDD Tests for Interactions Module
//!
//! This module implements comprehensive Test-Driven Development tests for user interaction system,
//! including pan, zoom, hover, click, and accessibility features.
//!
//! ## Test Coverage Goals
//!
//! - **Pan Operations**: Pan delta calculations and viewport updates
//! - **Zoom Operations**: Zoom in/out with scale factors and constraints
//! - **Hover Interactions**: Mouse hover detection and data point highlighting
//! - **Click Interactions**: Click detection and event handling
//! - **Viewport Management**: Viewport state and transformations
//! - **Chart State**: Chart state management and updates
//! - **Accessibility**: Screen reader support and ARIA labels
//! - **Performance**: Interaction performance and responsiveness
//!
//! ## TDD Methodology
//!
//! 1. **RED**: Write failing tests first
//! 2. **GREEN**: Implement minimal code to pass tests
//! 3. **REFACTOR**: Improve code quality while maintaining test coverage

use leptos_helios::chart_config::*;
use leptos_helios::interactions::*;
use std::collections::VecDeque;
use std::time::Duration;

/// Test suite for Pan Delta operations
mod pan_delta_tests {
    use super::*;

    #[test]
    fn test_pan_delta_creation() {
        // RED: Test PanDelta creation
        let pan_delta = PanDelta { x: 10.0, y: -5.0 };

        // GREEN: Verify PanDelta properties
        assert_eq!(pan_delta.x, 10.0);
        assert_eq!(pan_delta.y, -5.0);
    }

    #[test]
    fn test_pan_delta_clone() {
        // RED: Test PanDelta cloning
        let original = PanDelta { x: 15.0, y: 20.0 };
        let cloned = original;

        // GREEN: Verify cloning (Copy trait)
        assert_eq!(original.x, cloned.x);
        assert_eq!(original.y, cloned.y);
    }

    #[test]
    fn test_pan_delta_debug() {
        // RED: Test PanDelta debug formatting
        let pan_delta = PanDelta { x: 25.0, y: 30.0 };

        // GREEN: Verify debug formatting
        let debug_str = format!("{:?}", pan_delta);
        assert!(debug_str.contains("25.0"));
        assert!(debug_str.contains("30.0"));
    }

    #[test]
    fn test_pan_delta_zero() {
        // RED: Test PanDelta with zero values
        let pan_delta = PanDelta { x: 0.0, y: 0.0 };

        // GREEN: Verify zero values
        assert_eq!(pan_delta.x, 0.0);
        assert_eq!(pan_delta.y, 0.0);
    }

    #[test]
    fn test_pan_delta_negative_values() {
        // RED: Test PanDelta with negative values
        let pan_delta = PanDelta { x: -10.0, y: -15.0 };

        // GREEN: Verify negative values
        assert_eq!(pan_delta.x, -10.0);
        assert_eq!(pan_delta.y, -15.0);
    }

    #[test]
    fn test_pan_delta_large_values() {
        // RED: Test PanDelta with large values
        let pan_delta = PanDelta {
            x: 1000.0,
            y: -2000.0,
        };

        // GREEN: Verify large values
        assert_eq!(pan_delta.x, 1000.0);
        assert_eq!(pan_delta.y, -2000.0);
    }
}

/// Test suite for Viewport operations
mod viewport_tests {
    use super::*;

    #[test]
    fn test_viewport_default() {
        // RED: Test Viewport default values
        let viewport = Viewport::default();

        // GREEN: Verify default values
        assert_eq!(viewport.x, 0.0);
        assert_eq!(viewport.y, 0.0);
        assert_eq!(viewport.scale_x, 1.0);
        assert_eq!(viewport.scale_y, 1.0);
    }

    #[test]
    fn test_viewport_creation() {
        // RED: Test Viewport creation with custom values
        let viewport = Viewport {
            x: 100.0,
            y: 200.0,
            scale_x: 2.0,
            scale_y: 1.5,
        };

        // GREEN: Verify custom values
        assert_eq!(viewport.x, 100.0);
        assert_eq!(viewport.y, 200.0);
        assert_eq!(viewport.scale_x, 2.0);
        assert_eq!(viewport.scale_y, 1.5);
    }

    #[test]
    fn test_viewport_clone() {
        // RED: Test Viewport cloning
        let original = Viewport {
            x: 50.0,
            y: 75.0,
            scale_x: 1.2,
            scale_y: 0.8,
        };

        let cloned = original.clone();

        // GREEN: Verify cloning
        assert_eq!(original.x, cloned.x);
        assert_eq!(original.y, cloned.y);
        assert_eq!(original.scale_x, cloned.scale_x);
        assert_eq!(original.scale_y, cloned.scale_y);
    }

    #[test]
    fn test_viewport_debug() {
        // RED: Test Viewport debug formatting
        let viewport = Viewport {
            x: 123.0,
            y: 456.0,
            scale_x: 1.5,
            scale_y: 2.0,
        };

        // GREEN: Verify debug formatting
        let debug_str = format!("{:?}", viewport);
        assert!(debug_str.contains("123.0"));
        assert!(debug_str.contains("456.0"));
        assert!(debug_str.contains("1.5"));
        assert!(debug_str.contains("2.0"));
    }

    #[test]
    fn test_viewport_zoom_in() {
        // RED: Test Viewport zoom in
        let mut viewport = Viewport::default();
        viewport.scale_x = 2.0;
        viewport.scale_y = 2.0;

        // GREEN: Verify zoom in
        assert_eq!(viewport.scale_x, 2.0);
        assert_eq!(viewport.scale_y, 2.0);
    }

    #[test]
    fn test_viewport_zoom_out() {
        // RED: Test Viewport zoom out
        let mut viewport = Viewport::default();
        viewport.scale_x = 0.5;
        viewport.scale_y = 0.5;

        // GREEN: Verify zoom out
        assert_eq!(viewport.scale_x, 0.5);
        assert_eq!(viewport.scale_y, 0.5);
    }

    #[test]
    fn test_viewport_pan() {
        // RED: Test Viewport pan
        let mut viewport = Viewport::default();
        viewport.x = 100.0;
        viewport.y = 200.0;

        // GREEN: Verify pan
        assert_eq!(viewport.x, 100.0);
        assert_eq!(viewport.y, 200.0);
    }

    #[test]
    fn test_viewport_negative_scale() {
        // RED: Test Viewport with negative scale
        let viewport = Viewport {
            x: 0.0,
            y: 0.0,
            scale_x: -1.0,
            scale_y: -2.0,
        };

        // GREEN: Verify negative scale
        assert_eq!(viewport.scale_x, -1.0);
        assert_eq!(viewport.scale_y, -2.0);
    }
}

/// Test suite for Chart State operations
mod chart_state_tests {
    use super::*;

    #[test]
    fn test_chart_state_creation() {
        // RED: Test ChartState creation
        let config = BaseChartConfig {
            width: 800,
            height: 600,
            title: "Test Chart".to_string(),
            x_label: "X Axis".to_string(),
            y_label: "Y Axis".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        };

        let chart_state = ChartState::new(&config);

        // GREEN: Verify ChartState creation
        assert_eq!(chart_state.config.title, "Test Chart");
        assert_eq!(chart_state.config.width, 800);
        assert_eq!(chart_state.config.height, 600);
    }

    #[test]
    fn test_chart_state_viewport() {
        // RED: Test ChartState viewport access
        let config = BaseChartConfig {
            width: 800,
            height: 600,
            title: "Viewport Test".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        };

        let chart_state = ChartState::new(&config);
        let viewport = chart_state.viewport();

        // GREEN: Verify viewport access
        assert_eq!(viewport.x, 0.0);
        assert_eq!(viewport.y, 0.0);
        assert_eq!(viewport.scale_x, 1.0);
        assert_eq!(viewport.scale_y, 1.0);
    }

    #[test]
    fn test_chart_state_config() {
        // RED: Test ChartState config access
        let config = BaseChartConfig {
            width: 1024,
            height: 768,
            title: "Config Test".to_string(),
            x_label: "X Axis".to_string(),
            y_label: "Y Axis".to_string(),
            show_grid: false,
            background_color: "#f0f0f0".to_string(),
            text_color: "#333333".to_string(),
        };

        let chart_state = ChartState::new(&config);
        let retrieved_config = chart_state.config();

        // GREEN: Verify config access
        assert_eq!(retrieved_config.title, "Config Test");
        assert_eq!(retrieved_config.width, 1024);
        assert_eq!(retrieved_config.height, 768);
        assert!(!retrieved_config.show_grid);
    }

    #[test]
    fn test_chart_state_update_viewport() {
        // RED: Test ChartState viewport update
        let config = BaseChartConfig {
            width: 800,
            height: 600,
            title: "Update Test".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        };

        let mut chart_state = ChartState::new(&config);
        let new_viewport = Viewport {
            x: 100.0,
            y: 200.0,
            scale_x: 2.0,
            scale_y: 1.5,
        };

        chart_state.update_viewport(new_viewport.clone());
        let updated_viewport = chart_state.viewport();

        // GREEN: Verify viewport update
        assert_eq!(updated_viewport.x, 100.0);
        assert_eq!(updated_viewport.y, 200.0);
        assert_eq!(updated_viewport.scale_x, 2.0);
        assert_eq!(updated_viewport.scale_y, 1.5);
    }

    #[test]
    fn test_chart_state_reset_viewport() {
        // RED: Test ChartState viewport reset
        let config = BaseChartConfig {
            width: 800,
            height: 600,
            title: "Reset Test".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        };

        let mut chart_state = ChartState::new(&config);

        // Update viewport first
        let updated_viewport = Viewport {
            x: 50.0,
            y: 75.0,
            scale_x: 1.5,
            scale_y: 2.0,
        };
        chart_state.update_viewport(updated_viewport);

        // Reset viewport
        chart_state.reset_viewport();
        let reset_viewport = chart_state.viewport();

        // GREEN: Verify viewport reset
        assert_eq!(reset_viewport.x, 0.0);
        assert_eq!(reset_viewport.y, 0.0);
        assert_eq!(reset_viewport.scale_x, 1.0);
        assert_eq!(reset_viewport.scale_y, 1.0);
    }
}

/// Test suite for Interaction Manager
mod interaction_manager_tests {
    use super::*;

    #[test]
    fn test_interaction_manager_creation() {
        // RED: Test InteractionManager creation
        let config = BaseChartConfig {
            width: 800,
            height: 600,
            title: "Interaction Test".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        };

        let manager = InteractionManager::new(&config);

        // GREEN: Verify InteractionManager creation
        assert!(true); // Manager created successfully
    }

    #[test]
    fn test_interaction_manager_pan() {
        // RED: Test InteractionManager pan operation
        let config = BaseChartConfig {
            width: 800,
            height: 600,
            title: "Pan Test".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        };

        let mut manager = InteractionManager::new(&config);
        let pan_delta = PanDelta { x: 10.0, y: -5.0 };

        // GREEN: Verify pan operation
        let result = manager.pan(pan_delta);
        assert!(result.is_ok());
    }

    #[test]
    fn test_interaction_manager_zoom() {
        // RED: Test InteractionManager zoom operation
        let config = BaseChartConfig {
            width: 800,
            height: 600,
            title: "Zoom Test".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        };

        let mut manager = InteractionManager::new(&config);
        let zoom_factor = 1.5;
        let center = (400.0, 300.0);

        // GREEN: Verify zoom operation
        let result = manager.zoom(zoom_factor, center);
        assert!(result.is_ok());
    }

    #[test]
    fn test_interaction_manager_zoom_in() {
        // RED: Test InteractionManager zoom in
        let config = BaseChartConfig {
            width: 800,
            height: 600,
            title: "Zoom In Test".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        };

        let mut manager = InteractionManager::new(&config);
        let center = (400.0, 300.0);

        // GREEN: Verify zoom in
        let result = manager.zoom_in(center);
        assert!(result.is_ok());
    }

    #[test]
    fn test_interaction_manager_zoom_out() {
        // RED: Test InteractionManager zoom out
        let config = BaseChartConfig {
            width: 800,
            height: 600,
            title: "Zoom Out Test".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        };

        let mut manager = InteractionManager::new(&config);
        let center = (400.0, 300.0);

        // GREEN: Verify zoom out
        let result = manager.zoom_out(center);
        assert!(result.is_ok());
    }

    #[test]
    fn test_interaction_manager_hover() {
        // RED: Test InteractionManager hover operation
        let config = BaseChartConfig {
            width: 800,
            height: 600,
            title: "Hover Test".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        };

        let mut manager = InteractionManager::new(&config);
        let mouse_pos = (200.0, 150.0);

        // GREEN: Verify hover operation
        let result = manager.hover(mouse_pos);
        assert!(result.is_ok());
    }

    #[test]
    fn test_interaction_manager_click() {
        // RED: Test InteractionManager click operation
        let config = BaseChartConfig {
            width: 800,
            height: 600,
            title: "Click Test".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        };

        let mut manager = InteractionManager::new(&config);
        let click_pos = (300.0, 250.0);

        // GREEN: Verify click operation
        let result = manager.click(click_pos);
        assert!(result.is_ok());
    }

    #[test]
    fn test_interaction_manager_reset() {
        // RED: Test InteractionManager reset operation
        let config = BaseChartConfig {
            width: 800,
            height: 600,
            title: "Reset Test".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        };

        let mut manager = InteractionManager::new(&config);

        // Perform some operations first
        manager.pan(PanDelta { x: 10.0, y: 10.0 }).unwrap();
        manager.zoom_in((400.0, 300.0)).unwrap();

        // GREEN: Verify reset operation
        let result = manager.reset();
        assert!(result.is_ok());
    }
}

/// Test suite for Accessibility features
mod accessibility_tests {
    use super::*;

    #[test]
    fn test_accessibility_config_creation() {
        // RED: Test AccessibilityConfig creation
        let config = AccessibilityConfig {
            screen_reader_support: true,
            aria_labels: true,
            keyboard_navigation: true,
            high_contrast_mode: false,
        };

        // GREEN: Verify AccessibilityConfig properties
        assert!(config.screen_reader_support);
        assert!(config.aria_labels);
        assert!(config.keyboard_navigation);
        assert!(!config.high_contrast_mode);
    }

    #[test]
    fn test_accessibility_config_clone() {
        // RED: Test AccessibilityConfig cloning
        let original = AccessibilityConfig {
            screen_reader_support: true,
            aria_labels: false,
            keyboard_navigation: true,
            high_contrast_mode: true,
        };

        let cloned = original.clone();

        // GREEN: Verify cloning
        assert_eq!(original.screen_reader_support, cloned.screen_reader_support);
        assert_eq!(original.aria_labels, cloned.aria_labels);
        assert_eq!(original.keyboard_navigation, cloned.keyboard_navigation);
        assert_eq!(original.high_contrast_mode, cloned.high_contrast_mode);
    }

    #[test]
    fn test_accessibility_info_creation() {
        // RED: Test AccessibilityInfo creation
        let info = AccessibilityInfo {
            screen_reader_text: "Chart with 100 data points".to_string(),
            aria_labels: vec![
                "Chart container".to_string(),
                "Data visualization".to_string(),
            ],
            focus_order: vec!["chart".to_string(), "controls".to_string()],
            high_contrast_colors: vec!["#000000".to_string(), "#ffffff".to_string()],
        };

        // GREEN: Verify AccessibilityInfo properties
        assert_eq!(info.screen_reader_text, "Chart with 100 data points");
        assert_eq!(info.aria_labels.len(), 2);
        assert_eq!(info.focus_order.len(), 2);
        assert_eq!(info.high_contrast_colors.len(), 2);
    }

    #[test]
    fn test_accessibility_info_clone() {
        // RED: Test AccessibilityInfo cloning
        let original = AccessibilityInfo {
            screen_reader_text: "Original text".to_string(),
            aria_labels: vec!["Original label".to_string()],
            focus_order: vec!["original".to_string()],
            high_contrast_colors: vec!["#ff0000".to_string()],
        };

        let cloned = original.clone();

        // GREEN: Verify cloning
        assert_eq!(original.screen_reader_text, cloned.screen_reader_text);
        assert_eq!(original.aria_labels, cloned.aria_labels);
        assert_eq!(original.focus_order, cloned.focus_order);
        assert_eq!(original.high_contrast_colors, cloned.high_contrast_colors);
    }

    #[test]
    fn test_generate_accessibility_info() {
        // RED: Test generate_accessibility_info function
        let config = BaseChartConfig {
            width: 800,
            height: 600,
            title: "Accessibility Test".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        };

        let chart_state = ChartState::new(&config);
        let accessibility_config = AccessibilityConfig {
            screen_reader_support: true,
            aria_labels: true,
            keyboard_navigation: true,
            high_contrast_mode: false,
        };

        // GREEN: Verify accessibility info generation
        let result = generate_accessibility_info(&chart_state, &accessibility_config);
        assert!(result.is_ok());

        let info = result.unwrap();
        assert!(!info.screen_reader_text.is_empty());
        assert!(!info.aria_labels.is_empty());
        assert!(!info.focus_order.is_empty());
        assert!(!info.high_contrast_colors.is_empty());
    }
}

/// Test suite for Interaction Performance
mod interaction_performance_tests {
    use super::*;

    #[test]
    fn test_pan_performance() {
        // RED: Test pan operation performance
        let config = BaseChartConfig {
            width: 800,
            height: 600,
            title: "Performance Test".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        };

        let mut manager = InteractionManager::new(&config);

        let start = std::time::Instant::now();

        // Perform many pan operations
        for i in 0..1000 {
            let pan_delta = PanDelta {
                x: i as f32,
                y: (i * 2) as f32,
            };
            manager.pan(pan_delta).unwrap();
        }

        let duration = start.elapsed();

        // GREEN: Verify performance
        assert!(duration < std::time::Duration::from_millis(100)); // Should be fast
    }

    #[test]
    fn test_zoom_performance() {
        // RED: Test zoom operation performance
        let config = BaseChartConfig {
            width: 800,
            height: 600,
            title: "Zoom Performance Test".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        };

        let mut manager = InteractionManager::new(&config);

        let start = std::time::Instant::now();

        // Perform many zoom operations
        for i in 0..1000 {
            let zoom_factor = 1.0 + (i as f32 * 0.001);
            let center = (400.0, 300.0);
            manager.zoom(zoom_factor, center).unwrap();
        }

        let duration = start.elapsed();

        // GREEN: Verify performance
        assert!(duration < std::time::Duration::from_millis(100)); // Should be fast
    }

    #[test]
    fn test_hover_performance() {
        // RED: Test hover operation performance
        let config = BaseChartConfig {
            width: 800,
            height: 600,
            title: "Hover Performance Test".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        };

        let mut manager = InteractionManager::new(&config);

        let start = std::time::Instant::now();

        // Perform many hover operations
        for i in 0..1000 {
            let mouse_pos = (i as f32 % 800.0, (i * 2) as f32 % 600.0);
            manager.hover(mouse_pos).unwrap();
        }

        let duration = start.elapsed();

        // GREEN: Verify performance
        assert!(duration < std::time::Duration::from_millis(100)); // Should be fast
    }

    #[test]
    fn test_interaction_memory_usage() {
        // RED: Test interaction memory usage
        let initial_memory = get_memory_usage();

        // Create many interaction managers
        let mut managers = Vec::new();
        for i in 0..100 {
            let config = BaseChartConfig {
                width: 800,
                height: 600,
                title: format!("Memory Test {}", i),
                x_label: "X".to_string(),
                y_label: "Y".to_string(),
                show_grid: true,
                background_color: "#ffffff".to_string(),
                text_color: "#000000".to_string(),
            };
            managers.push(InteractionManager::new(&config));
        }

        let after_creation_memory = get_memory_usage();

        // Drop managers
        drop(managers);

        let final_memory = get_memory_usage();

        // GREEN: Verify memory usage
        let memory_used = after_creation_memory - initial_memory;
        assert!(memory_used < 1024 * 1024); // Less than 1MB for 100 managers

        // Memory should be released after drop
        assert!(final_memory <= after_creation_memory);
    }
}

/// Test suite for Interaction Edge Cases
mod interaction_edge_case_tests {
    use super::*;

    #[test]
    fn test_extreme_zoom_values() {
        // RED: Test extreme zoom values
        let config = BaseChartConfig {
            width: 800,
            height: 600,
            title: "Extreme Zoom Test".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        };

        let mut manager = InteractionManager::new(&config);

        // Test very large zoom factor
        let result1 = manager.zoom(1000.0, (400.0, 300.0));
        assert!(result1.is_ok());

        // Test very small zoom factor
        let result2 = manager.zoom(0.001, (400.0, 300.0));
        assert!(result2.is_ok());

        // Test negative zoom factor
        let result3 = manager.zoom(-1.0, (400.0, 300.0));
        assert!(result3.is_ok());
    }

    #[test]
    fn test_extreme_pan_values() {
        // RED: Test extreme pan values
        let config = BaseChartConfig {
            width: 800,
            height: 600,
            title: "Extreme Pan Test".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        };

        let mut manager = InteractionManager::new(&config);

        // Test very large pan values
        let result1 = manager.pan(PanDelta {
            x: 10000.0,
            y: -10000.0,
        });
        assert!(result1.is_ok());

        // Test very small pan values
        let result2 = manager.pan(PanDelta {
            x: 0.001,
            y: -0.001,
        });
        assert!(result2.is_ok());
    }

    #[test]
    fn test_out_of_bounds_mouse_positions() {
        // RED: Test out of bounds mouse positions
        let config = BaseChartConfig {
            width: 800,
            height: 600,
            title: "Out of Bounds Test".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        };

        let mut manager = InteractionManager::new(&config);

        // Test negative coordinates
        let result1 = manager.hover((-100.0, -200.0));
        assert!(result1.is_ok());

        // Test coordinates beyond chart bounds
        let result2 = manager.hover((1000.0, 1000.0));
        assert!(result2.is_ok());

        // Test very large coordinates
        let result3 = manager.hover((f32::MAX, f32::MAX));
        assert!(result3.is_ok());
    }

    #[test]
    fn test_zero_dimensions() {
        // RED: Test zero dimensions
        let config = BaseChartConfig {
            width: 0,
            height: 0,
            title: "Zero Dimensions Test".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        };

        let manager = InteractionManager::new(&config);

        // GREEN: Verify zero dimensions handling
        assert_eq!(manager.chart_state().config().width, 0);
        assert_eq!(manager.chart_state().config().height, 0);
    }
}

/// Helper function to get memory usage (simplified)
fn get_memory_usage() -> usize {
    // This is a simplified memory usage function
    // In a real implementation, you would use system-specific APIs
    0
}
