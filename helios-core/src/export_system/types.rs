//! Export system types and configurations

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    HeadlessRenderingError(String),

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
        orientation: PdfOrientation,
    },
    HTML {
        include_interactivity: bool,
        include_data: bool,
        template: Option<String>,
    },
    JSON {
        include_metadata: bool,
        pretty_print: bool,
    },
}

/// PDF unit types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PdfUnit {
    Points,
    Inches,
    Centimeters,
    Millimeters,
}

/// PDF orientation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PdfOrientation {
    Portrait,
    Landscape,
}

/// Export configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    pub format: ExportFormat,
    pub quality: Option<u8>,
    pub background_color: Option<String>,
    pub include_metadata: bool,
    pub custom_styles: Option<HashMap<String, String>>,
    pub template_variables: Option<HashMap<String, String>>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            format: ExportFormat::PNG {
                width: 800,
                height: 600,
                dpi: Some(96),
            },
            quality: Some(90),
            background_color: Some("#ffffff".to_string()),
            include_metadata: true,
            custom_styles: None,
            template_variables: None,
            title: None,
            description: None,
            author: None,
        }
    }
}

/// Export result
#[derive(Debug, Clone)]
pub struct ExportResult {
    pub file_path: String,
    pub file_size: u64,
    pub format: ExportFormat,
    pub metadata: Option<HashMap<String, String>>,
    pub success: bool,
    pub error_message: Option<String>,
}
