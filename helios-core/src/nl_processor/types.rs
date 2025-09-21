//! Types and configuration for natural language processing

use crate::chart::{MarkType, Intelligence};
use std::collections::HashMap;

/// Natural Language Query Processing errors
#[derive(Debug, thiserror::Error)]
pub enum NLError {
    /// Failed to parse the natural language query
    #[error("Failed to parse query: {0}")]
    ParseError(String),

    /// Unsupported chart type requested
    #[error("Unsupported chart type: {0}")]
    UnsupportedChartType(String),

    /// Missing required field in the query
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Invalid reference to data field
    #[error("Invalid data reference: {0}")]
    InvalidDataReference(String),

    /// Query is ambiguous and needs clarification
    #[error("Ambiguous query: {0}")]
    AmbiguousQuery(String),
}

/// Configuration for natural language processing
#[derive(Debug, Clone)]
pub struct NLConfig {
    /// Whether to enable fuzzy matching for queries
    pub fuzzy_matching: bool,
    /// Whether to provide automatic suggestions
    pub auto_suggestions: bool,
    /// Minimum confidence threshold for matches (0.0 to 1.0)
    pub confidence_threshold: f64,
    /// Maximum number of suggestions to provide
    pub max_suggestions: usize,
}

impl Default for NLConfig {
    fn default() -> Self {
        Self {
            fuzzy_matching: true,
            auto_suggestions: true,
            confidence_threshold: 0.7,
            max_suggestions: 5,
        }
    }
}

/// Configuration for intelligence features
#[derive(Debug, Clone)]
pub struct IntelligenceConfig {
    /// Enable forecasting capabilities
    pub forecasting: bool,
    /// Enable anomaly detection
    pub anomaly_detection: bool,
    /// Enable clustering analysis
    pub clustering: bool,
    /// Enable trend analysis
    pub trend_analysis: bool,
}

/// Configuration for forecasting
#[derive(Debug, Clone)]
pub struct ForecastConfig {
    /// Forecasting method to use
    pub method: ForecastMethod,
    /// Number of periods to forecast
    pub periods: u32,
    /// Confidence interval for forecasts
    pub confidence_interval: f64,
}

/// Available forecasting methods
#[derive(Debug, Clone, PartialEq)]
pub enum ForecastMethod {
    /// Linear regression forecasting
    Linear,
    /// Exponential smoothing
    Exponential,
    /// ARIMA model
    Arima,
    /// Seasonal decomposition
    Seasonal,
}

impl std::fmt::Display for ForecastMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForecastMethod::Linear => write!(f, "linear"),
            ForecastMethod::Exponential => write!(f, "exponential"),
            ForecastMethod::Arima => write!(f, "arima"),
            ForecastMethod::Seasonal => write!(f, "seasonal"),
        }
    }
}

/// Configuration for anomaly detection
#[derive(Debug, Clone)]
pub struct AnomalyConfig {
    /// Sensitivity threshold for anomaly detection
    pub sensitivity: f64,
    /// Method for anomaly detection
    pub method: String,
}

/// Configuration for clustering
#[derive(Debug, Clone)]
pub struct ClusteringConfig {
    /// Number of clusters
    pub num_clusters: usize,
    /// Clustering algorithm
    pub algorithm: String,
}

/// Result of query pattern matching
#[derive(Debug, Clone)]
pub struct QueryMatch {
    /// Detected chart type
    pub chart_type: MarkType,
    /// Extracted field references
    pub fields: Vec<String>,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Intelligence configuration if detected
    pub intelligence: Option<Intelligence>,
}

/// Data analysis result
#[derive(Debug, Clone)]
pub struct DataAnalysis {
    /// Detected data types for each column
    pub column_types: HashMap<String, String>,
    /// Suggested chart types
    pub suggested_charts: Vec<MarkType>,
    /// Data characteristics
    pub characteristics: Vec<String>,
}
