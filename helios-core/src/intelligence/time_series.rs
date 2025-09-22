//! Time Series Forecasting Module

use super::MLError;
use crate::DataFrame;
// use polars::prelude::*; // Currently unused
// use std::collections::HashMap; // Currently unused

/// Time series forecaster for trend analysis and prediction
pub struct TimeSeriesForecaster {
    /// Size of the sliding window for analysis
    pub window_size: usize,
    /// Forecast horizon (number of future points to predict)
    pub forecast_horizon: usize,
    /// Trend analysis results
    pub trend_analysis: Option<TrendAnalysis>,
    /// Seasonality analysis results
    pub seasonality_analysis: Option<SeasonalityAnalysis>,
}

/// Trend analysis results
#[derive(Debug, Clone)]
pub struct TrendAnalysis {
    /// Type of trend detected
    pub trend_type: TrendType,
    /// Trend strength (0.0 to 1.0)
    pub strength: f64,
    /// Trend direction
    pub direction: TrendDirection,
    /// Trend equation coefficients
    pub coefficients: Vec<f64>,
}

/// Types of trends
#[derive(Debug, Clone, PartialEq)]
pub enum TrendType {
    /// Linear trend
    Linear,
    /// Exponential trend
    Exponential,
    /// Logarithmic trend
    Logarithmic,
    /// No significant trend
    None,
}

/// Trend direction
#[derive(Debug, Clone, PartialEq)]
pub enum TrendDirection {
    /// Increasing trend
    Increasing,
    /// Decreasing trend
    Decreasing,
    /// Stable trend
    Stable,
}

/// Seasonality analysis results
#[derive(Debug, Clone)]
pub struct SeasonalityAnalysis {
    /// Whether seasonality was detected
    pub has_seasonality: bool,
    /// Seasonal period
    pub period: Option<usize>,
    /// Seasonal strength
    pub strength: f64,
    /// Seasonal pattern
    pub pattern: Vec<f64>,
}

/// Time series decomposition results
#[derive(Debug, Clone)]
pub struct TimeSeriesDecomposition {
    /// Trend component
    pub trend: Vec<f64>,
    /// Seasonal component
    pub seasonal: Vec<f64>,
    /// Residual component
    pub residual: Vec<f64>,
    /// Original time series
    pub original: Vec<f64>,
}

impl TimeSeriesForecaster {
    /// Create a new time series forecaster
    pub fn new(window_size: usize, forecast_horizon: usize) -> Self {
        Self {
            window_size,
            forecast_horizon,
            trend_analysis: None,
            seasonality_analysis: None,
        }
    }

    /// Analyze time series data for trends and seasonality
    pub fn analyze(&mut self, data: &DataFrame) -> Result<(), MLError> {
        if data.height() < self.window_size {
            return Err(MLError::InsufficientData(
                format!(
                    "Need at least {} data points for analysis",
                    self.window_size
                ),
            ));
        }

        // Extract time series values
        let values = self.extract_values(data)?;

        // Analyze trend
        self.trend_analysis = Some(self.analyze_trend(&values)?);

        // Analyze seasonality
        self.seasonality_analysis = Some(self.analyze_seasonality(&values)?);

        Ok(())
    }

    /// Generate forecasts based on analysis
    pub fn forecast(&self, data: &DataFrame) -> Result<Vec<f64>, MLError> {
        if self.trend_analysis.is_none() {
            return Err(MLError::ModelNotTrained);
        }

        let values = self.extract_values(data)?;
        let mut forecasts = Vec::with_capacity(self.forecast_horizon);

        // Simple forecasting based on trend and seasonality
        for i in 0..self.forecast_horizon {
            let forecast = self.generate_single_forecast(&values, i)?;
            forecasts.push(forecast);
        }

        Ok(forecasts)
    }

    /// Decompose time series into components
    pub fn decompose(&self, data: &DataFrame) -> Result<TimeSeriesDecomposition, MLError> {
        let values = self.extract_values(data)?;
        let _n = values.len();

        // Simple decomposition using moving averages
        let trend = self.calculate_trend_component(&values);
        let seasonal = self.calculate_seasonal_component(&values);
        let residual = self.calculate_residual_component(&values, &trend, &seasonal);

        Ok(TimeSeriesDecomposition {
            trend,
            seasonal,
            residual,
            original: values,
        })
    }

    /// Extract values from DataFrame
    fn extract_values(&self, data: &DataFrame) -> Result<Vec<f64>, MLError> {
        // Assume first numeric column contains the time series values
        let columns = data.get_columns();
        for column in columns {
            if let Ok(series) = column.f64() {
                return Ok(series.into_iter().filter_map(|v| v).collect());
            }
        }

        Err(MLError::InvalidData(
            "No numeric column found for time series analysis".to_string(),
        ))
    }

    /// Analyze trend in the time series
    fn analyze_trend(&self, values: &[f64]) -> Result<TrendAnalysis, MLError> {
        if values.len() < 2 {
            return Err(MLError::InsufficientData(
                "Need at least 2 points for trend analysis".to_string(),
            ));
        }

        // Calculate linear regression
        let (slope, intercept, r_squared) = self.calculate_linear_regression(values);

        // Determine trend type and direction
        let trend_type = if r_squared > 0.7 {
            TrendType::Linear
        } else {
            TrendType::None
        };

        let direction = if slope > 0.01 {
            TrendDirection::Increasing
        } else if slope < -0.01 {
            TrendDirection::Decreasing
        } else {
            TrendDirection::Stable
        };

        Ok(TrendAnalysis {
            trend_type,
            strength: r_squared,
            direction,
            coefficients: vec![intercept, slope],
        })
    }

    /// Analyze seasonality in the time series
    fn analyze_seasonality(&self, values: &[f64]) -> Result<SeasonalityAnalysis, MLError> {
        let n = values.len();
        if n < 12 {
            return Ok(SeasonalityAnalysis {
                has_seasonality: false,
                period: None,
                strength: 0.0,
                pattern: Vec::new(),
            });
        }

        // Simple seasonality detection using autocorrelation
        let max_period = (n / 4).min(24);
        let mut best_period = None;
        let mut best_strength = 0.0;

        for period in 2..=max_period {
            let strength = self.calculate_seasonal_strength(values, period);
            if strength > best_strength {
                best_strength = strength;
                best_period = Some(period);
            }
        }

        let has_seasonality = best_strength > 0.3;
        let pattern = if has_seasonality {
            self.extract_seasonal_pattern(values, best_period.unwrap())
        } else {
            Vec::new()
        };

        Ok(SeasonalityAnalysis {
            has_seasonality,
            period: best_period,
            strength: best_strength,
            pattern,
        })
    }

    /// Calculate linear regression
    fn calculate_linear_regression(&self, values: &[f64]) -> (f64, f64, f64) {
        let n = values.len() as f64;
        let x_values: Vec<f64> = (0..values.len()).map(|i| i as f64).collect();

        let sum_x: f64 = x_values.iter().sum();
        let sum_y: f64 = values.iter().sum();
        let sum_xy: f64 = x_values.iter().zip(values.iter()).map(|(a, b)| a * b).sum();
        let sum_x2: f64 = x_values.iter().map(|a| a * a).sum();
        let _sum_y2: f64 = values.iter().map(|a| a * a).sum();

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
        let intercept = (sum_y - slope * sum_x) / n;

        // Calculate R-squared
        let y_mean = sum_y / n;
        let ss_tot: f64 = values.iter().map(|yi| (yi - y_mean).powi(2)).sum();
        let ss_res: f64 = x_values
            .iter()
            .zip(values.iter())
            .map(|(xi, yi)| (yi - (slope * xi + intercept)).powi(2))
            .sum();
        let r_squared = 1.0 - (ss_res / ss_tot);

        (slope, intercept, r_squared)
    }

    /// Calculate seasonal strength
    fn calculate_seasonal_strength(&self, values: &[f64], period: usize) -> f64 {
        let n = values.len();
        if n < period * 2 {
            return 0.0;
        }

        let mut autocorr_sum = 0.0;
        let mut count = 0;

        for i in 0..(n - period) {
            autocorr_sum += values[i] * values[i + period];
            count += 1;
        }

        if count == 0 {
            return 0.0;
        }

        autocorr_sum / count as f64
    }

    /// Extract seasonal pattern
    fn extract_seasonal_pattern(&self, values: &[f64], period: usize) -> Vec<f64> {
        let mut pattern = vec![0.0; period];
        let mut counts = vec![0; period];

        for (i, &value) in values.iter().enumerate() {
            let phase = i % period;
            pattern[phase] += value;
            counts[phase] += 1;
        }

        for i in 0..period {
            if counts[i] > 0 {
                pattern[i] /= counts[i] as f64;
            }
        }

        pattern
    }

    /// Generate single forecast
    fn generate_single_forecast(&self, values: &[f64], step: usize) -> Result<f64, MLError> {
        let trend = &self.trend_analysis.as_ref().unwrap();
        let seasonal = &self.seasonality_analysis.as_ref().unwrap();

        // Base forecast from trend
        let n = values.len();
        let trend_forecast = trend.coefficients[0] + trend.coefficients[1] * (n + step) as f64;

        // Add seasonal component if present
        let seasonal_component = if seasonal.has_seasonality {
            let period = seasonal.period.unwrap_or(12);
            seasonal.pattern[(n + step) % period]
        } else {
            0.0
        };

        Ok(trend_forecast + seasonal_component)
    }

    /// Calculate trend component
    fn calculate_trend_component(&self, values: &[f64]) -> Vec<f64> {
        let window_size = (values.len() / 10).max(3).min(20);
        let mut trend = Vec::new();

        for i in 0..values.len() {
            let start = i.saturating_sub(window_size / 2);
            let end = (i + window_size / 2 + 1).min(values.len());
            let window_sum: f64 = values[start..end].iter().sum();
            let window_avg = window_sum / (end - start) as f64;
            trend.push(window_avg);
        }

        trend
    }

    /// Calculate seasonal component
    fn calculate_seasonal_component(&self, values: &[f64]) -> Vec<f64> {
        let seasonal = &self.seasonality_analysis.as_ref().unwrap();
        let mut seasonal_component = vec![0.0; values.len()];

        if seasonal.has_seasonality {
            let period = seasonal.period.unwrap_or(12);
            for i in 0..values.len() {
                let phase = i % period;
                seasonal_component[i] = seasonal.pattern[phase];
            }
        }

        seasonal_component
    }

    /// Calculate residual component
    fn calculate_residual_component(
        &self,
        values: &[f64],
        trend: &[f64],
        seasonal: &[f64],
    ) -> Vec<f64> {
        values
            .iter()
            .zip(trend.iter())
            .zip(seasonal.iter())
            .map(|((&original, &trend_val), &seasonal_val)| {
                original - trend_val - seasonal_val
            })
            .collect()
    }
}
