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
        data: &[(&str, f64)],
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
        data: &[(f64, f64)],
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
                .map(|(x, _)| *x)
                .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), x| {
                    (min.min(x), max.max(x))
                });
            let (min_y, max_y) = data
                .iter()
                .map(|(_, y)| *y)
                .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), y| {
                    (min.min(y), max.max(y))
                });

            let (r, g, b) = parse_color(&config.point_color);

            for (x, y) in data {
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
                        pixel_data[index + 3] = 255;
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
