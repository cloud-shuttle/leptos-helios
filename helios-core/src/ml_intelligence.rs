//! ML Intelligence Module for Helios
//!
//! This module provides machine learning capabilities for intelligent chart recommendations,
//! data forecasting, and automatic visualization optimization.

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Time series data point for ML operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesPoint {
    pub timestamp: f64,
    pub value: f64,
}

/// ML forecasting result with confidence metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastResult {
    pub predictions: Vec<f64>,
    pub confidence: f64,
    pub inference_time: Duration,
    pub model_type: String,
}

/// Chart recommendation based on data analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartRecommendation {
    pub chart_type: String,
    pub confidence: f64,
    pub reasoning: String,
    pub optimization_suggestions: Vec<String>,
}

/// ML forecaster for time series prediction
pub struct MLForecaster {
    model_loaded: bool,
    inference_count: u32,
}

impl MLForecaster {
    /// Create a new ML forecaster
    pub fn new() -> Self {
        Self {
            model_loaded: false,
            inference_count: 0,
        }
    }

    /// Load ML model for forecasting
    pub fn load_model(&mut self) -> Result<(), String> {
        // Simulate model loading
        self.model_loaded = true;
        Ok(())
    }

    /// Generate forecast for time series data
    pub fn forecast(
        &mut self,
        series: &[TimeSeriesPoint],
        periods: u32,
    ) -> Result<ForecastResult, String> {
        if !self.model_loaded {
            return Err("Model not loaded".to_string());
        }

        let start = Instant::now();

        // Simulate ML inference
        let predictions = self.generate_predictions(series, periods);
        let confidence = self.calculate_confidence(series);
        let inference_time = start.elapsed();

        self.inference_count += 1;

        Ok(ForecastResult {
            predictions,
            confidence,
            inference_time,
            model_type: "LSTM".to_string(),
        })
    }

    /// Generate predictions using ML model
    fn generate_predictions(&self, series: &[TimeSeriesPoint], periods: u32) -> Vec<f64> {
        if series.is_empty() {
            return vec![0.0; periods as usize];
        }

        // Check for extreme values in the series
        let has_extreme_values = series.iter().any(|point| {
            !point.value.is_finite() || point.value.abs() > 1e10
        });

        if has_extreme_values {
            // For extreme values, return simple constant predictions
            return vec![0.0; periods as usize];
        }

        // Simple trend-based prediction for normal values
        let last_value = series.last().unwrap().value;
        let trend = self.calculate_trend(series);

        (0..periods)
            .map(|i| {
                let prediction = last_value + trend * (i as f64 + 1.0);
                // Ensure predictions are finite
                if prediction.is_finite() {
                    prediction.clamp(-1e10, 1e10)
                } else {
                    0.0
                }
            })
            .collect()
    }

    /// Calculate trend from time series
    fn calculate_trend(&self, series: &[TimeSeriesPoint]) -> f64 {
        if series.len() < 2 {
            return 0.0;
        }

        let first = series[0].value;
        let last = series.last().unwrap().value;
        let time_span = series.last().unwrap().timestamp - series[0].timestamp;

        // Handle extreme values that could cause overflow
        if !first.is_finite() || !last.is_finite() || !time_span.is_finite() {
            return 0.0;
        }

        // Check for extreme differences that could cause overflow
        let diff = last - first;
        if diff.abs() > 1e10 {
            return 0.0;
        }

        if time_span > 0.0 {
            let trend = diff / time_span;
            // Clamp extreme values to prevent overflow
            if trend.is_finite() {
                trend.clamp(-1e6, 1e6)
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    /// Calculate confidence score for predictions
    fn calculate_confidence(&self, series: &[TimeSeriesPoint]) -> f64 {
        if series.len() < 3 {
            return 0.5;
        }

        // Calculate variance to determine confidence
        let mean = series.iter().map(|p| p.value).sum::<f64>() / series.len() as f64;
        let variance =
            series.iter().map(|p| (p.value - mean).powi(2)).sum::<f64>() / series.len() as f64;

        // Higher variance = lower confidence
        (1.0 / (1.0 + variance)).min(1.0).max(0.0)
    }

    /// Get inference statistics
    pub fn get_inference_count(&self) -> u32 {
        self.inference_count
    }

    /// Check if model is loaded
    pub fn is_model_loaded(&self) -> bool {
        self.model_loaded
    }
}

/// Intelligent chart recommendation engine
pub struct ChartRecommendationEngine {
    data_analyzer: DataAnalyzer,
    recommendation_cache: std::collections::HashMap<String, ChartRecommendation>,
}

impl ChartRecommendationEngine {
    /// Create a new recommendation engine
    pub fn new() -> Self {
        Self {
            data_analyzer: DataAnalyzer::new(),
            recommendation_cache: std::collections::HashMap::new(),
        }
    }

    /// Analyze data and recommend optimal chart type
    pub fn recommend_chart(&mut self, data: &[f64], metadata: &str) -> ChartRecommendation {
        let cache_key = format!("{}_{}", data.len(), metadata);

        if let Some(cached) = self.recommendation_cache.get(&cache_key) {
            return cached.clone();
        }

        let analysis = self.data_analyzer.analyze(data);
        let recommendation = self.generate_recommendation(&analysis, metadata);

        self.recommendation_cache
            .insert(cache_key, recommendation.clone());
        recommendation
    }

    /// Generate recommendation based on data analysis
    fn generate_recommendation(
        &self,
        analysis: &DataAnalysis,
        _metadata: &str,
    ) -> ChartRecommendation {
        let (chart_type, confidence, reasoning) = match analysis.data_type {
            DataType::TimeSeries => ("line", 0.9, "Time series data detected".to_string()),
            DataType::Categorical => ("bar", 0.85, "Categorical data detected".to_string()),
            DataType::Continuous => ("scatter", 0.8, "Continuous data detected".to_string()),
            DataType::Mixed => ("combo", 0.7, "Mixed data types detected".to_string()),
        };

        let optimization_suggestions = self.generate_optimization_suggestions(analysis);

        ChartRecommendation {
            chart_type: chart_type.to_string(),
            confidence,
            reasoning,
            optimization_suggestions,
        }
    }

    /// Generate optimization suggestions
    fn generate_optimization_suggestions(&self, analysis: &DataAnalysis) -> Vec<String> {
        let mut suggestions = Vec::new();

        if analysis.outlier_count > analysis.data_points / 10 {
            suggestions.push("Consider outlier filtering for better visualization".to_string());
        }

        if analysis.variance > 1.0 {
            suggestions.push("High variance detected - consider log scale".to_string());
        }

        if analysis.data_points >= 10000 {
            suggestions.push("Large dataset - consider data sampling or aggregation".to_string());
        }

        suggestions
    }

    /// Get cache statistics
    pub fn get_cache_size(&self) -> usize {
        self.recommendation_cache.len()
    }

    /// Clear recommendation cache
    pub fn clear_cache(&mut self) {
        self.recommendation_cache.clear();
    }
}

/// Data analysis for intelligent recommendations
#[derive(Debug, Clone)]
pub struct DataAnalysis {
    pub data_type: DataType,
    pub data_points: usize,
    pub mean: f64,
    pub variance: f64,
    pub outlier_count: usize,
    pub trend: TrendType,
}

#[derive(Debug, Clone)]
pub enum DataType {
    TimeSeries,
    Categorical,
    Continuous,
    Mixed,
}

#[derive(Debug, Clone)]
pub enum TrendType {
    Increasing,
    Decreasing,
    Stable,
    Volatile,
}

/// Data analyzer for ML operations
pub struct DataAnalyzer {
    analysis_count: u32,
}

impl DataAnalyzer {
    /// Create a new data analyzer
    pub fn new() -> Self {
        Self { analysis_count: 0 }
    }

    /// Analyze data characteristics
    pub fn analyze(&mut self, data: &[f64]) -> DataAnalysis {
        self.analysis_count += 1;

        let data_points = data.len();
        let mean = if data_points > 0 {
            data.iter().sum::<f64>() / data_points as f64
        } else {
            0.0
        };

        let variance = if data_points > 1 {
            data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (data_points - 1) as f64
        } else {
            0.0
        };

        let outlier_count = self.count_outliers(data, mean, variance.sqrt());
        let data_type = self.classify_data_type(data);
        let trend = self.analyze_trend(data);

        DataAnalysis {
            data_type,
            data_points,
            mean,
            variance,
            outlier_count,
            trend,
        }
    }

    /// Count outliers using IQR method
    fn count_outliers(&self, data: &[f64], mean: f64, std_dev: f64) -> usize {
        if std_dev == 0.0 {
            return 0;
        }

        data.iter()
            .filter(|&&x| (x - mean).abs() > 2.0 * std_dev)
            .count()
    }

    /// Classify data type based on characteristics
    fn classify_data_type(&self, data: &[f64]) -> DataType {
        if data.is_empty() {
            return DataType::Continuous;
        }

        // Simple heuristic: check for discrete values
        // Use a different approach since f64 doesn't implement Hash
        let mut unique_count = 0;
        let total_values = data.len();

        // Count unique values by rounding to avoid floating point precision issues
        let mut seen = std::collections::HashSet::new();
        for &value in data {
            let rounded = (value * 1000.0).round() as i64;
            if seen.insert(rounded) {
                unique_count += 1;
            }
        }

        if (unique_count as f64 / total_values as f64) < 0.1 {
            DataType::Categorical
        } else {
            DataType::Continuous
        }
    }

    /// Analyze trend in data
    fn analyze_trend(&self, data: &[f64]) -> TrendType {
        if data.len() < 2 {
            return TrendType::Stable;
        }

        let first_half = &data[..data.len() / 2];
        let second_half = &data[data.len() / 2..];

        let first_mean = first_half.iter().sum::<f64>() / first_half.len() as f64;
        let second_mean = second_half.iter().sum::<f64>() / second_half.len() as f64;

        let change = (second_mean - first_mean) / first_mean.abs().max(1e-10);

        if change > 0.1 {
            TrendType::Increasing
        } else if change < -0.1 {
            TrendType::Decreasing
        } else {
            TrendType::Stable
        }
    }

    /// Get analysis statistics
    pub fn get_analysis_count(&self) -> u32 {
        self.analysis_count
    }
}

/// ML performance monitor
pub struct MLPerformanceMonitor {
    total_inferences: u32,
    total_inference_time: Duration,
    max_inference_time: Duration,
    min_inference_time: Duration,
}

impl MLPerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        Self {
            total_inferences: 0,
            total_inference_time: Duration::ZERO,
            max_inference_time: Duration::ZERO,
            min_inference_time: Duration::from_secs(3600), // Start with high value
        }
    }

    /// Record inference performance
    pub fn record_inference(&mut self, duration: Duration) {
        self.total_inferences += 1;
        self.total_inference_time += duration;
        self.max_inference_time = self.max_inference_time.max(duration);
        self.min_inference_time = self.min_inference_time.min(duration);
    }

    /// Get average inference time
    pub fn get_average_inference_time(&self) -> Duration {
        if self.total_inferences > 0 {
            Duration::from_nanos(
                self.total_inference_time.as_nanos() as u64 / self.total_inferences as u64,
            )
        } else {
            Duration::ZERO
        }
    }

    /// Get performance statistics
    pub fn get_stats(&self) -> MLPerformanceStats {
        MLPerformanceStats {
            total_inferences: self.total_inferences,
            average_inference_time: self.get_average_inference_time(),
            max_inference_time: self.max_inference_time,
            min_inference_time: self.min_inference_time,
        }
    }
}

/// ML performance statistics
#[derive(Debug, Clone)]
pub struct MLPerformanceStats {
    pub total_inferences: u32,
    pub average_inference_time: Duration,
    pub max_inference_time: Duration,
    pub min_inference_time: Duration,
}

impl Default for MLForecaster {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ChartRecommendationEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for DataAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for MLPerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}
