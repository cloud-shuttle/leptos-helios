//! Correlation Analysis Module

// use super::types::*; // Currently unused
use super::AnalyticsError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Correlation analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationAnalysis {
    pub correlation_matrix: HashMap<String, HashMap<String, f64>>,
    pub significant_correlations: Vec<SignificantCorrelation>,
    pub correlation_strength: CorrelationStrength,
}

/// Significant correlation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignificantCorrelation {
    pub variable1: String,
    pub variable2: String,
    pub correlation_coefficient: f64,
    pub p_value: f64,
    pub significance_level: f64,
}

/// Correlation strength
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CorrelationStrength {
    VeryWeak,
    Weak,
    Moderate,
    Strong,
    VeryStrong,
}

/// Correlation analyzer
pub struct CorrelationAnalyzer {
    analysis: CorrelationAnalysis,
}

impl CorrelationAnalyzer {
    /// Create a new correlation analyzer
    pub fn new() -> Self {
        Self {
            analysis: CorrelationAnalysis {
                correlation_matrix: HashMap::new(),
                significant_correlations: Vec::new(),
                correlation_strength: CorrelationStrength::VeryWeak,
            },
        }
    }

    /// Analyze correlations between multiple data series
    pub fn analyze_correlations(
        &mut self,
        data_series: &HashMap<String, Vec<f64>>,
    ) -> Result<&CorrelationAnalysis, AnalyticsError> {
        if data_series.is_empty() {
            return Err(AnalyticsError::StatisticalError {
                message: "Cannot analyze correlations for empty dataset".to_string(),
            });
        }

        let series_names: Vec<String> = data_series.keys().cloned().collect();
        let mut correlation_matrix = HashMap::new();
        let mut significant_correlations = Vec::new();

        // Calculate pairwise correlations
        for (i, name1) in series_names.iter().enumerate() {
            let mut row = HashMap::new();
            let series1 = data_series.get(name1).unwrap();

            for (j, name2) in series_names.iter().enumerate() {
                let series2 = data_series.get(name2).unwrap();
                
                if i == j {
                    row.insert(name2.clone(), 1.0);
                } else {
                    let correlation = self.calculate_pearson_correlation(series1, series2)?;
                    row.insert(name2.clone(), correlation);

                    // Check for significant correlations
                    if correlation.abs() > 0.3 {
                        let p_value = self.calculate_p_value(correlation, series1.len());
                        if p_value < 0.05 {
                            significant_correlations.push(SignificantCorrelation {
                                variable1: name1.clone(),
                                variable2: name2.clone(),
                                correlation_coefficient: correlation,
                                p_value,
                                significance_level: 0.05,
                            });
                        }
                    }
                }
            }

            correlation_matrix.insert(name1.clone(), row);
        }

        // Determine overall correlation strength
        let correlation_strength = self.determine_correlation_strength(&significant_correlations);

        self.analysis = CorrelationAnalysis {
            correlation_matrix,
            significant_correlations,
            correlation_strength,
        };

        Ok(&self.analysis)
    }

    /// Calculate Pearson correlation coefficient
    fn calculate_pearson_correlation(
        &self,
        x: &[f64],
        y: &[f64],
    ) -> Result<f64, AnalyticsError> {
        if x.len() != y.len() {
            return Err(AnalyticsError::StatisticalError {
                message: "Data series must have the same length".to_string(),
            });
        }

        if x.is_empty() {
            return Err(AnalyticsError::StatisticalError {
                message: "Cannot calculate correlation for empty series".to_string(),
            });
        }

        let n = x.len() as f64;
        let sum_x: f64 = x.iter().sum();
        let sum_y: f64 = y.iter().sum();
        let sum_xy: f64 = x.iter().zip(y.iter()).map(|(a, b)| a * b).sum();
        let sum_x2: f64 = x.iter().map(|a| a * a).sum();
        let sum_y2: f64 = y.iter().map(|a| a * a).sum();

        let numerator = n * sum_xy - sum_x * sum_y;
        let denominator = ((n * sum_x2 - sum_x * sum_x) * (n * sum_y2 - sum_y * sum_y)).sqrt();

        if denominator == 0.0 {
            Ok(0.0)
        } else {
            Ok(numerator / denominator)
        }
    }

    /// Calculate p-value for correlation (simplified)
    fn calculate_p_value(&self, correlation: f64, n: usize) -> f64 {
        if n < 3 {
            return 1.0;
        }

        let t_statistic = correlation * ((n - 2) as f64 / (1.0 - correlation * correlation)).sqrt();
        
        // Simplified p-value calculation using t-distribution approximation
        // For large samples, t-distribution approaches normal distribution
        let p_value = if t_statistic.abs() > 2.0 {
            0.05 // Significant
        } else if t_statistic.abs() > 1.96 {
            0.05 // Significant at 95% confidence
        } else {
            0.1 // Not significant
        };

        p_value
    }

    /// Determine overall correlation strength
    fn determine_correlation_strength(&self, significant_correlations: &[SignificantCorrelation]) -> CorrelationStrength {
        if significant_correlations.is_empty() {
            return CorrelationStrength::VeryWeak;
        }

        let max_correlation = significant_correlations
            .iter()
            .map(|corr| corr.correlation_coefficient.abs())
            .fold(0.0, f64::max);

        match max_correlation {
            x if x >= 0.9 => CorrelationStrength::VeryStrong,
            x if x >= 0.7 => CorrelationStrength::Strong,
            x if x >= 0.5 => CorrelationStrength::Moderate,
            x if x >= 0.3 => CorrelationStrength::Weak,
            _ => CorrelationStrength::VeryWeak,
        }
    }

    /// Calculate correlation between two specific series
    pub fn calculate_pairwise_correlation(
        &self,
        series1: &[f64],
        series2: &[f64],
    ) -> Result<f64, AnalyticsError> {
        self.calculate_pearson_correlation(series1, series2)
    }

    /// Get correlation matrix
    pub fn get_correlation_matrix(&self) -> &HashMap<String, HashMap<String, f64>> {
        &self.analysis.correlation_matrix
    }

    /// Get significant correlations
    pub fn get_significant_correlations(&self) -> &[SignificantCorrelation] {
        &self.analysis.significant_correlations
    }

    /// Get correlation strength
    pub fn get_correlation_strength(&self) -> &CorrelationStrength {
        &self.analysis.correlation_strength
    }

    /// Find highly correlated pairs
    pub fn find_highly_correlated_pairs(
        &self,
        threshold: f64,
    ) -> Vec<&SignificantCorrelation> {
        self.analysis.significant_correlations
            .iter()
            .filter(|corr| corr.correlation_coefficient.abs() >= threshold)
            .collect()
    }

    /// Get correlation summary
    pub fn get_correlation_summary(&self) -> CorrelationSummary {
        let total_pairs = self.analysis.correlation_matrix.len() * 
                         (self.analysis.correlation_matrix.len() - 1) / 2;
        let significant_count = self.analysis.significant_correlations.len();
        let strong_count = self.find_highly_correlated_pairs(0.7).len();
        let moderate_count = self.find_highly_correlated_pairs(0.5).len();

        CorrelationSummary {
            total_variables: self.analysis.correlation_matrix.len(),
            total_pairs,
            significant_correlations: significant_count,
            strong_correlations: strong_count,
            moderate_correlations: moderate_count,
            overall_strength: self.analysis.correlation_strength.clone(),
        }
    }
}

/// Correlation summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationSummary {
    pub total_variables: usize,
    pub total_pairs: usize,
    pub significant_correlations: usize,
    pub strong_correlations: usize,
    pub moderate_correlations: usize,
    pub overall_strength: CorrelationStrength,
}

impl Default for CorrelationAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
