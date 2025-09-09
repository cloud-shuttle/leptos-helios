//! Anomaly Detection System
//!
//! This module provides comprehensive anomaly detection capabilities for Helios visualizations,
//! including real-time anomaly detection, multiple algorithms, and configurable alerting.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use thiserror::Error;

use crate::advanced_analytics::{AnalyticsError, DataPoint, DataSeries};

/// Anomaly detection errors
#[derive(Debug, Error)]
pub enum AnomalyError {
    #[error("Anomaly detection algorithm error: {message}")]
    AlgorithmError { message: String },

    #[error("Threshold configuration error: {message}")]
    ThresholdError { message: String },

    #[error("Alert system error: {message}")]
    AlertError { message: String },

    #[error("Stream processing error: {message}")]
    StreamError { message: String },

    #[error("Model training error: {message}")]
    TrainingError { message: String },
}

/// Anomaly detection algorithm trait
pub trait AnomalyAlgorithm: Send + Sync {
    fn detect_anomalies(&self, data: &DataSeries) -> Result<Vec<AnomalyResult>, AnomalyError>;
    fn detect_anomaly(
        &self,
        data_point: &DataPoint,
        context: &[DataPoint],
    ) -> Result<AnomalyScore, AnomalyError>;
    fn train(&mut self, training_data: &DataSeries) -> Result<(), AnomalyError>;
    fn get_algorithm_info(&self) -> AlgorithmInfo;
    fn update_threshold(&mut self, threshold: f64) -> Result<(), AnomalyError>;
}

/// Algorithm information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmInfo {
    pub name: String,
    pub description: String,
    pub algorithm_type: AlgorithmType,
    pub parameters: HashMap<String, f64>,
    pub performance_metrics: PerformanceMetrics,
}

/// Algorithm types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlgorithmType {
    Statistical,
    MachineLearning,
    TimeSeries,
    DensityBased,
    IsolationForest,
    OneClassSVM,
    LocalOutlierFactor,
    DBSCAN,
    ZScore,
    ModifiedZScore,
    InterquartileRange,
    SeasonalDecomposition,
    ARIMA,
    LSTM,
    Custom(String),
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub accuracy: f64,
    pub false_positive_rate: f64,
    pub false_negative_rate: f64,
    pub processing_time: Duration,
}

/// Anomaly result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyResult {
    pub data_point: DataPoint,
    pub anomaly_score: AnomalyScore,
    pub is_anomaly: bool,
    pub confidence: f64,
    pub explanation: String,
    pub detected_at: Instant,
}

/// Anomaly score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyScore {
    pub score: f64,
    pub normalized_score: f64,
    pub threshold: f64,
    pub severity: AnomalySeverity,
    pub algorithm_used: String,
}

/// Anomaly severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnomalySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Threshold manager
#[derive(Debug)]
pub struct ThresholdManager {
    thresholds: HashMap<String, f64>,
    adaptive_thresholds: HashMap<String, AdaptiveThreshold>,
    threshold_history: HashMap<String, VecDeque<f64>>,
}

/// Adaptive threshold
#[derive(Debug, Clone)]
pub struct AdaptiveThreshold {
    pub base_threshold: f64,
    pub sensitivity: f64,
    pub learning_rate: f64,
    pub min_threshold: f64,
    pub max_threshold: f64,
    pub recent_scores: VecDeque<f64>,
    pub window_size: usize,
}

impl ThresholdManager {
    /// Create a new threshold manager
    pub fn new() -> Self {
        Self {
            thresholds: HashMap::new(),
            adaptive_thresholds: HashMap::new(),
            threshold_history: HashMap::new(),
        }
    }

    /// Set static threshold
    pub fn set_threshold(
        &mut self,
        algorithm_name: &str,
        threshold: f64,
    ) -> Result<(), AnomalyError> {
        if threshold < 0.0 || threshold > 1.0 {
            return Err(AnomalyError::ThresholdError {
                message: "Threshold must be between 0.0 and 1.0".to_string(),
            });
        }

        self.thresholds
            .insert(algorithm_name.to_string(), threshold);
        Ok(())
    }

    /// Set adaptive threshold
    pub fn set_adaptive_threshold(
        &mut self,
        algorithm_name: &str,
        config: AdaptiveThreshold,
    ) -> Result<(), AnomalyError> {
        if config.base_threshold < 0.0 || config.base_threshold > 1.0 {
            return Err(AnomalyError::ThresholdError {
                message: "Base threshold must be between 0.0 and 1.0".to_string(),
            });
        }

        if config.sensitivity < 0.0 || config.sensitivity > 1.0 {
            return Err(AnomalyError::ThresholdError {
                message: "Sensitivity must be between 0.0 and 1.0".to_string(),
            });
        }

        self.adaptive_thresholds
            .insert(algorithm_name.to_string(), config);
        Ok(())
    }

    /// Get threshold for algorithm
    pub fn get_threshold(&mut self, algorithm_name: &str, recent_scores: &[f64]) -> f64 {
        if let Some(adaptive) = self.adaptive_thresholds.get_mut(algorithm_name) {
            self.update_adaptive_threshold(adaptive, recent_scores)
        } else {
            self.thresholds.get(algorithm_name).copied().unwrap_or(0.5)
        }
    }

    /// Update adaptive threshold
    fn update_adaptive_threshold(
        &mut self,
        adaptive: &mut AdaptiveThreshold,
        recent_scores: &[f64],
    ) -> f64 {
        // Add recent scores to the window
        for &score in recent_scores {
            adaptive.recent_scores.push_back(score);
            if adaptive.recent_scores.len() > adaptive.window_size {
                adaptive.recent_scores.pop_front();
            }
        }

        if adaptive.recent_scores.len() < 10 {
            return adaptive.base_threshold;
        }

        // Calculate statistics of recent scores
        let mean = adaptive.recent_scores.iter().sum::<f64>() / adaptive.recent_scores.len() as f64;
        let variance = adaptive
            .recent_scores
            .iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>()
            / adaptive.recent_scores.len() as f64;
        let std_dev = variance.sqrt();

        // Adjust threshold based on recent performance
        let adjustment =
            (mean - adaptive.base_threshold) * adaptive.learning_rate * adaptive.sensitivity;
        let new_threshold = adaptive.base_threshold + adjustment;

        // Clamp to min/max bounds
        let clamped_threshold = new_threshold
            .max(adaptive.min_threshold)
            .min(adaptive.max_threshold);

        // Update threshold history
        self.threshold_history
            .entry(adaptive.base_threshold.to_string())
            .or_insert_with(VecDeque::new)
            .push_back(clamped_threshold);

        clamped_threshold
    }

    /// Get threshold history
    pub fn get_threshold_history(&self, algorithm_name: &str) -> Option<&VecDeque<f64>> {
        self.threshold_history.get(algorithm_name)
    }
}

/// Alert system
#[derive(Debug)]
pub struct AlertSystem {
    alert_rules: Vec<AlertRule>,
    alert_history: VecDeque<Alert>,
    notification_channels: Vec<Box<dyn NotificationChannel>>,
    rate_limiting: RateLimiting,
}

/// Alert rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub id: String,
    pub name: String,
    pub condition: AlertCondition,
    pub severity: AnomalySeverity,
    pub cooldown_period: Duration,
    pub enabled: bool,
    pub notification_channels: Vec<String>,
}

/// Alert condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertCondition {
    ScoreAboveThreshold { threshold: f64 },
    ConsecutiveAnomalies { count: u32 },
    AnomalyRate { rate: f64, window: Duration },
    SeverityLevel { severity: AnomalySeverity },
    Custom { expression: String },
}

/// Alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: String,
    pub rule_id: String,
    pub anomaly_result: AnomalyResult,
    pub severity: AnomalySeverity,
    pub message: String,
    pub created_at: Instant,
    pub acknowledged: bool,
    pub resolved: bool,
}

/// Notification channel trait
pub trait NotificationChannel: Send + Sync {
    fn send_notification(&self, alert: &Alert) -> Result<(), AnomalyError>;
    fn get_channel_info(&self) -> ChannelInfo;
}

/// Channel information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelInfo {
    pub name: String,
    pub channel_type: ChannelType,
    pub enabled: bool,
    pub configuration: HashMap<String, String>,
}

/// Channel types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelType {
    Console,
    Email,
    Webhook,
    Slack,
    Discord,
    Custom(String),
}

/// Rate limiting
#[derive(Debug, Clone)]
pub struct RateLimiting {
    pub max_alerts_per_minute: u32,
    pub max_alerts_per_hour: u32,
    pub max_alerts_per_day: u32,
    pub alert_counts: HashMap<String, VecDeque<Instant>>,
}

impl AlertSystem {
    /// Create a new alert system
    pub fn new() -> Self {
        Self {
            alert_rules: Vec::new(),
            alert_history: VecDeque::new(),
            notification_channels: Vec::new(),
            rate_limiting: RateLimiting {
                max_alerts_per_minute: 10,
                max_alerts_per_hour: 100,
                max_alerts_per_day: 1000,
                alert_counts: HashMap::new(),
            },
        }
    }

    /// Add alert rule
    pub fn add_alert_rule(&mut self, rule: AlertRule) {
        self.alert_rules.push(rule);
    }

    /// Add notification channel
    pub fn add_notification_channel(&mut self, channel: Box<dyn NotificationChannel>) {
        self.notification_channels.push(channel);
    }

    /// Process anomaly result
    pub fn process_anomaly(
        &mut self,
        anomaly_result: &AnomalyResult,
    ) -> Result<Vec<Alert>, AnomalyError> {
        let mut triggered_alerts = Vec::new();

        for rule in &self.alert_rules {
            if !rule.enabled {
                continue;
            }

            if self.should_trigger_alert(rule, anomaly_result)? {
                if self.is_rate_limited(&rule.id) {
                    continue;
                }

                let alert = Alert {
                    id: format!(
                        "alert_{}_{}",
                        rule.id,
                        anomaly_result.detected_at.elapsed().as_millis()
                    ),
                    rule_id: rule.id.clone(),
                    anomaly_result: anomaly_result.clone(),
                    severity: rule.severity.clone(),
                    message: self.generate_alert_message(rule, anomaly_result),
                    created_at: Instant::now(),
                    acknowledged: false,
                    resolved: false,
                };

                // Send notifications
                for channel in &self.notification_channels {
                    if let Err(e) = channel.send_notification(&alert) {
                        eprintln!("Failed to send notification: {}", e);
                    }
                }

                triggered_alerts.push(alert.clone());
                self.alert_history.push_back(alert);

                // Clean up old alerts
                if self.alert_history.len() > 10000 {
                    self.alert_history.pop_front();
                }
            }
        }

        Ok(triggered_alerts)
    }

    /// Check if alert should be triggered
    fn should_trigger_alert(
        &self,
        rule: &AlertRule,
        anomaly_result: &AnomalyResult,
    ) -> Result<bool, AnomalyError> {
        match &rule.condition {
            AlertCondition::ScoreAboveThreshold { threshold } => {
                Ok(anomaly_result.anomaly_score.score > *threshold)
            }
            AlertCondition::ConsecutiveAnomalies { count } => {
                let recent_anomalies = self.count_recent_anomalies(*count);
                Ok(recent_anomalies >= *count)
            }
            AlertCondition::AnomalyRate { rate, window } => {
                let current_rate = self.calculate_anomaly_rate(*window);
                Ok(current_rate > *rate)
            }
            AlertCondition::SeverityLevel { severity } => Ok(std::mem::discriminant(
                &anomaly_result.anomaly_score.severity,
            ) == std::mem::discriminant(severity)),
            AlertCondition::Custom { expression: _ } => {
                // Custom expression evaluation would be implemented here
                Ok(false)
            }
        }
    }

    /// Count recent anomalies
    fn count_recent_anomalies(&self, count: u32) -> u32 {
        let cutoff = Instant::now() - Duration::from_secs(300); // 5 minutes
        self.alert_history
            .iter()
            .rev()
            .take(count as usize)
            .filter(|alert| alert.created_at > cutoff)
            .count() as u32
    }

    /// Calculate anomaly rate
    fn calculate_anomaly_rate(&self, window: Duration) -> f64 {
        let cutoff = Instant::now() - window;
        let recent_alerts = self
            .alert_history
            .iter()
            .filter(|alert| alert.created_at > cutoff)
            .count();

        recent_alerts as f64 / window.as_secs() as f64
    }

    /// Check rate limiting
    fn is_rate_limited(&mut self, rule_id: &str) -> bool {
        let now = Instant::now();
        let counts = self
            .rate_limiting
            .alert_counts
            .entry(rule_id.to_string())
            .or_insert_with(VecDeque::new);

        // Remove old timestamps
        counts.retain(|&timestamp| now.duration_since(timestamp) < Duration::from_secs(3600));

        // Check limits
        let recent_count = counts.len() as u32;
        if recent_count >= self.rate_limiting.max_alerts_per_hour {
            return true;
        }

        // Add current timestamp
        counts.push_back(now);
        false
    }

    /// Generate alert message
    fn generate_alert_message(&self, rule: &AlertRule, anomaly_result: &AnomalyResult) -> String {
        format!(
            "Anomaly Alert: {} - Score: {:.3}, Severity: {:?}, Time: {}",
            rule.name,
            anomaly_result.anomaly_score.score,
            anomaly_result.anomaly_score.severity,
            anomaly_result.detected_at.elapsed().as_secs()
        )
    }

    /// Get alert history
    pub fn get_alert_history(&self) -> &VecDeque<Alert> {
        &self.alert_history
    }

    /// Acknowledge alert
    pub fn acknowledge_alert(&mut self, alert_id: &str) -> Result<(), AnomalyError> {
        if let Some(alert) = self.alert_history.iter_mut().find(|a| a.id == alert_id) {
            alert.acknowledged = true;
            Ok(())
        } else {
            Err(AnomalyError::AlertError {
                message: format!("Alert {} not found", alert_id),
            })
        }
    }

    /// Resolve alert
    pub fn resolve_alert(&mut self, alert_id: &str) -> Result<(), AnomalyError> {
        if let Some(alert) = self.alert_history.iter_mut().find(|a| a.id == alert_id) {
            alert.resolved = true;
            Ok(())
        } else {
            Err(AnomalyError::AlertError {
                message: format!("Alert {} not found", alert_id),
            })
        }
    }
}

/// Anomaly detector
#[derive(Debug)]
pub struct AnomalyDetector {
    algorithms: Vec<Box<dyn AnomalyAlgorithm>>,
    threshold_manager: ThresholdManager,
    alert_system: AlertSystem,
    performance_monitor: PerformanceMonitor,
}

/// Performance monitor
#[derive(Debug)]
pub struct PerformanceMonitor {
    processing_times: VecDeque<Duration>,
    detection_counts: HashMap<String, u64>,
    false_positive_counts: HashMap<String, u64>,
    false_negative_counts: HashMap<String, u64>,
    last_reset: Instant,
}

impl AnomalyDetector {
    /// Create a new anomaly detector
    pub fn new() -> Self {
        Self {
            algorithms: Vec::new(),
            threshold_manager: ThresholdManager::new(),
            alert_system: AlertSystem::new(),
            performance_monitor: PerformanceMonitor {
                processing_times: VecDeque::new(),
                detection_counts: HashMap::new(),
                false_positive_counts: HashMap::new(),
                false_negative_counts: HashMap::new(),
                last_reset: Instant::now(),
            },
        }
    }

    /// Add anomaly algorithm
    pub fn add_algorithm(&mut self, algorithm: Box<dyn AnomalyAlgorithm>) {
        let info = algorithm.get_algorithm_info();
        self.algorithms.push(algorithm);
        self.performance_monitor
            .detection_counts
            .insert(info.name, 0);
        self.performance_monitor
            .false_positive_counts
            .insert(info.name, 0);
        self.performance_monitor
            .false_negative_counts
            .insert(info.name, 0);
    }

    /// Set threshold for algorithm
    pub fn set_threshold(
        &mut self,
        algorithm_name: &str,
        threshold: f64,
    ) -> Result<(), AnomalyError> {
        self.threshold_manager
            .set_threshold(algorithm_name, threshold)
    }

    /// Set adaptive threshold
    pub fn set_adaptive_threshold(
        &mut self,
        algorithm_name: &str,
        config: AdaptiveThreshold,
    ) -> Result<(), AnomalyError> {
        self.threshold_manager
            .set_adaptive_threshold(algorithm_name, config)
    }

    /// Add alert rule
    pub fn add_alert_rule(&mut self, rule: AlertRule) {
        self.alert_system.add_alert_rule(rule);
    }

    /// Add notification channel
    pub fn add_notification_channel(&mut self, channel: Box<dyn NotificationChannel>) {
        self.alert_system.add_notification_channel(channel);
    }

    /// Detect anomalies in data series
    pub fn detect_anomalies(
        &mut self,
        data: &DataSeries,
    ) -> Result<Vec<AnomalyResult>, AnomalyError> {
        let start_time = Instant::now();
        let mut all_anomalies = Vec::new();

        for algorithm in &self.algorithms {
            let algorithm_info = algorithm.get_algorithm_info();
            let algorithm_start = Instant::now();

            match algorithm.detect_anomalies(data) {
                Ok(anomalies) => {
                    let processing_time = algorithm_start.elapsed();
                    self.performance_monitor
                        .processing_times
                        .push_back(processing_time);

                    // Update detection count
                    if let Some(count) = self
                        .performance_monitor
                        .detection_counts
                        .get_mut(&algorithm_info.name)
                    {
                        *count += anomalies.len() as u64;
                    }

                    all_anomalies.extend(anomalies);
                }
                Err(e) => {
                    eprintln!("Algorithm {} failed: {}", algorithm_info.name, e);
                }
            }
        }

        // Process alerts for detected anomalies
        for anomaly in &all_anomalies {
            if let Err(e) = self.alert_system.process_anomaly(anomaly) {
                eprintln!("Failed to process alert: {}", e);
            }
        }

        let total_time = start_time.elapsed();
        self.performance_monitor
            .processing_times
            .push_back(total_time);

        Ok(all_anomalies)
    }

    /// Detect anomaly in single data point
    pub fn detect_anomaly(
        &mut self,
        data_point: &DataPoint,
        context: &[DataPoint],
    ) -> Result<Vec<AnomalyResult>, AnomalyError> {
        let mut results = Vec::new();

        for algorithm in &self.algorithms {
            let algorithm_info = algorithm.get_algorithm_info();

            match algorithm.detect_anomaly(data_point, context) {
                Ok(score) => {
                    let threshold = self
                        .threshold_manager
                        .get_threshold(&algorithm_info.name, &[score.score]);
                    let is_anomaly = score.score > threshold;

                    let result = AnomalyResult {
                        data_point: data_point.clone(),
                        anomaly_score: score,
                        is_anomaly,
                        confidence: if is_anomaly {
                            score.score
                        } else {
                            1.0 - score.score
                        },
                        explanation: format!("Detected by {} algorithm", algorithm_info.name),
                        detected_at: Instant::now(),
                    };

                    results.push(result);
                }
                Err(e) => {
                    eprintln!("Algorithm {} failed: {}", algorithm_info.name, e);
                }
            }
        }

        // Process alerts
        for result in &results {
            if let Err(e) = self.alert_system.process_anomaly(result) {
                eprintln!("Failed to process alert: {}", e);
            }
        }

        Ok(results)
    }

    /// Train algorithms
    pub fn train_algorithms(&mut self, training_data: &DataSeries) -> Result<(), AnomalyError> {
        for algorithm in &mut self.algorithms {
            if let Err(e) = algorithm.train(training_data) {
                eprintln!("Training failed: {}", e);
            }
        }
        Ok(())
    }

    /// Get performance statistics
    pub fn get_performance_stats(&self) -> PerformanceStats {
        let avg_processing_time = if !self.performance_monitor.processing_times.is_empty() {
            self.performance_monitor
                .processing_times
                .iter()
                .sum::<Duration>()
                / self.performance_monitor.processing_times.len() as u32
        } else {
            Duration::ZERO
        };

        PerformanceStats {
            total_detections: self.performance_monitor.detection_counts.values().sum(),
            algorithm_stats: self.performance_monitor.detection_counts.clone(),
            average_processing_time: avg_processing_time,
            total_alerts: self.alert_system.alert_history.len(),
            active_alerts: self
                .alert_system
                .alert_history
                .iter()
                .filter(|alert| !alert.resolved)
                .count(),
        }
    }

    /// Get alert history
    pub fn get_alert_history(&self) -> &VecDeque<Alert> {
        self.alert_system.get_alert_history()
    }
}

/// Performance statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceStats {
    pub total_detections: u64,
    pub algorithm_stats: HashMap<String, u64>,
    pub average_processing_time: Duration,
    pub total_alerts: usize,
    pub active_alerts: usize,
}

impl Default for ThresholdManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for AlertSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for AnomalyDetector {
    fn default() -> Self {
        Self::new()
    }
}
