//! Data caching for streaming data

use super::types::*;
use crate::chart_config::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Data cache configuration
#[derive(Debug, Clone)]
pub struct DataCacheConfig {
    pub cache_size: usize,
    pub ttl: Duration,
    pub eviction_policy: CacheEvictionPolicy,
    pub compression: bool,
}

/// Cache eviction policy
#[derive(Debug, Clone)]
pub enum CacheEvictionPolicy {
    LRU,
    LFU,
    FIFO,
    TTL,
}

/// Data cache
pub struct DataCache {
    config: DataCacheConfig,
    cache: HashMap<String, (Vec<DataPoint>, Instant)>,
    access_times: HashMap<String, Instant>,
    hit_count: usize,
    miss_count: usize,
}

impl DataCache {
    pub fn new(config: DataCacheConfig) -> Result<Self, ChartRenderError> {
        Ok(Self {
            config,
            cache: HashMap::new(),
            access_times: HashMap::new(),
            hit_count: 0,
            miss_count: 0,
        })
    }

    pub fn put(&mut self, key: &str, data: Vec<DataPoint>) -> Result<(), ChartRenderError> {
        let now = Instant::now();
        self.cache.insert(key.to_string(), (data, now));
        self.access_times.insert(key.to_string(), now);

        // Simple eviction if cache is full
        if self.cache.len() > self.config.cache_size {
            match self.config.eviction_policy {
                CacheEvictionPolicy::LRU => {
                    let oldest_key = self
                        .access_times
                        .iter()
                        .min_by_key(|(_, time)| *time)
                        .map(|(key, _)| key.clone());
                    if let Some(key) = oldest_key {
                        self.cache.remove(&key);
                        self.access_times.remove(&key);
                    }
                }
                CacheEvictionPolicy::FIFO => {
                    // Simple FIFO - remove first entry
                    if let Some(key) = self.cache.keys().next().cloned() {
                        self.cache.remove(&key);
                        self.access_times.remove(&key);
                    }
                }
                _ => {
                    // Default to LRU for other policies
                    let oldest_key = self
                        .access_times
                        .iter()
                        .min_by_key(|(_, time)| *time)
                        .map(|(key, _)| key.clone());
                    if let Some(key) = oldest_key {
                        self.cache.remove(&key);
                        self.access_times.remove(&key);
                    }
                }
            }
        }

        Ok(())
    }

    pub fn get(&mut self, key: &str) -> Option<Vec<DataPoint>> {
        let now = Instant::now();
        
        if let Some((data, insert_time)) = self.cache.get(key) {
            // Check TTL
            if now.duration_since(*insert_time) < self.config.ttl {
                self.access_times.insert(key.to_string(), now);
                self.hit_count += 1;
                Some(data.clone())
            } else {
                // Expired
                self.cache.remove(key);
                self.access_times.remove(key);
                self.miss_count += 1;
                None
            }
        } else {
            self.miss_count += 1;
            None
        }
    }

    pub fn hit_rate(&self) -> f64 {
        let total = self.hit_count + self.miss_count;
        if total == 0 {
            0.0
        } else {
            self.hit_count as f64 / total as f64
        }
    }

    pub fn size(&self) -> usize {
        self.cache.len()
    }
}
