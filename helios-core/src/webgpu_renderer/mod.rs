//! WebGPU renderer implementation for Helios visualization library
//!
//! This module provides high-performance GPU-accelerated rendering using WebGPU.
//! It includes device management, shader compilation, buffer pooling, and
//! optimized rendering pipelines for different chart types.

mod types;
mod buffer_pool;
mod shader_manager;
mod device_manager;

pub use types::*;
use buffer_pool::BufferPool;
use shader_manager::ShaderManager;
use device_manager::DeviceManager;

// Main WebGPU renderer module

/// WebGPU renderer for high-performance chart rendering
pub struct WebGpuRenderer {
    device_manager: DeviceManager,
    buffer_pool: BufferPool,
    shader_manager: ShaderManager,
    surface: Option<wgpu::Surface<'static>>,
    surface_config: Option<wgpu::SurfaceConfiguration>,
    memory_usage: MemoryUsage,
}

impl WebGpuRenderer {
    /// Create a new WebGPU renderer
    pub async fn new() -> Result<Self, WebGpuError> {
        let device_manager = DeviceManager::new().await?;
        let device = device_manager.device();
        let buffer_pool = BufferPool::new(device.clone());
        let shader_manager = ShaderManager::new(device);

        Ok(Self {
            device_manager,
            buffer_pool,
            shader_manager,
            surface: None,
            surface_config: None,
            memory_usage: MemoryUsage::default(),
        })
    }

    /// Initialize surface for rendering (placeholder implementation)
    pub fn init_surface(&mut self, _window: &dyn std::any::Any) -> Result<(), WebGpuError> {
        // This is a placeholder implementation since winit is not available
        // In a real implementation, this would initialize the surface
        Err(WebGpuError::SurfaceCreationFailed("Surface initialization not implemented without winit".to_string()))
    }

    /// Resize the surface
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), WebGpuError> {
        if let (Some(surface), Some(config)) = (&self.surface, &mut self.surface_config) {
            config.width = width;
            config.height = height;
            surface.configure(&self.device_manager.device(), config);
        }
        Ok(())
    }

    /// Render a frame
    pub fn render(&mut self) -> Result<(), WebGpuError> {
        if let (Some(surface), Some(_config)) = (&self.surface, &self.surface_config) {
            let output = surface.get_current_texture()
                .map_err(|e| WebGpuError::RenderingFailed(format!("Failed to get current texture: {:?}", e)))?;

            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder = self.device_manager.device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

            {
                let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });
            }

            self.device_manager.queue().submit(std::iter::once(encoder.finish()));
            output.present();
        }
        Ok(())
    }

    /// Get memory usage statistics
    pub fn get_memory_usage(&self) -> &MemoryUsage {
        &self.memory_usage
    }

    /// Get buffer pool statistics
    pub fn get_buffer_stats(&self) -> BufferPoolStats {
        self.buffer_pool.get_stats()
    }

    /// Get shader cache statistics
    pub fn get_shader_stats(&self) -> (usize, usize) {
        self.shader_manager.get_cache_stats()
    }

    /// Get adapter information
    pub fn get_adapter_info(&self) -> Option<wgpu::AdapterInfo> {
        self.device_manager.get_adapter_info()
    }

    /// Check if specific features are supported
    pub fn supports_features(&self, features: wgpu::Features) -> bool {
        self.device_manager.supports_features(features)
    }

    /// Get adapter limits
    pub fn get_limits(&self) -> Option<wgpu::Limits> {
        self.device_manager.get_limits()
    }
}

impl Buffer {
    /// Create a new buffer wrapper
    pub fn new(buffer: wgpu::Buffer, size: u64, usage: wgpu::BufferUsages) -> Self {
        Self {
            buffer,
            size,
            usage,
        }
    }

    /// Get the underlying WebGPU buffer
    pub fn get_buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    /// Get buffer size
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Get buffer usage flags
    pub fn usage(&self) -> wgpu::BufferUsages {
        self.usage
    }
}
