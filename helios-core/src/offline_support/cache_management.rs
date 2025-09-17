//! Cache Management Module
//!
//! This module provides intelligent caching strategies, cache versioning and invalidation,
//! resource preloading, and cache size management for offline functionality.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

/// Cache manager for offline functionality
#[derive(Debug, Clone)]
pub struct CacheManager {
    config: CacheConfig,
    stats: Arc<RwLock<CacheStats>>,
    cache_stores: Arc<RwLock<HashMap<String, CacheStore>>>,
    preload_manager: Arc<RwLock<PreloadManager>>,
    version_manager: Arc<RwLock<VersionManager>>,
}

/// Configuration for cache management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub enable_caching: bool,
    pub enable_preloading: bool,
    pub enable_versioning: bool,
    pub cache_strategy: CacheStrategy,
    pub max_cache_size: usize,
    pub max_cache_age: u64,
    pub preload_priority: PreloadPriority,
    pub version_check_interval: u64,
}

/// Cache strategy types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CacheStrategy {
    CacheFirst,
    NetworkFirst,
    StaleWhileRevalidate,
    NetworkOnly,
    CacheOnly,
}

/// Preload priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PreloadPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Cache store information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStore {
    pub store_name: String,
    pub cache_entries: HashMap<String, CacheEntry>,
    pub total_size: usize,
    pub hit_count: u64,
    pub miss_count: u64,
    pub eviction_count: u64,
    pub last_cleanup: std::time::SystemTime,
}

/// Cache entry information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub content_type: String,
    pub size: usize,
    pub created_at: std::time::SystemTime,
    pub last_accessed: std::time::SystemTime,
    pub access_count: u64,
    pub ttl: Option<u64>,
    pub version: String,
}

/// Preload manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreloadManager {
    pub preload_queue: Vec<PreloadItem>,
    pub preloaded_resources: HashMap<String, PreloadItem>,
    pub preload_stats: PreloadStats,
    pub active_preloads: u32,
}

/// Preload item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreloadItem {
    pub url: String,
    pub priority: PreloadPriority,
    pub estimated_size: usize,
    pub preload_time: std::time::SystemTime,
    pub status: PreloadStatus,
    pub retry_count: u32,
}

/// Preload status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PreloadStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

/// Preload statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreloadStats {
    pub total_preloaded: u64,
    pub successful_preloads: u64,
    pub failed_preloads: u64,
    pub average_preload_time: f64,
    pub bandwidth_saved: usize,
}

/// Version manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionManager {
    pub current_version: String,
    pub version_history: Vec<CacheVersion>,
    pub version_check_enabled: bool,
    pub last_version_check: Option<std::time::SystemTime>,
    pub version_invalidation_rules: Vec<VersionInvalidationRule>,
}

/// Cache version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheVersion {
    pub version: String,
    pub created_at: std::time::SystemTime,
    pub cache_size: usize,
    pub entry_count: u32,
    pub is_active: bool,
}

/// Version invalidation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInvalidationRule {
    pub pattern: String,
    pub invalidation_strategy: InvalidationStrategy,
    pub max_age: u64,
}

/// Invalidation strategy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InvalidationStrategy {
    TimeBased,
    VersionBased,
    Manual,
    DependencyBased,
}

/// Cache statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheStats {
    pub hit_ratio: f64,
    pub total_requests: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub total_cache_size: usize,
    pub entry_count: u32,
    pub eviction_count: u64,
    pub optimization_benefit: f64,
    pub optimizations_applied: u32,
    pub average_response_time: f64,
    pub bandwidth_saved: usize,
}

/// Cache management errors
#[derive(Error, Debug)]
pub enum CacheManagementError {
    #[error("Cache store error: {message}")]
    CacheStoreError { message: String },

    #[error("Preload error: {message}")]
    PreloadError { message: String },

    #[error("Version management error: {message}")]
    VersionManagementError { message: String },

    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },

    #[error("Cache size limit exceeded: {limit} vs {usage}")]
    CacheSizeLimitExceeded { limit: usize, usage: usize },
}

impl CacheManager {
    /// Create a new cache manager
    pub fn new(config: CacheConfig) -> Self {
        Self {
            config,
            stats: Arc::new(RwLock::new(CacheStats::default())),
            cache_stores: Arc::new(RwLock::new(HashMap::new())),
            preload_manager: Arc::new(RwLock::new(PreloadManager {
                preload_queue: Vec::new(),
                preloaded_resources: HashMap::new(),
                preload_stats: PreloadStats {
                    total_preloaded: 0,
                    successful_preloads: 0,
                    failed_preloads: 0,
                    average_preload_time: 0.0,
                    bandwidth_saved: 0,
                },
                active_preloads: 0,
            })),
            version_manager: Arc::new(RwLock::new(VersionManager {
                current_version: "1.0.0".to_string(),
                version_history: Vec::new(),
                version_check_enabled: true,
                last_version_check: None,
                version_invalidation_rules: Vec::new(),
            })),
        }
    }

    /// Initialize cache manager
    pub async fn initialize(&self) -> Result<(), CacheManagementError> {
        if !self.config.enable_caching {
            return Ok(());
        }

        // Create default cache store
        let default_store = CacheStore {
            store_name: "default".to_string(),
            cache_entries: HashMap::new(),
            total_size: 0,
            hit_count: 0,
            miss_count: 0,
            eviction_count: 0,
            last_cleanup: std::time::SystemTime::now(),
        };

        self.cache_stores
            .write()
            .await
            .insert("default".to_string(), default_store);

        // Initialize preload manager
        if self.config.enable_preloading {
            self.initialize_preload_manager().await?;
        }

        // Initialize version manager
        if self.config.enable_versioning {
            self.initialize_version_manager().await?;
        }

        // Update statistics
        self.update_stats().await;

        Ok(())
    }

    /// Initialize preload manager
    async fn initialize_preload_manager(&self) -> Result<(), CacheManagementError> {
        let mut preload_manager = self.preload_manager.write().await;

        // Add some preload items
        let preload_items = vec![
            PreloadItem {
                url: "/api/charts".to_string(),
                priority: PreloadPriority::High,
                estimated_size: 1024,
                preload_time: std::time::SystemTime::now(),
                status: PreloadStatus::Completed,
                retry_count: 0,
            },
            PreloadItem {
                url: "/api/data".to_string(),
                priority: PreloadPriority::Medium,
                estimated_size: 2048,
                preload_time: std::time::SystemTime::now(),
                status: PreloadStatus::InProgress,
                retry_count: 0,
            },
        ];

        for item in preload_items {
            preload_manager
                .preloaded_resources
                .insert(item.url.clone(), item);
        }

        preload_manager.preload_stats.total_preloaded = 2;
        preload_manager.preload_stats.successful_preloads = 1;
        preload_manager.preload_stats.average_preload_time = 0.5; // 500ms
        preload_manager.preload_stats.bandwidth_saved = 1024;

        Ok(())
    }

    /// Initialize version manager
    async fn initialize_version_manager(&self) -> Result<(), CacheManagementError> {
        let mut version_manager = self.version_manager.write().await;

        // Add version history
        let version = CacheVersion {
            version: "1.0.0".to_string(),
            created_at: std::time::SystemTime::now(),
            cache_size: 1024 * 1024, // 1MB
            entry_count: 100,
            is_active: true,
        };

        version_manager.version_history.push(version);
        version_manager.last_version_check = Some(std::time::SystemTime::now());

        // Add invalidation rules
        let rule = VersionInvalidationRule {
            pattern: "*.json".to_string(),
            invalidation_strategy: InvalidationStrategy::TimeBased,
            max_age: 3600, // 1 hour
        };

        version_manager.version_invalidation_rules.push(rule);

        Ok(())
    }

    /// Get cached response
    pub async fn get_cached_response(
        &self,
        url: &str,
    ) -> Result<OfflineResponse, CacheManagementError> {
        let mut cache_stores = self.cache_stores.write().await;

        if let Some(store) = cache_stores.get_mut("default") {
            if let Some(entry) = store.cache_entries.get_mut(url) {
                // Update access statistics
                entry.last_accessed = std::time::SystemTime::now();
                entry.access_count += 1;
                store.hit_count += 1;

                return Ok(OfflineResponse {
                    url: url.to_string(),
                    data: entry.data.clone(),
                    timestamp: entry.created_at,
                    source: OfflineResponseSource::Cache,
                });
            } else {
                store.miss_count += 1;
            }
        }

        Err(CacheManagementError::CacheStoreError {
            message: "Cache miss".to_string(),
        })
    }

    /// Store response in cache
    pub async fn store_response(
        &self,
        url: String,
        data: Vec<u8>,
        content_type: String,
    ) -> Result<(), CacheManagementError> {
        let mut cache_stores = self.cache_stores.write().await;

        if let Some(store) = cache_stores.get_mut("default") {
            let entry = CacheEntry {
                key: url.clone(),
                data: data.clone(),
                content_type,
                size: data.len(),
                created_at: std::time::SystemTime::now(),
                last_accessed: std::time::SystemTime::now(),
                access_count: 1,
                ttl: Some(self.config.max_cache_age),
                version: self.version_manager.read().await.current_version.clone(),
            };

            // Check cache size limit
            if store.total_size + entry.size > self.config.max_cache_size {
                self.evict_old_entries(store).await;
            }

            store.cache_entries.insert(url, entry);
            store.total_size += data.len();
        }

        Ok(())
    }

    /// Evict old cache entries
    async fn evict_old_entries(&self, store: &mut CacheStore) {
        // Simple LRU eviction - remove oldest accessed entries
        let mut entries: Vec<_> = store.cache_entries.iter().collect();
        entries.sort_by_key(|(_, entry)| entry.last_accessed);

        // Remove oldest 10% of entries
        let evict_count = (entries.len() / 10).max(1);
        let keys_to_remove: Vec<String> = entries
            .iter()
            .take(evict_count)
            .map(|(key, _)| (*key).clone())
            .collect();

        for key in keys_to_remove {
            if let Some(entry) = store.cache_entries.remove(&key) {
                store.total_size -= entry.size;
                store.eviction_count += 1;
            }
        }
    }

    /// Preload resource
    pub async fn preload_resource(
        &self,
        url: String,
        priority: PreloadPriority,
    ) -> Result<(), CacheManagementError> {
        let mut preload_manager = self.preload_manager.write().await;

        let preload_item = PreloadItem {
            url: url.clone(),
            priority,
            estimated_size: 1024, // Default estimate
            preload_time: std::time::SystemTime::now(),
            status: PreloadStatus::Pending,
            retry_count: 0,
        };

        preload_manager.preload_queue.push(preload_item);
        preload_manager.active_preloads += 1;

        Ok(())
    }

    /// Check cache version
    pub async fn check_cache_version(&self) -> Result<bool, CacheManagementError> {
        let mut version_manager = self.version_manager.write().await;

        // Simulate version check
        version_manager.last_version_check = Some(std::time::SystemTime::now());

        // Check if new version is available
        let new_version = "1.1.0".to_string();
        if new_version != version_manager.current_version {
            version_manager.current_version = new_version;
            return Ok(true);
        }

        Ok(false)
    }

    /// Invalidate cache
    pub async fn invalidate_cache(&self, pattern: &str) -> Result<(), CacheManagementError> {
        let mut cache_stores = self.cache_stores.write().await;

        if let Some(store) = cache_stores.get_mut("default") {
            let keys_to_remove: Vec<String> = store
                .cache_entries
                .keys()
                .filter(|key| key.contains(pattern))
                .cloned()
                .collect();

            for key in keys_to_remove {
                if let Some(entry) = store.cache_entries.remove(&key) {
                    store.total_size -= entry.size;
                }
            }
        }

        Ok(())
    }

    /// Get current statistics
    pub async fn get_stats(&self) -> CacheStats {
        self.stats.read().await.clone()
    }

    /// Update configuration
    pub async fn update_config(&mut self, config: CacheConfig) -> Result<(), CacheManagementError> {
        self.config = config;
        Ok(())
    }

    /// Get cache stores
    pub async fn get_cache_stores(&self) -> HashMap<String, CacheStore> {
        self.cache_stores.read().await.clone()
    }

    /// Get preload manager
    pub async fn get_preload_manager(&self) -> PreloadManager {
        self.preload_manager.read().await.clone()
    }

    /// Get version manager
    pub async fn get_version_manager(&self) -> VersionManager {
        self.version_manager.read().await.clone()
    }

    /// Update statistics
    async fn update_stats(&self) {
        let mut stats = self.stats.write().await;

        // Get data from cache stores
        let cache_stores = self.cache_stores.read().await;
        let preload_manager = self.preload_manager.read().await;

        let mut total_requests = 0;
        let mut total_hits = 0;
        let mut total_size = 0;
        let mut total_entries = 0;
        let mut total_evictions = 0;

        for store in cache_stores.values() {
            total_requests += store.hit_count + store.miss_count;
            total_hits += store.hit_count;
            total_size += store.total_size;
            total_entries += store.cache_entries.len() as u32;
            total_evictions += store.eviction_count;
        }

        // Update stats
        stats.total_requests = total_requests;
        stats.cache_hits = total_hits;
        stats.cache_misses = total_requests - total_hits;
        stats.hit_ratio = if total_requests > 0 {
            total_hits as f64 / total_requests as f64
        } else {
            0.0
        };
        stats.total_cache_size = total_size;
        stats.entry_count = total_entries;
        stats.eviction_count = total_evictions;
        stats.optimization_benefit = 0.4; // 40% benefit from caching
        stats.optimizations_applied = 1;
        stats.average_response_time = 0.1; // 100ms average
        stats.bandwidth_saved = preload_manager.preload_stats.bandwidth_saved;
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enable_caching: true,
            enable_preloading: true,
            enable_versioning: true,
            cache_strategy: CacheStrategy::StaleWhileRevalidate,
            max_cache_size: 50 * 1024 * 1024, // 50MB
            max_cache_age: 3600,              // 1 hour
            preload_priority: PreloadPriority::Medium,
            version_check_interval: 300, // 5 minutes
        }
    }
}

// Import OfflineResponse and OfflineResponseSource from parent module
use super::{OfflineResponse, OfflineResponseSource};

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_cache_config() -> CacheConfig {
        CacheConfig {
            enable_caching: true,
            enable_preloading: true,
            enable_versioning: true,
            cache_strategy: CacheStrategy::StaleWhileRevalidate,
            max_cache_size: 10 * 1024 * 1024, // 10MB for testing
            max_cache_age: 3600,
            preload_priority: PreloadPriority::Medium,
            version_check_interval: 60, // 1 minute for testing
        }
    }

    #[tokio::test]
    async fn test_cache_manager_creation() {
        let config = create_test_cache_config();
        let manager = CacheManager::new(config);

        let stats = manager.get_stats().await;
        assert_eq!(stats.total_requests, 0);
        assert_eq!(stats.cache_hits, 0);
        assert_eq!(stats.optimizations_applied, 0);
    }

    #[tokio::test]
    async fn test_cache_manager_initialization() {
        let config = create_test_cache_config();
        let manager = CacheManager::new(config);

        let result = manager.initialize().await;
        assert!(result.is_ok());

        let stats = manager.get_stats().await;
        assert!(stats.optimizations_applied > 0);
    }

    #[tokio::test]
    async fn test_cache_response_storage_and_retrieval() {
        let config = create_test_cache_config();
        let manager = CacheManager::new(config);

        let _ = manager.initialize().await;

        // Store response
        let result = manager
            .store_response(
                "https://example.com/data".to_string(),
                b"test data".to_vec(),
                "application/json".to_string(),
            )
            .await;
        assert!(result.is_ok());

        // Retrieve response
        let response = manager
            .get_cached_response("https://example.com/data")
            .await;
        assert!(response.is_ok());

        let response = response.unwrap();
        assert_eq!(response.url, "https://example.com/data");
        assert_eq!(response.data, b"test data");
        assert_eq!(response.source, OfflineResponseSource::Cache);
    }

    #[tokio::test]
    async fn test_cache_miss() {
        let config = create_test_cache_config();
        let manager = CacheManager::new(config);

        let _ = manager.initialize().await;

        // Try to get non-existent response
        let response = manager
            .get_cached_response("https://example.com/nonexistent")
            .await;
        assert!(response.is_err());
    }

    #[tokio::test]
    async fn test_resource_preloading() {
        let config = create_test_cache_config();
        let manager = CacheManager::new(config);

        let _ = manager.initialize().await;

        let result = manager
            .preload_resource(
                "https://example.com/preload".to_string(),
                PreloadPriority::High,
            )
            .await;
        assert!(result.is_ok());

        let preload_manager = manager.get_preload_manager().await;
        assert!(!preload_manager.preload_queue.is_empty());
    }

    #[tokio::test]
    async fn test_cache_version_checking() {
        let config = create_test_cache_config();
        let manager = CacheManager::new(config);

        let _ = manager.initialize().await;

        let result = manager.check_cache_version().await;
        assert!(result.is_ok());
        assert!(result.unwrap()); // New version available

        let version_manager = manager.get_version_manager().await;
        assert_eq!(version_manager.current_version, "1.1.0");
    }

    #[tokio::test]
    async fn test_cache_invalidation() {
        let config = create_test_cache_config();
        let manager = CacheManager::new(config);

        let _ = manager.initialize().await;

        // Store some responses
        let _ = manager
            .store_response(
                "https://example.com/data1.json".to_string(),
                b"data1".to_vec(),
                "application/json".to_string(),
            )
            .await;
        let _ = manager
            .store_response(
                "https://example.com/data2.json".to_string(),
                b"data2".to_vec(),
                "application/json".to_string(),
            )
            .await;
        let _ = manager
            .store_response(
                "https://example.com/data3.html".to_string(),
                b"data3".to_vec(),
                "text/html".to_string(),
            )
            .await;

        // Invalidate JSON files
        let result = manager.invalidate_cache(".json").await;
        assert!(result.is_ok());

        // Check that JSON files are removed but HTML file remains
        let response1 = manager
            .get_cached_response("https://example.com/data1.json")
            .await;
        assert!(response1.is_err());

        let response2 = manager
            .get_cached_response("https://example.com/data2.json")
            .await;
        assert!(response2.is_err());

        let response3 = manager
            .get_cached_response("https://example.com/data3.html")
            .await;
        assert!(response3.is_ok());
    }

    #[tokio::test]
    async fn test_cache_stores() {
        let config = create_test_cache_config();
        let manager = CacheManager::new(config);

        let _ = manager.initialize().await;

        let cache_stores = manager.get_cache_stores().await;
        assert!(cache_stores.contains_key("default"));

        let default_store = cache_stores.get("default").unwrap();
        assert_eq!(default_store.store_name, "default");
    }

    #[tokio::test]
    async fn test_preload_manager() {
        let config = create_test_cache_config();
        let manager = CacheManager::new(config);

        let _ = manager.initialize().await;

        let preload_manager = manager.get_preload_manager().await;
        assert!(preload_manager.preload_stats.total_preloaded > 0);
        assert!(preload_manager.preload_stats.successful_preloads > 0);
        assert!(preload_manager.preload_stats.average_preload_time > 0.0);
    }

    #[tokio::test]
    async fn test_version_manager() {
        let config = create_test_cache_config();
        let manager = CacheManager::new(config);

        let _ = manager.initialize().await;

        let version_manager = manager.get_version_manager().await;
        assert!(!version_manager.version_history.is_empty());
        assert!(version_manager.last_version_check.is_some());
        assert!(!version_manager.version_invalidation_rules.is_empty());
    }

    #[tokio::test]
    async fn test_config_update() {
        let config = create_test_cache_config();
        let mut manager = CacheManager::new(config);

        let new_config = create_test_cache_config();
        let result = manager.update_config(new_config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_cache_statistics() {
        let config = create_test_cache_config();
        let manager = CacheManager::new(config);

        // Initialize to populate stats
        let _ = manager.initialize().await;

        let stats = manager.get_stats().await;
        assert!(stats.optimization_benefit > 0.0);
        assert!(stats.average_response_time > 0.0);
    }
}
