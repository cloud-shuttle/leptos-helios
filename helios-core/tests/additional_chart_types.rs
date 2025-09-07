//! Additional Chart Types Tests
//! Tests for bar, scatter, and area chart implementations

use leptos_helios::*;

/// Helper function to create a base chart config
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
fn test_bar_chart_rendering() {
    // Given: Sample data for bar chart
    let data = vec![
        ("Category A".to_string(), 25.0),
        ("Category B".to_string(), 40.0),
        ("Category C".to_string(), 30.0),
        ("Category D".to_string(), 35.0),
        ("Category E".to_string(), 20.0),
    ];

    let config = BarChartConfig {
        base: create_base_config("Test Bar Chart", 800, 600),
        colors: vec![
            "#00d4ff".to_string(),
            "#ff6b6b".to_string(),
            "#4ecdc4".to_string(),
            "#45b7d1".to_string(),
            "#96ceb4".to_string(),
        ],
        bar_width: 0.8,
        show_values: true,
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
fn test_horizontal_bar_chart() {
    // Given: Sample data for horizontal bar chart
    let data = vec![
        ("Long Category Name A".to_string(), 75.0),
        ("Category B".to_string(), 60.0),
        ("Category C".to_string(), 85.0),
        ("Category D".to_string(), 45.0),
    ];

    let config = BarChartConfig {
        base: create_base_config("Horizontal Bar Chart", 800, 600),
        colors: vec![
            "#00d4ff".to_string(),
            "#ff6b6b".to_string(),
            "#4ecdc4".to_string(),
            "#45b7d1".to_string(),
        ],
        bar_width: 0.7,
        show_values: true,
        horizontal: true,
        show_legend: true,
        corner_radius: Some(2.0),
        spacing: Some(0.15),
    };

    // When: Rendering a horizontal bar chart
    let renderer = ChartRenderer::auto_detect().unwrap();
    let result = renderer.render_bar_chart(&data, &config);

    // Then: Should render successfully
    assert!(
        result.is_ok(),
        "Horizontal bar chart rendering should succeed"
    );

    let render_result = result.unwrap();
    assert_eq!(render_result.width, 800);
    assert_eq!(render_result.height, 600);
}

#[test]
fn test_scatter_plot_rendering() {
    // Given: Sample data for scatter plot
    let data = vec![
        (1.0, 2.0, Some("Point A".to_string())),
        (2.0, 3.5, Some("Point B".to_string())),
        (3.0, 1.0, Some("Point C".to_string())),
        (4.0, 4.0, Some("Point D".to_string())),
        (5.0, 2.5, Some("Point E".to_string())),
        (6.0, 3.0, Some("Point F".to_string())),
    ];

    let config = ScatterPlotConfig {
        base: create_base_config("Test Scatter Plot", 800, 600),
        point_color: "#00d4ff".to_string(),
        point_size: 6.0,
        show_trend_line: true,
        trend_line_color: "#ff6b6b".to_string(),
        trend_line_width: 2.0,
        show_legend: true,
        point_shape: Some(PointShape::Circle),
        opacity: Some(0.8),
        jitter: Some(0.1),
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
fn test_scatter_plot_with_different_shapes() {
    // Given: Sample data with different point shapes
    let data = vec![
        (1.0, 2.0, Some("Circle".to_string())),
        (2.0, 3.0, Some("Square".to_string())),
        (3.0, 1.0, Some("Triangle".to_string())),
        (4.0, 4.0, Some("Diamond".to_string())),
    ];

    let config = ScatterPlotConfig {
        base: create_base_config("Scatter with Shapes", 800, 600),
        point_color: "#00d4ff".to_string(),
        point_size: 8.0,
        show_trend_line: false,
        trend_line_color: "#ff6b6b".to_string(),
        trend_line_width: 2.0,
        show_legend: true,
        point_shape: Some(PointShape::Circle),
        opacity: Some(0.9),
        jitter: None,
    };

    // When: Rendering scatter plot with shapes
    let renderer = ChartRenderer::auto_detect().unwrap();
    let result = renderer.render_scatter_plot(&data, &config);

    // Then: Should render successfully
    assert!(
        result.is_ok(),
        "Scatter plot with shapes should render successfully"
    );
}

#[test]
fn test_area_chart_rendering() {
    // Given: Sample data for area chart
    let data = vec![
        (0.0, 1.0),
        (1.0, 2.5),
        (2.0, 1.8),
        (3.0, 3.2),
        (4.0, 2.0),
        (5.0, 4.0),
        (6.0, 3.5),
    ];

    let config = AreaChartConfig {
        base: create_base_config("Test Area Chart", 800, 600),
        fill_color: "#00d4ff".to_string(),
        stroke_color: "#0066cc".to_string(),
        stroke_width: 2.0,
        opacity: 0.7,
        interpolation: InterpolationType::Smooth,
        show_legend: true,
        gradient: Some(GradientConfig {
            start_color: "#00d4ff".to_string(),
            end_color: "#0066cc".to_string(),
            direction: GradientDirection::Vertical,
        }),
    };

    // When: Rendering an area chart
    let renderer = ChartRenderer::auto_detect().unwrap();
    let result = renderer.render_area_chart(&data, &config);

    // Then: Should render successfully
    assert!(result.is_ok(), "Area chart rendering should succeed");

    let render_result = result.unwrap();
    assert_eq!(render_result.width, 800);
    assert_eq!(render_result.height, 600);
    assert!(
        !render_result.pixel_data.is_empty(),
        "Pixel data should not be empty"
    );
}

#[test]
fn test_stacked_area_chart() {
    // Given: Sample data for stacked area chart
    let data = vec![
        StackedAreaData {
            x: 0.0,
            values: vec![1.0, 2.0, 1.5],
            labels: vec![
                "Series A".to_string(),
                "Series B".to_string(),
                "Series C".to_string(),
            ],
        },
        StackedAreaData {
            x: 1.0,
            values: vec![1.5, 2.5, 2.0],
            labels: vec![
                "Series A".to_string(),
                "Series B".to_string(),
                "Series C".to_string(),
            ],
        },
        StackedAreaData {
            x: 2.0,
            values: vec![2.0, 3.0, 1.8],
            labels: vec![
                "Series A".to_string(),
                "Series B".to_string(),
                "Series C".to_string(),
            ],
        },
    ];

    let config = StackedAreaChartConfig {
        base: create_base_config("Stacked Area Chart", 800, 600),
        colors: vec![
            "#00d4ff".to_string(),
            "#ff6b6b".to_string(),
            "#4ecdc4".to_string(),
        ],
        stroke_width: 1.0,
        opacity: 0.8,
        interpolation: InterpolationType::Smooth,
        show_legend: true,
    };

    // When: Rendering a stacked area chart
    let renderer = ChartRenderer::auto_detect().unwrap();
    let result = renderer.render_stacked_area_chart(&data, &config);

    // Then: Should render successfully
    assert!(
        result.is_ok(),
        "Stacked area chart rendering should succeed"
    );

    let render_result = result.unwrap();
    assert_eq!(render_result.width, 800);
    assert_eq!(render_result.height, 600);
}

#[test]
fn test_chart_type_detection() {
    // Given: Different chart specifications
    let line_spec = ChartSpec {
        data: DataReference::Static(serde_json::json!(vec![(0.0, 1.0), (1.0, 2.0)])),
        mark: MarkType::Line {
            interpolate: Some(Interpolation::Linear),
            stroke_width: Some(2.0),
            stroke_dash: None,
        },
        encoding: Encoding::default(),
        transform: vec![],
        selection: vec![],
        intelligence: None,
        config: ChartConfig::default(),
    };

    let bar_spec = ChartSpec {
        data: DataReference::Static(serde_json::json!(vec![("A", 10.0), ("B", 20.0)])),
        mark: MarkType::Bar {
            width: Some(BarWidth::Fixed(0.8)),
            corner_radius: Some(4.0),
        },
        encoding: Encoding::default(),
        transform: vec![],
        selection: vec![],
        intelligence: None,
        config: ChartConfig::default(),
    };

    let area_spec = ChartSpec {
        data: DataReference::Static(serde_json::json!(vec![(0.0, 1.0), (1.0, 2.0)])),
        mark: MarkType::Area {
            interpolate: Some(Interpolation::Smooth),
            opacity: Some(0.7),
        },
        encoding: Encoding::default(),
        transform: vec![],
        selection: vec![],
        intelligence: None,
        config: ChartConfig::default(),
    };

    // When: Detecting chart types
    let line_type = ChartType::from_spec(&line_spec);
    let bar_type = ChartType::from_spec(&bar_spec);
    let area_type = ChartType::from_spec(&area_spec);

    // Then: Should detect correct types
    assert_eq!(line_type, ChartType::Line);
    assert_eq!(bar_type, ChartType::Bar);
    assert_eq!(area_type, ChartType::Area);
}

#[test]
fn test_chart_rendering_performance() {
    // Given: Large datasets for performance testing
    let bar_data: Vec<(String, f64)> = (0..1000)
        .map(|i| (format!("Category {}", i), (i as f64) * 0.1))
        .collect();

    let scatter_data: Vec<(f64, f64, Option<String>)> = (0..5000)
        .map(|i| (i as f64 * 0.1, (i as f64 * 0.1).sin(), None))
        .collect();

    let area_data: Vec<(f64, f64)> = (0..2000)
        .map(|i| (i as f64 * 0.1, (i as f64 * 0.1).cos()))
        .collect();

    let bar_config = BarChartConfig {
        base: create_base_config("Performance Bar", 1920, 1080),
        colors: vec!["#00d4ff".to_string()],
        bar_width: 0.8,
        show_values: false,
        horizontal: false,
        show_legend: false,
        corner_radius: None,
        spacing: None,
    };

    let scatter_config = ScatterPlotConfig {
        base: create_base_config("Performance Scatter", 1920, 1080),
        point_color: "#00d4ff".to_string(),
        point_size: 2.0,
        show_trend_line: false,
        trend_line_color: "#ff6b6b".to_string(),
        trend_line_width: 1.0,
        show_legend: false,
        point_shape: Some(PointShape::Circle),
        opacity: Some(0.8),
        jitter: None,
    };

    let area_config = AreaChartConfig {
        base: create_base_config("Performance Area", 1920, 1080),
        fill_color: "#00d4ff".to_string(),
        stroke_color: "#0066cc".to_string(),
        stroke_width: 1.0,
        opacity: 0.7,
        interpolation: InterpolationType::Linear,
        show_legend: false,
        gradient: None,
    };

    // When: Rendering with large datasets
    let renderer = ChartRenderer::auto_detect().unwrap();

    let start = std::time::Instant::now();
    let bar_result = renderer.render_bar_chart(&bar_data, &bar_config);
    let bar_duration = start.elapsed();

    let start = std::time::Instant::now();
    let scatter_result = renderer.render_scatter_plot(&scatter_data, &scatter_config);
    let scatter_duration = start.elapsed();

    let start = std::time::Instant::now();
    let area_result = renderer.render_area_chart(&area_data, &area_config);
    let area_duration = start.elapsed();

    // Then: Should render within reasonable time
    assert!(
        bar_result.is_ok(),
        "Bar chart performance test should succeed"
    );
    assert!(
        scatter_result.is_ok(),
        "Scatter plot performance test should succeed"
    );
    assert!(
        area_result.is_ok(),
        "Area chart performance test should succeed"
    );

    assert!(
        bar_duration.as_millis() < 200,
        "Bar chart with 1000 points should render in < 200ms, took {}ms",
        bar_duration.as_millis()
    );

    assert!(
        scatter_duration.as_millis() < 300,
        "Scatter plot with 5000 points should render in < 300ms, took {}ms",
        scatter_duration.as_millis()
    );

    assert!(
        area_duration.as_millis() < 250,
        "Area chart with 2000 points should render in < 250ms, took {}ms",
        area_duration.as_millis()
    );
}

#[test]
fn test_chart_error_handling() {
    // Given: Invalid configurations
    let empty_data: Vec<(String, f64)> = vec![];
    let config = BarChartConfig {
        base: create_base_config("Error Test", 800, 600),
        colors: vec![],
        bar_width: 0.8,
        show_values: true,
        horizontal: false,
        show_legend: true,
        corner_radius: None,
        spacing: None,
    };

    // When: Rendering with invalid data
    let renderer = ChartRenderer::auto_detect().unwrap();
    let result = renderer.render_bar_chart(&empty_data, &config);

    // Then: Should handle error gracefully
    assert!(result.is_err(), "Should return error for empty data");

    let error = result.unwrap_err();
    let error_msg = error.to_string();
    assert!(
        error_msg.contains("empty") || error_msg.contains("data"),
        "Error should mention empty data, got: {}",
        error_msg
    );
}
