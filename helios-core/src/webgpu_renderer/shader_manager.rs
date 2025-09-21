//! Shader management for WebGPU rendering

use super::types::{WebGpuError, WebGpuShader};
use std::collections::HashMap;
use std::sync::Arc;

/// Shader manager for WebGPU shaders
pub struct ShaderManager {
    device: Arc<wgpu::Device>,
    shader_cache: HashMap<String, wgpu::ShaderModule>,
    render_pipelines: HashMap<String, wgpu::RenderPipeline>,
}

impl ShaderManager {
    /// Create a new shader manager
    pub fn new(device: Arc<wgpu::Device>) -> Self {
        Self {
            device,
            shader_cache: HashMap::new(),
            render_pipelines: HashMap::new(),
        }
    }

    /// Compile a shader from source
    pub fn compile_shader(&mut self, shader: &WebGpuShader) -> Result<wgpu::ShaderModule, WebGpuError> {
        // Check cache first
        if let Some(cached_shader) = self.shader_cache.get(&shader.name) {
            return Ok(cached_shader.clone());
        }

        // Compile new shader
        let shader_module = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&shader.name),
            source: wgpu::ShaderSource::Wgsl(shader.source.clone().into()),
        });

        // Cache the compiled shader
        self.shader_cache.insert(shader.name.clone(), shader_module.clone());

        Ok(shader_module)
    }

    /// Get a cached shader module
    pub fn get_shader(&self, name: &str) -> Option<&wgpu::ShaderModule> {
        self.shader_cache.get(name)
    }

    /// Create a render pipeline
    pub fn create_render_pipeline(
        &mut self,
        name: &str,
        shader_module: &wgpu::ShaderModule,
        vertex_buffer_layouts: &[wgpu::VertexBufferLayout],
        color_targets: &[Option<wgpu::ColorTargetState>],
    ) -> Result<wgpu::RenderPipeline, WebGpuError> {
        let pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(name),
            layout: None, // Use automatic layout
            vertex: wgpu::VertexState {
                module: shader_module,
                entry_point: Some("vs_main"),
                buffers: vertex_buffer_layouts,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: shader_module,
                entry_point: Some("fs_main"),
                targets: color_targets,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            cache: None,
            multiview: None,
        });

        // Cache the pipeline
        self.render_pipelines.insert(name.to_string(), pipeline.clone());

        Ok(pipeline)
    }

    /// Get a cached render pipeline
    pub fn get_render_pipeline(&self, name: &str) -> Option<&wgpu::RenderPipeline> {
        self.render_pipelines.get(name)
    }

    /// Clear shader cache
    pub fn clear_cache(&mut self) {
        self.shader_cache.clear();
        self.render_pipelines.clear();
    }

    /// Get shader cache statistics
    pub fn get_cache_stats(&self) -> (usize, usize) {
        (self.shader_cache.len(), self.render_pipelines.len())
    }
}

impl WebGpuShader {
    /// Create a new shader definition
    pub fn new(name: String, source: String, entry_point: String) -> Self {
        Self {
            name,
            source,
            entry_point,
        }
    }

    /// Create a basic vertex shader
    pub fn basic_vertex() -> Self {
        Self::new(
            "basic_vertex".to_string(),
            include_str!("shaders/basic_vertex.wgsl").to_string(),
            "vs_main".to_string(),
        )
    }

    /// Create a basic fragment shader
    pub fn basic_fragment() -> Self {
        Self::new(
            "basic_fragment".to_string(),
            include_str!("shaders/basic_fragment.wgsl").to_string(),
            "fs_main".to_string(),
        )
    }
}
