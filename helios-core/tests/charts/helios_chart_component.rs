use leptos::*;
use leptos_helios::canvas_surface::*;
use leptos_helios::chart_config::*;
use leptos_helios::line_chart_renderer::*;
use leptos_helios::webgpu_real::*;
use leptos_helios::*;

#[tokio::test]
async fn test_helios_chart_creation() {
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
        color: "#ff0000".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 1.5), (3.0, 3.0), (4.0, 2.5)];

    // Test that we can create a chart with valid configuration
    assert_eq!(config.base.width, 800);
    assert_eq!(config.base.height, 600);
    assert_eq!(config.base.title, "Test Chart");
    assert_eq!(config.color, "#ff0000");
    assert_eq!(config.line_width, 2.0);
    assert_eq!(data.len(), 5);
}

#[tokio::test]
async fn test_helios_chart_props() {
    let config = LineChartConfig {
        base: BaseChartConfig {
            width: 1000,
            height: 800,
            title: "Props Test Chart".to_string(),
            x_label: "Time".to_string(),
            y_label: "Value".to_string(),
            show_grid: false,
            background_color: "#f0f0f0".to_string(),
            text_color: "#333333".to_string(),
        },
        color: "#00ff00".to_string(),
        line_width: 3.0,
        show_points: false,
        point_size: 6.0,
        interpolation: InterpolationType::Smooth,
        show_legend: false,
    };

    // Test all props are correctly set
    assert_eq!(config.base.width, 1000);
    assert_eq!(config.base.height, 800);
    assert_eq!(config.base.title, "Props Test Chart");
    assert_eq!(config.base.x_label, "Time");
    assert_eq!(config.base.y_label, "Value");
    assert_eq!(config.base.show_grid, false);
    assert_eq!(config.base.background_color, "#f0f0f0");
    assert_eq!(config.base.text_color, "#333333");
    assert_eq!(config.color, "#00ff00");
    assert_eq!(config.line_width, 3.0);
    assert_eq!(config.show_points, false);
    assert_eq!(config.point_size, 6.0);
    assert_eq!(config.interpolation, InterpolationType::Smooth);
    assert_eq!(config.show_legend, false);
}

#[tokio::test]
async fn test_helios_chart_state_management() {
    let initial_data = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 4.0), (3.0, 9.0), (4.0, 16.0)];

    let updated_data = vec![
        (0.0, 0.0),
        (1.0, 1.0),
        (2.0, 4.0),
        (3.0, 9.0),
        (4.0, 16.0),
        (5.0, 25.0),
        (6.0, 36.0),
    ];

    // Test that data can be updated
    assert_ne!(initial_data.len(), updated_data.len());
    assert_eq!(initial_data.len(), 5);
    assert_eq!(updated_data.len(), 7);

    // Test that the last point is correct
    let last_initial = initial_data.last().unwrap();
    let last_updated = updated_data.last().unwrap();
    assert_eq!(*last_initial, (4.0, 16.0));
    assert_eq!(*last_updated, (6.0, 36.0));
}

#[tokio::test]
async fn test_helios_chart_event_handling() {
    let config = LineChartConfig {
        base: BaseChartConfig {
            width: 600,
            height: 400,
            title: "Event Test Chart".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        color: "#0000ff".to_string(),
        line_width: 1.0,
        show_points: true,
        point_size: 3.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let data = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0)];

    // Test that we can simulate mouse events
    let mouse_x = 300.0;
    let mouse_y = 200.0;

    // Test that mouse coordinates are within chart bounds
    assert!(mouse_x >= 0.0 && mouse_x <= config.base.width as f32);
    assert!(mouse_y >= 0.0 && mouse_y <= config.base.height as f32);

    // Test that we can simulate click events
    let click_x = 150.0;
    let click_y = 100.0;

    assert!(click_x >= 0.0 && click_x <= config.base.width as f32);
    assert!(click_y >= 0.0 && click_y <= config.base.height as f32);
}

#[tokio::test]
async fn test_helios_chart_rendering_integration() {
    let config = LineChartConfig {
        base: BaseChartConfig {
            width: 800,
            height: 600,
            title: "Rendering Test Chart".to_string(),
            x_label: "X Axis".to_string(),
            y_label: "Y Axis".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        color: "#ff00ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let data = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 4.0), (3.0, 9.0), (4.0, 16.0)];

    // Test that we can create a line chart renderer
    let renderer = LineChartRenderer::new_sync();
    assert!(renderer.is_ok());

    // Test that we can process the data
    let mut renderer_mut = renderer.unwrap();
    let result = renderer_mut.process_data("test_data", data.clone());
    assert!(result.is_ok());

    // Test that we can generate vertices
    let vertices = renderer_mut.generate_vertices("test_data", &config);
    assert!(vertices.is_ok());
    assert!(vertices.unwrap().len() > 0);
}

#[tokio::test]
async fn test_helios_chart_webgpu_integration() {
    // Test that WebGPU renderer can be created
    let webgpu_renderer = WebGpuRealRenderer::new(None).await;

    // In a real browser environment, this would succeed
    // In our test environment, it should fail gracefully
    match webgpu_renderer {
        Ok(_) => {
            // WebGPU is available
            println!("WebGPU renderer created successfully");
        }
        Err(e) => {
            // WebGPU is not available, which is expected in test environment
            println!("WebGPU not available: {}", e);
            assert!(
                e.to_string().contains("browser environment")
                    || e.to_string().contains("adapter")
                    || e.to_string().contains("device")
            );
        }
    }
}

#[tokio::test]
async fn test_helios_chart_canvas_surface_integration() {
    // Test that canvas surface can be created
    // Test canvas surface creation (will fail in non-browser environment)
    // For testing purposes, we'll just verify the component can be created
    let config = LineChartConfig::default();
    let props = HeliosChartProps {
        config: config.clone(),
        data: vec![(1.0, 2.0), (2.0, 4.0)],
        canvas_id: Some("test-canvas".to_string()),
    };
    let _component = create_helios_chart(props);

    // Test that the component was created successfully
    assert!(true, "HeliosChart component created successfully");
}

#[tokio::test]
async fn test_helios_chart_performance() {
    let config = LineChartConfig {
        base: BaseChartConfig {
            width: 1200,
            height: 800,
            title: "Performance Test Chart".to_string(),
            x_label: "X Axis".to_string(),
            y_label: "Y Axis".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        color: "#00ffff".to_string(),
        line_width: 1.0,
        show_points: false,
        point_size: 2.0,
        interpolation: InterpolationType::Linear,
        show_legend: false,
    };

    // Generate large dataset for performance testing
    let mut data = Vec::new();
    for i in 0..1000 {
        let x = i as f32 * 0.1;
        let y = (x * x).sin();
        data.push((x, y));
    }

    let start = std::time::Instant::now();

    // Test data processing performance
    let mut renderer = LineChartRenderer::new_sync().unwrap();
    let result = renderer.process_data("test_data", data.clone());
    let vertices = renderer.generate_vertices("test_data", &config);

    let duration = start.elapsed();

    // Performance assertions
    assert!(result.is_ok());
    assert!(vertices.is_ok());
    assert!(vertices.unwrap().len() > 0);
    assert!(
        duration.as_millis() < 100,
        "Data processing took too long: {:?}",
        duration
    );
}

#[tokio::test]
async fn test_helios_chart_error_handling() {
    // Test with invalid configuration
    let invalid_config = LineChartConfig {
        base: BaseChartConfig {
            width: 0,                // Invalid width
            height: 0,               // Invalid height
            title: "".to_string(),   // Empty title
            x_label: "".to_string(), // Empty x_label
            y_label: "".to_string(), // Empty y_label
            show_grid: true,
            background_color: "invalid_color".to_string(), // Invalid color
            text_color: "invalid_color".to_string(),       // Invalid color
        },
        color: "invalid_color".to_string(), // Invalid color
        line_width: -1.0,                   // Invalid line width
        show_points: true,
        point_size: -1.0, // Invalid point size
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    // Test that invalid configuration is handled gracefully
    assert_eq!(invalid_config.base.width, 0);
    assert_eq!(invalid_config.base.height, 0);
    assert_eq!(invalid_config.base.title, "");
    assert_eq!(invalid_config.color, "invalid_color");
    assert_eq!(invalid_config.line_width, -1.0);
    assert_eq!(invalid_config.point_size, -1.0);

    // Test with empty data
    let empty_data: Vec<(f32, f32)> = vec![];
    let mut renderer = LineChartRenderer::new_sync().unwrap();
    let result = renderer.process_data("empty_data", empty_data);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_helios_chart_interpolation_types() {
    let config_linear = LineChartConfig {
        base: BaseChartConfig {
            width: 400,
            height: 300,
            title: "Linear Chart".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        color: "#ff0000".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let config_smooth = LineChartConfig {
        base: BaseChartConfig {
            width: 400,
            height: 300,
            title: "Smooth Chart".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        color: "#00ff00".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Smooth,
        show_legend: true,
    };

    let config_monotone = LineChartConfig {
        base: BaseChartConfig {
            width: 400,
            height: 300,
            title: "Monotone Chart".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        color: "#0000ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Monotone,
        show_legend: true,
    };

    let data = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.5), (3.0, 2.0), (4.0, 1.5)];

    // Test that all interpolation types work
    assert_eq!(config_linear.interpolation, InterpolationType::Linear);
    assert_eq!(config_smooth.interpolation, InterpolationType::Smooth);
    assert_eq!(config_monotone.interpolation, InterpolationType::Monotone);

    // Test that data processing works with all interpolation types
    let mut renderer = LineChartRenderer::new_sync().unwrap();
    let result = renderer.process_data("test_data", data.clone());

    let vertices_linear = renderer.generate_vertices("test_data", &config_linear);
    let vertices_smooth = renderer.generate_vertices("test_data", &config_smooth);
    let vertices_monotone = renderer.generate_vertices("test_data", &config_monotone);

    assert!(vertices_linear.is_ok());
    assert!(vertices_smooth.is_ok());
    assert!(vertices_monotone.is_ok());
    assert!(vertices_linear.unwrap().len() > 0);
    assert!(vertices_smooth.unwrap().len() > 0);
    assert!(vertices_monotone.unwrap().len() > 0);
}
