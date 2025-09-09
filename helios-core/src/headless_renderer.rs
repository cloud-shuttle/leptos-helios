//! Headless Rendering API
//!
//! This module provides server-side chart generation capabilities for Helios,
//! enabling chart rendering without a browser environment.

use crate::chart::ChartSpec;
use crate::export_system::ExportConfig;
use polars::prelude::DataFrame;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

/// Headless rendering errors
#[derive(Debug, thiserror::Error)]
pub enum HeadlessError {
    #[error("Browser initialization failed: {0}")]
    BrowserInitFailed(String),

    #[error("Rendering failed: {0}")]
    RenderingFailed(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Timeout error: {0}")]
    TimeoutError(String),

    #[error("Resource error: {0}")]
    ResourceError(String),
}

/// Headless rendering configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadlessConfig {
    /// Browser executable path (e.g., Chrome, Chromium)
    pub browser_path: Option<String>,

    /// Browser arguments
    pub browser_args: Vec<String>,

    /// Viewport dimensions
    pub viewport_width: u32,
    pub viewport_height: u32,

    /// Rendering timeout in milliseconds
    pub timeout_ms: u64,

    /// Enable GPU acceleration
    pub enable_gpu: bool,

    /// Memory limit in MB
    pub memory_limit_mb: u64,

    /// Enable JavaScript
    pub enable_javascript: bool,

    /// User agent string
    pub user_agent: Option<String>,

    /// Additional headers
    pub headers: HashMap<String, String>,
}

impl Default for HeadlessConfig {
    fn default() -> Self {
        Self {
            browser_path: None,
            browser_args: vec![
                "--headless".to_string(),
                "--no-sandbox".to_string(),
                "--disable-dev-shm-usage".to_string(),
                "--disable-gpu".to_string(),
                "--disable-web-security".to_string(),
                "--disable-features=VizDisplayCompositor".to_string(),
            ],
            viewport_width: 1920,
            viewport_height: 1080,
            timeout_ms: 30000,
            enable_gpu: false,
            memory_limit_mb: 512,
            enable_javascript: true,
            user_agent: Some("Helios-Headless/1.0".to_string()),
            headers: HashMap::new(),
        }
    }
}

/// Headless rendering engine
#[derive(Clone)]
pub struct HeadlessRenderer {
    config: HeadlessConfig,
    browser_initialized: bool,
    render_count: u64,
    total_render_time: std::time::Duration,
}

impl HeadlessRenderer {
    /// Create a new headless renderer
    pub fn new(config: HeadlessConfig) -> Result<Self, HeadlessError> {
        Ok(Self {
            config,
            browser_initialized: false,
            render_count: 0,
            total_render_time: std::time::Duration::ZERO,
        })
    }

    /// Initialize the headless browser
    pub async fn initialize(&mut self) -> Result<(), HeadlessError> {
        // Mock initialization - in real implementation would start browser process
        self.browser_initialized = true;
        Ok(())
    }

    /// Render chart to PNG
    pub async fn render_to_png(
        &mut self,
        spec: &ChartSpec,
        data: &DataFrame,
        width: u32,
        height: u32,
        dpi: Option<u32>,
        config: &ExportConfig,
    ) -> Result<Vec<u8>, HeadlessError> {
        if !self.browser_initialized {
            return Err(HeadlessError::BrowserInitFailed(
                "Browser not initialized".to_string(),
            ));
        }

        let start_time = Instant::now();

        // Mock PNG generation - would use headless browser to render and capture
        let mut png_data = Vec::new();
        png_data.extend_from_slice(b"\x89PNG\x0d\x0a\x1a\x0a"); // PNG signature

        // Add mock image data based on chart spec
        let image_size = (width * height * 4) as usize;
        let mut image_data = vec![0u8; image_size];

        // Generate mock chart visualization
        self.generate_mock_chart_data(&mut image_data, width, height, spec, data);

        // Compress to PNG (mock)
        png_data.extend_from_slice(&image_size.to_be_bytes());
        png_data.extend_from_slice(&image_data);

        let render_time = start_time.elapsed();
        self.render_count += 1;
        self.total_render_time += render_time;

        Ok(png_data)
    }

    /// Render chart to PDF
    pub async fn render_to_pdf(
        &mut self,
        spec: &ChartSpec,
        data: &DataFrame,
        width: f32,
        height: f32,
        config: &ExportConfig,
    ) -> Result<Vec<u8>, HeadlessError> {
        if !self.browser_initialized {
            return Err(HeadlessError::BrowserInitFailed(
                "Browser not initialized".to_string(),
            ));
        }

        let start_time = Instant::now();

        // Mock PDF generation
        let mut pdf_data = Vec::new();
        pdf_data.extend_from_slice(b"%PDF-1.4\n"); // PDF header

        // Add mock PDF content
        let content = format!(
            r#"1 0 obj
<<
/Type /Catalog
/Pages 2 0 R
>>
endobj

2 0 obj
<<
/Type /Pages
/Kids [3 0 R]
/Count 1
>>
endobj

3 0 obj
<<
/Type /Page
/Parent 2 0 R
/MediaBox [0 0 {} {}]
/Contents 4 0 R
>>
endobj

4 0 obj
<<
/Length {}
>>
stream
BT
/F1 12 Tf
100 700 Td
(Helios Chart) Tj
ET
endstream
endobj

xref
0 5
0000000000 65535 f
0000000009 00000 n
0000000058 00000 n
0000000115 00000 n
0000000204 00000 n
trailer
<<
/Size 5
/Root 1 0 R
>>
startxref
{}
%%EOF"#,
            width, height, 50, 300
        );

        pdf_data.extend_from_slice(content.as_bytes());

        let render_time = start_time.elapsed();
        self.render_count += 1;
        self.total_render_time += render_time;

        Ok(pdf_data)
    }

    /// Render chart to SVG
    pub async fn render_to_svg(
        &mut self,
        spec: &ChartSpec,
        data: &DataFrame,
        width: Option<u32>,
        height: Option<u32>,
        config: &ExportConfig,
    ) -> Result<String, HeadlessError> {
        if !self.browser_initialized {
            return Err(HeadlessError::BrowserInitFailed(
                "Browser not initialized".to_string(),
            ));
        }

        let start_time = Instant::now();

        let w = width.unwrap_or(800);
        let h = height.unwrap_or(600);
        let bg_color = config.background_color.as_deref().unwrap_or("#ffffff");

        // Generate SVG content based on chart spec
        let svg_content = self.generate_svg_content(spec, data, w, h, bg_color);

        let render_time = start_time.elapsed();
        self.render_count += 1;
        self.total_render_time += render_time;

        Ok(svg_content)
    }

    /// Render chart to HTML
    pub async fn render_to_html(
        &mut self,
        spec: &ChartSpec,
        data: &DataFrame,
        standalone: bool,
        include_data: bool,
        config: &ExportConfig,
    ) -> Result<String, HeadlessError> {
        if !self.browser_initialized {
            return Err(HeadlessError::BrowserInitFailed(
                "Browser not initialized".to_string(),
            ));
        }

        let start_time = Instant::now();

        let html_content = if standalone {
            self.generate_standalone_html(spec, data, include_data, config)
        } else {
            self.generate_embedded_html(spec, data, include_data, config)
        };

        let render_time = start_time.elapsed();
        self.render_count += 1;
        self.total_render_time += render_time;

        Ok(html_content)
    }

    /// Get rendering statistics
    pub fn get_stats(&self) -> HeadlessStats {
        HeadlessStats {
            render_count: self.render_count,
            total_render_time: self.total_render_time,
            average_render_time: if self.render_count > 0 {
                self.total_render_time / self.render_count as u32
            } else {
                std::time::Duration::ZERO
            },
            browser_initialized: self.browser_initialized,
        }
    }

    /// Close the headless browser
    pub async fn close(&mut self) -> Result<(), HeadlessError> {
        self.browser_initialized = false;
        Ok(())
    }

    // Private helper methods

    fn generate_mock_chart_data(
        &self,
        image_data: &mut [u8],
        width: u32,
        height: u32,
        spec: &ChartSpec,
        data: &DataFrame,
    ) {
        // Mock chart data generation based on chart type
        // In real implementation, this would render the actual chart

        let center_x = width / 2;
        let center_y = height / 2;

        // Generate a simple pattern based on data
        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;
                if idx + 3 < image_data.len() {
                    // Create a gradient pattern
                    let r = ((x * 255) / width) as u8;
                    let g = ((y * 255) / height) as u8;
                    let b = 128;
                    let a = 255;

                    image_data[idx] = r;
                    image_data[idx + 1] = g;
                    image_data[idx + 2] = b;
                    image_data[idx + 3] = a;
                }
            }
        }
    }

    fn generate_svg_content(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
        width: u32,
        height: u32,
        bg_color: &str,
    ) -> String {
        format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">
    <rect width="100%" height="100%" fill="{}"/>
    <text x="50%" y="50%" text-anchor="middle" font-family="Arial" font-size="16" fill="rgb(51,51,51)">
        Helios Chart (SVG)
    </text>
    <text x="50%" y="60%" text-anchor="middle" font-family="Arial" font-size="12" fill="rgb(102,102,102)">
        Data points: {}
    </text>
    <!-- Chart content would be rendered here based on spec -->
</svg>"#,
            width,
            height,
            bg_color,
            data.height()
        )
    }

    fn generate_standalone_html(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
        include_data: bool,
        config: &ExportConfig,
    ) -> String {
        let data_section = if include_data {
            format!("const chartData = {};", self.serialize_dataframe(data))
        } else {
            "// Data not included in standalone export".to_string()
        };

        let spec_json = serde_json::to_string(spec).unwrap_or_else(|_| "{}".to_string());

        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>{}</title>
    <meta name="description" content="{}">
    <meta name="author" content="{}">
    <style>
        body {{ margin: 0; padding: 20px; font-family: Arial, sans-serif; background: {}; }}
        .chart-container {{ width: 100%; height: 600px; border: 1px solid #ddd; }}
        .metadata {{ margin-top: 20px; padding: 10px; background: #f5f5f5; }}
    </style>
</head>
<body>
    <div class="chart-container" id="chart"></div>
    <div class="metadata">
        <strong>Chart Specification:</strong>
        <pre>{}</pre>
    </div>

    <script>
        {}
        // Chart rendering logic would go here
        console.log('Helios chart initialized');
    </script>
</body>
</html>"#,
            config.title.as_deref().unwrap_or("Helios Chart"),
            config.description.as_deref().unwrap_or(""),
            config.author.as_deref().unwrap_or(""),
            config.background_color.as_deref().unwrap_or("#ffffff"),
            spec_json,
            data_section
        )
    }

    fn generate_embedded_html(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
        include_data: bool,
        config: &ExportConfig,
    ) -> String {
        let data_section = if include_data {
            format!("data-chart-data='{}'", self.serialize_dataframe(data))
        } else {
            "".to_string()
        };

        let spec_json = serde_json::to_string(spec).unwrap_or_else(|_| "{}".to_string());

        format!(
            r#"<div class="helios-chart" data-spec='{}' {}>
    <!-- Chart will be rendered here by Helios -->
</div>"#,
            spec_json, data_section
        )
    }

    fn serialize_dataframe(&self, df: &DataFrame) -> String {
        // Mock serialization - in real implementation would properly serialize DataFrame
        let mut rows = Vec::new();
        for i in 0..df.height().min(1000) {
            // Limit to 1000 rows for export
            let mut row = serde_json::Map::new();
            for column in df.get_columns() {
                let value = match column.get(i) {
                    Ok(av) => format!("{}", av),
                    Err(_) => "null".to_string(),
                };
                row.insert(column.name().to_string(), serde_json::Value::String(value));
            }
            rows.push(serde_json::Value::Object(row));
        }
        serde_json::to_string(&rows).unwrap_or_else(|_| "[]".to_string())
    }
}

/// Headless rendering statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadlessStats {
    pub render_count: u64,
    pub total_render_time: std::time::Duration,
    pub average_render_time: std::time::Duration,
    pub browser_initialized: bool,
}

/// Headless rendering service for batch operations
pub struct HeadlessService {
    renderer: HeadlessRenderer,
    max_concurrent_renders: usize,
    active_renders: usize,
}

impl HeadlessService {
    /// Create a new headless service
    pub fn new(config: HeadlessConfig) -> Result<Self, HeadlessError> {
        Ok(Self {
            renderer: HeadlessRenderer::new(config)?,
            max_concurrent_renders: 4,
            active_renders: 0,
        })
    }

    /// Initialize the service
    pub async fn initialize(&mut self) -> Result<(), HeadlessError> {
        self.renderer.initialize().await
    }

    /// Render multiple charts in batch
    pub async fn render_batch(
        &mut self,
        requests: Vec<HeadlessRenderRequest>,
    ) -> Result<Vec<HeadlessRenderResult>, HeadlessError> {
        let mut results = Vec::new();

        for request in requests {
            let result = self.render_single(request).await?;
            results.push(result);
        }

        Ok(results)
    }

    /// Render a single chart
    async fn render_single(
        &mut self,
        request: HeadlessRenderRequest,
    ) -> Result<HeadlessRenderResult, HeadlessError> {
        let start_time = Instant::now();

        let output = match request.format {
            HeadlessFormat::PNG { width, height, dpi } => {
                let data = self
                    .renderer
                    .render_to_png(
                        &request.spec,
                        &request.data,
                        width,
                        height,
                        dpi,
                        &request.config,
                    )
                    .await?;
                HeadlessOutput::Png(data)
            }
            HeadlessFormat::SVG { width, height } => {
                let data = self
                    .renderer
                    .render_to_svg(&request.spec, &request.data, width, height, &request.config)
                    .await?;
                HeadlessOutput::Svg(data)
            }
            HeadlessFormat::PDF { width, height } => {
                let data = self
                    .renderer
                    .render_to_pdf(&request.spec, &request.data, width, height, &request.config)
                    .await?;
                HeadlessOutput::Pdf(data)
            }
            HeadlessFormat::HTML {
                standalone,
                include_data,
            } => {
                let data = self
                    .renderer
                    .render_to_html(
                        &request.spec,
                        &request.data,
                        standalone,
                        include_data,
                        &request.config,
                    )
                    .await?;
                HeadlessOutput::Html(data)
            }
        };

        let duration = start_time.elapsed();

        Ok(HeadlessRenderResult {
            output,
            render_time: duration,
            success: true,
            error: None,
        })
    }

    /// Get service statistics
    pub fn get_stats(&self) -> HeadlessStats {
        self.renderer.get_stats()
    }

    /// Close the service
    pub async fn close(&mut self) -> Result<(), HeadlessError> {
        self.renderer.close().await
    }
}

/// Headless render request
#[derive(Debug, Clone)]
pub struct HeadlessRenderRequest {
    pub spec: ChartSpec,
    pub data: DataFrame,
    pub format: HeadlessFormat,
    pub config: ExportConfig,
}

/// Headless render format
#[derive(Debug, Clone)]
pub enum HeadlessFormat {
    PNG {
        width: u32,
        height: u32,
        dpi: Option<u32>,
    },
    SVG {
        width: Option<u32>,
        height: Option<u32>,
    },
    PDF {
        width: f32,
        height: f32,
    },
    HTML {
        standalone: bool,
        include_data: bool,
    },
}

/// Headless render result
#[derive(Debug)]
pub struct HeadlessRenderResult {
    pub output: HeadlessOutput,
    pub render_time: std::time::Duration,
    pub success: bool,
    pub error: Option<String>,
}

/// Headless output data
#[derive(Debug)]
pub enum HeadlessOutput {
    Png(Vec<u8>),
    Svg(String),
    Pdf(Vec<u8>),
    Html(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chart::{ChartSpec, MarkType};
    use polars::prelude::*;

    #[tokio::test]
    async fn test_headless_renderer_initialization() {
        let config = HeadlessConfig::default();
        let mut renderer = HeadlessRenderer::new(config).unwrap();

        assert!(!renderer.browser_initialized);

        renderer.initialize().await.unwrap();
        assert!(renderer.browser_initialized);

        renderer.close().await.unwrap();
        assert!(!renderer.browser_initialized);
    }

    #[tokio::test]
    async fn test_png_rendering() {
        let config = HeadlessConfig::default();
        let mut renderer = HeadlessRenderer::new(config).unwrap();
        renderer.initialize().await.unwrap();

        let spec = ChartSpec::new();
        let data = df! {
            "x" => [1, 2, 3],
            "y" => [10, 20, 15],
        }
        .unwrap();

        let export_config = ExportConfig::default();

        let png_data = renderer
            .render_to_png(&spec, &data, 800, 600, Some(96), &export_config)
            .await
            .unwrap();

        assert!(!png_data.is_empty());
        assert!(png_data.starts_with(b"\x89PNG"));

        let stats = renderer.get_stats();
        assert_eq!(stats.render_count, 1);
        assert!(stats.total_render_time > std::time::Duration::ZERO);
    }

    #[tokio::test]
    async fn test_svg_rendering() {
        let config = HeadlessConfig::default();
        let mut renderer = HeadlessRenderer::new(config).unwrap();
        renderer.initialize().await.unwrap();

        let spec = ChartSpec::new();
        let data = df! {
            "category" => ["A", "B", "C"],
            "value" => [10, 20, 15],
        }
        .unwrap();

        let export_config = ExportConfig::default();

        let svg_content = renderer
            .render_to_svg(&spec, &data, Some(600), Some(400), &export_config)
            .await
            .unwrap();

        assert!(svg_content.contains("<svg"));
        assert!(svg_content.contains("Helios Chart"));
        assert!(svg_content.contains("Data points: 3"));
    }

    #[tokio::test]
    async fn test_html_rendering() {
        let config = HeadlessConfig::default();
        let mut renderer = HeadlessRenderer::new(config).unwrap();
        renderer.initialize().await.unwrap();

        let spec = ChartSpec::new();
        let data = df! {
            "x" => [1, 2, 3],
            "y" => [5, 15, 10],
        }
        .unwrap();

        let export_config = ExportConfig {
            title: Some("Test Chart".to_string()),
            description: Some("Test description".to_string()),
            ..Default::default()
        };

        let html_content = renderer
            .render_to_html(&spec, &data, true, true, &export_config)
            .await
            .unwrap();

        assert!(html_content.contains("<!DOCTYPE html>"));
        assert!(html_content.contains("Test Chart"));
        assert!(html_content.contains("Test description"));
        assert!(html_content.contains("chartData"));
    }

    #[tokio::test]
    async fn test_headless_service() {
        let config = HeadlessConfig::default();
        let mut service = HeadlessService::new(config).unwrap();
        service.initialize().await.unwrap();

        let spec = ChartSpec::new();
        let data = df! { "x" => [1, 2], "y" => [10, 20] }.unwrap();

        let requests = vec![
            HeadlessRenderRequest {
                spec: spec.clone(),
                data: data.clone(),
                format: HeadlessFormat::PNG {
                    width: 400,
                    height: 300,
                    dpi: Some(96),
                },
                config: ExportConfig::default(),
            },
            HeadlessRenderRequest {
                spec,
                data,
                format: HeadlessFormat::SVG {
                    width: Some(500),
                    height: Some(400),
                },
                config: ExportConfig::default(),
            },
        ];

        let results = service.render_batch(requests).await.unwrap();

        assert_eq!(results.len(), 2);
        assert!(results[0].success);
        assert!(results[1].success);
        assert!(results[0].render_time > std::time::Duration::ZERO);
        assert!(results[1].render_time > std::time::Duration::ZERO);

        match &results[0].output {
            HeadlessOutput::Png(data) => assert!(!data.is_empty()),
            _ => panic!("Expected PNG output"),
        }

        match &results[1].output {
            HeadlessOutput::Svg(data) => assert!(data.contains("<svg")),
            _ => panic!("Expected SVG output"),
        }

        service.close().await.unwrap();
    }
}
