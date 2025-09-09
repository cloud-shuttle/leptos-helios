//! Forecasting Engine
//!
//! This module provides comprehensive forecasting capabilities for Helios visualizations,
//! including time series forecasting, trend analysis, and predictive analytics.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use thiserror::Error;

use crate::advanced_analytics::{AnalyticsError, DataPoint, DataSeries};

/// Forecasting errors
#[derive(Debug, Error)]
pub enum ForecastingError {
    #[error("Forecasting model error: {message}")]
    ModelError { message: String },

    #[error("Data preprocessing error: {message}")]
    PreprocessingError { message: String },

    #[error("Prediction error: {message}")]
    PredictionError { message: String },

    #[error("Model training error: {message}")]
    TrainingError { message: String },

    #[error("Validation error: {message}")]
    ValidationError { message: String },
}

/// Forecasting model trait
pub trait ForecastingModel: Send + Sync {
    fn predict(&self, data: &DataSeries, horizon: u32) -> Result<ForecastResult, ForecastingError>;
    fn train(&mut self, training_data: &DataSeries) -> Result<(), ForecastingError>;
    fn validate(&self, validation_data: &DataSeries) -> Result<ValidationResult, ForecastingError>;
    fn get_model_info(&self) -> ForecastingModelInfo;
    fn update(&mut self, new_data: &DataPoint) -> Result<(), ForecastingError>;
}

/// Forecasting model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastingModelInfo {
    pub name: String,
    pub model_type: ForecastingModelType,
    pub parameters: HashMap<String, f64>,
    pub performance_metrics: ForecastingMetrics,
    pub training_time: Option<Duration>,
    pub last_trained: Option<Instant>,
}

/// Forecasting model types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForecastingModelType {
    LinearRegression,
    PolynomialRegression,
    ExponentialSmoothing,
    ARIMA,
    SARIMA,
    LSTM,
    GRU,
    Prophet,
    SeasonalNaive,
    MovingAverage,
    HoltWinters,
    Custom(String),
}

/// Forecasting metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastingMetrics {
    pub mae: f64,      // Mean Absolute Error
    pub mse: f64,      // Mean Squared Error
    pub rmse: f64,     // Root Mean Squared Error
    pub mape: f64,     // Mean Absolute Percentage Error
    pub smape: f64,    // Symmetric Mean Absolute Percentage Error
    pub r2_score: f64, // R-squared
    pub aic: f64,      // Akaike Information Criterion
    pub bic: f64,      // Bayesian Information Criterion
}

/// Forecast result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastResult {
    pub predictions: Vec<ForecastPoint>,
    pub confidence_intervals: Vec<ConfidenceInterval>,
    pub model_used: String,
    pub forecast_horizon: u32,
    pub generated_at: Instant,
    pub metadata: HashMap<String, String>,
}

/// Forecast point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastPoint {
    pub timestamp: u64,
    pub value: f64,
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub confidence_level: f64,
}

/// Confidence interval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceInterval {
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub confidence_level: f64,
}

/// Validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub metrics: ForecastingMetrics,
    pub predictions: Vec<ForecastPoint>,
    pub actual_values: Vec<f64>,
    pub residuals: Vec<f64>,
    pub validation_period: Duration,
}

/// Forecasting engine
#[derive(Debug)]
pub struct ForecastingEngine {
    models: HashMap<String, Box<dyn ForecastingModel>>,
    active_model: Option<String>,
    preprocessing_pipeline: PreprocessingPipeline,
    postprocessing_pipeline: PostprocessingPipeline,
    performance_monitor: ForecastingPerformanceMonitor,
}

/// Preprocessing pipeline for forecasting
pub struct PreprocessingPipeline {
    steps: Vec<Box<dyn PreprocessingStep>>,
}

/// Preprocessing step trait
pub trait PreprocessingStep: Send + Sync {
    fn process(&self, data: &mut DataSeries) -> Result<(), ForecastingError>;
    fn get_step_info(&self) -> PreprocessingStepInfo;
}

/// Preprocessing step information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreprocessingStepInfo {
    pub name: String,
    pub description: String,
    pub parameters: HashMap<String, f64>,
}

/// Postprocessing pipeline for forecasting
pub struct PostprocessingPipeline {
    steps: Vec<Box<dyn PostprocessingStep>>,
}

/// Postprocessing step trait
pub trait PostprocessingStep: Send + Sync {
    fn process(&self, forecast: &mut ForecastResult) -> Result<(), ForecastingError>;
    fn get_step_info(&self) -> PostprocessingStepInfo;
}

/// Postprocessing step information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostprocessingStepInfo {
    pub name: String,
    pub description: String,
    pub parameters: HashMap<String, f64>,
}

/// Forecasting performance monitor
#[derive(Debug)]
pub struct ForecastingPerformanceMonitor {
    prediction_times: VecDeque<Duration>,
    accuracy_history: VecDeque<f64>,
    model_performance: HashMap<String, Vec<ForecastingMetrics>>,
    last_reset: Instant,
}

impl ForecastingEngine {
    /// Create a new forecasting engine
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
            active_model: None,
            preprocessing_pipeline: PreprocessingPipeline { steps: Vec::new() },
            postprocessing_pipeline: PostprocessingPipeline { steps: Vec::new() },
            performance_monitor: ForecastingPerformanceMonitor {
                prediction_times: VecDeque::new(),
                accuracy_history: VecDeque::new(),
                model_performance: HashMap::new(),
                last_reset: Instant::now(),
            },
        }
    }

    /// Register a forecasting model
    pub fn register_model(&mut self, name: String, model: Box<dyn ForecastingModel>) {
        self.models.insert(name.clone(), model);
        if self.active_model.is_none() {
            self.active_model = Some(name);
        }
    }

    /// Set active model
    pub fn set_active_model(&mut self, name: &str) -> Result<(), ForecastingError> {
        if self.models.contains_key(name) {
            self.active_model = Some(name.to_string());
            Ok(())
        } else {
            Err(ForecastingError::ModelError {
                message: format!("Model {} not found", name),
            })
        }
    }

    /// Add preprocessing step
    pub fn add_preprocessing_step(&mut self, step: Box<dyn PreprocessingStep>) {
        self.preprocessing_pipeline.steps.push(step);
    }

    /// Add postprocessing step
    pub fn add_postprocessing_step(&mut self, step: Box<dyn PostprocessingStep>) {
        self.postprocessing_pipeline.steps.push(step);
    }

    /// Train active model
    pub fn train_active_model(
        &mut self,
        training_data: &DataSeries,
    ) -> Result<(), ForecastingError> {
        let model_name =
            self.active_model
                .as_ref()
                .ok_or_else(|| ForecastingError::ModelError {
                    message: "No active model set".to_string(),
                })?;

        let model =
            self.models
                .get_mut(model_name)
                .ok_or_else(|| ForecastingError::ModelError {
                    message: format!("Active model {} not found", model_name),
                })?;

        // Preprocess training data
        let mut processed_data = training_data.clone();
        for step in &self.preprocessing_pipeline.steps {
            step.process(&mut processed_data)?;
        }

        // Train model
        model.train(&processed_data)?;

        Ok(())
    }

    /// Generate forecast
    pub fn forecast(
        &self,
        data: &DataSeries,
        horizon: u32,
    ) -> Result<ForecastResult, ForecastingError> {
        let start_time = Instant::now();

        let model_name =
            self.active_model
                .as_ref()
                .ok_or_else(|| ForecastingError::ModelError {
                    message: "No active model set".to_string(),
                })?;

        let model = self
            .models
            .get(model_name)
            .ok_or_else(|| ForecastingError::ModelError {
                message: format!("Active model {} not found", model_name),
            })?;

        // Preprocess data
        let mut processed_data = data.clone();
        for step in &self.preprocessing_pipeline.steps {
            step.process(&mut processed_data)?;
        }

        // Generate forecast
        let mut forecast = model.predict(&processed_data, horizon)?;

        // Postprocess forecast
        for step in &self.postprocessing_pipeline.steps {
            step.process(&mut forecast)?;
        }

        // Record performance
        let prediction_time = start_time.elapsed();
        // Note: In a real implementation, we'd update the performance monitor here

        Ok(forecast)
    }

    /// Validate model
    pub fn validate_model(
        &self,
        model_name: &str,
        validation_data: &DataSeries,
    ) -> Result<ValidationResult, ForecastingError> {
        let model = self
            .models
            .get(model_name)
            .ok_or_else(|| ForecastingError::ModelError {
                message: format!("Model {} not found", model_name),
            })?;

        // Preprocess validation data
        let mut processed_data = validation_data.clone();
        for step in &self.preprocessing_pipeline.steps {
            step.process(&mut processed_data)?;
        }

        model.validate(&processed_data)
    }

    /// Update model with new data point
    pub fn update_model(
        &mut self,
        model_name: &str,
        new_data: &DataPoint,
    ) -> Result<(), ForecastingError> {
        let model =
            self.models
                .get_mut(model_name)
                .ok_or_else(|| ForecastingError::ModelError {
                    message: format!("Model {} not found", model_name),
                })?;

        model.update(new_data)
    }

    /// Get model information
    pub fn get_model_info(&self, model_name: &str) -> Option<&ForecastingModelInfo> {
        self.models
            .get(model_name)
            .map(|model| model.get_model_info())
    }

    /// List all models
    pub fn list_models(&self) -> Vec<&ForecastingModelInfo> {
        self.models
            .values()
            .map(|model| model.get_model_info())
            .collect()
    }

    /// Get performance statistics
    pub fn get_performance_stats(&self) -> ForecastingPerformanceStats {
        let avg_prediction_time = if !self.performance_monitor.prediction_times.is_empty() {
            self.performance_monitor
                .prediction_times
                .iter()
                .sum::<Duration>()
                / self.performance_monitor.prediction_times.len() as u32
        } else {
            Duration::ZERO
        };

        let avg_accuracy = if !self.performance_monitor.accuracy_history.is_empty() {
            self.performance_monitor
                .accuracy_history
                .iter()
                .sum::<f64>()
                / self.performance_monitor.accuracy_history.len() as f64
        } else {
            0.0
        };

        ForecastingPerformanceStats {
            total_models: self.models.len(),
            active_model: self.active_model.clone(),
            average_prediction_time: avg_prediction_time,
            average_accuracy: avg_accuracy,
            model_performance: self.performance_monitor.model_performance.clone(),
        }
    }
}

/// Forecasting performance statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastingPerformanceStats {
    pub total_models: usize,
    pub active_model: Option<String>,
    pub average_prediction_time: Duration,
    pub average_accuracy: f64,
    pub model_performance: HashMap<String, Vec<ForecastingMetrics>>,
}

/// Simple linear regression forecasting model
#[derive(Debug)]
pub struct LinearRegressionModel {
    slope: f64,
    intercept: f64,
    r_squared: f64,
    last_trained: Option<Instant>,
    training_time: Option<Duration>,
}

impl LinearRegressionModel {
    /// Create a new linear regression model
    pub fn new() -> Self {
        Self {
            slope: 0.0,
            intercept: 0.0,
            r_squared: 0.0,
            last_trained: None,
            training_time: None,
        }
    }

    /// Calculate linear regression parameters
    fn calculate_parameters(&mut self, data: &DataSeries) -> Result<(), ForecastingError> {
        let values: Vec<f64> = data.data_points.iter().map(|dp| dp.value).collect();

        if values.len() < 2 {
            return Err(ForecastingError::TrainingError {
                message: "Need at least 2 data points for linear regression".to_string(),
            });
        }

        let n = values.len() as f64;
        let x_mean = (n - 1.0) / 2.0;
        let y_mean = values.iter().sum::<f64>() / n;

        let mut numerator = 0.0;
        let mut denominator = 0.0;

        for (i, &y) in values.iter().enumerate() {
            let x = i as f64;
            numerator += (x - x_mean) * (y - y_mean);
            denominator += (x - x_mean).powi(2);
        }

        self.slope = if denominator != 0.0 {
            numerator / denominator
        } else {
            0.0
        };
        self.intercept = y_mean - self.slope * x_mean;

        // Calculate R-squared
        let mut ss_res = 0.0;
        let mut ss_tot = 0.0;

        for (i, &y) in values.iter().enumerate() {
            let x = i as f64;
            let y_pred = self.slope * x + self.intercept;
            ss_res += (y - y_pred).powi(2);
            ss_tot += (y - y_mean).powi(2);
        }

        self.r_squared = if ss_tot != 0.0 {
            1.0 - (ss_res / ss_tot)
        } else {
            0.0
        };

        Ok(())
    }
}

impl ForecastingModel for LinearRegressionModel {
    fn predict(&self, data: &DataSeries, horizon: u32) -> Result<ForecastResult, ForecastingError> {
        let values: Vec<f64> = data.data_points.iter().map(|dp| dp.value).collect();

        if values.is_empty() {
            return Err(ForecastingError::PredictionError {
                message: "Cannot predict from empty data".to_string(),
            });
        }

        let mut predictions = Vec::new();
        let mut confidence_intervals = Vec::new();

        // Calculate standard error for confidence intervals
        let n = values.len() as f64;
        let residual_variance = if n > 2.0 {
            let mut ss_res = 0.0;
            let y_mean = values.iter().sum::<f64>() / n;

            for (i, &y) in values.iter().enumerate() {
                let x = i as f64;
                let y_pred = self.slope * x + self.intercept;
                ss_res += (y - y_pred).powi(2);
            }

            ss_res / (n - 2.0)
        } else {
            1.0
        };

        let standard_error = residual_variance.sqrt();

        // Generate predictions
        for i in 0..horizon {
            let x = (values.len() + i) as f64;
            let prediction = self.slope * x + self.intercept;

            // Calculate confidence interval (simplified)
            let margin_of_error = 1.96 * standard_error; // 95% confidence
            let lower_bound = prediction - margin_of_error;
            let upper_bound = prediction + margin_of_error;

            predictions.push(ForecastPoint {
                timestamp: Instant::now().elapsed().as_secs() + i as u64,
                value: prediction,
                lower_bound,
                upper_bound,
                confidence_level: 0.95,
            });

            confidence_intervals.push(ConfidenceInterval {
                lower_bound,
                upper_bound,
                confidence_level: 0.95,
            });
        }

        Ok(ForecastResult {
            predictions,
            confidence_intervals,
            model_used: "Linear Regression".to_string(),
            forecast_horizon: horizon,
            generated_at: Instant::now(),
            metadata: HashMap::from([
                ("slope".to_string(), self.slope.to_string()),
                ("intercept".to_string(), self.intercept.to_string()),
                ("r_squared".to_string(), self.r_squared.to_string()),
            ]),
        })
    }

    fn train(&mut self, training_data: &DataSeries) -> Result<(), ForecastingError> {
        let start_time = Instant::now();

        self.calculate_parameters(training_data)?;

        self.training_time = Some(start_time.elapsed());
        self.last_trained = Some(Instant::now());

        Ok(())
    }

    fn validate(&self, validation_data: &DataSeries) -> Result<ValidationResult, ForecastingError> {
        let values: Vec<f64> = validation_data
            .data_points
            .iter()
            .map(|dp| dp.value)
            .collect();

        if values.is_empty() {
            return Err(ForecastingError::ValidationError {
                message: "Cannot validate with empty data".to_string(),
            });
        }

        let mut predictions = Vec::new();
        let mut actual_values = values.clone();
        let mut residuals = Vec::new();

        // Generate predictions for validation
        for (i, &actual) in values.iter().enumerate() {
            let x = i as f64;
            let prediction = self.slope * x + self.intercept;
            let residual = actual - prediction;

            predictions.push(ForecastPoint {
                timestamp: Instant::now().elapsed().as_secs() + i as u64,
                value: prediction,
                lower_bound: prediction - 1.0,
                upper_bound: prediction + 1.0,
                confidence_level: 0.95,
            });

            residuals.push(residual);
        }

        // Calculate metrics
        let n = values.len() as f64;
        let mae = residuals.iter().map(|&r| r.abs()).sum::<f64>() / n;
        let mse = residuals.iter().map(|&r| r.powi(2)).sum::<f64>() / n;
        let rmse = mse.sqrt();

        let mape = if !values.iter().any(|&v| v == 0.0) {
            residuals
                .iter()
                .zip(values.iter())
                .map(|(&r, &actual)| (r / actual).abs())
                .sum::<f64>()
                / n
                * 100.0
        } else {
            0.0
        };

        let smape = residuals
            .iter()
            .zip(values.iter())
            .map(|(&r, &actual)| {
                r.abs() / ((actual.abs() + predictions.iter().map(|p| p.value).sum::<f64>()) / 2.0)
            })
            .sum::<f64>()
            / n
            * 100.0;

        let y_mean = values.iter().sum::<f64>() / n;
        let ss_tot = values.iter().map(|&y| (y - y_mean).powi(2)).sum::<f64>();
        let ss_res = residuals.iter().map(|&r| r.powi(2)).sum::<f64>();
        let r2_score = if ss_tot != 0.0 {
            1.0 - (ss_res / ss_tot)
        } else {
            0.0
        };

        let metrics = ForecastingMetrics {
            mae,
            mse,
            rmse,
            mape,
            smape,
            r2_score,
            aic: n * (mse.ln() + 2.0 * 2.0 / n), // Simplified AIC
            bic: n * (mse.ln() + 2.0 * 2.0 * (n.ln()) / n), // Simplified BIC
        };

        Ok(ValidationResult {
            metrics,
            predictions,
            actual_values,
            residuals,
            validation_period: Duration::from_secs(values.len() as u64),
        })
    }

    fn get_model_info(&self) -> ForecastingModelInfo {
        ForecastingModelInfo {
            name: "Linear Regression".to_string(),
            model_type: ForecastingModelType::LinearRegression,
            parameters: HashMap::from([
                ("slope".to_string(), self.slope),
                ("intercept".to_string(), self.intercept),
                ("r_squared".to_string(), self.r_squared),
            ]),
            performance_metrics: ForecastingMetrics {
                mae: 0.0,
                mse: 0.0,
                rmse: 0.0,
                mape: 0.0,
                smape: 0.0,
                r2_score: self.r_squared,
                aic: 0.0,
                bic: 0.0,
            },
            training_time: self.training_time,
            last_trained: self.last_trained,
        }
    }

    fn update(&mut self, new_data: &DataPoint) -> Result<(), ForecastingError> {
        // Simple online update - in practice, this would be more sophisticated
        // For now, we'll just update the intercept based on the new data point
        self.intercept = (self.intercept + new_data.value) / 2.0;
        Ok(())
    }
}

/// Simple moving average forecasting model
#[derive(Debug)]
pub struct MovingAverageModel {
    window_size: usize,
    values: VecDeque<f64>,
    last_trained: Option<Instant>,
    training_time: Option<Duration>,
}

impl MovingAverageModel {
    /// Create a new moving average model
    pub fn new(window_size: usize) -> Self {
        Self {
            window_size,
            values: VecDeque::new(),
            last_trained: None,
            training_time: None,
        }
    }
}

impl ForecastingModel for MovingAverageModel {
    fn predict(&self, data: &DataSeries, horizon: u32) -> Result<ForecastResult, ForecastingError> {
        let values: Vec<f64> = data.data_points.iter().map(|dp| dp.value).collect();

        if values.is_empty() {
            return Err(ForecastingError::PredictionError {
                message: "Cannot predict from empty data".to_string(),
            });
        }

        let mut predictions = Vec::new();
        let mut confidence_intervals = Vec::new();

        // Calculate moving average
        let recent_values = if values.len() >= self.window_size {
            &values[values.len() - self.window_size..]
        } else {
            &values
        };

        let average = recent_values.iter().sum::<f64>() / recent_values.len() as f64;
        let variance = recent_values
            .iter()
            .map(|&x| (x - average).powi(2))
            .sum::<f64>()
            / recent_values.len() as f64;
        let standard_error = variance.sqrt();

        // Generate predictions
        for i in 0..horizon {
            let margin_of_error = 1.96 * standard_error;
            let lower_bound = average - margin_of_error;
            let upper_bound = average + margin_of_error;

            predictions.push(ForecastPoint {
                timestamp: Instant::now().elapsed().as_secs() + i as u64,
                value: average,
                lower_bound,
                upper_bound,
                confidence_level: 0.95,
            });

            confidence_intervals.push(ConfidenceInterval {
                lower_bound,
                upper_bound,
                confidence_level: 0.95,
            });
        }

        Ok(ForecastResult {
            predictions,
            confidence_intervals,
            model_used: "Moving Average".to_string(),
            forecast_horizon: horizon,
            generated_at: Instant::now(),
            metadata: HashMap::from([
                ("window_size".to_string(), self.window_size.to_string()),
                ("average".to_string(), average.to_string()),
            ]),
        })
    }

    fn train(&mut self, training_data: &DataSeries) -> Result<(), ForecastingError> {
        let start_time = Instant::now();

        self.values.clear();
        for data_point in &training_data.data_points {
            self.values.push_back(data_point.value);
            if self.values.len() > self.window_size {
                self.values.pop_front();
            }
        }

        self.training_time = Some(start_time.elapsed());
        self.last_trained = Some(Instant::now());

        Ok(())
    }

    fn validate(&self, validation_data: &DataSeries) -> Result<ValidationResult, ForecastingError> {
        let values: Vec<f64> = validation_data
            .data_points
            .iter()
            .map(|dp| dp.value)
            .collect();

        if values.is_empty() {
            return Err(ForecastingError::ValidationError {
                message: "Cannot validate with empty data".to_string(),
            });
        }

        let mut predictions = Vec::new();
        let mut actual_values = values.clone();
        let mut residuals = Vec::new();

        // Generate predictions for validation
        for (i, &actual) in values.iter().enumerate() {
            let prediction = if i >= self.window_size {
                let window_values = &values[i - self.window_size..i];
                window_values.iter().sum::<f64>() / window_values.len() as f64
            } else {
                values[..i].iter().sum::<f64>() / i.max(1) as f64
            };

            let residual = actual - prediction;

            predictions.push(ForecastPoint {
                timestamp: Instant::now().elapsed().as_secs() + i as u64,
                value: prediction,
                lower_bound: prediction - 1.0,
                upper_bound: prediction + 1.0,
                confidence_level: 0.95,
            });

            residuals.push(residual);
        }

        // Calculate metrics
        let n = values.len() as f64;
        let mae = residuals.iter().map(|&r| r.abs()).sum::<f64>() / n;
        let mse = residuals.iter().map(|&r| r.powi(2)).sum::<f64>() / n;
        let rmse = mse.sqrt();

        let metrics = ForecastingMetrics {
            mae,
            mse,
            rmse,
            mape: 0.0,     // Simplified
            smape: 0.0,    // Simplified
            r2_score: 0.0, // Simplified
            aic: 0.0,      // Simplified
            bic: 0.0,      // Simplified
        };

        Ok(ValidationResult {
            metrics,
            predictions,
            actual_values,
            residuals,
            validation_period: Duration::from_secs(values.len() as u64),
        })
    }

    fn get_model_info(&self) -> ForecastingModelInfo {
        ForecastingModelInfo {
            name: "Moving Average".to_string(),
            model_type: ForecastingModelType::MovingAverage,
            parameters: HashMap::from([("window_size".to_string(), self.window_size as f64)]),
            performance_metrics: ForecastingMetrics {
                mae: 0.0,
                mse: 0.0,
                rmse: 0.0,
                mape: 0.0,
                smape: 0.0,
                r2_score: 0.0,
                aic: 0.0,
                bic: 0.0,
            },
            training_time: self.training_time,
            last_trained: self.last_trained,
        }
    }

    fn update(&mut self, new_data: &DataPoint) -> Result<(), ForecastingError> {
        self.values.push_back(new_data.value);
        if self.values.len() > self.window_size {
            self.values.pop_front();
        }
        Ok(())
    }
}

impl Default for ForecastingEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for LinearRegressionModel {
    fn default() -> Self {
        Self::new()
    }
}
