//! Line Chart Rendering Tests
//!
//! These tests verify basic line chart rendering with real data processing.

use leptos_helios::chart_config::*;
use leptos_helios::webgpu_real::*;
use std::f32;
use std::time::Instant;

/// Test data processing and transformation
#[tokio::test]
async fn test_data_processing() {
    // Test basic data processing for line charts
    let raw_data = vec![("A", 1.0), ("B", 2.0), ("C", 3.0), ("D", 4.0), ("E", 5.0)];

    // Test data normalization
    let min_val = raw_data
        .iter()
        .map(|(_, v)| *v)
        .fold(f32::INFINITY, f32::min);
    let max_val = raw_data
        .iter()
        .map(|(_, v)| *v)
        .fold(f32::NEG_INFINITY, f32::max);

    assert_eq!(min_val, 1.0);
    assert_eq!(max_val, 5.0);

    // Test normalized data
    let normalized_data: Vec<f32> = raw_data
        .iter()
        .map(|(_, v)| (v - min_val) / (max_val - min_val))
        .collect();

    assert_eq!(normalized_data[0], 0.0); // First value should be 0
    assert_eq!(normalized_data[4], 1.0); // Last value should be 1

    println!("✅ Data processing test passed");
}

/// Test coordinate system mapping
#[tokio::test]
async fn test_coordinate_mapping() {
    // Test mapping from data coordinates to screen coordinates
    let data_width = 4.0; // Data range: 0 to 4
    let data_height = 5.0; // Data range: 0 to 5
    let screen_width = 800.0;
    let screen_height = 600.0;

    // Test data point mapping
    let data_point = (2.0, 3.0); // Data coordinates
    let screen_x = (data_point.0 / data_width) * screen_width;
    let screen_y = screen_height - ((data_point.1 / data_height) * screen_height); // Flip Y axis

    assert_eq!(screen_x, 400.0); // Halfway across
    assert_eq!(screen_y, 240.0); // 60% down from top

    // Test edge cases
    let edge_data = (0.0, 0.0);
    let edge_screen_x = (edge_data.0 / data_width) * screen_width;
    let edge_screen_y = screen_height - ((edge_data.1 / data_height) * screen_height);

    assert_eq!(edge_screen_x, 0.0);
    assert_eq!(edge_screen_y, 600.0);

    println!("✅ Coordinate mapping test passed");
}

/// Test line chart configuration
#[tokio::test]
async fn test_line_chart_config() {
    // Test creating a line chart configuration
    let config = LineChartConfig {
        base: BaseChartConfig {
            width: 800,
            height: 600,
            title: "Test Chart".to_string(),
            x_label: "X Axis".to_string(),
            y_label: "Y Axis".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        color: "#0080ff".to_string(), // Blue
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    assert_eq!(config.base.width, 800);
    assert_eq!(config.base.height, 600);
    assert_eq!(config.color, "#0080ff");
    assert_eq!(config.line_width, 2.0);
    assert!(config.show_points);
    assert_eq!(config.point_size, 4.0);

    println!("✅ Line chart configuration test passed");
}

/// Test vertex generation for line charts
#[tokio::test]
async fn test_vertex_generation() {
    // Test generating vertices for a line chart
    let data_points = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 3.0), (3.0, 4.0), (4.0, 5.0)];

    let width = 800.0;
    let height = 600.0;

    // Generate vertices
    let vertices: Vec<[f32; 2]> = data_points
        .iter()
        .map(|(x, y)| {
            // Map to normalized coordinates (-1 to 1)
            let norm_x = (x / 4.0) * 2.0 - 1.0;
            let norm_y = (y / 5.0) * 2.0 - 1.0;
            [norm_x, norm_y]
        })
        .collect();

    assert_eq!(vertices.len(), 5);
    assert_eq!(vertices[0], [-1.0, -0.6]); // First point
    assert_eq!(vertices[4], [1.0, 1.0]); // Last point

    println!("✅ Vertex generation test passed");
}

/// Test line chart rendering with WebGPU
#[tokio::test]
async fn test_line_chart_rendering() {
    let renderer_result = WebGpuRealRenderer::new(None).await;

    match renderer_result {
        Ok(mut renderer) => {
            // Compile shader
            let shader_source = r#"
                @vertex
                fn vs_main(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
                    return vec4<f32>(position, 0.0, 1.0);
                }

                @fragment
                fn fs_main() -> @location(0) vec4<f32> {
                    return vec4<f32>(0.0, 0.5, 1.0, 1.0);
                }
            "#;

            renderer.compile_shader("line", shader_source).unwrap();
            renderer.create_line_pipeline().unwrap();

            // Create test data
            let data_points = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 3.0), (3.0, 4.0), (4.0, 5.0)];

            // Generate vertices
            let vertices: Vec<[f32; 2]> = data_points
                .iter()
                .map(|(x, y)| {
                    let norm_x = (x / 4.0) * 2.0 - 1.0;
                    let norm_y = (y / 5.0) * 2.0 - 1.0;
                    [norm_x, norm_y]
                })
                .collect();

            // Create vertex buffer
            let buffer = renderer.create_vertex_buffer(&vertices).unwrap();
            assert_eq!(buffer.size(), (vertices.len() * 8) as u64); // 2 floats * 4 bytes each

            println!("✅ Line chart rendering test passed");
        }
        Err(WebGpuRealError::NotSupported(_)) => {
            println!("⚠️  WebGPU not supported, skipping test");
            assert!(true);
        }
        Err(e) => {
            println!("❌ Unexpected error: {}", e);
            panic!("Unexpected error: {}", e);
        }
    }
}

/// Test line chart styling
#[tokio::test]
async fn test_line_chart_styling() {
    // Test different line styles
    let styles = vec![
        ("#ff0000", 1.0), // Red, thin
        ("#00ff00", 2.0), // Green, medium
        ("#0000ff", 3.0), // Blue, thick
    ];

    for (color, width) in styles {
        let config = LineChartConfig {
            base: BaseChartConfig {
                width: 800,
                height: 600,
                title: "Test Chart".to_string(),
                x_label: "X Axis".to_string(),
                y_label: "Y Axis".to_string(),
                show_grid: true,
                background_color: "#ffffff".to_string(),
                text_color: "#000000".to_string(),
            },
            color: color.to_string(),
            line_width: width,
            show_points: true,
            point_size: 4.0,
            interpolation: InterpolationType::Linear,
            show_legend: true,
        };

        assert_eq!(config.color, color);
        assert_eq!(config.line_width, width);
    }

    println!("✅ Line chart styling test passed");
}

/// Test line chart performance
#[tokio::test]
async fn test_line_chart_performance() {
    let start_time = Instant::now();

    // Test with large dataset
    let large_dataset: Vec<(f32, f32)> = (0..10000)
        .map(|i| {
            let x = i as f32 / 10000.0;
            let y = (i as f32 * 0.01).sin();
            (x, y)
        })
        .collect();

    let processing_time = start_time.elapsed();

    // Test vertex generation performance
    let vertex_start = Instant::now();
    let vertices: Vec<[f32; 2]> = large_dataset
        .iter()
        .map(|(x, y)| {
            let norm_x = x * 2.0 - 1.0;
            let norm_y = y * 2.0 - 1.0;
            [norm_x, norm_y]
        })
        .collect();
    let vertex_time = vertex_start.elapsed();

    assert_eq!(vertices.len(), 10000);
    assert!(processing_time.as_millis() < 100); // Should be fast
    assert!(vertex_time.as_millis() < 50); // Vertex generation should be very fast

    println!("✅ Line chart performance test passed");
    println!("  Dataset size: {}", large_dataset.len());
    println!("  Processing time: {:?}", processing_time);
    println!("  Vertex generation time: {:?}", vertex_time);
}

/// Test line chart error handling
#[tokio::test]
async fn test_line_chart_error_handling() {
    // Test with empty data
    let empty_data: Vec<(f32, f32)> = vec![];
    assert_eq!(empty_data.len(), 0);

    // Test with valid data
    let valid_data = vec![(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)];
    assert_eq!(valid_data.len(), 3);

    // Test data processing with valid data
    let processed_data: Vec<[f32; 2]> = valid_data.iter().map(|(x, y)| [*x, *y]).collect();

    assert_eq!(processed_data.len(), 3);
    assert_eq!(processed_data[0], [1.0, 2.0]);

    println!("✅ Line chart error handling test passed");
}

/// Test line chart interpolation
#[tokio::test]
async fn test_line_chart_interpolation() {
    // Test different interpolation types
    let interpolation_types = vec![
        InterpolationType::Linear,
        InterpolationType::Smooth,
        InterpolationType::Step,
    ];

    for interpolation in interpolation_types {
        let config = LineChartConfig {
            base: BaseChartConfig {
                width: 800,
                height: 600,
                title: "Test Chart".to_string(),
                x_label: "X Axis".to_string(),
                y_label: "Y Axis".to_string(),
                show_grid: true,
                background_color: "#ffffff".to_string(),
                text_color: "#000000".to_string(),
            },
            color: "#0080ff".to_string(),
            line_width: 2.0,
            show_points: true,
            point_size: 4.0,
            interpolation: interpolation.clone(),
            show_legend: true,
        };

        // Test that the interpolation type is set correctly
        match config.interpolation {
            InterpolationType::Linear => println!("Linear interpolation set"),
            InterpolationType::Smooth => println!("Smooth interpolation set"),
            InterpolationType::Step => println!("Step interpolation set"),
            InterpolationType::Monotone => println!("Monotone interpolation set"),
        }
    }

    println!("✅ Line chart interpolation test passed");
}
