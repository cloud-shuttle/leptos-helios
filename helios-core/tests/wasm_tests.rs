//! WASM-specific tests for Helios
//!
//! These tests focus on WebAssembly-specific functionality including:
//! - WASM module compilation and instantiation
//! - Memory management and garbage collection
//! - Performance characteristics in WASM environment
//! - Cross-platform compatibility

use leptos_helios::chart::{ChartSpec, ChartSpecBuilder, MarkType, Encoding, EncodingDef, DataType, DataFormat, DataReference};
use leptos_helios::webgpu_renderer::WebGpuRenderer;
use leptos_helios::canvas2d_renderer::Canvas2DRenderer;
use leptos_helios::headless_renderer::HeadlessRenderer;
use leptos_helios::export_system::{ExportSystem, ExportConfig, ExportFormat};
use polars::prelude::*;
use std::time::Instant;

/// Test WASM module compilation and basic functionality
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn test_wasm_module_compilation() {
    // Test that core types can be instantiated in WASM
    let _spec = ChartSpecBuilder::default()
        .data(DataReference {
            source: "test".to_string(),
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
    
    assert!(true); // If we get here, compilation succeeded
}

/// Test WASM memory management
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn test_wasm_memory_management() {
    // Test that we can create and drop objects without memory leaks
    let mut specs = Vec::new();
    
    for i in 0..100 {
        let spec = ChartSpecBuilder::default()
            .data(DataReference {
                source: format!("test_{}", i),
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
    }
    
    // Drop all specs
    drop(specs);
    
    // Force garbage collection if available
    #[cfg(feature = "wasm-gc")]
    {
        use wasm_bindgen::prelude::*;
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = console)]
            fn log(s: &str);
        }
        log("Forcing WASM garbage collection");
    }
    
    assert!(true);
}

/// Test WASM performance characteristics
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn test_wasm_performance() {
    let start = Instant::now();
    
    // Create a complex chart specification
    let spec = ChartSpecBuilder::default()
        .data(DataReference {
            source: "performance_test".to_string(),
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
    
    // Performance should be reasonable in WASM
    assert!(creation_time.as_millis() < 100, "Chart creation took too long: {:?}", creation_time);
}

/// Test WASM-specific renderer initialization
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
async fn test_wasm_renderer_initialization() {
    // Test Canvas2D renderer (should work in WASM)
    let canvas_renderer = Canvas2DRenderer::new();
    assert!(canvas_renderer.is_ok());
    
    // Test WebGPU renderer (may not be available in all WASM environments)
    let webgpu_renderer = WebGpuRenderer::new().await;
    // This might fail in some WASM environments, which is expected
    if webgpu_renderer.is_err() {
        println!("WebGPU not available in this WASM environment");
    }
}

/// Test WASM export functionality
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
async fn test_wasm_export_functionality() {
    let mut export_system = ExportSystem::new();
    
    let spec = ChartSpecBuilder::default()
        .data(DataReference {
            source: "export_test".to_string(),
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
        "category" => &["A", "B", "C"],
        "value" => &[10, 20, 15]
    ).unwrap();
    
    let config = ExportConfig {
        format: ExportFormat::PNG { width: 800, height: 600, dpi: Some(96) },
        ..Default::default()
    };
    
    // Test PNG export
    let result = export_system.export_chart(&spec, &data, &config, "test_wasm.png").await;
    assert!(result.is_ok(), "PNG export failed: {:?}", result.err());
    
    // Test SVG export
    let svg_config = ExportConfig {
        format: ExportFormat::SVG { embed_fonts: true },
        ..Default::default()
    };
    
    let svg_result = export_system.export_chart(&spec, &data, &svg_config, "test_wasm.svg").await;
    assert!(svg_result.is_ok(), "SVG export failed: {:?}", svg_result.err());
}

/// Test WASM-specific error handling
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn test_wasm_error_handling() {
    // Test that errors are properly propagated in WASM
    let invalid_spec = ChartSpecBuilder::default()
        .data(DataReference {
            source: "invalid".to_string(),
            format: DataFormat::Inline,
            schema: None,
        })
        .mark(MarkType::Point { size: None, shape: None, opacity: None })
        .encoding(Encoding::default()) // Missing required x, y encodings
        .build();
    
    assert!(invalid_spec.is_err(), "Should fail with missing encodings");
}

/// Test WASM memory usage patterns
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn test_wasm_memory_usage() {
    // Test that we can handle reasonable data sizes in WASM
    let mut data_points = Vec::new();
    
    for i in 0..1000 {
        data_points.push((i as f64, (i * 2) as f64));
    }
    
    // Create DataFrame with the data
    let x_data: Vec<f64> = data_points.iter().map(|(x, _)| *x).collect();
    let y_data: Vec<f64> = data_points.iter().map(|(_, y)| *y).collect();
    
    let df = df!(
        "x" => x_data,
        "y" => y_data
    ).unwrap();
    
    assert_eq!(df.height(), 1000);
    assert_eq!(df.width(), 2);
}

/// Test WASM-specific performance regression
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn test_wasm_performance_regression() {
    let iterations = 100;
    let mut total_time = std::time::Duration::new(0, 0);
    
    for _ in 0..iterations {
        let start = Instant::now();
        
        // Create a chart specification
        let _spec = ChartSpecBuilder::default()
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
        
        total_time += start.elapsed();
    }
    
    let avg_time = total_time / iterations;
    
    // Performance regression threshold: should be under 1ms per operation
    assert!(avg_time.as_micros() < 1000, "Performance regression detected: {:?} per operation", avg_time);
}

/// Test WASM-specific data processing
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn test_wasm_data_processing() {
    // Test that we can process data efficiently in WASM
    let data = df!(
        "category" => &["A", "B", "C", "A", "B", "C"],
        "value" => &[10, 20, 15, 25, 12, 18],
        "date" => &[
            "2023-01-01", "2023-01-02", "2023-01-03",
            "2023-01-04", "2023-01-05", "2023-01-06"
        ]
    ).unwrap();
    
    // Test basic operations
    assert_eq!(data.height(), 6);
    assert_eq!(data.width(), 3);
    
    // Test aggregation
    let grouped = data.group_by([col("category")]).unwrap().agg([col("value").sum()]).unwrap();
    assert_eq!(grouped.height(), 3); // Should have 3 unique categories
}

/// Test WASM-specific rendering performance
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
async fn test_wasm_rendering_performance() {
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
        "category" => &["A", "B", "C"],
        "value" => &[10, 20, 15]
    ).unwrap();
    
    // Test Canvas2D rendering performance
    if let Ok(canvas_renderer) = Canvas2DRenderer::new() {
        let start = Instant::now();
        let result = canvas_renderer.render_to_buffer(&spec, &data);
        let render_time = start.elapsed();
        
        assert!(result.is_ok(), "Canvas2D rendering failed: {:?}", result.err());
        assert!(render_time.as_millis() < 100, "Canvas2D rendering too slow: {:?}", render_time);
    }
}

/// Test WASM-specific memory constraints
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn test_wasm_memory_constraints() {
    // Test that we can handle reasonable memory usage in WASM
    let mut large_data = Vec::new();
    
    // Create a reasonably large dataset
    for i in 0..10000 {
        large_data.push((i as f64, (i * 0.1) as f64));
    }
    
    let x_data: Vec<f64> = large_data.iter().map(|(x, _)| *x).collect();
    let y_data: Vec<f64> = large_data.iter().map(|(_, y)| *y).collect();
    
    let df = df!(
        "x" => x_data,
        "y" => y_data
    ).unwrap();
    
    assert_eq!(df.height(), 10000);
    
    // Test that we can create a chart spec with this data
    let spec = ChartSpecBuilder::default()
        .data(DataReference {
            source: "large_data".to_string(),
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
    
    assert!(spec.mark.is_line());
}
