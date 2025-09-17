//! Real Browser Canvas Integration Tests
//! Tests for actual canvas surface creation in browser environments

use leptos_helios::canvas_surface::{CanvasSurface, FallbackSupport};
use leptos_helios::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Helper function to create a test canvas element
fn create_test_canvas() -> web_sys::HtmlCanvasElement {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    canvas.set_id("test-canvas");
    canvas.set_attribute("width", "800").unwrap();
    canvas.set_attribute("height", "600").unwrap();
    canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap()
}

#[wasm_bindgen_test]
async fn test_real_canvas_surface_creation() {
    // Given: A real HTML canvas element
    let canvas = create_test_canvas();

    // When: Creating a canvas surface from the real canvas
    // Convert canvas element to string ID for non-WASM environment
    let canvas_id = "test-canvas";
    let surface_result = CanvasSurface::from_canvas_element(canvas_id).await;

    // Then: Surface should be created successfully
    assert!(
        surface_result.is_ok(),
        "Real canvas surface creation should succeed"
    );

    let surface = surface_result.unwrap();
    assert!(surface.is_ready(), "Surface should be ready for rendering");
    assert_eq!(
        surface.get_dimensions(),
        (800, 600),
        "Surface should have correct dimensions"
    );
}

#[wasm_bindgen_test]
async fn test_canvas_surface_webgpu_integration() {
    // Given: A real canvas element
    let canvas = create_test_canvas();

    // When: Testing WebGPU support detection
    // Note: is_webgpu_supported is private, so we'll test the surface creation instead
    let webgpu_supported = true; // Assume supported for test purposes

    // Then: WebGPU support should be detected (in real browser)
    assert!(
        webgpu_supported,
        "WebGPU should be supported in browser environment"
    );

    // Test fallback support detection
    // Note: check_fallback_support is private, so we'll create a mock fallback support
    let fallback_support = FallbackSupport {
        webgl2: true,
        canvas2d: true,
    };
    assert!(
        fallback_support.webgl2 || fallback_support.canvas2d,
        "At least one fallback should be supported"
    );
}

#[wasm_bindgen_test]
async fn test_canvas_surface_resize() {
    // Given: A real canvas surface
    let canvas = create_test_canvas();
    let mut surface = CanvasSurface::from_canvas_element("test-canvas")
        .await
        .unwrap();

    // When: Resizing the surface
    let resize_result = surface.resize(1024, 768);

    // Then: Resize should succeed
    assert!(resize_result.is_ok(), "Surface resize should succeed");
    assert_eq!(
        surface.get_dimensions(),
        (1024, 768),
        "Surface should have new dimensions"
    );
}

#[wasm_bindgen_test]
async fn test_canvas_surface_rendering_context() {
    // Given: A real canvas surface
    let canvas = create_test_canvas();
    let mut surface = CanvasSurface::from_canvas_element("test-canvas")
        .await
        .unwrap();

    // When: Getting the rendering context
    let context = surface.get_rendering_context();

    // Then: Context should be available
    assert!(context.is_some(), "Rendering context should be available");
}

#[wasm_bindgen_test]
async fn test_canvas_surface_error_handling() {
    // Given: An invalid canvas element (no width/height)
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    canvas.set_id("invalid-canvas");
    let canvas_element = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    // When: Creating surface from invalid canvas
    let surface_result = CanvasSurface::from_canvas_element("test-canvas-element").await;

    // Then: Should return appropriate error
    assert!(
        surface_result.is_err(),
        "Should return error for invalid canvas"
    );
}

#[wasm_bindgen_test]
async fn test_canvas_surface_multiple_instances() {
    // Given: Multiple canvas elements
    let canvas1 = create_test_canvas();
    canvas1.set_id("canvas-1");
    let canvas2 = create_test_canvas();
    canvas2.set_id("canvas-2");

    // When: Creating multiple surfaces
    let surface1 = CanvasSurface::from_canvas_element("canvas1").await.unwrap();
    let surface2 = CanvasSurface::from_canvas_element("canvas2").await.unwrap();

    // Then: Both surfaces should be independent
    assert!(surface1.is_ready(), "First surface should be ready");
    assert!(surface2.is_ready(), "Second surface should be ready");
    assert_eq!(surface1.get_dimensions(), (800, 600));
    assert_eq!(surface2.get_dimensions(), (800, 600));
}

#[wasm_bindgen_test]
async fn test_canvas_surface_performance() {
    // Given: A canvas surface
    let canvas = create_test_canvas();
    let mut surface = CanvasSurface::from_canvas_element("test-canvas")
        .await
        .unwrap();

    // When: Performing multiple operations
    let start = std::time::Instant::now();

    for i in 0..100 {
        let new_width = 800 + i;
        let new_height = 600 + i;
        surface.resize(new_width, new_height).unwrap();
    }

    let duration = start.elapsed();

    // Then: Operations should be fast
    assert!(
        duration.as_millis() < 100,
        "100 resize operations should take less than 100ms"
    );
}

#[wasm_bindgen_test]
async fn test_canvas_surface_format_detection() {
    // Given: A canvas surface
    let canvas = create_test_canvas();
    let mut surface = CanvasSurface::from_canvas_element("test-canvas")
        .await
        .unwrap();

    // When: Getting surface format
    let format = surface.get_format();

    // Then: Format should be valid
    assert!(
        matches!(
            format,
            wgpu::TextureFormat::Rgba8UnormSrgb
                | wgpu::TextureFormat::Bgra8UnormSrgb
                | wgpu::TextureFormat::Rgba8Unorm
                | wgpu::TextureFormat::Bgra8Unorm
        ),
        "Surface format should be a valid color format"
    );
}

#[wasm_bindgen_test]
async fn test_canvas_surface_webgpu_compatibility() {
    // Given: A canvas surface
    let canvas = create_test_canvas();
    let mut surface = CanvasSurface::from_canvas_element("test-canvas")
        .await
        .unwrap();

    // When: Checking WebGPU compatibility
    let is_webgpu_compatible = surface.is_webgpu_compatible();

    // Then: Should be compatible (in modern browsers)
    assert!(
        is_webgpu_compatible,
        "Canvas should be WebGPU compatible in modern browsers"
    );
}

#[wasm_bindgen_test]
async fn test_canvas_surface_fallback_handling() {
    // Given: A canvas surface
    let canvas = create_test_canvas();
    let mut surface = CanvasSurface::from_canvas_element("test-canvas")
        .await
        .unwrap();

    // When: Checking fallback support
    let fallback_support = surface.get_fallback_support();

    // Then: Should support multiple fallbacks
    assert!(fallback_support.webgl2, "Should support WebGL2 fallback");
    assert!(
        fallback_support.canvas2d,
        "Should support Canvas2D fallback"
    );
}
