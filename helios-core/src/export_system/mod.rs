//! Export System
//!
//! This module provides comprehensive export capabilities for Helios visualizations,
//! including static exports (PNG, SVG, PDF), interactive exports, and headless rendering.

mod types;
mod template_engine;
mod file_writer;

// Re-export public types
pub use types::*;
use template_engine::TemplateEngine;
use file_writer::FileWriter;

use crate::chart::ChartSpec;
use crate::headless_renderer::{HeadlessConfig, HeadlessError, HeadlessRenderer};
use polars::prelude::DataFrame;
use std::collections::HashMap;
use std::path::Path;

/// Main export system
pub struct ExportSystem {
    template_engine: TemplateEngine,
    file_writer: FileWriter,
    headless_renderer: Option<HeadlessRenderer>,
}

impl ExportSystem {
    /// Create a new export system
    pub fn new(output_dir: &str) -> Self {
        Self {
            template_engine: TemplateEngine::new(),
            file_writer: FileWriter::new(output_dir),
            headless_renderer: None,
        }
    }

    /// Create export system with headless rendering
    pub fn with_headless_renderer(output_dir: &str, config: HeadlessConfig) -> Result<Self, ExportError> {
        let headless_renderer = HeadlessRenderer::new(config)
            .map_err(|e| ExportError::HeadlessRenderingError(format!("Failed to create headless renderer: {:?}", e)))?;

        Ok(Self {
            template_engine: TemplateEngine::new(),
            file_writer: FileWriter::new(output_dir),
            headless_renderer: Some(headless_renderer),
        })
    }

    /// Export chart to specified format
    pub async fn export_chart(
        &mut self,
        spec: &ChartSpec,
        data: &DataFrame,
        config: &ExportConfig,
        filename: &str,
    ) -> Result<ExportResult, ExportError> {
        match &config.format {
            ExportFormat::PNG { .. } => {
                self.export_png(spec, data, config, filename).await
            }
            ExportFormat::SVG { .. } => {
                self.export_svg(spec, data, config, filename).await
            }
            ExportFormat::PDF { .. } => {
                self.export_pdf(spec, data, config, filename).await
            }
            ExportFormat::HTML { .. } => {
                self.export_html(spec, data, config, filename).await
            }
            ExportFormat::JSON { .. } => {
                self.export_json(spec, data, config, filename).await
            }
        }
    }

    /// Export as PNG
    async fn export_png(
        &mut self,
        spec: &ChartSpec,
        data: &DataFrame,
        config: &ExportConfig,
        filename: &str,
    ) -> Result<ExportResult, ExportError> {
        if let Some(renderer) = &mut self.headless_renderer {
            // Use headless renderer for PNG export
            let (width, height, dpi) = match &config.format {
                ExportFormat::PNG { width, height, dpi } => (*width, *height, *dpi),
                _ => (800, 600, Some(96)),
            };
            let image_data = renderer.render_to_png(spec, data, width, height, dpi, config)
                .await
                .map_err(|e| ExportError::RenderingFailed(format!("Headless rendering failed: {:?}", e)))?;

            self.file_writer.write_file(filename, &image_data)
        } else {
            // Fallback to basic PNG export
            let png_data = self.create_basic_png(spec, config)?;
            self.file_writer.write_file(filename, &png_data)
        }
    }

    /// Export as SVG
    async fn export_svg(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
        config: &ExportConfig,
        filename: &str,
    ) -> Result<ExportResult, ExportError> {
        let svg_content = self.generate_svg(spec, data, config)?;
        self.file_writer.write_text_file(filename, &svg_content)
    }

    /// Export as PDF
    async fn export_pdf(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
        config: &ExportConfig,
        filename: &str,
    ) -> Result<ExportResult, ExportError> {
        // For now, create a simple PDF placeholder
        let pdf_content = self.create_basic_pdf(spec, config)?;
        self.file_writer.write_file(filename, &pdf_content)
    }

    /// Export as HTML
    async fn export_html(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
        config: &ExportConfig,
        filename: &str,
    ) -> Result<ExportResult, ExportError> {
        let chart_html = self.generate_chart_html(spec, data, config)?;
        let html_content = self.template_engine.render_chart_html(&chart_html, None)?;
        self.file_writer.write_text_file(filename, &html_content)
    }

    /// Export as JSON
    async fn export_json(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
        config: &ExportConfig,
        filename: &str,
    ) -> Result<ExportResult, ExportError> {
        let json_content = self.generate_json(spec, data, config)?;
        self.file_writer.write_text_file(filename, &json_content)
    }

    /// Generate SVG content
    fn generate_svg(&self, spec: &ChartSpec, data: &DataFrame, config: &ExportConfig) -> Result<String, ExportError> {
        // Basic SVG generation
        let width = match &config.format {
            ExportFormat::SVG { width, .. } => width.unwrap_or(800),
            _ => 800,
        };
        let height = match &config.format {
            ExportFormat::SVG { height, .. } => height.unwrap_or(600),
            _ => 600,
        };

        let svg = format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">
                <rect width="100%" height="100%" fill="{}"/>
                <text x="50%" y="50%" text-anchor="middle" font-family="Arial" font-size="16">
                    Chart: {:?}
                </text>
            </svg>"#,
            width,
            height,
            config.background_color.as_deref().unwrap_or("#ffffff"),
            spec.mark
        );

        Ok(svg)
    }

    /// Generate chart HTML
    fn generate_chart_html(&self, spec: &ChartSpec, data: &DataFrame, config: &ExportConfig) -> Result<String, ExportError> {
        // Basic HTML generation
        let html = format!(
            r#"<div class="chart" data-spec="{:?}">
                <h3>Chart Visualization</h3>
                <div class="chart-content">
                    <p>Chart type: {:?}</p>
                    <p>Data points: {}</p>
                </div>
            </div>"#,
            spec.mark,
            spec.mark,
            data.height()
        );

        Ok(html)
    }

    /// Generate JSON content
    fn generate_json(&self, spec: &ChartSpec, data: &DataFrame, config: &ExportConfig) -> Result<String, ExportError> {
        use serde_json;
        
        let json_data = serde_json::json!({
            "chart_spec": spec,
            "data_summary": {
                "rows": data.height(),
                "columns": data.width(),
                "column_names": data.get_column_names()
            },
            "export_config": config,
            "exported_at": chrono::Utc::now().to_rfc3339()
        });

        if matches!(config.format, ExportFormat::JSON { pretty_print: true, .. }) {
            serde_json::to_string_pretty(&json_data)
                .map_err(|e| ExportError::ConfigurationError(format!("JSON serialization failed: {}", e)))
        } else {
            serde_json::to_string(&json_data)
                .map_err(|e| ExportError::ConfigurationError(format!("JSON serialization failed: {}", e)))
        }
    }

    /// Create basic PNG (placeholder)
    fn create_basic_png(&self, spec: &ChartSpec, config: &ExportConfig) -> Result<Vec<u8>, ExportError> {
        // This is a placeholder - in a real implementation, you would generate actual PNG data
        // For now, return a minimal PNG header
        Ok(vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
            // ... more PNG data would go here
        ])
    }

    /// Create basic PDF (placeholder)
    fn create_basic_pdf(&self, spec: &ChartSpec, config: &ExportConfig) -> Result<Vec<u8>, ExportError> {
        // This is a placeholder - in a real implementation, you would generate actual PDF data
        let pdf_content = format!(
            "%PDF-1.4\n1 0 obj\n<<\n/Type /Catalog\n/Pages 2 0 R\n>>\nendobj\n2 0 obj\n<<\n/Type /Pages\n/Kids [3 0 R]\n/Count 1\n>>\nendobj\n3 0 obj\n<<\n/Type /Page\n/Parent 2 0 R\n/MediaBox [0 0 612 792]\n/Contents 4 0 R\n>>\nendobj\n4 0 obj\n<<\n/Length 44\n>>\nstream\nBT\n/F1 12 Tf\n100 700 Td\n(Chart: {:?}) Tj\nET\nendstream\nendobj\nxref\n0 5\n0000000000 65535 f \n0000000009 00000 n \n0000000058 00000 n \n0000000115 00000 n \n0000000204 00000 n \ntrailer\n<<\n/Size 5\n/Root 1 0 R\n>>\nstartxref\n297\n%%EOF",
            spec.mark
        );
        Ok(pdf_content.into_bytes())
    }

    /// Set template variables
    pub fn set_template_variables(&mut self, variables: HashMap<String, String>) {
        self.template_engine.set_variables(variables);
    }

    /// Add custom template
    pub fn add_template(&mut self, name: &str, content: &str) {
        self.template_engine.add_template(name, content);
    }
}

/// Export system builder
pub struct ExportSystemBuilder {
    output_dir: String,
    headless_config: Option<HeadlessConfig>,
    template_variables: HashMap<String, String>,
}

impl ExportSystemBuilder {
    /// Create a new builder
    pub fn new(output_dir: &str) -> Self {
        Self {
            output_dir: output_dir.to_string(),
            headless_config: None,
            template_variables: HashMap::new(),
        }
    }

    /// Enable headless rendering
    pub fn with_headless_rendering(mut self, config: HeadlessConfig) -> Self {
        self.headless_config = Some(config);
        self
    }

    /// Set template variables
    pub fn with_template_variables(mut self, variables: HashMap<String, String>) -> Self {
        self.template_variables = variables;
        self
    }

    /// Build the export system
    pub fn build(self) -> Result<ExportSystem, ExportError> {
        let mut export_system = if let Some(headless_config) = self.headless_config {
            ExportSystem::with_headless_renderer(&self.output_dir, headless_config)?
        } else {
            ExportSystem::new(&self.output_dir)
        };

        if !self.template_variables.is_empty() {
            export_system.set_template_variables(self.template_variables);
        }

        Ok(export_system)
    }
}
