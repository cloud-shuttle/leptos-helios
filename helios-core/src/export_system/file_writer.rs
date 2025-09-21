//! File writer for export system

use super::types::{ExportError, ExportResult, ExportFormat};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// File writer for export operations
pub struct FileWriter {
    output_dir: String,
}

impl FileWriter {
    /// Create a new file writer
    pub fn new(output_dir: &str) -> Self {
        Self {
            output_dir: output_dir.to_string(),
        }
    }

    /// Write content to file
    pub fn write_file(&self, filename: &str, content: &[u8]) -> Result<ExportResult, ExportError> {
        let file_path = Path::new(&self.output_dir).join(filename);
        
        // Ensure directory exists
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| ExportError::FileError(format!("Failed to create directory: {}", e)))?;
        }

        // Write file
        fs::write(&file_path, content)
            .map_err(|e| ExportError::FileError(format!("Failed to write file: {}", e)))?;

        // Get file size
        let metadata = fs::metadata(&file_path)
            .map_err(|e| ExportError::FileError(format!("Failed to get file metadata: {}", e)))?;

        Ok(ExportResult {
            file_path: file_path.to_string_lossy().to_string(),
            file_size: metadata.len(),
            format: self.detect_format(filename),
            metadata: Some(self.create_metadata(&file_path, metadata.len())),
            success: true,
            error_message: None,
        })
    }

    /// Write text content to file
    pub fn write_text_file(&self, filename: &str, content: &str) -> Result<ExportResult, ExportError> {
        self.write_file(filename, content.as_bytes())
    }

    /// Detect format from filename
    fn detect_format(&self, filename: &str) -> ExportFormat {
        let extension = Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            "png" => ExportFormat::PNG {
                width: 800,
                height: 600,
                dpi: Some(96),
            },
            "svg" => ExportFormat::SVG {
                width: Some(800),
                height: Some(600),
            },
            "pdf" => ExportFormat::PDF {
                width: 8.5,
                height: 11.0,
                unit: super::types::PdfUnit::Inches,
                orientation: super::types::PdfOrientation::Portrait,
            },
            "html" => ExportFormat::HTML {
                include_interactivity: true,
                include_data: false,
                template: None,
            },
            "json" => ExportFormat::JSON {
                include_metadata: true,
                pretty_print: true,
            },
            _ => ExportFormat::PNG {
                width: 800,
                height: 600,
                dpi: Some(96),
            },
        }
    }

    /// Create metadata for export result
    fn create_metadata(&self, file_path: &Path, file_size: u64) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("file_size".to_string(), file_size.to_string());
        metadata.insert("created_at".to_string(), chrono::Utc::now().to_rfc3339());
        metadata.insert("format".to_string(), 
            file_path.extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("unknown")
                .to_string()
        );
        metadata
    }

    /// Ensure output directory exists
    pub fn ensure_output_dir(&self) -> Result<(), ExportError> {
        fs::create_dir_all(&self.output_dir)
            .map_err(|e| ExportError::FileError(format!("Failed to create output directory: {}", e)))
    }

    /// Get output directory
    pub fn output_dir(&self) -> &str {
        &self.output_dir
    }
}
