# WebGPU Abstraction and Fallback Strategy

## ⚠️ Implementation Status
- **Status**: 📋 Planned - Core rendering pipeline not yet implemented
- **ETA**: Q1 2026
- **Current State**: Type definitions and API surface only

## Overview

Leptos Helios implements a sophisticated WebGPU abstraction layer with comprehensive fallback support to ensure maximum browser compatibility and performance. This document outlines our approach to handling different rendering backends and the graceful degradation strategy.

## Architecture

### 1. Rendering Backend Hierarchy

Our rendering system follows a three-tier fallback approach:

```
WebGPU (Primary) → WebGL2 (Fallback) → Canvas2D (Last Resort)
```

#### WebGPU (Primary Backend)
- **Target**: Modern browsers with WebGPU support
- **Performance**: Highest performance, GPU-accelerated rendering
- **Features**: Full shader support, compute shaders, advanced rendering techniques
- **Browser Support**: Chrome 113+, Firefox 110+ (with flags), Safari 16.4+

#### WebGL2 (Fallback Backend)
- **Target**: Browsers without WebGPU but with WebGL2 support
- **Performance**: High performance, GPU-accelerated rendering
- **Features**: Shader support, most rendering techniques
- **Browser Support**: Chrome 56+, Firefox 51+, Safari 15+

#### Canvas2D (Last Resort)
- **Target**: Legacy browsers or when GPU acceleration fails
- **Performance**: CPU-based rendering, lower performance
- **Features**: Basic 2D rendering, no shaders
- **Browser Support**: All modern browsers

### 2. Core Components

#### CanvasSurface (`helios-core/src/canvas_surface.rs`)

The `CanvasSurface` is the central abstraction that manages the rendering context and handles fallback logic.

```rust
pub struct CanvasSurface {
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
    device: Arc<Device>,
    queue: Arc<Queue>,
    canvas_element: HtmlCanvasElement,
    fallback_support: FallbackSupport,
}
```

**Key Features:**
- **Automatic Detection**: Detects available rendering backends at runtime
- **Graceful Fallback**: Seamlessly switches between backends when needed
- **WASM Integration**: Full support for WebAssembly environments
- **Error Handling**: Comprehensive error handling for unsupported environments

#### FallbackSupport Structure

```rust
pub struct FallbackSupport {
    pub webgpu_supported: bool,
    pub webgl2_supported: bool,
    pub canvas2d_supported: bool,
}
```

This structure is populated at runtime and used to determine the best available rendering backend.

### 3. Implementation Strategy

#### Runtime Detection

```rust
impl CanvasSurface {
    #[cfg(target_arch = "wasm32")]
    pub async fn from_canvas_element(canvas: &HtmlCanvasElement) -> Result<Self, CanvasSurfaceError> {
        // 1. Try WebGPU first
        if let Ok(webgpu_surface) = Self::try_webgpu(canvas).await {
            return Ok(webgpu_surface);
        }

        // 2. Fall back to WebGL2
        if let Ok(webgl2_surface) = Self::try_webgl2(canvas) {
            return Ok(webgl2_surface);
        }

        // 3. Fall back to Canvas2D
        if let Ok(canvas2d_surface) = Self::try_canvas2d(canvas) {
            return Ok(canvas2d_surface);
        }

        Err(CanvasSurfaceError::NoSupportedBackend)
    }
}
```

#### Backend-Specific Implementations

Each rendering backend has its own implementation module:

- **WebGPU**: `helios-core/src/webgpu_real.rs` - Full WebGPU implementation
- **WebGL2**: `helios-core/src/renderer.rs` - WebGL2 fallback implementation
- **Canvas2D**: `helios-core/src/renderer.rs` - Canvas2D fallback implementation

### 4. Error Handling Strategy

#### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum CanvasSurfaceError {
    #[error("WebGPU not supported: {0}")]
    WebGpuNotSupported(String),

    #[error("Context creation failed: {0}")]
    ContextCreationFailed(String),

    #[error("Surface creation failed: {0}")]
    SurfaceCreationFailed(String),

    #[error("No supported rendering backend available")]
    NoSupportedBackend,
}
```

#### Graceful Degradation

When a higher-performance backend fails, the system automatically falls back to the next available option:

1. **WebGPU Failure**: Falls back to WebGL2
2. **WebGL2 Failure**: Falls back to Canvas2D
3. **All Backends Fail**: Returns a descriptive error

### 5. Performance Considerations

#### Backend Selection Logic

The system prioritizes backends based on:
1. **Performance**: WebGPU > WebGL2 > Canvas2D
2. **Feature Support**: WebGPU > WebGL2 > Canvas2D
3. **Browser Compatibility**: Canvas2D > WebGL2 > WebGPU

#### Feature Detection

```rust
impl CanvasSurface {
    pub fn is_webgpu_supported() -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            // Check for WebGPU support in browser
            web_sys::window()
                .and_then(|w| w.navigator())
                .and_then(|n| n.gpu())
                .is_some()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            false // WebGPU only available in browsers
        }
    }

    fn check_fallback_support() -> FallbackSupport {
        FallbackSupport {
            webgpu_supported: Self::is_webgpu_supported(),
            webgl2_supported: Self::is_webgl2_supported(),
            canvas2d_supported: true, // Always available
        }
    }
}
```

### 6. WASM Integration

#### Browser-Specific Code

```rust
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

#[cfg(target_arch = "wasm32")]
impl CanvasSurface {
    pub async fn from_canvas_element(canvas: &HtmlCanvasElement) -> Result<Self, CanvasSurfaceError> {
        // WebGPU surface creation for WASM targets
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: None,
            force_fallback_adapter: false,
        }).await
        .map_err(|e| CanvasSurfaceError::WebGpuNotSupported(format!("Adapter request failed: {}", e)))?;

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: Default::default(),
            trace: Default::default(),
        }).await
        .map_err(|e| CanvasSurfaceError::ContextCreationFailed(format!("Device request failed: {}", e)))?;

        let surface = instance.create_surface(canvas)
            .map_err(|e| CanvasSurfaceError::SurfaceCreationFailed(format!("Failed to create surface: {}", e)))?;

        // Configure surface
        let surface_config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_default_config(&adapter, 800, 600).unwrap().format,
            width: 800,
            height: 600,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &surface_config);

        let fallback_support = Self::check_fallback_support();

        Ok(Self {
            surface,
            surface_config,
            device: Arc::new(device),
            queue: Arc::new(queue),
            canvas_element: canvas.clone(),
            fallback_support,
        })
    }
}
```

### 7. Testing Strategy

#### Unit Tests

- **Backend Detection**: Tests for each backend's availability detection
- **Fallback Logic**: Tests for graceful fallback between backends
- **Error Handling**: Tests for proper error propagation

#### Integration Tests

- **WASM Tests**: Browser-specific tests using `wasm-bindgen-test`
- **Cross-Browser**: Tests across different browser environments
- **Performance**: Benchmarks for each backend

#### Test Structure

```rust
#[wasm_bindgen_test]
async fn test_webgpu_surface_creation() {
    let canvas = create_test_canvas();
    let canvas_surface = CanvasSurface::from_canvas_element(&canvas).await;
    assert!(canvas_surface.is_ok());

    let webgpu_supported = CanvasSurface::is_webgpu_supported();
    assert!(webgpu_supported);

    let fallback_support = CanvasSurface::check_fallback_support();
    assert!(fallback_support.webgl2_supported || fallback_support.canvas2d_supported);
}
```

### 8. Future Considerations

#### WebGPU Evolution

- **API Changes**: Monitor WebGPU specification changes
- **Feature Additions**: Support for new WebGPU features as they become available
- **Performance Optimizations**: Continuous optimization for WebGPU rendering

#### Browser Support

- **Progressive Enhancement**: Add support for new browsers as they implement WebGPU
- **Feature Detection**: Enhanced feature detection for better fallback decisions
- **Performance Monitoring**: Runtime performance monitoring to optimize backend selection

### 9. Configuration

#### Nginx Configuration

The nginx configuration includes proper headers for WebGPU support:

```nginx
location ~* \.wasm$ {
    add_header Content-Type application/wasm;
    add_header Cross-Origin-Embedder-Policy require-corp;
    add_header Cross-Origin-Opener-Policy same-origin;
}
```

These headers are required for WebGPU to function properly in browsers.

### 10. Best Practices

#### Development

1. **Always Test Fallbacks**: Test your application with each rendering backend
2. **Graceful Degradation**: Ensure features work across all backends
3. **Performance Monitoring**: Monitor performance across different backends
4. **Error Handling**: Implement comprehensive error handling for backend failures

#### Deployment

1. **Browser Testing**: Test across different browsers and versions
2. **Feature Detection**: Use runtime feature detection rather than user agent sniffing
3. **Progressive Enhancement**: Build for the lowest common denominator, enhance for better backends
4. **Monitoring**: Monitor backend usage and performance in production

## Conclusion

The WebGPU abstraction and fallback strategy in Leptos Helios provides a robust, performant, and compatible rendering solution that works across all modern browsers. By implementing a three-tier fallback system with comprehensive error handling and runtime detection, we ensure that users get the best possible rendering experience regardless of their browser's capabilities.

This approach allows developers to leverage the latest WebGPU features while maintaining compatibility with older browsers, making Leptos Helios a truly universal charting solution for the web.
