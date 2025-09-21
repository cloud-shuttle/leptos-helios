//! Buffer pool management for WebGPU rendering

use super::types::{BufferPoolStats, WebGpuError};
use std::sync::Arc;
use wgpu::util::DeviceExt;

/// Buffer pool for efficient GPU memory management
pub struct BufferPool {
    device: Arc<wgpu::Device>,
    vertex_buffers: Vec<wgpu::Buffer>,
    index_buffers: Vec<wgpu::Buffer>,
    uniform_buffers: Vec<wgpu::Buffer>,
    total_allocations: usize,
    total_deallocations: usize,
    allocated_bytes: u64,
}

impl BufferPool {
    /// Create a new buffer pool
    pub fn new(device: Arc<wgpu::Device>) -> Self {
        Self {
            device,
            vertex_buffers: Vec::new(),
            index_buffers: Vec::new(),
            uniform_buffers: Vec::new(),
            total_allocations: 0,
            total_deallocations: 0,
            allocated_bytes: 0,
        }
    }

    /// Allocate a vertex buffer
    pub fn allocate_vertex_buffer(&mut self, size: u64, data: Option<&[u8]>) -> Result<wgpu::Buffer, WebGpuError> {
        let buffer = if let Some(data) = data {
            self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: data,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            })
        } else {
            self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Vertex Buffer"),
                size,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        };

        self.vertex_buffers.push(buffer.clone());
        self.total_allocations += 1;
        self.allocated_bytes += size;
        
        Ok(buffer)
    }

    /// Allocate an index buffer
    pub fn allocate_index_buffer(&mut self, size: u64, data: Option<&[u8]>) -> Result<wgpu::Buffer, WebGpuError> {
        let buffer = if let Some(data) = data {
            self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: data,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            })
        } else {
            self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Index Buffer"),
                size,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        };

        self.index_buffers.push(buffer.clone());
        self.total_allocations += 1;
        self.allocated_bytes += size;
        
        Ok(buffer)
    }

    /// Allocate a uniform buffer
    pub fn allocate_uniform_buffer(&mut self, size: u64, data: Option<&[u8]>) -> Result<wgpu::Buffer, WebGpuError> {
        let buffer = if let Some(data) = data {
            self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: data,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            })
        } else {
            self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Uniform Buffer"),
                size,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        };

        self.uniform_buffers.push(buffer.clone());
        self.total_allocations += 1;
        self.allocated_bytes += size;
        
        Ok(buffer)
    }

    /// Deallocate a buffer (mark for cleanup)
    pub fn deallocate_buffer(&mut self, buffer: &wgpu::Buffer) {
        // In a real implementation, we would track buffer IDs and remove them
        // For now, we just update statistics
        self.total_deallocations += 1;
    }

    /// Get buffer pool statistics
    pub fn get_stats(&self) -> BufferPoolStats {
        BufferPoolStats {
            total_allocations: self.total_allocations,
            total_deallocations: self.total_deallocations,
            allocated_bytes: self.allocated_bytes,
            active_buffers: self.vertex_buffers.len() + self.index_buffers.len() + self.uniform_buffers.len(),
            vertex_buffers: self.vertex_buffers.len(),
            index_buffers: self.index_buffers.len(),
            uniform_buffers: self.uniform_buffers.len(),
        }
    }

    /// Clear all buffers (for cleanup)
    pub fn clear(&mut self) {
        self.vertex_buffers.clear();
        self.index_buffers.clear();
        self.uniform_buffers.clear();
        self.total_allocations = 0;
        self.total_deallocations = 0;
        self.allocated_bytes = 0;
    }
}
