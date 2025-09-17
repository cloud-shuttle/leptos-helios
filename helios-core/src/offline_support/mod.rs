//! Offline Support Module
//!
//! This module provides comprehensive offline support features for the Helios charting library,
//! including Progressive Web App capabilities, service worker management, and offline data storage.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

pub mod cache_management;
pub mod offline_storage;
pub mod pwa_features;
pub mod service_worker;
pub mod sync_management;

pub use cache_management::*;
pub use offline_storage::*;
pub use pwa_features::*;
pub use service_worker::*;
pub use sync_management::*;

/// Offline support manager that coordinates all offline functionality
#[derive(Debug, Clone)]
pub struct OfflineSupportManager {
    service_worker: Arc<RwLock<ServiceWorkerManager>>,
    cache_manager: Arc<RwLock<CacheManager>>,
    offline_storage: Arc<RwLock<OfflineStorageManager>>,
    sync_manager: Arc<RwLock<SyncManager>>,
    pwa_features: Arc<RwLock<PwaFeaturesManager>>,
    config: OfflineSupportConfig,
    stats: Arc<RwLock<OfflineSupportStats>>,
}

/// Configuration for offline support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineSupportConfig {
    pub service_worker_config: ServiceWorkerConfig,
    pub cache_config: CacheConfig,
    pub storage_config: OfflineStorageConfig,
    pub sync_config: SyncConfig,
    pub pwa_config: PwaConfig,
    pub enable_offline_mode: bool,
    pub auto_sync_enabled: bool,
    pub cache_size_limit: usize,
    pub sync_interval: u64,
}

/// Offline support statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OfflineSupportStats {
    pub is_online: bool,
    pub cache_hit_ratio: f64,
    pub offline_data_size: usize,
    pub sync_operations: u64,
    pub failed_sync_operations: u64,
    pub service_worker_status: ServiceWorkerStatus,
    pub pwa_install_status: PwaInstallStatus,
    pub total_offline_requests: u64,
    pub successful_offline_requests: u64,
    pub optimization_benefits: HashMap<String, f64>,
    pub total_optimizations_applied: u32,
}

/// Offline support errors
#[derive(Error, Debug)]
pub enum OfflineSupportError {
    #[error("Service worker error: {message}")]
    ServiceWorkerError { message: String },

    #[error("Cache management error: {message}")]
    CacheManagementError { message: String },

    #[error("Offline storage error: {message}")]
    OfflineStorageError { message: String },

    #[error("Sync management error: {message}")]
    SyncManagementError { message: String },

    #[error("PWA features error: {message}")]
    PwaFeaturesError { message: String },

    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },

    #[error("Network error: {message}")]
    NetworkError { message: String },
}

impl From<ServiceWorkerError> for OfflineSupportError {
    fn from(err: ServiceWorkerError) -> Self {
        OfflineSupportError::ServiceWorkerError {
            message: err.to_string(),
        }
    }
}

impl From<CacheManagementError> for OfflineSupportError {
    fn from(err: CacheManagementError) -> Self {
        OfflineSupportError::CacheManagementError {
            message: err.to_string(),
        }
    }
}

impl From<OfflineStorageError> for OfflineSupportError {
    fn from(err: OfflineStorageError) -> Self {
        OfflineSupportError::OfflineStorageError {
            message: err.to_string(),
        }
    }
}

impl From<SyncManagementError> for OfflineSupportError {
    fn from(err: SyncManagementError) -> Self {
        OfflineSupportError::SyncManagementError {
            message: err.to_string(),
        }
    }
}

impl From<PwaFeaturesError> for OfflineSupportError {
    fn from(err: PwaFeaturesError) -> Self {
        OfflineSupportError::PwaFeaturesError {
            message: err.to_string(),
        }
    }
}

impl OfflineSupportManager {
    /// Create a new offline support manager
    pub fn new(config: OfflineSupportConfig) -> Self {
        Self {
            service_worker: Arc::new(RwLock::new(ServiceWorkerManager::new(
                config.service_worker_config.clone(),
            ))),
            cache_manager: Arc::new(RwLock::new(CacheManager::new(config.cache_config.clone()))),
            offline_storage: Arc::new(RwLock::new(OfflineStorageManager::new(
                config.storage_config.clone(),
            ))),
            sync_manager: Arc::new(RwLock::new(SyncManager::new(config.sync_config.clone()))),
            pwa_features: Arc::new(RwLock::new(PwaFeaturesManager::new(
                config.pwa_config.clone(),
            ))),
            config,
            stats: Arc::new(RwLock::new(OfflineSupportStats::default())),
        }
    }

    /// Initialize offline support
    pub async fn initialize(&self) -> Result<(), OfflineSupportError> {
        if !self.config.enable_offline_mode {
            return Ok(());
        }

        // Initialize all subsystems
        self.service_worker.read().await.initialize().await?;
        self.cache_manager.read().await.initialize().await?;
        self.offline_storage.read().await.initialize().await?;
        self.sync_manager.read().await.initialize().await?;
        self.pwa_features.read().await.initialize().await?;

        // Update statistics
        self.update_stats().await;

        Ok(())
    }

    /// Handle offline request
    pub async fn handle_offline_request(
        &self,
        request: OfflineRequest,
    ) -> Result<OfflineResponse, OfflineSupportError> {
        let mut stats = self.stats.write().await;
        stats.total_offline_requests += 1;

        // Try cache first
        if let Ok(cached_response) = self
            .cache_manager
            .read()
            .await
            .get_cached_response(&request.url)
            .await
        {
            stats.successful_offline_requests += 1;
            return Ok(cached_response);
        }

        // Try offline storage (only for specific keys, not URLs)
        if request.url.starts_with("data://") {
            let key = request.url.strip_prefix("data://").unwrap_or(&request.url);
            if let Ok(stored_data) = self.offline_storage.read().await.get_data(key).await {
                stats.successful_offline_requests += 1;
                return Ok(OfflineResponse {
                    url: request.url,
                    data: stored_data,
                    timestamp: std::time::SystemTime::now(),
                    source: OfflineResponseSource::OfflineStorage,
                });
            }
        }

        // Return offline fallback
        Ok(OfflineResponse {
            url: request.url,
            data: b"Offline - Data not available".to_vec(),
            timestamp: std::time::SystemTime::now(),
            source: OfflineResponseSource::OfflineFallback,
        })
    }

    /// Sync data when online
    pub async fn sync_data(&self) -> Result<(), OfflineSupportError> {
        if !self.config.auto_sync_enabled {
            return Ok(());
        }

        let result = self.sync_manager.read().await.sync_pending_data().await;

        let mut stats = self.stats.write().await;
        stats.sync_operations += 1;
        if result.is_err() {
            stats.failed_sync_operations += 1;
        }

        result.map_err(Into::into)
    }

    /// Get current statistics
    pub async fn get_stats(&self) -> OfflineSupportStats {
        self.stats.read().await.clone()
    }

    /// Update configuration
    pub async fn update_config(
        &mut self,
        config: OfflineSupportConfig,
    ) -> Result<(), OfflineSupportError> {
        self.config = config.clone();

        // Update subsystem configurations
        {
            let mut service_worker = self.service_worker.write().await;
            service_worker
                .update_config(config.service_worker_config)
                .await?;
        }

        {
            let mut cache_manager = self.cache_manager.write().await;
            cache_manager.update_config(config.cache_config).await?;
        }

        {
            let mut offline_storage = self.offline_storage.write().await;
            offline_storage.update_config(config.storage_config).await?;
        }

        {
            let mut sync_manager = self.sync_manager.write().await;
            sync_manager.update_config(config.sync_config).await?;
        }

        {
            let mut pwa_features = self.pwa_features.write().await;
            pwa_features.update_config(config.pwa_config).await?;
        }

        Ok(())
    }

    /// Check if device is online
    pub async fn is_online(&self) -> bool {
        self.stats.read().await.is_online
    }

    /// Set online status
    pub async fn set_online_status(&self, is_online: bool) {
        let mut stats = self.stats.write().await;
        stats.is_online = is_online;
    }

    /// Update statistics
    async fn update_stats(&self) {
        let mut stats = self.stats.write().await;

        // Collect stats from all subsystems
        let service_worker_stats = self.service_worker.read().await.get_stats().await;
        let cache_stats = self.cache_manager.read().await.get_stats().await;
        let storage_stats = self.offline_storage.read().await.get_stats().await;
        let sync_stats = self.sync_manager.read().await.get_stats().await;
        let pwa_stats = self.pwa_features.read().await.get_stats().await;

        // Update combined stats
        stats.cache_hit_ratio = cache_stats.hit_ratio;
        stats.offline_data_size = storage_stats.total_data_size;
        stats.sync_operations = sync_stats.total_syncs;
        stats.failed_sync_operations = sync_stats.failed_syncs;
        stats.service_worker_status = service_worker_stats.status;
        stats.pwa_install_status = pwa_stats.install_status;

        // Update optimization benefits
        stats.optimization_benefits.insert(
            "service_worker".to_string(),
            service_worker_stats.optimization_benefit,
        );
        stats.optimization_benefits.insert(
            "cache_management".to_string(),
            cache_stats.optimization_benefit,
        );
        stats.optimization_benefits.insert(
            "offline_storage".to_string(),
            storage_stats.optimization_benefit,
        );
        stats.optimization_benefits.insert(
            "sync_management".to_string(),
            sync_stats.optimization_benefit,
        );
        stats
            .optimization_benefits
            .insert("pwa_features".to_string(), pwa_stats.optimization_benefit);

        stats.total_optimizations_applied = service_worker_stats.optimizations_applied
            + cache_stats.optimizations_applied
            + storage_stats.optimizations_applied
            + sync_stats.optimizations_applied
            + pwa_stats.optimizations_applied;
    }
}

/// Offline request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineRequest {
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

/// Offline response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineResponse {
    pub url: String,
    pub data: Vec<u8>,
    pub timestamp: std::time::SystemTime,
    pub source: OfflineResponseSource,
}

/// Offline response source
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OfflineResponseSource {
    Cache,
    OfflineStorage,
    OfflineFallback,
}

impl Default for OfflineSupportConfig {
    fn default() -> Self {
        Self {
            service_worker_config: ServiceWorkerConfig::default(),
            cache_config: CacheConfig::default(),
            storage_config: OfflineStorageConfig::default(),
            sync_config: SyncConfig::default(),
            pwa_config: PwaConfig::default(),
            enable_offline_mode: true,
            auto_sync_enabled: true,
            cache_size_limit: 50 * 1024 * 1024, // 50MB
            sync_interval: 300,                 // 5 minutes
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_offline_support_config() -> OfflineSupportConfig {
        OfflineSupportConfig {
            service_worker_config: ServiceWorkerConfig::default(),
            cache_config: CacheConfig::default(),
            storage_config: OfflineStorageConfig::default(),
            sync_config: SyncConfig::default(),
            pwa_config: PwaConfig::default(),
            enable_offline_mode: true,
            auto_sync_enabled: true,
            cache_size_limit: 10 * 1024 * 1024, // 10MB for testing
            sync_interval: 60,                  // 1 minute for testing
        }
    }

    #[tokio::test]
    async fn test_offline_support_manager_creation() {
        let config = create_test_offline_support_config();
        let manager = OfflineSupportManager::new(config);

        let stats = manager.get_stats().await;
        assert_eq!(stats.total_offline_requests, 0);
        assert_eq!(stats.sync_operations, 0);
        assert_eq!(stats.total_optimizations_applied, 0);
    }

    #[tokio::test]
    async fn test_offline_support_initialization() {
        let config = create_test_offline_support_config();
        let manager = OfflineSupportManager::new(config);

        let result = manager.initialize().await;
        assert!(result.is_ok());

        let stats = manager.get_stats().await;
        assert!(stats.total_optimizations_applied > 0);
    }

    #[tokio::test]
    async fn test_offline_request_handling() {
        let config = create_test_offline_support_config();
        let manager = OfflineSupportManager::new(config);

        let _ = manager.initialize().await;

        let request = OfflineRequest {
            url: "https://example.com/data".to_string(),
            method: "GET".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let response = manager.handle_offline_request(request).await;
        assert!(response.is_ok());

        let response = response.unwrap();
        assert_eq!(response.url, "https://example.com/data");
        assert_eq!(response.source, OfflineResponseSource::OfflineFallback);
    }

    #[tokio::test]
    async fn test_data_sync() {
        let config = create_test_offline_support_config();
        let manager = OfflineSupportManager::new(config);

        let _ = manager.initialize().await;

        let result = manager.sync_data().await;
        assert!(result.is_ok());

        let stats = manager.get_stats().await;
        assert!(stats.sync_operations > 0);
    }

    #[tokio::test]
    async fn test_online_status_management() {
        let config = create_test_offline_support_config();
        let manager = OfflineSupportManager::new(config);

        // Initially offline
        assert!(!manager.is_online().await);

        // Set online
        manager.set_online_status(true).await;
        assert!(manager.is_online().await);

        // Set offline
        manager.set_online_status(false).await;
        assert!(!manager.is_online().await);
    }

    #[tokio::test]
    async fn test_config_update() {
        let config = create_test_offline_support_config();
        let mut manager = OfflineSupportManager::new(config);

        let new_config = create_test_offline_support_config();
        let result = manager.update_config(new_config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_offline_support_statistics() {
        let config = create_test_offline_support_config();
        let manager = OfflineSupportManager::new(config);

        // Initialize to populate stats
        let _ = manager.initialize().await;

        let stats = manager.get_stats().await;
        assert!(stats.optimization_benefits.len() > 0);
        assert!(stats.total_optimizations_applied > 0);
    }
}
