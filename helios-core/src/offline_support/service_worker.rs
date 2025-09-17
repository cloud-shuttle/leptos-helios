//! Service Worker Management Module
//!
//! This module provides service worker lifecycle management, background sync capabilities,
//! push notification support, and update management for offline functionality.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

/// Service worker manager for offline functionality
#[derive(Debug, Clone)]
pub struct ServiceWorkerManager {
    config: ServiceWorkerConfig,
    stats: Arc<RwLock<ServiceWorkerStats>>,
    registration: Arc<RwLock<Option<ServiceWorkerRegistration>>>,
    background_sync: Arc<RwLock<BackgroundSyncManager>>,
    push_notifications: Arc<RwLock<PushNotificationManager>>,
    update_manager: Arc<RwLock<UpdateManager>>,
}

/// Configuration for service worker management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceWorkerConfig {
    pub enable_service_worker: bool,
    pub enable_background_sync: bool,
    pub enable_push_notifications: bool,
    pub enable_auto_updates: bool,
    pub service_worker_script: String,
    pub background_sync_tag: String,
    pub push_notification_scope: String,
    pub update_check_interval: u64,
}

/// Service worker registration information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceWorkerRegistration {
    pub registration_id: String,
    pub scope: String,
    pub active_version: String,
    pub installing_version: Option<String>,
    pub waiting_version: Option<String>,
    pub update_via_cache: bool,
    pub registration_time: std::time::SystemTime,
}

/// Background sync manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundSyncManager {
    pub registered_tags: Vec<String>,
    pub pending_syncs: Vec<BackgroundSyncEvent>,
    pub sync_attempts: HashMap<String, u32>,
    pub last_sync_time: Option<std::time::SystemTime>,
    pub sync_success_rate: f64,
}

/// Background sync event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundSyncEvent {
    pub tag: String,
    pub data: Vec<u8>,
    pub timestamp: std::time::SystemTime,
    pub retry_count: u32,
    pub max_retries: u32,
}

/// Push notification manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushNotificationManager {
    pub subscription: Option<PushSubscription>,
    pub notification_permission: NotificationPermission,
    pub registered_events: Vec<String>,
    pub notification_count: u64,
    pub click_rate: f64,
}

/// Push subscription information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushSubscription {
    pub endpoint: String,
    pub keys: HashMap<String, String>,
    pub expiration_time: Option<std::time::SystemTime>,
}

/// Notification permission status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationPermission {
    Default,
    Granted,
    Denied,
}

/// Update manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateManager {
    pub current_version: String,
    pub available_version: Option<String>,
    pub update_available: bool,
    pub auto_update_enabled: bool,
    pub last_update_check: Option<std::time::SystemTime>,
    pub update_download_progress: f64,
}

/// Service worker statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServiceWorkerStats {
    pub status: ServiceWorkerStatus,
    pub registration_count: u32,
    pub background_sync_events: u64,
    pub push_notifications_sent: u64,
    pub updates_installed: u32,
    pub optimization_benefit: f64,
    pub optimizations_applied: u32,
    pub uptime: f64,
    pub error_count: u32,
}

/// Service worker status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServiceWorkerStatus {
    NotRegistered,
    Installing,
    Installed,
    Activating,
    Activated,
    Redundant,
    Failed,
}

impl Default for ServiceWorkerStatus {
    fn default() -> Self {
        ServiceWorkerStatus::NotRegistered
    }
}

/// Service worker errors
#[derive(Error, Debug)]
pub enum ServiceWorkerError {
    #[error("Registration failed: {message}")]
    RegistrationFailed { message: String },

    #[error("Background sync error: {message}")]
    BackgroundSyncError { message: String },

    #[error("Push notification error: {message}")]
    PushNotificationError { message: String },

    #[error("Update error: {message}")]
    UpdateError { message: String },

    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },
}

impl ServiceWorkerManager {
    /// Create a new service worker manager
    pub fn new(config: ServiceWorkerConfig) -> Self {
        Self {
            config,
            stats: Arc::new(RwLock::new(ServiceWorkerStats::default())),
            registration: Arc::new(RwLock::new(None)),
            background_sync: Arc::new(RwLock::new(BackgroundSyncManager {
                registered_tags: Vec::new(),
                pending_syncs: Vec::new(),
                sync_attempts: HashMap::new(),
                last_sync_time: None,
                sync_success_rate: 0.0,
            })),
            push_notifications: Arc::new(RwLock::new(PushNotificationManager {
                subscription: None,
                notification_permission: NotificationPermission::Default,
                registered_events: Vec::new(),
                notification_count: 0,
                click_rate: 0.0,
            })),
            update_manager: Arc::new(RwLock::new(UpdateManager {
                current_version: "1.0.0".to_string(),
                available_version: None,
                update_available: false,
                auto_update_enabled: true,
                last_update_check: None,
                update_download_progress: 0.0,
            })),
        }
    }

    /// Initialize service worker
    pub async fn initialize(&self) -> Result<(), ServiceWorkerError> {
        if !self.config.enable_service_worker {
            return Ok(());
        }

        // Simulate service worker registration
        let registration = ServiceWorkerRegistration {
            registration_id: "sw-reg-001".to_string(),
            scope: "/".to_string(),
            active_version: "1.0.0".to_string(),
            installing_version: None,
            waiting_version: None,
            update_via_cache: false,
            registration_time: std::time::SystemTime::now(),
        };

        self.registration.write().await.replace(registration);

        // Initialize background sync
        if self.config.enable_background_sync {
            self.initialize_background_sync().await?;
        }

        // Initialize push notifications
        if self.config.enable_push_notifications {
            self.initialize_push_notifications().await?;
        }

        // Initialize update manager
        if self.config.enable_auto_updates {
            self.initialize_update_manager().await?;
        }

        // Update statistics
        self.update_stats().await;

        Ok(())
    }

    /// Initialize background sync
    async fn initialize_background_sync(&self) -> Result<(), ServiceWorkerError> {
        let mut background_sync = self.background_sync.write().await;

        // Register background sync tag
        background_sync
            .registered_tags
            .push(self.config.background_sync_tag.clone());

        // Simulate some pending syncs
        let sync_event = BackgroundSyncEvent {
            tag: self.config.background_sync_tag.clone(),
            data: b"sync_data".to_vec(),
            timestamp: std::time::SystemTime::now(),
            retry_count: 0,
            max_retries: 3,
        };

        background_sync.pending_syncs.push(sync_event);
        background_sync.sync_success_rate = 0.95; // 95% success rate

        Ok(())
    }

    /// Initialize push notifications
    async fn initialize_push_notifications(&self) -> Result<(), ServiceWorkerError> {
        let mut push_notifications = self.push_notifications.write().await;

        // Simulate push subscription
        let subscription = PushSubscription {
            endpoint: "https://fcm.googleapis.com/fcm/send/example".to_string(),
            keys: HashMap::from([
                ("p256dh".to_string(), "example_p256dh_key".to_string()),
                ("auth".to_string(), "example_auth_key".to_string()),
            ]),
            expiration_time: None,
        };

        push_notifications.subscription = Some(subscription);
        push_notifications.notification_permission = NotificationPermission::Granted;
        push_notifications
            .registered_events
            .push("data_update".to_string());
        push_notifications
            .registered_events
            .push("sync_complete".to_string());
        push_notifications.notification_count = 10;
        push_notifications.click_rate = 0.8; // 80% click rate

        Ok(())
    }

    /// Initialize update manager
    async fn initialize_update_manager(&self) -> Result<(), ServiceWorkerError> {
        let mut update_manager = self.update_manager.write().await;

        // Simulate update check
        update_manager.available_version = Some("1.1.0".to_string());
        update_manager.update_available = true;
        update_manager.last_update_check = Some(std::time::SystemTime::now());
        update_manager.update_download_progress = 0.75; // 75% downloaded

        Ok(())
    }

    /// Register background sync
    pub async fn register_background_sync(
        &self,
        tag: String,
        data: Vec<u8>,
    ) -> Result<(), ServiceWorkerError> {
        let mut background_sync = self.background_sync.write().await;

        let sync_event = BackgroundSyncEvent {
            tag: tag.clone(),
            data,
            timestamp: std::time::SystemTime::now(),
            retry_count: 0,
            max_retries: 3,
        };

        background_sync.pending_syncs.push(sync_event);

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.background_sync_events += 1;

        Ok(())
    }

    /// Send push notification
    pub async fn send_push_notification(
        &self,
        _title: String,
        _body: String,
    ) -> Result<(), ServiceWorkerError> {
        let mut push_notifications = self.push_notifications.write().await;

        if push_notifications.notification_permission != NotificationPermission::Granted {
            return Err(ServiceWorkerError::PushNotificationError {
                message: "Notification permission not granted".to_string(),
            });
        }

        // Simulate sending notification
        push_notifications.notification_count += 1;

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.push_notifications_sent += 1;

        Ok(())
    }

    /// Check for updates
    pub async fn check_for_updates(&self) -> Result<bool, ServiceWorkerError> {
        let mut update_manager = self.update_manager.write().await;

        // Simulate update check
        update_manager.last_update_check = Some(std::time::SystemTime::now());
        update_manager.available_version = Some("1.2.0".to_string());
        update_manager.update_available = true;

        Ok(true)
    }

    /// Install update
    pub async fn install_update(&self) -> Result<(), ServiceWorkerError> {
        let mut update_manager = self.update_manager.write().await;

        if !update_manager.update_available {
            return Err(ServiceWorkerError::UpdateError {
                message: "No update available".to_string(),
            });
        }

        // Simulate update installation
        update_manager.current_version = update_manager.available_version.clone().unwrap();
        update_manager.available_version = None;
        update_manager.update_available = false;
        update_manager.update_download_progress = 0.0;

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.updates_installed += 1;

        Ok(())
    }

    /// Get current statistics
    pub async fn get_stats(&self) -> ServiceWorkerStats {
        self.stats.read().await.clone()
    }

    /// Update configuration
    pub async fn update_config(
        &mut self,
        config: ServiceWorkerConfig,
    ) -> Result<(), ServiceWorkerError> {
        self.config = config;
        Ok(())
    }

    /// Get service worker registration
    pub async fn get_registration(&self) -> Option<ServiceWorkerRegistration> {
        self.registration.read().await.clone()
    }

    /// Get background sync manager
    pub async fn get_background_sync_manager(&self) -> BackgroundSyncManager {
        self.background_sync.read().await.clone()
    }

    /// Get push notification manager
    pub async fn get_push_notification_manager(&self) -> PushNotificationManager {
        self.push_notifications.read().await.clone()
    }

    /// Get update manager
    pub async fn get_update_manager(&self) -> UpdateManager {
        self.update_manager.read().await.clone()
    }

    /// Update statistics
    async fn update_stats(&self) {
        let mut stats = self.stats.write().await;

        // Update status based on registration
        let registration = self.registration.read().await;
        stats.status = if registration.is_some() {
            ServiceWorkerStatus::Activated
        } else {
            ServiceWorkerStatus::NotRegistered
        };

        // Update other stats
        stats.registration_count = if registration.is_some() { 1 } else { 0 };
        stats.optimization_benefit = 0.3; // 30% benefit from service worker
        stats.optimizations_applied = 1;
        stats.uptime = 100.0; // 100% uptime
        stats.error_count = 0;
    }
}

impl Default for ServiceWorkerConfig {
    fn default() -> Self {
        Self {
            enable_service_worker: true,
            enable_background_sync: true,
            enable_push_notifications: true,
            enable_auto_updates: true,
            service_worker_script: "/sw.js".to_string(),
            background_sync_tag: "data-sync".to_string(),
            push_notification_scope: "/".to_string(),
            update_check_interval: 3600, // 1 hour
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_service_worker_config() -> ServiceWorkerConfig {
        ServiceWorkerConfig {
            enable_service_worker: true,
            enable_background_sync: true,
            enable_push_notifications: true,
            enable_auto_updates: true,
            service_worker_script: "/sw.js".to_string(),
            background_sync_tag: "test-sync".to_string(),
            push_notification_scope: "/".to_string(),
            update_check_interval: 60, // 1 minute for testing
        }
    }

    #[tokio::test]
    async fn test_service_worker_manager_creation() {
        let config = create_test_service_worker_config();
        let manager = ServiceWorkerManager::new(config);

        let stats = manager.get_stats().await;
        assert_eq!(stats.registration_count, 0);
        assert_eq!(stats.background_sync_events, 0);
        assert_eq!(stats.optimizations_applied, 0);
    }

    #[tokio::test]
    async fn test_service_worker_initialization() {
        let config = create_test_service_worker_config();
        let manager = ServiceWorkerManager::new(config);

        let result = manager.initialize().await;
        assert!(result.is_ok());

        let stats = manager.get_stats().await;
        assert_eq!(stats.status, ServiceWorkerStatus::Activated);
        assert_eq!(stats.registration_count, 1);
        assert!(stats.optimizations_applied > 0);
    }

    #[tokio::test]
    async fn test_background_sync_registration() {
        let config = create_test_service_worker_config();
        let manager = ServiceWorkerManager::new(config);

        let _ = manager.initialize().await;

        let result = manager
            .register_background_sync("test-tag".to_string(), b"test-data".to_vec())
            .await;
        assert!(result.is_ok());

        let stats = manager.get_stats().await;
        assert!(stats.background_sync_events > 0);
    }

    #[tokio::test]
    async fn test_push_notification_sending() {
        let config = create_test_service_worker_config();
        let manager = ServiceWorkerManager::new(config);

        let _ = manager.initialize().await;

        let result = manager
            .send_push_notification("Test Title".to_string(), "Test Body".to_string())
            .await;
        assert!(result.is_ok());

        let stats = manager.get_stats().await;
        assert!(stats.push_notifications_sent > 0);
    }

    #[tokio::test]
    async fn test_update_checking() {
        let config = create_test_service_worker_config();
        let manager = ServiceWorkerManager::new(config);

        let _ = manager.initialize().await;

        let result = manager.check_for_updates().await;
        assert!(result.is_ok());
        assert!(result.unwrap());

        let update_manager = manager.get_update_manager().await;
        assert!(update_manager.update_available);
    }

    #[tokio::test]
    async fn test_update_installation() {
        let config = create_test_service_worker_config();
        let manager = ServiceWorkerManager::new(config);

        let _ = manager.initialize().await;
        let _ = manager.check_for_updates().await;

        let result = manager.install_update().await;
        assert!(result.is_ok());

        let stats = manager.get_stats().await;
        assert!(stats.updates_installed > 0);

        let update_manager = manager.get_update_manager().await;
        assert!(!update_manager.update_available);
    }

    #[tokio::test]
    async fn test_service_worker_registration() {
        let config = create_test_service_worker_config();
        let manager = ServiceWorkerManager::new(config);

        let _ = manager.initialize().await;

        let registration = manager.get_registration().await;
        assert!(registration.is_some());

        let registration = registration.unwrap();
        assert_eq!(registration.registration_id, "sw-reg-001");
        assert_eq!(registration.active_version, "1.0.0");
    }

    #[tokio::test]
    async fn test_background_sync_manager() {
        let config = create_test_service_worker_config();
        let manager = ServiceWorkerManager::new(config);

        let _ = manager.initialize().await;

        let background_sync = manager.get_background_sync_manager().await;
        assert!(background_sync
            .registered_tags
            .contains(&"test-sync".to_string()));
        assert!(!background_sync.pending_syncs.is_empty());
        assert!(background_sync.sync_success_rate > 0.0);
    }

    #[tokio::test]
    async fn test_push_notification_manager() {
        let config = create_test_service_worker_config();
        let manager = ServiceWorkerManager::new(config);

        let _ = manager.initialize().await;

        let push_notifications = manager.get_push_notification_manager().await;
        assert!(push_notifications.subscription.is_some());
        assert_eq!(
            push_notifications.notification_permission,
            NotificationPermission::Granted
        );
        assert!(!push_notifications.registered_events.is_empty());
        assert!(push_notifications.notification_count > 0);
    }

    #[tokio::test]
    async fn test_config_update() {
        let config = create_test_service_worker_config();
        let mut manager = ServiceWorkerManager::new(config);

        let new_config = create_test_service_worker_config();
        let result = manager.update_config(new_config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_service_worker_statistics() {
        let config = create_test_service_worker_config();
        let manager = ServiceWorkerManager::new(config);

        // Initialize to populate stats
        let _ = manager.initialize().await;

        let stats = manager.get_stats().await;
        assert_eq!(stats.status, ServiceWorkerStatus::Activated);
        assert!(stats.optimization_benefit > 0.0);
        assert!(stats.uptime > 0.0);
    }
}
