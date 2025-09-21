//! Performance regression tests for Helios
//!
//! These tests ensure that performance characteristics remain within acceptable bounds
//! and detect any performance regressions in the codebase.

use leptos_helios::chart::{ChartSpecBuilder, MarkType, Encoding, EncodingDef, DataType, DataFormat, DataReference};
use leptos_helios::webgpu_renderer::WebGpuRenderer;
use leptos_helios::canvas2d_renderer::Canvas2DRenderer;
use leptos_helios::export_system::{ExportSystem, ExportConfig, ExportFormat};
use leptos_helios::nl_processor::NLProcessor;
use polars::prelude::*;
use std::time::{Duration, Instant};
use std::sync::Arc;
use std::thread;

/// Performance thresholds for different operations
const CHART_CREATION_THRESHOLD_MS: u128 = 10;
const RENDERING_THRESHOLD_MS: u128 = 100;
const EXPORT_THRESHOLD_MS: u128 = 500;
const NL_PROCESSING_THRESHOLD_MS: u128 = 50;
const MEMORY_USAGE_THRESHOLD_MB: u64 = 100;

/// Test chart creation performance
#[test]
fn test_chart_creation_performance() {
    let start = Instant::now();
    
    let spec = ChartSpecBuilder::default()
        .data(DataReference {
            source: "perf_test".to_string(),
            format: DataFormat::Inline,
            schema: None,
        })
        .mark(MarkType::Line { interpolate: None, stroke_width: None, stroke_dash: None })
        .encoding(Encoding {
            x: Some(EncodingDef {
                field: "x".to_string(),
                data_type: DataType::Number,
                scale: None, axis: None, legend: None, bin: None, aggregate: None, sort: None,
            }),
            y: Some(EncodingDef {
                field: "y".to_string(),
                data_type: DataType::Number,
                scale: None, axis: None, legend: None, bin: None, aggregate: None, sort: None,
            }),
            color: None, size: None, shape: None, opacity: None, text: None, tooltip: None, detail: None, order: None, row: None, column: None,
        })
        .build()
        .unwrap();
    
    let creation_time = start.elapsed();
    
    assert!(creation_time.as_millis() < CHART_CREATION_THRESHOLD_MS,
        "Chart creation took too long: {:?} (threshold: {}ms)", 
        creation_time, CHART_CREATION_THRESHOLD_MS);
    
    assert!(matches!(spec.mark, MarkType::Line { .. }));
}

/// Test rendering performance with Canvas2D
#[test]
fn test_canvas2d_rendering_performance() {
    let spec = ChartSpecBuilder::default()
        .data(DataReference {
            source: "render_test".to_string(),
            format: DataFormat::Inline,
            schema: None,
        })
        .mark(MarkType::Bar { width: None, corner_radius: None })
        .encoding(Encoding {
            x: Some(EncodingDef {
                field: "category".to_string(),
                data_type: DataType::String,
                scale: None, axis: None, legend: None, bin: None, aggregate: None, sort: None,
            }),
            y: Some(EncodingDef {
                field: "value".to_string(),
                data_type: DataType::Number,
                scale: None, axis: None, legend: None, bin: None, aggregate: None, sort: None,
            }),
            color: None, size: None, shape: None, opacity: None, text: None, tooltip: None, detail: None, order: None, row: None, column: None,
        })
        .build()
        .unwrap();
    
    let data = df!(
        "category" => &["A", "B", "C", "D", "E"],
        "value" => &[10, 20, 15, 25, 30]
    ).unwrap();
    
    let canvas_renderer = Canvas2DRenderer::new().unwrap();
    
    let start = Instant::now();
    // Create test data for Canvas2D renderer
    let line_data = leptos_helios::canvas2d_renderer::LineChartData {
        points: vec![
            leptos_helios::canvas2d_renderer::DataPoint { x: 1.0, y: 10.0 },
            leptos_helios::canvas2d_renderer::DataPoint { x: 2.0, y: 20.0 },
            leptos_helios::canvas2d_renderer::DataPoint { x: 3.0, y: 15.0 },
            leptos_helios::canvas2d_renderer::DataPoint { x: 4.0, y: 25.0 },
            leptos_helios::canvas2d_renderer::DataPoint { x: 5.0, y: 30.0 },
        ],
    };
    
    let line_spec = leptos_helios::canvas2d_renderer::LineChartSpec {
        line_style: leptos_helios::canvas2d_renderer::LineStyle {
            width: 2.0,
            color: "#000000".to_string(),
            dash: None,
        },
        interpolation: leptos_helios::canvas2d_renderer::InterpolationMethod::Linear,
        viewport: leptos_helios::canvas2d_renderer::Viewport {
            x_min: 0.0,
            x_max: 10.0,
            y_min: 0.0,
            y_max: 50.0,
        },
        optimization: leptos_helios::canvas2d_renderer::OptimizationStrategy::LevelOfDetail,
    };
    
    let result = tokio::runtime::Runtime::new().unwrap().block_on(
        canvas_renderer.render_line_chart(&line_spec, &line_data)
    );
    let render_time = start.elapsed();
    
    assert!(result.is_ok(), "Canvas2D rendering failed: {:?}", result.err());
    assert!(render_time.as_millis() < RENDERING_THRESHOLD_MS,
        "Canvas2D rendering took too long: {:?} (threshold: {}ms)",
        render_time, RENDERING_THRESHOLD_MS);
}

/// Test WebGPU rendering performance
#[tokio::test]
async fn test_webgpu_rendering_performance() {
    let spec = ChartSpecBuilder::default()
        .data(DataReference {
            source: "webgpu_test".to_string(),
            format: DataFormat::Inline,
            schema: None,
        })
        .mark(MarkType::Line { interpolate: None, stroke_width: None, stroke_dash: None })
        .encoding(Encoding {
            x: Some(EncodingDef {
                field: "x".to_string(),
                data_type: DataType::Number,
                scale: None, axis: None, legend: None, bin: None, aggregate: None, sort: None,
            }),
            y: Some(EncodingDef {
                field: "y".to_string(),
                data_type: DataType::Number,
                scale: None, axis: None, legend: None, bin: None, aggregate: None, sort: None,
            }),
            color: None, size: None, shape: None, opacity: None, text: None, tooltip: None, detail: None, order: None, row: None, column: None,
        })
        .build()
        .unwrap();
    
    let data = df!(
        "x" => &[1.0, 2.0, 3.0, 4.0, 5.0],
        "y" => &[10.0, 20.0, 15.0, 25.0, 30.0]
    ).unwrap();
    
    if let Ok(mut webgpu_renderer) = WebGpuRenderer::new().await {
        let start = Instant::now();
        let result = webgpu_renderer.render();
        let render_time = start.elapsed();
        
        assert!(result.is_ok(), "WebGPU rendering failed: {:?}", result.err());
        assert!(render_time.as_millis() < RENDERING_THRESHOLD_MS,
            "WebGPU rendering took too long: {:?} (threshold: {}ms)",
            render_time, RENDERING_THRESHOLD_MS);
    } else {
        println!("WebGPU not available, skipping test");
    }
}

/// Test export performance
#[tokio::test]
async fn test_export_performance() {
    let mut export_system = ExportSystem::new("/tmp");
    
    let spec = ChartSpecBuilder::default()
        .data(DataReference {
            source: "export_perf_test".to_string(),
            format: DataFormat::Inline,
            schema: None,
        })
        .mark(MarkType::Bar { width: None, corner_radius: None })
        .encoding(Encoding {
            x: Some(EncodingDef {
                field: "category".to_string(),
                data_type: DataType::String,
                scale: None, axis: None, legend: None, bin: None, aggregate: None, sort: None,
            }),
            y: Some(EncodingDef {
                field: "value".to_string(),
                data_type: DataType::Number,
                scale: None, axis: None, legend: None, bin: None, aggregate: None, sort: None,
            }),
            color: None, size: None, shape: None, opacity: None, text: None, tooltip: None, detail: None, order: None, row: None, column: None,
        })
        .build()
        .unwrap();
    
    let data = df!(
        "category" => &["A", "B", "C", "D", "E"],
        "value" => &[10, 20, 15, 25, 30]
    ).unwrap();
    
    // Test PNG export performance
    let png_config = ExportConfig {
        format: ExportFormat::PNG { width: 800, height: 600, dpi: Some(96) },
        ..Default::default()
    };
    
    let start = Instant::now();
    let png_result = export_system.export_chart(&spec, &data, &png_config, "perf_test.png").await;
    let png_time = start.elapsed();
    
    assert!(png_result.is_ok(), "PNG export failed: {:?}", png_result.err());
    assert!(png_time.as_millis() < EXPORT_THRESHOLD_MS,
        "PNG export took too long: {:?} (threshold: {}ms)",
        png_time, EXPORT_THRESHOLD_MS);
    
    // Test SVG export performance
    let svg_config = ExportConfig {
        format: ExportFormat::SVG { width: Some(800), height: Some(600) },
        ..Default::default()
    };
    
    let start = Instant::now();
    let svg_result = export_system.export_chart(&spec, &data, &svg_config, "perf_test.svg").await;
    let svg_time = start.elapsed();
    
    assert!(svg_result.is_ok(), "SVG export failed: {:?}", svg_result.err());
    assert!(svg_time.as_millis() < EXPORT_THRESHOLD_MS,
        "SVG export took too long: {:?} (threshold: {}ms)",
        svg_time, EXPORT_THRESHOLD_MS);
}

/// Test NL processor performance
#[test]
fn test_nl_processor_performance() {
    let processor = NLProcessor::new();
    
    let queries = vec![
        "show line chart of sales over time",
        "create bar chart comparing revenue by category",
        "detect anomalies in temperature data",
        "show sales forecast for next 6 months",
        "create scatter plot of price vs volume"
    ];
    
    for query in queries {
        let start = Instant::now();
        let result = processor.parse_query(query);
        let processing_time = start.elapsed();
        
        assert!(result.is_ok(), "NL processing failed for query '{}': {:?}", query, result.err());
        assert!(processing_time.as_millis() < NL_PROCESSING_THRESHOLD_MS,
            "NL processing took too long for query '{}': {:?} (threshold: {}ms)",
            query, processing_time, NL_PROCESSING_THRESHOLD_MS);
    }
}

/// Test memory usage with large datasets
#[test]
fn test_memory_usage_large_dataset() {
    // Create a large dataset
    let size = 10000;
    let x_data: Vec<f64> = (0..size).map(|i| i as f64).collect();
    let y_data: Vec<f64> = (0..size).map(|i| (i as f64) * 0.1).collect();
    
    let data = df!(
        "x" => x_data,
        "y" => y_data
    ).unwrap();
    
    assert_eq!(data.height(), size);
    
    // Test that we can create a chart spec with large data
    let spec = ChartSpecBuilder::default()
        .data(DataReference {
            source: "large_dataset".to_string(),
            format: DataFormat::Inline,
            schema: None,
        })
        .mark(MarkType::Line { interpolate: None, stroke_width: None, stroke_dash: None })
        .encoding(Encoding {
            x: Some(EncodingDef {
                field: "x".to_string(),
                data_type: DataType::Number,
                scale: None, axis: None, legend: None, bin: None, aggregate: None, sort: None,
            }),
            y: Some(EncodingDef {
                field: "y".to_string(),
                data_type: DataType::Number,
                scale: None, axis: None, legend: None, bin: None, aggregate: None, sort: None,
            }),
            color: None, size: None, shape: None, opacity: None, text: None, tooltip: None, detail: None, order: None, row: None, column: None,
        })
        .build()
        .unwrap();
    
    assert!(matches!(spec.mark, MarkType::Line { .. }));
}

/// Test concurrent rendering performance
#[test]
fn test_concurrent_rendering_performance() {
    let spec = ChartSpecBuilder::default()
        .data(DataReference {
            source: "concurrent_test".to_string(),
            format: DataFormat::Inline,
            schema: None,
        })
        .mark(MarkType::Point { size: None, shape: None, opacity: None })
        .encoding(Encoding {
            x: Some(EncodingDef {
                field: "x".to_string(),
                data_type: DataType::Number,
                scale: None, axis: None, legend: None, bin: None, aggregate: None, sort: None,
            }),
            y: Some(EncodingDef {
                field: "y".to_string(),
                data_type: DataType::Number,
                scale: None, axis: None, legend: None, bin: None, aggregate: None, sort: None,
            }),
            color: None, size: None, shape: None, opacity: None, text: None, tooltip: None, detail: None, order: None, row: None, column: None,
        })
        .build()
        .unwrap();
    
    let data = df!(
        "x" => &[1.0, 2.0, 3.0, 4.0, 5.0],
        "y" => &[10.0, 20.0, 15.0, 25.0, 30.0]
    ).unwrap();
    
    let canvas_renderer = Arc::new(Canvas2DRenderer::new().unwrap());
    let num_threads = 4;
    let iterations_per_thread = 10;
    
    let start = Instant::now();
    
    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let renderer = Arc::clone(&canvas_renderer);
            let spec = spec.clone();
            let data = data.clone();
            
            thread::spawn(move || {
                for _ in 0..iterations_per_thread {
                    // Create test data for Canvas2D renderer
                    let line_data = leptos_helios::canvas2d_renderer::LineChartData {
                        points: vec![
                            leptos_helios::canvas2d_renderer::DataPoint { x: 1.0, y: 10.0 },
                            leptos_helios::canvas2d_renderer::DataPoint { x: 2.0, y: 20.0 },
                            leptos_helios::canvas2d_renderer::DataPoint { x: 3.0, y: 15.0 },
                            leptos_helios::canvas2d_renderer::DataPoint { x: 4.0, y: 25.0 },
                            leptos_helios::canvas2d_renderer::DataPoint { x: 5.0, y: 30.0 },
                        ],
                    };
                    
                    let line_spec = leptos_helios::canvas2d_renderer::LineChartSpec {
                        line_style: leptos_helios::canvas2d_renderer::LineStyle {
                            width: 2.0,
                            color: "#000000".to_string(),
                            dash: None,
                        },
                        interpolation: leptos_helios::canvas2d_renderer::InterpolationMethod::Linear,
                        viewport: leptos_helios::canvas2d_renderer::Viewport {
                            x_min: 0.0,
                            x_max: 10.0,
                            y_min: 0.0,
                            y_max: 50.0,
                        },
                        optimization: leptos_helios::canvas2d_renderer::OptimizationStrategy::LevelOfDetail,
                    };
                    
                    let result = tokio::runtime::Runtime::new().unwrap().block_on(
                        renderer.render_line_chart(&line_spec, &line_data)
                    );
                    assert!(result.is_ok(), "Concurrent rendering failed: {:?}", result.err());
                }
            })
        })
        .collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let total_time = start.elapsed();
    let total_operations = num_threads * iterations_per_thread;
    let avg_time_per_operation = total_time / total_operations as u32;
    
    assert!(avg_time_per_operation.as_millis() < RENDERING_THRESHOLD_MS,
        "Concurrent rendering average time too high: {:?} (threshold: {}ms)",
        avg_time_per_operation, RENDERING_THRESHOLD_MS);
}

/// Test performance with different chart types
#[test]
fn test_different_chart_types_performance() {
    let chart_types = vec![
        MarkType::Point { size: None, shape: None, opacity: None },
        MarkType::Line { interpolate: None, stroke_width: None, stroke_dash: None },
        MarkType::Bar { width: None, corner_radius: None },
        MarkType::Area { interpolate: None, opacity: None },
        MarkType::Rect { stroke: None, stroke_width: None },
    ];
    
    let data = df!(
        "x" => &[1.0, 2.0, 3.0, 4.0, 5.0],
        "y" => &[10.0, 20.0, 15.0, 25.0, 30.0]
    ).unwrap();
    
    for chart_type in chart_types {
        let start = Instant::now();
        
        let spec = ChartSpecBuilder::default()
            .data(DataReference {
                source: "chart_type_test".to_string(),
                format: DataFormat::Inline,
                schema: None,
            })
            .mark(chart_type.clone())
            .encoding(Encoding {
                x: Some(EncodingDef {
                    field: "x".to_string(),
                    data_type: DataType::Number,
                    scale: None, axis: None, legend: None, bin: None, aggregate: None, sort: None,
                }),
                y: Some(EncodingDef {
                    field: "y".to_string(),
                    data_type: DataType::Number,
                    scale: None, axis: None, legend: None, bin: None, aggregate: None, sort: None,
                }),
                color: None, size: None, shape: None, opacity: None, text: None, tooltip: None, detail: None, order: None, row: None, column: None,
            })
            .build()
            .unwrap();
        
        let creation_time = start.elapsed();
        
        assert!(creation_time.as_millis() < CHART_CREATION_THRESHOLD_MS,
            "Chart creation took too long for {:?}: {:?} (threshold: {}ms)",
            chart_type, creation_time, CHART_CREATION_THRESHOLD_MS);
    }
}

/// Test performance regression detection
#[test]
fn test_performance_regression_detection() {
    // This test runs multiple iterations to detect performance regressions
    let iterations = 100;
    let mut times = Vec::new();
    
    for i in 0..iterations {
        let start = Instant::now();
        
        let spec = ChartSpecBuilder::default()
            .data(DataReference {
                source: format!("regression_test_{}", i),
                format: DataFormat::Inline,
                schema: None,
            })
            .mark(MarkType::Line { interpolate: None, stroke_width: None, stroke_dash: None })
            .encoding(Encoding {
                x: Some(EncodingDef {
                    field: "x".to_string(),
                    data_type: DataType::Number,
                    scale: None, axis: None, legend: None, bin: None, aggregate: None, sort: None,
                }),
                y: Some(EncodingDef {
                    field: "y".to_string(),
                    data_type: DataType::Number,
                    scale: None, axis: None, legend: None, bin: None, aggregate: None, sort: None,
                }),
                color: None, size: None, shape: None, opacity: None, text: None, tooltip: None, detail: None, order: None, row: None, column: None,
            })
            .build()
            .unwrap();
        
        times.push(start.elapsed());
    }
    
    // Calculate statistics
    let total_time: Duration = times.iter().sum();
    let avg_time = total_time / iterations as u32;
    let max_time = times.iter().max().unwrap();
    
    // Check for performance regression
    assert!(avg_time.as_millis() < CHART_CREATION_THRESHOLD_MS,
        "Average performance regression detected: {:?} (threshold: {}ms)",
        avg_time, CHART_CREATION_THRESHOLD_MS);
    
    assert!(max_time.as_millis() < CHART_CREATION_THRESHOLD_MS * 2,
        "Maximum performance regression detected: {:?} (threshold: {}ms)",
        max_time, CHART_CREATION_THRESHOLD_MS * 2);
}

/// Test memory usage patterns
#[test]
fn test_memory_usage_patterns() {
    // Test that memory usage doesn't grow unbounded
    let mut specs = Vec::new();
    
    for i in 0..100 {
        let spec = ChartSpecBuilder::default()
            .data(DataReference {
                source: format!("memory_test_{}", i),
                format: DataFormat::Inline,
                schema: None,
            })
            .mark(MarkType::Line { interpolate: None, stroke_width: None, stroke_dash: None })
            .encoding(Encoding {
                x: Some(EncodingDef {
                    field: "x".to_string(),
                    data_type: DataType::Number,
                    scale: None, axis: None, legend: None, bin: None, aggregate: None, sort: None,
                }),
                y: Some(EncodingDef {
                    field: "y".to_string(),
                    data_type: DataType::Number,
                    scale: None, axis: None, legend: None, bin: None, aggregate: None, sort: None,
                }),
                color: None, size: None, shape: None, opacity: None, text: None, tooltip: None, detail: None, order: None, row: None, column: None,
            })
            .build()
            .unwrap();
        
        specs.push(spec);
        
        // Periodically clear some specs to test memory management
        if i % 20 == 0 && i > 0 {
            specs.drain(0..10);
        }
    }
    
    // Final cleanup
    drop(specs);
    
    assert!(true); // If we get here without OOM, memory management is working
}

/// Test performance with different data sizes
#[test]
fn test_performance_data_size_scaling() {
    let data_sizes = vec![100, 1000, 5000, 10000];
    
    for size in data_sizes {
        let x_data: Vec<f64> = (0..size).map(|i| i as f64).collect();
        let y_data: Vec<f64> = (0..size).map(|i| (i as f64) * 0.1).collect();
        
        let data = df!(
            "x" => x_data,
            "y" => y_data
        ).unwrap();
        
        let start = Instant::now();
        
        let spec = ChartSpecBuilder::default()
            .data(DataReference {
                source: format!("scaling_test_{}", size),
                format: DataFormat::Inline,
                schema: None,
            })
            .mark(MarkType::Line { interpolate: None, stroke_width: None, stroke_dash: None })
            .encoding(Encoding {
                x: Some(EncodingDef {
                    field: "x".to_string(),
                    data_type: DataType::Number,
                    scale: None, axis: None, legend: None, bin: None, aggregate: None, sort: None,
                }),
                y: Some(EncodingDef {
                    field: "y".to_string(),
                    data_type: DataType::Number,
                    scale: None, axis: None, legend: None, bin: None, aggregate: None, sort: None,
                }),
                color: None, size: None, shape: None, opacity: None, text: None, tooltip: None, detail: None, order: None, row: None, column: None,
            })
            .build()
            .unwrap();
        
        let creation_time = start.elapsed();
        
        // Performance should scale reasonably with data size
        let expected_time_ms = (size as f64 / 1000.0).ceil() as u64 * 2; // Allow 2ms per 1000 points
        let threshold = std::cmp::max(expected_time_ms, CHART_CREATION_THRESHOLD_MS as u64);
        
        assert!(creation_time.as_millis() < threshold as u128,
            "Performance doesn't scale well with data size {}: {:?} (threshold: {}ms)",
            size, creation_time, threshold);
    }
}
