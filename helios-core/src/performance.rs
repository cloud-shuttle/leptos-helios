//! Performance optimization module for Helios
//! Provides SIMD support, caching, memory pooling, and parallel processing

use std::alloc::{GlobalAlloc, Layout, System};
use std::collections::HashMap;
use std::hash::Hash;
use std::ptr::NonNull;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime};

// SIMD support
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

// Parallel processing
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

/// Performance optimization configuration
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    pub simd_enabled: bool,
    pub cache_enabled: bool,
    pub memory_pool_enabled: bool,
    pub parallel_processing: bool,
    pub max_cache_size: usize,
    pub memory_pool_size: usize,
    pub work_stealing_enabled: bool,
    pub profiling_enabled: bool,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            simd_enabled: true,
            cache_enabled: true,
            memory_pool_enabled: true,
            parallel_processing: true,
            max_cache_size: 100 * 1024 * 1024,  // 100MB
            memory_pool_size: 50 * 1024 * 1024, // 50MB
            work_stealing_enabled: true,
            profiling_enabled: true,
        }
    }
}

/// SIMD vector operations for data processing
pub struct SimdProcessor {
    config: PerformanceConfig,
    capabilities: SimdCapabilities,
}

impl SimdProcessor {
    pub fn new(config: PerformanceConfig) -> Self {
        let capabilities = SimdCapabilities::detect();
        Self {
            config,
            capabilities,
        }
    }

    /// Vectorized sum operation
    pub fn vectorized_sum(&self, data: &[f32]) -> f32 {
        if !self.config.simd_enabled || !self.capabilities.sse2_available {
            return data.iter().sum();
        }

        #[cfg(target_arch = "x86_64")]
        unsafe {
            self.sse2_sum(data)
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            data.iter().sum()
        }
    }

    /// Vectorized mean operation
    pub fn vectorized_mean(&self, data: &[f32]) -> f32 {
        if data.is_empty() {
            return 0.0;
        }
        self.vectorized_sum(data) / data.len() as f32
    }

    /// Vectorized standard deviation
    pub fn vectorized_std(&self, data: &[f32]) -> f32 {
        if data.len() < 2 {
            return 0.0;
        }

        let mean = self.vectorized_mean(data);
        let variance = self.vectorized_variance(data, mean);
        variance.sqrt()
    }

    /// Vectorized variance calculation
    pub fn vectorized_variance(&self, data: &[f32], mean: f32) -> f32 {
        if data.len() < 2 {
            return 0.0;
        }

        if !self.config.simd_enabled || !self.capabilities.sse2_available {
            return data.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / (data.len() - 1) as f32;
        }

        #[cfg(target_arch = "x86_64")]
        unsafe {
            self.sse2_variance(data, mean)
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            data.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / (data.len() - 1) as f32
        }
    }

    /// Vectorized filtering operation
    pub fn vectorized_filter(&self, data: &[f32], threshold: f32) -> Vec<f32> {
        if !self.config.simd_enabled || !self.capabilities.sse2_available {
            return data.iter().filter(|&&x| x > threshold).copied().collect();
        }

        #[cfg(target_arch = "x86_64")]
        unsafe {
            self.sse2_filter(data, threshold)
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            data.iter().filter(|&&x| x > threshold).copied().collect()
        }
    }

    #[cfg(target_arch = "x86_64")]
    unsafe fn sse2_sum(&self, data: &[f32]) -> f32 {
        let mut sum = _mm_setzero_ps();
        let chunks = data.chunks_exact(4);
        let remainder = chunks.remainder();

        for chunk in chunks {
            let values = _mm_loadu_ps(chunk.as_ptr());
            sum = _mm_add_ps(sum, values);
        }

        // Horizontal sum
        let sum_array = std::mem::transmute::<__m128, [f32; 4]>(sum);
        let mut result = sum_array[0] + sum_array[1] + sum_array[2] + sum_array[3];

        // Add remainder
        for &value in remainder {
            result += value;
        }

        result
    }

    #[cfg(target_arch = "x86_64")]
    unsafe fn sse2_variance(&self, data: &[f32], mean: f32) -> f32 {
        let mean_vec = _mm_set1_ps(mean);
        let mut sum_squared_diff = _mm_setzero_ps();
        let chunks = data.chunks_exact(4);
        let remainder = chunks.remainder();

        for chunk in chunks {
            let values = _mm_loadu_ps(chunk.as_ptr());
            let diff = _mm_sub_ps(values, mean_vec);
            let squared_diff = _mm_mul_ps(diff, diff);
            sum_squared_diff = _mm_add_ps(sum_squared_diff, squared_diff);
        }

        // Horizontal sum
        let sum_array = std::mem::transmute::<__m128, [f32; 4]>(sum_squared_diff);
        let mut result = sum_array[0] + sum_array[1] + sum_array[2] + sum_array[3];

        // Add remainder
        for &value in remainder {
            let diff = value - mean;
            result += diff * diff;
        }

        result / (data.len() - 1) as f32
    }

    #[cfg(target_arch = "x86_64")]
    unsafe fn sse2_filter(&self, data: &[f32], threshold: f32) -> Vec<f32> {
        let threshold_vec = _mm_set1_ps(threshold);
        let mut result = Vec::with_capacity(data.len());

        let chunks = data.chunks_exact(4);
        let remainder = chunks.remainder();

        for chunk in chunks {
            let values = _mm_loadu_ps(chunk.as_ptr());
            let mask = _mm_cmpgt_ps(values, threshold_vec);
            let mask_array = std::mem::transmute::<__m128, [f32; 4]>(mask);

            for (i, &value) in chunk.iter().enumerate() {
                if mask_array[i] != 0.0 {
                    result.push(value);
                }
            }
        }

        // Process remainder
        for &value in remainder {
            if value > threshold {
                result.push(value);
            }
        }

        result
    }
}

/// SIMD capabilities detection
#[derive(Debug, Clone)]
pub struct SimdCapabilities {
    pub sse2_available: bool,
    pub sse3_available: bool,
    pub sse4_1_available: bool,
    pub sse4_2_available: bool,
    pub avx_available: bool,
    pub avx2_available: bool,
    pub neon_available: bool,
}

impl SimdCapabilities {
    pub fn detect() -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            Self {
                sse2_available: is_x86_feature_detected!("sse2"),
                sse3_available: is_x86_feature_detected!("sse3"),
                sse4_1_available: is_x86_feature_detected!("sse4.1"),
                sse4_2_available: is_x86_feature_detected!("sse4.2"),
                avx_available: is_x86_feature_detected!("avx"),
                avx2_available: is_x86_feature_detected!("avx2"),
                neon_available: false,
            }
        }
        #[cfg(target_arch = "aarch64")]
        {
            Self {
                sse2_available: false,
                sse3_available: false,
                sse4_1_available: false,
                sse4_2_available: false,
                avx_available: false,
                avx2_available: false,
                neon_available: true, // Assume NEON is available on aarch64
            }
        }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            Self {
                sse2_available: false,
                sse3_available: false,
                sse4_1_available: false,
                sse4_2_available: false,
                avx_available: false,
                avx2_available: false,
                neon_available: false,
            }
        }
    }
}

/// Intelligent caching system
pub struct CacheManager {
    cache: Arc<RwLock<HashMap<CacheKey, CacheEntry>>>,
    config: PerformanceConfig,
    stats: Arc<Mutex<CacheStats>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheKey {
    pub operation_type: String,
    pub data_hash: u64,
    pub parameters_hash: u64,
}

#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub data: Vec<u8>,
    pub timestamp: SystemTime,
    pub access_count: u64,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub total_size: usize,
}

impl CacheManager {
    pub fn new(config: PerformanceConfig) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            config,
            stats: Arc::new(Mutex::new(CacheStats {
                hits: 0,
                misses: 0,
                evictions: 0,
                total_size: 0,
            })),
        }
    }

    pub fn get<T>(&self, key: &CacheKey) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let cache = self.cache.read().ok()?;
        let entry = cache.get(key)?;

        // Update access count
        let mut stats = self.stats.lock().ok()?;
        stats.hits += 1;

        // Deserialize data
        bincode::deserialize(&entry.data).ok()
    }

    pub fn put<T>(&self, key: CacheKey, value: &T) -> Result<(), CacheError>
    where
        T: serde::Serialize,
    {
        let serialized =
            bincode::serialize(value).map_err(|e| CacheError::Serialization(e.to_string()))?;

        let entry = CacheEntry {
            data: serialized.clone(),
            timestamp: SystemTime::now(),
            access_count: 0,
            size: serialized.len(),
        };

        let mut cache = self
            .cache
            .write()
            .map_err(|e| CacheError::LockError(e.to_string()))?;

        // Check if we need to evict entries
        self.evict_if_needed(&mut cache, entry.size)?;

        cache.insert(key, entry);

        let mut stats = self
            .stats
            .lock()
            .map_err(|e| CacheError::LockError(e.to_string()))?;
        stats.total_size += serialized.len();

        Ok(())
    }

    fn evict_if_needed(
        &self,
        cache: &mut HashMap<CacheKey, CacheEntry>,
        new_size: usize,
    ) -> Result<(), CacheError> {
        let mut stats = self
            .stats
            .lock()
            .map_err(|e| CacheError::LockError(e.to_string()))?;

        if stats.total_size + new_size > self.config.max_cache_size {
            // Evict least recently used entries
            let mut entries: Vec<_> = cache.iter().collect();
            entries.sort_by_key(|(_, entry)| entry.timestamp);

            let mut to_remove = Vec::new();
            let mut freed_size = 0;

            for (key, entry) in entries {
                to_remove.push(key.clone());
                freed_size += entry.size;

                if stats.total_size + new_size - freed_size <= self.config.max_cache_size {
                    break;
                }
            }

            for key in to_remove {
                if let Some(entry) = cache.remove(&key) {
                    stats.total_size -= entry.size;
                    stats.evictions += 1;
                }
            }
        }

        Ok(())
    }

    pub fn get_stats(&self) -> Result<CacheStats, CacheError> {
        let stats = self
            .stats
            .lock()
            .map_err(|e| CacheError::LockError(e.to_string()))?;
        Ok(stats.clone())
    }

    pub fn clear(&self) -> Result<(), CacheError> {
        let mut cache = self
            .cache
            .write()
            .map_err(|e| CacheError::LockError(e.to_string()))?;
        cache.clear();

        let mut stats = self
            .stats
            .lock()
            .map_err(|e| CacheError::LockError(e.to_string()))?;
        stats.total_size = 0;

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Lock error: {0}")]
    LockError(String),

    #[error("Cache full")]
    CacheFull,
}

/// Memory pool for efficient buffer management
pub struct MemoryPool {
    pools: Arc<Mutex<HashMap<usize, Vec<NonNull<u8>>>>>,
    config: PerformanceConfig,
    allocator: Arc<dyn GlobalAlloc + Send + Sync>,
}

impl MemoryPool {
    pub fn new(config: PerformanceConfig) -> Self {
        Self {
            #[allow(clippy::arc_with_non_send_sync)]
            pools: Arc::new(Mutex::new(HashMap::new())),
            config,
            allocator: Arc::new(System),
        }
    }

    pub fn allocate(&self, size: usize) -> Result<NonNull<u8>, MemoryPoolError> {
        if size == 0 {
            return Err(MemoryPoolError::InvalidSize);
        }

        // Try to get from pool first
        {
            let mut pools = self
                .pools
                .lock()
                .map_err(|e| MemoryPoolError::LockError(e.to_string()))?;

            if let Some(pool) = pools.get_mut(&size) {
                if let Some(ptr) = pool.pop() {
                    return Ok(ptr);
                }
            }
        }

        // Allocate new memory
        let layout = Layout::from_size_align(size, 8).map_err(|_| MemoryPoolError::InvalidSize)?;

        unsafe {
            let ptr = self.allocator.alloc(layout);
            if ptr.is_null() {
                return Err(MemoryPoolError::AllocationFailed);
            }
            Ok(NonNull::new_unchecked(ptr))
        }
    }

    pub fn deallocate(&self, ptr: NonNull<u8>, size: usize) -> Result<(), MemoryPoolError> {
        if !self.config.memory_pool_enabled {
            let layout =
                Layout::from_size_align(size, 8).map_err(|_| MemoryPoolError::InvalidSize)?;
            unsafe {
                self.allocator.dealloc(ptr.as_ptr(), layout);
            }
            return Ok(());
        }

        let mut pools = self
            .pools
            .lock()
            .map_err(|e| MemoryPoolError::LockError(e.to_string()))?;

        // Add to pool
        pools.entry(size).or_insert_with(Vec::new).push(ptr);

        Ok(())
    }

    pub fn cleanup(&self) -> Result<(), MemoryPoolError> {
        let mut pools = self
            .pools
            .lock()
            .map_err(|e| MemoryPoolError::LockError(e.to_string()))?;

        for (size, pool) in pools.iter_mut() {
            let layout =
                Layout::from_size_align(*size, 8).map_err(|_| MemoryPoolError::InvalidSize)?;

            for ptr in pool.drain(..) {
                unsafe {
                    self.allocator.dealloc(ptr.as_ptr(), layout);
                }
            }
        }

        pools.clear();
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MemoryPoolError {
    #[error("Invalid size")]
    InvalidSize,

    #[error("Allocation failed")]
    AllocationFailed,

    #[error("Lock error: {0}")]
    LockError(String),
}

/// Performance profiler for metrics collection
pub struct PerformanceProfiler {
    metrics: Arc<Mutex<HashMap<String, PerformanceMetric>>>,
    config: PerformanceConfig,
}

#[derive(Debug, Clone)]
pub struct PerformanceMetric {
    pub name: String,
    pub total_time: Duration,
    pub call_count: u64,
    pub min_time: Duration,
    pub max_time: Duration,
    pub avg_time: Duration,
}

impl PerformanceProfiler {
    pub fn new(config: PerformanceConfig) -> Self {
        Self {
            metrics: Arc::new(Mutex::new(HashMap::new())),
            config,
        }
    }

    pub fn start_timer(&self, name: String) -> PerformanceTimer {
        PerformanceTimer {
            name,
            start_time: Instant::now(),
            profiler: self.clone(),
        }
    }

    pub fn record_metric(&self, name: String, duration: Duration) -> Result<(), ProfilerError> {
        if !self.config.profiling_enabled {
            return Ok(());
        }

        let mut metrics = self
            .metrics
            .lock()
            .map_err(|e| ProfilerError::LockError(e.to_string()))?;

        let metric = metrics
            .entry(name.clone())
            .or_insert_with(|| PerformanceMetric {
                name: name.clone(),
                total_time: Duration::ZERO,
                call_count: 0,
                min_time: Duration::MAX,
                max_time: Duration::ZERO,
                avg_time: Duration::ZERO,
            });

        metric.call_count += 1;
        metric.total_time += duration;
        metric.min_time = metric.min_time.min(duration);
        metric.max_time = metric.max_time.max(duration);
        metric.avg_time = metric.total_time / metric.call_count as u32;

        Ok(())
    }

    pub fn get_metrics(&self) -> Result<Vec<PerformanceMetric>, ProfilerError> {
        let metrics = self
            .metrics
            .lock()
            .map_err(|e| ProfilerError::LockError(e.to_string()))?;
        Ok(metrics.values().cloned().collect())
    }

    pub fn clear_metrics(&self) -> Result<(), ProfilerError> {
        let mut metrics = self
            .metrics
            .lock()
            .map_err(|e| ProfilerError::LockError(e.to_string()))?;
        metrics.clear();
        Ok(())
    }
}

impl Clone for PerformanceProfiler {
    fn clone(&self) -> Self {
        Self {
            metrics: Arc::clone(&self.metrics),
            config: self.config.clone(),
        }
    }
}

pub struct PerformanceTimer {
    name: String,
    start_time: Instant,
    profiler: PerformanceProfiler,
}

impl Drop for PerformanceTimer {
    fn drop(&mut self) {
        let duration = self.start_time.elapsed();
        let _ = self.profiler.record_metric(self.name.clone(), duration);
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ProfilerError {
    #[error("Lock error: {0}")]
    LockError(String),
}

/// Work-stealing parallel processor
pub struct ParallelProcessor {
    config: PerformanceConfig,
    thread_pool: Arc<rayon::ThreadPool>,
}

impl ParallelProcessor {
    pub fn new(config: PerformanceConfig) -> Self {
        let thread_pool = Arc::new(
            rayon::ThreadPoolBuilder::new()
                .num_threads(num_cpus::get())
                .build()
                .expect("Failed to create thread pool"),
        );

        Self {
            config,
            thread_pool,
        }
    }

    pub fn parallel_map<T, U, F>(&self, data: &[T], f: F) -> Vec<U>
    where
        T: Send + Sync,
        U: Send,
        F: Fn(&T) -> U + Send + Sync,
    {
        if !self.config.parallel_processing {
            return data.iter().map(f).collect();
        }

        self.thread_pool
            .install(|| data.par_iter().map(f).collect())
    }

    pub fn parallel_reduce<T, F>(&self, data: &[T], identity: T, f: F) -> T
    where
        T: Send + Sync + Clone,
        F: Fn(T, &T) -> T + Send + Sync,
    {
        if !self.config.parallel_processing {
            return data.iter().fold(identity, f);
        }

        self.thread_pool.install(|| {
            data.par_iter()
                .fold(|| identity.clone(), &f)
                .reduce(|| identity.clone(), |acc, x| f(acc, &x))
        })
    }

    pub fn parallel_filter<T, F>(&self, data: &[T], f: F) -> Vec<T>
    where
        T: Send + Sync + Clone,
        F: Fn(&T) -> bool + Send + Sync,
    {
        if !self.config.parallel_processing {
            return data.iter().filter(|x| f(x)).cloned().collect();
        }

        self.thread_pool
            .install(|| data.par_iter().filter(|x| f(x)).cloned().collect())
    }
}

/// Main performance manager that coordinates all optimization systems
pub struct PerformanceManager {
    config: PerformanceConfig,
    simd_processor: SimdProcessor,
    cache_manager: CacheManager,
    memory_pool: MemoryPool,
    profiler: PerformanceProfiler,
    parallel_processor: ParallelProcessor,
}

impl PerformanceManager {
    pub fn new(config: PerformanceConfig) -> Self {
        Self {
            simd_processor: SimdProcessor::new(config.clone()),
            cache_manager: CacheManager::new(config.clone()),
            memory_pool: MemoryPool::new(config.clone()),
            profiler: PerformanceProfiler::new(config.clone()),
            parallel_processor: ParallelProcessor::new(config.clone()),
            config,
        }
    }

    pub fn simd_processor(&self) -> &SimdProcessor {
        &self.simd_processor
    }

    pub fn cache_manager(&self) -> &CacheManager {
        &self.cache_manager
    }

    pub fn memory_pool(&self) -> &MemoryPool {
        &self.memory_pool
    }

    pub fn profiler(&self) -> &PerformanceProfiler {
        &self.profiler
    }

    pub fn parallel_processor(&self) -> &ParallelProcessor {
        &self.parallel_processor
    }

    pub fn config(&self) -> &PerformanceConfig {
        &self.config
    }

    pub fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.memory_pool.cleanup()?;
        self.cache_manager.clear()?;
        self.profiler.clear_metrics()?;
        Ok(())
    }
}
