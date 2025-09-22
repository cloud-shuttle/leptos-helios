//! Common types and data structures for advanced analytics

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Model identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModelId(pub String);

/// Algorithm identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AlgorithmId(pub String);

/// Data point for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    pub timestamp: Option<u64>,
    pub value: f64,
    pub metadata: HashMap<String, String>,
}

/// Data series for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSeries {
    pub id: String,
    pub name: String,
    pub data_points: Vec<DataPoint>,
    pub metadata: HashMap<String, String>,
}

/// Feature extraction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureExtractionResult {
    pub features: HashMap<String, f64>,
    pub feature_names: Vec<String>,
    pub extraction_time: Duration,
}

/// Model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: ModelId,
    pub name: String,
    pub model_type: ModelType,
    pub accuracy: Option<f64>,
    pub training_time: Option<Duration>,
    pub parameters: HashMap<String, f64>,
    pub metadata: HashMap<String, String>,
}

/// Model types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelType {
    LinearRegression,
    PolynomialRegression,
    RandomForest,
    SupportVectorMachine,
    NeuralNetwork,
    DecisionTree,
    KMeans,
    DBSCAN,
    GaussianMixture,
    Custom(String),
}

/// Feature extractor information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureExtractorInfo {
    pub name: String,
    pub description: String,
    pub feature_count: usize,
    pub supported_data_types: Vec<String>,
}

/// Preprocessing step information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreprocessingStepInfo {
    pub name: String,
    pub description: String,
    pub parameters: HashMap<String, f64>,
}

/// Postprocessing step information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostprocessingStepInfo {
    pub name: String,
    pub description: String,
    pub parameters: HashMap<String, f64>,
}
