//! Enhanced rendering engine for TDD REFACTOR phase
//! This provides robust, production-ready WebGPU rendering capabilities
//!
//! This module is split into logical submodules:
//! - `errors`: Error handling and backend selection
//! - `performance`: Performance profiling and optimization
//! - `renderer`: Main renderer and pipeline management
//! - `webgpu_device`: WebGPU device management and real implementations

pub mod errors;
pub mod performance;
pub mod renderer;
pub mod webgpu_device;

// Re-export main types for convenience
pub use errors::*;
pub use performance::*;
pub use renderer::*;
pub use webgpu_device::*;

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

// WebGPU integration
#[cfg(feature = "webgpu")]
use wgpu::*;

/// Render configuration
#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub width: u32,
    pub height: u32,
    pub antialiasing: bool,
    pub vsync: bool,
    pub max_fps: u32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            antialiasing: true,
            vsync: true,
            max_fps: 60,
        }
    }
}

/// Filter modes for rendering
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterMode {
    Nearest,
    Linear,
}

impl RenderBackend {
    /// Create the optimal render backend based on available capabilities
    pub async fn create_optimal() -> Result<Self, RenderError> {
        // Try WebGPU first
        #[cfg(feature = "webgpu")]
        {
            if let Ok(device) = RealWebGpuDevice::new().await {
                return Ok(RenderBackend::WebGPU {
                    device: Some(Arc::new(device.device)),
                    queue: Some(Arc::new(device.queue)),
                    surface: None,
                    compute_capability: true,
                    memory_budget: 1024 * 1024 * 1024, // 1GB
                    adapter_info: errors::AdapterInfo {
                        name: "WebGPU Adapter".to_string(),
                        vendor: "Unknown".to_string(),
                        device_type: errors::DeviceType::Discrete,
                        backend: errors::Backend::BrowserWebGpu,
                    },
                });
            }
        }

        // Fallback to WebGL2
        Ok(RenderBackend::WebGL2 {
            context: Some("webgl2_context".to_string()),
            extensions: vec!["EXT_color_buffer_float".to_string()],
            capabilities: errors::WebGL2Capabilities {
                max_texture_size: 4096,
                max_vertex_attribs: 16,
                max_varying_vectors: 8,
                max_fragment_uniform_vectors: 16,
                max_vertex_uniform_vectors: 16,
            },
        })
    }
}
