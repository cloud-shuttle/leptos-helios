//! TDD Tests for Phase 3: ML Intelligence Module
//!
//! This module tests the ML intelligence capabilities including forecasting,
//! chart recommendations, and performance validation.

use leptos_helios::ml_intelligence::*;
use proptest::prelude::*;
use std::time::Duration;

/// Test ML forecaster basic functionality
#[test]
fn test_ml_forecaster_creation() {
    let forecaster = MLForecaster::new();

    assert!(!forecaster.is_model_loaded());
    assert_eq!(forecaster.get_inference_count(), 0);
}

#[test]
fn test_ml_forecaster_model_loading() {
    let mut forecaster = MLForecaster::new();

    assert!(!forecaster.is_model_loaded());

    let result = forecaster.load_model();
    assert!(result.is_ok());
    assert!(forecaster.is_model_loaded());
}

#[test]
fn test_ml_forecaster_forecast_without_model() {
    let mut forecaster = MLForecaster::new();
    let series = vec![
        TimeSeriesPoint {
            timestamp: 1.0,
            value: 10.0,
        },
        TimeSeriesPoint {
            timestamp: 2.0,
            value: 12.0,
        },
    ];

    let result = forecaster.forecast(&series, 3);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Model not loaded");
}

#[test]
fn test_ml_forecaster_forecast_with_model() {
    let mut forecaster = MLForecaster::new();
    forecaster.load_model().unwrap();

    let series = vec![
        TimeSeriesPoint {
            timestamp: 1.0,
            value: 10.0,
        },
        TimeSeriesPoint {
            timestamp: 2.0,
            value: 12.0,
        },
        TimeSeriesPoint {
            timestamp: 3.0,
            value: 14.0,
        },
    ];

    let result = forecaster.forecast(&series, 2);
    assert!(result.is_ok());

    let forecast = result.unwrap();
    assert_eq!(forecast.predictions.len(), 2);
    assert!(forecast.confidence >= 0.0 && forecast.confidence <= 1.0);
    assert_eq!(forecast.model_type, "LSTM");
    assert_eq!(forecaster.get_inference_count(), 1);
}

#[test]
fn test_ml_forecaster_performance_requirement() {
    let mut forecaster = MLForecaster::new();
    forecaster.load_model().unwrap();

    let series = vec![
        TimeSeriesPoint {
            timestamp: 1.0,
            value: 10.0,
        },
        TimeSeriesPoint {
            timestamp: 2.0,
            value: 12.0,
        },
        TimeSeriesPoint {
            timestamp: 3.0,
            value: 14.0,
        },
    ];

    let result = forecaster.forecast(&series, 10);
    assert!(result.is_ok());

    let forecast = result.unwrap();
    // TDD requirement: ML inference <50ms
    assert!(
        forecast.inference_time < Duration::from_millis(50),
        "ML inference too slow: {:?}",
        forecast.inference_time
    );
}

#[test]
fn test_ml_forecaster_empty_series() {
    let mut forecaster = MLForecaster::new();
    forecaster.load_model().unwrap();

    let series = vec![];
    let result = forecaster.forecast(&series, 5);

    assert!(result.is_ok());
    let forecast = result.unwrap();
    assert_eq!(forecast.predictions.len(), 5);
    assert_eq!(forecast.predictions, vec![0.0; 5]);
}

#[test]
fn test_ml_forecaster_single_point() {
    let mut forecaster = MLForecaster::new();
    forecaster.load_model().unwrap();

    let series = vec![TimeSeriesPoint {
        timestamp: 1.0,
        value: 42.0,
    }];
    let result = forecaster.forecast(&series, 3);

    assert!(result.is_ok());
    let forecast = result.unwrap();
    assert_eq!(forecast.predictions.len(), 3);
    // With single point, trend should be 0, so all predictions should be the same
    assert_eq!(forecast.predictions, vec![42.0, 42.0, 42.0]);
}

/// Test chart recommendation engine
#[test]
fn test_chart_recommendation_engine_creation() {
    let engine = ChartRecommendationEngine::new();

    assert_eq!(engine.get_cache_size(), 0);
}

#[test]
fn test_chart_recommendation_basic() {
    let mut engine = ChartRecommendationEngine::new();
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    let recommendation = engine.recommend_chart(&data, "test");

    assert!(!recommendation.chart_type.is_empty());
    assert!(recommendation.confidence >= 0.0 && recommendation.confidence <= 1.0);
    assert!(!recommendation.reasoning.is_empty());
}

#[test]
fn test_chart_recommendation_caching() {
    let mut engine = ChartRecommendationEngine::new();
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    // First recommendation
    let rec1 = engine.recommend_chart(&data, "test");
    assert_eq!(engine.get_cache_size(), 1);

    // Second recommendation should use cache
    let rec2 = engine.recommend_chart(&data, "test");
    assert_eq!(engine.get_cache_size(), 1);
    assert_eq!(rec1.chart_type, rec2.chart_type);
}

#[test]
fn test_chart_recommendation_cache_clear() {
    let mut engine = ChartRecommendationEngine::new();
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    engine.recommend_chart(&data, "test");
    assert_eq!(engine.get_cache_size(), 1);

    engine.clear_cache();
    assert_eq!(engine.get_cache_size(), 0);
}

#[test]
fn test_chart_recommendation_empty_data() {
    let mut engine = ChartRecommendationEngine::new();
    let data = vec![];

    let recommendation = engine.recommend_chart(&data, "empty");

    assert!(!recommendation.chart_type.is_empty());
    assert!(recommendation.confidence >= 0.0 && recommendation.confidence <= 1.0);
}

/// Test data analyzer
#[test]
fn test_data_analyzer_creation() {
    let analyzer = DataAnalyzer::new();

    assert_eq!(analyzer.get_analysis_count(), 0);
}

#[test]
fn test_data_analyzer_basic_analysis() {
    let mut analyzer = DataAnalyzer::new();
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    let analysis = analyzer.analyze(&data);

    assert_eq!(analysis.data_points, 5);
    assert_eq!(analysis.mean, 3.0);
    assert!(analysis.variance >= 0.0);
    assert_eq!(analyzer.get_analysis_count(), 1);
}

#[test]
fn test_data_analyzer_empty_data() {
    let mut analyzer = DataAnalyzer::new();
    let data = vec![];

    let analysis = analyzer.analyze(&data);

    assert_eq!(analysis.data_points, 0);
    assert_eq!(analysis.mean, 0.0);
    assert_eq!(analysis.variance, 0.0);
}

#[test]
fn test_data_analyzer_single_point() {
    let mut analyzer = DataAnalyzer::new();
    let data = vec![42.0];

    let analysis = analyzer.analyze(&data);

    assert_eq!(analysis.data_points, 1);
    assert_eq!(analysis.mean, 42.0);
    assert_eq!(analysis.variance, 0.0);
}

#[test]
fn test_data_analyzer_outlier_detection() {
    let mut analyzer = DataAnalyzer::new();
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 100.0]; // 100 is an outlier

    let analysis = analyzer.analyze(&data);

    assert_eq!(analysis.data_points, 6);
    assert!(analysis.outlier_count > 0);
}

/// Test ML performance monitor
#[test]
fn test_ml_performance_monitor_creation() {
    let monitor = MLPerformanceMonitor::new();
    let stats = monitor.get_stats();

    assert_eq!(stats.total_inferences, 0);
    assert_eq!(stats.average_inference_time, Duration::ZERO);
}

#[test]
fn test_ml_performance_monitor_recording() {
    let mut monitor = MLPerformanceMonitor::new();

    monitor.record_inference(Duration::from_millis(10));
    monitor.record_inference(Duration::from_millis(20));
    monitor.record_inference(Duration::from_millis(30));

    let stats = monitor.get_stats();
    assert_eq!(stats.total_inferences, 3);
    assert_eq!(stats.average_inference_time, Duration::from_millis(20));
    assert_eq!(stats.max_inference_time, Duration::from_millis(30));
    assert_eq!(stats.min_inference_time, Duration::from_millis(10));
}

#[test]
fn test_ml_performance_monitor_single_inference() {
    let mut monitor = MLPerformanceMonitor::new();

    monitor.record_inference(Duration::from_millis(25));

    let stats = monitor.get_stats();
    assert_eq!(stats.total_inferences, 1);
    assert_eq!(stats.average_inference_time, Duration::from_millis(25));
    assert_eq!(stats.max_inference_time, Duration::from_millis(25));
    assert_eq!(stats.min_inference_time, Duration::from_millis(25));
}

/// Property-based tests for ML components
proptest! {
    #[test]
    fn test_forecasting_mathematical_properties(
        series in prop::collection::vec(
            (0.0f64..1000.0, 0.0f64..100.0),
            1..100
        ).prop_map(|points| {
            points.into_iter().enumerate().map(|(i, (_, value))| {
                TimeSeriesPoint {
                    timestamp: i as f64,
                    value
                }
            }).collect::<Vec<_>>()
        }),
        forecast_periods in 1u32..100
    ) {
        let mut forecaster = MLForecaster::new();
        forecaster.load_model().unwrap();

        let forecast = forecaster.forecast(&series, forecast_periods).unwrap();

        // Mathematical properties that must always hold
        prop_assert_eq!(forecast.predictions.len(), forecast_periods as usize);
        prop_assert!(forecast.confidence >= 0.0 && forecast.confidence <= 1.0);

        // Performance property: <50ms inference time
        prop_assert!(forecast.inference_time < Duration::from_millis(50));

        // Consistency property: same input should give same output
        let forecast2 = forecaster.forecast(&series, forecast_periods).unwrap();
        prop_assert_eq!(forecast.predictions, forecast2.predictions);
    }

    #[test]
    fn test_chart_recommendation_properties(
        data in prop::collection::vec(-1000.0f64..1000.0, 0..1000),
        metadata in ".*"
    ) {
        let mut engine = ChartRecommendationEngine::new();
        let recommendation = engine.recommend_chart(&data, &metadata);

        // Properties that must always hold
        prop_assert!(!recommendation.chart_type.is_empty());
        prop_assert!(recommendation.confidence >= 0.0 && recommendation.confidence <= 1.0);
        prop_assert!(!recommendation.reasoning.is_empty());

        // Optimization suggestions should be reasonable
        for suggestion in &recommendation.optimization_suggestions {
            prop_assert!(!suggestion.is_empty());
            prop_assert!(suggestion.len() > 10); // Meaningful suggestions
        }
    }

    #[test]
    fn test_data_analysis_properties(
        data in prop::collection::vec(-1000.0f64..1000.0, 0..1000)
    ) {
        let mut analyzer = DataAnalyzer::new();
        let analysis = analyzer.analyze(&data);

        // Mathematical properties that must always hold
        prop_assert_eq!(analysis.data_points, data.len());

        if !data.is_empty() {
            prop_assert!(analysis.variance >= 0.0);
            prop_assert!(analysis.outlier_count <= data.len());
        } else {
            prop_assert_eq!(analysis.mean, 0.0);
            prop_assert_eq!(analysis.variance, 0.0);
        }
    }

    #[test]
    fn test_ml_performance_monitor_properties(
        durations in prop::collection::vec(1u64..1000, 1..100)
    ) {
        let mut monitor = MLPerformanceMonitor::new();

        for duration in &durations {
            monitor.record_inference(Duration::from_millis(*duration));
        }

        let stats = monitor.get_stats();

        // Properties that must always hold
        prop_assert_eq!(stats.total_inferences, durations.len() as u32);
        prop_assert!(stats.average_inference_time >= stats.min_inference_time);
        prop_assert!(stats.average_inference_time <= stats.max_inference_time);

        if !durations.is_empty() {
            let expected_avg = durations.iter().sum::<u64>() / durations.len() as u64;
            prop_assert_eq!(stats.average_inference_time.as_millis() as u64, expected_avg);
        }
    }
}

/// Integration tests for ML components working together
#[test]
fn test_ml_integration_forecast_and_recommend() {
    let mut forecaster = MLForecaster::new();
    let mut engine = ChartRecommendationEngine::new();

    forecaster.load_model().unwrap();

    // Generate time series data
    let series = (0..50)
        .map(|i| TimeSeriesPoint {
            timestamp: i as f64,
            value: (i as f64 * 0.5 + (i as f64 * 0.1).sin() * 10.0),
        })
        .collect::<Vec<_>>();

    // Generate forecast
    let forecast = forecaster.forecast(&series, 10).unwrap();
    assert_eq!(forecast.predictions.len(), 10);

    // Extract values for recommendation
    let values = series.iter().map(|p| p.value).collect::<Vec<_>>();

    // Get chart recommendation
    let recommendation = engine.recommend_chart(&values, "time_series");
    assert!(!recommendation.chart_type.is_empty());

    // Verify performance requirements
    assert!(forecast.inference_time < Duration::from_millis(50));
    assert!(recommendation.confidence > 0.0);
}

#[test]
fn test_ml_integration_performance_monitoring() {
    let mut forecaster = MLForecaster::new();
    let mut monitor = MLPerformanceMonitor::new();

    forecaster.load_model().unwrap();

    // Run multiple inferences and monitor performance
    for i in 0..10 {
        let series = vec![
            TimeSeriesPoint {
                timestamp: 1.0,
                value: i as f64,
            },
            TimeSeriesPoint {
                timestamp: 2.0,
                value: (i + 1) as f64,
            },
        ];

        let forecast = forecaster.forecast(&series, 5).unwrap();
        monitor.record_inference(forecast.inference_time);
    }

    let stats = monitor.get_stats();
    assert_eq!(stats.total_inferences, 10);
    assert!(stats.average_inference_time < Duration::from_millis(50));
}

/// Edge case tests
#[test]
fn test_ml_edge_cases_extreme_values() {
    let mut forecaster = MLForecaster::new();
    forecaster.load_model().unwrap();

    // Test with extreme values
    let series = vec![
        TimeSeriesPoint {
            timestamp: 0.0,
            value: f64::MIN_POSITIVE,
        },
        TimeSeriesPoint {
            timestamp: 1.0,
            value: f64::MAX,
        },
    ];

    let result = forecaster.forecast(&series, 3);
    assert!(result.is_ok());

    let forecast = result.unwrap();
    assert_eq!(forecast.predictions.len(), 3);
    assert!(forecast.predictions.iter().all(|&x| x.is_finite()));
}

#[test]
fn test_ml_edge_cases_very_large_dataset() {
    let mut engine = ChartRecommendationEngine::new();

    // Test with very large dataset
    let large_data = (0..10000).map(|i| i as f64).collect::<Vec<_>>();

    let recommendation = engine.recommend_chart(&large_data, "large_dataset");

    assert!(!recommendation.chart_type.is_empty());
    assert!(recommendation
        .optimization_suggestions
        .iter()
        .any(|s| s.contains("sampling") || s.contains("aggregation")));
}

#[test]
fn test_ml_edge_cases_constant_data() {
    let mut analyzer = DataAnalyzer::new();

    // Test with constant data
    let constant_data = vec![42.0; 100];

    let analysis = analyzer.analyze(&constant_data);

    assert_eq!(analysis.data_points, 100);
    assert_eq!(analysis.mean, 42.0);
    assert_eq!(analysis.variance, 0.0);
    assert_eq!(analysis.outlier_count, 0);
}

/// Performance regression tests
#[test]
fn test_ml_performance_regression_forecasting() {
    let mut forecaster = MLForecaster::new();
    forecaster.load_model().unwrap();

    let series = (0..1000)
        .map(|i| TimeSeriesPoint {
            timestamp: i as f64,
            value: (i as f64 * 0.1).sin(),
        })
        .collect::<Vec<_>>();

    let start = std::time::Instant::now();
    let forecast = forecaster.forecast(&series, 100).unwrap();
    let total_time = start.elapsed();

    // Performance regression test: total operation <100ms
    assert!(
        total_time < Duration::from_millis(100),
        "ML forecasting performance regression: {:?}",
        total_time
    );

    // Individual inference should still be <50ms
    assert!(forecast.inference_time < Duration::from_millis(50));
}

#[test]
fn test_ml_performance_regression_recommendations() {
    let mut engine = ChartRecommendationEngine::new();

    let data = (0..5000)
        .map(|i| (i as f64 * 0.1).cos())
        .collect::<Vec<_>>();

    let start = std::time::Instant::now();
    let recommendation = engine.recommend_chart(&data, "performance_test");
    let total_time = start.elapsed();

    // Performance regression test: recommendation <10ms
    assert!(
        total_time < Duration::from_millis(10),
        "Chart recommendation performance regression: {:?}",
        total_time
    );

    assert!(!recommendation.chart_type.is_empty());
}
