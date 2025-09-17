//! Rendering error types and backend selection
//!
//! This module provides comprehensive error handling and backend selection
//! for the rendering system, including WebGPU, WebGL2, and Canvas2D backends.

use std::sync::Arc;

// WebGPU integration
#[cfg(feature = "webgpu")]
use wgpu::*;

/// Rendering error types
#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("WebGPU error: {0}")]
    WebGPU(String),

    #[error("WebGL error: {0}")]
    WebGL(String),

    #[error("Canvas error: {0}")]
    Canvas(String),

    #[error("Buffer error: {0}")]
    Buffer(String),

    #[error("Shader error: {0}")]
    Shader(String),

    #[error("Performance error: {0}")]
    Performance(String),
}

/// Render backend selection
#[derive(Debug, Clone)]
pub enum RenderBackend {
    /// Primary: WebGPU for maximum performance
    WebGPU {
        device: Option<Arc<Device>>,
        queue: Option<Arc<Queue>>,
        surface: Option<Arc<Surface<'static>>>,
        compute_capability: bool,
        memory_budget: usize,
        adapter_info: AdapterInfo,
    },

    /// Fallback: WebGL2 for broad compatibility
    WebGL2 {
        context: Option<String>, // Placeholder for WebGL2RenderingContext
        extensions: Vec<String>,
        capabilities: WebGL2Capabilities,
    },

    /// Last resort: Canvas 2D for universal support
    Canvas2D {
        context: Option<String>, // Placeholder for CanvasRenderingContext2d
    },
}

#[derive(Debug, Clone)]
pub struct AdapterInfo {
    pub name: String,
    pub vendor: String,
    pub device_type: DeviceType,
    pub backend: Backend,
}

#[derive(Debug, Clone)]
pub struct WebGL2Capabilities {
    pub max_texture_size: u32,
    pub max_vertex_attribs: u32,
    pub max_varying_vectors: u32,
    pub max_fragment_uniform_vectors: u32,
    pub max_vertex_uniform_vectors: u32,
}

pub struct WebGpuCapabilities {
    pub max_texture_size: u32,
    pub max_buffer_size: u64,
    pub supported_formats: Vec<String>,
    pub compute_shader_support: bool,
}

#[derive(Debug, Clone)]
pub enum DeviceType {
    Discrete,
    Integrated,
    Virtual,
    Cpu,
}

#[derive(Debug, Clone)]
pub enum Backend {
    Vulkan,
    Metal,
    Dx12,
    Dx11,
    Gl,
    BrowserWebGpu,
}
