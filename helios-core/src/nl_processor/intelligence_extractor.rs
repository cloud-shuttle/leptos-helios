//! Intelligence feature extraction from natural language queries

use super::types::{IntelligenceConfig, ForecastConfig, ForecastMethod, AnomalyConfig, ClusteringConfig};
use crate::chart::{Intelligence, IntelligenceFeature, ForecastConfig as ChartForecastConfig, AnomalyConfig as ChartAnomalyConfig, ClusterConfig};
use std::collections::HashMap;

/// Intelligence feature extractor
pub struct IntelligenceExtractor {
    config: IntelligenceConfig,
}

impl IntelligenceExtractor {
    /// Create a new intelligence extractor
    pub fn new(config: IntelligenceConfig) -> Self {
        Self { config }
    }

    /// Extract intelligence configuration from query
    pub fn extract_intelligence_config(&self, query: &str) -> Option<Intelligence> {
        let query_lower = query.to_lowercase();
        let mut intelligence = None;

        let mut features = Vec::new();
        let mut parameters = HashMap::new();
        let mut forecast = None;
        let mut anomaly_detection = None;
        let mut trend_analysis = None;
        let mut clustering = None;

        // Check for forecasting
        if self.config.forecasting && self.is_forecast_query(&query_lower) {
            if let Some(forecast_config) = self.extract_forecast_config(&query_lower) {
                features.push(IntelligenceFeature::Forecasting);
                forecast = Some(ChartForecastConfig {
                    enabled: true,
                    horizon: Some(forecast_config.periods),
                    confidence_interval: Some(forecast_config.confidence_interval),
                });
            }
        }

        // Check for anomaly detection
        if self.config.anomaly_detection && self.is_anomaly_query(&query_lower) {
            if let Some(anomaly_config) = self.extract_anomaly_config(&query_lower) {
                features.push(IntelligenceFeature::AnomalyDetection);
                anomaly_detection = Some(ChartAnomalyConfig {
                    enabled: true,
                    threshold: Some(anomaly_config.sensitivity),
                    method: Some(anomaly_config.method),
                });
            }
        }

        // Check for clustering
        if self.config.clustering && self.is_clustering_query(&query_lower) {
            if let Some(clustering_config) = self.extract_clustering_config(&query_lower) {
                features.push(IntelligenceFeature::Clustering);
                clustering = Some(ClusterConfig {
                    enabled: true,
                    algorithm: Some(clustering_config.algorithm),
                    num_clusters: Some(clustering_config.num_clusters as u32),
                });
            }
        }

        // Check for trend analysis
        if self.config.trend_analysis && self.is_trend_query(&query_lower) {
            features.push(IntelligenceFeature::TrendAnalysis);
            trend_analysis = Some(true);
        }

        if !features.is_empty() {
            intelligence = Some(Intelligence {
                features,
                parameters,
                forecast,
                anomaly_detection,
                trend_analysis,
                clustering,
            });
        }

        intelligence
    }

    /// Extract forecast configuration from query
    pub fn extract_forecast_periods(&self, query: &str) -> Option<u32> {
        let query_lower = query.to_lowercase();
        
        // Look for number patterns
        let words: Vec<&str> = query_lower.split_whitespace().collect();
        for (i, word) in words.iter().enumerate() {
            if word.contains("forecast") || word.contains("predict") {
                // Look for number in next few words
                for j in 1..=3 {
                    if i + j < words.len() {
                        if let Ok(num) = words[i + j].parse::<u32>() {
                            return Some(num);
                        }
                    }
                }
            }
        }

        // Default periods
        if query_lower.contains("forecast") || query_lower.contains("predict") {
            Some(12) // Default to 12 periods
        } else {
            None
        }
    }

    /// Check if query is about forecasting
    fn is_forecast_query(&self, query: &str) -> bool {
        let forecast_keywords = [
            "forecast", "predict", "future", "next", "upcoming", 
            "projection", "estimate", "anticipate"
        ];
        
        forecast_keywords.iter().any(|keyword| query.contains(keyword))
    }

    /// Check if query is about anomaly detection
    fn is_anomaly_query(&self, query: &str) -> bool {
        let anomaly_keywords = [
            "anomaly", "anomalies", "outlier", "outliers", "unusual", "abnormal", "exception", "exceptions",
            "spike", "spikes", "drop", "drops", "deviation", "deviations", "irregular", "irregulars"
        ];
        
        anomaly_keywords.iter().any(|keyword| query.contains(keyword))
    }

    /// Check if query is about clustering
    fn is_clustering_query(&self, query: &str) -> bool {
        let clustering_keywords = [
            "cluster", "group", "segment", "classify", "categorize",
            "similar", "pattern", "grouping"
        ];
        
        clustering_keywords.iter().any(|keyword| query.contains(keyword))
    }

    /// Check if query is about trend analysis
    fn is_trend_query(&self, query: &str) -> bool {
        let trend_keywords = [
            "trend", "direction", "slope", "increase", "decrease",
            "growth", "decline", "change over time"
        ];
        
        trend_keywords.iter().any(|keyword| query.contains(keyword))
    }

    /// Extract forecast configuration
    fn extract_forecast_config(&self, query: &str) -> Option<ForecastConfig> {
        let method = if query.contains("linear") {
            ForecastMethod::Linear
        } else if query.contains("exponential") {
            ForecastMethod::Exponential
        } else if query.contains("arima") {
            ForecastMethod::Arima
        } else if query.contains("seasonal") {
            ForecastMethod::Seasonal
        } else {
            ForecastMethod::Linear // Default
        };

        let periods = self.extract_forecast_periods(query).unwrap_or(12);
        let confidence_interval = if query.contains("95%") {
            0.95
        } else if query.contains("90%") {
            0.90
        } else {
            0.95 // Default
        };

        Some(ForecastConfig {
            method,
            periods,
            confidence_interval,
        })
    }

    /// Extract anomaly detection configuration
    fn extract_anomaly_config(&self, query: &str) -> Option<AnomalyConfig> {
        let sensitivity = if query.contains("high") || query.contains("sensitive") {
            0.8
        } else if query.contains("low") || query.contains("conservative") {
            0.3
        } else {
            0.5 // Default
        };

        let method = if query.contains("statistical") {
            "statistical".to_string()
        } else if query.contains("isolation") {
            "isolation_forest".to_string()
        } else {
            "statistical".to_string() // Default
        };

        Some(AnomalyConfig {
            sensitivity,
            method,
        })
    }

    /// Extract clustering configuration
    fn extract_clustering_config(&self, query: &str) -> Option<ClusteringConfig> {
        let num_clusters = if let Some(num) = self.extract_number_from_query(query) {
            num as usize
        } else {
            3 // Default
        };

        let algorithm = if query.contains("kmeans") || query.contains("k-means") {
            "kmeans".to_string()
        } else if query.contains("hierarchical") {
            "hierarchical".to_string()
        } else {
            "kmeans".to_string() // Default
        };

        Some(ClusteringConfig {
            num_clusters,
            algorithm,
        })
    }

    /// Extract number from query text
    fn extract_number_from_query(&self, query: &str) -> Option<u32> {
        let words: Vec<&str> = query.split_whitespace().collect();
        for word in words {
            if let Ok(num) = word.parse::<u32>() {
                return Some(num);
            }
        }
        None
    }
}
