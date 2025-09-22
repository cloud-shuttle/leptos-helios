//! Advanced Analytics Engine
//!
//! This module provides comprehensive analytics capabilities for Helios visualizations,
//! including enhanced ML pipelines, statistical analysis, anomaly detection, and forecasting.

pub mod ml_models;
pub mod statistical_analysis;
pub mod time_series;
pub mod correlation;
pub mod types;

// Re-export main types for backward compatibility
pub use types::*;
pub use ml_models::*;
pub use statistical_analysis::*;
pub use time_series::*;
pub use correlation::*;

/// Advanced analytics errors
#[derive(Debug, thiserror::Error)]
pub enum AnalyticsError {
    #[error("ML model error: {message}")]
    MLModelError { message: String },

    #[error("Statistical analysis error: {message}")]
    StatisticalError { message: String },

    #[error("Anomaly detection error: {message}")]
    AnomalyDetectionError { message: String },

    #[error("Forecasting error: {message}")]
    ForecastingError { message: String },

    #[error("Data processing error: {message}")]
    DataProcessingError { message: String },

    #[error("Algorithm registration error: {message}")]
    AlgorithmRegistrationError { message: String },
}
