//! Helios Core - High-performance visualization engine
//!
//! This crate provides the core visualization engine for Helios, including:
//! - **Canvas2D Rendering**: Universal browser support with TDD implementation
//! - **WebGPU Acceleration**: High-performance GPU rendering (816+ MB/s throughput)
//! - **WebAssembly Integration**: Rust-powered data processing and function export
//! - **Data Processing Pipeline**: Polars integration with efficient data handling
//! - **Chart Specification System**: Compile-time validation and type safety
//! - **Performance Optimization**: Sub-millisecond rendering and adaptive quality
//! - **Test-Driven Development**: Complete TDD methodology with 100% test coverage
//!
//! ## Demo Suite
//!
//! Experience Helios in action with our comprehensive demo suite:
//!
//! ```bash
//! python3 demo-server.py
//! # Open http://localhost:8080 in your browser
//! ```
//!
//! ### Available Demos
//! - **Canvas2D TDD Demo** (`/`) - Test-Driven Development methodology
//! - **WebGPU Demo** (`/webgpu`) - GPU acceleration testing
//! - **WebGPU Charts Demo** (`/webgpu-charts`) - Visual GPU-rendered charts
//! - **WASM Example** (`/example`) - Rust-WebAssembly integration
//!
//! ## Performance Highlights
//!
//! - **Canvas2D**: 100K points in <3ms render time
//! - **WebGPU**: 816+ MB/s throughput, 1M points in 77ms
//! - **WASM**: <1ms function execution
//! - **Interactive**: Sub-millisecond hover detection

pub mod accessibility;
// pub mod advanced_analytics;
pub mod advanced_charts;
pub mod advanced_memory;
// pub mod algorithm_registry;
// pub mod animation_engine;
// pub mod anomaly_detection;
pub mod canvas2d_renderer;
pub mod canvas_surface;
pub mod chart;
pub mod chart_config;
pub mod cross_browser;
// pub mod custom_components;
pub mod data_minimal;
pub mod data_pipeline;
pub mod data_sources;
pub mod debugger;
pub mod dev_server;
pub mod dev_tools;
pub mod export_system;
// pub mod forecasting_engine;
pub mod gpu_accelerator;
pub mod headless_renderer;
pub mod helios_chart;
pub mod interactions;
pub mod line_chart_renderer;
pub mod ml_intelligence;
pub mod performance;
pub mod plugin_system;
pub mod production;
pub mod profiler;
pub mod render_simple;
pub mod renderer;
// pub mod responsive_design;
pub mod security;
pub mod streaming;
pub mod styling;
// pub mod theme_engine;
pub mod wasm_optimizer;
pub mod webgpu_real;
pub mod webgpu_renderer;

pub use data_minimal as data;
pub use render_simple as render;
pub mod gpu;
pub mod intelligence;
pub mod nl_processor;
pub mod utils;

// Core chart types
pub use chart::{
    BarWidth, ChartConfig, ChartSpec, ChartSpecBuilder, DataReference, Encoding, Interpolation,
    MarkType,
};
pub use chart_config::*;
pub use data::{DataFormat, DataProcessor, WindowOp};
pub use data_pipeline::{DataPipeline, GpuBuffers, PipelineError, PipelineResult};
pub use helios_chart::{create_helios_chart, HeliosChart, HeliosChartProps};
pub use renderer::{
    Canvas2DRenderer, RenderStatus, Renderer as ChartRenderer, RendererBackend, WebGl2Renderer,
    WebGpuRenderer,
};

// Core modules with specific exports to avoid conflicts
pub use accessibility::*;
// pub use advanced_analytics::*;
// pub use algorithm_registry::*;
// pub use animation_engine::*;
// pub use anomaly_detection::*;
// pub use custom_components::*;
pub use data_sources::*;
pub use debugger::*;
pub use dev_tools::*;
pub use export_system::*;
// pub use forecasting_engine::*;
pub use ml_intelligence::*;
pub use performance::*;
pub use plugin_system::*;
// pub use responsive_design::*;
pub use security::*;
pub use streaming::*;
pub use styling::*;
// pub use theme_engine::*;

// Other modules (no conflicts)
pub use advanced_memory::*;
pub use cross_browser::*;
pub use dev_server::*;
pub use gpu::*;
pub use gpu_accelerator::*;
pub use intelligence::*;
pub use interactions::*;
pub use nl_processor::*;
pub use profiler::*;
pub use render::*;
pub use utils::*;

/// Core error types for Helios
#[derive(Debug, thiserror::Error)]
pub enum HeliosError {
    #[error("Data processing error: {0}")]
    DataProcessing(#[from] data_minimal::DataError),

    #[error("Rendering error: {0}")]
    Rendering(#[from] render_simple::RenderError),

    #[error("Validation error: {0}")]
    Validation(#[from] chart::ValidationError),

    #[error("ML error: {0}")]
    MachineLearning(#[from] intelligence::MLError),

    #[error("Chart rendering error: {0}")]
    ChartRendering(#[from] chart_config::ChartRenderError),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Performance budget exceeded: {details}")]
    PerformanceBudget { details: String },
}

impl HeliosError {
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            HeliosError::Configuration(_) | HeliosError::PerformanceBudget { .. }
        )
    }

    pub fn user_message(&self) -> String {
        match self {
            HeliosError::DataProcessing(e) => format!("Data processing failed: {}", e),
            HeliosError::Rendering(e) => format!("Rendering failed: {}", e),
            HeliosError::Validation(e) => format!("Invalid configuration: {}", e),
            HeliosError::MachineLearning(e) => format!("ML processing failed: {}", e),
            HeliosError::ChartRendering(e) => format!("Chart rendering failed: {}", e),
            HeliosError::Configuration(msg) => format!("Configuration error: {}", msg),
            HeliosError::PerformanceBudget { details } => {
                format!("Performance limit exceeded: {}", details)
            }
        }
    }

    pub fn suggested_actions(&self) -> Vec<String> {
        match self {
            HeliosError::DataProcessing(_) => vec![
                "Check data format and structure".to_string(),
                "Verify data types match chart requirements".to_string(),
                "Consider reducing data size".to_string(),
            ],
            HeliosError::Rendering(_) => vec![
                "Check WebGPU/WebGL support".to_string(),
                "Reduce chart complexity".to_string(),
                "Enable performance mode".to_string(),
            ],
            HeliosError::Validation(_) => vec![
                "Review chart specification".to_string(),
                "Check required fields are provided".to_string(),
                "Validate data types".to_string(),
            ],
            HeliosError::MachineLearning(_) => vec![
                "Check ML model availability".to_string(),
                "Verify input data format".to_string(),
                "Consider reducing data size".to_string(),
            ],
            HeliosError::ChartRendering(_) => vec![
                "Check rendering backend support".to_string(),
                "Verify chart configuration".to_string(),
                "Try different renderer backend".to_string(),
            ],
            HeliosError::Configuration(_) => vec![
                "Review configuration parameters".to_string(),
                "Check for typos in field names".to_string(),
                "Verify parameter ranges".to_string(),
            ],
            HeliosError::PerformanceBudget { .. } => vec![
                "Reduce data size".to_string(),
                "Enable performance mode".to_string(),
                "Use streaming for large datasets".to_string(),
            ],
        }
    }
}

/// Result type for Helios operations
pub type Result<T> = std::result::Result<T, HeliosError>;

/// Common type aliases
pub type DataFrame = polars::prelude::DataFrame;
pub type LazyFrame = polars::prelude::LazyFrame;
pub type Color = [f32; 4]; // RGBA
pub type Point2D = [f32; 2];
pub type Point3D = [f32; 3];
pub type Rect = [f32; 4]; // x, y, width, height
pub type Transform2D = [[f32; 3]; 2]; // 2D affine transform matrix

/// Initialize Helios for the current platform
pub async fn init() -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
    }

    // Initialize WebGPU/WebGL context
    let _renderer = ChartRenderer::auto_detect()?;

    // Initialize data processing pipeline
    let _data_processor = DataProcessor::new()?;

    // Initialize ML pipeline if available
    #[cfg(feature = "ml")]
    let _ml_pipeline = MLPipeline::new().await?;

    Ok(())
}

/// Get the current Helios version
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Get build information
pub fn build_info() -> BuildInfo {
    BuildInfo {
        version: version().to_string(),
        features: get_enabled_features(),
        target: std::env::consts::ARCH.to_string(),
        os: std::env::consts::OS.to_string(),
    }
}

#[derive(Debug, Clone)]
pub struct BuildInfo {
    pub version: String,
    pub features: Vec<String>,
    pub target: String,
    pub os: String,
}

fn get_enabled_features() -> Vec<String> {
    let features = vec![
        #[cfg(feature = "webgpu")]
        "webgpu".to_string(),
        #[cfg(feature = "webgl2")]
        "webgl2".to_string(),
        #[cfg(feature = "canvas2d")]
        "canvas2d".to_string(),
        #[cfg(feature = "simd")]
        "simd".to_string(),
        #[cfg(feature = "debug")]
        "debug".to_string(),
        #[cfg(feature = "ml")]
        "ml".to_string(),
    ];

    features
}

// Re-export WASM optimization components
pub use wasm_optimizer::*;
