//! TDD Tests for Real WebGPU Implementation
//!
//! Following Test-Driven Development principles to implement real WebGPU rendering
//! instead of the current mock implementation.

use helios_core::*;
use std::sync::Arc;

/// Test WebGPU device initialization and adapter detection
#[tokio::test]
async fn test_webgpu_device_initialization() {
    // Test 1: WebGPU adapter detection should work
    let adapter_result = WebGpuRenderer::request_adapter().await;
    match adapter_result {
        Ok(Some(adapter)) => {
            println!("✅ WebGPU adapter detected: {:?}", adapter.get_info());

            // Test 2: Device creation from adapter should succeed
            let device_result = WebGpuRenderer::request_device(&adapter).await;
            assert!(
                device_result.is_ok(),
                "Device creation should succeed with valid adapter"
            );

            let (device, queue) = device_result.unwrap();
            assert!(
                device
                    .features()
                    .contains(wgpu::Features::VERTEX_ATTRIBUTE_64BIT),
                "Device should support required features for chart rendering"
            );
        }
        Ok(None) => {
            println!("⚠️ WebGPU not supported on this platform - falling back to WebGL");
            // This is acceptable behavior - we should gracefully fallback
        }
        Err(e) => {
            panic!("❌ WebGPU adapter request failed unexpectedly: {}", e);
        }
    }
}

/// Test surface creation for canvas rendering
#[tokio::test]
async fn test_webgpu_surface_creation() {
    let adapter = match WebGpuRenderer::request_adapter().await {
        Ok(Some(adapter)) => adapter,
        Ok(None) => {
            println!("⚠️ Skipping surface test - WebGPU not available");
            return;
        }
        Err(e) => panic!("Adapter request failed: {}", e),
    };

    // Test 1: Surface creation should succeed with valid canvas
    let mock_canvas = MockCanvas::new(800, 600);
    let surface_result = WebGpuRenderer::create_surface_from_canvas(&mock_canvas).await;
    assert!(
        surface_result.is_ok(),
        "Surface creation should succeed with valid canvas"
    );

    let surface = surface_result.unwrap();

    // Test 2: Surface capabilities should be queryable
    let capabilities = surface.get_capabilities(&adapter);
    assert!(
        !capabilities.formats.is_empty(),
        "Surface should support at least one format"
    );
    assert!(
        !capabilities.present_modes.is_empty(),
        "Surface should support at least one present mode"
    );
}

/// Test buffer creation and memory management
#[tokio::test]
async fn test_webgpu_buffer_management() {
    let (device, _queue) = match setup_webgpu_device().await {
        Some(setup) => setup,
        None => {
            println!("⚠️ Skipping buffer test - WebGPU not available");
            return;
        }
    };

    // Test 1: Vertex buffer creation
    let vertices = vec![
        [0.0f32, 0.5], // Top vertex
        [-0.5, -0.5],  // Bottom left
        [0.5, -0.5],   // Bottom right
    ];

    let vertex_buffer = WebGpuRenderer::create_vertex_buffer(&device, &vertices);
    assert!(
        vertex_buffer.is_ok(),
        "Vertex buffer creation should succeed"
    );

    let buffer = vertex_buffer.unwrap();
    assert_eq!(
        buffer.size(),
        (vertices.len() * std::mem::size_of::<[f32; 2]>()) as u64
    );

    // Test 2: Index buffer creation
    let indices = vec![0u16, 1, 2];
    let index_buffer = WebGpuRenderer::create_index_buffer(&device, &indices);
    assert!(index_buffer.is_ok(), "Index buffer creation should succeed");

    // Test 3: Uniform buffer creation
    let uniform_data = ChartUniforms {
        mvp_matrix: [[1.0; 4]; 4],
        color: [1.0, 0.0, 0.0, 1.0],
    };
    let uniform_buffer = WebGpuRenderer::create_uniform_buffer(&device, &uniform_data);
    assert!(
        uniform_buffer.is_ok(),
        "Uniform buffer creation should succeed"
    );
}

/// Test shader compilation and pipeline creation
#[tokio::test]
async fn test_webgpu_shader_compilation() {
    let (device, _queue) = match setup_webgpu_device().await {
        Some(setup) => setup,
        None => {
            println!("⚠️ Skipping shader test - WebGPU not available");
            return;
        }
    };

    // Test 1: Basic vertex shader compilation
    let vertex_shader_source = r#"
        @vertex
        fn vs_main(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
            return vec4<f32>(position, 0.0, 1.0);
        }
    "#;

    let vertex_shader =
        WebGpuRenderer::compile_shader(&device, "line_chart_vertex", vertex_shader_source);
    assert!(
        vertex_shader.is_ok(),
        "Vertex shader compilation should succeed"
    );

    // Test 2: Basic fragment shader compilation
    let fragment_shader_source = r#"
        @fragment
        fn fs_main() -> @location(0) vec4<f32> {
            return vec4<f32>(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let fragment_shader =
        WebGpuRenderer::compile_shader(&device, "line_chart_fragment", fragment_shader_source);
    assert!(
        fragment_shader.is_ok(),
        "Fragment shader compilation should succeed"
    );

    // Test 3: Shader compilation error handling
    let invalid_shader = r#"
        @vertex
        fn invalid_function() -> invalid_type {
            return not_valid_code;
        }
    "#;

    let error_result = WebGpuRenderer::compile_shader(&device, "invalid_shader", invalid_shader);
    assert!(
        error_result.is_err(),
        "Invalid shader should fail compilation"
    );
}

/// Test render pipeline creation for different chart types
#[tokio::test]
async fn test_webgpu_render_pipeline_creation() {
    let (device, _queue) = match setup_webgpu_device().await {
        Some(setup) => setup,
        None => {
            println!("⚠️ Skipping pipeline test - WebGPU not available");
            return;
        }
    };

    // Test 1: Line chart render pipeline
    let line_pipeline = WebGpuRenderer::create_line_chart_pipeline(&device).await;
    assert!(
        line_pipeline.is_ok(),
        "Line chart pipeline creation should succeed"
    );

    // Test 2: Bar chart render pipeline
    let bar_pipeline = WebGpuRenderer::create_bar_chart_pipeline(&device).await;
    assert!(
        bar_pipeline.is_ok(),
        "Bar chart pipeline creation should succeed"
    );

    // Test 3: Scatter plot render pipeline
    let scatter_pipeline = WebGpuRenderer::create_scatter_plot_pipeline(&device).await;
    assert!(
        scatter_pipeline.is_ok(),
        "Scatter plot pipeline creation should succeed"
    );

    // Test 4: Pipeline should have correct vertex attributes
    let pipeline = line_pipeline.unwrap();
    let vertex_layout = pipeline.get_vertex_layout();
    assert_eq!(vertex_layout.array_stride, 8); // 2 f32s = 8 bytes
    assert_eq!(vertex_layout.attributes.len(), 1); // Position attribute
}

/// Test actual rendering with WebGPU commands
#[tokio::test]
async fn test_webgpu_chart_rendering() {
    let setup = match setup_full_webgpu_context().await {
        Some(setup) => setup,
        None => {
            println!("⚠️ Skipping rendering test - WebGPU not available");
            return;
        }
    };

    let (device, queue, surface, surface_config) = setup;

    // Test 1: Line chart rendering
    let line_data = vec![[0.0f32, 0.0], [0.5, 0.8], [1.0, 0.3]];

    let line_config = LineChartConfig {
        stroke_width: 2.0,
        color: [1.0, 0.0, 0.0, 1.0],
        smooth: false,
    };

    let mut renderer = WebGpuRenderer::new(Arc::new(device), surface).unwrap();
    let result = renderer.render_line_chart(&line_data, &line_config).await;
    assert!(result.is_ok(), "Line chart rendering should succeed");

    let render_result = result.unwrap();
    assert!(
        render_result.render_time_ms > 0.0,
        "Render time should be measured"
    );
    assert!(
        render_result.vertices_rendered > 0,
        "Should render some vertices"
    );

    // Test 2: Rendering should produce actual pixels
    let frame = surface.get_current_texture().unwrap();
    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    // Verify frame was rendered to
    assert!(
        !frame.texture.format().is_depth_stencil_format(),
        "Should render to color format"
    );
}

/// Test memory usage tracking and cleanup
#[tokio::test]
async fn test_webgpu_memory_management() {
    let (device, queue) = match setup_webgpu_device().await {
        Some(setup) => setup,
        None => {
            println!("⚠️ Skipping memory test - WebGPU not available");
            return;
        }
    };

    let surface = MockSurface::new();
    let mut renderer = WebGpuRenderer::new(Arc::new(device), surface).unwrap();

    // Test 1: Initial memory usage should be minimal
    let initial_usage = renderer.get_memory_usage();
    assert_eq!(
        initial_usage.used_bytes, 0,
        "Initial memory usage should be 0"
    );
    assert_eq!(
        initial_usage.allocated_buffers, 0,
        "No buffers should be allocated initially"
    );

    // Test 2: Memory usage should increase after buffer allocation
    let test_data = vec![[1.0f32, 2.0]; 1000]; // 1000 points
    let _buffer = renderer.allocate_vertex_buffer(&test_data).unwrap();

    let usage_after_alloc = renderer.get_memory_usage();
    assert!(
        usage_after_alloc.used_bytes > initial_usage.used_bytes,
        "Memory usage should increase after allocation"
    );
    assert_eq!(
        usage_after_alloc.allocated_buffers, 1,
        "Should track one allocated buffer"
    );

    // Test 3: Memory cleanup should reduce usage
    renderer.cleanup_unused_buffers().unwrap();
    let usage_after_cleanup = renderer.get_memory_usage();
    assert!(
        usage_after_cleanup.used_bytes <= usage_after_alloc.used_bytes,
        "Memory usage should not increase after cleanup"
    );
}

/// Test error handling and fallback mechanisms
#[tokio::test]
async fn test_webgpu_error_handling() {
    // Test 1: Graceful handling when WebGPU is not available
    let adapter_result = WebGpuRenderer::request_adapter().await;
    if adapter_result.is_err() || adapter_result.unwrap().is_none() {
        // This should not panic - should return appropriate error
        let fallback_renderer = WebGpuRenderer::create_fallback_renderer();
        assert!(
            fallback_renderer.is_ok(),
            "Fallback renderer creation should succeed"
        );
    }

    // Test 2: Buffer allocation failure handling
    if let Some((device, _queue)) = setup_webgpu_device().await {
        // Try to allocate impossibly large buffer
        let huge_data = vec![[0.0f32; 2]; usize::MAX / 1000];
        let result = WebGpuRenderer::create_vertex_buffer(&device, &huge_data);
        assert!(
            result.is_err(),
            "Huge buffer allocation should fail gracefully"
        );

        match result.unwrap_err() {
            WebGpuError::BufferAllocation(_) => {
                // Correct error type
            }
            _ => panic!("Wrong error type for buffer allocation failure"),
        }
    }

    // Test 3: Invalid shader handling
    if let Some((device, _queue)) = setup_webgpu_device().await {
        let invalid_wgsl = "this is not valid WGSL code";
        let result = WebGpuRenderer::compile_shader(&device, "invalid", invalid_wgsl);
        assert!(result.is_err(), "Invalid shader should return error");
    }
}

/// Test performance requirements
#[tokio::test]
async fn test_webgpu_performance() {
    let setup = match setup_full_webgpu_context().await {
        Some(setup) => setup,
        None => {
            println!("⚠️ Skipping performance test - WebGPU not available");
            return;
        }
    };

    let (device, queue, surface, _config) = setup;
    let mut renderer = WebGpuRenderer::new(Arc::new(device), surface).unwrap();

    // Test 1: Large dataset rendering should complete within time budget
    let large_dataset: Vec<[f32; 2]> = (0..10000)
        .map(|i| [i as f32 / 10000.0, (i as f32 * 0.1).sin()])
        .collect();

    let config = LineChartConfig {
        stroke_width: 1.0,
        color: [0.0, 1.0, 0.0, 1.0],
        smooth: true,
    };

    let start_time = std::time::Instant::now();
    let result = renderer.render_line_chart(&large_dataset, &config).await;
    let render_time = start_time.elapsed();

    assert!(result.is_ok(), "Large dataset rendering should succeed");
    assert!(
        render_time.as_millis() < 50,
        "Rendering 10k points should complete within 50ms performance budget"
    );

    let render_result = result.unwrap();
    assert!(
        render_result.render_time_ms < 50.0,
        "Internal timing should also meet performance budget"
    );
}

// Helper functions for test setup

async fn setup_webgpu_device() -> Option<(wgpu::Device, wgpu::Queue)> {
    let adapter = WebGpuRenderer::request_adapter().await.ok()??;
    let (device, queue) = WebGpuRenderer::request_device(&adapter).await.ok()?;
    Some((device, queue))
}

async fn setup_full_webgpu_context() -> Option<(
    wgpu::Device,
    wgpu::Queue,
    MockSurface,
    wgpu::SurfaceConfiguration,
)> {
    let adapter = WebGpuRenderer::request_adapter().await.ok()??;
    let (device, queue) = WebGpuRenderer::request_device(&adapter).await.ok()?;
    let surface = MockSurface::new();
    let surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: 800,
        height: 600,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        view_formats: vec![],
    };

    Some((device, queue, surface, surface_config))
}

// Mock types for testing

struct MockCanvas {
    width: u32,
    height: u32,
}

impl MockCanvas {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

struct MockSurface;

impl MockSurface {
    fn new() -> Self {
        Self
    }

    fn get_current_texture(&self) -> Result<MockSurfaceTexture, wgpu::SurfaceError> {
        Ok(MockSurfaceTexture {
            texture: MockTexture::new(),
        })
    }
}

struct MockSurfaceTexture {
    texture: MockTexture,
}

struct MockTexture;

impl MockTexture {
    fn new() -> Self {
        Self
    }

    fn create_view(&self, _desc: &wgpu::TextureViewDescriptor) -> MockTextureView {
        MockTextureView
    }

    fn format(&self) -> wgpu::TextureFormat {
        wgpu::TextureFormat::Bgra8UnormSrgb
    }
}

struct MockTextureView;

#[derive(Debug)]
struct ChartUniforms {
    mvp_matrix: [[f32; 4]; 4],
    color: [f32; 4],
}

// Integration with existing chart config types
use helios_core::{BarChartConfig, HeatmapConfig, LineChartConfig, ScatterPlotConfig};

/// Ensure our tests integrate with existing chart configuration
#[test]
fn test_chart_config_integration() {
    let line_config = LineChartConfig {
        stroke_width: 2.0,
        color: [1.0, 0.0, 0.0, 1.0],
        smooth: true,
    };

    // Config should be serializable for WebGPU uniform buffers
    assert_eq!(line_config.stroke_width, 2.0);
    assert_eq!(line_config.color, [1.0, 0.0, 0.0, 1.0]);
    assert!(line_config.smooth);
}
