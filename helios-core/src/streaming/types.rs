//! Core types for streaming data system

use crate::chart_config::*;
use std::time::{Duration, Instant};

/// Data types for streaming
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    TimeSeries,
    Events,
    Metrics,
    Logs,
}

/// Data point for streaming
#[derive(Debug, Clone)]
pub struct DataPoint {
    pub timestamp: Instant,
    pub value: f64,
    pub metadata: Option<String>,
}

/// Stream configuration
#[derive(Debug, Clone)]
pub struct StreamConfig {
    pub stream_id: String,
    pub buffer_size: usize,
    pub update_interval: Duration,
    pub data_type: DataType,
}

/// Stream statistics
#[derive(Debug, Clone)]
pub struct StreamStats {
    pub total_published: usize,
    pub active_subscribers: usize,
    pub buffer_usage: usize,
    pub last_update: Option<Instant>,
}

/// Streaming error types
#[derive(Debug, Clone)]
pub enum StreamingError {
    ConnectionFailed(String),
    InvalidData(String),
    BufferOverflow,
    SubscriberNotFound,
    ConfigurationError(String),
    ProcessingError(String),
    QualityCheckFailed(String),
    SynchronizationError(String),
}

impl From<StreamingError> for ChartRenderError {
    fn from(err: StreamingError) -> Self {
        match err {
            StreamingError::ConnectionFailed(msg) => {
                ChartRenderError::InvalidConfig(format!("Connection failed: {}", msg))
            }
            StreamingError::InvalidData(msg) => {
                ChartRenderError::InvalidConfig(format!("Invalid data: {}", msg))
            }
            StreamingError::BufferOverflow => {
                ChartRenderError::InvalidConfig("Buffer overflow".to_string())
            }
            StreamingError::SubscriberNotFound => {
                ChartRenderError::InvalidConfig("Subscriber not found".to_string())
            }
            StreamingError::ConfigurationError(msg) => {
                ChartRenderError::InvalidConfig(format!("Configuration error: {}", msg))
            }
            StreamingError::ProcessingError(msg) => {
                ChartRenderError::InvalidConfig(format!("Processing error: {}", msg))
            }
            StreamingError::QualityCheckFailed(msg) => {
                ChartRenderError::InvalidConfig(format!("Quality check failed: {}", msg))
            }
            StreamingError::SynchronizationError(msg) => {
                ChartRenderError::InvalidConfig(format!("Synchronization error: {}", msg))
            }
        }
    }
}
