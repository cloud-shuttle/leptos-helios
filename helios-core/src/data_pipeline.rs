//! Data Pipeline for processing and optimizing chart data
//!
//! This module provides the DataPipeline struct that handles:
//! - Data processing and validation
//! - Optimization for rendering
//! - GPU buffer creation

use polars::prelude::*;
use std::time::{Duration, Instant};

/// Result type for data pipeline operations
pub type PipelineResult<T> = Result<T, PipelineError>;

/// Errors that can occur during data pipeline operations
#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("Data validation failed: {0}")]
    ValidationError(String),

    #[error("Processing timeout: {0}")]
    TimeoutError(String),

    #[error("GPU buffer creation failed: {0}")]
    GpuBufferError(String),

    #[error("Optimization failed: {0}")]
    OptimizationError(String),
}

/// GPU buffer information for rendering
#[derive(Debug, Clone)]
pub struct GpuBuffers {
    pub vertex_count: usize,
    pub buffer_size: usize,
    pub is_valid: bool,
}

impl GpuBuffers {
    /// Create new GPU buffers
    pub fn new(vertex_count: usize, buffer_size: usize) -> Self {
        Self {
            vertex_count,
            buffer_size,
            is_valid: true,
        }
    }

    /// Get the number of vertices
    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    /// Check if buffers are valid
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
}

/// Data Pipeline for processing chart data
#[derive(Debug)]
pub struct DataPipeline {
    processing_timeout: Duration,
    optimization_enabled: bool,
}

impl DataPipeline {
    /// Create a new DataPipeline
    pub fn new() -> Self {
        Self {
            processing_timeout: Duration::from_millis(100),
            optimization_enabled: true,
        }
    }

    /// Process raw data with validation
    pub fn process(&self, data: &DataFrame) -> PipelineResult<DataFrame> {
        let start = Instant::now();

        // Validate data
        if data.is_empty() {
            return Err(PipelineError::ValidationError(
                "Empty DataFrame".to_string(),
            ));
        }

        // Check for timeout
        if start.elapsed() > self.processing_timeout {
            return Err(PipelineError::TimeoutError(
                "Processing timeout".to_string(),
            ));
        }

        // For now, just return the data as-is
        // In a real implementation, this would do data cleaning, validation, etc.
        Ok(data.clone())
    }

    /// Optimize data for rendering
    pub fn optimize(&self, data: &DataFrame) -> PipelineResult<DataFrame> {
        let start = Instant::now();

        if !self.optimization_enabled {
            return Ok(data.clone());
        }

        // Check for timeout
        if start.elapsed() > Duration::from_millis(20) {
            return Err(PipelineError::TimeoutError(
                "Optimization timeout".to_string(),
            ));
        }

        // For now, just return the data as-is
        // In a real implementation, this would do data optimization, sampling, etc.
        Ok(data.clone())
    }

    /// Convert optimized data to GPU buffers
    pub fn to_gpu_buffers(&self, data: &DataFrame) -> PipelineResult<GpuBuffers> {
        let start = Instant::now();

        // Estimate vertex count based on data size
        let vertex_count = data.height();
        let buffer_size = vertex_count * 8; // 8 bytes per vertex (2 floats for x,y)

        // Check for timeout
        if start.elapsed() > Duration::from_millis(30) {
            return Err(PipelineError::TimeoutError(
                "GPU buffer creation timeout".to_string(),
            ));
        }

        Ok(GpuBuffers::new(vertex_count, buffer_size))
    }

    /// Set processing timeout
    pub fn set_processing_timeout(&mut self, timeout: Duration) {
        self.processing_timeout = timeout;
    }

    /// Enable or disable optimization
    pub fn set_optimization_enabled(&mut self, enabled: bool) {
        self.optimization_enabled = enabled;
    }
}

impl Default for DataPipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_pipeline_creation() {
        let pipeline = DataPipeline::new();
        assert!(pipeline.optimization_enabled);
        assert_eq!(pipeline.processing_timeout, Duration::from_millis(100));
    }

    #[test]
    fn test_data_pipeline_process_empty_data() {
        let pipeline = DataPipeline::new();
        let empty_df = DataFrame::empty();

        let result = pipeline.process(&empty_df);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PipelineError::ValidationError(_)
        ));
    }

    #[test]
    fn test_gpu_buffers_creation() {
        let buffers = GpuBuffers::new(1000, 8000);
        assert_eq!(buffers.vertex_count(), 1000);
        assert_eq!(buffers.buffer_size, 8000);
        assert!(buffers.is_valid());
    }
}
