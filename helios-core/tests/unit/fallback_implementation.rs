use leptos_helios::chart_config::*;
use leptos_helios::renderer::*;
use leptos_helios::*;

#[tokio::test]
async fn test_automatic_detection() {
    // Test that auto_detect finds the best available backend
    let renderer = ChartRenderer::auto_detect();

    match renderer {
        Ok(r) => {
            // Should have a valid backend
            let backend = r.backend();
            assert!(matches!(
                backend,
                RendererBackend::WebGPU | RendererBackend::WebGL2 | RendererBackend::Canvas2D
            ));
            println!("Auto-detected backend: {:?}", backend);
        }
        Err(e) => {
            // Should only fail if no backends are available
            println!("Auto-detection failed: {}", e);
            assert!(e
                .to_string()
                .contains("No suitable rendering backend found"));
        }
    }
}

#[tokio::test]
async fn test_backend_availability_check() {
    // Test that we can check backend availability
    let webgpu_available = ChartRenderer::is_backend_available(RendererBackend::WebGPU);
    let webgl2_available = ChartRenderer::is_backend_available(RendererBackend::WebGL2);
    let canvas2d_available = ChartRenderer::is_backend_available(RendererBackend::Canvas2D);

    // At least one backend should be available
    assert!(webgpu_available || webgl2_available || canvas2d_available);

    println!("WebGPU available: {}", webgpu_available);
    println!("WebGL2 available: {}", webgl2_available);
    println!("Canvas2D available: {}", canvas2d_available);
}

#[tokio::test]
async fn test_best_backend_selection() {
    // Test that we can get the best available backend
    let best_backend = ChartRenderer::get_best_backend();

    match best_backend {
        Some(backend) => {
            // Should be the most capable available backend
            assert!(matches!(
                backend,
                RendererBackend::WebGPU | RendererBackend::WebGL2 | RendererBackend::Canvas2D
            ));
            println!("Best backend: {:?}", backend);

            // Verify it's actually available
            assert!(ChartRenderer::is_backend_available(backend));
        }
        None => {
            // No backends available - this should be rare
            println!("No backends available");
        }
    }
}

#[tokio::test]
async fn test_backend_switching() {
    // Test that we can switch between backends
    let mut renderer = ChartRenderer::auto_detect().unwrap();
    let original_backend = renderer.backend();

    // Try to switch to each backend
    for backend in [
        RendererBackend::WebGPU,
        RendererBackend::WebGL2,
        RendererBackend::Canvas2D,
    ] {
        if ChartRenderer::is_backend_available(backend) {
            let result = renderer.switch_backend(backend);
            match result {
                Ok(_) => {
                    assert_eq!(renderer.backend(), backend);
                    println!("Successfully switched to {:?}", backend);
                }
                Err(e) => {
                    println!("Failed to switch to {:?}: {}", backend, e);
                }
            }
        } else {
            println!("Backend {:?} not available", backend);
        }
    }

    // Switch back to original backend
    if ChartRenderer::is_backend_available(original_backend) {
        renderer.switch_backend(original_backend).unwrap();
        assert_eq!(renderer.backend(), original_backend);
    }
}

#[tokio::test]
async fn test_fallback_chain_execution() {
    // Test the complete fallback chain
    let mut current_backend = None;

    // Try WebGPU first
    if ChartRenderer::is_backend_available(RendererBackend::WebGPU) {
        current_backend = Some(RendererBackend::WebGPU);
        println!("Using WebGPU backend");
    }
    // Fallback to WebGL2
    else if ChartRenderer::is_backend_available(RendererBackend::WebGL2) {
        current_backend = Some(RendererBackend::WebGL2);
        println!("Using WebGL2 backend");
    }
    // Final fallback to Canvas2D
    else if ChartRenderer::is_backend_available(RendererBackend::Canvas2D) {
        current_backend = Some(RendererBackend::Canvas2D);
        println!("Using Canvas2D backend");
    }

    // Should have found at least one backend
    assert!(current_backend.is_some());

    // Create renderer with the selected backend
    let renderer = ChartRenderer::new(current_backend.unwrap()).unwrap();
    assert_eq!(renderer.backend(), current_backend.unwrap());
}

#[tokio::test]
async fn test_renderer_with_different_backends() {
    // Test that renderers work with different backends
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
        color: "#ff0000".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let data = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 4.0), (3.0, 9.0), (4.0, 16.0)];

    // Test with each available backend
    for backend in [
        RendererBackend::WebGPU,
        RendererBackend::WebGL2,
        RendererBackend::Canvas2D,
    ] {
        if ChartRenderer::is_backend_available(backend) {
            let renderer = ChartRenderer::new(backend).unwrap();
            let result = renderer.render_line_chart(&data, &config);

            match result {
                Ok(render_result) => {
                    assert_eq!(render_result.width, config.base.width);
                    assert_eq!(render_result.height, config.base.height);
                    assert!(!render_result.pixel_data.is_empty());
                    println!("Successfully rendered with {:?} backend", backend);
                }
                Err(e) => {
                    println!("Failed to render with {:?} backend: {}", backend, e);
                }
            }
        }
    }
}

#[tokio::test]
async fn test_fallback_system_robustness() {
    // Test that the fallback system is robust
    let mut renderer = ChartRenderer::auto_detect().unwrap();
    let original_backend = renderer.backend();

    // Test multiple switches
    for _ in 0..3 {
        for backend in [
            RendererBackend::WebGPU,
            RendererBackend::WebGL2,
            RendererBackend::Canvas2D,
        ] {
            if ChartRenderer::is_backend_available(backend) {
                let result = renderer.switch_backend(backend);
                if result.is_ok() {
                    assert_eq!(renderer.backend(), backend);
                }
            }
        }
    }

    // Should still be able to switch back to original
    if ChartRenderer::is_backend_available(original_backend) {
        renderer.switch_backend(original_backend).unwrap();
        assert_eq!(renderer.backend(), original_backend);
    }
}

#[tokio::test]
async fn test_fallback_system_performance() {
    // Test that the fallback system doesn't have significant performance overhead
    let start = std::time::Instant::now();

    // Create multiple renderers
    let mut renderers = vec![];
    for _ in 0..10 {
        if let Ok(renderer) = ChartRenderer::auto_detect() {
            renderers.push(renderer);
        }
    }

    let duration = start.elapsed();

    // Should be fast (less than 100ms for 10 renderers)
    assert!(
        duration.as_millis() < 100,
        "Fallback system too slow: {:?}",
        duration
    );
    println!("Created {} renderers in {:?}", renderers.len(), duration);
}

#[tokio::test]
async fn test_fallback_system_error_handling() {
    // Test that the fallback system handles errors gracefully
    let renderer = ChartRenderer::auto_detect();

    match renderer {
        Ok(r) => {
            // Test with invalid configuration
            let invalid_config = LineChartConfig {
                base: BaseChartConfig {
                    width: 0,  // Invalid width
                    height: 0, // Invalid height
                    title: "Invalid".to_string(),
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

            let data = vec![(0.0, 0.0), (1.0, 1.0)];
            let result = r.render_line_chart(&data, &invalid_config);

            // Should return an error for invalid config
            assert!(result.is_err());
            println!("Correctly handled invalid configuration");
        }
        Err(e) => {
            println!("No renderer available: {}", e);
        }
    }
}
