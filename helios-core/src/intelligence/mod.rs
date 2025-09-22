//! Intelligence and Machine Learning Module
//!
//! This module provides machine learning capabilities for Helios visualizations,
//! including forecasting, anomaly detection, and clustering analysis.

pub mod ml_pipeline;
pub mod time_series;
pub mod anomaly_detection;
pub mod clustering;

// Re-export main types for backward compatibility
pub use ml_pipeline::*;
pub use time_series::*;
pub use anomaly_detection::*;
pub use clustering::*;

/// Machine Learning error types
#[derive(Debug, thiserror::Error)]
pub enum MLError {
    /// Training process failed
    #[error("Training failed: {0}")]
    TrainingFailed(String),

    /// Prediction process failed
    #[error("Prediction failed: {0}")]
    PredictionFailed(String),

    /// Invalid data provided
    #[error("Invalid data: {0}")]
    InvalidData(String),

    /// Model has not been trained yet
    #[error("Model not trained")]
    ModelNotTrained,

    /// Insufficient data for the operation
    #[error("Insufficient data: {0}")]
    InsufficientData(String),
}
