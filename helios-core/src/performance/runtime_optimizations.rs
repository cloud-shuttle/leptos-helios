//! Runtime Performance Optimizations
//!
//! This module provides runtime performance optimizations for leptos-helios,
//! including memory pooling, caching, and algorithmic improvements.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// High-performance memory pool for frequently allocated objects
pub struct MemoryPool<T> {
    pool: Arc<Mutex<Vec<T>>>,
    factory: Box<dyn Fn() -> T + Send + Sync>,
    max_size: usize,
    stats: Arc<Mutex<PoolStats>>,
}

#[derive(Debug, Clone)]
pub struct PoolStats {
    pub allocations: u64,
    pub deallocations: u64,
    pub pool_hits: u64,
    pub pool_misses: u64,
}

impl<T> MemoryPool<T> {
    pub fn new<F>(factory: F, max_size: usize) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            pool: Arc::new(Mutex::new(Vec::new())),
            factory: Box::new(factory),
            max_size,
            stats: Arc::new(Mutex::new(PoolStats {
                allocations: 0,
                deallocations: 0,
                pool_hits: 0,
                pool_misses: 0,
            })),
        }
    }

    pub fn acquire(&self) -> T {
        let mut pool = self.pool.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        if let Some(item) = pool.pop() {
            stats.pool_hits += 1;
            stats.allocations += 1;
            item
        } else {
            stats.pool_misses += 1;
            stats.allocations += 1;
            (self.factory)()
        }
    }

    pub fn release(&self, item: T) {
        let mut pool = self.pool.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        if pool.len() < self.max_size {
            pool.push(item);
        }
        stats.deallocations += 1;
    }

    pub fn get_stats(&self) -> PoolStats {
        self.stats.lock().unwrap().clone()
    }

    pub fn clear(&self) {
        let mut pool = self.pool.lock().unwrap();
        pool.clear();
    }
}

/// High-performance cache with LRU eviction
pub struct LruCache<K, V> {
    cache: Arc<Mutex<HashMap<K, (V, Instant)>>>,
    max_size: usize,
    ttl: Duration,
    stats: Arc<Mutex<CacheStats>>,
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub size: usize,
}

impl<K, V> LruCache<K, V>
where
    K: Clone + std::hash::Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
{
    pub fn new(max_size: usize, ttl: Duration) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            max_size,
            ttl,
            stats: Arc::new(Mutex::new(CacheStats {
                hits: 0,
                misses: 0,
                evictions: 0,
                size: 0,
            })),
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let mut cache = self.cache.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        if let Some((value, timestamp)) = cache.get(key) {
            if timestamp.elapsed() < self.ttl {
                stats.hits += 1;
                Some(value.clone())
            } else {
                cache.remove(key);
                stats.misses += 1;
                stats.evictions += 1;
                stats.size = cache.len();
                None
            }
        } else {
            stats.misses += 1;
            None
        }
    }

    pub fn insert(&self, key: K, value: V) {
        let mut cache = self.cache.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        // Evict oldest entries if cache is full
        while cache.len() >= self.max_size {
            if let Some(oldest_key) = cache.keys().next().cloned() {
                cache.remove(&oldest_key);
                stats.evictions += 1;
            }
        }
        
        cache.insert(key, (value, Instant::now()));
        stats.size = cache.len();
    }

    pub fn get_stats(&self) -> CacheStats {
        let stats = self.stats.lock().unwrap();
        let cache = self.cache.lock().unwrap();
        CacheStats {
            hits: stats.hits,
            misses: stats.misses,
            evictions: stats.evictions,
            size: cache.len(),
        }
    }

    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        cache.clear();
        stats.size = 0;
    }
}

/// Optimized data structure for fast lookups and insertions
pub struct FastHashMap<K, V> {
    data: Arc<Mutex<HashMap<K, V>>>,
    stats: Arc<Mutex<HashMapStats>>,
}

#[derive(Debug, Clone)]
pub struct HashMapStats {
    pub insertions: u64,
    pub lookups: u64,
    pub removals: u64,
    pub size: usize,
}

impl<K, V> FastHashMap<K, V>
where
    K: Clone + std::hash::Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
{
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(Mutex::new(HashMapStats {
                insertions: 0,
                lookups: 0,
                removals: 0,
                size: 0,
            })),
        }
    }

    pub fn insert(&self, key: K, value: V) -> Option<V> {
        let mut data = self.data.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        let result = data.insert(key, value);
        stats.insertions += 1;
        stats.size = data.len();
        result
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let data = self.data.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        let result = data.get(key).cloned();
        stats.lookups += 1;
        result
    }

    pub fn remove(&self, key: &K) -> Option<V> {
        let mut data = self.data.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        let result = data.remove(key);
        stats.removals += 1;
        stats.size = data.len();
        result
    }

    pub fn get_stats(&self) -> HashMapStats {
        let stats = self.stats.lock().unwrap();
        let data = self.data.lock().unwrap();
        HashMapStats {
            insertions: stats.insertions,
            lookups: stats.lookups,
            removals: stats.removals,
            size: data.len(),
        }
    }

    pub fn clear(&self) {
        let mut data = self.data.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        data.clear();
        stats.size = 0;
    }
}

/// Performance monitoring and profiling utilities
pub struct PerformanceProfiler {
    measurements: Arc<Mutex<HashMap<String, Vec<Duration>>>>,
    active_timers: Arc<Mutex<HashMap<String, Instant>>>,
}

impl PerformanceProfiler {
    pub fn new() -> Self {
        Self {
            measurements: Arc::new(Mutex::new(HashMap::new())),
            active_timers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn start_timer(&self, name: &str) {
        let mut timers = self.active_timers.lock().unwrap();
        timers.insert(name.to_string(), Instant::now());
    }

    pub fn end_timer(&self, name: &str) -> Option<Duration> {
        let mut timers = self.active_timers.lock().unwrap();
        let mut measurements = self.measurements.lock().unwrap();
        
        if let Some(start_time) = timers.remove(name) {
            let duration = start_time.elapsed();
            measurements.entry(name.to_string()).or_insert_with(Vec::new).push(duration);
            Some(duration)
        } else {
            None
        }
    }

    pub fn get_average_time(&self, name: &str) -> Option<Duration> {
        let measurements = self.measurements.lock().unwrap();
        if let Some(times) = measurements.get(name) {
            if !times.is_empty() {
                let total: Duration = times.iter().sum();
                Some(total / times.len() as u32)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_stats(&self) -> HashMap<String, PerformanceStats> {
        let measurements = self.measurements.lock().unwrap();
        let mut stats = HashMap::new();
        
        for (name, times) in measurements.iter() {
            if !times.is_empty() {
                let total: Duration = times.iter().sum();
                let average = total / times.len() as u32;
                let min = *times.iter().min().unwrap();
                let max = *times.iter().max().unwrap();
                
                stats.insert(name.clone(), PerformanceStats {
                    count: times.len(),
                    total_time: total,
                    average_time: average,
                    min_time: min,
                    max_time: max,
                });
            }
        }
        
        stats
    }
}

#[derive(Debug, Clone)]
pub struct PerformanceStats {
    pub count: usize,
    pub total_time: Duration,
    pub average_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
}

impl Default for PerformanceProfiler {
    fn default() -> Self {
        Self::new()
    }
}
