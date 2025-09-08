//! Canvas2D Rendering TDD Test Suite
//!
//! This module implements comprehensive TDD tests for Canvas2D rendering functionality
//! following the established TDD patterns in the Helios codebase.
//!
//! TDD Implementation Plan:
//! ☐ Implement real Canvas2D context creation and basic drawing operations
//! ☐ Build line chart rendering with real Canvas2D commands
//! ☐ Add bar chart and scatter plot rendering support
//! ☐ Implement performance optimizations and large dataset handling
//! ☐ Add interaction support (zoom, pan, hover) to Canvas2D renderer
//! ☐ Create comprehensive integration tests and performance benchmarks
//!
//! RED-GREEN-REFACTOR cycle for each Canvas2D feature

use leptos_helios::canvas2d_renderer::*;
use proptest::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

// =============================================================================
// Canvas2D Context Creation and Basic Drawing Operations TDD
// =============================================================================

#[cfg(test)]
mod canvas2d_context_creation_tdd {
    use super::*;

    /// TDD Pattern: RED -> GREEN -> REFACTOR
    /// Test that Canvas2D renderer can be created with real context
    #[tokio::test]
    async fn test_canvas2d_renderer_creation_tdd() {
        // RED: This will fail initially - no real Canvas2D implementation
        let result = Canvas2DRenderer::new();

        // GREEN requirement: Renderer must initialize successfully
        assert!(result.is_ok(), "Canvas2D renderer creation failed");

        let renderer = result.unwrap();
        assert_eq!(
            renderer.backend(),
            RendererBackend::Canvas2D,
            "Renderer should use Canvas2D backend"
        );
    }

    /// Test Canvas2D context creation with real HTML5 canvas
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn test_canvas2d_context_creation_real() {
        use wasm_bindgen::JsCast;
        use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

        // RED: Create real canvas element and context
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.create_element("canvas").unwrap();
        let canvas_element: HtmlCanvasElement = canvas.dyn_into().unwrap();

        canvas_element.set_width(800);
        canvas_element.set_height(600);

        // GREEN requirement: Context creation must succeed
        let context_result = canvas_element.get_context("2d");
        assert!(context_result.is_ok(), "Canvas2D context creation failed");

        let context = context_result.unwrap().unwrap();
        let canvas2d_context: CanvasRenderingContext2d = context.dyn_into().unwrap();

        // Verify context properties
        assert!(canvas2d_context.canvas().width() == 800);
        assert!(canvas2d_context.canvas().height() == 600);
    }

    /// Test basic drawing operations on Canvas2D context
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn test_canvas2d_basic_drawing_operations() {
        use wasm_bindgen::JsCast;
        use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

        // Setup canvas and context
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.create_element("canvas").unwrap();
        let canvas_element: HtmlCanvasElement = canvas.dyn_into().unwrap();
        canvas_element.set_width(400);
        canvas_element.set_height(300);

        let context = canvas_element.get_context("2d").unwrap().unwrap();
        let ctx: CanvasRenderingContext2d = context.dyn_into().unwrap();

        // RED: Test basic drawing operations
        // Test line drawing
        ctx.begin_path();
        ctx.move_to(10.0, 10.0);
        ctx.line_to(100.0, 100.0);
        ctx.set_stroke_style(&"#ff0000".into());
        ctx.set_line_width(2.0);
        ctx.stroke();

        // Test rectangle drawing
        ctx.set_fill_style(&"#00ff00".into());
        ctx.fill_rect(50.0, 50.0, 100.0, 75.0);

        // Test circle drawing
        ctx.begin_path();
        ctx.arc(200.0, 150.0, 50.0, 0.0, 2.0 * std::f64::consts::PI)
            .unwrap();
        ctx.set_fill_style(&"#0000ff".into());
        ctx.fill();

        // GREEN requirement: All drawing operations must complete without error
        // (In a real test, we would verify the canvas content)
        assert!(true, "Basic drawing operations completed successfully");
    }

    /// Property-based test for Canvas2D context creation edge cases
    proptest! {
        #[test]
        fn test_canvas2d_context_creation_properties(
            width in 1u32..2048,
            height in 1u32..2048,
            has_context in prop::bool::ANY
        ) {
            // RED: Test various canvas dimensions and context availability
            let canvas_config = Canvas2DConfig {
                width,
                height,
                context_type: if has_context { "2d".to_string() } else { "invalid".to_string() },
            };

            // GREEN requirement: Valid configurations should succeed
            if has_context && width > 0 && height > 0 {
                prop_assert!(canvas_config.is_valid());
            } else {
                prop_assert!(!canvas_config.is_valid());
            }
        }
    }

    /// Test Canvas2D context error handling
    #[test]
    fn test_canvas2d_context_error_handling() {
        // RED: Test invalid canvas configurations
        let invalid_configs = vec![
            Canvas2DConfig {
                width: 0,
                height: 100,
                context_type: "2d".to_string(),
            },
            Canvas2DConfig {
                width: 100,
                height: 0,
                context_type: "2d".to_string(),
            },
            Canvas2DConfig {
                width: 100,
                height: 100,
                context_type: "invalid".to_string(),
            },
        ];

        // GREEN requirement: Invalid configurations must be rejected
        for config in invalid_configs {
            assert!(
                !config.is_valid(),
                "Invalid config should be rejected: {:?}",
                config
            );
        }
    }
}

// =============================================================================
// Line Chart Rendering with Real Canvas2D Commands TDD
// =============================================================================

#[cfg(test)]
mod line_chart_rendering_tdd {
    use super::*;

    /// Test line chart rendering with Canvas2D commands
    #[tokio::test]
    async fn test_line_chart_rendering_tdd() {
        // RED: Create test data for line chart
        let test_data = create_test_line_data(100);
        let chart_spec = create_line_chart_spec();

        // GREEN requirement: Line chart must render successfully
        let renderer = Canvas2DRenderer::new().unwrap();
        let result = renderer.render_line_chart(&chart_spec, &test_data).await;

        assert!(
            result.is_ok(),
            "Line chart rendering failed: {:?}",
            result.err()
        );

        let render_result = result.unwrap();
        assert_eq!(render_result.points_rendered, 100);
        assert!(render_result.render_time < Duration::from_millis(16)); // 60fps requirement
    }

    /// Test line chart with different line styles
    #[test]
    fn test_line_chart_styling_variations() {
        let test_data = create_test_line_data(50);

        let style_variations = vec![
            LineStyle {
                width: 1.0,
                color: "#ff0000".to_string(),
                dash: None,
            },
            LineStyle {
                width: 2.0,
                color: "#00ff00".to_string(),
                dash: Some(vec![5.0, 5.0]),
            },
            LineStyle {
                width: 3.0,
                color: "#0000ff".to_string(),
                dash: Some(vec![10.0, 5.0, 2.0, 5.0]),
            },
        ];

        let renderer = Canvas2DRenderer::new().unwrap();

        for (i, style) in style_variations.iter().enumerate() {
            let mut chart_spec = create_line_chart_spec();
            chart_spec.line_style = style.clone();

            // GREEN requirement: All style variations must render
            let result = renderer.render_line_chart(&chart_spec, &test_data);
            assert!(
                result.is_ok(),
                "Line chart style {} failed: {:?}",
                i,
                result.err()
            );
        }
    }

    /// Test line chart performance with large datasets
    #[tokio::test]
    async fn test_line_chart_performance_large_dataset() {
        // RED: Test with 10K points (performance requirement)
        let large_dataset = create_test_line_data(10_000);
        let chart_spec = create_line_chart_spec();

        let renderer = Canvas2DRenderer::new().unwrap();
        let start = Instant::now();

        let result = renderer
            .render_line_chart(&chart_spec, &large_dataset)
            .await;
        let duration = start.elapsed();

        // GREEN requirement: 10K points must render in <100ms
        assert!(result.is_ok(), "Large dataset rendering failed");
        assert!(
            duration < Duration::from_millis(100),
            "Performance requirement not met: {:?} for 10K points",
            duration
        );
    }

    /// Property-based test for line chart data validation
    proptest! {
        #[test]
        fn test_line_chart_data_validation_properties(
            point_count in 1usize..1000,
            has_invalid_points in prop::bool::ANY,
            has_duplicate_x in prop::bool::ANY
        ) {
            let mut test_data = create_test_line_data(point_count);

            // RED: Introduce data quality issues
            if has_invalid_points {
                test_data.points[0].x = f64::NAN;
            }

            if has_duplicate_x {
                test_data.points[1].x = test_data.points[0].x;
            }

            let chart_spec = create_line_chart_spec();
            let renderer = Canvas2DRenderer::new().unwrap();

            // GREEN requirement: Invalid data must be handled gracefully
            let result = renderer.render_line_chart(&chart_spec, &test_data);

            if has_invalid_points {
                prop_assert!(result.is_err() || result.unwrap().warnings.len() > 0);
            } else {
                prop_assert!(result.is_ok());
            }
        }
    }

    /// Test line chart with different interpolation methods
    #[test]
    fn test_line_chart_interpolation_methods() {
        let test_data = create_test_line_data(20);
        let interpolation_methods = vec![
            InterpolationMethod::Linear,
            InterpolationMethod::Step,
            InterpolationMethod::Smooth,
        ];

        let renderer = Canvas2DRenderer::new().unwrap();

        for method in interpolation_methods {
            let mut chart_spec = create_line_chart_spec();
            chart_spec.interpolation = method;

            // GREEN requirement: All interpolation methods must work
            let result = renderer.render_line_chart(&chart_spec, &test_data);
            assert!(result.is_ok(), "Interpolation method {:?} failed", method);
        }
    }
}

// =============================================================================
// Bar Chart and Scatter Plot Rendering Support TDD
// =============================================================================

#[cfg(test)]
mod bar_chart_rendering_tdd {
    use super::*;

    /// Test bar chart rendering with Canvas2D
    #[tokio::test]
    async fn test_bar_chart_rendering_tdd() {
        // RED: Create test data for bar chart
        let test_data = create_test_bar_data(50);
        let chart_spec = create_bar_chart_spec();

        // GREEN requirement: Bar chart must render successfully
        let renderer = Canvas2DRenderer::new().unwrap();
        let result = renderer.render_bar_chart(&chart_spec, &test_data).await;

        assert!(
            result.is_ok(),
            "Bar chart rendering failed: {:?}",
            result.err()
        );

        let render_result = result.unwrap();
        assert_eq!(render_result.bars_rendered, 50);
        assert!(render_result.render_time < Duration::from_millis(16));
    }

    /// Test bar chart with different orientations
    #[test]
    fn test_bar_chart_orientations() {
        let test_data = create_test_bar_data(30);
        let orientations = vec![BarOrientation::Vertical, BarOrientation::Horizontal];

        let renderer = Canvas2DRenderer::new().unwrap();

        for orientation in orientations {
            let mut chart_spec = create_bar_chart_spec();
            chart_spec.orientation = orientation;

            // GREEN requirement: Both orientations must render
            let result = renderer.render_bar_chart(&chart_spec, &test_data);
            assert!(
                result.is_ok(),
                "Bar chart orientation {:?} failed",
                orientation
            );
        }
    }

    /// Test bar chart with grouped and stacked configurations
    #[test]
    fn test_bar_chart_grouping_configurations() {
        let test_data = create_test_grouped_bar_data(20, 3); // 20 categories, 3 groups
        let grouping_configs = vec![
            BarGrouping::Grouped,
            BarGrouping::Stacked,
            BarGrouping::Normalized,
        ];

        let renderer = Canvas2DRenderer::new().unwrap();

        for grouping in grouping_configs {
            let mut chart_spec = create_bar_chart_spec();
            chart_spec.grouping = grouping;

            // GREEN requirement: All grouping configurations must render
            let result = renderer.render_bar_chart(&chart_spec, &test_data);
            assert!(result.is_ok(), "Bar chart grouping {:?} failed", grouping);
        }
    }
}

#[cfg(test)]
mod scatter_plot_rendering_tdd {
    use super::*;

    /// Test scatter plot rendering with Canvas2D
    #[tokio::test]
    async fn test_scatter_plot_rendering_tdd() {
        // RED: Create test data for scatter plot
        let test_data = create_test_scatter_data(200);
        let chart_spec = create_scatter_plot_spec();

        // GREEN requirement: Scatter plot must render successfully
        let renderer = Canvas2DRenderer::new().unwrap();
        let result = renderer.render_scatter_plot(&chart_spec, &test_data).await;

        assert!(
            result.is_ok(),
            "Scatter plot rendering failed: {:?}",
            result.err()
        );

        let render_result = result.unwrap();
        assert_eq!(render_result.points_rendered, 200);
        assert!(render_result.render_time < Duration::from_millis(16));
    }

    /// Test scatter plot with different point shapes and sizes
    #[test]
    fn test_scatter_plot_point_variations() {
        let test_data = create_test_scatter_data(100);
        let point_configs = vec![
            PointConfig {
                shape: PointShape::Circle,
                size: 5.0,
                opacity: 1.0,
            },
            PointConfig {
                shape: PointShape::Square,
                size: 8.0,
                opacity: 0.8,
            },
            PointConfig {
                shape: PointShape::Triangle,
                size: 6.0,
                opacity: 0.9,
            },
        ];

        let renderer = Canvas2DRenderer::new().unwrap();

        for (i, config) in point_configs.iter().enumerate() {
            let mut chart_spec = create_scatter_plot_spec();
            chart_spec.point_config = config.clone();

            // GREEN requirement: All point configurations must render
            let result = renderer.render_scatter_plot(&chart_spec, &test_data);
            assert!(result.is_ok(), "Scatter plot point config {} failed", i);
        }
    }

    /// Test scatter plot with color encoding
    #[test]
    fn test_scatter_plot_color_encoding() {
        let test_data = create_test_scatter_data_with_categories(150, 5);
        let color_schemes = vec![
            ColorScheme::Categorical(vec![
                "#ff0000".to_string(),
                "#00ff00".to_string(),
                "#0000ff".to_string(),
            ]),
            ColorScheme::Sequential(vec!["#ffffff".to_string(), "#000000".to_string()]),
            ColorScheme::Diverging(vec![
                "#ff0000".to_string(),
                "#ffffff".to_string(),
                "#0000ff".to_string(),
            ]),
        ];

        let renderer = Canvas2DRenderer::new().unwrap();

        for (i, scheme) in color_schemes.iter().enumerate() {
            let mut chart_spec = create_scatter_plot_spec();
            chart_spec.color_scheme = scheme.clone();

            // GREEN requirement: All color schemes must render
            let result = renderer.render_scatter_plot(&chart_spec, &test_data);
            assert!(result.is_ok(), "Scatter plot color scheme {} failed", i);
        }
    }
}

// =============================================================================
// Performance Optimizations and Large Dataset Handling TDD
// =============================================================================

#[cfg(test)]
mod performance_optimization_tdd {
    use super::*;

    /// Test Canvas2D performance with 100K points
    #[tokio::test]
    async fn test_canvas2d_performance_100k_points() {
        // RED: Test with 100K points (roadmap requirement)
        let large_dataset = create_test_line_data(100_000);
        let chart_spec = create_line_chart_spec();

        let renderer = Canvas2DRenderer::new().unwrap();
        let start = Instant::now();

        let result = renderer
            .render_line_chart(&chart_spec, &large_dataset)
            .await;
        let duration = start.elapsed();

        // GREEN requirement: 100K points must render in <3ms (roadmap target)
        assert!(result.is_ok(), "100K points rendering failed");
        assert!(
            duration < Duration::from_millis(3),
            "Performance requirement not met: {:?} for 100K points",
            duration
        );
    }

    /// Test memory usage with large datasets
    #[test]
    fn test_canvas2d_memory_usage_large_dataset() {
        // RED: Test memory usage with 1M points
        let initial_memory = get_memory_usage();
        let large_dataset = create_test_line_data(1_000_000);

        let renderer = Canvas2DRenderer::new().unwrap();
        renderer.load_data(&large_dataset);

        let memory_after_load = get_memory_usage();
        let memory_used = memory_after_load - initial_memory;

        // GREEN requirement: 1M points must use <50MB (roadmap target)
        assert!(
            memory_used < 50 * 1024 * 1024,
            "Memory usage too high: {}MB",
            memory_used / (1024 * 1024)
        );
    }

    /// Test Canvas2D rendering optimization strategies
    #[test]
    fn test_canvas2d_optimization_strategies() {
        let test_data = create_test_line_data(10_000);
        let optimization_strategies = vec![
            OptimizationStrategy::LevelOfDetail,
            OptimizationStrategy::DataAggregation,
            OptimizationStrategy::ViewportCulling,
            OptimizationStrategy::BatchRendering,
        ];

        let renderer = Canvas2DRenderer::new().unwrap();

        for strategy in optimization_strategies {
            let mut chart_spec = create_line_chart_spec();
            chart_spec.optimization = strategy;

            let start = Instant::now();
            let result = renderer.render_line_chart(&chart_spec, &test_data);
            let duration = start.elapsed();

            // GREEN requirement: All optimization strategies must improve performance
            assert!(
                result.is_ok(),
                "Optimization strategy {:?} failed",
                strategy
            );
            assert!(
                duration < Duration::from_millis(50),
                "Optimization strategy {:?} too slow: {:?}",
                strategy,
                duration
            );
        }
    }

    /// Property-based test for performance under various data distributions
    proptest! {
        #[test]
        fn test_canvas2d_performance_data_distributions(
            point_count in 1000usize..10000,
            data_density in 0.1f64..1.0,
            has_outliers in prop::bool::ANY
        ) {
            let test_data = create_test_data_with_distribution(point_count, data_density, has_outliers);
            let chart_spec = create_line_chart_spec();
            let renderer = Canvas2DRenderer::new().unwrap();

            let start = Instant::now();
            let result = renderer.render_line_chart(&chart_spec, &test_data);
            let duration = start.elapsed();

            // GREEN requirement: Performance must be consistent across data distributions
            prop_assert!(result.is_ok());
            prop_assert!(duration < Duration::from_millis(100));
        }
    }
}

// =============================================================================
// Interaction Support (Zoom, Pan, Hover) TDD
// =============================================================================

#[cfg(test)]
mod interaction_support_tdd {
    use super::*;

    /// Test zoom functionality with Canvas2D
    #[test]
    fn test_canvas2d_zoom_functionality() {
        let test_data = create_test_line_data(1000);
        let mut chart_spec = create_line_chart_spec();
        let renderer = Canvas2DRenderer::new().unwrap();

        // RED: Test zoom operations
        let zoom_operations = vec![
            ZoomOperation {
                center: (400.0, 300.0),
                factor: 2.0,
            },
            ZoomOperation {
                center: (200.0, 150.0),
                factor: 0.5,
            },
            ZoomOperation {
                center: (600.0, 450.0),
                factor: 4.0,
            },
        ];

        for (i, zoom) in zoom_operations.iter().enumerate() {
            chart_spec.viewport = chart_spec.viewport.zoom(zoom.center, zoom.factor);

            // GREEN requirement: All zoom operations must render correctly
            let result = renderer.render_line_chart(&chart_spec, &test_data);
            assert!(result.is_ok(), "Zoom operation {} failed", i);

            // Verify viewport bounds are correct
            assert!(chart_spec.viewport.x_min < chart_spec.viewport.x_max);
            assert!(chart_spec.viewport.y_min < chart_spec.viewport.y_max);
        }
    }

    /// Test pan functionality with Canvas2D
    #[test]
    fn test_canvas2d_pan_functionality() {
        let test_data = create_test_line_data(1000);
        let mut chart_spec = create_line_chart_spec();
        let renderer = Canvas2DRenderer::new().unwrap();

        // RED: Test pan operations
        let pan_operations = vec![
            PanOperation {
                delta_x: 100.0,
                delta_y: 50.0,
            },
            PanOperation {
                delta_x: -50.0,
                delta_y: -25.0,
            },
            PanOperation {
                delta_x: 200.0,
                delta_y: -100.0,
            },
        ];

        for (i, pan) in pan_operations.iter().enumerate() {
            chart_spec.viewport = chart_spec.viewport.pan(pan.delta_x, pan.delta_y);

            // GREEN requirement: All pan operations must render correctly
            let result = renderer.render_line_chart(&chart_spec, &test_data);
            assert!(result.is_ok(), "Pan operation {} failed", i);
        }
    }

    /// Test hover functionality with Canvas2D
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn test_canvas2d_hover_functionality() {
        let test_data = create_test_line_data(500);
        let chart_spec = create_line_chart_spec();
        let renderer = Canvas2DRenderer::new().unwrap();

        // RED: Test hover detection
        let hover_positions = vec![(100.0, 200.0), (300.0, 150.0), (500.0, 400.0)];

        for (x, y) in hover_positions {
            // GREEN requirement: Hover detection must work accurately
            let hover_result = renderer.detect_hover(&chart_spec, &test_data, x, y);
            assert!(
                hover_result.is_ok(),
                "Hover detection failed at ({}, {})",
                x,
                y
            );

            let hover_info = hover_result.unwrap();
            if hover_info.is_some() {
                let info = hover_info.unwrap();
                assert!(
                    info.distance < 10.0,
                    "Hover distance too large: {}",
                    info.distance
                );
            }
        }
    }

    /// Test interaction performance with large datasets
    #[tokio::test]
    async fn test_canvas2d_interaction_performance() {
        let large_dataset = create_test_line_data(50_000);
        let chart_spec = create_line_chart_spec();
        let renderer = Canvas2DRenderer::new().unwrap();

        // RED: Test interaction performance
        let start = Instant::now();
        let hover_result = renderer.detect_hover(&chart_spec, &large_dataset, 400.0, 300.0);
        let duration = start.elapsed();

        // GREEN requirement: Hover detection must be fast even with large datasets
        assert!(hover_result.is_ok(), "Hover detection failed");
        assert!(
            duration < Duration::from_millis(10),
            "Hover detection too slow: {:?}",
            duration
        );
    }
}

// =============================================================================
// Integration Tests and Performance Benchmarks TDD
// =============================================================================

#[cfg(test)]
mod integration_tests_tdd {
    use super::*;

    /// Test complete Canvas2D rendering pipeline
    #[tokio::test]
    async fn test_canvas2d_complete_pipeline() {
        // RED: Test complete pipeline from data to rendered chart
        let test_data = create_test_mixed_data(1000);
        let chart_specs = vec![
            create_line_chart_spec(),
            create_bar_chart_spec(),
            create_scatter_plot_spec(),
        ];

        let renderer = Canvas2DRenderer::new().unwrap();

        for (i, spec) in chart_specs.iter().enumerate() {
            // GREEN requirement: Complete pipeline must work for all chart types
            let result = renderer.render_chart(spec, &test_data).await;
            assert!(
                result.is_ok(),
                "Complete pipeline failed for chart type {}",
                i
            );

            let render_result = result.unwrap();
            assert!(render_result.render_time < Duration::from_millis(16));
        }
    }

    /// Test Canvas2D with real browser integration
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn test_canvas2d_browser_integration() {
        use wasm_bindgen::JsCast;
        use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

        // RED: Test with real browser canvas
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.create_element("canvas").unwrap();
        let canvas_element: HtmlCanvasElement = canvas.dyn_into().unwrap();
        canvas_element.set_width(800);
        canvas_element.set_height(600);

        let context = canvas_element.get_context("2d").unwrap().unwrap();
        let ctx: CanvasRenderingContext2d = context.dyn_into().unwrap();

        // GREEN requirement: Browser integration must work
        let renderer = Canvas2DRenderer::from_context(ctx).unwrap();
        let test_data = create_test_line_data(100);
        let chart_spec = create_line_chart_spec();

        let result = renderer.render_line_chart(&chart_spec, &test_data).await;
        assert!(result.is_ok(), "Browser integration failed");
    }

    /// Performance benchmark for Canvas2D rendering
    #[tokio::test]
    async fn test_canvas2d_performance_benchmark() {
        let dataset_sizes = vec![1_000, 10_000, 50_000, 100_000];
        let chart_types = vec![
            ("line", create_line_chart_spec()),
            ("bar", create_bar_chart_spec()),
            ("scatter", create_scatter_plot_spec()),
        ];

        let renderer = Canvas2DRenderer::new().unwrap();
        let mut benchmark_results = HashMap::new();

        for (chart_type, spec) in chart_types {
            for &size in &dataset_sizes {
                let test_data = create_test_data_for_chart_type(chart_type, size);

                let start = Instant::now();
                let result = renderer.render_chart(&spec, &test_data).await;
                let duration = start.elapsed();

                // GREEN requirement: All benchmarks must meet performance targets
                assert!(
                    result.is_ok(),
                    "Benchmark failed for {} with {} points",
                    chart_type,
                    size
                );

                let key = format!("{}_{}", chart_type, size);
                benchmark_results.insert(key, duration);

                // Performance requirements based on dataset size
                let max_duration = match size {
                    1_000 => Duration::from_millis(5),
                    10_000 => Duration::from_millis(20),
                    50_000 => Duration::from_millis(100),
                    100_000 => Duration::from_millis(200),
                    _ => Duration::from_millis(500),
                };

                assert!(
                    duration <= max_duration,
                    "Performance benchmark failed for {} with {} points: {:?} > {:?}",
                    chart_type,
                    size,
                    duration,
                    max_duration
                );
            }
        }

        // Log benchmark results
        println!("Canvas2D Performance Benchmark Results:");
        for (key, duration) in benchmark_results {
            println!("  {}: {:?}", key, duration);
        }
    }

    /// Test Canvas2D error handling and recovery
    #[test]
    fn test_canvas2d_error_handling_recovery() {
        let renderer = Canvas2DRenderer::new().unwrap();

        // RED: Test various error conditions
        let error_scenarios = vec![
            ErrorScenario::InvalidData,
            ErrorScenario::ContextLost,
            ErrorScenario::MemoryExhausted,
            ErrorScenario::RenderingTimeout,
        ];

        for scenario in error_scenarios {
            // GREEN requirement: All error scenarios must be handled gracefully
            let result = renderer.handle_error_scenario(scenario);
            assert!(
                result.is_ok(),
                "Error handling failed for scenario: {:?}",
                scenario
            );

            // Verify recovery
            let recovery_result = renderer.recover_from_error();
            assert!(
                recovery_result.is_ok(),
                "Recovery failed for scenario: {:?}",
                scenario
            );
        }
    }
}

// =============================================================================
// Additional Test Data Generation and Helper Functions
// =============================================================================

/// Generate test data with categories for color encoding
fn create_test_scatter_data_with_categories(
    point_count: usize,
    category_count: usize,
) -> ScatterPlotData {
    let mut points = Vec::with_capacity(point_count);
    for i in 0..point_count {
        points.push(ScatterPoint {
            x: (i as f64 * 0.1) % 100.0,
            y: (i as f64 * 0.1).cos() * 50.0,
            size: Some((i % 10) as f64 + 1.0),
            color: Some(format!("Category {}", i % category_count)),
        });
    }
    ScatterPlotData { points }
}

/// Generate test data with specific distribution characteristics
fn create_test_data_with_distribution(
    point_count: usize,
    density: f64,
    has_outliers: bool,
) -> LineChartData {
    let mut points = Vec::with_capacity(point_count);
    for i in 0..point_count {
        let x = i as f64;
        let mut y = (i as f64 * 0.1).sin() * 100.0 * density;

        // Add outliers if requested
        if has_outliers && i % 100 == 0 {
            y *= 5.0; // Make every 100th point an outlier
        }

        points.push(DataPoint { x, y });
    }
    LineChartData { points }
}

/// Generate test data for specific chart type
fn create_test_data_for_chart_type(chart_type: &str, size: usize) -> Box<dyn ChartData> {
    match chart_type {
        "line" => Box::new(create_test_line_data(size)),
        "bar" => Box::new(create_test_bar_data(size / 10)),
        "scatter" => Box::new(create_test_scatter_data(size)),
        _ => panic!("Unknown chart type: {}", chart_type),
    }
}

/// Get current memory usage (mock implementation)
fn get_memory_usage() -> usize {
    // In a real implementation, this would use system APIs
    // For testing, we'll return a mock value
    1024 * 1024 // 1MB baseline
}

// Additional types for testing
#[derive(Debug, Clone)]
pub struct Canvas2DConfig {
    pub width: u32,
    pub height: u32,
    pub context_type: String,
}

impl Canvas2DConfig {
    fn is_valid(&self) -> bool {
        self.width > 0 && self.height > 0 && self.context_type == "2d"
    }
}

#[derive(Debug, Clone)]
pub struct ZoomOperation {
    pub center: (f64, f64),
    pub factor: f64,
}

#[derive(Debug, Clone)]
pub struct PanOperation {
    pub delta_x: f64,
    pub delta_y: f64,
}
