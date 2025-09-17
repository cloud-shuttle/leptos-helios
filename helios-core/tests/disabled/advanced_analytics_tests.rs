//! Advanced Analytics Tests
//!
//! Comprehensive test suite for the Helios advanced analytics system, including
//! ML pipelines, statistical analysis, anomaly detection, forecasting, and algorithm registry.

use leptos_helios::intelligence::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[test]
fn test_ml_pipeline_creation() {
    let pipeline = MLPipeline::new("test_pipeline".to_string(), ModelType::LinearRegression);
    assert_eq!(pipeline.name, "test_pipeline");
    assert_eq!(pipeline.model_type, ModelType::LinearRegression);
    assert!(!pipeline.trained);
}

#[test]
fn test_ml_pipeline_training() {
    let mut pipeline = MLPipeline::new("test_pipeline".to_string(), ModelType::LinearRegression);

    // Create test data
    let data = df! {
        "x" => [1.0, 2.0, 3.0, 4.0],
        "y" => [2.0, 4.0, 6.0, 8.0],
    }.unwrap();

    let result = pipeline.train(&data, "y");
    assert!(result.is_ok());
    assert!(pipeline.trained);
}

#[test]
fn test_anomaly_detection() {
    let mut pipeline = MLPipeline::new("test_pipeline".to_string(), ModelType::AnomalyDetection);

    // Create test data
    let data = df! {
        "value" => [1.0, 2.0, 3.0, 100.0, 4.0, 5.0], // 100.0 is an anomaly
    }.unwrap();

    let result = pipeline.train(&data, "value");
    assert!(result.is_ok());
    assert!(pipeline.trained);

}

#[test]
fn test_statistical_analyzer_creation() {
    let analyzer = StatisticalAnalyzer::new();
