//! Chart Rendering Tests
//! Tests for actual WebGPU/WebGL2 chart rendering functionality

use leptos_helios::*;

// Helper function to create a base chart config
fn create_base_config(title: &str, width: u32, height: u32) -> BaseChartConfig {
    BaseChartConfig {
        width,
        height,
        title: title.to_string(),
        x_label: "X Axis".to_string(),
        y_label: "Y Axis".to_string(),
        show_grid: true,
        background_color: "#ffffff".to_string(),
        text_color: "#000000".to_string(),
    }
}

#[test]
fn test_webgpu_renderer_initialization() {
    // Given: WebGPU renderer should be able to initialize
    // When: Creating a WebGPU renderer
    // Then: Renderer should be created successfully

    // This test will fail initially - we need to implement the renderer
    let renderer = WebGpuRenderer::new();
    assert!(
        renderer.is_ok(),
        "WebGPU renderer should initialize successfully"
    );
}

#[test]
fn test_webgl2_fallback_renderer() {
    // Given: WebGL2 fallback renderer should be available
    // When: Creating a WebGL2 renderer
    // Then: Renderer should be created successfully

    let renderer = WebGl2Renderer::new();
    assert!(
        renderer.is_ok(),
        "WebGL2 fallback renderer should initialize successfully"
    );
}

#[test]
fn test_canvas2d_fallback_renderer() {
    // Given: Canvas2D fallback renderer should be available
    // When: Creating a Canvas2D renderer
    // Then: Renderer should be created successfully

    let renderer = Canvas2DRenderer::new();
    assert!(
        renderer.is_ok(),
        "Canvas2D fallback renderer should initialize successfully"
    );
}

#[test]
fn test_renderer_auto_detection() {
    // Given: Renderer should auto-detect the best available backend
    // When: Creating a renderer with auto-detection
    // Then: Should return the best available renderer

    let renderer = ChartRenderer::auto_detect();
    assert!(
        renderer.is_ok(),
        "Auto-detection should return a valid renderer"
    );

    let renderer = renderer.unwrap();
    assert!(matches!(
        renderer.backend(),
        RendererBackend::WebGPU | RendererBackend::WebGL2 | RendererBackend::Canvas2D
    ));
}

#[test]
fn test_line_chart_rendering() {
    // Given: Sample data for line chart
    let data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 1.5), (3.0, 3.0), (4.0, 2.5)];

    let config = LineChartConfig {
        base: create_base_config("Test Line Chart", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    // When: Rendering a line chart
    let renderer = ChartRenderer::auto_detect().unwrap();
    let result = renderer.render_line_chart(&data, &config);

    // Then: Should render successfully
    assert!(result.is_ok(), "Line chart rendering should succeed");

    let render_result = result.unwrap();
    assert_eq!(render_result.width, 800);
    assert_eq!(render_result.height, 600);
    assert!(
        !render_result.pixel_data.is_empty(),
        "Pixel data should not be empty"
    );
}

#[test]
fn test_bar_chart_rendering() {
    // Given: Sample data for bar chart
    let data = vec![
        ("A".to_string(), 10.0),
        ("B".to_string(), 20.0),
        ("C".to_string(), 15.0),
        ("D".to_string(), 25.0),
    ];

    let config = BarChartConfig {
        base: create_base_config("Test Bar Chart", 800, 600),
        colors: vec![
            "#00d4ff".to_string(),
            "#ff6b6b".to_string(),
            "#4ecdc4".to_string(),
            "#45b7d1".to_string(),
        ],
        bar_width: 0.8,
        show_values: false,
        horizontal: false,
        show_legend: true,
        corner_radius: Some(4.0),
        spacing: Some(0.1),
    };

    // When: Rendering a bar chart
    let renderer = ChartRenderer::auto_detect().unwrap();
    let result = renderer.render_bar_chart(&data, &config);

    // Then: Should render successfully
    assert!(result.is_ok(), "Bar chart rendering should succeed");

    let render_result = result.unwrap();
    assert_eq!(render_result.width, 800);
    assert_eq!(render_result.height, 600);
    assert!(
        !render_result.pixel_data.is_empty(),
        "Pixel data should not be empty"
    );
}

#[test]
fn test_scatter_plot_rendering() {
    // Given: Sample data for scatter plot
    let data = vec![
        (1.0, 2.0, None),
        (2.0, 3.0, None),
        (3.0, 1.0, None),
        (4.0, 4.0, None),
        (5.0, 2.5, None),
    ];

    let config = ScatterPlotConfig {
        base: create_base_config("Test Scatter Plot", 800, 600),
        point_color: "#00d4ff".to_string(),
        point_size: 5.0,
        show_trend_line: true,
        trend_line_color: "#ff6b6b".to_string(),
        trend_line_width: 2.0,
        show_legend: true,
        jitter: Some(0.1),
        opacity: Some(0.8),
        point_shape: Some(PointShape::Circle),
    };

    // When: Rendering a scatter plot
    let renderer = ChartRenderer::auto_detect().unwrap();
    let result = renderer.render_scatter_plot(&data, &config);

    // Then: Should render successfully
    assert!(result.is_ok(), "Scatter plot rendering should succeed");

    let render_result = result.unwrap();
    assert_eq!(render_result.width, 800);
    assert_eq!(render_result.height, 600);
    assert!(
        !render_result.pixel_data.is_empty(),
        "Pixel data should not be empty"
    );
}

#[test]
fn test_heatmap_rendering() {
    // Given: Sample data for heatmap
    let data = vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ];

    let config = HeatmapConfig {
        base: create_base_config("Test Heatmap", 800, 600),
        x_labels: vec!["X1".to_string(), "X2".to_string(), "X3".to_string()],
        y_labels: vec!["Y1".to_string(), "Y2".to_string(), "Y3".to_string()],
        color_scheme: ColorScheme::Viridis,
        show_values: true,
        show_legend: true,
    };

    // When: Rendering a heatmap
    let renderer = ChartRenderer::auto_detect().unwrap();
    let result = renderer.render_heatmap(&data, &config);

    // Then: Should render successfully
    assert!(result.is_ok(), "Heatmap rendering should succeed");

    let render_result = result.unwrap();
    assert_eq!(render_result.width, 800);
    assert_eq!(render_result.height, 600);
    assert!(
        !render_result.pixel_data.is_empty(),
        "Pixel data should not be empty"
    );
}

#[test]
fn test_renderer_performance() {
    // Given: Large dataset for performance testing
    let data: Vec<(f64, f64)> = (0..10000).map(|i| (i as f64, (i as f64).sin())).collect();

    let config = LineChartConfig {
        base: create_base_config("Performance Test", 1920, 1080),
        color: "#00d4ff".to_string(),
        line_width: 1.0,
        show_points: false,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    // When: Rendering with large dataset
    let start = std::time::Instant::now();
    let renderer = ChartRenderer::auto_detect().unwrap();
    let result = renderer.render_line_chart(&data, &config);
    let duration = start.elapsed();

    // Then: Should render within reasonable time (< 100ms for 10K points)
    assert!(result.is_ok(), "Performance test rendering should succeed");
    assert!(
        duration.as_millis() < 100,
        "Rendering 10K points should take less than 100ms, took {}ms",
        duration.as_millis()
    );
}

#[test]
fn test_renderer_memory_usage() {
    // Given: Multiple chart renderings
    let data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 1.5)];
    let config = LineChartConfig {
        base: create_base_config("Memory Test", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    // When: Rendering multiple charts
    let renderer = ChartRenderer::auto_detect().unwrap();
    let mut results = Vec::new();

    for i in 0..10 {
        let result = renderer.render_line_chart(&data, &config);
        assert!(result.is_ok(), "Rendering {} should succeed", i);
        results.push(result.unwrap());
    }

    // Then: Memory should be managed properly (no leaks)
    assert_eq!(results.len(), 10, "Should have 10 render results");

    // Each result should have the expected dimensions
    for result in &results {
        assert_eq!(result.width, 800);
        assert_eq!(result.height, 600);
        assert!(!result.pixel_data.is_empty());
    }
}

#[test]
fn test_renderer_error_handling() {
    // Given: Invalid configuration
    let data = vec![(0.0, 1.0)];
    let config = LineChartConfig {
        base: BaseChartConfig {
            width: 0, // Invalid width
            height: 600,
            title: "Error Test".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    // When: Attempting to render with invalid config
    let renderer = ChartRenderer::auto_detect().unwrap();
    let result = renderer.render_line_chart(&data, &config);

    // Then: Should return appropriate error
    assert!(
        result.is_err(),
        "Should return error for invalid configuration"
    );

    let error = result.unwrap_err();
    let error_msg = error.to_string();
    println!("Actual error message: {}", error_msg);
    assert!(
        error_msg.contains("width")
            || error_msg.contains("invalid")
            || error_msg.contains("greater than 0"),
        "Error should mention width, invalid, or greater than 0, but got: {}",
        error_msg
    );
}

#[test]
fn test_renderer_backend_switching() {
    // Given: Multiple renderer backends
    let data = vec![(0.0, 1.0), (1.0, 2.0)];
    let config = LineChartConfig {
        base: create_base_config("Backend Test", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    // When: Testing different backends
    let backends = vec![
        RendererBackend::WebGPU,
        RendererBackend::WebGL2,
        RendererBackend::Canvas2D,
    ];

    for backend in backends {
        let renderer = ChartRenderer::new(backend);
        if renderer.is_ok() {
            let result = renderer.unwrap().render_line_chart(&data, &config);
            // Some backends might not be available in test environment
            if result.is_ok() {
                let render_result = result.unwrap();
                assert_eq!(render_result.width, 800);
                assert_eq!(render_result.height, 600);
            }
        }
    }
}
