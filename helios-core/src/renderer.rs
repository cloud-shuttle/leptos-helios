//! Chart Renderer Implementation
//! WebGPU/WebGL2/Canvas2D renderer implementations for chart rendering

use crate::chart_config::*;

/// Renderer backend types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RendererBackend {
    WebGPU,
    WebGL2,
    Canvas2D,
}

/// WebGPU renderer implementation
pub struct WebGpuRenderer {
    backend: RendererBackend,
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    // Note: Surface requires lifetime parameter, but we'll use Option<()> for now
    surface: Option<()>,
}

impl WebGpuRenderer {
    pub fn new() -> Result<Self, ChartRenderError> {
        // In a real implementation, this would initialize WebGPU
        // For now, we'll create a mock implementation
        Ok(Self {
            backend: RendererBackend::WebGPU,
            device: None,
            queue: None,
            surface: None,
        })
    }

    pub fn backend(&self) -> RendererBackend {
        self.backend
    }
}

/// WebGL2 renderer implementation
pub struct WebGl2Renderer {
    backend: RendererBackend,
    // Note: Using Option<()> for now since web_sys types aren't available in tests
    context: Option<()>,
}

impl WebGl2Renderer {
    pub fn new() -> Result<Self, ChartRenderError> {
        // In a real implementation, this would initialize WebGL2
        // For now, we'll create a mock implementation
        Ok(Self {
            backend: RendererBackend::WebGL2,
            context: None,
        })
    }

    pub fn backend(&self) -> RendererBackend {
        self.backend
    }
}

/// Canvas2D renderer implementation
pub struct Canvas2DRenderer {
    backend: RendererBackend,
    // Note: Using Option<()> for now since web_sys types aren't available in tests
    context: Option<()>,
}

impl Canvas2DRenderer {
    pub fn new() -> Result<Self, ChartRenderError> {
        // In a real implementation, this would initialize Canvas2D
        // For now, we'll create a mock implementation
        Ok(Self {
            backend: RendererBackend::Canvas2D,
            context: None,
        })
    }

    pub fn backend(&self) -> RendererBackend {
        self.backend
    }
}

/// Main renderer that can use different backends
pub struct Renderer {
    backend: RendererBackend,
    webgpu_renderer: Option<WebGpuRenderer>,
    webgl2_renderer: Option<WebGl2Renderer>,
    canvas2d_renderer: Option<Canvas2DRenderer>,
}

impl Renderer {
    /// Create a new renderer with auto-detection of the best available backend
    pub fn auto_detect() -> Result<Self, ChartRenderError> {
        // Try WebGPU first, then WebGL2, then Canvas2D
        if let Ok(webgpu) = WebGpuRenderer::new() {
            return Ok(Self {
                backend: RendererBackend::WebGPU,
                webgpu_renderer: Some(webgpu),
                webgl2_renderer: None,
                canvas2d_renderer: None,
            });
        }

        if let Ok(webgl2) = WebGl2Renderer::new() {
            return Ok(Self {
                backend: RendererBackend::WebGL2,
                webgpu_renderer: None,
                webgl2_renderer: Some(webgl2),
                canvas2d_renderer: None,
            });
        }

        if let Ok(canvas2d) = Canvas2DRenderer::new() {
            return Ok(Self {
                backend: RendererBackend::Canvas2D,
                webgpu_renderer: None,
                webgl2_renderer: None,
                canvas2d_renderer: Some(canvas2d),
            });
        }

        Err(ChartRenderError::BackendError(
            "No suitable rendering backend found".to_string(),
        ))
    }

    /// Create a renderer with a specific backend
    pub fn new(backend: RendererBackend) -> Result<Self, ChartRenderError> {
        match backend {
            RendererBackend::WebGPU => {
                let webgpu = WebGpuRenderer::new()?;
                Ok(Self {
                    backend,
                    webgpu_renderer: Some(webgpu),
                    webgl2_renderer: None,
                    canvas2d_renderer: None,
                })
            }
            RendererBackend::WebGL2 => {
                let webgl2 = WebGl2Renderer::new()?;
                Ok(Self {
                    backend,
                    webgpu_renderer: None,
                    webgl2_renderer: Some(webgl2),
                    canvas2d_renderer: None,
                })
            }
            RendererBackend::Canvas2D => {
                let canvas2d = Canvas2DRenderer::new()?;
                Ok(Self {
                    backend,
                    webgpu_renderer: None,
                    webgl2_renderer: None,
                    canvas2d_renderer: Some(canvas2d),
                })
            }
        }
    }

    /// Get the current backend
    pub fn backend(&self) -> RendererBackend {
        self.backend
    }

    /// Switch to a different backend
    pub fn switch_backend(&mut self, new_backend: RendererBackend) -> Result<(), ChartRenderError> {
        match new_backend {
            RendererBackend::WebGPU => {
                let webgpu = WebGpuRenderer::new()?;
                self.backend = RendererBackend::WebGPU;
                self.webgpu_renderer = Some(webgpu);
                self.webgl2_renderer = None;
                self.canvas2d_renderer = None;
            }
            RendererBackend::WebGL2 => {
                let webgl2 = WebGl2Renderer::new()?;
                self.backend = RendererBackend::WebGL2;
                self.webgpu_renderer = None;
                self.webgl2_renderer = Some(webgl2);
                self.canvas2d_renderer = None;
            }
            RendererBackend::Canvas2D => {
                let canvas2d = Canvas2DRenderer::new()?;
                self.backend = RendererBackend::Canvas2D;
                self.webgpu_renderer = None;
                self.webgl2_renderer = None;
                self.canvas2d_renderer = Some(canvas2d);
            }
        }
        Ok(())
    }

    /// Check if a specific backend is available
    pub fn is_backend_available(backend: RendererBackend) -> bool {
        match backend {
            RendererBackend::WebGPU => WebGpuRenderer::new().is_ok(),
            RendererBackend::WebGL2 => WebGl2Renderer::new().is_ok(),
            RendererBackend::Canvas2D => Canvas2DRenderer::new().is_ok(),
        }
    }

    /// Get the best available backend
    pub fn get_best_backend() -> Option<RendererBackend> {
        if Self::is_backend_available(RendererBackend::WebGPU) {
            Some(RendererBackend::WebGPU)
        } else if Self::is_backend_available(RendererBackend::WebGL2) {
            Some(RendererBackend::WebGL2)
        } else if Self::is_backend_available(RendererBackend::Canvas2D) {
            Some(RendererBackend::Canvas2D)
        } else {
            None
        }
    }

    /// Render a line chart
    pub fn render_line_chart(
        &self,
        _data: &[(f64, f64)],
        config: &LineChartConfig,
    ) -> Result<RenderResult, ChartRenderError> {
        // Validate configuration
        if config.base.width == 0 || config.base.height == 0 {
            return Err(ChartRenderError::InvalidConfig(
                "Width and height must be greater than 0".to_string(),
            ));
        }

        // Create mock pixel data for testing
        let pixel_count = (config.base.width * config.base.height) as usize;
        let mut pixel_data = vec![0u8; pixel_count * 4]; // RGBA

        // Fill with a simple pattern based on the data
        for (i, pixel) in pixel_data.chunks_exact_mut(4).enumerate() {
            let x = i % config.base.width as usize;
            let y = i / config.base.width as usize;

            // Simple gradient pattern
            pixel[0] = (x * 255 / config.base.width as usize) as u8; // R
            pixel[1] = (y * 255 / config.base.height as usize) as u8; // G
            pixel[2] = 128; // B
            pixel[3] = 255; // A
        }

        Ok(RenderResult {
            width: config.base.width,
            height: config.base.height,
            pixel_data,
            format: PixelFormat::RGBA8,
        })
    }

    /// Render a bar chart
    pub fn render_bar_chart(
        &self,
        data: &[(String, f64)],
        config: &BarChartConfig,
    ) -> Result<RenderResult, ChartRenderError> {
        // Validate configuration
        if config.base.width == 0 || config.base.height == 0 {
            return Err(ChartRenderError::InvalidConfig(
                "Width and height must be greater than 0".to_string(),
            ));
        }

        // Create mock pixel data for testing
        let pixel_count = (config.base.width * config.base.height) as usize;
        let mut pixel_data = vec![0u8; pixel_count * 4]; // RGBA

        // Fill with a pattern representing bars
        for (i, pixel) in pixel_data.chunks_exact_mut(4).enumerate() {
            let x = i % config.base.width as usize;
            let y = i / config.base.width as usize;

            // Bar pattern
            let bar_width = config.base.width as usize / data.len().max(1);
            let bar_index = x / bar_width;

            if bar_index < data.len() {
                let value = data[bar_index].1;
                let max_value = data.iter().map(|(_, v)| *v).fold(0.0, f64::max);
                let normalized_value = if max_value > 0.0 {
                    value / max_value
                } else {
                    0.0
                };

                if y as f64 / config.base.height as f64 > (1.0 - normalized_value) {
                    // Bar color
                    let color_index = bar_index % config.colors.len();
                    let color = &config.colors[color_index];
                    let (r, g, b) = parse_color(color);
                    pixel[0] = r;
                    pixel[1] = g;
                    pixel[2] = b;
                    pixel[3] = 255;
                } else {
                    // Background
                    pixel[0] = 255;
                    pixel[1] = 255;
                    pixel[2] = 255;
                    pixel[3] = 255;
                }
            } else {
                // Background
                pixel[0] = 255;
                pixel[1] = 255;
                pixel[2] = 255;
                pixel[3] = 255;
            }
        }

        Ok(RenderResult {
            width: config.base.width,
            height: config.base.height,
            pixel_data,
            format: PixelFormat::RGBA8,
        })
    }

    /// Render a scatter plot
    pub fn render_scatter_plot(
        &self,
        data: &[(f64, f64, Option<String>)],
        config: &ScatterPlotConfig,
    ) -> Result<RenderResult, ChartRenderError> {
        // Validate configuration
        if config.base.width == 0 || config.base.height == 0 {
            return Err(ChartRenderError::InvalidConfig(
                "Width and height must be greater than 0".to_string(),
            ));
        }

        // Create mock pixel data for testing
        let pixel_count = (config.base.width * config.base.height) as usize;
        let mut pixel_data = vec![0u8; pixel_count * 4]; // RGBA

        // Fill background
        for pixel in pixel_data.chunks_exact_mut(4) {
            pixel[0] = 255; // R
            pixel[1] = 255; // G
            pixel[2] = 255; // B
            pixel[3] = 255; // A
        }

        // Draw scatter points
        if !data.is_empty() {
            let (min_x, max_x) = data
                .iter()
                .map(|(x, _, _)| *x)
                .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), x| {
                    (min.min(x), max.max(x))
                });
            let (min_y, max_y) = data
                .iter()
                .map(|(_, y, _)| *y)
                .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), y| {
                    (min.min(y), max.max(y))
                });

            let (r, g, b) = parse_color(&config.point_color);
            let opacity = config.opacity.unwrap_or(1.0);

            for (x, y, _label) in data {
                let normalized_x = (x - min_x) / (max_x - min_x);
                let normalized_y = (y - min_y) / (max_y - min_y);

                let pixel_x = (normalized_x * (config.base.width - 1) as f64) as usize;
                let pixel_y = (normalized_y * (config.base.height - 1) as f64) as usize;

                if pixel_x < config.base.width as usize && pixel_y < config.base.height as usize {
                    let index = (pixel_y * config.base.width as usize + pixel_x) * 4;
                    if index + 3 < pixel_data.len() {
                        pixel_data[index] = r;
                        pixel_data[index + 1] = g;
                        pixel_data[index + 2] = b;
                        pixel_data[index + 3] = (255.0 * opacity) as u8;
                    }
                }
            }

            // Draw trend line if enabled
            if config.show_trend_line && data.len() > 1 {
                let (trend_r, trend_g, trend_b) = parse_color(&config.trend_line_color);

                // Simple linear regression for trend line
                let n = data.len() as f64;
                let sum_x: f64 = data.iter().map(|(x, _, _)| *x).sum();
                let sum_y: f64 = data.iter().map(|(_, y, _)| *y).sum();
                let sum_xy: f64 = data.iter().map(|(x, y, _)| *x * *y).sum();
                let sum_x2: f64 = data.iter().map(|(x, _, _)| *x * *x).sum();

                let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
                let intercept = (sum_y - slope * sum_x) / n;

                // Draw trend line
                let start_x = min_x;
                let end_x = max_x;
                let start_y = slope * start_x + intercept;
                let end_y = slope * end_x + intercept;

                let start_pixel_x =
                    ((start_x - min_x) / (max_x - min_x) * (config.base.width - 1) as f64) as usize;
                let start_pixel_y = ((start_y - min_y) / (max_y - min_y)
                    * (config.base.height - 1) as f64) as usize;
                let end_pixel_x =
                    ((end_x - min_x) / (max_x - min_x) * (config.base.width - 1) as f64) as usize;
                let end_pixel_y =
                    ((end_y - min_y) / (max_y - min_y) * (config.base.height - 1) as f64) as usize;

                // Simple line drawing
                let steps = ((end_pixel_x as i32 - start_pixel_x as i32)
                    .abs()
                    .max((end_pixel_y as i32 - start_pixel_y as i32).abs())
                    as usize)
                    .max(1);
                for step in 0..=steps {
                    let t = step as f64 / steps as f64;
                    let x = start_pixel_x as f64 + t * (end_pixel_x as f64 - start_pixel_x as f64);
                    let y = start_pixel_y as f64 + t * (end_pixel_y as f64 - start_pixel_y as f64);

                    if x >= 0.0
                        && x < config.base.width as f64
                        && y >= 0.0
                        && y < config.base.height as f64
                    {
                        let index = (y as usize * config.base.width as usize + x as usize) * 4;
                        if index + 3 < pixel_data.len() {
                            pixel_data[index] = trend_r;
                            pixel_data[index + 1] = trend_g;
                            pixel_data[index + 2] = trend_b;
                            pixel_data[index + 3] = 255;
                        }
                    }
                }
            }
        }

        Ok(RenderResult {
            width: config.base.width,
            height: config.base.height,
            pixel_data,
            format: PixelFormat::RGBA8,
        })
    }

    /// Render a heatmap
    pub fn render_heatmap(
        &self,
        data: &[Vec<f64>],
        config: &HeatmapConfig,
    ) -> Result<RenderResult, ChartRenderError> {
        // Validate configuration
        if config.base.width == 0 || config.base.height == 0 {
            return Err(ChartRenderError::InvalidConfig(
                "Width and height must be greater than 0".to_string(),
            ));
        }

        if data.is_empty() || data[0].is_empty() {
            return Err(ChartRenderError::InvalidConfig(
                "Heatmap data cannot be empty".to_string(),
            ));
        }

        // Create mock pixel data for testing
        let pixel_count = (config.base.width * config.base.height) as usize;
        let mut pixel_data = vec![0u8; pixel_count * 4]; // RGBA

        // Find min/max values for normalization
        let mut min_val = f64::INFINITY;
        let mut max_val = f64::NEG_INFINITY;

        for row in data {
            for &val in row {
                min_val = min_val.min(val);
                max_val = max_val.max(val);
            }
        }

        let value_range = max_val - min_val;

        // Fill with heatmap pattern
        for (i, pixel) in pixel_data.chunks_exact_mut(4).enumerate() {
            let x = i % config.base.width as usize;
            let y = i / config.base.width as usize;

            // Map pixel coordinates to data coordinates
            let data_x = (x * data[0].len()) / config.base.width as usize;
            let data_y = (y * data.len()) / config.base.height as usize;

            if data_y < data.len() && data_x < data[data_y].len() {
                let value = data[data_y][data_x];
                let normalized = if value_range > 0.0 {
                    (value - min_val) / value_range
                } else {
                    0.0
                };

                // Simple color mapping (blue to red)
                pixel[0] = (normalized * 255.0) as u8; // R
                pixel[1] = 0; // G
                pixel[2] = ((1.0 - normalized) * 255.0) as u8; // B
                pixel[3] = 255; // A
            } else {
                // Background
                pixel[0] = 255;
                pixel[1] = 255;
                pixel[2] = 255;
                pixel[3] = 255;
            }
        }

        Ok(RenderResult {
            width: config.base.width,
            height: config.base.height,
            pixel_data,
            format: PixelFormat::RGBA8,
        })
    }

    /// Render an area chart
    pub fn render_area_chart(
        &self,
        data: &[(f64, f64)],
        config: &AreaChartConfig,
    ) -> Result<RenderResult, ChartRenderError> {
        // Validate input
        if data.is_empty() {
            return Err(ChartRenderError::InvalidConfig(
                "Data cannot be empty".to_string(),
            ));
        }

        if config.base.width == 0 || config.base.height == 0 {
            return Err(ChartRenderError::InvalidConfig(
                "Width and height must be greater than 0".to_string(),
            ));
        }

        // Create pixel buffer
        let mut pixel_data = vec![0u8; (config.base.width * config.base.height * 4) as usize];

        // Parse colors
        let (fill_r, fill_g, fill_b) = parse_color(&config.fill_color);
        let (stroke_r, stroke_g, stroke_b) = parse_color(&config.stroke_color);

        // Find data bounds
        let (min_x, max_x, min_y, max_y) = data.iter().fold(
            (
                f64::INFINITY,
                f64::NEG_INFINITY,
                f64::INFINITY,
                f64::NEG_INFINITY,
            ),
            |(min_x, max_x, min_y, max_y), (x, y)| {
                (min_x.min(*x), max_x.max(*x), min_y.min(*y), max_y.max(*y))
            },
        );

        let x_range = max_x - min_x;
        let y_range = max_y - min_y;

        if x_range <= 0.0 || y_range <= 0.0 {
            return Err(ChartRenderError::InvalidConfig(
                "Invalid data range".to_string(),
            ));
        }

        // Render area chart
        let margin = 50.0;
        let chart_width = config.base.width as f64 - 2.0 * margin;
        let chart_height = config.base.height as f64 - 2.0 * margin;

        // Fill area under the curve
        for y in 0..config.base.height {
            for x in 0..config.base.width {
                let pixel_index = ((y * config.base.width + x) * 4) as usize;

                // Convert screen coordinates to data coordinates
                let screen_x = x as f64;
                let screen_y = y as f64;

                if screen_x >= margin
                    && screen_x <= config.base.width as f64 - margin
                    && screen_y >= margin
                    && screen_y <= config.base.height as f64 - margin
                {
                    let data_x = min_x + (screen_x - margin) / chart_width * x_range;
                    let data_y = max_y - (screen_y - margin) / chart_height * y_range;

                    // Check if point is under the curve
                    let mut is_under_curve = false;
                    for i in 0..data.len() - 1 {
                        let (x1, y1) = data[i];
                        let (x2, y2) = data[i + 1];

                        if data_x >= x1 && data_x <= x2 && data_y <= y1.max(y2) {
                            is_under_curve = true;
                            break;
                        }
                    }

                    if is_under_curve {
                        pixel_data[pixel_index] = fill_r;
                        pixel_data[pixel_index + 1] = fill_g;
                        pixel_data[pixel_index + 2] = fill_b;
                        pixel_data[pixel_index + 3] = (255.0 * config.opacity) as u8;
                    } else {
                        // Background
                        pixel_data[pixel_index] = 255;
                        pixel_data[pixel_index + 1] = 255;
                        pixel_data[pixel_index + 2] = 255;
                        pixel_data[pixel_index + 3] = 255;
                    }
                } else {
                    // Background
                    pixel_data[pixel_index] = 255;
                    pixel_data[pixel_index + 1] = 255;
                    pixel_data[pixel_index + 2] = 255;
                    pixel_data[pixel_index + 3] = 255;
                }
            }
        }

        // Draw stroke line
        for i in 0..data.len() - 1 {
            let (x1, y1) = data[i];
            let (x2, y2) = data[i + 1];

            let screen_x1 = margin + (x1 - min_x) / x_range * chart_width;
            let screen_y1 = margin + (max_y - y1) / y_range * chart_height;
            let screen_x2 = margin + (x2 - min_x) / x_range * chart_width;
            let screen_y2 = margin + (max_y - y2) / y_range * chart_height;

            // Simple line drawing
            let steps = ((screen_x2 - screen_x1)
                .abs()
                .max((screen_y2 - screen_y1).abs()) as usize)
                .max(1);
            for step in 0..=steps {
                let t = step as f64 / steps as f64;
                let x = screen_x1 + t * (screen_x2 - screen_x1);
                let y = screen_y1 + t * (screen_y2 - screen_y1);

                if x >= 0.0
                    && x < config.base.width as f64
                    && y >= 0.0
                    && y < config.base.height as f64
                {
                    let pixel_index = ((y as u32 * config.base.width + x as u32) * 4) as usize;
                    pixel_data[pixel_index] = stroke_r;
                    pixel_data[pixel_index + 1] = stroke_g;
                    pixel_data[pixel_index + 2] = stroke_b;
                    pixel_data[pixel_index + 3] = 255;
                }
            }
        }

        Ok(RenderResult {
            width: config.base.width,
            height: config.base.height,
            pixel_data,
            format: PixelFormat::RGBA8,
        })
    }

    /// Render a stacked area chart
    pub fn render_stacked_area_chart(
        &self,
        data: &[StackedAreaData],
        config: &StackedAreaChartConfig,
    ) -> Result<RenderResult, ChartRenderError> {
        // Validate input
        if data.is_empty() {
            return Err(ChartRenderError::InvalidConfig(
                "Data cannot be empty".to_string(),
            ));
        }

        if config.base.width == 0 || config.base.height == 0 {
            return Err(ChartRenderError::InvalidConfig(
                "Width and height must be greater than 0".to_string(),
            ));
        }

        // Create pixel buffer
        let mut pixel_data = vec![0u8; (config.base.width * config.base.height * 4) as usize];

        // Find data bounds
        let (min_x, max_x, min_y, max_y) = data.iter().fold(
            (
                f64::INFINITY,
                f64::NEG_INFINITY,
                f64::INFINITY,
                f64::NEG_INFINITY,
            ),
            |(min_x, max_x, min_y, max_y), point| {
                let total_value: f64 = point.values.iter().sum();
                (
                    min_x.min(point.x),
                    max_x.max(point.x),
                    min_y.min(0.0),
                    max_y.max(total_value),
                )
            },
        );

        let x_range = max_x - min_x;
        let y_range = max_y - min_y;

        if x_range <= 0.0 || y_range <= 0.0 {
            return Err(ChartRenderError::InvalidConfig(
                "Invalid data range".to_string(),
            ));
        }

        // Render stacked area chart
        let margin = 50.0;
        let chart_width = config.base.width as f64 - 2.0 * margin;
        let chart_height = config.base.height as f64 - 2.0 * margin;

        // Fill background
        for pixel in pixel_data.chunks_exact_mut(4) {
            pixel[0] = 255;
            pixel[1] = 255;
            pixel[2] = 255;
            pixel[3] = 255;
        }

        // Render each series
        for (series_idx, color) in config.colors.iter().enumerate() {
            let (r, g, b) = parse_color(color);

            // Draw area for this series
            for i in 0..data.len() - 1 {
                let point1 = &data[i];
                let point2 = &data[i + 1];

                if series_idx < point1.values.len() && series_idx < point2.values.len() {
                    let y1_bottom = point1.values[..series_idx].iter().sum::<f64>();
                    let y1_top = y1_bottom + point1.values[series_idx];
                    let y2_bottom = point2.values[..series_idx].iter().sum::<f64>();
                    let y2_top = y2_bottom + point2.values[series_idx];

                    // Convert to screen coordinates
                    let screen_x1 = margin + (point1.x - min_x) / x_range * chart_width;
                    let screen_x2 = margin + (point2.x - min_x) / x_range * chart_width;
                    let screen_y1_bottom = margin + (max_y - y1_bottom) / y_range * chart_height;
                    let screen_y1_top = margin + (max_y - y1_top) / y_range * chart_height;
                    let screen_y2_bottom = margin + (max_y - y2_bottom) / y_range * chart_height;
                    let screen_y2_top = margin + (max_y - y2_top) / y_range * chart_height;

                    // Fill area between the curves
                    let steps = ((screen_x2 - screen_x1).abs() as usize).max(1);
                    for step in 0..=steps {
                        let t = step as f64 / steps as f64;
                        let x = screen_x1 + t * (screen_x2 - screen_x1);
                        let y_bottom = screen_y1_bottom + t * (screen_y2_bottom - screen_y1_bottom);
                        let y_top = screen_y1_top + t * (screen_y2_top - screen_y1_top);

                        if x >= 0.0 && x < config.base.width as f64 {
                            let start_y = y_bottom.min(y_top) as u32;
                            let end_y = y_bottom.max(y_top) as u32;

                            for y in start_y..=end_y {
                                if y < config.base.height {
                                    let pixel_index =
                                        ((y * config.base.width + x as u32) * 4) as usize;
                                    pixel_data[pixel_index] = r;
                                    pixel_data[pixel_index + 1] = g;
                                    pixel_data[pixel_index + 2] = b;
                                    pixel_data[pixel_index + 3] = (255.0 * config.opacity) as u8;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(RenderResult {
            width: config.base.width,
            height: config.base.height,
            pixel_data,
            format: PixelFormat::RGBA8,
        })
    }
}

/// Parse a hex color string to RGB values
fn parse_color(color: &str) -> (u8, u8, u8) {
    if color.starts_with('#') && color.len() == 7 {
        let r = u8::from_str_radix(&color[1..3], 16).unwrap_or(0);
        let g = u8::from_str_radix(&color[3..5], 16).unwrap_or(0);
        let b = u8::from_str_radix(&color[5..7], 16).unwrap_or(0);
        (r, g, b)
    } else {
        // Default color
        (0, 212, 255) // #00d4ff
    }
}
