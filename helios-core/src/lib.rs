//! Helios Core - High-performance visualization engine
//!
//! This crate provides the core visualization engine for Helios, including:
//! - WebGPU-based rendering with fallbacks
//! - Data processing pipeline with Polars integration
//! - Chart specification system with compile-time validation
//! - Performance optimization and adaptive quality systems

pub mod advanced_charts;
pub mod canvas_surface;
pub mod chart;
pub mod chart_config;
pub mod data_minimal;
pub mod helios_chart;
pub mod interactions;
pub mod line_chart_renderer;
pub mod performance;
pub mod production;
pub mod render_simple;
pub mod renderer;
pub mod streaming;
pub mod webgpu_real;
pub mod webgpu_renderer;

pub use data_minimal as data;
pub use render_simple as render;
pub mod gpu;
pub mod intelligence;
pub mod utils;

pub use chart::{ChartSpec, ChartSpecBuilder, DataReference, Encoding, MarkType};
pub use chart_config::*;
pub use data::{DataFormat, DataProcessor, WindowOp};
pub use gpu::*;
pub use helios_chart::*;
pub use intelligence::*;
pub use interactions::*;
pub use render::*;
pub use renderer::{
    Canvas2DRenderer, Renderer as ChartRenderer, RendererBackend, WebGl2Renderer, WebGpuRenderer,
};
pub use streaming::*;
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
    ];

    features
}
