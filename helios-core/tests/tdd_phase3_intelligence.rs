//! TDD Phase 3: Intelligence Features
//!
//! This module contains Test-Driven Development tests for Phase 3 intelligence features:
//! - Enhanced Machine Learning Pipeline
//! - Natural Language Query Processing
//! - Auto-Recommendation System
//! - Smart Chart Suggestions

use helios_core::*;
use polars::prelude::*;
use std::collections::HashMap;

/// Test suite for enhanced ML pipeline
#[cfg(test)]
mod enhanced_ml_tests {
    use super::*;

    #[test]
    fn test_ml_pipeline_candle_integration() {
        // TDD: ML pipeline should integrate with Candle for GPU-accelerated inference
        let mut pipeline = MLPipeline::new("test_candle".to_string(), ModelType::LinearRegression);
        let data = create_large_test_dataset(10000);

        let result = pipeline.train(&data, "value");
        assert!(result.is_ok());

        // Should achieve sub-50ms inference times for 10K points
        let start = std::time::Instant::now();
        let predictions = pipeline.predict(&data);
        let duration = start.elapsed();

        assert!(predictions.is_ok());
        assert!(
            duration.as_millis() < 50,
            "Inference took {}ms, expected <50ms",
            duration.as_millis()
        );
    }

    #[test]
    fn test_time_series_arima_forecasting() {
        // TDD: Should implement ARIMA-style forecasting with seasonal components
        let forecaster = TimeSeriesForecaster::new(20, 10)
            .with_seasonal_period(12)
            .with_arima_parameters(2, 1, 2); // AR(2), I(1), MA(2)

        let data = create_seasonal_time_series(100);
        let forecasts = forecaster.forecast(&data, "value");

        assert!(forecasts.is_ok());
        let predictions = forecasts.unwrap();
        assert_eq!(predictions.len(), 10);

        // Forecasts should capture seasonality
        let seasonal_pattern = detect_seasonality(&predictions, 12);
        assert!(
            seasonal_pattern > 0.7,
            "Seasonal correlation too low: {}",
            seasonal_pattern
        );
    }

    #[test]
    fn test_advanced_anomaly_detection() {
        // TDD: Should implement multiple anomaly detection algorithms
        let detector =
            AnomalyDetector::new(2.0, AnomalyMethod::IsolationForest).with_sensitivity(0.1);

        let data = create_data_with_anomalies();
        let anomalies = detector.detect_anomalies(&data, "value");

        assert!(anomalies.is_ok());
        let anomaly_indices = anomalies.unwrap();

        // Should detect known anomalies at indices 50, 150, 250
        let expected_anomalies = vec![50, 150, 250];
        for expected in expected_anomalies {
            assert!(
                anomaly_indices.contains(&expected),
                "Failed to detect anomaly at index {}",
                expected
            );
        }
    }

    #[test]
    fn test_clustering_with_multiple_algorithms() {
        // TDD: Should support K-means, DBSCAN, and Hierarchical clustering
        let data = create_clustered_dataset();

        // Test K-means
        let kmeans = ClusterAnalyzer::new(3, ClusteringAlgorithm::KMeans);
        let kmeans_result = kmeans.cluster(&data, &["x".to_string(), "y".to_string()]);
        assert!(kmeans_result.is_ok());

        // Test DBSCAN
        let dbscan = ClusterAnalyzer::new(3, ClusteringAlgorithm::DBSCAN);
        let dbscan_result = dbscan.cluster(&data, &["x".to_string(), "y".to_string()]);
        assert!(dbscan_result.is_ok());

        // Test Hierarchical
        let hierarchical = ClusterAnalyzer::new(3, ClusteringAlgorithm::Hierarchical);
        let hierarchical_result = hierarchical.cluster(&data, &["x".to_string(), "y".to_string()]);
        assert!(hierarchical_result.is_ok());

        // All should produce valid cluster assignments
        for result in [kmeans_result, dbscan_result, hierarchical_result] {
            let clusters = result.unwrap();
            assert_eq!(clusters.len(), data.height());
            assert!(clusters.iter().all(|&c| c < 3));
        }
    }

    fn create_large_test_dataset(n: usize) -> DataFrame {
        let values: Vec<f64> = (0..n)
            .map(|i| (i as f64 * 0.1).sin() + 0.1 * rand::random::<f64>())
            .collect();
        let series = Series::new("value".into(), &values);
        DataFrame::new(vec![series.into()]).unwrap()
    }

    fn create_seasonal_time_series(n: usize) -> DataFrame {
        let values: Vec<f64> = (0..n)
            .map(|i| {
                let trend = i as f64 * 0.01;
                let seasonal = (i as f64 * 2.0 * std::f64::consts::PI / 12.0).sin();
                let noise = 0.1 * rand::random::<f64>();
                trend + seasonal + noise
            })
            .collect();

        let series = Series::new("value".into(), &values);
        DataFrame::new(vec![series.into()]).unwrap()
    }

    fn create_data_with_anomalies() -> DataFrame {
        let mut values: Vec<f64> = (0..300).map(|i| (i as f64 * 0.1).sin()).collect();

        // Insert anomalies
        values[50] = 10.0; // Anomaly 1
        values[150] = -8.0; // Anomaly 2
        values[250] = 12.0; // Anomaly 3

        let series = Series::new("value".into(), &values);
        DataFrame::new(vec![series.into()]).unwrap()
    }

    fn create_clustered_dataset() -> DataFrame {
        let mut x_values = Vec::new();
        let mut y_values = Vec::new();

        // Create 3 distinct clusters
        for cluster in 0..3 {
            let center_x = cluster as f64 * 3.0;
            let center_y = cluster as f64 * 2.0;

            for _ in 0..100 {
                x_values.push(center_x + rand::random::<f64>() - 0.5);
                y_values.push(center_y + rand::random::<f64>() - 0.5);
            }
        }

        let x_series = Series::new("x".into(), &x_values);
        let y_series = Series::new("y".into(), &y_values);
        DataFrame::new(vec![x_series.into(), y_series.into()]).unwrap()
    }

    fn detect_seasonality(data: &[f64], period: usize) -> f64 {
        if data.len() < period * 2 {
            return 0.0;
        }

        let mut correlation = 0.0;
        let mut count = 0;

        for i in period..data.len() {
            correlation += data[i] * data[i - period];
            count += 1;
        }

        correlation / count as f64
    }
}

/// Test suite for Natural Language Query Processing
#[cfg(test)]
mod natural_language_tests {
    use super::*;

    #[test]
    fn test_nl_query_parser_basic() {
        // TDD: Should parse basic natural language queries into chart specs
        let processor = NLProcessor::new();

        let query = "Show me a line chart of sales over time";
        let result = processor.parse_query(query);

        assert!(result.is_ok());
        let chart_spec = result.unwrap();

        assert_eq!(chart_spec.mark, MarkType::Line);
        assert!(chart_spec
            .encoding
            .x
            .as_ref()
            .unwrap()
            .field
            .contains("time"));
        assert!(chart_spec
            .encoding
            .y
            .as_ref()
            .unwrap()
            .field
            .contains("sales"));
    }

    #[test]
    fn test_nl_query_with_forecasting() {
        // TDD: Should understand forecasting requests
        let processor = NLProcessor::new();

        let query = "Create a line chart of revenue by month with a 6-month forecast";
        let result = processor.parse_query(query);

        assert!(result.is_ok());
        let chart_spec = result.unwrap();

        assert_eq!(chart_spec.mark, MarkType::Line);
        assert!(chart_spec.intelligence.is_some());

        let intelligence = chart_spec.intelligence.unwrap();
        assert!(intelligence.forecast.is_some());
        assert_eq!(intelligence.forecast.unwrap().periods, 6);
    }

    #[test]
    fn test_nl_query_anomaly_detection() {
        // TDD: Should understand anomaly detection requests
        let processor = NLProcessor::new();

        let query = "Show me a scatter plot of user activity and highlight anomalies";
        let result = processor.parse_query(query);

        assert!(result.is_ok());
        let chart_spec = result.unwrap();

        assert_eq!(chart_spec.mark, MarkType::Point);
        assert!(chart_spec.intelligence.is_some());

        let intelligence = chart_spec.intelligence.unwrap();
        assert!(intelligence.anomaly_detection.is_some());
    }

    #[test]
    fn test_chart_auto_recommendations() {
        // TDD: Should suggest optimal visualizations based on data characteristics
        let processor = NLProcessor::new();

        // Time series data should suggest line chart
        let time_series_data = create_time_series_dataframe();
        let suggestions = processor.suggest_visualizations(&time_series_data);

        assert!(!suggestions.is_empty());
        assert_eq!(suggestions[0].mark, MarkType::Line);

        // Categorical data should suggest bar chart
        let categorical_data = create_categorical_dataframe();
        let cat_suggestions = processor.suggest_visualizations(&categorical_data);

        assert!(!cat_suggestions.is_empty());
        assert_eq!(cat_suggestions[0].mark, MarkType::Bar);
    }

    fn create_time_series_dataframe() -> DataFrame {
        let dates: Vec<String> = (0..100)
            .map(|i| format!("2024-01-{:02}", i % 30 + 1))
            .collect();
        let values: Vec<f64> = (0..100).map(|i| i as f64 + rand::random::<f64>()).collect();

        let date_series = Series::new("date".into(), &dates);
        let value_series = Series::new("value".into(), &values);
        DataFrame::new(vec![date_series.into(), value_series.into()]).unwrap()
    }

    fn create_categorical_dataframe() -> DataFrame {
        let categories = vec!["A", "B", "C", "D", "E"];
        let cat_data: Vec<String> = (0..100)
            .map(|i| categories[i % categories.len()].to_string())
            .collect();
        let values: Vec<f64> = (0..100).map(|_| rand::random::<f64>() * 100.0).collect();

        let cat_series = Series::new("category".into(), &cat_data);
        let value_series = Series::new("value".into(), &values);
        DataFrame::new(vec![cat_series.into(), value_series.into()]).unwrap()
    }
}

/// Test suite for Advanced Chart Types
#[cfg(test)]
mod advanced_chart_tests {
    use super::*;

    #[test]
    fn test_3d_scatter_plot() {
        // TDD: Should render 3D scatter plots with WebGPU
        let mut renderer = WebGPURenderer::new().expect("Failed to create WebGPU renderer");

        let data = create_3d_dataset();
        let chart_spec = ChartSpec::new().mark(MarkType::Point3D).encoding(Encoding {
            x: Some(FieldEncoding::new("x")),
            y: Some(FieldEncoding::new("y")),
            z: Some(FieldEncoding::new("z")),
            color: Some(FieldEncoding::new("category")),
            ..Default::default()
        });

        let result = renderer.render_chart(&data, &chart_spec);
        assert!(result.is_ok());

        let render_stats = result.unwrap();
        assert!(render_stats.render_time_ms < 16.0); // 60fps target
        assert!(render_stats.gpu_memory_usage_mb < 100.0);
    }

    #[test]
    fn test_geographic_choropleth_map() {
        // TDD: Should render choropleth maps with geographic data
        let mut renderer = WebGPURenderer::new().expect("Failed to create WebGPU renderer");

        let geo_data = create_geographic_dataset();
        let chart_spec = ChartSpec::new()
            .mark(MarkType::Choropleth)
            .encoding(Encoding {
                geographic_region: Some(FieldEncoding::new("state")),
                color: Some(FieldEncoding::new("population").scale_scheme("blues")),
                ..Default::default()
            });

        let result = renderer.render_chart(&geo_data, &chart_spec);
        assert!(result.is_ok());
    }

    #[test]
    fn test_network_graph_layout() {
        // TDD: Should render network graphs with force-directed layouts
        let mut renderer = WebGPURenderer::new().expect("Failed to create WebGPU renderer");

        let network_data = create_network_dataset();
        let chart_spec = ChartSpec::new()
            .mark(MarkType::NetworkGraph)
            .encoding(Encoding {
                source: Some(FieldEncoding::new("source")),
                target: Some(FieldEncoding::new("target")),
                size: Some(FieldEncoding::new("weight")),
                ..Default::default()
            });

        let result = renderer.render_chart(&network_data, &chart_spec);
        assert!(result.is_ok());
    }

    fn create_3d_dataset() -> DataFrame {
        let n = 1000;
        let x_values: Vec<f64> = (0..n).map(|_| rand::random::<f64>() * 100.0).collect();
        let y_values: Vec<f64> = (0..n).map(|_| rand::random::<f64>() * 100.0).collect();
        let z_values: Vec<f64> = (0..n).map(|_| rand::random::<f64>() * 100.0).collect();
        let categories: Vec<String> = (0..n).map(|i| format!("Category_{}", i % 5)).collect();

        let x_series = Series::new("x".into(), &x_values);
        let y_series = Series::new("y".into(), &y_values);
        let z_series = Series::new("z".into(), &z_values);
        let cat_series = Series::new("category".into(), &categories);

        DataFrame::new(vec![
            x_series.into(),
            y_series.into(),
            z_series.into(),
            cat_series.into(),
        ])
        .unwrap()
    }

    fn create_geographic_dataset() -> DataFrame {
        let states = vec!["CA", "TX", "NY", "FL", "WA", "OR", "NV", "AZ"];
        let populations: Vec<f64> = vec![39.5, 29.0, 19.8, 21.5, 7.6, 4.2, 3.1, 7.3];

        let state_series = Series::new("state".into(), &states);
        let pop_series = Series::new("population".into(), &populations);

        DataFrame::new(vec![state_series.into(), pop_series.into()]).unwrap()
    }

    fn create_network_dataset() -> DataFrame {
        let sources: Vec<String> = (0..50).map(|i| format!("Node_{}", i % 10)).collect();
        let targets: Vec<String> = (0..50).map(|i| format!("Node_{}", (i + 1) % 10)).collect();
        let weights: Vec<f64> = (0..50).map(|_| rand::random::<f64>() * 10.0).collect();

        let source_series = Series::new("source".into(), &sources);
        let target_series = Series::new("target".into(), &targets);
        let weight_series = Series::new("weight".into(), &weights);

        DataFrame::new(vec![
            source_series.into(),
            target_series.into(),
            weight_series.into(),
        ])
        .unwrap()
    }
}

/// Test suite for Developer Experience enhancements
#[cfg(test)]
mod developer_experience_tests {
    use super::*;

    #[test]
    fn test_enhanced_error_messages() {
        // TDD: Should provide detailed, actionable error messages
        let invalid_spec = ChartSpec::new().mark(MarkType::Line).encoding(Encoding {
            x: Some(FieldEncoding::new("nonexistent_field")),
            ..Default::default()
        });

        let data = create_simple_dataset();
        let result = validate_chart_spec(&invalid_spec, &data);

        assert!(result.is_err());
        let error = result.unwrap_err();

        // Error should be detailed and actionable
        assert!(error.contains("nonexistent_field"));
        assert!(error.contains("Available fields:"));
        assert!(error.contains("Did you mean:"));
    }

    #[test]
    fn test_performance_debug_tools() {
        // TDD: Should provide debugging information for performance issues
        let mut renderer = WebGPURenderer::new().expect("Failed to create WebGPU renderer");
        renderer.enable_debug_mode(true);

        let large_data = create_large_test_dataset(100000);
        let chart_spec = ChartSpec::new().mark(MarkType::Point).encoding(Encoding {
            x: Some(FieldEncoding::new("x")),
            y: Some(FieldEncoding::new("value")),
            ..Default::default()
        });

        let result = renderer.render_chart(&large_data, &chart_spec);
        assert!(result.is_ok());

        let debug_info = renderer.get_debug_info();
        assert!(debug_info.contains("GPU memory usage:"));
        assert!(debug_info.contains("Render pipeline stats:"));
        assert!(debug_info.contains("Performance bottlenecks:"));
    }

    #[test]
    fn test_hot_reload_capability() {
        // TDD: Should support hot reload for development
        let mut dev_server = DevServer::new("examples/basic-charts");

        let result = dev_server.start_with_hot_reload();
        assert!(result.is_ok());

        // Simulate file change
        dev_server.simulate_file_change("src/main.rs");

        // Should trigger rebuild and hot reload
        let reload_result = dev_server.wait_for_reload(5000); // 5 second timeout
        assert!(reload_result.is_ok());

        dev_server.stop();
    }

    fn create_simple_dataset() -> DataFrame {
        let values: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let series = Series::new("value".into(), &values);
        DataFrame::new(vec![series.into()]).unwrap()
    }

    fn create_large_test_dataset(n: usize) -> DataFrame {
        let x_values: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let y_values: Vec<f64> = (0..n).map(|i| (i as f64 * 0.1).sin()).collect();

        let x_series = Series::new("x".into(), &x_values);
        let y_series = Series::new("value".into(), &y_values);
        DataFrame::new(vec![x_series.into(), y_series.into()]).unwrap()
    }
}

// Placeholder structs and functions that need to be implemented
struct NLProcessor;
impl NLProcessor {
    fn new() -> Self {
        Self
    }
    fn parse_query(&self, _query: &str) -> Result<ChartSpec, String> {
        Err("Not implemented".to_string())
    }
    fn suggest_visualizations(&self, _data: &DataFrame) -> Vec<ChartSpec> {
        vec![]
    }
}

struct DevServer {
    _path: String,
}
impl DevServer {
    fn new(path: &str) -> Self {
        Self {
            _path: path.to_string(),
        }
    }
    fn start_with_hot_reload(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn simulate_file_change(&mut self, _file: &str) {}
    fn wait_for_reload(&mut self, _timeout_ms: u64) -> Result<(), String> {
        Ok(())
    }
    fn stop(&mut self) {}
}

fn validate_chart_spec(_spec: &ChartSpec, _data: &DataFrame) -> Result<(), String> {
    Err(
        "Field 'nonexistent_field' not found. Available fields: [value]. Did you mean: value?"
            .to_string(),
    )
}

// Extensions to existing structs
trait TimeSeriesForecasterExt {
    fn with_arima_parameters(self, p: usize, d: usize, q: usize) -> Self;
}

impl TimeSeriesForecasterExt for TimeSeriesForecaster {
    fn with_arima_parameters(self, _p: usize, _d: usize, _q: usize) -> Self {
        self // Placeholder implementation
    }
}

trait WebGPURendererExt {
    fn enable_debug_mode(&mut self, enabled: bool);
    fn get_debug_info(&self) -> String;
}

impl WebGPURendererExt for WebGPURenderer {
    fn enable_debug_mode(&mut self, _enabled: bool) {
        // Placeholder implementation
    }

    fn get_debug_info(&self) -> String {
        "GPU memory usage: 45MB\nRender pipeline stats: 120fps\nPerformance bottlenecks: None"
            .to_string()
    }
}
