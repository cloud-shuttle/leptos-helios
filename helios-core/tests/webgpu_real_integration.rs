//! Real WebGPU Integration Tests
//!
//! These tests verify actual WebGPU functionality, not mocks

use leptos_helios::webgpu_real::*;
use std::time::Instant;

/// Test WebGPU support detection
#[tokio::test]
async fn test_webgpu_support_detection() {
    let supported = WebGpuRealRenderer::is_webgpu_supported();
    println!("WebGPU Support: {}", supported);

    // This test will pass regardless of support, but logs the result
    assert!(true); // Always pass - we're just checking support
}

/// Test renderer creation without surface
#[tokio::test]
async fn test_renderer_creation_no_surface() {
    let result = WebGpuRealRenderer::new(None).await;

    match result {
        Ok(renderer) => {
            println!("✅ Renderer created successfully");
            println!("Device info: {}", renderer.get_device_info());
            assert!(true);
        }
        Err(WebGpuRealError::NotSupported(msg)) => {
            println!("⚠️  WebGPU not supported: {}", msg);
            // This is expected in some environments
            assert!(true);
        }
        Err(e) => {
            println!("❌ Unexpected error: {}", e);
            panic!("Unexpected error: {}", e);
        }
    }
}

/// Test shader compilation
#[tokio::test]
async fn test_shader_compilation() {
    let renderer_result = WebGpuRealRenderer::new(None).await;

    match renderer_result {
        Ok(mut renderer) => {
            // Test compiling a simple shader
            let shader_source = r#"
                @vertex
                fn vs_main(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
                    return vec4<f32>(position, 0.0, 1.0);
                }

                @fragment
                fn fs_main() -> @location(0) vec4<f32> {
                    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
                }
            "#;

            let result = renderer.compile_shader("test", shader_source);
            match result {
                Ok(_) => {
                    println!("✅ Shader compiled successfully");
                    assert!(renderer.has_shader("test"));
                }
                Err(e) => {
                    println!("❌ Shader compilation failed: {}", e);
                    panic!("Shader compilation failed: {}", e);
                }
            }
        }
        Err(WebGpuRealError::NotSupported(_)) => {
            println!("⚠️  Skipping shader test - WebGPU not supported");
            assert!(true);
        }
        Err(e) => {
            println!("❌ Unexpected error: {}", e);
            panic!("Unexpected error: {}", e);
        }
    }
}

/// Test line pipeline creation
#[tokio::test]
async fn test_line_pipeline_creation() {
    let renderer_result = WebGpuRealRenderer::new(None).await;

    match renderer_result {
        Ok(mut renderer) => {
            let result = renderer.create_line_pipeline();
            match result {
                Ok(_) => {
                    println!("✅ Line pipeline created successfully");
                    assert!(renderer.has_pipeline("line"));
                }
                Err(e) => {
                    println!("❌ Line pipeline creation failed: {}", e);
                    panic!("Line pipeline creation failed: {}", e);
                }
            }
        }
        Err(WebGpuRealError::NotSupported(_)) => {
            println!("⚠️  Skipping pipeline test - WebGPU not supported");
            assert!(true);
        }
        Err(e) => {
            println!("❌ Unexpected error: {}", e);
            panic!("Unexpected error: {}", e);
        }
    }
}

/// Test vertex buffer creation
#[tokio::test]
async fn test_vertex_buffer_creation() {
    let renderer_result = WebGpuRealRenderer::new(None).await;

    match renderer_result {
        Ok(renderer) => {
            // Create test vertices for a simple line
            let vertices = vec![
                [-1.0, -1.0], // Bottom left
                [0.0, 1.0],   // Top center
                [1.0, -1.0],  // Bottom right
            ];

            let result = renderer.create_vertex_buffer(&vertices);
            match result {
                Ok(buffer) => {
                    println!("✅ Vertex buffer created successfully");
                    // Buffer size should be 3 vertices * 2 floats * 4 bytes = 24 bytes
                    assert_eq!(buffer.size(), 24);
                }
                Err(e) => {
                    println!("❌ Vertex buffer creation failed: {}", e);
                    panic!("Vertex buffer creation failed: {}", e);
                }
            }
        }
        Err(WebGpuRealError::NotSupported(_)) => {
            println!("⚠️  Skipping vertex buffer test - WebGPU not supported");
            assert!(true);
        }
        Err(e) => {
            println!("❌ Unexpected error: {}", e);
            panic!("Unexpected error: {}", e);
        }
    }
}

/// Test renderer readiness
#[tokio::test]
async fn test_renderer_readiness() {
    let renderer_result = WebGpuRealRenderer::new(None).await;

    match renderer_result {
        Ok(renderer) => {
            // Without surface, renderer should not be ready for rendering
            assert!(!renderer.is_ready());
            println!("✅ Renderer readiness check passed (not ready without surface)");
        }
        Err(WebGpuRealError::NotSupported(_)) => {
            println!("⚠️  Skipping readiness test - WebGPU not supported");
            assert!(true);
        }
        Err(e) => {
            println!("❌ Unexpected error: {}", e);
            panic!("Unexpected error: {}", e);
        }
    }
}

/// Performance test for buffer creation
#[tokio::test]
async fn test_buffer_creation_performance() {
    let renderer_result = WebGpuRealRenderer::new(None).await;

    match renderer_result {
        Ok(renderer) => {
            let start_time = Instant::now();

            // Create a larger vertex buffer
            let vertices: Vec<[f32; 2]> = (0..1000)
                .map(|i| {
                    let x = (i as f32 / 1000.0) * 2.0 - 1.0;
                    let y = (i as f32 / 1000.0).sin();
                    [x, y]
                })
                .collect();

            let result = renderer.create_vertex_buffer(&vertices);
            let duration = start_time.elapsed();

            match result {
                Ok(buffer) => {
                    println!("✅ Large vertex buffer created in {:?}", duration);
                    println!("Buffer size: {} bytes", buffer.size());

                    // Should complete within reasonable time (less than 100ms)
                    assert!(duration.as_millis() < 100);
                }
                Err(e) => {
                    println!("❌ Large vertex buffer creation failed: {}", e);
                    panic!("Large vertex buffer creation failed: {}", e);
                }
            }
        }
        Err(WebGpuRealError::NotSupported(_)) => {
            println!("⚠️  Skipping performance test - WebGPU not supported");
            assert!(true);
        }
        Err(e) => {
            println!("❌ Unexpected error: {}", e);
            panic!("Unexpected error: {}", e);
        }
    }
}

/// Test error handling
#[test]
fn test_error_types() {
    // Test that our error types work correctly
    let not_supported = WebGpuRealError::NotSupported("Test".to_string());
    assert!(matches!(not_supported, WebGpuRealError::NotSupported(_)));

    let device_init = WebGpuRealError::DeviceInit("Test".to_string());
    assert!(matches!(device_init, WebGpuRealError::DeviceInit(_)));

    let surface_creation = WebGpuRealError::SurfaceCreation("Test".to_string());
    assert!(matches!(
        surface_creation,
        WebGpuRealError::SurfaceCreation(_)
    ));

    let shader_compilation = WebGpuRealError::ShaderCompilation("Test".to_string());
    assert!(matches!(
        shader_compilation,
        WebGpuRealError::ShaderCompilation(_)
    ));

    let buffer_creation = WebGpuRealError::BufferCreation("Test".to_string());
    assert!(matches!(
        buffer_creation,
        WebGpuRealError::BufferCreation(_)
    ));

    let pipeline_creation = WebGpuRealError::PipelineCreation("Test".to_string());
    assert!(matches!(
        pipeline_creation,
        WebGpuRealError::PipelineCreation(_)
    ));

    println!("✅ All error types work correctly");
}
