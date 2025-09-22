//! Machine Learning Models and Pipelines

use super::types::*;
use super::AnalyticsError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Machine learning model trait
pub trait MLModel: Send + Sync {
    fn predict(&self, features: &HashMap<String, f64>) -> Result<f64, AnalyticsError>;
    fn train(&mut self, training_data: &[DataSeries]) -> Result<(), AnalyticsError>;
    fn get_model_info(&self) -> ModelInfo;
    fn save_model(&self) -> Result<Vec<u8>, AnalyticsError>;
    fn load_model(&mut self, data: &[u8]) -> Result<(), AnalyticsError>;
}

/// Feature extractor trait
pub trait FeatureExtractor: Send + Sync {
    fn extract_features(
        &self,
        data: &DataSeries,
    ) -> Result<FeatureExtractionResult, AnalyticsError>;
    fn get_feature_names(&self) -> Vec<String>;
    fn get_extractor_info(&self) -> FeatureExtractorInfo;
}

/// Preprocessing pipeline
pub struct PreprocessingPipeline {
    steps: Vec<Box<dyn PreprocessingStep>>,
}

/// Preprocessing step trait
pub trait PreprocessingStep: Send + Sync {
    fn process(&self, data: &mut DataSeries) -> Result<(), AnalyticsError>;
    fn get_step_info(&self) -> PreprocessingStepInfo;
}

/// Postprocessing pipeline
pub struct PostprocessingPipeline {
    steps: Vec<Box<dyn PostprocessingStep>>,
}

/// Postprocessing step trait
pub trait PostprocessingStep: Send + Sync {
    fn process(
        &self,
        prediction: f64,
        features: &HashMap<String, f64>,
    ) -> Result<f64, AnalyticsError>;
    fn get_step_info(&self) -> PostprocessingStepInfo;
}

/// Machine learning pipeline
pub struct MLPipeline {
    models: HashMap<ModelId, Box<dyn MLModel>>,
    feature_extractors: Vec<Box<dyn FeatureExtractor>>,
    preprocessing: PreprocessingPipeline,
    postprocessing: PostprocessingPipeline,
    active_model: Option<ModelId>,
}

impl MLPipeline {
    /// Create a new ML pipeline
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
            feature_extractors: Vec::new(),
            preprocessing: PreprocessingPipeline { steps: Vec::new() },
            postprocessing: PostprocessingPipeline { steps: Vec::new() },
            active_model: None,
        }
    }

    /// Add a model to the pipeline
    pub fn add_model(&mut self, model_id: ModelId, model: Box<dyn MLModel>) {
        self.models.insert(model_id.clone(), model);
        if self.active_model.is_none() {
            self.active_model = Some(model_id);
        }
    }

    /// Set the active model
    pub fn set_active_model(&mut self, model_id: ModelId) -> Result<(), AnalyticsError> {
        if self.models.contains_key(&model_id) {
            self.active_model = Some(model_id);
            Ok(())
        } else {
            Err(AnalyticsError::MLModelError {
                message: format!("Model {} not found", model_id.0),
            })
        }
    }

    /// Add a feature extractor
    pub fn add_feature_extractor(&mut self, extractor: Box<dyn FeatureExtractor>) {
        self.feature_extractors.push(extractor);
    }

    /// Add a preprocessing step
    pub fn add_preprocessing_step(&mut self, step: Box<dyn PreprocessingStep>) {
        self.preprocessing.steps.push(step);
    }

    /// Add a postprocessing step
    pub fn add_postprocessing_step(&mut self, step: Box<dyn PostprocessingStep>) {
        self.postprocessing.steps.push(step);
    }

    /// Train the active model
    pub fn train(&mut self, training_data: &[DataSeries]) -> Result<(), AnalyticsError> {
        let active_model_id = self.active_model.as_ref()
            .ok_or_else(|| AnalyticsError::MLModelError {
                message: "No active model set".to_string(),
            })?;

        let model = self.models.get_mut(active_model_id)
            .ok_or_else(|| AnalyticsError::MLModelError {
                message: format!("Active model {} not found", active_model_id.0),
            })?;

        // Apply preprocessing
        let mut processed_data = training_data.to_vec();
        for step in &self.preprocessing.steps {
            for data in &mut processed_data {
                step.process(data)?;
            }
        }

        model.train(&processed_data)
    }

    /// Make a prediction
    pub fn predict(&self, data: &DataSeries) -> Result<f64, AnalyticsError> {
        let active_model_id = self.active_model.as_ref()
            .ok_or_else(|| AnalyticsError::MLModelError {
                message: "No active model set".to_string(),
            })?;

        let model = self.models.get(active_model_id)
            .ok_or_else(|| AnalyticsError::MLModelError {
                message: format!("Active model {} not found", active_model_id.0),
            })?;

        // Extract features
        let mut features = HashMap::new();
        for extractor in &self.feature_extractors {
            let result = extractor.extract_features(data)?;
            features.extend(result.features);
        }

        // Make prediction
        let prediction = model.predict(&features)?;

        // Apply postprocessing
        let mut final_prediction = prediction;
        for step in &self.postprocessing.steps {
            final_prediction = step.process(final_prediction, &features)?;
        }

        Ok(final_prediction)
    }

    /// Get pipeline information
    pub fn get_pipeline_info(&self) -> PipelineInfo {
        PipelineInfo {
            model_count: self.models.len(),
            feature_extractor_count: self.feature_extractors.len(),
            preprocessing_step_count: self.preprocessing.steps.len(),
            postprocessing_step_count: self.postprocessing.steps.len(),
            active_model: self.active_model.clone(),
        }
    }
}

/// Pipeline information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineInfo {
    pub model_count: usize,
    pub feature_extractor_count: usize,
    pub preprocessing_step_count: usize,
    pub postprocessing_step_count: usize,
    pub active_model: Option<ModelId>,
}

impl Default for MLPipeline {
    fn default() -> Self {
        Self::new()
    }
}
