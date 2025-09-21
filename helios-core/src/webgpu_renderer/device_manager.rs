//! WebGPU device and adapter management

use super::types::WebGpuError;
use std::sync::Arc;

/// WebGPU device manager
pub struct DeviceManager {
    instance: wgpu::Instance,
    adapter: Option<wgpu::Adapter>,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
}

impl DeviceManager {
    /// Create a new device manager
    pub async fn new() -> Result<Self, WebGpuError> {
        // Create WebGPU instance
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Request adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .map_err(|e| WebGpuError::AdapterRequestFailed(format!("{:?}", e)))?;

        // Request device
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
                trace: wgpu::Trace::default(),
            })
            .await?;

        Ok(Self {
            instance,
            adapter: Some(adapter),
            device: Arc::new(device),
            queue: Arc::new(queue),
        })
    }

    /// Get the WebGPU instance
    pub fn instance(&self) -> &wgpu::Instance {
        &self.instance
    }

    /// Get the WebGPU adapter
    pub fn adapter(&self) -> Option<&wgpu::Adapter> {
        self.adapter.as_ref()
    }

    /// Get the WebGPU device
    pub fn device(&self) -> Arc<wgpu::Device> {
        self.device.clone()
    }

    /// Get the WebGPU queue
    pub fn queue(&self) -> Arc<wgpu::Queue> {
        self.queue.clone()
    }

    /// Create a surface for the given window (placeholder implementation)
    pub fn create_surface(&self, _window: &dyn std::any::Any) -> Result<wgpu::Surface, WebGpuError> {
        // This is a placeholder implementation since winit is not available
        // In a real implementation, this would create a surface from the window
        Err(WebGpuError::SurfaceCreationFailed("Surface creation not implemented without winit".to_string()))
    }

    /// Configure surface for rendering
    pub fn configure_surface(
        &self,
        surface: &wgpu::Surface,
        width: u32,
        height: u32,
    ) -> Result<wgpu::SurfaceConfiguration, WebGpuError> {
        let surface_caps = surface.get_capabilities(&self.adapter().unwrap());
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&self.device, &config);
        Ok(config)
    }

    /// Get adapter info
    pub fn get_adapter_info(&self) -> Option<wgpu::AdapterInfo> {
        self.adapter.as_ref().map(|adapter| adapter.get_info())
    }

    /// Check if adapter supports specific features
    pub fn supports_features(&self, features: wgpu::Features) -> bool {
        self.adapter
            .as_ref()
            .map(|adapter| adapter.features().contains(features))
            .unwrap_or(false)
    }

    /// Get adapter limits
    pub fn get_limits(&self) -> Option<wgpu::Limits> {
        self.adapter.as_ref().map(|adapter| adapter.limits())
    }
}
