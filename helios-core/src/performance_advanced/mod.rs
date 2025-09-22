//! Performance Optimization Module
//!
//! This module provides comprehensive performance optimizations for leptos-helios,
//! including runtime optimizations, algorithmic improvements, and memory management.

use std::collections::HashMap;
use std::time::Duration;

pub mod runtime_optimizations;
pub mod algorithmic_optimizations;
pub mod memory_optimizations;

// Re-export main types for convenience
pub use runtime_optimizations::*;
pub use algorithmic_optimizations::*;
pub use memory_optimizations::*;

// Disambiguate conflicting types
pub use runtime_optimizations::PoolStats as RuntimePoolStats;
pub use memory_optimizations::PoolStats as MemoryPoolStats;

/// Performance optimization manager that coordinates all optimization strategies
pub struct PerformanceOptimizer {
    profiler: PerformanceProfiler,
    memory_pool: MemoryPool<Vec<f64>>,
    cache: LruCache<String, Vec<f64>>,
    string_interner: StringInterner,
}

impl PerformanceOptimizer {
    pub fn new() -> Self {
        Self {
            profiler: PerformanceProfiler::new(),
            memory_pool: MemoryPool::new(|| Vec::with_capacity(1000), 100),
            cache: LruCache::new(1000, std::time::Duration::from_secs(300)), // 5 minutes TTL
            string_interner: StringInterner::new(),
        }
    }

    /// Optimize data processing with caching and memory pooling
    pub fn optimize_data_processing(&self, data: &[f64], operation: &str) -> Vec<f64> {
        self.profiler.start_timer("data_processing");
        
        // Check cache first
        let cache_key = format!("{}_{}", operation, data.len());
        if let Some(cached_result) = self.cache.get(&cache_key) {
            self.profiler.end_timer("data_processing");
            return cached_result;
        }
        
        // Process data
        let mut result = self.memory_pool.acquire();
        result.clear();
        result.reserve(data.len());
        
        match operation {
            "sort" => {
                let mut sorted_data = data.to_vec();
                optimized_quicksort(&mut sorted_data, |a, b| a.partial_cmp(b).unwrap());
                result.extend_from_slice(&sorted_data);
            }
            "moving_average" => {
                let window_size = 10.min(data.len());
                let ma_result = optimized_moving_average(data, window_size);
                result.extend_from_slice(&ma_result);
            }
            "percentile" => {
                let mut sorted_data = data.to_vec();
                let p50 = fast_percentile(&mut sorted_data, 50.0);
                result.push(p50);
            }
            _ => {
                result.extend_from_slice(data);
            }
        }
        
        // Cache result
        self.cache.insert(cache_key, result.clone());
        
        self.profiler.end_timer("data_processing");
        result
    }

    /// Get performance statistics
    pub fn get_performance_stats(&self) -> PerformanceOptimizerStats {
        let profiler_stats = self.profiler.get_stats();
        let pool_stats = self.memory_pool.get_stats();
        let cache_stats = self.cache.get_stats();
        let interner_stats = self.string_interner.get_stats();
        
        PerformanceOptimizerStats {
            profiler_stats,
            pool_stats,
            cache_stats,
            interner_stats,
        }
    }

    /// Clean up resources
    pub fn cleanup(&self) {
        self.memory_pool.clear();
        self.cache.clear();
    }
}

#[derive(Debug, Clone)]
pub struct PerformanceOptimizerStats {
    pub profiler_stats: HashMap<String, PerformanceStats>,
    pub pool_stats: runtime_optimizations::PoolStats,
    pub cache_stats: CacheStats,
    pub interner_stats: InternerStats,
}

impl Default for PerformanceOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Benchmark utilities for performance testing
pub struct PerformanceBenchmark {
    results: HashMap<String, Vec<Duration>>,
}

impl PerformanceBenchmark {
    pub fn new() -> Self {
        Self {
            results: HashMap::new(),
        }
    }

    pub fn benchmark<F>(&mut self, name: &str, iterations: usize, f: F)
    where
        F: Fn(),
    {
        let mut times = Vec::new();
        
        for _ in 0..iterations {
            let start = std::time::Instant::now();
            f();
            times.push(start.elapsed());
        }
        
        self.results.insert(name.to_string(), times);
    }

    pub fn get_results(&self) -> &HashMap<String, Vec<Duration>> {
        &self.results
    }

    pub fn get_summary(&self) -> HashMap<String, BenchmarkSummary> {
        let mut summary = HashMap::new();
        
        for (name, times) in &self.results {
            if !times.is_empty() {
                let total: Duration = times.iter().sum();
                let average = total / times.len() as u32;
                let min = *times.iter().min().unwrap();
                let max = *times.iter().max().unwrap();
                
                summary.insert(name.clone(), BenchmarkSummary {
                    iterations: times.len(),
                    total_time: total,
                    average_time: average,
                    min_time: min,
                    max_time: max,
                });
            }
        }
        
        summary
    }
}

#[derive(Debug, Clone)]
pub struct BenchmarkSummary {
    pub iterations: usize,
    pub total_time: Duration,
    pub average_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
}

impl Default for PerformanceBenchmark {
    fn default() -> Self {
        Self::new()
    }
}
