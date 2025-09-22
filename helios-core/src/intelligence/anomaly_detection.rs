//! Anomaly Detection Module

use super::MLError;
use crate::DataFrame;
// use polars::prelude::*; // Currently unused
// use std::collections::HashMap; // Currently unused

/// Anomaly detector for identifying outliers and unusual patterns
pub struct AnomalyDetector {
    /// Threshold for anomaly detection
    pub threshold: f64,
    /// Method used for anomaly detection
    pub method: AnomalyMethod,
    /// Training data statistics
    pub statistics: Option<DataStatistics>,
    /// Detected anomalies
    pub anomalies: Vec<AnomalyPoint>,
}

/// Methods for anomaly detection
#[derive(Debug, Clone, PartialEq)]
pub enum AnomalyMethod {
    /// Statistical methods (Z-score, IQR)
    Statistical,
    /// Isolation Forest algorithm
    IsolationForest,
    /// Local Outlier Factor
    LocalOutlierFactor,
    /// One-Class SVM
    OneClassSVM,
}

/// Data statistics for anomaly detection
#[derive(Debug, Clone)]
pub struct DataStatistics {
    /// Mean of the data
    pub mean: f64,
    /// Standard deviation of the data
    pub std_dev: f64,
    /// Median of the data
    pub median: f64,
    /// First quartile (Q1)
    pub q1: f64,
    /// Third quartile (Q3)
    pub q3: f64,
    /// Interquartile range (IQR)
    pub iqr: f64,
    /// Minimum value
    pub min: f64,
    /// Maximum value
    pub max: f64,
}

/// Anomaly point information
#[derive(Debug, Clone)]
pub struct AnomalyPoint {
    /// Index of the anomaly point
    pub index: usize,
    /// Value of the anomaly point
    pub value: f64,
    /// Anomaly score (higher = more anomalous)
    pub score: f64,
    /// Reason for being flagged as anomaly
    pub reason: AnomalyReason,
}

/// Reasons for anomaly detection
#[derive(Debug, Clone, PartialEq)]
pub enum AnomalyReason {
    /// Z-score is too high
    HighZScore,
    /// Outside IQR bounds
    OutsideIQR,
    /// Isolation Forest anomaly
    IsolationForest,
    /// Local outlier factor
    LocalOutlier,
    /// One-class SVM outlier
    OneClassSVM,
}

/// Anomaly detection results
#[derive(Debug, Clone)]
pub struct AnomalyResults {
    /// Detected anomalies
    pub anomalies: Vec<AnomalyPoint>,
    /// Total number of anomalies
    pub count: usize,
    /// Anomaly rate (percentage)
    pub rate: f64,
    /// Data statistics used for detection
    pub statistics: DataStatistics,
}

impl AnomalyDetector {
    /// Create a new anomaly detector
    pub fn new(threshold: f64, method: AnomalyMethod) -> Self {
        Self {
            threshold,
            method,
            statistics: None,
            anomalies: Vec::new(),
        }
    }

    /// Train the anomaly detector on normal data
    pub fn train(&mut self, data: &DataFrame) -> Result<(), MLError> {
        if data.height() < 5 {
            return Err(MLError::InsufficientData(
                "Need at least 5 data points for anomaly detection training".to_string(),
            ));
        }

        let values = self.extract_values(data)?;
        self.statistics = Some(self.calculate_statistics(&values));

        Ok(())
    }

    /// Detect anomalies in the data
    pub fn detect(&mut self, data: &DataFrame) -> Result<AnomalyResults, MLError> {
        if self.statistics.is_none() {
            return Err(MLError::ModelNotTrained);
        }

        let values = self.extract_values(data)?;
        let statistics = self.statistics.as_ref().unwrap();

        let anomalies = match self.method {
            AnomalyMethod::Statistical => {
                self.detect_statistical_anomalies(&values, statistics)?
            }
            AnomalyMethod::IsolationForest => {
                self.detect_isolation_forest_anomalies(&values)?
            }
            AnomalyMethod::LocalOutlierFactor => {
                self.detect_lof_anomalies(&values)?
            }
            AnomalyMethod::OneClassSVM => {
                self.detect_one_class_svm_anomalies(&values)?
            }
        };

        self.anomalies = anomalies.clone();
        let count = anomalies.len();
        let rate = (count as f64 / values.len() as f64) * 100.0;

        Ok(AnomalyResults {
            anomalies,
            count,
            rate,
            statistics: statistics.clone(),
        })
    }

    /// Extract values from DataFrame
    fn extract_values(&self, data: &DataFrame) -> Result<Vec<f64>, MLError> {
        let columns = data.get_columns();
        for column in columns {
            if let Ok(series) = column.f64() {
                return Ok(series.into_iter().filter_map(|v| v).collect());
            }
        }

        Err(MLError::InvalidData(
            "No numeric column found for anomaly detection".to_string(),
        ))
    }

    /// Calculate basic statistics
    fn calculate_statistics(&self, values: &[f64]) -> DataStatistics {
        let n = values.len() as f64;
        let mean = values.iter().sum::<f64>() / n;

        let variance = values
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>()
            / (n - 1.0);
        let std_dev = variance.sqrt();

        let mut sorted_values = values.to_vec();
        sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let median = if sorted_values.len() % 2 == 0 {
            let mid = sorted_values.len() / 2;
            (sorted_values[mid - 1] + sorted_values[mid]) / 2.0
        } else {
            sorted_values[sorted_values.len() / 2]
        };

        let q1 = self.calculate_percentile(&sorted_values, 25.0);
        let q3 = self.calculate_percentile(&sorted_values, 75.0);
        let iqr = q3 - q1;

        DataStatistics {
            mean,
            std_dev,
            median,
            q1,
            q3,
            iqr,
            min: *sorted_values.first().unwrap(),
            max: *sorted_values.last().unwrap(),
        }
    }

    /// Calculate percentile
    fn calculate_percentile(&self, sorted_values: &[f64], percentile: f64) -> f64 {
        let n = sorted_values.len() as f64;
        let index = (percentile / 100.0) * (n - 1.0);
        let lower = index.floor() as usize;
        let upper = index.ceil() as usize;

        if lower == upper {
            sorted_values[lower]
        } else {
            let weight = index - lower as f64;
            sorted_values[lower] * (1.0 - weight) + sorted_values[upper] * weight
        }
    }

    /// Detect anomalies using statistical methods
    fn detect_statistical_anomalies(
        &self,
        values: &[f64],
        statistics: &DataStatistics,
    ) -> Result<Vec<AnomalyPoint>, MLError> {
        let mut anomalies = Vec::new();

        for (i, &value) in values.iter().enumerate() {
            let mut is_anomaly = false;
            let mut reason = AnomalyReason::HighZScore;
            let mut score = 0.0;

            // Z-score method
            if statistics.std_dev > 0.0 {
                let z_score = (value - statistics.mean).abs() / statistics.std_dev;
                if z_score > self.threshold {
                    is_anomaly = true;
                    score = z_score;
                    reason = AnomalyReason::HighZScore;
                }
            }

            // IQR method
            if !is_anomaly {
                let lower_bound = statistics.q1 - 1.5 * statistics.iqr;
                let upper_bound = statistics.q3 + 1.5 * statistics.iqr;
                if value < lower_bound || value > upper_bound {
                    is_anomaly = true;
                    score = if value < lower_bound {
                        (lower_bound - value) / statistics.iqr
                    } else {
                        (value - upper_bound) / statistics.iqr
                    };
                    reason = AnomalyReason::OutsideIQR;
                }
            }

            if is_anomaly {
                anomalies.push(AnomalyPoint {
                    index: i,
                    value,
                    score,
                    reason,
                });
            }
        }

        Ok(anomalies)
    }

    /// Detect anomalies using Isolation Forest (simplified)
    fn detect_isolation_forest_anomalies(&self, values: &[f64]) -> Result<Vec<AnomalyPoint>, MLError> {
        let mut anomalies = Vec::new();

        // Simplified isolation forest implementation
        for (i, &value) in values.iter().enumerate() {
            // Simple heuristic: values far from median are anomalies
            let median = self.calculate_median(values);
            let mad = self.calculate_mad(values, median);
            
            if mad > 0.0 {
                let score = (value - median).abs() / mad;
                if score > self.threshold {
                    anomalies.push(AnomalyPoint {
                        index: i,
                        value,
                        score,
                        reason: AnomalyReason::IsolationForest,
                    });
                }
            }
        }

        Ok(anomalies)
    }

    /// Detect anomalies using Local Outlier Factor (simplified)
    fn detect_lof_anomalies(&self, values: &[f64]) -> Result<Vec<AnomalyPoint>, MLError> {
        let mut anomalies = Vec::new();

        // Simplified LOF implementation
        for (i, &value) in values.iter().enumerate() {
            let mut distances = Vec::new();
            
            // Calculate distances to other points
            for (j, &other_value) in values.iter().enumerate() {
                if i != j {
                    distances.push((value - other_value).abs());
                }
            }

            distances.sort_by(|a, b| a.partial_cmp(b).unwrap());
            
            // Use k-nearest neighbors (k=3)
            let k = 3.min(distances.len());
            if k > 0 {
                let avg_distance = distances[..k].iter().sum::<f64>() / k as f64;
                let score = avg_distance;
                
                if score > self.threshold {
                    anomalies.push(AnomalyPoint {
                        index: i,
                        value,
                        score,
                        reason: AnomalyReason::LocalOutlier,
                    });
                }
            }
        }

        Ok(anomalies)
    }

    /// Detect anomalies using One-Class SVM (simplified)
    fn detect_one_class_svm_anomalies(&self, values: &[f64]) -> Result<Vec<AnomalyPoint>, MLError> {
        let mut anomalies = Vec::new();

        // Simplified one-class SVM implementation
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let std_dev = self.calculate_std_dev(values, mean);

        for (i, &value) in values.iter().enumerate() {
            if std_dev > 0.0 {
                let distance = (value - mean).abs() / std_dev;
                if distance > self.threshold {
                    anomalies.push(AnomalyPoint {
                        index: i,
                        value,
                        score: distance,
                        reason: AnomalyReason::OneClassSVM,
                    });
                }
            }
        }

        Ok(anomalies)
    }

    /// Calculate median
    fn calculate_median(&self, values: &[f64]) -> f64 {
        let mut sorted_values = values.to_vec();
        sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        if sorted_values.len() % 2 == 0 {
            let mid = sorted_values.len() / 2;
            (sorted_values[mid - 1] + sorted_values[mid]) / 2.0
        } else {
            sorted_values[sorted_values.len() / 2]
        }
    }

    /// Calculate Median Absolute Deviation
    fn calculate_mad(&self, values: &[f64], median: f64) -> f64 {
        let mut deviations = values.iter().map(|&x| (x - median).abs()).collect::<Vec<_>>();
        deviations.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        if deviations.len() % 2 == 0 {
            let mid = deviations.len() / 2;
            (deviations[mid - 1] + deviations[mid]) / 2.0
        } else {
            deviations[deviations.len() / 2]
        }
    }

    /// Calculate standard deviation
    fn calculate_std_dev(&self, values: &[f64], mean: f64) -> f64 {
        let n = values.len() as f64;
        let variance = values
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>()
            / (n - 1.0);
        variance.sqrt()
    }
}
