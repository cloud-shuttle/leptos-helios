//! Core data processing types and data structures
//!
//! This module provides the fundamental types used throughout the data processing system,
//! including processing strategies, configurations, and data specifications.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Data processing strategy selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessingStrategy {
    CPU(RayonConfig),
    GPU(ComputeConfig),
    Streaming(StreamConfig),
    Hybrid(HybridConfig),
}

impl ProcessingStrategy {
    /// Get the name of the strategy
    pub fn name(&self) -> &'static str {
        match self {
            ProcessingStrategy::CPU(_) => "CPU",
            ProcessingStrategy::GPU(_) => "GPU",
            ProcessingStrategy::Streaming(_) => "Streaming",
            ProcessingStrategy::Hybrid(_) => "Hybrid",
        }
    }

    /// Check if this strategy supports parallel processing
    pub fn supports_parallel(&self) -> bool {
        match self {
            ProcessingStrategy::CPU(_)
            | ProcessingStrategy::GPU(_)
            | ProcessingStrategy::Hybrid(_) => true,
            ProcessingStrategy::Streaming(_) => false,
        }
    }

    /// Get estimated memory usage for this strategy
    pub fn estimated_memory_usage(&self, data_size: usize) -> usize {
        match self {
            ProcessingStrategy::CPU(config) => {
                let threads = config.num_threads.unwrap_or(num_cpus::get());
                data_size / threads
            }
            ProcessingStrategy::GPU(config) => config.memory_budget,
            ProcessingStrategy::Streaming(config) => config.buffer_size,
            ProcessingStrategy::Hybrid(config) => config
                .cpu_config
                .memory_budget()
                .max(config.gpu_config.memory_budget),
        }
    }
}

/// CPU processing configuration using Rayon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RayonConfig {
    pub num_threads: Option<usize>,
    pub chunk_size: Option<usize>,
    pub enable_simd: bool,
}

impl RayonConfig {
    /// Create a new Rayon configuration
    pub fn new() -> Self {
        Self {
            num_threads: None,
            chunk_size: None,
            enable_simd: true,
        }
    }

    /// Set number of threads
    pub fn with_threads(mut self, threads: usize) -> Self {
        self.num_threads = Some(threads);
        self
    }

    /// Set chunk size
    pub fn with_chunk_size(mut self, size: usize) -> Self {
        self.chunk_size = Some(size);
        self
    }

    /// Enable or disable SIMD
    pub fn with_simd(mut self, enable: bool) -> Self {
        self.enable_simd = enable;
        self
    }

    /// Get memory budget for this configuration
    pub fn memory_budget(&self) -> usize {
        // Estimate memory usage based on configuration
        let threads = self.num_threads.unwrap_or(num_cpus::get());
        let chunk_size = self.chunk_size.unwrap_or(1024);
        threads * chunk_size * 8 // Assume 8 bytes per element
    }
}

/// GPU processing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeConfig {
    pub workgroup_size: u32,
    pub memory_budget: usize,
    pub enable_shared_memory: bool,
}

impl ComputeConfig {
    /// Create a new compute configuration
    pub fn new() -> Self {
        Self {
            workgroup_size: 64,
            memory_budget: 1024 * 1024 * 1024, // 1GB
            enable_shared_memory: true,
        }
    }

    /// Set workgroup size
    pub fn with_workgroup_size(mut self, size: u32) -> Self {
        self.workgroup_size = size;
        self
    }

    /// Set memory budget
    pub fn with_memory_budget(mut self, budget: usize) -> Self {
        self.memory_budget = budget;
        self
    }

    /// Enable or disable shared memory
    pub fn with_shared_memory(mut self, enable: bool) -> Self {
        self.enable_shared_memory = enable;
        self
    }
}

/// Streaming processing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamConfig {
    pub buffer_size: usize,
    pub batch_size: usize,
    pub enable_backpressure: bool,
    pub compression: bool,
}

impl StreamConfig {
    /// Create a new stream configuration
    pub fn new() -> Self {
        Self {
            buffer_size: 1024 * 1024, // 1MB
            batch_size: 1000,
            enable_backpressure: true,
            compression: false,
        }
    }

    /// Set buffer size
    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    /// Set batch size
    pub fn with_batch_size(mut self, size: usize) -> Self {
        self.batch_size = size;
        self
    }

    /// Enable or disable backpressure
    pub fn with_backpressure(mut self, enable: bool) -> Self {
        self.enable_backpressure = enable;
        self
    }

    /// Enable or disable compression
    pub fn with_compression(mut self, enable: bool) -> Self {
        self.compression = enable;
        self
    }
}

/// Hybrid processing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridConfig {
    pub cpu_threshold: usize,
    pub gpu_threshold: usize,
    pub cpu_config: RayonConfig,
    pub gpu_config: ComputeConfig,
}

impl HybridConfig {
    /// Create a new hybrid configuration
    pub fn new() -> Self {
        Self {
            cpu_threshold: 1000,
            gpu_threshold: 10000,
            cpu_config: RayonConfig::new(),
            gpu_config: ComputeConfig::new(),
        }
    }

    /// Set CPU threshold
    pub fn with_cpu_threshold(mut self, threshold: usize) -> Self {
        self.cpu_threshold = threshold;
        self
    }

    /// Set GPU threshold
    pub fn with_gpu_threshold(mut self, threshold: usize) -> Self {
        self.gpu_threshold = threshold;
        self
    }

    /// Set CPU configuration
    pub fn with_cpu_config(mut self, config: RayonConfig) -> Self {
        self.cpu_config = config;
        self
    }

    /// Set GPU configuration
    pub fn with_gpu_config(mut self, config: ComputeConfig) -> Self {
        self.gpu_config = config;
        self
    }
}

/// Performance metric for strategy evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetric {
    pub strategy: String,
    pub data_size: usize,
    pub duration: f64,
    pub timestamp: SystemTime,
    pub memory_usage: usize,
    pub cpu_usage: f64,
    pub gpu_usage: Option<f64>,
}

impl PerformanceMetric {
    /// Create a new performance metric
    pub fn new(strategy: impl Into<String>, data_size: usize, duration: f64) -> Self {
        Self {
            strategy: strategy.into(),
            data_size,
            duration,
            timestamp: SystemTime::now(),
            memory_usage: 0,
            cpu_usage: 0.0,
            gpu_usage: None,
        }
    }

    /// Set memory usage
    pub fn with_memory_usage(mut self, usage: usize) -> Self {
        self.memory_usage = usage;
        self
    }

    /// Set CPU usage
    pub fn with_cpu_usage(mut self, usage: f64) -> Self {
        self.cpu_usage = usage;
        self
    }

    /// Set GPU usage
    pub fn with_gpu_usage(mut self, usage: f64) -> Self {
        self.gpu_usage = Some(usage);
        self
    }

    /// Calculate throughput (operations per second)
    pub fn throughput(&self) -> f64 {
        if self.duration > 0.0 {
            self.data_size as f64 / self.duration
        } else {
            0.0
        }
    }

    /// Calculate efficiency (throughput per resource usage)
    pub fn efficiency(&self) -> f64 {
        let resource_usage = self.cpu_usage + self.gpu_usage.unwrap_or(0.0);
        if resource_usage > 0.0 {
            self.throughput() / resource_usage
        } else {
            0.0
        }
    }
}

/// Device capabilities for strategy selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCapabilities {
    pub cpu_cores: usize,
    pub cpu_frequency: f64,
    pub memory_total: usize,
    pub memory_available: usize,
    pub gpu_available: bool,
    pub gpu_memory: Option<usize>,
    pub gpu_compute_units: Option<usize>,
    pub simd_support: bool,
    pub avx_support: bool,
    pub sse_support: bool,
}

impl DeviceCapabilities {
    /// Create new device capabilities
    pub fn new() -> Self {
        Self {
            cpu_cores: num_cpus::get(),
            cpu_frequency: 0.0,  // Would need system-specific code to get this
            memory_total: 0,     // Would need system-specific code to get this
            memory_available: 0, // Would need system-specific code to get this
            gpu_available: false,
            gpu_memory: None,
            gpu_compute_units: None,
            simd_support: true,
            avx_support: true,
            sse_support: true,
        }
    }

    /// Check if GPU processing is recommended
    pub fn should_use_gpu(&self, data_size: usize) -> bool {
        self.gpu_available
            && data_size > 10000
            && self.gpu_memory.map_or(false, |mem| mem > data_size * 8)
    }

    /// Check if CPU processing is recommended
    pub fn should_use_cpu(&self, data_size: usize) -> bool {
        data_size < 1000 || !self.gpu_available
    }

    /// Check if hybrid processing is recommended
    pub fn should_use_hybrid(&self, data_size: usize) -> bool {
        self.gpu_available && data_size > 1000 && data_size < 100000
    }

    /// Get optimal number of CPU threads
    pub fn optimal_cpu_threads(&self) -> usize {
        self.cpu_cores
    }

    /// Get optimal chunk size for CPU processing
    pub fn optimal_chunk_size(&self) -> usize {
        if self.avx_support {
            1024
        } else if self.sse_support {
            512
        } else {
            256
        }
    }
}

/// Data specification for processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSpec {
    pub columns: Vec<ColumnSpec>,
    pub row_count: usize,
    pub memory_size: usize,
    pub data_types: HashMap<String, String>,
    pub nullable_columns: Vec<String>,
    pub indexed_columns: Vec<String>,
}

impl DataSpec {
    /// Create a new data specification
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            row_count: 0,
            memory_size: 0,
            data_types: HashMap::new(),
            nullable_columns: Vec::new(),
            indexed_columns: Vec::new(),
        }
    }

    /// Add a column specification
    pub fn add_column(&mut self, column: ColumnSpec) {
        self.columns.push(column);
    }

    /// Set row count
    pub fn with_row_count(mut self, count: usize) -> Self {
        self.row_count = count;
        self
    }

    /// Set memory size
    pub fn with_memory_size(mut self, size: usize) -> Self {
        self.memory_size = size;
        self
    }

    /// Add data type information
    pub fn add_data_type(&mut self, column: String, data_type: String) {
        self.data_types.insert(column, data_type);
    }

    /// Add nullable column
    pub fn add_nullable_column(&mut self, column: String) {
        if !self.nullable_columns.contains(&column) {
            self.nullable_columns.push(column);
        }
    }

    /// Add indexed column
    pub fn add_indexed_column(&mut self, column: String) {
        if !self.indexed_columns.contains(&column) {
            self.indexed_columns.push(column);
        }
    }

    /// Get column count
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Check if data is small
    pub fn is_small(&self) -> bool {
        self.row_count < 1000
    }

    /// Check if data is large
    pub fn is_large(&self) -> bool {
        self.row_count > 100000
    }

    /// Get estimated processing time
    pub fn estimated_processing_time(&self) -> Duration {
        // Rough estimation based on data size
        let base_time = self.row_count as u64 * 100; // 100ns per row
        Duration::from_nanos(base_time)
    }
}

/// Column specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnSpec {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub indexed: bool,
    pub unique: bool,
    pub min_value: Option<String>,
    pub max_value: Option<String>,
    pub distinct_count: Option<usize>,
}

impl ColumnSpec {
    /// Create a new column specification
    pub fn new(name: impl Into<String>, data_type: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data_type: data_type.into(),
            nullable: false,
            indexed: false,
            unique: false,
            min_value: None,
            max_value: None,
            distinct_count: None,
        }
    }

    /// Set nullable flag
    pub fn with_nullable(mut self, nullable: bool) -> Self {
        self.nullable = nullable;
        self
    }

    /// Set indexed flag
    pub fn with_indexed(mut self, indexed: bool) -> Self {
        self.indexed = indexed;
        self
    }

    /// Set unique flag
    pub fn with_unique(mut self, unique: bool) -> Self {
        self.unique = unique;
        self
    }

    /// Set value range
    pub fn with_value_range(mut self, min: impl Into<String>, max: impl Into<String>) -> Self {
        self.min_value = Some(min.into());
        self.max_value = Some(max.into());
        self
    }

    /// Set distinct count
    pub fn with_distinct_count(mut self, count: usize) -> Self {
        self.distinct_count = Some(count);
        self
    }
}
