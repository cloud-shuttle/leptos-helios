//! Statistical Analysis Module

// use super::types::*; // Currently unused
use super::AnalyticsError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Statistical analyzer
pub struct StatisticalAnalyzer {
    descriptive_stats: DescriptiveStatistics,
    inferential_stats: InferentialStatistics,
}

/// Descriptive statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescriptiveStatistics {
    pub mean: f64,
    pub median: f64,
    pub mode: Option<f64>,
    pub standard_deviation: f64,
    pub variance: f64,
    pub min: f64,
    pub max: f64,
    pub range: f64,
    pub quartiles: Quartiles,
    pub skewness: f64,
    pub kurtosis: f64,
    pub coefficient_of_variation: f64,
}

/// Quartiles information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quartiles {
    pub q1: f64,
    pub q2: f64,
    pub q3: f64,
    pub iqr: f64,
}

/// Inferential statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferentialStatistics {
    pub confidence_interval: ConfidenceInterval,
    pub hypothesis_test: Option<HypothesisTest>,
    pub effect_size: Option<f64>,
    pub p_value: Option<f64>,
}

/// Confidence interval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceInterval {
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub confidence_level: f64,
}

/// Hypothesis test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypothesisTest {
    pub test_type: String,
    pub null_hypothesis: String,
    pub alternative_hypothesis: String,
    pub test_statistic: f64,
    pub critical_value: f64,
    pub degrees_of_freedom: Option<u32>,
}

impl StatisticalAnalyzer {
    /// Create a new statistical analyzer
    pub fn new() -> Self {
        Self {
            descriptive_stats: DescriptiveStatistics {
                mean: 0.0,
                median: 0.0,
                mode: None,
                standard_deviation: 0.0,
                variance: 0.0,
                min: 0.0,
                max: 0.0,
                range: 0.0,
                quartiles: Quartiles {
                    q1: 0.0,
                    q2: 0.0,
                    q3: 0.0,
                    iqr: 0.0,
                },
                skewness: 0.0,
                kurtosis: 0.0,
                coefficient_of_variation: 0.0,
            },
            inferential_stats: InferentialStatistics {
                confidence_interval: ConfidenceInterval {
                    lower_bound: 0.0,
                    upper_bound: 0.0,
                    confidence_level: 0.95,
                },
                hypothesis_test: None,
                effect_size: None,
                p_value: None,
            },
        }
    }

    /// Calculate descriptive statistics
    pub fn calculate_descriptive_stats(&mut self, data: &[f64]) -> Result<&DescriptiveStatistics, AnalyticsError> {
        if data.is_empty() {
            return Err(AnalyticsError::StatisticalError {
                message: "Cannot calculate statistics for empty dataset".to_string(),
            });
        }

        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Basic statistics
        let n = data.len() as f64;
        let sum: f64 = data.iter().sum();
        let mean = sum / n;

        // Variance and standard deviation
        let variance = data.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / (n - 1.0);
        let standard_deviation = variance.sqrt();

        // Median
        let median = if sorted_data.len() % 2 == 0 {
            let mid = sorted_data.len() / 2;
            (sorted_data[mid - 1] + sorted_data[mid]) / 2.0
        } else {
            sorted_data[sorted_data.len() / 2]
        };

        // Quartiles
        let q1 = self.calculate_percentile(&sorted_data, 25.0);
        let q2 = median;
        let q3 = self.calculate_percentile(&sorted_data, 75.0);
        let iqr = q3 - q1;

        // Min and max
        let min = *sorted_data.first().unwrap();
        let max = *sorted_data.last().unwrap();
        let range = max - min;

        // Mode (most frequent value)
        let mode = self.calculate_mode(data);

        // Skewness
        let skewness = self.calculate_skewness(data, mean, standard_deviation);

        // Kurtosis
        let kurtosis = self.calculate_kurtosis(data, mean, standard_deviation);

        // Coefficient of variation
        let coefficient_of_variation = if mean != 0.0 {
            standard_deviation / mean
        } else {
            0.0
        };

        self.descriptive_stats = DescriptiveStatistics {
            mean,
            median,
            mode,
            standard_deviation,
            variance,
            min,
            max,
            range,
            quartiles: Quartiles { q1, q2, q3, iqr },
            skewness,
            kurtosis,
            coefficient_of_variation,
        };

        Ok(&self.descriptive_stats)
    }

    /// Calculate percentile
    fn calculate_percentile(&self, sorted_data: &[f64], percentile: f64) -> f64 {
        let n = sorted_data.len() as f64;
        let index = (percentile / 100.0) * (n - 1.0);
        let lower = index.floor() as usize;
        let upper = index.ceil() as usize;

        if lower == upper {
            sorted_data[lower]
        } else {
            let weight = index - lower as f64;
            sorted_data[lower] * (1.0 - weight) + sorted_data[upper] * weight
        }
    }

    /// Calculate mode
    fn calculate_mode(&self, data: &[f64]) -> Option<f64> {
        let mut frequency_map: HashMap<String, usize> = HashMap::new();
        
        for value in data {
            let key = format!("{:.6}", value); // Round to avoid floating point precision issues
            *frequency_map.entry(key).or_insert(0) += 1;
        }

        frequency_map
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(value, _)| value.parse().unwrap())
    }

    /// Calculate skewness
    fn calculate_skewness(&self, data: &[f64], mean: f64, std_dev: f64) -> f64 {
        if std_dev == 0.0 {
            return 0.0;
        }

        let n = data.len() as f64;
        let sum_cubed_deviations: f64 = data.iter()
            .map(|x| ((x - mean) / std_dev).powi(3))
            .sum();

        sum_cubed_deviations / n
    }

    /// Calculate kurtosis
    fn calculate_kurtosis(&self, data: &[f64], mean: f64, std_dev: f64) -> f64 {
        if std_dev == 0.0 {
            return 0.0;
        }

        let n = data.len() as f64;
        let sum_fourth_deviations: f64 = data.iter()
            .map(|x| ((x - mean) / std_dev).powi(4))
            .sum();

        (sum_fourth_deviations / n) - 3.0 // Excess kurtosis
    }

    /// Calculate confidence interval
    pub fn calculate_confidence_interval(
        &mut self,
        data: &[f64],
        confidence_level: f64,
    ) -> Result<&ConfidenceInterval, AnalyticsError> {
        if data.is_empty() {
            return Err(AnalyticsError::StatisticalError {
                message: "Cannot calculate confidence interval for empty dataset".to_string(),
            });
        }

        let n = data.len() as f64;
        let mean = data.iter().sum::<f64>() / n;
        let std_dev = self.calculate_sample_standard_deviation(data, mean);

        // For large samples (n > 30), use normal distribution
        // For small samples, use t-distribution (simplified to normal for now)
        let z_score = match confidence_level {
            0.90 => 1.645,
            0.95 => 1.96,
            0.99 => 2.576,
            _ => 1.96, // Default to 95%
        };

        let margin_of_error = z_score * (std_dev / n.sqrt());
        let lower_bound = mean - margin_of_error;
        let upper_bound = mean + margin_of_error;

        self.inferential_stats.confidence_interval = ConfidenceInterval {
            lower_bound,
            upper_bound,
            confidence_level,
        };

        Ok(&self.inferential_stats.confidence_interval)
    }

    /// Calculate sample standard deviation
    fn calculate_sample_standard_deviation(&self, data: &[f64], mean: f64) -> f64 {
        let n = data.len() as f64;
        let variance = data.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / (n - 1.0);
        variance.sqrt()
    }

    /// Get descriptive statistics
    pub fn get_descriptive_stats(&self) -> &DescriptiveStatistics {
        &self.descriptive_stats
    }

    /// Get inferential statistics
    pub fn get_inferential_stats(&self) -> &InferentialStatistics {
        &self.inferential_stats
    }
}

impl Default for StatisticalAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
