//! Advanced Analytics Tests
//!
//! Comprehensive test suite for the Helios advanced analytics system, including
//! ML pipelines, statistical analysis, anomaly detection, forecasting, and algorithm registry.

use leptos_helios::advanced_analytics::*;
use leptos_helios::algorithm_registry::*;
use leptos_helios::anomaly_detection::*;
use leptos_helios::forecasting_engine::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[test]
fn test_ml_pipeline_creation() {
    let pipeline = MLPipeline::new();
    assert_eq!(pipeline.list_models().len(), 0);
    assert!(pipeline.active_model.is_none());
}

#[test]
fn test_ml_pipeline_model_registration() {
    let mut pipeline = MLPipeline::new();

    // Create a simple mock model
    let model = MockMLModel::new("test-model".to_string());
    let model_id = ModelId("test-model".to_string());

    pipeline.register_model(model_id.clone(), Box::new(model));

    assert_eq!(pipeline.list_models().len(), 1);
    assert_eq!(pipeline.active_model, Some(model_id));
}

#[test]
fn test_ml_pipeline_active_model_switching() {
    let mut pipeline = MLPipeline::new();

    // Register multiple models
    let model1 = MockMLModel::new("model1".to_string());
    let model2 = MockMLModel::new("model2".to_string());

    pipeline.register_model(ModelId("model1".to_string()), Box::new(model1));
    pipeline.register_model(ModelId("model2".to_string()), Box::new(model2));

    // Switch active model
    pipeline
        .set_active_model(&ModelId("model2".to_string()))
        .unwrap();
    assert_eq!(pipeline.active_model, Some(ModelId("model2".to_string())));

    // Try to switch to non-existent model
    let result = pipeline.set_active_model(&ModelId("non-existent".to_string()));
    assert!(result.is_err());
}

#[test]
fn test_statistical_analyzer_creation() {
    let analyzer = StatisticalAnalyzer::new();
    let stats = analyzer.get_descriptive_statistics();
    assert_eq!(stats.mean, 0.0);
    assert_eq!(stats.median, 0.0);
    assert_eq!(stats.standard_deviation, 0.0);
}

#[test]
fn test_descriptive_statistics_calculation() {
    let mut analyzer = StatisticalAnalyzer::new();
    let data = create_sample_data_series();

    let stats = analyzer.calculate_descriptive_statistics(&data).unwrap();

    assert!(stats.mean > 0.0);
    assert!(stats.median > 0.0);
    assert!(stats.standard_deviation > 0.0);
    assert!(stats.min > 0.0);
    assert!(stats.max > 0.0);
    assert!(stats.range > 0.0);
}

#[test]
fn test_confidence_interval_calculation() {
    let mut analyzer = StatisticalAnalyzer::new();
    let data = create_sample_data_series();

    let ci = analyzer.calculate_confidence_interval(&data, 0.95).unwrap();

    assert!(ci.confidence_level == 0.95);
    assert!(ci.lower_bound < ci.upper_bound);
    assert!(ci.lower_bound > 0.0);
    assert!(ci.upper_bound > 0.0);
}

#[test]
fn test_time_series_analysis() {
    let mut analyzer = StatisticalAnalyzer::new();
    let data = create_time_series_data();

    let analysis = analyzer.analyze_time_series(&data).unwrap();

    // Check trend analysis
    assert!(analysis.trend.trend_strength >= 0.0);
    assert!(analysis.trend.trend_strength <= 1.0);

    // Check seasonality analysis
    assert!(analysis.seasonality.seasonal_strength >= 0.0);
    assert!(analysis.seasonality.seasonal_strength <= 1.0);

    // Check stationarity test
    assert!(analysis.stationarity.test_statistic != 0.0);

    // Check autocorrelation
    assert!(!analysis.autocorrelation.autocorrelation_function.is_empty());
    assert!(!analysis
        .autocorrelation
        .partial_autocorrelation_function
        .is_empty());
}

#[test]
fn test_anomaly_detector_creation() {
    let detector = AnomalyDetector::new();
    let stats = detector.get_performance_stats();
    assert_eq!(stats.total_detections, 0);
    assert_eq!(stats.total_alerts, 0);
    assert_eq!(stats.active_alerts, 0);
}

#[test]
fn test_anomaly_detector_algorithm_management() {
    let mut detector = AnomalyDetector::new();

    // Add mock algorithm
    let algorithm = MockAnomalyAlgorithm::new("test-algorithm".to_string());
    detector.add_algorithm(Box::new(algorithm));

    let stats = detector.get_performance_stats();
    assert!(stats.algorithm_stats.contains_key("test-algorithm"));
}

#[test]
fn test_threshold_management() {
    let mut detector = AnomalyDetector::new();

    // Set static threshold
    detector.set_threshold("test-algorithm", 0.8).unwrap();

    // Set adaptive threshold
    let adaptive_config = AdaptiveThreshold {
        base_threshold: 0.7,
        sensitivity: 0.5,
        learning_rate: 0.1,
        min_threshold: 0.3,
        max_threshold: 0.9,
        recent_scores: std::collections::VecDeque::new(),
        window_size: 100,
    };

    detector
        .set_adaptive_threshold("test-algorithm", adaptive_config)
        .unwrap();
}

#[test]
fn test_alert_system_creation() {
    let alert_system = AlertSystem::new();
    let history = alert_system.get_alert_history();
    assert!(history.is_empty());
}

#[test]
fn test_alert_rule_management() {
    let mut alert_system = AlertSystem::new();

    let rule = AlertRule {
        id: "test-rule".to_string(),
        name: "Test Alert Rule".to_string(),
        condition: AlertCondition::ScoreAboveThreshold { threshold: 0.8 },
        severity: AnomalySeverity::High,
        cooldown_period: Duration::from_secs(60),
        enabled: true,
        notification_channels: Vec::new(),
    };

    alert_system.add_alert_rule(rule);

    // Test alert processing
    let anomaly_result = create_sample_anomaly_result();
    let alerts = alert_system.process_anomaly(&anomaly_result).unwrap();

    // Should trigger alert if score > 0.8
    if anomaly_result.anomaly_score.score > 0.8 {
        assert!(!alerts.is_empty());
    }
}

#[test]
fn test_forecasting_engine_creation() {
    let engine = ForecastingEngine::new();
    assert_eq!(engine.list_models().len(), 0);
    assert!(engine.active_model.is_none());
}

#[test]
fn test_forecasting_engine_model_registration() {
    let mut engine = ForecastingEngine::new();

    let model = LinearRegressionModel::new();
    engine.register_model("linear-regression".to_string(), Box::new(model));

    assert_eq!(engine.list_models().len(), 1);
    assert_eq!(engine.active_model, Some("linear-regression".to_string()));
}

#[test]
fn test_linear_regression_forecasting() {
    let mut engine = ForecastingEngine::new();

    let model = LinearRegressionModel::new();
    engine.register_model("linear-regression".to_string(), Box::new(model));

    // Train model
    let training_data = create_time_series_data();
    engine.train_active_model(&training_data).unwrap();

    // Generate forecast
    let forecast = engine.forecast(&training_data, 5).unwrap();

    assert_eq!(forecast.forecast_horizon, 5);
    assert_eq!(forecast.predictions.len(), 5);
    assert_eq!(forecast.confidence_intervals.len(), 5);
    assert_eq!(forecast.model_used, "Linear Regression");

    // Check that predictions are reasonable
    for prediction in &forecast.predictions {
        assert!(prediction.value.is_finite());
        assert!(prediction.lower_bound <= prediction.value);
        assert!(prediction.upper_bound >= prediction.value);
    }
}

#[test]
fn test_moving_average_forecasting() {
    let mut engine = ForecastingEngine::new();

    let model = MovingAverageModel::new(5);
    engine.register_model("moving-average".to_string(), Box::new(model));

    // Train model
    let training_data = create_time_series_data();
    engine.train_active_model(&training_data).unwrap();

    // Generate forecast
    let forecast = engine.forecast(&training_data, 3).unwrap();

    assert_eq!(forecast.forecast_horizon, 3);
    assert_eq!(forecast.predictions.len(), 3);
    assert_eq!(forecast.model_used, "Moving Average");
}

#[test]
fn test_model_validation() {
    let mut engine = ForecastingEngine::new();

    let model = LinearRegressionModel::new();
    engine.register_model("linear-regression".to_string(), Box::new(model));

    // Train model
    let training_data = create_time_series_data();
    engine.train_active_model(&training_data).unwrap();

    // Validate model
    let validation_data = create_validation_data();
    let validation_result = engine
        .validate_model("linear-regression", &validation_data)
        .unwrap();

    assert!(validation_result.metrics.mae >= 0.0);
    assert!(validation_result.metrics.mse >= 0.0);
    assert!(validation_result.metrics.rmse >= 0.0);
    assert_eq!(
        validation_result.predictions.len(),
        validation_result.actual_values.len()
    );
    assert_eq!(
        validation_result.residuals.len(),
        validation_result.actual_values.len()
    );
}

#[test]
fn test_algorithm_registry_creation() {
    let registry = AlgorithmRegistry::new();
    assert_eq!(registry.get_algorithm_count(), 0);
    assert_eq!(registry.get_category_count(), 0);
}

#[test]
fn test_algorithm_registration() {
    let mut registry = AlgorithmRegistry::new();

    let algorithm = MockAlgorithm::new("test-algorithm".to_string());
    registry.register_algorithm(Box::new(algorithm)).unwrap();

    assert_eq!(registry.get_algorithm_count(), 1);
    assert_eq!(registry.get_category_count(), 1);
}

#[test]
fn test_algorithm_search() {
    let mut registry = AlgorithmRegistry::new();

    let algorithm = MockAlgorithm::new("linear-regression".to_string());
    registry.register_algorithm(Box::new(algorithm)).unwrap();

    // Search for algorithms
    let results = registry.search_algorithms("linear");
    assert!(!results.is_empty());

    let results = registry.search_algorithms("regression");
    assert!(!results.is_empty());

    let results = registry.search_algorithms("nonexistent");
    assert!(results.is_empty());
}

#[test]
fn test_algorithm_execution() {
    let mut registry = AlgorithmRegistry::new();

    let algorithm = MockAlgorithm::new("test-algorithm".to_string());
    registry.register_algorithm(Box::new(algorithm)).unwrap();

    // Execute algorithm
    let data = create_sample_data_series();
    let parameters = HashMap::new();
    let result = registry
        .execute_algorithm(
            &AlgorithmId("test-algorithm".to_string()),
            &data,
            &parameters,
        )
        .unwrap();

    assert!(result.success);
    assert_eq!(
        result.algorithm_id,
        AlgorithmId("test-algorithm".to_string())
    );
    assert!(result.execution_time > Duration::ZERO);
}

#[test]
fn test_plugin_loading() {
    let mut registry = AlgorithmRegistry::new();

    // Load plugin (simulated)
    registry.load_plugin("/path/to/plugin.so").unwrap();

    // In a real implementation, this would verify the plugin was loaded
    // For now, we just verify no error occurred
}

#[test]
fn test_performance_monitoring() {
    let mut registry = AlgorithmRegistry::new();

    let algorithm = MockAlgorithm::new("test-algorithm".to_string());
    registry.register_algorithm(Box::new(algorithm)).unwrap();

    // Execute algorithm multiple times
    let data = create_sample_data_series();
    let parameters = HashMap::new();

    for _ in 0..5 {
        registry
            .execute_algorithm(
                &AlgorithmId("test-algorithm".to_string()),
                &data,
                &parameters,
            )
            .unwrap();
    }

    // Check performance statistics
    let stats = registry.get_performance_stats(&AlgorithmId("test-algorithm".to_string()));
    assert!(stats.is_some());

    let stats = stats.unwrap();
    assert!(stats.execution_time > Duration::ZERO);
}

#[test]
fn test_validation_rules() {
    let mut registry = AlgorithmRegistry::new();

    // Add validation rule
    let rule = BasicValidationRule::new("basic-validation".to_string());
    registry.add_validation_rule(Box::new(rule));

    // Try to register invalid algorithm
    let invalid_algorithm = MockAlgorithm::new("".to_string()); // Empty name
    let result = registry.register_algorithm(Box::new(invalid_algorithm));
    assert!(result.is_err());
}

#[test]
fn test_algorithm_categories() {
    let mut registry = AlgorithmRegistry::new();

    let algorithm = MockAlgorithm::new("test-algorithm".to_string());
    registry.register_algorithm(Box::new(algorithm)).unwrap();

    // List algorithms by category
    let algorithms = registry.list_algorithms_by_category(&AlgorithmCategory::Statistical);
    assert!(!algorithms.is_empty());

    let algorithms = registry.list_algorithms_by_category(&AlgorithmCategory::MachineLearning);
    assert!(algorithms.is_empty());
}

// Helper functions for creating test data

fn create_sample_data_series() -> DataSeries {
    let mut data_points = Vec::new();
    for i in 0..100 {
        data_points.push(DataPoint {
            timestamp: Some(Instant::now().elapsed().as_secs() + i as u64),
            value: (i as f64 * 0.1) + (i as f64 * 0.05).sin(),
            metadata: HashMap::new(),
        });
    }

    DataSeries {
        id: "test-series".to_string(),
        name: "Test Data Series".to_string(),
        data_points,
        metadata: HashMap::new(),
    }
}

fn create_time_series_data() -> DataSeries {
    let mut data_points = Vec::new();
    for i in 0..50 {
        let trend = i as f64 * 0.1;
        let seasonal = (i as f64 * 0.2).sin() * 0.5;
        let noise = (i as f64 * 0.3).cos() * 0.1;
        let value = trend + seasonal + noise;

        data_points.push(DataPoint {
            timestamp: Some(Instant::now().elapsed().as_secs() + i as u64),
            value,
            metadata: HashMap::new(),
        });
    }

    DataSeries {
        id: "time-series".to_string(),
        name: "Time Series Data".to_string(),
        data_points,
        metadata: HashMap::new(),
    }
}

fn create_validation_data() -> DataSeries {
    let mut data_points = Vec::new();
    for i in 0..20 {
        data_points.push(DataPoint {
            timestamp: Some(Instant::now().elapsed().as_secs() + i as u64),
            value: (i as f64 * 0.15) + (i as f64 * 0.1).cos(),
            metadata: HashMap::new(),
        });
    }

    DataSeries {
        id: "validation-series".to_string(),
        name: "Validation Data".to_string(),
        data_points,
        metadata: HashMap::new(),
    }
}

fn create_sample_anomaly_result() -> AnomalyResult {
    AnomalyResult {
        data_point: DataPoint {
            timestamp: Some(Instant::now().elapsed().as_secs()),
            value: 10.0,
            metadata: HashMap::new(),
        },
        anomaly_score: AnomalyScore {
            score: 0.9,
            normalized_score: 0.9,
            threshold: 0.5,
            severity: AnomalySeverity::High,
            algorithm_used: "test-algorithm".to_string(),
        },
        is_anomaly: true,
        confidence: 0.9,
        explanation: "Test anomaly".to_string(),
        detected_at: Instant::now(),
    }
}

// Mock implementations for testing

struct MockMLModel {
    id: ModelId,
    name: String,
}

impl MockMLModel {
    fn new(name: String) -> Self {
        Self {
            id: ModelId(name.clone()),
            name,
        }
    }
}

impl MLModel for MockMLModel {
    fn predict(&self, _features: &HashMap<String, f64>) -> Result<f64, AnalyticsError> {
        Ok(1.0)
    }

    fn train(&mut self, _training_data: &[DataSeries]) -> Result<(), AnalyticsError> {
        Ok(())
    }

    fn get_model_info(&self) -> ModelInfo {
        ModelInfo {
            id: self.id.clone(),
            name: self.name.clone(),
            model_type: ModelType::LinearRegression,
            accuracy: Some(0.95),
            training_time: Some(Duration::from_millis(100)),
            last_trained: Some(Instant::now()),
            parameters: HashMap::new(),
        }
    }

    fn save_model(&self) -> Result<Vec<u8>, AnalyticsError> {
        Ok(vec![1, 2, 3, 4])
    }

    fn load_model(&mut self, _data: &[u8]) -> Result<(), AnalyticsError> {
        Ok(())
    }
}

struct MockAnomalyAlgorithm {
    name: String,
}

impl MockAnomalyAlgorithm {
    fn new(name: String) -> Self {
        Self { name }
    }
}

impl AnomalyAlgorithm for MockAnomalyAlgorithm {
    fn detect_anomalies(&self, _data: &DataSeries) -> Result<Vec<AnomalyResult>, AnomalyError> {
        Ok(vec![create_sample_anomaly_result()])
    }

    fn detect_anomaly(
        &self,
        data_point: &DataPoint,
        _context: &[DataPoint],
    ) -> Result<AnomalyScore, AnomalyError> {
        Ok(AnomalyScore {
            score: if data_point.value > 5.0 { 0.9 } else { 0.1 },
            normalized_score: if data_point.value > 5.0 { 0.9 } else { 0.1 },
            threshold: 0.5,
            severity: if data_point.value > 5.0 {
                AnomalySeverity::High
            } else {
                AnomalySeverity::Low
            },
            algorithm_used: self.name.clone(),
        })
    }

    fn train(&mut self, _training_data: &DataSeries) -> Result<(), AnomalyError> {
        Ok(())
    }

    fn get_algorithm_info(&self) -> AlgorithmInfo {
        AlgorithmInfo {
            name: self.name.clone(),
            description: "Mock anomaly detection algorithm".to_string(),
            algorithm_type: AlgorithmType::Statistical,
            parameters: HashMap::new(),
            performance_metrics: PerformanceMetrics {
                precision: 0.95,
                recall: 0.90,
                f1_score: 0.92,
                accuracy: 0.93,
                false_positive_rate: 0.05,
                false_negative_rate: 0.10,
                processing_time: Duration::from_millis(10),
            },
        }
    }

    fn update_threshold(&mut self, _threshold: f64) -> Result<(), AnomalyError> {
        Ok(())
    }
}

struct MockAlgorithm {
    name: String,
}

impl MockAlgorithm {
    fn new(name: String) -> Self {
        Self { name }
    }
}

impl Algorithm for MockAlgorithm {
    fn get_metadata(&self) -> &AlgorithmMetadata {
        // This would normally be stored in the struct, but for testing we'll create it
        static METADATA: std::sync::OnceLock<AlgorithmMetadata> = std::sync::OnceLock::new();
        METADATA.get_or_init(|| AlgorithmMetadata {
            id: AlgorithmId("test-algorithm".to_string()),
            name: "Test Algorithm".to_string(),
            description: "A test algorithm for unit testing".to_string(),
            version: "1.0.0".to_string(),
            author: "Test Author".to_string(),
            category: AlgorithmCategory::Statistical,
            tags: vec!["test".to_string(), "mock".to_string()],
            parameters: HashMap::new(),
            dependencies: Vec::new(),
            performance_characteristics: PerformanceCharacteristics {
                time_complexity: "O(n)".to_string(),
                space_complexity: "O(1)".to_string(),
                typical_execution_time: Duration::from_millis(10),
                memory_usage: 1024,
                scalability: ScalabilityInfo {
                    max_data_size: Some(1000000),
                    recommended_data_size: Some(10000),
                    performance_degradation: 0.1,
                    bottleneck: None,
                },
                parallelizable: true,
            },
            compatibility: CompatibilityInfo {
                data_types: vec!["numeric".to_string()],
                data_formats: vec!["csv".to_string(), "json".to_string()],
                minimum_data_points: 1,
                maximum_data_points: None,
                required_features: Vec::new(),
                platform_requirements: Vec::new(),
            },
            documentation_url: None,
            source_url: None,
            license: None,
            created_at: Instant::now(),
            updated_at: Instant::now(),
        })
    }

    fn validate_parameters(
        &self,
        _parameters: &HashMap<String, ParameterValue>,
    ) -> Result<(), RegistryError> {
        Ok(())
    }

    fn execute(
        &self,
        _data: &DataSeries,
        _parameters: &HashMap<String, ParameterValue>,
    ) -> Result<AlgorithmResult, RegistryError> {
        Ok(AlgorithmResult {
            algorithm_id: AlgorithmId("test-algorithm".to_string()),
            execution_time: Duration::from_millis(5),
            success: true,
            result_data: ResultData::Custom(HashMap::from([(
                "result".to_string(),
                "success".to_string(),
            )])),
            metadata: HashMap::new(),
            performance_metrics: PerformanceMetrics {
                execution_time: Duration::from_millis(5),
                memory_usage: 512,
                cpu_usage: 0.1,
                accuracy: Some(0.95),
                precision: Some(0.90),
                recall: Some(0.85),
                f1_score: Some(0.87),
            },
        })
    }

    fn get_parameter_schema(&self) -> HashMap<String, ParameterInfo> {
        HashMap::new()
    }
}
