//! Performance Profiler
//!
//! This module provides comprehensive performance profiling capabilities for Helios,
//! including timing analysis, memory usage tracking, and bottleneck identification.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::broadcast;

/// Performance profiler errors
#[derive(Debug, thiserror::Error)]
pub enum ProfilerError {
    #[error("Profiler not started")]
    NotStarted,

    #[error("Profile session not found: {0}")]
    SessionNotFound(String),

    #[error("Metric collection failed: {0}")]
    MetricCollectionFailed(String),

    #[error("Analysis failed: {0}")]
    AnalysisFailed(String),
}

/// Performance metrics for a specific operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub operation_name: String,
    pub duration: Duration,
    pub memory_usage: usize,
    pub cpu_usage: f64,
    pub render_time: Duration,
    pub data_processing_time: Duration,
    pub frame_rate: f64,
    pub bottlenecks: Vec<Bottleneck>,
    pub timestamp: u64,
}

/// Identified bottleneck in performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bottleneck {
    pub location: String,
    pub description: String,
    pub severity: BottleneckSeverity,
    pub suggested_fix: String,
    pub impact_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BottleneckSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Performance profile session
#[derive(Debug, Clone)]
pub struct ProfileSession {
    pub id: String,
    pub name: String,
    pub start_time: Instant,
    pub metrics: Vec<PerformanceMetrics>,
    pub active: bool,
}

/// Main performance profiler
pub struct PerformanceProfiler {
    sessions: Arc<Mutex<HashMap<String, ProfileSession>>>,
    current_session: Arc<Mutex<Option<String>>>,
    metric_sender: broadcast::Sender<PerformanceMetrics>,
    enabled: bool,
}

impl PerformanceProfiler {
    /// Create a new performance profiler
    pub fn new() -> Self {
        let (metric_sender, _) = broadcast::channel(1000);

        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            current_session: Arc::new(Mutex::new(None)),
            metric_sender,
            enabled: true,
        }
    }

    /// Start a new profiling session
    pub fn start_session(&mut self, name: &str) -> Result<String, ProfilerError> {
        if !self.enabled {
            return Err(ProfilerError::NotStarted);
        }

        let session_id = uuid::Uuid::new_v4().to_string();
        let session = ProfileSession {
            id: session_id.clone(),
            name: name.to_string(),
            start_time: Instant::now(),
            metrics: Vec::new(),
            active: true,
        };

        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(session_id.clone(), session);

        let mut current = self.current_session.lock().unwrap();
        *current = Some(session_id.clone());

        println!("🔍 Started profiling session: {} ({})", name, session_id);
        Ok(session_id)
    }

    /// Stop the current profiling session
    pub fn stop_session(&mut self, session_id: &str) -> Result<ProfileSession, ProfilerError> {
        let mut sessions = self.sessions.lock().unwrap();
        let session = sessions
            .get_mut(session_id)
            .ok_or_else(|| ProfilerError::SessionNotFound(session_id.to_string()))?;

        session.active = false;
        let result = session.clone();

        let mut current = self.current_session.lock().unwrap();
        if current.as_ref() == Some(&session_id.to_string()) {
            *current = None;
        }

        println!("🏁 Stopped profiling session: {}", session.name);
        Ok(result)
    }

    /// Record performance metrics for an operation
    pub fn record_metrics(
        &self,
        operation_name: &str,
        start_time: Instant,
    ) -> Result<(), ProfilerError> {
        if !self.enabled {
            return Ok(());
        }

        let current_session = self.current_session.lock().unwrap();
        let session_id = current_session.as_ref().ok_or(ProfilerError::NotStarted)?;

        let duration = start_time.elapsed();
        let memory_usage = self.get_memory_usage();
        let cpu_usage = self.get_cpu_usage();
        let bottlenecks = self.identify_bottlenecks(operation_name, duration);

        let metrics = PerformanceMetrics {
            operation_name: operation_name.to_string(),
            duration,
            memory_usage,
            cpu_usage,
            render_time: Duration::from_millis(16), // Mock: 60fps target
            data_processing_time: duration / 3,     // Mock: ~33% of total time
            frame_rate: 1000.0 / duration.as_millis() as f64,
            bottlenecks,
            timestamp: start_time.elapsed().as_millis() as u64,
        };

        // Add metrics to session
        let mut sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.get_mut(session_id) {
            session.metrics.push(metrics.clone());
        }

        // Broadcast metrics
        let _ = self.metric_sender.send(metrics);

        Ok(())
    }

    /// Get comprehensive performance analysis
    pub fn analyze_performance(
        &self,
        session_id: &str,
    ) -> Result<PerformanceAnalysis, ProfilerError> {
        let sessions = self.sessions.lock().unwrap();
        let session = sessions
            .get(session_id)
            .ok_or_else(|| ProfilerError::SessionNotFound(session_id.to_string()))?;

        if session.metrics.is_empty() {
            return Err(ProfilerError::AnalysisFailed(
                "No metrics collected".to_string(),
            ));
        }

        let total_duration: Duration = session.metrics.iter().map(|m| m.duration).sum();

        let avg_memory = session
            .metrics
            .iter()
            .map(|m| m.memory_usage)
            .sum::<usize>()
            / session.metrics.len();

        let avg_frame_rate = session.metrics.iter().map(|m| m.frame_rate).sum::<f64>()
            / session.metrics.len() as f64;

        let critical_bottlenecks: Vec<_> = session
            .metrics
            .iter()
            .flat_map(|m| &m.bottlenecks)
            .filter(|b| {
                matches!(
                    b.severity,
                    BottleneckSeverity::Critical | BottleneckSeverity::High
                )
            })
            .cloned()
            .collect();

        let performance_score = self.calculate_performance_score(&session.metrics);

        let recommendations =
            self.generate_recommendations(&critical_bottlenecks, performance_score);

        Ok(PerformanceAnalysis {
            session_id: session_id.to_string(),
            session_name: session.name.clone(),
            total_duration,
            avg_memory_usage: avg_memory,
            avg_frame_rate,
            performance_score,
            critical_bottlenecks,
            recommendations,
            metrics_count: session.metrics.len(),
        })
    }

    /// Calculate overall performance score (0-100)
    fn calculate_performance_score(&self, metrics: &[PerformanceMetrics]) -> f64 {
        if metrics.is_empty() {
            return 0.0;
        }

        let frame_rate_score = metrics
            .iter()
            .map(|m| (m.frame_rate / 60.0).min(1.0) * 40.0) // 40 points for frame rate
            .sum::<f64>()
            / metrics.len() as f64;

        let memory_score = metrics
            .iter()
            .map(|m| {
                let memory_mb = m.memory_usage as f64 / 1024.0 / 1024.0;
                if memory_mb < 50.0 {
                    30.0
                } else if memory_mb < 100.0 {
                    20.0
                } else {
                    10.0
                }
            })
            .sum::<f64>()
            / metrics.len() as f64; // 30 points for memory

        let bottleneck_score = metrics
            .iter()
            .map(|m| {
                let critical_count = m
                    .bottlenecks
                    .iter()
                    .filter(|b| matches!(b.severity, BottleneckSeverity::Critical))
                    .count();
                if critical_count == 0 {
                    30.0
                } else {
                    30.0 - (critical_count as f64 * 10.0).min(30.0)
                }
            })
            .sum::<f64>()
            / metrics.len() as f64; // 30 points for bottlenecks

        frame_rate_score + memory_score + bottleneck_score
    }

    /// Generate performance improvement recommendations
    fn generate_recommendations(&self, bottlenecks: &[Bottleneck], score: f64) -> Vec<String> {
        let mut recommendations = Vec::new();

        if score < 70.0 {
            recommendations.push("Overall performance is below acceptable levels. Consider comprehensive optimization.".to_string());
        }

        for bottleneck in bottlenecks {
            recommendations.push(format!(
                "{}: {}",
                bottleneck.location, bottleneck.suggested_fix
            ));
        }

        if bottlenecks.iter().any(|b| b.location.contains("render")) {
            recommendations.push(
                "Consider enabling WebGPU acceleration for better rendering performance."
                    .to_string(),
            );
        }

        if bottlenecks.iter().any(|b| b.location.contains("data")) {
            recommendations
                .push("Consider using streaming data processing for large datasets.".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("Performance is within acceptable ranges.".to_string());
        }

        recommendations
    }

    /// Identify performance bottlenecks
    fn identify_bottlenecks(&self, operation: &str, duration: Duration) -> Vec<Bottleneck> {
        let mut bottlenecks = Vec::new();

        // Check for slow operations (>100ms)
        if duration.as_millis() > 100 {
            bottlenecks.push(Bottleneck {
                location: operation.to_string(),
                description: format!(
                    "Operation took {}ms, exceeding recommended 100ms threshold",
                    duration.as_millis()
                ),
                severity: if duration.as_millis() > 500 {
                    BottleneckSeverity::Critical
                } else {
                    BottleneckSeverity::High
                },
                suggested_fix: "Consider optimization or breaking into smaller operations"
                    .to_string(),
                impact_ms: duration.as_millis() as u64 - 100,
            });
        }

        // Mock bottleneck detection based on operation type
        if operation.contains("render") && duration.as_millis() > 16 {
            bottlenecks.push(Bottleneck {
                location: "Render Pipeline".to_string(),
                description: "Rendering exceeds 60fps target".to_string(),
                severity: BottleneckSeverity::Medium,
                suggested_fix: "Enable GPU acceleration or reduce visual complexity".to_string(),
                impact_ms: duration.as_millis() as u64 - 16,
            });
        }

        if operation.contains("data") && duration.as_millis() > 50 {
            bottlenecks.push(Bottleneck {
                location: "Data Processing".to_string(),
                description: "Data processing is slower than expected".to_string(),
                severity: BottleneckSeverity::Medium,
                suggested_fix: "Use Polars lazy evaluation or streaming processing".to_string(),
                impact_ms: duration.as_millis() as u64 - 50,
            });
        }

        bottlenecks
    }

    /// Get current memory usage (mock implementation)
    fn get_memory_usage(&self) -> usize {
        // In real implementation, would use system APIs
        42 * 1024 * 1024 // 42MB mock
    }

    /// Get current CPU usage (mock implementation)
    fn get_cpu_usage(&self) -> f64 {
        // In real implementation, would use system APIs
        25.5 // 25.5% mock
    }

    /// Subscribe to real-time performance metrics
    pub fn subscribe_to_metrics(&self) -> broadcast::Receiver<PerformanceMetrics> {
        self.metric_sender.subscribe()
    }

    /// Get all sessions
    pub fn get_sessions(&self) -> Vec<ProfileSession> {
        let sessions = self.sessions.lock().unwrap();
        sessions.values().cloned().collect()
    }
}

/// Comprehensive performance analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnalysis {
    pub session_id: String,
    pub session_name: String,
    pub total_duration: Duration,
    pub avg_memory_usage: usize,
    pub avg_frame_rate: f64,
    pub performance_score: f64,
    pub critical_bottlenecks: Vec<Bottleneck>,
    pub recommendations: Vec<String>,
    pub metrics_count: usize,
}

/// RAII performance measurement helper
pub struct ProfiledOperation {
    profiler: Arc<PerformanceProfiler>,
    operation_name: String,
    start_time: Instant,
}

impl ProfiledOperation {
    pub fn new(profiler: Arc<PerformanceProfiler>, operation_name: String) -> Self {
        let start_time = Instant::now();
        Self {
            profiler,
            operation_name,
            start_time,
        }
    }
}

impl Drop for ProfiledOperation {
    fn drop(&mut self) {
        let _ = self
            .profiler
            .record_metrics(&self.operation_name, self.start_time);
    }
}

/// Macro for easy performance profiling
#[macro_export]
macro_rules! profile_operation {
    ($profiler:expr, $name:expr, $block:block) => {{
        let _profiled_op = ProfiledOperation::new($profiler.clone(), $name.to_string());
        $block
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use tokio::time::{sleep, Duration as TokioDuration};

    #[tokio::test]
    async fn test_profiler_session_lifecycle() {
        let mut profiler = PerformanceProfiler::new();

        // Start session
        let session_id = profiler.start_session("test_session").unwrap();
        assert!(!session_id.is_empty());

        // Record some metrics
        let start = Instant::now();
        sleep(TokioDuration::from_millis(10)).await;
        profiler.record_metrics("test_operation", start).unwrap();

        // Stop session
        let session = profiler.stop_session(&session_id).unwrap();
        assert_eq!(session.name, "test_session");
        assert!(!session.active);
        assert_eq!(session.metrics.len(), 1);
    }

    #[tokio::test]
    async fn test_bottleneck_detection() {
        let mut profiler = PerformanceProfiler::new();
        let session_id = profiler.start_session("bottleneck_test").unwrap();

        // Simulate slow operation
        let start = Instant::now();
        sleep(TokioDuration::from_millis(200)).await;
        profiler
            .record_metrics("slow_render_operation", start)
            .unwrap();

        let analysis = profiler.analyze_performance(&session_id).unwrap();
        assert!(!analysis.critical_bottlenecks.is_empty());
        assert!(analysis.performance_score < 100.0);
        assert!(!analysis.recommendations.is_empty());
    }

    #[test]
    fn test_performance_score_calculation() {
        let profiler = PerformanceProfiler::new();

        let good_metrics = vec![PerformanceMetrics {
            operation_name: "fast_op".to_string(),
            duration: Duration::from_millis(5),
            memory_usage: 20 * 1024 * 1024, // 20MB
            cpu_usage: 15.0,
            render_time: Duration::from_millis(8),
            data_processing_time: Duration::from_millis(2),
            frame_rate: 120.0, // Above 60fps
            bottlenecks: Vec::new(),
            timestamp: 0,
        }];

        let score = profiler.calculate_performance_score(&good_metrics);
        assert!(
            score > 90.0,
            "Expected high score for good metrics, got {}",
            score
        );
    }

    #[test]
    fn test_profiled_operation_macro() {
        let profiler = Arc::new(PerformanceProfiler::new());
        let mut prof_mut = PerformanceProfiler::new();
        let _session_id = prof_mut.start_session("macro_test").unwrap();

        profile_operation!(profiler, "test_macro_operation", {
            thread::sleep(Duration::from_millis(1));
            42
        });

        // Operation should be automatically recorded when block exits
    }
}
