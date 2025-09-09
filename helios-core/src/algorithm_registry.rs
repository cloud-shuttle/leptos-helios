//! Algorithm Registry System
//!
//! This module provides a comprehensive algorithm registry for Helios analytics,
//! including algorithm discovery, validation, and plugin-based algorithm system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use thiserror::Error;

use crate::advanced_analytics::{AnalyticsError, DataSeries};
use crate::anomaly_detection::{AnomalyAlgorithm, AnomalyError};
use crate::forecasting_engine::{ForecastingError, ForecastingModel};

/// Algorithm registry errors
#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("Algorithm registration error: {message}")]
    RegistrationError { message: String },

    #[error("Algorithm validation error: {message}")]
    ValidationError { message: String },

    #[error("Algorithm discovery error: {message}")]
    DiscoveryError { message: String },

    #[error("Plugin loading error: {message}")]
    PluginError { message: String },

    #[error("Algorithm execution error: {message}")]
    ExecutionError { message: String },
}

/// Algorithm identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AlgorithmId(pub String);

/// Algorithm metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmMetadata {
    pub id: AlgorithmId,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub category: AlgorithmCategory,
    pub tags: Vec<String>,
    pub parameters: HashMap<String, ParameterInfo>,
    pub dependencies: Vec<String>,
    pub performance_characteristics: PerformanceCharacteristics,
    pub compatibility: CompatibilityInfo,
    pub documentation_url: Option<String>,
    pub source_url: Option<String>,
    pub license: Option<String>,
    pub created_at: Instant,
    pub updated_at: Instant,
}

/// Algorithm categories
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlgorithmCategory {
    Statistical,
    MachineLearning,
    TimeSeries,
    AnomalyDetection,
    Forecasting,
    Clustering,
    Classification,
    Regression,
    DimensionalityReduction,
    Optimization,
    Custom(String),
}

/// Parameter information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterInfo {
    pub name: String,
    pub parameter_type: ParameterType,
    pub description: String,
    pub required: bool,
    pub default_value: Option<ParameterValue>,
    pub constraints: ParameterConstraints,
}

/// Parameter types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterType {
    Integer,
    Float,
    Boolean,
    String,
    Array,
    Object,
    Enum(Vec<String>),
}

/// Parameter value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterValue {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Array(Vec<ParameterValue>),
    Object(HashMap<String, ParameterValue>),
}

/// Parameter constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterConstraints {
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub allowed_values: Option<Vec<String>>,
    pub pattern: Option<String>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

/// Performance characteristics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceCharacteristics {
    pub time_complexity: String,
    pub space_complexity: String,
    pub typical_execution_time: Duration,
    pub memory_usage: usize,
    pub scalability: ScalabilityInfo,
    pub parallelizable: bool,
}

/// Scalability information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalabilityInfo {
    pub max_data_size: Option<usize>,
    pub recommended_data_size: Option<usize>,
    pub performance_degradation: f64,
    pub bottleneck: Option<String>,
}

/// Compatibility information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityInfo {
    pub data_types: Vec<String>,
    pub data_formats: Vec<String>,
    pub minimum_data_points: usize,
    pub maximum_data_points: Option<usize>,
    pub required_features: Vec<String>,
    pub platform_requirements: Vec<String>,
}

/// Algorithm trait
pub trait Algorithm: Send + Sync {
    fn get_metadata(&self) -> &AlgorithmMetadata;
    fn validate_parameters(
        &self,
        parameters: &HashMap<String, ParameterValue>,
    ) -> Result<(), RegistryError>;
    fn execute(
        &self,
        data: &DataSeries,
        parameters: &HashMap<String, ParameterValue>,
    ) -> Result<AlgorithmResult, RegistryError>;
    fn get_parameter_schema(&self) -> HashMap<String, ParameterInfo>;
}

/// Algorithm result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmResult {
    pub algorithm_id: AlgorithmId,
    pub execution_time: Duration,
    pub success: bool,
    pub result_data: ResultData,
    pub metadata: HashMap<String, String>,
    pub performance_metrics: PerformanceMetrics,
}

/// Result data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResultData {
    AnomalyDetection(Vec<AnomalyDetectionResult>),
    Forecasting(ForecastingResult),
    Statistical(StatisticalResult),
    Clustering(ClusteringResult),
    Classification(ClassificationResult),
    Regression(RegressionResult),
    Custom(HashMap<String, String>),
}

/// Anomaly detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyDetectionResult {
    pub data_point_index: usize,
    pub anomaly_score: f64,
    pub is_anomaly: bool,
    pub confidence: f64,
}

/// Forecasting result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastingResult {
    pub predictions: Vec<f64>,
    pub confidence_intervals: Vec<(f64, f64)>,
    pub forecast_horizon: u32,
    pub model_accuracy: f64,
}

/// Statistical result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalResult {
    pub descriptive_stats: HashMap<String, f64>,
    pub inferential_stats: HashMap<String, f64>,
    pub correlation_matrix: HashMap<String, HashMap<String, f64>>,
}

/// Clustering result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusteringResult {
    pub clusters: Vec<Cluster>,
    pub cluster_count: usize,
    pub silhouette_score: f64,
}

/// Cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cluster {
    pub id: usize,
    pub centroid: Vec<f64>,
    pub data_point_indices: Vec<usize>,
    pub size: usize,
}

/// Classification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationResult {
    pub predictions: Vec<String>,
    pub probabilities: Vec<HashMap<String, f64>>,
    pub accuracy: f64,
    pub confusion_matrix: HashMap<String, HashMap<String, usize>>,
}

/// Regression result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionResult {
    pub predictions: Vec<f64>,
    pub residuals: Vec<f64>,
    pub r_squared: f64,
    pub coefficients: HashMap<String, f64>,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub execution_time: Duration,
    pub memory_usage: usize,
    pub cpu_usage: f64,
    pub accuracy: Option<f64>,
    pub precision: Option<f64>,
    pub recall: Option<f64>,
    pub f1_score: Option<f64>,
}

/// Algorithm validator
#[derive(Debug)]
pub struct AlgorithmValidator {
    validation_rules: Vec<Box<dyn ValidationRule>>,
    performance_benchmarks: HashMap<AlgorithmId, PerformanceBenchmark>,
}

/// Validation rule trait
pub trait ValidationRule: Send + Sync {
    fn validate(&self, algorithm: &dyn Algorithm) -> Result<(), RegistryError>;
    fn get_rule_name(&self) -> String;
}

/// Performance benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBenchmark {
    pub algorithm_id: AlgorithmId,
    pub benchmark_data: DataSeries,
    pub expected_execution_time: Duration,
    pub expected_memory_usage: usize,
    pub expected_accuracy: Option<f64>,
    pub tolerance: f64,
}

/// Algorithm registry
#[derive(Debug)]
pub struct AlgorithmRegistry {
    algorithms: HashMap<AlgorithmId, Box<dyn Algorithm>>,
    metadata: HashMap<AlgorithmId, AlgorithmMetadata>,
    categories: HashMap<AlgorithmCategory, Vec<AlgorithmId>>,
    validator: AlgorithmValidator,
    performance_monitor: PerformanceMonitor,
    plugin_manager: PluginManager,
}

/// Performance monitor
#[derive(Debug)]
pub struct PerformanceMonitor {
    execution_history: HashMap<AlgorithmId, VecDeque<PerformanceMetrics>>,
    average_performance: HashMap<AlgorithmId, PerformanceMetrics>,
    last_updated: Instant,
}

/// Plugin manager
#[derive(Debug)]
pub struct PluginManager {
    loaded_plugins: HashMap<String, PluginInfo>,
    plugin_paths: Vec<String>,
    plugin_configurations: HashMap<String, HashMap<String, String>>,
}

/// Plugin information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub path: String,
    pub loaded_at: Instant,
    pub algorithms: Vec<AlgorithmId>,
    pub dependencies: Vec<String>,
    pub status: PluginStatus,
}

/// Plugin status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginStatus {
    Loaded,
    Failed,
    Disabled,
    Outdated,
}

impl AlgorithmRegistry {
    /// Create a new algorithm registry
    pub fn new() -> Self {
        Self {
            algorithms: HashMap::new(),
            metadata: HashMap::new(),
            categories: HashMap::new(),
            validator: AlgorithmValidator {
                validation_rules: Vec::new(),
                performance_benchmarks: HashMap::new(),
            },
            performance_monitor: PerformanceMonitor {
                execution_history: HashMap::new(),
                average_performance: HashMap::new(),
                last_updated: Instant::now(),
            },
            plugin_manager: PluginManager {
                loaded_plugins: HashMap::new(),
                plugin_paths: Vec::new(),
                plugin_configurations: HashMap::new(),
            },
        }
    }

    /// Register an algorithm
    pub fn register_algorithm(
        &mut self,
        algorithm: Box<dyn Algorithm>,
    ) -> Result<(), RegistryError> {
        let metadata = algorithm.get_metadata().clone();
        let algorithm_id = metadata.id.clone();

        // Validate algorithm
        for rule in &self.validator.validation_rules {
            rule.validate(algorithm.as_ref())?;
        }

        // Check for conflicts
        if self.algorithms.contains_key(&algorithm_id) {
            return Err(RegistryError::RegistrationError {
                message: format!("Algorithm {} is already registered", algorithm_id.0),
            });
        }

        // Register algorithm
        self.algorithms.insert(algorithm_id.clone(), algorithm);
        self.metadata.insert(algorithm_id.clone(), metadata.clone());

        // Add to category
        self.categories
            .entry(metadata.category.clone())
            .or_insert_with(Vec::new)
            .push(algorithm_id);

        // Initialize performance monitoring
        self.performance_monitor
            .execution_history
            .insert(algorithm_id, VecDeque::new());

        Ok(())
    }

    /// Unregister an algorithm
    pub fn unregister_algorithm(
        &mut self,
        algorithm_id: &AlgorithmId,
    ) -> Result<(), RegistryError> {
        if let Some(metadata) = self.metadata.remove(algorithm_id) {
            self.algorithms.remove(algorithm_id);

            // Remove from category
            if let Some(category_algorithms) = self.categories.get_mut(&metadata.category) {
                category_algorithms.retain(|id| id != algorithm_id);
            }

            // Remove performance monitoring
            self.performance_monitor
                .execution_history
                .remove(algorithm_id);
            self.performance_monitor
                .average_performance
                .remove(algorithm_id);

            Ok(())
        } else {
            Err(RegistryError::RegistrationError {
                message: format!("Algorithm {} not found", algorithm_id.0),
            })
        }
    }

    /// Get algorithm by ID
    pub fn get_algorithm(&self, algorithm_id: &AlgorithmId) -> Option<&dyn Algorithm> {
        self.algorithms.get(algorithm_id).map(|alg| alg.as_ref())
    }

    /// Get algorithm metadata
    pub fn get_metadata(&self, algorithm_id: &AlgorithmId) -> Option<&AlgorithmMetadata> {
        self.metadata.get(algorithm_id)
    }

    /// List algorithms by category
    pub fn list_algorithms_by_category(
        &self,
        category: &AlgorithmCategory,
    ) -> Vec<&AlgorithmMetadata> {
        self.categories
            .get(category)
            .map(|ids| ids.iter().filter_map(|id| self.metadata.get(id)).collect())
            .unwrap_or_default()
    }

    /// Search algorithms
    pub fn search_algorithms(&self, query: &str) -> Vec<&AlgorithmMetadata> {
        let query_lower = query.to_lowercase();

        self.metadata
            .values()
            .filter(|metadata| {
                metadata.name.to_lowercase().contains(&query_lower)
                    || metadata.description.to_lowercase().contains(&query_lower)
                    || metadata
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// Execute algorithm
    pub fn execute_algorithm(
        &mut self,
        algorithm_id: &AlgorithmId,
        data: &DataSeries,
        parameters: &HashMap<String, ParameterValue>,
    ) -> Result<AlgorithmResult, RegistryError> {
        let algorithm =
            self.algorithms
                .get(algorithm_id)
                .ok_or_else(|| RegistryError::ExecutionError {
                    message: format!("Algorithm {} not found", algorithm_id.0),
                })?;

        // Validate parameters
        algorithm.validate_parameters(parameters)?;

        // Execute algorithm
        let start_time = Instant::now();
        let result = algorithm.execute(data, parameters)?;
        let execution_time = start_time.elapsed();

        // Update performance monitoring
        self.update_performance_monitoring(algorithm_id, &result, execution_time);

        Ok(result)
    }

    /// Add validation rule
    pub fn add_validation_rule(&mut self, rule: Box<dyn ValidationRule>) {
        self.validator.validation_rules.push(rule);
    }

    /// Add performance benchmark
    pub fn add_performance_benchmark(&mut self, benchmark: PerformanceBenchmark) {
        self.validator
            .performance_benchmarks
            .insert(benchmark.algorithm_id.clone(), benchmark);
    }

    /// Load plugin
    pub fn load_plugin(&mut self, plugin_path: &str) -> Result<(), RegistryError> {
        // In a real implementation, this would load dynamic libraries
        // For now, we'll simulate plugin loading
        let plugin_name = std::path::Path::new(plugin_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let plugin_info = PluginInfo {
            name: plugin_name.clone(),
            version: "1.0.0".to_string(),
            path: plugin_path.to_string(),
            loaded_at: Instant::now(),
            algorithms: Vec::new(),
            dependencies: Vec::new(),
            status: PluginStatus::Loaded,
        };

        self.plugin_manager
            .loaded_plugins
            .insert(plugin_name, plugin_info);
        self.plugin_manager
            .plugin_paths
            .push(plugin_path.to_string());

        Ok(())
    }

    /// Get performance statistics
    pub fn get_performance_stats(&self, algorithm_id: &AlgorithmId) -> Option<&PerformanceMetrics> {
        self.performance_monitor
            .average_performance
            .get(algorithm_id)
    }

    /// Get all algorithms
    pub fn list_all_algorithms(&self) -> Vec<&AlgorithmMetadata> {
        self.metadata.values().collect()
    }

    /// Get algorithm count
    pub fn get_algorithm_count(&self) -> usize {
        self.algorithms.len()
    }

    /// Get category count
    pub fn get_category_count(&self) -> usize {
        self.categories.len()
    }

    /// Update performance monitoring
    fn update_performance_monitoring(
        &mut self,
        algorithm_id: &AlgorithmId,
        result: &AlgorithmResult,
        execution_time: Duration,
    ) {
        if let Some(history) = self
            .performance_monitor
            .execution_history
            .get_mut(algorithm_id)
        {
            history.push_back(result.performance_metrics.clone());

            // Keep only recent history
            if history.len() > 100 {
                history.pop_front();
            }

            // Update average performance
            let avg_metrics = PerformanceMetrics {
                execution_time: history.iter().map(|m| m.execution_time).sum::<Duration>()
                    / history.len() as u32,
                memory_usage: history.iter().map(|m| m.memory_usage).sum::<usize>() / history.len(),
                cpu_usage: history.iter().map(|m| m.cpu_usage).sum::<f64>() / history.len() as f64,
                accuracy: history
                    .iter()
                    .filter_map(|m| m.accuracy)
                    .fold(0.0, |acc, x| acc + x)
                    / history
                        .iter()
                        .filter(|m| m.accuracy.is_some())
                        .count()
                        .max(1) as f64,
                precision: history
                    .iter()
                    .filter_map(|m| m.precision)
                    .fold(0.0, |acc, x| acc + x)
                    / history
                        .iter()
                        .filter(|m| m.precision.is_some())
                        .count()
                        .max(1) as f64,
                recall: history
                    .iter()
                    .filter_map(|m| m.recall)
                    .fold(0.0, |acc, x| acc + x)
                    / history.iter().filter(|m| m.recall.is_some()).count().max(1) as f64,
                f1_score: history
                    .iter()
                    .filter_map(|m| m.f1_score)
                    .fold(0.0, |acc, x| acc + x)
                    / history
                        .iter()
                        .filter(|m| m.f1_score.is_some())
                        .count()
                        .max(1) as f64,
            };

            self.performance_monitor
                .average_performance
                .insert(algorithm_id.clone(), avg_metrics);
        }

        self.performance_monitor.last_updated = Instant::now();
    }
}

/// Basic validation rule
#[derive(Debug)]
pub struct BasicValidationRule {
    name: String,
}

impl BasicValidationRule {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl ValidationRule for BasicValidationRule {
    fn validate(&self, algorithm: &dyn Algorithm) -> Result<(), RegistryError> {
        let metadata = algorithm.get_metadata();

        // Basic validation checks
        if metadata.name.is_empty() {
            return Err(RegistryError::ValidationError {
                message: "Algorithm name cannot be empty".to_string(),
            });
        }

        if metadata.description.is_empty() {
            return Err(RegistryError::ValidationError {
                message: "Algorithm description cannot be empty".to_string(),
            });
        }

        if metadata.version.is_empty() {
            return Err(RegistryError::ValidationError {
                message: "Algorithm version cannot be empty".to_string(),
            });
        }

        Ok(())
    }

    fn get_rule_name(&self) -> String {
        self.name.clone()
    }
}

impl Default for AlgorithmRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for AlgorithmValidator {
    fn default() -> Self {
        Self {
            validation_rules: Vec::new(),
            performance_benchmarks: HashMap::new(),
        }
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self {
            execution_history: HashMap::new(),
            average_performance: HashMap::new(),
            last_updated: Instant::now(),
        }
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self {
            loaded_plugins: HashMap::new(),
            plugin_paths: Vec::new(),
            plugin_configurations: HashMap::new(),
        }
    }
}
