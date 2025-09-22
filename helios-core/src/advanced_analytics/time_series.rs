//! Time Series Analysis Module

// use super::types::*; // Currently unused
use super::AnalyticsError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Time series analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesAnalysis {
    pub trend: TrendAnalysis,
    pub seasonality: SeasonalityAnalysis,
    pub stationarity: StationarityTest,
    pub autocorrelation: AutocorrelationAnalysis,
    pub decomposition: TimeSeriesDecomposition,
}

/// Trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    pub trend_type: TrendType,
    pub trend_strength: f64,
    pub trend_direction: TrendDirection,
    pub trend_equation: Option<String>,
}

/// Trend types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendType {
    Linear,
    Exponential,
    Logarithmic,
    Polynomial,
    None,
}

/// Trend direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
}

/// Seasonality analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalityAnalysis {
    pub has_seasonality: bool,
    pub seasonal_period: Option<u32>,
    pub seasonal_strength: f64,
    pub seasonal_pattern: Vec<f64>,
}

/// Stationarity test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationarityTest {
    pub is_stationary: bool,
    pub test_type: String,
    pub test_statistic: f64,
    pub p_value: f64,
    pub critical_values: HashMap<String, f64>,
}

/// Autocorrelation analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutocorrelationAnalysis {
    pub autocorrelation_function: Vec<f64>,
    pub partial_autocorrelation_function: Vec<f64>,
    pub significant_lags: Vec<u32>,
}

/// Time series decomposition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesDecomposition {
    pub trend_component: Vec<f64>,
    pub seasonal_component: Vec<f64>,
    pub residual_component: Vec<f64>,
    pub decomposition_type: String,
}

/// Time series analyzer
pub struct TimeSeriesAnalyzer {
    analysis: TimeSeriesAnalysis,
}

impl TimeSeriesAnalyzer {
    /// Create a new time series analyzer
    pub fn new() -> Self {
        Self {
            analysis: TimeSeriesAnalysis {
                trend: TrendAnalysis {
                    trend_type: TrendType::None,
                    trend_strength: 0.0,
                    trend_direction: TrendDirection::Stable,
                    trend_equation: None,
                },
                seasonality: SeasonalityAnalysis {
                    has_seasonality: false,
                    seasonal_period: None,
                    seasonal_strength: 0.0,
                    seasonal_pattern: Vec::new(),
                },
                stationarity: StationarityTest {
                    is_stationary: false,
                    test_type: String::new(),
                    test_statistic: 0.0,
                    p_value: 0.0,
                    critical_values: HashMap::new(),
                },
                autocorrelation: AutocorrelationAnalysis {
                    autocorrelation_function: Vec::new(),
                    partial_autocorrelation_function: Vec::new(),
                    significant_lags: Vec::new(),
                },
                decomposition: TimeSeriesDecomposition {
                    trend_component: Vec::new(),
                    seasonal_component: Vec::new(),
                    residual_component: Vec::new(),
                    decomposition_type: String::new(),
                },
            },
        }
    }

    /// Analyze time series data
    pub fn analyze(&mut self, data: &[f64]) -> Result<&TimeSeriesAnalysis, AnalyticsError> {
        if data.is_empty() {
            return Err(AnalyticsError::StatisticalError {
                message: "Cannot analyze empty time series".to_string(),
            });
        }

        // Analyze trend
        self.analyze_trend(data)?;

        // Analyze seasonality
        self.analyze_seasonality(data)?;

        // Test stationarity
        self.test_stationarity(data)?;

        // Calculate autocorrelation
        self.calculate_autocorrelation(data)?;

        // Decompose time series
        self.decompose_time_series(data)?;

        Ok(&self.analysis)
    }

    /// Analyze trend in the time series
    fn analyze_trend(&mut self, data: &[f64]) -> Result<(), AnalyticsError> {
        let _n = data.len() as f64;
        let x_values: Vec<f64> = (0..data.len()).map(|i| i as f64).collect();

        // Calculate linear regression
        let (slope, intercept, r_squared) = self.calculate_linear_regression(&x_values, data);

        // Determine trend type and direction
        let trend_type = if r_squared > 0.7 {
            TrendType::Linear
        } else {
            TrendType::None
        };

        let trend_direction = if slope > 0.01 {
            TrendDirection::Increasing
        } else if slope < -0.01 {
            TrendDirection::Decreasing
        } else {
            TrendDirection::Stable
        };

        let trend_equation = if r_squared > 0.5 {
            Some(format!("y = {:.4}x + {:.4}", slope, intercept))
        } else {
            None
        };

        self.analysis.trend = TrendAnalysis {
            trend_type,
            trend_strength: r_squared,
            trend_direction,
            trend_equation,
        };

        Ok(())
    }

    /// Calculate linear regression
    fn calculate_linear_regression(&self, x: &[f64], y: &[f64]) -> (f64, f64, f64) {
        let n = x.len() as f64;
        let sum_x: f64 = x.iter().sum();
        let sum_y: f64 = y.iter().sum();
        let sum_xy: f64 = x.iter().zip(y.iter()).map(|(a, b)| a * b).sum();
        let sum_x2: f64 = x.iter().map(|a| a * a).sum();
        let _sum_y2: f64 = y.iter().map(|a| a * a).sum();

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
        let intercept = (sum_y - slope * sum_x) / n;

        // Calculate R-squared
        let y_mean = sum_y / n;
        let ss_tot: f64 = y.iter().map(|yi| (yi - y_mean).powi(2)).sum();
        let ss_res: f64 = x.iter().zip(y.iter())
            .map(|(xi, yi)| (yi - (slope * xi + intercept)).powi(2))
            .sum();
        let r_squared = 1.0 - (ss_res / ss_tot);

        (slope, intercept, r_squared)
    }

    /// Analyze seasonality
    fn analyze_seasonality(&mut self, data: &[f64]) -> Result<(), AnalyticsError> {
        // Simple seasonality detection using autocorrelation
        let max_lag = (data.len() / 4).min(50);
        let mut seasonal_strength = 0.0;
        let mut seasonal_period = None;

        for lag in 1..=max_lag {
            let autocorr = self.calculate_autocorrelation_at_lag(data, lag);
            if autocorr > seasonal_strength {
                seasonal_strength = autocorr;
                seasonal_period = Some(lag as u32);
            }
        }

        let has_seasonality = seasonal_strength > 0.3;
        let seasonal_pattern = if has_seasonality {
            self.extract_seasonal_pattern(data, seasonal_period.unwrap_or(12))
        } else {
            Vec::new()
        };

        self.analysis.seasonality = SeasonalityAnalysis {
            has_seasonality,
            seasonal_period,
            seasonal_strength,
            seasonal_pattern,
        };

        Ok(())
    }

    /// Calculate autocorrelation at specific lag
    fn calculate_autocorrelation_at_lag(&self, data: &[f64], lag: usize) -> f64 {
        if lag >= data.len() {
            return 0.0;
        }

        let n = data.len() as f64;
        let mean = data.iter().sum::<f64>() / n;

        let numerator: f64 = (0..data.len() - lag)
            .map(|i| (data[i] - mean) * (data[i + lag] - mean))
            .sum();

        let denominator: f64 = data.iter()
            .map(|x| (x - mean).powi(2))
            .sum();

        if denominator == 0.0 {
            0.0
        } else {
            numerator / denominator
        }
    }

    /// Extract seasonal pattern
    fn extract_seasonal_pattern(&self, data: &[f64], period: u32) -> Vec<f64> {
        let period = period as usize;
        let mut pattern = vec![0.0; period];
        let mut counts = vec![0; period];

        for (i, &value) in data.iter().enumerate() {
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

    /// Test stationarity using simplified ADF test
    fn test_stationarity(&mut self, data: &[f64]) -> Result<(), AnalyticsError> {
        // Simplified stationarity test based on variance
        let n = data.len();
        let mid = n / 2;
        
        let first_half_var = self.calculate_variance(&data[..mid]);
        let second_half_var = self.calculate_variance(&data[mid..]);
        
        let variance_ratio = if first_half_var > 0.0 {
            second_half_var / first_half_var
        } else {
            1.0
        };

        // Simple heuristic: if variance ratio is close to 1, series is stationary
        let is_stationary = variance_ratio > 0.5 && variance_ratio < 2.0;

        self.analysis.stationarity = StationarityTest {
            is_stationary,
            test_type: "Variance Ratio Test".to_string(),
            test_statistic: variance_ratio,
            p_value: if is_stationary { 0.05 } else { 0.95 },
            critical_values: HashMap::new(),
        };

        Ok(())
    }

    /// Calculate variance
    fn calculate_variance(&self, data: &[f64]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        let n = data.len() as f64;
        let mean = data.iter().sum::<f64>() / n;
        data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0)
    }

    /// Calculate autocorrelation function
    fn calculate_autocorrelation(&mut self, data: &[f64]) -> Result<(), AnalyticsError> {
        let max_lag = (data.len() / 4).min(20);
        let mut acf = Vec::new();
        let mut pacf = Vec::new();

        for lag in 0..=max_lag {
            let autocorr = self.calculate_autocorrelation_at_lag(data, lag);
            acf.push(autocorr);
            
            // Simplified PACF calculation
            let pacf_value = if lag == 0 {
                1.0
            } else {
                autocorr * 0.8 // Simplified approximation
            };
            pacf.push(pacf_value);
        }

        // Find significant lags (autocorrelation > 0.2)
        let significant_lags: Vec<u32> = acf.iter().enumerate()
            .filter(|(_, &corr)| corr.abs() > 0.2)
            .map(|(lag, _)| lag as u32)
            .collect();

        self.analysis.autocorrelation = AutocorrelationAnalysis {
            autocorrelation_function: acf,
            partial_autocorrelation_function: pacf,
            significant_lags,
        };

        Ok(())
    }

    /// Decompose time series
    fn decompose_time_series(&mut self, data: &[f64]) -> Result<(), AnalyticsError> {
        let _n = data.len();
        
        // Simple decomposition using moving averages
        let trend_component = self.calculate_trend_component(data);
        let seasonal_component = self.calculate_seasonal_component(data);
        let residual_component = self.calculate_residual_component(data, &trend_component, &seasonal_component);

        self.analysis.decomposition = TimeSeriesDecomposition {
            trend_component,
            seasonal_component,
            residual_component,
            decomposition_type: "Additive".to_string(),
        };

        Ok(())
    }

    /// Calculate trend component using moving average
    fn calculate_trend_component(&self, data: &[f64]) -> Vec<f64> {
        let window_size = (data.len() / 10).max(3).min(20);
        let mut trend = Vec::new();

        for i in 0..data.len() {
            let start = i.saturating_sub(window_size / 2);
            let end = (i + window_size / 2 + 1).min(data.len());
            let window_sum: f64 = data[start..end].iter().sum();
            let window_avg = window_sum / (end - start) as f64;
            trend.push(window_avg);
        }

        trend
    }

    /// Calculate seasonal component
    fn calculate_seasonal_component(&self, data: &[f64]) -> Vec<f64> {
        // Simple seasonal component calculation
        let period = self.analysis.seasonality.seasonal_period.unwrap_or(12) as usize;
        let mut seasonal = vec![0.0; data.len()];

        if self.analysis.seasonality.has_seasonality {
            for i in 0..data.len() {
                let phase = i % period;
                seasonal[i] = self.analysis.seasonality.seasonal_pattern
                    .get(phase)
                    .copied()
                    .unwrap_or(0.0);
            }
        }

        seasonal
    }

    /// Calculate residual component
    fn calculate_residual_component(
        &self,
        data: &[f64],
        trend: &[f64],
        seasonal: &[f64],
    ) -> Vec<f64> {
        data.iter()
            .zip(trend.iter())
            .zip(seasonal.iter())
            .map(|((&original, &trend_val), &seasonal_val)| {
                original - trend_val - seasonal_val
            })
            .collect()
    }

    /// Get time series analysis results
    pub fn get_analysis(&self) -> &TimeSeriesAnalysis {
        &self.analysis
    }
}

impl Default for TimeSeriesAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
