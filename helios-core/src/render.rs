//! Rendering engine with WebGPU support and fallbacks

use std::collections::HashMap;
use std::time::{Duration, Instant};
use wgpu::*;

/// Rendering error types
#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("WebGPU error: {0}")]
    WebGPU(#[from] wgpu::Error),

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
        device: Device,
        queue: Queue,
        surface: Surface,
        compute_capability: bool,
        memory_budget: usize,
    },

    /// Fallback: WebGL2 for broad compatibility
    WebGL2 {
        context: WebGl2RenderingContext,
        extensions: Vec<String>,
    },

    /// Last resort: Canvas 2D for universal support
    Canvas2D { context: CanvasRenderingContext2d },
}

impl RenderBackend {
    pub async fn create_optimal() -> Result<Self, RenderError> {
        if Self::webgpu_available().await {
            Self::webgpu_backend().await
        } else if Self::webgl2_available() {
            Self::webgl2_backend()
        } else {
            Self::canvas2d_backend()
        }
    }

    async fn webgpu_available() -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            // Check WebGPU availability in browser
            false // Placeholder - would use web-sys
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Check native WebGPU support
            false // Placeholder
        }
    }

    fn webgl2_available() -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            // Check WebGL2 availability
            false // Placeholder - would use web-sys
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Check native OpenGL support
            false // Placeholder
        }
    }

    async fn webgpu_backend() -> Result<Self, RenderError> {
        // WebGPU backend implementation
        Err(RenderError::WebGPU(wgpu::Error::DeviceLost))
    }

    fn webgl2_backend() -> Result<Self, RenderError> {
        // WebGL2 backend implementation
        Err(RenderError::WebGL("WebGL2 not implemented".to_string()))
    }

    fn canvas2d_backend() -> Result<Self, RenderError> {
        // Canvas 2D backend implementation
        Err(RenderError::Canvas("Canvas 2D not implemented".to_string()))
    }

    pub fn performance_characteristics(&self) -> PerformanceProfile {
        match self {
            RenderBackend::WebGPU { .. } => PerformanceProfile {
                max_points: 10_000_000,
                target_fps: 60,
                memory_efficiency: 0.95,
                compute_shaders: true,
            },
            RenderBackend::WebGL2 { .. } => PerformanceProfile {
                max_points: 1_000_000,
                target_fps: 60,
                memory_efficiency: 0.80,
                compute_shaders: false,
            },
            RenderBackend::Canvas2D { .. } => PerformanceProfile {
                max_points: 10_000,
                target_fps: 30,
                memory_efficiency: 0.60,
                compute_shaders: false,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct PerformanceProfile {
    pub max_points: u32,
    pub target_fps: u32,
    pub memory_efficiency: f32,
    pub compute_shaders: bool,
}

/// Main renderer
pub struct Renderer {
    backend: RenderBackend,
    pipelines: HashMap<ChartType, RenderPipeline>,
    buffer_pool: BufferPool,
    frame_timer: FrameTimer,
    quality_manager: AdaptiveQualityManager,
}

impl Renderer {
    pub async fn new() -> Result<Self, RenderError> {
        let backend = RenderBackend::create_optimal().await?;
        let buffer_pool = BufferPool::new(&backend)?;
        let frame_timer = FrameTimer::new();
        let quality_manager = AdaptiveQualityManager::new();

        Ok(Self {
            backend,
            pipelines: HashMap::new(),
            buffer_pool,
            frame_timer,
            quality_manager,
        })
    }

    pub fn render(&mut self, spec: &ChartSpec) -> RenderStats {
        let start_time = Instant::now();

        // Adaptive quality based on frame timing
        let quality_level = self.frame_timer.suggest_quality();
        let render_config = self.quality_manager.get_render_config(quality_level);

        // Get or create render pipeline for chart type
        let chart_type = ChartType::from_spec(spec);
        let pipeline = self.get_or_create_pipeline(chart_type);

        // Efficient GPU buffer management
        let buffers = self.buffer_pool.get_buffers_for_spec(spec);

        // Execute render pass
        let stats = self.execute_render_pass(&pipeline, &buffers, &render_config);

        // Update frame timing for adaptation
        let frame_time = start_time.elapsed();
        self.frame_timer.record_frame(frame_time);
        self.quality_manager.update_frame_stats(frame_time);

        stats
    }

    fn get_or_create_pipeline(&mut self, chart_type: ChartType) -> &RenderPipeline {
        self.pipelines
            .entry(chart_type)
            .or_insert_with(|| RenderPipeline::new(&self.backend, chart_type))
    }

    fn execute_render_pass(
        &self,
        pipeline: &RenderPipeline,
        buffers: &RenderBuffers,
        config: &RenderConfig,
    ) -> RenderStats {
        // Execute the actual rendering
        RenderStats {
            frame_time: Duration::from_millis(3), // Placeholder
            triangles_rendered: 1000,             // Placeholder
            draw_calls: 1,                        // Placeholder
            memory_used: 1024 * 1024,             // 1MB placeholder
            gpu_utilization: 0.5,                 // Placeholder
            cache_hit_rate: 0.95,                 // Placeholder
        }
    }
}

/// Chart type classification
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChartType {
    Point,
    Line,
    Bar,
    Area,
    Text,
    Rect,
    Composite,
}

impl ChartType {
    pub fn from_spec(spec: &ChartSpec) -> Self {
        match spec.mark {
            MarkType::Point { .. } => ChartType::Point,
            MarkType::Line { .. } => ChartType::Line,
            MarkType::Bar { .. } => ChartType::Bar,
            MarkType::Area { .. } => ChartType::Area,
            MarkType::Text { .. } => ChartType::Text,
            MarkType::Rect { .. } => ChartType::Rect,
            MarkType::Composite(_) => ChartType::Composite,
        }
    }
}

/// Render pipeline for specific chart types
pub struct RenderPipeline {
    chart_type: ChartType,
    vertex_shader: ShaderModule,
    fragment_shader: ShaderModule,
    compute_shader: Option<ShaderModule>,
    render_pipeline: wgpu::RenderPipeline,
    compute_pipeline: Option<wgpu::ComputePipeline>,
}

impl RenderPipeline {
    pub fn new(backend: &RenderBackend, chart_type: ChartType) -> Self {
        match backend {
            RenderBackend::WebGPU { device, .. } => {
                Self::create_webgpu_pipeline(device, chart_type)
            }
            _ => {
                // Fallback pipeline creation
                Self {
                    chart_type,
                    vertex_shader: ShaderModule::default(),
                    fragment_shader: ShaderModule::default(),
                    compute_shader: None,
                    render_pipeline: wgpu::RenderPipeline::default(),
                    compute_pipeline: None,
                }
            }
        }
    }

    fn create_webgpu_pipeline(device: &Device, chart_type: ChartType) -> Self {
        // Create WebGPU pipeline
        let vertex_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Vertex Shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/vertex.wgsl").into()),
        });

        let fragment_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Fragment Shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/fragment.wgsl").into()),
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: None,
            vertex: VertexState {
                module: &vertex_shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &fragment_shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: TextureFormat::Bgra8UnormSrgb,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            chart_type,
            vertex_shader,
            fragment_shader,
            compute_shader: None,
            render_pipeline,
            compute_pipeline: None,
        }
    }
}

/// GPU buffer management
pub struct BufferPool {
    device: Arc<Device>,
    available_buffers: HashMap<BufferSpec, VecDeque<Buffer>>,
    allocated_size: AtomicUsize,
    max_pool_size: usize,
}

impl BufferPool {
    pub fn new(backend: &RenderBackend) -> Result<Self, RenderError> {
        match backend {
            RenderBackend::WebGPU { device, .. } => {
                Ok(Self {
                    device: Arc::new(device.clone()),
                    available_buffers: HashMap::new(),
                    allocated_size: AtomicUsize::new(0),
                    max_pool_size: 100 * 1024 * 1024, // 100MB
                })
            }
            _ => Err(RenderError::Buffer(
                "Buffer pool requires WebGPU backend".to_string(),
            )),
        }
    }

    pub fn get_buffer(&mut self, spec: BufferSpec) -> Buffer {
        // Try to reuse existing buffer
        if let Some(buffer) = self
            .available_buffers
            .get_mut(&spec)
            .and_then(|queue| queue.pop_front())
        {
            return buffer;
        }

        // Create new buffer if needed
        self.create_buffer(spec)
    }

    pub fn return_buffer(&mut self, buffer: Buffer, spec: BufferSpec) {
        // Return to pool if we have space
        if self.allocated_size.load(Ordering::Relaxed) < self.max_pool_size {
            self.available_buffers
                .entry(spec)
                .or_default()
                .push_back(buffer);
        }
        // Otherwise let it drop and be deallocated
    }

    fn create_buffer(&self, spec: BufferSpec) -> Buffer {
        self.allocated_size.fetch_add(spec.size, Ordering::Relaxed);

        self.device.create_buffer(&BufferDescriptor {
            label: Some(&spec.label),
            size: spec.size as u64,
            usage: spec.usage,
            mapped_at_creation: false,
        })
    }

    pub fn get_buffers_for_spec(&mut self, spec: &ChartSpec) -> RenderBuffers {
        // Create buffers for chart specification
        RenderBuffers {
            vertex_buffer: self.get_buffer(BufferSpec {
                label: "vertex".to_string(),
                size: 1024 * 1024, // 1MB placeholder
                usage: BufferUsages::VERTEX,
            }),
            index_buffer: self.get_buffer(BufferSpec {
                label: "index".to_string(),
                size: 256 * 1024, // 256KB placeholder
                usage: BufferUsages::INDEX,
            }),
            uniform_buffer: self.get_buffer(BufferSpec {
                label: "uniform".to_string(),
                size: 64 * 1024, // 64KB placeholder
                usage: BufferUsages::UNIFORM,
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BufferSpec {
    pub label: String,
    pub size: usize,
    pub usage: BufferUsages,
}

/// Render buffers for a chart
pub struct RenderBuffers {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub uniform_buffer: Buffer,
}

/// Frame timing for performance optimization
pub struct FrameTimer {
    frame_times: VecDeque<Duration>,
    max_samples: usize,
}

impl FrameTimer {
    pub fn new() -> Self {
        Self {
            frame_times: VecDeque::new(),
            max_samples: 60, // Keep 1 second of samples at 60fps
        }
    }

    pub fn record_frame(&mut self, frame_time: Duration) {
        self.frame_times.push_back(frame_time);
        if self.frame_times.len() > self.max_samples {
            self.frame_times.pop_front();
        }
    }

    pub fn average_frame_time(&self) -> Duration {
        if self.frame_times.is_empty() {
            return Duration::from_millis(16); // 60fps default
        }

        let total: Duration = self.frame_times.iter().sum();
        total / self.frame_times.len() as u32
    }

    pub fn fps(&self) -> f64 {
        let avg_frame_time = self.average_frame_time();
        if avg_frame_time.as_secs_f64() > 0.0 {
            1.0 / avg_frame_time.as_secs_f64()
        } else {
            0.0
        }
    }

    pub fn suggest_quality(&self) -> f32 {
        let avg_frame_time = self.average_frame_time();
        let target_frame_time = Duration::from_millis(16); // 60fps

        if avg_frame_time > target_frame_time * 2 {
            0.3 // Low quality
        } else if avg_frame_time > target_frame_time * 1.5 {
            0.5 // Medium quality
        } else if avg_frame_time < target_frame_time * 0.5 {
            1.0 // High quality
        } else {
            0.8 // Balanced quality
        }
    }
}

/// Adaptive quality management
pub struct AdaptiveQualityManager {
    frame_timer: FrameTimer,
    quality_level: f32,
    target_frame_time: Duration,
    quality_config: QualityConfig,
}

impl AdaptiveQualityManager {
    pub fn new() -> Self {
        Self {
            frame_timer: FrameTimer::new(),
            quality_level: 0.8,
            target_frame_time: Duration::from_millis(16), // 60fps
            quality_config: QualityConfig::default(),
        }
    }

    pub fn update_frame_stats(&mut self, frame_time: Duration) {
        self.frame_timer.record_frame(frame_time);

        let avg_frame_time = self.frame_timer.average_frame_time();

        // Adjust quality based on performance
        if avg_frame_time > self.target_frame_time * 1.2 {
            // Too slow - reduce quality
            self.quality_level = (self.quality_level - 0.1).max(0.3);
        } else if avg_frame_time < self.target_frame_time * 0.8 {
            // Fast enough - increase quality
            self.quality_level = (self.quality_level + 0.05).min(1.0);
        }
    }

    pub fn get_render_config(&self, quality_level: f32) -> RenderConfig {
        RenderConfig {
            point_size: self.quality_config.base_point_size * quality_level,
            anti_aliasing: quality_level > 0.7,
            msaa_samples: if quality_level > 0.8 { 4 } else { 1 },
            lod_bias: (1.0 - quality_level) * 2.0,
            texture_filtering: if quality_level > 0.6 {
                FilterMode::Linear
            } else {
                FilterMode::Nearest
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct QualityConfig {
    pub base_point_size: f32,
}

impl Default for QualityConfig {
    fn default() -> Self {
        Self {
            base_point_size: 4.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub point_size: f32,
    pub anti_aliasing: bool,
    pub msaa_samples: u32,
    pub lod_bias: f32,
    pub texture_filtering: FilterMode,
}

#[derive(Debug, Clone)]
pub enum FilterMode {
    Nearest,
    Linear,
}

/// Rendering statistics
#[derive(Debug, Clone)]
pub struct RenderStats {
    pub frame_time: Duration,
    pub triangles_rendered: u32,
    pub draw_calls: u32,
    pub memory_used: usize,
    pub gpu_utilization: f32,
    pub cache_hit_rate: f32,
}

impl RenderStats {
    pub fn fps(&self) -> f64 {
        if self.frame_time.as_secs_f64() > 0.0 {
            1.0 / self.frame_time.as_secs_f64()
        } else {
            0.0
        }
    }

    pub fn is_within_budget(&self, budget: &PerformanceBudget) -> bool {
        self.frame_time <= budget.max_frame_time
            && self.memory_used <= budget.max_memory
            && self.gpu_utilization <= budget.max_gpu_utilization
    }

    pub fn suggest_optimizations(&self) -> Vec<OptimizationSuggestion> {
        let mut suggestions = Vec::new();

        if self.frame_time > Duration::from_millis(16) {
            suggestions.push(OptimizationSuggestion::ReduceQuality);
        }

        if self.memory_used > 100 * 1024 * 1024 {
            suggestions.push(OptimizationSuggestion::ReduceDataSize);
        }

        if self.gpu_utilization > 0.9 {
            suggestions.push(OptimizationSuggestion::EnableLOD);
        }

        if self.cache_hit_rate < 0.8 {
            suggestions.push(OptimizationSuggestion::ImproveCaching);
        }

        suggestions
    }
}

#[derive(Debug, Clone)]
pub struct PerformanceBudget {
    pub max_frame_time: Duration,
    pub max_memory: usize,
    pub max_gpu_utilization: f32,
}

#[derive(Debug, Clone)]
pub enum OptimizationSuggestion {
    ReduceQuality,
    ReduceDataSize,
    EnableLOD,
    ImproveCaching,
    UseStreaming,
    EnableGPUProcessing,
}

// Re-export types from other modules
use crate::chart::{ChartSpec, MarkType};
use std::collections::VecDeque;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
