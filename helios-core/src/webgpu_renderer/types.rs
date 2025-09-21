//! WebGPU renderer types and error definitions

use thiserror::Error;

/// WebGPU-specific error types
#[derive(Error, Debug)]
pub enum WebGpuError {
    #[error("Failed to create WebGPU instance: {0}")]
    InstanceCreationFailed(String),
    
    #[error("Failed to request adapter: {0}")]
    AdapterRequestFailed(String),
    
    #[error("Failed to request device: {0}")]
    DeviceRequestFailed(#[from] wgpu::RequestDeviceError),
    
    #[error("Failed to create surface: {0}")]
    SurfaceCreationFailed(String),
    
    #[error("Failed to configure surface: {0}")]
    SurfaceConfigurationFailed(String),
    
    #[error("Shader compilation failed: {0}")]
    ShaderCompilationFailed(String),
    
    #[error("Buffer creation failed: {0}")]
    BufferCreationFailed(String),
    
    #[error("Render pipeline creation failed: {0}")]
    PipelineCreationFailed(String),
    
    #[error("Memory allocation failed: {0}")]
    MemoryAllocationFailed(String),
    
    #[error("Rendering failed: {0}")]
    RenderingFailed(String),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
    
    #[error("Buffer allocation failed: {0}")]
    BufferAllocation(String),
}

impl From<wgpu::RequestAdapterError> for WebGpuError {
    fn from(err: wgpu::RequestAdapterError) -> Self {
        WebGpuError::AdapterRequestFailed(err.to_string())
    }
}

/// Chart uniforms for WebGPU shaders
#[derive(Debug, Clone, Copy)]
pub struct ChartUniforms {
    pub view_matrix: [[f32; 4]; 4],
    pub projection_matrix: [[f32; 4]; 4],
    pub model_matrix: [[f32; 4]; 4],
    pub color: [f32; 4],
    pub opacity: f32,
    pub _padding: [f32; 3], // Ensure 16-byte alignment
}

/// Memory usage tracking for WebGPU resources
#[derive(Debug, Clone)]
pub struct MemoryUsage {
    /// Total bytes used by WebGPU resources
    pub used_bytes: usize,
    /// Number of allocated buffers
    pub allocated_buffers: usize,
    /// Number of shader modules
    pub shader_modules: usize,
    /// Number of render pipelines
    pub render_pipelines: usize,
}

impl Default for MemoryUsage {
    fn default() -> Self {
        Self {
            used_bytes: 0,
            allocated_buffers: 0,
            shader_modules: 0,
            render_pipelines: 0,
        }
    }
}

/// Buffer pool statistics
#[derive(Debug, Clone)]
pub struct BufferPoolStats {
    pub total_allocations: usize,
    pub total_deallocations: usize,
    pub allocated_bytes: u64,
    pub active_buffers: usize,
    pub vertex_buffers: usize,
    pub index_buffers: usize,
    pub uniform_buffers: usize,
}

impl Default for BufferPoolStats {
    fn default() -> Self {
        Self {
            total_allocations: 0,
            total_deallocations: 0,
            allocated_bytes: 0,
            active_buffers: 0,
            vertex_buffers: 0,
            index_buffers: 0,
            uniform_buffers: 0,
        }
    }
}

/// WebGPU shader definition
#[derive(Debug, Clone)]
pub struct WebGpuShader {
    pub name: String,
    pub source: String,
    pub entry_point: String,
}

/// Buffer wrapper for WebGPU buffers
pub struct Buffer {
    pub buffer: wgpu::Buffer,
    pub size: u64,
    pub usage: wgpu::BufferUsages,
}
