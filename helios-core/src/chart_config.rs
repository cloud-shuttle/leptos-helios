//! Chart Configuration Types
//! Configuration structures for different chart types

use serde::{Deserialize, Serialize};

/// Color schemes for visualizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColorScheme {
    Viridis,
    Plasma,
    Inferno,
    Magma,
    Cividis,
    Turbo,
    Rainbow,
    Spectral,
    RdYlBu,
    RdBu,
    PiYG,
    PRGn,
    BrBG,
    RdGy,
    RdYlGn,
    Set1,
    Set2,
    Set3,
    Pastel1,
    Pastel2,
    Dark2,
    Paired,
    Accent,
}

impl Default for ColorScheme {
    fn default() -> Self {
        ColorScheme::Viridis
    }
}

/// Base chart configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseChartConfig {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub x_label: String,
    pub y_label: String,
    pub show_grid: bool,
    pub background_color: String,
    pub text_color: String,
}

impl Default for BaseChartConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            title: "Chart".to_string(),
            x_label: "X Axis".to_string(),
            y_label: "Y Axis".to_string(),
            show_grid: true,
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        }
    }
}

/// Line chart configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineChartConfig {
    #[serde(flatten)]
    pub base: BaseChartConfig,
    pub color: String,
    pub line_width: f32,
    pub show_points: bool,
    pub point_size: f32,
    pub interpolation: InterpolationType,
    pub show_legend: bool,
}

impl Default for LineChartConfig {
    fn default() -> Self {
        Self {
            base: BaseChartConfig::default(),
            color: "#00d4ff".to_string(),
            line_width: 2.0,
            show_points: true,
            point_size: 4.0,
            interpolation: InterpolationType::Linear,
            show_legend: true,
        }
    }
}

/// Bar chart configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarChartConfig {
    #[serde(flatten)]
    pub base: BaseChartConfig,
    pub colors: Vec<String>,
    pub bar_width: f32,
    pub show_values: bool,
    pub horizontal: bool,
    pub show_legend: bool,
}

impl Default for BarChartConfig {
    fn default() -> Self {
        Self {
            base: BaseChartConfig::default(),
            colors: vec![
                "#00d4ff".to_string(),
                "#ff6b6b".to_string(),
                "#4ecdc4".to_string(),
                "#45b7d1".to_string(),
            ],
            bar_width: 0.8,
            show_values: false,
            horizontal: false,
            show_legend: true,
        }
    }
}

/// Scatter plot configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScatterPlotConfig {
    #[serde(flatten)]
    pub base: BaseChartConfig,
    pub point_color: String,
    pub point_size: f32,
    pub show_trend_line: bool,
    pub trend_line_color: String,
    pub trend_line_width: f32,
    pub show_legend: bool,
}

impl Default for ScatterPlotConfig {
    fn default() -> Self {
        Self {
            base: BaseChartConfig::default(),
            point_color: "#00d4ff".to_string(),
            point_size: 5.0,
            show_trend_line: false,
            trend_line_color: "#ff6b6b".to_string(),
            trend_line_width: 2.0,
            show_legend: true,
        }
    }
}

/// Heatmap configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeatmapConfig {
    #[serde(flatten)]
    pub base: BaseChartConfig,
    pub x_labels: Vec<String>,
    pub y_labels: Vec<String>,
    pub color_scheme: ColorScheme,
    pub show_values: bool,
    pub show_legend: bool,
}

impl Default for HeatmapConfig {
    fn default() -> Self {
        Self {
            base: BaseChartConfig::default(),
            x_labels: Vec::new(),
            y_labels: Vec::new(),
            color_scheme: ColorScheme::Viridis,
            show_values: false,
            show_legend: true,
        }
    }
}

/// Interpolation types for line charts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InterpolationType {
    Linear,
    Step,
    Smooth,
    Monotone,
}

impl Default for InterpolationType {
    fn default() -> Self {
        InterpolationType::Linear
    }
}

/// Render result containing pixel data
#[derive(Debug, Clone)]
pub struct RenderResult {
    pub width: u32,
    pub height: u32,
    pub pixel_data: Vec<u8>, // RGBA format
    pub format: PixelFormat,
}

/// Pixel format for render results
#[derive(Debug, Clone)]
pub enum PixelFormat {
    RGBA8,
    RGB8,
    Grayscale8,
}

impl Default for PixelFormat {
    fn default() -> Self {
        PixelFormat::RGBA8
    }
}

/// Chart rendering error
#[derive(Debug, thiserror::Error)]
pub enum ChartRenderError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Rendering failed: {0}")]
    RenderFailed(String),

    #[error("Unsupported chart type: {0}")]
    UnsupportedChartType(String),

    #[error("Backend error: {0}")]
    BackendError(String),

    #[error("Memory allocation failed")]
    MemoryError,

    #[error("Data processing error: {0}")]
    DataError(String),
}

/// Result type for chart rendering operations
pub type ChartRenderResult<T> = Result<T, ChartRenderError>;

/// Point shape for scatter plots
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PointShape {
    Circle,
    Square,
    Triangle,
    Diamond,
    Cross,
    Plus,
}

impl Default for PointShape {
    fn default() -> Self {
        PointShape::Circle
    }
}

/// Render result with performance metrics for WebGPU
#[derive(Debug, Clone)]
pub struct WebGpuRenderResult {
    pub render_time_ms: f64,
    pub memory_used_bytes: usize,
    pub vertices_rendered: usize,
}
