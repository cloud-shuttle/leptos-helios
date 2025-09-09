//! Export System
//!
//! This module provides comprehensive export capabilities for Helios visualizations,
//! including static exports (PNG, SVG, PDF), interactive exports, and headless rendering.

use crate::chart::ChartSpec;
use crate::headless_renderer::{HeadlessConfig, HeadlessError, HeadlessRenderer};
use polars::prelude::DataFrame;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Export system errors
#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("Rendering failed: {0}")]
    RenderingFailed(String),

    #[error("File I/O error: {0}")]
    FileError(String),

    #[error("Invalid export format: {0}")]
    InvalidFormat(String),

    #[error("Export configuration error: {0}")]
    ConfigurationError(String),

    #[error("Headless browser error: {0}")]
    HeadlessBrowserError(String),

    #[error("Headless rendering error: {0}")]
    HeadlessRenderingError(#[from] HeadlessError),

    #[error("Template error: {0}")]
    TemplateError(String),
}

/// Export format types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExportFormat {
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
        unit: PdfUnit,
    },
    HTML {
        standalone: bool,
        include_data: bool,
    },
    JSON {
        include_metadata: bool,
    },
    CSV,
    Parquet,
}

/// PDF measurement units
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PdfUnit {
    Points,
    Inches,
    Millimeters,
    Centimeters,
}

/// Export configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    pub format: ExportFormat,
    pub quality: Option<f32>,
    pub background_color: Option<String>,
    pub theme: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            format: ExportFormat::PNG {
                width: 800,
                height: 600,
                dpi: Some(96),
            },
            quality: Some(0.9),
            background_color: Some("#ffffff".to_string()),
            theme: Some("default".to_string()),
            title: None,
            description: None,
            author: None,
            metadata: HashMap::new(),
        }
    }
}

/// Export result information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    pub file_path: String,
    pub format: ExportFormat,
    pub size_bytes: u64,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub export_duration_ms: u64,
    pub metadata: HashMap<String, String>,
}

/// Main export system
pub struct ExportSystem {
    headless_renderer: Option<HeadlessRenderer>,
    template_engine: TemplateEngine,
    file_writer: FileWriter,
}

impl ExportSystem {
    /// Create new export system
    pub fn new() -> Result<Self, ExportError> {
        let headless_config = HeadlessConfig::default();
        Ok(Self {
            headless_renderer: Some(HeadlessRenderer::new(headless_config)?),
            template_engine: TemplateEngine::new(),
            file_writer: FileWriter::new(),
        })
    }

    /// Export chart with data to specified format
    pub async fn export_chart(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
        config: &ExportConfig,
        output_path: &Path,
    ) -> Result<ExportResult, ExportError> {
        let start_time = std::time::Instant::now();

        let result = match &config.format {
            ExportFormat::PNG { width, height, dpi } => {
                self.export_to_png(spec, data, *width, *height, *dpi, config, output_path)
                    .await?
            }
            ExportFormat::SVG { width, height } => {
                self.export_to_svg(spec, data, *width, *height, config, output_path)
                    .await?
            }
            ExportFormat::PDF {
                width,
                height,
                unit,
            } => {
                self.export_to_pdf(spec, data, *width, *height, unit, config, output_path)
                    .await?
            }
            ExportFormat::HTML {
                standalone,
                include_data,
            } => {
                self.export_to_html(spec, data, *standalone, *include_data, config, output_path)
                    .await?
            }
            ExportFormat::JSON { include_metadata } => {
                self.export_to_json(spec, data, *include_metadata, config, output_path)
                    .await?
            }
            ExportFormat::CSV => self.export_to_csv(data, config, output_path).await?,
            ExportFormat::Parquet => self.export_to_parquet(data, config, output_path).await?,
        };

        let duration = start_time.elapsed();
        Ok(ExportResult {
            file_path: output_path.to_string_lossy().to_string(),
            format: config.format.clone(),
            size_bytes: result.size_bytes,
            width: result.width,
            height: result.height,
            export_duration_ms: duration.as_millis() as u64,
            metadata: result.metadata,
        })
    }

    /// Export multiple charts as a batch
    pub async fn export_batch(
        &self,
        charts: &[(ChartSpec, DataFrame, ExportConfig, String)],
    ) -> Result<Vec<ExportResult>, ExportError> {
        let mut results = Vec::new();

        for (spec, data, config, output_path) in charts {
            let result = self
                .export_chart(spec, data, config, Path::new(output_path))
                .await?;
            results.push(result);
        }

        Ok(results)
    }

    /// Create interactive HTML export
    pub async fn create_interactive_export(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
        config: &ExportConfig,
        output_path: &Path,
    ) -> Result<ExportResult, ExportError> {
        let html_content = self
            .template_engine
            .render_interactive_chart(spec, data, config)?;

        self.file_writer
            .write_text_file(output_path, &html_content)
            .await?;

        let size_bytes = html_content.len() as u64;

        Ok(ExportResult {
            file_path: output_path.to_string_lossy().to_string(),
            format: config.format.clone(),
            size_bytes,
            width: None,
            height: None,
            export_duration_ms: 0,
            metadata: HashMap::new(),
        })
    }

    /// Export to PNG format
    async fn export_to_png(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
        width: u32,
        height: u32,
        dpi: Option<u32>,
        config: &ExportConfig,
        output_path: &Path,
    ) -> Result<InternalExportResult, ExportError> {
        let mut renderer = self
            .headless_renderer
            .as_ref()
            .ok_or_else(|| {
                ExportError::HeadlessBrowserError("Headless renderer not available".to_string())
            })?
            .clone();

        // Initialize renderer
        renderer.initialize().await?;

        // Render chart to PNG
        let png_data = renderer
            .render_to_png(spec, data, width, height, dpi, config)
            .await?;

        self.file_writer
            .write_binary_file(output_path, &png_data)
            .await?;

        Ok(InternalExportResult {
            size_bytes: png_data.len() as u64,
            width: Some(width),
            height: Some(height),
            metadata: HashMap::new(),
        })
    }

    /// Export to SVG format
    async fn export_to_svg(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
        width: Option<u32>,
        height: Option<u32>,
        config: &ExportConfig,
        output_path: &Path,
    ) -> Result<InternalExportResult, ExportError> {
        let svg_content = self
            .template_engine
            .render_svg_chart(spec, data, width, height, config)?;

        self.file_writer
            .write_text_file(output_path, &svg_content)
            .await?;

        Ok(InternalExportResult {
            size_bytes: svg_content.len() as u64,
            width,
            height,
            metadata: HashMap::new(),
        })
    }

    /// Export to PDF format
    async fn export_to_pdf(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
        width: f32,
        height: f32,
        unit: &PdfUnit,
        config: &ExportConfig,
        output_path: &Path,
    ) -> Result<InternalExportResult, ExportError> {
        let mut renderer = self
            .headless_renderer
            .as_ref()
            .ok_or_else(|| {
                ExportError::HeadlessBrowserError("Headless renderer not available".to_string())
            })?
            .clone();

        // Initialize renderer
        renderer.initialize().await?;

        // Convert units to points for PDF generation
        let (width_pts, height_pts) = match unit {
            PdfUnit::Points => (width, height),
            PdfUnit::Inches => (width * 72.0, height * 72.0),
            PdfUnit::Millimeters => (width * 2.834, height * 2.834),
            PdfUnit::Centimeters => (width * 28.34, height * 28.34),
        };

        let pdf_data = renderer
            .render_to_pdf(spec, data, width_pts, height_pts, config)
            .await?;

        self.file_writer
            .write_binary_file(output_path, &pdf_data)
            .await?;

        Ok(InternalExportResult {
            size_bytes: pdf_data.len() as u64,
            width: Some(width_pts as u32),
            height: Some(height_pts as u32),
            metadata: HashMap::new(),
        })
    }

    /// Export to HTML format
    async fn export_to_html(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
        standalone: bool,
        include_data: bool,
        config: &ExportConfig,
        output_path: &Path,
    ) -> Result<InternalExportResult, ExportError> {
        let html_content = if standalone {
            self.template_engine
                .render_standalone_html(spec, data, include_data, config)?
        } else {
            self.template_engine
                .render_embedded_html(spec, data, include_data, config)?
        };

        self.file_writer
            .write_text_file(output_path, &html_content)
            .await?;

        Ok(InternalExportResult {
            size_bytes: html_content.len() as u64,
            width: None,
            height: None,
            metadata: HashMap::new(),
        })
    }

    /// Export to JSON format
    async fn export_to_json(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
        include_metadata: bool,
        config: &ExportConfig,
        output_path: &Path,
    ) -> Result<InternalExportResult, ExportError> {
        let json_content = if include_metadata {
            serde_json::to_string_pretty(&ExportBundle {
                spec: spec.clone(),
                data: serialize_dataframe(data)?,
                config: config.clone(),
                metadata: config.metadata.clone(),
            })
            .map_err(|e| ExportError::FileError(e.to_string()))?
        } else {
            serde_json::to_string_pretty(&ChartExport {
                spec: spec.clone(),
                data: serialize_dataframe(data)?,
            })
            .map_err(|e| ExportError::FileError(e.to_string()))?
        };

        self.file_writer
            .write_text_file(output_path, &json_content)
            .await?;

        Ok(InternalExportResult {
            size_bytes: json_content.len() as u64,
            width: None,
            height: None,
            metadata: HashMap::new(),
        })
    }

    /// Export to CSV format
    async fn export_to_csv(
        &self,
        data: &DataFrame,
        _config: &ExportConfig,
        output_path: &Path,
    ) -> Result<InternalExportResult, ExportError> {
        let csv_content = dataframe_to_csv(data)?;

        self.file_writer
            .write_text_file(output_path, &csv_content)
            .await?;

        Ok(InternalExportResult {
            size_bytes: csv_content.len() as u64,
            width: None,
            height: None,
            metadata: HashMap::new(),
        })
    }

    /// Export to Parquet format
    async fn export_to_parquet(
        &self,
        data: &DataFrame,
        _config: &ExportConfig,
        output_path: &Path,
    ) -> Result<InternalExportResult, ExportError> {
        // Mock implementation - would use actual Polars parquet writer
        let parquet_data = mock_parquet_serialize(data)?;

        self.file_writer
            .write_binary_file(output_path, &parquet_data)
            .await?;

        Ok(InternalExportResult {
            size_bytes: parquet_data.len() as u64,
            width: None,
            height: None,
            metadata: HashMap::new(),
        })
    }
}

/// Internal export result
struct InternalExportResult {
    size_bytes: u64,
    width: Option<u32>,
    height: Option<u32>,
    metadata: HashMap<String, String>,
}

/// Template rendering engine
struct TemplateEngine;

impl TemplateEngine {
    fn new() -> Self {
        Self
    }

    fn render_interactive_chart(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
        config: &ExportConfig,
    ) -> Result<String, ExportError> {
        let chart_json =
            serde_json::to_string(spec).map_err(|e| ExportError::TemplateError(e.to_string()))?;
        let data_json = serialize_dataframe(data)?;
        let config_json =
            serde_json::to_string(config).map_err(|e| ExportError::TemplateError(e.to_string()))?;

        Ok(format!(
            r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>{}</title>
    <script src="https://unpkg.com/helios-charts@latest/dist/helios.js"></script>
    <style>
        body {{ margin: 0; padding: 20px; font-family: Arial, sans-serif; }}
        #chart {{ width: 100%; height: 600px; }}
        .title {{ font-size: 24px; margin-bottom: 20px; }}
        .description {{ color: #666; margin-bottom: 20px; }}
    </style>
</head>
<body>
    <div class="title">{}</div>
    <div class="description">{}</div>
    <div id="chart"></div>

    <script>
        const spec = {};
        const data = {};
        const config = {};

        const chart = new Helios.Chart('#chart', spec, data, config);
        chart.render();
    </script>
</body>
</html>
        "#,
            config.title.as_deref().unwrap_or("Helios Chart"),
            config.title.as_deref().unwrap_or("Untitled Chart"),
            config.description.as_deref().unwrap_or(""),
            chart_json,
            data_json,
            config_json
        ))
    }

    fn render_svg_chart(
        &self,
        _spec: &ChartSpec,
        _data: &DataFrame,
        width: Option<u32>,
        height: Option<u32>,
        config: &ExportConfig,
    ) -> Result<String, ExportError> {
        let w = width.unwrap_or(800);
        let h = height.unwrap_or(600);
        let bg_color = config.background_color.as_deref().unwrap_or("#ffffff");

        let text_color = "#333333";
        Ok(format!(
            r#"
<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">
    <rect width="100%" height="100%" fill="{}"/>
    <text x="50%" y="50%" text-anchor="middle" font-family="Arial" font-size="16" fill="{}">
        Helios Chart (SVG)
    </text>
    <!-- Chart content would be rendered here -->
</svg>
        "#,
            w, h, bg_color, text_color
        ))
    }

    fn render_standalone_html(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
        include_data: bool,
        config: &ExportConfig,
    ) -> Result<String, ExportError> {
        let data_section = if include_data {
            format!("const chartData = {};", serialize_dataframe(data)?)
        } else {
            "// Data not included in standalone export".to_string()
        };

        let spec_json =
            serde_json::to_string(spec).map_err(|e| ExportError::TemplateError(e.to_string()))?;

        Ok(format!(
            r#"
<!DOCTYPE html>
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
</html>
        "#,
            config.title.as_deref().unwrap_or("Helios Chart"),
            config.description.as_deref().unwrap_or(""),
            config.author.as_deref().unwrap_or(""),
            config.background_color.as_deref().unwrap_or("#ffffff"),
            spec_json,
            data_section
        ))
    }

    fn render_embedded_html(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
        include_data: bool,
        _config: &ExportConfig,
    ) -> Result<String, ExportError> {
        let data_section = if include_data {
            format!("data-chart-data='{}'", serialize_dataframe(data)?)
        } else {
            "".to_string()
        };

        let spec_json =
            serde_json::to_string(spec).map_err(|e| ExportError::TemplateError(e.to_string()))?;

        Ok(format!(
            r#"
<div class="helios-chart" data-spec='{}' {}>
    <!-- Chart will be rendered here by Helios -->
</div>
        "#,
            spec_json, data_section
        ))
    }
}

/// File writing utility
struct FileWriter;

impl FileWriter {
    fn new() -> Self {
        Self
    }

    async fn write_text_file(&self, path: &Path, content: &str) -> Result<(), ExportError> {
        tokio::fs::write(path, content)
            .await
            .map_err(|e| ExportError::FileError(e.to_string()))?;
        Ok(())
    }

    async fn write_binary_file(&self, path: &Path, data: &[u8]) -> Result<(), ExportError> {
        tokio::fs::write(path, data)
            .await
            .map_err(|e| ExportError::FileError(e.to_string()))?;
        Ok(())
    }
}

/// Export bundle for JSON with metadata
#[derive(Serialize, Deserialize)]
struct ExportBundle {
    spec: ChartSpec,
    data: serde_json::Value,
    config: ExportConfig,
    metadata: HashMap<String, String>,
}

/// Chart export without metadata
#[derive(Serialize, Deserialize)]
struct ChartExport {
    spec: ChartSpec,
    data: serde_json::Value,
}

/// Helper functions

fn serialize_dataframe(df: &DataFrame) -> Result<serde_json::Value, ExportError> {
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
    Ok(serde_json::Value::Array(rows))
}

fn dataframe_to_csv(df: &DataFrame) -> Result<String, ExportError> {
    let mut csv_content = String::new();

    // Header
    let column_names: Vec<String> = df
        .get_column_names()
        .iter()
        .map(|name| name.to_string())
        .collect();
    csv_content.push_str(&column_names.join(","));
    csv_content.push('\n');

    // Rows
    for i in 0..df.height() {
        let mut row_values = Vec::new();
        for column in df.get_columns() {
            let value = match column.get(i) {
                Ok(av) => format!("{}", av),
                Err(_) => "".to_string(),
            };
            row_values.push(value);
        }
        csv_content.push_str(&row_values.join(","));
        csv_content.push('\n');
    }

    Ok(csv_content)
}

fn mock_parquet_serialize(df: &DataFrame) -> Result<Vec<u8>, ExportError> {
    // Mock parquet serialization - would use actual Polars parquet writer
    let mut parquet_data = Vec::new();
    parquet_data.extend_from_slice(b"PAR1"); // Parquet magic number
    parquet_data.extend_from_slice(&(df.height() as u32).to_le_bytes());
    parquet_data.extend_from_slice(&(df.width() as u32).to_le_bytes());
    Ok(parquet_data)
}

/// Export system builder for configuration
pub struct ExportSystemBuilder {
    enable_headless: bool,
    template_path: Option<String>,
    default_config: ExportConfig,
}

impl ExportSystemBuilder {
    pub fn new() -> Self {
        Self {
            enable_headless: true,
            template_path: None,
            default_config: ExportConfig::default(),
        }
    }

    pub fn with_headless(mut self, enable: bool) -> Self {
        self.enable_headless = enable;
        self
    }

    pub fn with_template_path(mut self, path: String) -> Self {
        self.template_path = Some(path);
        self
    }

    pub fn with_default_config(mut self, config: ExportConfig) -> Self {
        self.default_config = config;
        self
    }

    pub fn build(self) -> Result<ExportSystem, ExportError> {
        Ok(ExportSystem {
            headless_renderer: if self.enable_headless {
                let headless_config = HeadlessConfig::default();
                Some(HeadlessRenderer::new(headless_config)?)
            } else {
                None
            },
            template_engine: TemplateEngine::new(),
            file_writer: FileWriter::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chart::{ChartSpec, MarkType};
    use polars::prelude::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_png_export() {
        let export_system = ExportSystem::new().unwrap();
        let spec = ChartSpecBuilder::new()
            .mark(MarkType::Bar {
                width: None,
                corner_radius: None,
            })
            .build()
            .unwrap_or_else(|_| ChartSpec::new());
        let data = df! {
            "category" => ["A", "B", "C"],
            "value" => [10, 20, 15],
        }
        .unwrap();

        let config = ExportConfig {
            format: ExportFormat::PNG {
                width: 800,
                height: 600,
                dpi: Some(96),
            },
            ..Default::default()
        };

        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("test_chart.png");

        let result = export_system
            .export_chart(&spec, &data, &config, &output_path)
            .await
            .unwrap();

        assert_eq!(result.format, config.format);
        assert!(result.size_bytes > 0);
        assert_eq!(result.width, Some(800));
        assert_eq!(result.height, Some(600));
        assert!(std::path::Path::new(&result.file_path).exists());
    }

    #[tokio::test]
    async fn test_svg_export() {
        let export_system = ExportSystem::new().unwrap();
        let spec = ChartSpecBuilder::new()
            .mark(MarkType::Line {
                interpolate: None,
                stroke_width: None,
                stroke_dash: None,
            })
            .build()
            .unwrap_or_else(|_| ChartSpec::new());
        let data = df! {
            "x" => [1, 2, 3, 4],
            "y" => [10, 20, 15, 25],
        }
        .unwrap();

        let config = ExportConfig {
            format: ExportFormat::SVG {
                width: Some(600),
                height: Some(400),
            },
            background_color: Some("#f0f0f0".to_string()),
            ..Default::default()
        };

        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("test_chart.svg");

        let result = export_system
            .export_chart(&spec, &data, &config, &output_path)
            .await
            .unwrap();

        assert_eq!(result.format, config.format);
        assert!(result.size_bytes > 0);
        assert_eq!(result.width, Some(600));
        assert_eq!(result.height, Some(400));
    }

    #[tokio::test]
    async fn test_pdf_export() {
        let export_system = ExportSystem::new().unwrap();
        let spec = ChartSpecBuilder::new()
            .mark(MarkType::Point {
                size: None,
                shape: None,
                opacity: None,
            })
            .build()
            .unwrap_or_else(|_| ChartSpec::new());
        let data = df! {
            "x" => [1.0, 2.0, 3.0],
            "y" => [10.0, 20.0, 15.0],
        }
        .unwrap();

        let config = ExportConfig {
            format: ExportFormat::PDF {
                width: 8.5,
                height: 11.0,
                unit: PdfUnit::Inches,
            },
            title: Some("Test Chart".to_string()),
            ..Default::default()
        };

        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("test_chart.pdf");

        let result = export_system
            .export_chart(&spec, &data, &config, &output_path)
            .await
            .unwrap();

        assert_eq!(result.format, config.format);
        assert!(result.size_bytes > 0);
        assert!(result.width.is_some());
        assert!(result.height.is_some());
    }

    #[tokio::test]
    async fn test_html_export() {
        let export_system = ExportSystem::new().unwrap();
        let spec = ChartSpecBuilder::new()
            .mark(MarkType::Area {
                interpolate: None,
                opacity: None,
            })
            .build()
            .unwrap_or_else(|_| ChartSpec::new());
        let data = df! {
            "date" => ["2023-01-01", "2023-01-02", "2023-01-03"],
            "value" => [100, 110, 105],
        }
        .unwrap();

        let config = ExportConfig {
            format: ExportFormat::HTML {
                standalone: true,
                include_data: true,
            },
            title: Some("Interactive Chart".to_string()),
            description: Some("Test description".to_string()),
            ..Default::default()
        };

        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("test_chart.html");

        let result = export_system
            .export_chart(&spec, &data, &config, &output_path)
            .await
            .unwrap();

        assert_eq!(result.format, config.format);
        assert!(result.size_bytes > 0);

        // Verify HTML content
        let html_content = tokio::fs::read_to_string(&output_path).await.unwrap();
        assert!(html_content.contains("Interactive Chart"));
        assert!(html_content.contains("Test description"));
        assert!(html_content.contains("<!DOCTYPE html>"));
    }

    #[tokio::test]
    async fn test_json_export() {
        let export_system = ExportSystem::new().unwrap();
        let spec = ChartSpecBuilder::new()
            .mark(MarkType::Bar {
                width: None,
                corner_radius: None,
            })
            .build()
            .unwrap_or_else(|_| ChartSpec::new());
        let data = df! {
            "category" => ["A", "B", "C"],
            "value" => [10, 20, 15],
        }
        .unwrap();

        let config = ExportConfig {
            format: ExportFormat::JSON {
                include_metadata: true,
            },
            title: Some("Test Chart".to_string()),
            ..Default::default()
        };

        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("test_chart.json");

        let result = export_system
            .export_chart(&spec, &data, &config, &output_path)
            .await
            .unwrap();

        assert_eq!(result.format, config.format);
        assert!(result.size_bytes > 0);

        // Verify JSON content
        let json_content = tokio::fs::read_to_string(&output_path).await.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_content).unwrap();
        assert!(parsed.get("spec").is_some());
        assert!(parsed.get("data").is_some());
        assert!(parsed.get("config").is_some());
        assert!(parsed.get("metadata").is_some());
    }

    #[tokio::test]
    async fn test_csv_export() {
        let export_system = ExportSystem::new().unwrap();
        let spec = ChartSpecBuilder::new()
            .mark(MarkType::Bar {
                width: None,
                corner_radius: None,
            })
            .build()
            .unwrap_or_else(|_| ChartSpec::new());
        let data = df! {
            "name" => ["Alice", "Bob", "Charlie"],
            "age" => [30, 25, 35],
            "salary" => [50000.0, 45000.0, 60000.0],
        }
        .unwrap();

        let config = ExportConfig {
            format: ExportFormat::CSV,
            ..Default::default()
        };

        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("test_data.csv");

        let result = export_system
            .export_chart(&spec, &data, &config, &output_path)
            .await
            .unwrap();

        assert_eq!(result.format, config.format);
        assert!(result.size_bytes > 0);

        // Verify CSV content
        let csv_content = tokio::fs::read_to_string(&output_path).await.unwrap();
        assert!(csv_content.contains("name,age,salary"));
        assert!(csv_content.contains("Alice"));
        assert!(csv_content.contains("30"));
    }

    #[tokio::test]
    async fn test_batch_export() {
        let export_system = ExportSystem::new().unwrap();
        let temp_dir = tempdir().unwrap();

        let charts = vec![
            (
                ChartSpecBuilder::new()
                    .mark(MarkType::Bar {
                        width: None,
                        corner_radius: None,
                    })
                    .build()
                    .unwrap_or_else(|_| ChartSpec::new()),
                df! { "x" => [1, 2, 3], "y" => [10, 20, 15] }.unwrap(),
                ExportConfig {
                    format: ExportFormat::PNG {
                        width: 400,
                        height: 300,
                        dpi: Some(96),
                    },
                    ..Default::default()
                },
                temp_dir
                    .path()
                    .join("chart1.png")
                    .to_string_lossy()
                    .to_string(),
            ),
            (
                ChartSpecBuilder::new()
                    .mark(MarkType::Line {
                        interpolate: None,
                        stroke_width: None,
                        stroke_dash: None,
                    })
                    .build()
                    .unwrap_or_else(|_| ChartSpec::new()),
                df! { "x" => [1, 2, 3], "y" => [5, 15, 10] }.unwrap(),
                ExportConfig {
                    format: ExportFormat::SVG {
                        width: Some(500),
                        height: Some(400),
                    },
                    ..Default::default()
                },
                temp_dir
                    .path()
                    .join("chart2.svg")
                    .to_string_lossy()
                    .to_string(),
            ),
        ];

        let results = export_system.export_batch(&charts).await.unwrap();

        assert_eq!(results.len(), 2);
        assert!(results[0].size_bytes > 0);
        assert!(results[1].size_bytes > 0);
        assert!(std::path::Path::new(&results[0].file_path).exists());
        assert!(std::path::Path::new(&results[1].file_path).exists());
    }

    #[tokio::test]
    async fn test_export_system_builder() {
        let config = ExportConfig {
            format: ExportFormat::PNG {
                width: 1200,
                height: 800,
                dpi: Some(150),
            },
            quality: Some(0.95),
            background_color: Some("#f8f8f8".to_string()),
            ..Default::default()
        };

        let export_system = ExportSystemBuilder::new()
            .with_headless(true)
            .with_default_config(config)
            .build()
            .unwrap();

        let spec = ChartSpecBuilder::new()
            .mark(MarkType::Bar {
                width: None,
                corner_radius: None,
            })
            .build()
            .unwrap_or_else(|_| ChartSpec::new());
        let data = df! { "x" => [1, 2], "y" => [10, 20] }.unwrap();
        let config = ExportConfig::default();

        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("builder_test.png");

        let result = export_system
            .export_chart(&spec, &data, &config, &output_path)
            .await
            .unwrap();
        assert!(result.size_bytes > 0);
    }

    #[tokio::test]
    async fn test_interactive_export() {
        let export_system = ExportSystem::new().unwrap();
        let spec = ChartSpecBuilder::new()
            .mark(MarkType::Point {
                size: None,
                shape: None,
                opacity: None,
            })
            .build()
            .unwrap_or_else(|_| ChartSpec::new());
        let data = df! {
            "x" => [1.0, 2.0, 3.0, 4.0],
            "y" => [10.0, 20.0, 15.0, 25.0],
        }
        .unwrap();

        let config = ExportConfig {
            format: ExportFormat::HTML {
                standalone: true,
                include_data: true,
            },
            title: Some("Interactive Scatter Plot".to_string()),
            description: Some("Hover and click to interact".to_string()),
            ..Default::default()
        };

        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("interactive_chart.html");

        let result = export_system
            .create_interactive_export(&spec, &data, &config, &output_path)
            .await
            .unwrap();

        assert!(result.size_bytes > 0);

        let html_content = tokio::fs::read_to_string(&output_path).await.unwrap();
        assert!(html_content.contains("Interactive Scatter Plot"));
        assert!(html_content.contains("Hover and click to interact"));
        assert!(html_content.contains("helios-charts"));
    }
}
