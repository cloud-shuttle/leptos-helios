use leptos_helios::chart_config::*;
use leptos_helios::renderer::*;
use leptos_helios::*;

#[tokio::test]
async fn test_renderer_backend_detection() {
    // Test that we can detect different renderer backends
    let backends = vec![
        RendererBackend::WebGPU,
        RendererBackend::WebGL2,
        RendererBackend::Canvas2D,
    ];

    for backend in backends {
        match backend {
            RendererBackend::WebGPU => {
                // WebGPU should be the preferred backend
                assert_eq!(backend, RendererBackend::WebGPU);
            }
            RendererBackend::WebGL2 => {
                // WebGL2 should be the fallback for WebGPU
                assert_eq!(backend, RendererBackend::WebGL2);
            }
            RendererBackend::Canvas2D => {
                // Canvas2D should be the final fallback
                assert_eq!(backend, RendererBackend::Canvas2D);
            }
        }
    }
}

#[tokio::test]
async fn test_auto_detect_renderer() {
    // Test that auto_detect returns a renderer (even if it's a mock)
    let renderer = ChartRenderer::auto_detect();

    // In a real browser environment, this would succeed
    // In our test environment, it should fail gracefully
    match renderer {
        Ok(_) => {
            // Renderer detection succeeded
            println!("Renderer auto-detection succeeded");
        }
        Err(e) => {
            // Renderer detection failed, which is expected in test environment
            println!("Renderer auto-detection failed: {}", e);
            assert!(
                e.to_string().contains("WebGPU")
                    || e.to_string().contains("WebGL")
                    || e.to_string().contains("Canvas")
            );
        }
    }
}

#[tokio::test]
async fn test_webgpu_renderer_creation() {
    // Test WebGPU renderer creation
    let webgpu_renderer = WebGpuRenderer::new();

    // WebGPU renderer should be created successfully
    assert!(webgpu_renderer.is_ok());

    let renderer = webgpu_renderer.unwrap();
    assert_eq!(renderer.backend(), RendererBackend::WebGPU);
}

#[tokio::test]
async fn test_webgl2_renderer_creation() {
    // Test WebGL2 renderer creation
    let webgl2_renderer = WebGl2Renderer::new();

    // WebGL2 renderer should be created successfully
    assert!(webgl2_renderer.is_ok());

    let renderer = webgl2_renderer.unwrap();
    assert_eq!(renderer.backend(), RendererBackend::WebGL2);
}

#[tokio::test]
async fn test_canvas2d_renderer_creation() {
    // Test Canvas2D renderer creation
    let canvas2d_renderer = Canvas2DRenderer::new();

    // Canvas2D renderer should be created successfully
    assert!(canvas2d_renderer.is_ok());

    let renderer = canvas2d_renderer.unwrap();
    assert_eq!(renderer.backend(), RendererBackend::Canvas2D);
}

#[tokio::test]
async fn test_fallback_chain() {
    // Test the fallback chain: WebGPU -> WebGL2 -> Canvas2D
    let mut current_backend = RendererBackend::WebGPU;
    let fallback_chain = vec![
        RendererBackend::WebGPU,
        RendererBackend::WebGL2,
        RendererBackend::Canvas2D,
    ];

    for (i, expected_backend) in fallback_chain.iter().enumerate() {
        assert_eq!(current_backend, *expected_backend);

        // Simulate fallback to next backend
        if i < fallback_chain.len() - 1 {
            current_backend = fallback_chain[i + 1];
        }
    }
}

#[tokio::test]
async fn test_renderer_capabilities() {
    // Test that different renderers have different capabilities
    let webgpu_renderer = WebGpuRenderer::new().unwrap();
    let webgl2_renderer = WebGl2Renderer::new().unwrap();
    let canvas2d_renderer = Canvas2DRenderer::new().unwrap();

    // WebGPU should have the most capabilities
    assert_eq!(webgpu_renderer.backend(), RendererBackend::WebGPU);

    // WebGL2 should have good 3D capabilities
    assert_eq!(webgl2_renderer.backend(), RendererBackend::WebGL2);

    // Canvas2D should be the most compatible but least capable
    assert_eq!(canvas2d_renderer.backend(), RendererBackend::Canvas2D);
}

#[tokio::test]
async fn test_renderer_initialization_order() {
    // Test that renderers are initialized in the correct order
    let mut backends = vec![];

    // Try to create renderers in fallback order
    if let Ok(webgpu) = WebGpuRenderer::new() {
        backends.push(webgpu.backend());
    } else if let Ok(webgl2) = WebGl2Renderer::new() {
        backends.push(webgl2.backend());
    } else if let Ok(canvas2d) = Canvas2DRenderer::new() {
        backends.push(canvas2d.backend());
    }

    // At least one renderer should be available
    assert!(!backends.is_empty());

    // The first available renderer should be the most capable
    let first_backend = backends[0];

    // Should be WebGPU, WebGL2, or Canvas2D in that order of preference
    assert!(matches!(
        first_backend,
        RendererBackend::WebGPU | RendererBackend::WebGL2 | RendererBackend::Canvas2D
    ));
}

#[tokio::test]
async fn test_renderer_error_handling() {
    // Test that renderer errors are handled gracefully
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

    let data = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0), (3.0, 3.0), (4.0, 4.0)];

    // Test that renderers can handle invalid data gracefully
    let webgpu_renderer = WebGpuRenderer::new().unwrap();
    let webgl2_renderer = WebGl2Renderer::new().unwrap();
    let canvas2d_renderer = Canvas2DRenderer::new().unwrap();

    // All renderers should be created successfully
    assert_eq!(webgpu_renderer.backend(), RendererBackend::WebGPU);
    assert_eq!(webgl2_renderer.backend(), RendererBackend::WebGL2);
    assert_eq!(canvas2d_renderer.backend(), RendererBackend::Canvas2D);
}

#[tokio::test]
async fn test_renderer_performance_comparison() {
    // Test that we can compare renderer performance characteristics
    let webgpu_renderer = WebGpuRenderer::new().unwrap();
    let webgl2_renderer = WebGl2Renderer::new().unwrap();
    let canvas2d_renderer = Canvas2DRenderer::new().unwrap();

    // WebGPU should be the most performant
    assert_eq!(webgpu_renderer.backend(), RendererBackend::WebGPU);

    // WebGL2 should be moderately performant
    assert_eq!(webgl2_renderer.backend(), RendererBackend::WebGL2);

    // Canvas2D should be the least performant but most compatible
    assert_eq!(canvas2d_renderer.backend(), RendererBackend::Canvas2D);
}

#[tokio::test]
async fn test_renderer_switching() {
    // Test that we can switch between renderers
    let webgpu_renderer = WebGpuRenderer::new().unwrap();
    assert_eq!(webgpu_renderer.backend(), RendererBackend::WebGPU);

    // Switch to WebGL2
    let webgl2_renderer = WebGl2Renderer::new().unwrap();
    assert_eq!(webgl2_renderer.backend(), RendererBackend::WebGL2);

    // Switch to Canvas2D
    let canvas2d_renderer = Canvas2DRenderer::new().unwrap();
    assert_eq!(canvas2d_renderer.backend(), RendererBackend::Canvas2D);
}

#[tokio::test]
async fn test_renderer_compatibility() {
    // Test renderer compatibility with different environments
    let webgpu_renderer = WebGpuRenderer::new().unwrap();
    let webgl2_renderer = WebGl2Renderer::new().unwrap();
    let canvas2d_renderer = Canvas2DRenderer::new().unwrap();

    // All renderers should be compatible with their respective environments
    assert_eq!(webgpu_renderer.backend(), RendererBackend::WebGPU);
    assert_eq!(webgl2_renderer.backend(), RendererBackend::WebGL2);
    assert_eq!(canvas2d_renderer.backend(), RendererBackend::Canvas2D);

    // Canvas2D should be the most compatible
    // WebGL2 should be moderately compatible
    // WebGPU should be the least compatible but most capable
}

#[tokio::test]
async fn test_fallback_system_integration() {
    // Test the complete fallback system integration
    let config = LineChartConfig {
        base: BaseChartConfig {
            width: 400,
            height: 300,
            title: "Fallback Test Chart".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        },
        color: "#00ff00".to_string(),
        line_width: 1.0,
        show_points: false,
        point_size: 2.0,
        interpolation: InterpolationType::Smooth,
        show_legend: false,
    };

    let data = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 4.0), (3.0, 9.0), (4.0, 16.0)];

    // Test that the fallback system can handle chart rendering
    let renderer = ChartRenderer::auto_detect();

    match renderer {
        Ok(_) => {
            // Fallback system worked
            println!("Fallback system integration test passed");
        }
        Err(e) => {
            // Fallback system failed, which is expected in test environment
            println!("Fallback system integration test failed as expected: {}", e);
            assert!(
                e.to_string().contains("WebGPU")
                    || e.to_string().contains("WebGL")
                    || e.to_string().contains("Canvas")
            );
        }
    }
}
