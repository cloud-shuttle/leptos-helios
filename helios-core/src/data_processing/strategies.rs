//! Data processing strategies and strategy selection
//!
//! This module provides strategy selection logic for optimal data processing,
//! including performance benchmarking and adaptive strategy selection.

use super::errors::{DataError, DataResult};
use super::types::*;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Strategy selector for optimal data processing
pub struct StrategySelector {
    benchmarks: HashMap<String, f64>,
    device_capabilities: DeviceCapabilities,
    performance_history: Vec<PerformanceMetric>,
}

impl Default for StrategySelector {
    fn default() -> Self {
        Self::new()
    }
}

impl StrategySelector {
    /// Create a new strategy selector
    pub fn new() -> Self {
        Self {
            benchmarks: HashMap::new(),
            device_capabilities: DeviceCapabilities::new(),
            performance_history: Vec::new(),
        }
    }

    /// Create a strategy selector with custom device capabilities
    pub fn with_capabilities(capabilities: DeviceCapabilities) -> Self {
        Self {
            benchmarks: HashMap::new(),
            device_capabilities: capabilities,
            performance_history: Vec::new(),
        }
    }

    /// Select the optimal processing strategy for given data
    pub fn select_strategy(&self, data_spec: &DataSpec) -> DataResult<ProcessingStrategy> {
        // Simple strategy selection based on data size and device capabilities
        if data_spec.is_small() {
            Ok(ProcessingStrategy::CPU(RayonConfig::new()))
        } else if data_spec.is_large()
            && self.device_capabilities.should_use_gpu(data_spec.row_count)
        {
            Ok(ProcessingStrategy::GPU(ComputeConfig::new()))
        } else if self
            .device_capabilities
            .should_use_hybrid(data_spec.row_count)
        {
            Ok(ProcessingStrategy::Hybrid(HybridConfig::new()))
        } else {
            Ok(ProcessingStrategy::Streaming(StreamConfig::new()))
        }
    }

    /// Record performance metrics for a strategy
    pub fn record_performance(&mut self, metric: PerformanceMetric) {
        self.performance_history.push(metric);

        // Keep only recent history (last 1000 entries)
        if self.performance_history.len() > 1000 {
            self.performance_history
                .drain(0..self.performance_history.len() - 1000);
        }
    }

    /// Get performance benchmark for a strategy
    pub fn get_benchmark(&self, strategy_name: &str) -> Option<f64> {
        self.benchmarks.get(strategy_name).copied()
    }

    /// Update benchmark for a strategy
    pub fn update_benchmark(&mut self, strategy_name: String, performance: f64) {
        self.benchmarks.insert(strategy_name, performance);
    }

    /// Get the best performing strategy for similar data
    pub fn get_best_strategy(&self, data_size: usize) -> Option<String> {
        let similar_metrics: Vec<&PerformanceMetric> = self
            .performance_history
            .iter()
            .filter(|m| (m.data_size as f64 - data_size as f64).abs() / (data_size as f64) < 0.1)
            .collect();

        if similar_metrics.is_empty() {
            return None;
        }

        let best_metric = similar_metrics
            .iter()
            .max_by(|a, b| a.throughput().partial_cmp(&b.throughput()).unwrap())?;

        Some(best_metric.strategy.clone())
    }

    /// Get performance statistics for a strategy
    pub fn get_strategy_stats(&self, strategy_name: &str) -> StrategyStats {
        let metrics: Vec<&PerformanceMetric> = self
            .performance_history
            .iter()
            .filter(|m| m.strategy == strategy_name)
            .collect();

        if metrics.is_empty() {
            return StrategyStats::new(strategy_name);
        }

        let mut stats = StrategyStats::new(strategy_name);

        for metric in metrics {
            stats.add_metric(metric);
        }

        stats
    }

    /// Get device capabilities
    pub fn device_capabilities(&self) -> &DeviceCapabilities {
        &self.device_capabilities
    }

    /// Update device capabilities
    pub fn update_capabilities(&mut self, capabilities: DeviceCapabilities) {
        self.device_capabilities = capabilities;
    }

    /// Clear performance history
    pub fn clear_history(&mut self) {
        self.performance_history.clear();
    }

    /// Get performance history
    pub fn performance_history(&self) -> &[PerformanceMetric] {
        &self.performance_history
    }
}

/// Performance statistics for a strategy
#[derive(Debug, Clone)]
pub struct StrategyStats {
    pub strategy_name: String,
    pub total_executions: usize,
    pub average_duration: f64,
    pub min_duration: f64,
    pub max_duration: f64,
    pub average_throughput: f64,
    pub average_memory_usage: usize,
    pub average_cpu_usage: f64,
    pub average_gpu_usage: Option<f64>,
    pub success_rate: f64,
    pub last_updated: SystemTime,
}

impl StrategyStats {
    /// Create new strategy statistics
    pub fn new(strategy_name: impl Into<String>) -> Self {
        Self {
            strategy_name: strategy_name.into(),
            total_executions: 0,
            average_duration: 0.0,
            min_duration: f64::INFINITY,
            max_duration: 0.0,
            average_throughput: 0.0,
            average_memory_usage: 0,
            average_cpu_usage: 0.0,
            average_gpu_usage: None,
            success_rate: 0.0,
            last_updated: SystemTime::now(),
        }
    }

    /// Add a performance metric
    pub fn add_metric(&mut self, metric: &PerformanceMetric) {
        self.total_executions += 1;

        // Update duration statistics
        self.average_duration = (self.average_duration * (self.total_executions - 1) as f64
            + metric.duration)
            / self.total_executions as f64;
        self.min_duration = self.min_duration.min(metric.duration);
        self.max_duration = self.max_duration.max(metric.duration);

        // Update throughput
        self.average_throughput = (self.average_throughput * (self.total_executions - 1) as f64
            + metric.throughput())
            / self.total_executions as f64;

        // Update memory usage
        self.average_memory_usage = (self.average_memory_usage * (self.total_executions - 1)
            + metric.memory_usage)
            / self.total_executions;

        // Update CPU usage
        self.average_cpu_usage = (self.average_cpu_usage * (self.total_executions - 1) as f64
            + metric.cpu_usage)
            / self.total_executions as f64;

        // Update GPU usage
        if let Some(gpu_usage) = metric.gpu_usage {
            match self.average_gpu_usage {
                Some(avg) => {
                    self.average_gpu_usage = Some(
                        (avg * (self.total_executions - 1) as f64 + gpu_usage)
                            / self.total_executions as f64,
                    );
                }
                None => {
                    self.average_gpu_usage = Some(gpu_usage);
                }
            }
        }

        self.last_updated = SystemTime::now();
    }

    /// Get efficiency score (throughput per resource usage)
    pub fn efficiency_score(&self) -> f64 {
        let resource_usage = self.average_cpu_usage + self.average_gpu_usage.unwrap_or(0.0);
        if resource_usage > 0.0 {
            self.average_throughput / resource_usage
        } else {
            0.0
        }
    }

    /// Get reliability score (based on success rate and consistency)
    pub fn reliability_score(&self) -> f64 {
        if self.total_executions == 0 {
            return 0.0;
        }

        let consistency = if self.max_duration > 0.0 {
            1.0 - (self.max_duration - self.min_duration) / self.max_duration
        } else {
            1.0
        };

        self.success_rate * consistency
    }

    /// Get overall score (combination of efficiency and reliability)
    pub fn overall_score(&self) -> f64 {
        let efficiency = self.efficiency_score();
        let reliability = self.reliability_score();

        // Weighted combination: 60% efficiency, 40% reliability
        0.6 * efficiency + 0.4 * reliability
    }
}

/// Strategy recommendation engine
pub struct StrategyRecommender {
    selector: StrategySelector,
    learning_rate: f64,
    exploration_rate: f64,
}

impl StrategyRecommender {
    /// Create a new strategy recommender
    pub fn new() -> Self {
        Self {
            selector: StrategySelector::new(),
            learning_rate: 0.1,
            exploration_rate: 0.1,
        }
    }

    /// Create a strategy recommender with custom parameters
    pub fn with_parameters(learning_rate: f64, exploration_rate: f64) -> Self {
        Self {
            selector: StrategySelector::new(),
            learning_rate,
            exploration_rate,
        }
    }

    /// Recommend a strategy for given data
    pub fn recommend_strategy(&self, data_spec: &DataSpec) -> DataResult<ProcessingStrategy> {
        // Check if we have historical data for similar workloads
        if let Some(best_strategy) = self.selector.get_best_strategy(data_spec.row_count) {
            // Use historical best strategy
            self.strategy_from_name(&best_strategy, data_spec)
        } else {
            // Use exploration or default strategy selection
            if rand::random::<f64>() < self.exploration_rate {
                self.explore_strategy(data_spec)
            } else {
                self.selector.select_strategy(data_spec)
            }
        }
    }

    /// Learn from performance feedback
    pub fn learn(&mut self, strategy_name: &str, performance: f64, data_spec: &DataSpec) {
        // Update benchmark
        let current_benchmark = self.selector.get_benchmark(strategy_name).unwrap_or(0.0);
        let new_benchmark =
            current_benchmark + self.learning_rate * (performance - current_benchmark);
        self.selector
            .update_benchmark(strategy_name.to_string(), new_benchmark);
    }

    /// Get strategy selector
    pub fn selector(&self) -> &StrategySelector {
        &self.selector
    }

    /// Get mutable strategy selector
    pub fn selector_mut(&mut self) -> &mut StrategySelector {
        &mut self.selector
    }

    /// Set learning rate
    pub fn set_learning_rate(&mut self, rate: f64) {
        self.learning_rate = rate.clamp(0.0, 1.0);
    }

    /// Set exploration rate
    pub fn set_exploration_rate(&mut self, rate: f64) {
        self.exploration_rate = rate.clamp(0.0, 1.0);
    }

    /// Explore a random strategy
    fn explore_strategy(&self, data_spec: &DataSpec) -> DataResult<ProcessingStrategy> {
        let strategies = vec![
            ProcessingStrategy::CPU(RayonConfig::new()),
            ProcessingStrategy::GPU(ComputeConfig::new()),
            ProcessingStrategy::Streaming(StreamConfig::new()),
            ProcessingStrategy::Hybrid(HybridConfig::new()),
        ];

        let index = rand::random::<usize>() % strategies.len();
        Ok(strategies[index].clone())
    }

    /// Convert strategy name to strategy instance
    fn strategy_from_name(
        &self,
        name: &str,
        data_spec: &DataSpec,
    ) -> DataResult<ProcessingStrategy> {
        match name {
            "CPU" => Ok(ProcessingStrategy::CPU(RayonConfig::new())),
            "GPU" => Ok(ProcessingStrategy::GPU(ComputeConfig::new())),
            "Streaming" => Ok(ProcessingStrategy::Streaming(StreamConfig::new())),
            "Hybrid" => Ok(ProcessingStrategy::Hybrid(HybridConfig::new())),
            _ => Err(DataError::processing(format!("Unknown strategy: {}", name))),
        }
    }
}

impl Default for StrategyRecommender {
    fn default() -> Self {
        Self::new()
    }
}
