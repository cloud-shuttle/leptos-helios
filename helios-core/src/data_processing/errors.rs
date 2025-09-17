//! Data processing error types and handling
//!
//! This module provides comprehensive error handling for data processing operations,
//! including Polars integration, data validation, and processing errors.

use polars::prelude::*;

/// Data processing error types
#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("Polars error: {0}")]
    Polars(#[from] PolarsError),

    #[error("Data format error: {0}")]
    Format(String),

    #[error("Data validation error: {0}")]
    Validation(String),

    #[error("Data source error: {0}")]
    Source(String),

    #[error("Processing error: {0}")]
    Processing(String),

    #[error("Expression parsing error: {0}")]
    Expression(String),

    #[error("Column not found: {0}")]
    ColumnNotFound(String),

    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },

    #[error("Memory allocation error: {0}")]
    MemoryAllocation(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("IO error: {0}")]
    Io(String),

    #[error("Timeout error: {0}")]
    Timeout(String),

    #[error("Concurrency error: {0}")]
    Concurrency(String),
}

impl DataError {
    /// Create a new format error
    pub fn format(message: impl Into<String>) -> Self {
        Self::Format(message.into())
    }

    /// Create a new validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }

    /// Create a new source error
    pub fn source(message: impl Into<String>) -> Self {
        Self::Source(message.into())
    }

    /// Create a new processing error
    pub fn processing(message: impl Into<String>) -> Self {
        Self::Processing(message.into())
    }

    /// Create a new expression parsing error
    pub fn expression(message: impl Into<String>) -> Self {
        Self::Expression(message.into())
    }

    /// Create a new column not found error
    pub fn column_not_found(column_name: impl Into<String>) -> Self {
        Self::ColumnNotFound(column_name.into())
    }

    /// Create a new type mismatch error
    pub fn type_mismatch(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::TypeMismatch {
            expected: expected.into(),
            actual: actual.into(),
        }
    }

    /// Create a new memory allocation error
    pub fn memory_allocation(message: impl Into<String>) -> Self {
        Self::MemoryAllocation(message.into())
    }

    /// Create a new configuration error
    pub fn configuration(message: impl Into<String>) -> Self {
        Self::Configuration(message.into())
    }

    /// Create a new serialization error
    pub fn serialization(message: impl Into<String>) -> Self {
        Self::Serialization(message.into())
    }

    /// Create a new deserialization error
    pub fn deserialization(message: impl Into<String>) -> Self {
        Self::Deserialization(message.into())
    }

    /// Create a new IO error
    pub fn io(message: impl Into<String>) -> Self {
        Self::Io(message.into())
    }

    /// Create a new timeout error
    pub fn timeout(message: impl Into<String>) -> Self {
        Self::Timeout(message.into())
    }

    /// Create a new concurrency error
    pub fn concurrency(message: impl Into<String>) -> Self {
        Self::Concurrency(message.into())
    }

    /// Check if this is a recoverable error
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            DataError::Timeout(_) | DataError::MemoryAllocation(_) | DataError::Concurrency(_)
        )
    }

    /// Check if this is a data validation error
    pub fn is_validation_error(&self) -> bool {
        matches!(
            self,
            DataError::Validation(_) | DataError::TypeMismatch { .. }
        )
    }

    /// Check if this is a configuration error
    pub fn is_configuration_error(&self) -> bool {
        matches!(self, DataError::Configuration(_))
    }

    /// Check if this is a processing error
    pub fn is_processing_error(&self) -> bool {
        matches!(
            self,
            DataError::Processing(_) | DataError::Expression(_) | DataError::ColumnNotFound(_)
        )
    }

    /// Get error severity level
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            DataError::Polars(_) => ErrorSeverity::High,
            DataError::MemoryAllocation(_) => ErrorSeverity::Critical,
            DataError::TypeMismatch { .. } => ErrorSeverity::High,
            DataError::ColumnNotFound(_) => ErrorSeverity::Medium,
            DataError::Validation(_) => ErrorSeverity::Medium,
            DataError::Format(_) => ErrorSeverity::Low,
            DataError::Configuration(_) => ErrorSeverity::Low,
            _ => ErrorSeverity::Medium,
        }
    }
}

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl ErrorSeverity {
    /// Check if this severity level should cause processing to stop
    pub fn should_stop_processing(&self) -> bool {
        matches!(self, ErrorSeverity::Critical | ErrorSeverity::High)
    }

    /// Check if this severity level should be logged
    pub fn should_log(&self) -> bool {
        matches!(
            self,
            ErrorSeverity::Medium | ErrorSeverity::High | ErrorSeverity::Critical
        )
    }
}

/// Result type alias for data processing operations
pub type DataResult<T> = Result<T, DataError>;

/// Error context for better error reporting
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub operation: String,
    pub data_size: Option<usize>,
    pub column_name: Option<String>,
    pub row_index: Option<usize>,
    pub additional_info: std::collections::HashMap<String, String>,
}

impl ErrorContext {
    /// Create a new error context
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            data_size: None,
            column_name: None,
            row_index: None,
            additional_info: std::collections::HashMap::new(),
        }
    }

    /// Set data size
    pub fn with_data_size(mut self, size: usize) -> Self {
        self.data_size = Some(size);
        self
    }

    /// Set column name
    pub fn with_column_name(mut self, name: impl Into<String>) -> Self {
        self.column_name = Some(name.into());
        self
    }

    /// Set row index
    pub fn with_row_index(mut self, index: usize) -> Self {
        self.row_index = Some(index);
        self
    }

    /// Add additional information
    pub fn add_info(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.additional_info.insert(key.into(), value.into());
    }

    /// Get formatted error message
    pub fn format_error(&self, error: &DataError) -> String {
        let mut message = format!("Error in {}: {}", self.operation, error);

        if let Some(size) = self.data_size {
            message.push_str(&format!(" (Data size: {})", size));
        }

        if let Some(column) = &self.column_name {
            message.push_str(&format!(" (Column: {})", column));
        }

        if let Some(row) = self.row_index {
            message.push_str(&format!(" (Row: {})", row));
        }

        if !self.additional_info.is_empty() {
            message.push_str(" (Additional info: ");
            let info_parts: Vec<String> = self
                .additional_info
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            message.push_str(&info_parts.join(", "));
            message.push(')');
        }

        message
    }
}
