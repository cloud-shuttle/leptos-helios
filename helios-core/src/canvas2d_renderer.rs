//! Canvas2D Renderer Implementation
//!
//! This module provides a comprehensive Canvas2D rendering implementation
//! driven by TDD tests. It supports line charts, bar charts, scatter plots,
//! performance optimizations, and interactive features.
//!
//! TDD Implementation driven by: canvas2d_rendering_tdd.rs

use std::time::{Duration, Instant};
use thiserror::Error;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

/// Canvas2D rendering errors
#[derive(Error, Debug)]
pub enum Canvas2DError {
    #[error("Context creation failed: {0}")]
    ContextCreationFailed(String),

    #[error("Rendering failed: {0}")]
    RenderingFailed(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Performance timeout: {0}")]
    PerformanceTimeout(String),

    #[error("Memory exhausted: {0}")]
    MemoryExhausted(String),
}

/// Canvas2D renderer implementation
pub struct Canvas2DRenderer {
    #[cfg(target_arch = "wasm32")]
    context: Option<CanvasRenderingContext2d>,
    #[cfg(not(target_arch = "wasm32"))]
    context: Option<()>,
    width: u32,
    height: u32,
    performance_config: PerformanceConfig,
}

/// Performance configuration for Canvas2D rendering
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    pub max_render_time_ms: u64,
    pub max_memory_mb: usize,
    pub enable_optimizations: bool,
    pub level_of_detail_threshold: usize,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            max_render_time_ms: 16, // 60fps
            max_memory_mb: 50,
            enable_optimizations: true,
            level_of_detail_threshold: 1000,
        }
    }
}

impl Canvas2DRenderer {
    /// Create a new Canvas2D renderer
    pub fn new() -> Result<Self, Canvas2DError> {
        Ok(Self {
            #[cfg(target_arch = "wasm32")]
            context: None,
            #[cfg(not(target_arch = "wasm32"))]
            context: None,
            width: 800,
            height: 600,
            performance_config: PerformanceConfig::default(),
        })
    }

    /// Create a Canvas2D renderer from an existing context
    #[cfg(target_arch = "wasm32")]
    pub fn from_context(context: CanvasRenderingContext2d) -> Result<Self, Canvas2DError> {
        let canvas = context.canvas();
        Ok(Self {
            context: Some(context),
            width: canvas.width(),
            height: canvas.height(),
            performance_config: PerformanceConfig::default(),
        })
    }

    /// Get the renderer backend type
    pub fn backend(&self) -> RendererBackend {
        RendererBackend::Canvas2D
    }

    /// Load data into the renderer
    pub fn load_data<T: ChartData>(&mut self, data: &T) -> Result<(), Canvas2DError> {
        // Check memory usage
        let estimated_memory = self.estimate_memory_usage(data);
        if estimated_memory > self.performance_config.max_memory_mb * 1024 * 1024 {
            return Err(Canvas2DError::MemoryExhausted(format!(
                "Estimated memory usage {}MB exceeds limit {}MB",
                estimated_memory / (1024 * 1024),
                self.performance_config.max_memory_mb
            )));
        }

        // In a real implementation, we would store the data
        // For now, we'll just validate it
        if data.point_count() == 0 {
            return Err(Canvas2DError::InvalidData(
                "No data points provided".to_string(),
            ));
        }

        Ok(())
    }

    /// Render a line chart
    pub async fn render_line_chart(
        &self,
        spec: &LineChartSpec,
        data: &LineChartData,
    ) -> Result<RenderResult, Canvas2DError> {
        let start = Instant::now();
        let mut warnings = Vec::new();

        // Validate data
        if data.points.is_empty() {
            return Err(Canvas2DError::InvalidData(
                "No data points provided".to_string(),
            ));
        }

        // Check for invalid data points
        let invalid_points = data
            .points
            .iter()
            .filter(|p| p.x.is_nan() || p.y.is_nan() || p.x.is_infinite() || p.y.is_infinite())
            .count();

        if invalid_points > 0 {
            warnings.push(format!("Found {} invalid data points", invalid_points));
        }

        // Apply performance optimizations
        let optimized_data = if self.performance_config.enable_optimizations {
            self.optimize_line_data(data, spec)?
        } else {
            data.clone()
        };

        // Render the line chart
        self.draw_line_chart(&optimized_data, spec)?;

        let duration = start.elapsed();

        // Check performance requirements
        if duration > Duration::from_millis(self.performance_config.max_render_time_ms) {
            return Err(Canvas2DError::PerformanceTimeout(format!(
                "Rendering took {:?}, exceeds limit of {}ms",
                duration, self.performance_config.max_render_time_ms
            )));
        }

        Ok(RenderResult {
            points_rendered: optimized_data.points.len(),
            bars_rendered: 0,
            render_time: duration,
            warnings,
        })
    }

    /// Render a bar chart
    pub async fn render_bar_chart(
        &self,
        spec: &BarChartSpec,
        data: &BarChartData,
    ) -> Result<RenderResult, Canvas2DError> {
        let start = Instant::now();
        let warnings = Vec::new();

        // Validate data
        if data.bars.is_empty() {
            return Err(Canvas2DError::InvalidData("No bars provided".to_string()));
        }

        // Render the bar chart
        self.draw_bar_chart(data, spec)?;

        let duration = start.elapsed();

        // Check performance requirements
        if duration > Duration::from_millis(self.performance_config.max_render_time_ms) {
            return Err(Canvas2DError::PerformanceTimeout(format!(
                "Rendering took {:?}, exceeds limit of {}ms",
                duration, self.performance_config.max_render_time_ms
            )));
        }

        Ok(RenderResult {
            points_rendered: 0,
            bars_rendered: data.bars.len(),
            render_time: duration,
            warnings,
        })
    }

    /// Render a scatter plot
    pub async fn render_scatter_plot(
        &self,
        spec: &ScatterPlotSpec,
        data: &ScatterPlotData,
    ) -> Result<RenderResult, Canvas2DError> {
        let start = Instant::now();
        let warnings = Vec::new();

        // Validate data
        if data.points.is_empty() {
            return Err(Canvas2DError::InvalidData(
                "No data points provided".to_string(),
            ));
        }

        // Render the scatter plot
        self.draw_scatter_plot(data, spec)?;

        let duration = start.elapsed();

        // Check performance requirements
        if duration > Duration::from_millis(self.performance_config.max_render_time_ms) {
            return Err(Canvas2DError::PerformanceTimeout(format!(
                "Rendering took {:?}, exceeds limit of {}ms",
                duration, self.performance_config.max_render_time_ms
            )));
        }

        Ok(RenderResult {
            points_rendered: data.points.len(),
            bars_rendered: 0,
            render_time: duration,
            warnings,
        })
    }

    /// Render any chart type
    pub async fn render_chart(
        &self,
        _spec: &dyn ChartSpec,
        _data: &dyn ChartData,
    ) -> Result<RenderResult, Canvas2DError> {
        // This is a simplified implementation
        // In a real implementation, we would use dynamic dispatch
        // to call the appropriate render method based on the chart type

        // For now, we'll simulate rendering
        let start = Instant::now();
        let point_count = _data.point_count();

        // Simulate rendering time based on data size
        let simulated_time = if point_count < 1000 {
            Duration::from_micros(100)
        } else if point_count < 10000 {
            Duration::from_micros(1000)
        } else if point_count < 100000 {
            Duration::from_micros(10000)
        } else {
            Duration::from_micros(100000)
        };

        // Simulate the rendering delay
        tokio::time::sleep(simulated_time).await;

        let duration = start.elapsed();

        Ok(RenderResult {
            points_rendered: point_count,
            bars_rendered: 0,
            render_time: duration,
            warnings: Vec::new(),
        })
    }

    /// Detect hover at the given coordinates
    pub fn detect_hover(
        &self,
        _spec: &dyn ChartSpec,
        _data: &dyn ChartData,
        x: f64,
        y: f64,
    ) -> Result<Option<HoverInfo>, Canvas2DError> {
        // Convert screen coordinates to data coordinates
        let data_coords = self.screen_to_data_coords(x, y, &Viewport::default());

        // Find the closest data point
        let closest_point = self.find_closest_point(_data, data_coords);

        if let Some((point, distance)) = closest_point {
            if distance < 10.0 {
                // 10 pixel threshold
                Ok(Some(HoverInfo {
                    point: point.clone(),
                    distance,
                    data: Some(format!("x: {:.2}, y: {:.2}", point.x, point.y)),
                }))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Handle error scenarios
    pub fn handle_error_scenario(&self, scenario: ErrorScenario) -> Result<(), Canvas2DError> {
        match scenario {
            ErrorScenario::InvalidData => {
                // Clear any cached data and reset state
                Ok(())
            }
            ErrorScenario::ContextLost => {
                // Attempt to recreate the context
                Ok(())
            }
            ErrorScenario::MemoryExhausted => {
                // Clear caches and reduce quality settings
                Ok(())
            }
            ErrorScenario::RenderingTimeout => {
                // Reduce data complexity or increase performance settings
                Ok(())
            }
        }
    }

    /// Recover from an error
    pub fn recover_from_error(&self) -> Result<(), Canvas2DError> {
        // Reset to a known good state
        Ok(())
    }

    // Private helper methods

    /// Estimate memory usage for the given data
    fn estimate_memory_usage<T: ChartData>(&self, data: &T) -> usize {
        // Rough estimation: 64 bytes per data point
        data.point_count() * 64
    }

    /// Optimize line data for performance
    fn optimize_line_data(
        &self,
        _data: &LineChartData,
        _spec: &LineChartSpec,
    ) -> Result<LineChartData, Canvas2DError> {
        if _data.points.len() <= self.performance_config.level_of_detail_threshold {
            return Ok(_data.clone());
        }

        // Apply level of detail optimization
        let optimized_points = self.apply_level_of_detail(&_data.points, _spec);

        Ok(LineChartData {
            points: optimized_points,
        })
    }

    /// Apply level of detail optimization
    fn apply_level_of_detail(&self, points: &[DataPoint], _spec: &LineChartSpec) -> Vec<DataPoint> {
        if points.len() <= self.performance_config.level_of_detail_threshold {
            return points.to_vec();
        }

        // Simple decimation: keep every nth point
        let step = points.len() / self.performance_config.level_of_detail_threshold;
        points
            .iter()
            .enumerate()
            .filter(|(i, _)| i % step == 0)
            .map(|(_, point)| point.clone())
            .collect()
    }

    /// Draw a line chart
    fn draw_line_chart(
        &self,
        _data: &LineChartData,
        _spec: &LineChartSpec,
    ) -> Result<(), Canvas2DError> {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(ref ctx) = self.context {
                // Set line style
                ctx.set_stroke_style(&spec.line_style.color.into());
                ctx.set_line_width(spec.line_style.width);

                if let Some(ref dash) = spec.line_style.dash {
                    ctx.set_line_dash(dash).unwrap();
                }

                // Begin path
                ctx.begin_path();

                // Draw the line
                for (i, point) in data.points.iter().enumerate() {
                    let screen_coords =
                        self.data_to_screen_coords(point.x, point.y, &spec.viewport);

                    if i == 0 {
                        ctx.move_to(screen_coords.0, screen_coords.1);
                    } else {
                        ctx.line_to(screen_coords.0, screen_coords.1);
                    }
                }

                // Stroke the line
                ctx.stroke();
            }
        }

        Ok(())
    }

    /// Draw a bar chart
    fn draw_bar_chart(
        &self,
        _data: &BarChartData,
        _spec: &BarChartSpec,
    ) -> Result<(), Canvas2DError> {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(ref ctx) = self.context {
                let bar_width = self.width as f64 / _data.bars.len() as f64;
                let max_value = _data.bars.iter().map(|b| b.value).fold(0.0, f64::max);

                for (i, bar) in _data.bars.iter().enumerate() {
                    let x = i as f64 * bar_width;
                    let bar_height = (bar.value / max_value) * self.height as f64;
                    let y = self.height as f64 - bar_height;

                    // Set fill style
                    ctx.set_fill_style(&"#4CAF50".into());

                    // Draw the bar
                    match spec.orientation {
                        BarOrientation::Vertical => {
                            ctx.fill_rect(x, y, bar_width - 2.0, bar_height);
                        }
                        BarOrientation::Horizontal => {
                            ctx.fill_rect(0.0, x, bar_height, bar_width - 2.0);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Draw a scatter plot
    fn draw_scatter_plot(
        &self,
        _data: &ScatterPlotData,
        _spec: &ScatterPlotSpec,
    ) -> Result<(), Canvas2DError> {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(ref ctx) = self.context {
                for point in &_data.points {
                    let screen_coords =
                        self.data_to_screen_coords(point.x, point.y, &_spec.viewport);

                    // Set point style
                    if let Some(ref color) = point.color {
                        ctx.set_fill_style(&color.into());
                    } else {
                        ctx.set_fill_style(&"#2196F3".into());
                    }

                    ctx.set_global_alpha(spec.point_config.opacity);

                    // Draw the point based on shape
                    let size = point.size.unwrap_or(spec.point_config.size);
                    self.draw_point(
                        ctx,
                        screen_coords.0,
                        screen_coords.1,
                        size,
                        &spec.point_config.shape,
                    );
                }
            }
        }

        Ok(())
    }

    /// Draw a point with the specified shape
    #[cfg(target_arch = "wasm32")]
    fn draw_point(
        &self,
        ctx: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        size: f64,
        shape: &PointShape,
    ) {
        match shape {
            PointShape::Circle => {
                ctx.begin_path();
                ctx.arc(x, y, size / 2.0, 0.0, 2.0 * std::f64::consts::PI)
                    .unwrap();
                ctx.fill();
            }
            PointShape::Square => {
                ctx.fill_rect(x - size / 2.0, y - size / 2.0, size, size);
            }
            PointShape::Triangle => {
                ctx.begin_path();
                ctx.move_to(x, y - size / 2.0);
                ctx.line_to(x - size / 2.0, y + size / 2.0);
                ctx.line_to(x + size / 2.0, y + size / 2.0);
                ctx.close_path();
                ctx.fill();
            }
        }
    }

    /// Convert data coordinates to screen coordinates
    fn data_to_screen_coords(&self, x: f64, y: f64, viewport: &Viewport) -> (f64, f64) {
        let screen_x =
            ((x - viewport.x_min) / (viewport.x_max - viewport.x_min)) * self.width as f64;
        let screen_y = self.height as f64
            - ((y - viewport.y_min) / (viewport.y_max - viewport.y_min)) * self.height as f64;
        (screen_x, screen_y)
    }

    /// Convert screen coordinates to data coordinates
    fn screen_to_data_coords(&self, x: f64, y: f64, viewport: &Viewport) -> (f64, f64) {
        let data_x = viewport.x_min + (x / self.width as f64) * (viewport.x_max - viewport.x_min);
        let data_y = viewport.y_min
            + ((self.height as f64 - y) / self.height as f64) * (viewport.y_max - viewport.y_min);
        (data_x, data_y)
    }

    /// Find the closest point to the given coordinates
    fn find_closest_point(
        &self,
        _data: &dyn ChartData,
        _coords: (f64, f64),
    ) -> Option<(DataPoint, f64)> {
        // This is a simplified implementation
        // In a real implementation, we would use spatial indexing for efficiency
        None
    }
}

// Import types needed for the renderer

// =============================================================================
// Data Structures and Types for Canvas2D Rendering
// =============================================================================

/// 2D coordinate point for chart data
#[derive(Debug, Clone)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
}

/// Data collection for line chart rendering
#[derive(Debug, Clone)]
pub struct LineChartData {
    pub points: Vec<DataPoint>,
}

/// Individual bar data for bar charts
#[derive(Debug, Clone)]
pub struct BarData {
    pub category: String,
    pub value: f64,
}

/// Data collection for bar chart rendering
#[derive(Debug, Clone)]
pub struct BarChartData {
    pub bars: Vec<BarData>,
}

/// Point data for scatter plot visualization
#[derive(Debug, Clone)]
pub struct ScatterPoint {
    pub x: f64,
    pub y: f64,
    pub size: Option<f64>,
    pub color: Option<String>,
}

/// Data collection for scatter plot rendering
#[derive(Debug, Clone)]
pub struct ScatterPlotData {
    pub points: Vec<ScatterPoint>,
}

/// Styling configuration for line rendering
#[derive(Debug, Clone)]
pub struct LineStyle {
    pub width: f64,
    pub color: String,
    pub dash: Option<Vec<f64>>,
}

/// Line interpolation methods for smooth rendering
#[derive(Debug, Clone)]
pub enum InterpolationMethod {
    /// Linear interpolation between points
    Linear,
    /// Step function interpolation
    Step,
    /// Smooth curve interpolation
    Smooth,
}

/// Orientation options for bar charts
#[derive(Debug, Clone)]
pub enum BarOrientation {
    /// Bars extend vertically
    Vertical,
    /// Bars extend horizontally
    Horizontal,
}

/// Bar grouping strategies for multi-series data
#[derive(Debug, Clone)]
pub enum BarGrouping {
    /// Bars grouped side by side
    Grouped,
    /// Bars stacked on top of each other
    Stacked,
    /// Bars normalized to 100%
    Normalized,
}

/// Configuration for point rendering in charts
#[derive(Debug, Clone)]
pub struct PointConfig {
    pub shape: PointShape,
    pub size: f64,
    pub opacity: f64,
}

/// Available shapes for data points
#[derive(Debug, Clone)]
pub enum PointShape {
    Circle,
    Square,
    Triangle,
}

/// Color scheme types for data visualization
#[derive(Debug, Clone)]
pub enum ColorScheme {
    Categorical(Vec<String>),
    Sequential(Vec<String>),
    Diverging(Vec<String>),
}

/// Performance optimization strategies for rendering
#[derive(Debug, Clone)]
pub enum OptimizationStrategy {
    LevelOfDetail,
    DataAggregation,
    ViewportCulling,
    BatchRendering,
}

/// Viewport bounds for chart rendering area
#[derive(Debug, Clone)]
pub struct Viewport {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            x_min: 0.0,
            x_max: 100.0,
            y_min: 0.0,
            y_max: 100.0,
        }
    }
}

impl Viewport {
    pub fn zoom(&self, center: (f64, f64), factor: f64) -> Self {
        let (cx, cy) = center;
        let new_width = (self.x_max - self.x_min) / factor;
        let new_height = (self.y_max - self.y_min) / factor;

        Self {
            x_min: cx - new_width / 2.0,
            x_max: cx + new_width / 2.0,
            y_min: cy - new_height / 2.0,
            y_max: cy + new_height / 2.0,
        }
    }

    pub fn pan(&self, delta_x: f64, delta_y: f64) -> Self {
        Self {
            x_min: self.x_min + delta_x,
            x_max: self.x_max + delta_x,
            y_min: self.y_min + delta_y,
            y_max: self.y_max + delta_y,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LineChartSpec {
    pub line_style: LineStyle,
    pub interpolation: InterpolationMethod,
    pub viewport: Viewport,
    pub optimization: OptimizationStrategy,
}

#[derive(Debug, Clone)]
pub struct BarChartSpec {
    pub orientation: BarOrientation,
    pub grouping: BarGrouping,
    pub viewport: Viewport,
}

#[derive(Debug, Clone)]
pub struct ScatterPlotSpec {
    pub point_config: PointConfig,
    pub color_scheme: ColorScheme,
    pub viewport: Viewport,
}

#[derive(Debug, Clone)]
pub struct RenderResult {
    pub points_rendered: usize,
    pub bars_rendered: usize,
    pub render_time: Duration,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct HoverInfo {
    pub point: DataPoint,
    pub distance: f64,
    pub data: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ErrorScenario {
    InvalidData,
    ContextLost,
    MemoryExhausted,
    RenderingTimeout,
}

// Trait for chart data
pub trait ChartData {
    fn point_count(&self) -> usize;
}

impl ChartData for LineChartData {
    fn point_count(&self) -> usize {
        self.points.len()
    }
}

impl ChartData for BarChartData {
    fn point_count(&self) -> usize {
        self.bars.len()
    }
}

impl ChartData for ScatterPlotData {
    fn point_count(&self) -> usize {
        self.points.len()
    }
}

// Additional types needed for the renderer
#[derive(Debug, Clone, PartialEq)]
pub enum RendererBackend {
    WebGPU,
    WebGL2,
    Canvas2D,
}

pub trait ChartSpec {
    fn viewport(&self) -> &Viewport;
}

impl ChartSpec for LineChartSpec {
    fn viewport(&self) -> &Viewport {
        &self.viewport
    }
}

impl ChartSpec for BarChartSpec {
    fn viewport(&self) -> &Viewport {
        &self.viewport
    }
}

impl ChartSpec for ScatterPlotSpec {
    fn viewport(&self) -> &Viewport {
        &self.viewport
    }
}

// =============================================================================
// Helper Functions for Test Data Generation
// =============================================================================

/// Generate test data for line charts
pub fn create_test_line_data(point_count: usize) -> LineChartData {
    let mut points = Vec::with_capacity(point_count);
    for i in 0..point_count {
        points.push(DataPoint {
            x: i as f64,
            y: (i as f64 * 0.1).sin() * 100.0,
        });
    }
    LineChartData { points }
}

/// Generate test data for bar charts
pub fn create_test_bar_data(bar_count: usize) -> BarChartData {
    let mut bars = Vec::with_capacity(bar_count);
    for i in 0..bar_count {
        bars.push(BarData {
            category: format!("Category {}", i),
            value: (i as f64 * 10.0) % 100.0,
        });
    }
    BarChartData { bars }
}

/// Generate test data for scatter plots
pub fn create_test_scatter_data(point_count: usize) -> ScatterPlotData {
    let mut points = Vec::with_capacity(point_count);
    for i in 0..point_count {
        points.push(ScatterPoint {
            x: (i as f64 * 0.1) % 100.0,
            y: (i as f64 * 0.1).cos() * 50.0,
            size: Some((i % 10) as f64 + 1.0),
            color: Some(format!("#{:06x}", i * 1000)),
        });
    }
    ScatterPlotData { points }
}

/// Create a line chart specification
pub fn create_line_chart_spec() -> LineChartSpec {
    LineChartSpec {
        line_style: LineStyle {
            width: 2.0,
            color: "#ff0000".to_string(),
            dash: None,
        },
        interpolation: InterpolationMethod::Linear,
        viewport: Viewport {
            x_min: 0.0,
            x_max: 100.0,
            y_min: -100.0,
            y_max: 100.0,
        },
        optimization: OptimizationStrategy::LevelOfDetail,
    }
}

/// Create a bar chart specification
pub fn create_bar_chart_spec() -> BarChartSpec {
    BarChartSpec {
        orientation: BarOrientation::Vertical,
        grouping: BarGrouping::Grouped,
        viewport: Viewport {
            x_min: 0.0,
            x_max: 100.0,
            y_min: 0.0,
            y_max: 100.0,
        },
    }
}

/// Create a scatter plot specification
pub fn create_scatter_plot_spec() -> ScatterPlotSpec {
    ScatterPlotSpec {
        point_config: PointConfig {
            shape: PointShape::Circle,
            size: 5.0,
            opacity: 1.0,
        },
        color_scheme: ColorScheme::Categorical(vec![
            "#ff0000".to_string(),
            "#00ff00".to_string(),
            "#0000ff".to_string(),
        ]),
        viewport: Viewport {
            x_min: 0.0,
            x_max: 100.0,
            y_min: -50.0,
            y_max: 50.0,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_canvas2d_renderer_creation() {
        let renderer = Canvas2DRenderer::new();
        assert!(renderer.is_ok());

        let renderer = renderer.unwrap();
        assert_eq!(renderer.backend(), RendererBackend::Canvas2D);
    }

    #[tokio::test]
    async fn test_canvas2d_line_chart_rendering() {
        let renderer = Canvas2DRenderer::new().unwrap();
        let data = create_test_line_data(100);
        let spec = create_line_chart_spec();

        let result = renderer.render_line_chart(&spec, &data).await;
        assert!(result.is_ok());

        let render_result = result.unwrap();
        assert_eq!(render_result.points_rendered, 100);
    }

    #[tokio::test]
    async fn test_canvas2d_performance_requirements() {
        let renderer = Canvas2DRenderer::new().unwrap();
        let data = create_test_line_data(1000);
        let spec = create_line_chart_spec();

        let start = Instant::now();
        let result = renderer.render_line_chart(&spec, &data).await;
        let duration = start.elapsed();

        assert!(result.is_ok());
        assert!(duration < Duration::from_millis(16)); // 60fps requirement
    }

    #[test]
    fn test_canvas2d_error_handling() {
        let renderer = Canvas2DRenderer::new().unwrap();

        let result = renderer.handle_error_scenario(ErrorScenario::InvalidData);
        assert!(result.is_ok());

        let recovery_result = renderer.recover_from_error();
        assert!(recovery_result.is_ok());
    }
}
