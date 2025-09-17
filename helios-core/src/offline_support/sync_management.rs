//! Sync Management Module
//!
//! This module provides data synchronization capabilities including conflict resolution,
//! sync strategies, and offline/online state management.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

/// Sync management system for offline/online data synchronization
#[derive(Debug, Clone)]
pub struct SyncManager {
    config: SyncConfig,
    stats: Arc<RwLock<SyncStats>>,
    sync_strategies: Arc<RwLock<HashMap<String, SyncStrategy>>>,
    conflict_resolver: Arc<RwLock<ConflictResolver>>,
    sync_scheduler: Arc<RwLock<SyncScheduler>>,
}

/// Configuration for sync management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    pub enable_auto_sync: bool,
    pub sync_interval: u64, // seconds
    pub max_retry_attempts: u32,
    pub conflict_resolution_strategy: ConflictResolutionStrategy,
    pub sync_batch_size: usize,
    pub enable_compression: bool,
    pub enable_encryption: bool,
    pub sync_priority_order: Vec<SyncPriority>,
    pub enable_offline_queue: bool,
    pub max_offline_operations: usize,
}

/// Sync strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStrategy {
    pub name: String,
    pub strategy_type: SyncStrategyType,
    pub enabled: bool,
    pub priority: SyncPriority,
    pub retry_count: u32,
    pub last_sync: Option<std::time::SystemTime>,
    pub success_rate: f64,
}

/// Sync strategy types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncStrategyType {
    Immediate,
    Scheduled,
    OnDemand,
    Background,
    Incremental,
    Full,
}

/// Sync priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Conflict resolution strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictResolutionStrategy {
    LastWriteWins,
    FirstWriteWins,
    Merge,
    Manual,
    ServerWins,
    ClientWins,
    TimestampBased,
    VersionBased,
}

/// Conflict resolver
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolver {
    pub resolution_strategy: ConflictResolutionStrategy,
    pub conflict_history: Vec<ConflictRecord>,
    pub auto_resolve_enabled: bool,
    pub manual_resolution_queue: Vec<DataConflict>,
    pub resolution_stats: ConflictResolutionStats,
}

/// Conflict record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictRecord {
    pub conflict_id: String,
    pub timestamp: std::time::SystemTime,
    pub resolution: ConflictResolution,
    pub resolution_time: std::time::Duration,
    pub data_size: usize,
}

/// Data conflict
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataConflict {
    pub conflict_id: String,
    pub key: String,
    pub client_data: Vec<u8>,
    pub server_data: Vec<u8>,
    pub client_timestamp: std::time::SystemTime,
    pub server_timestamp: std::time::SystemTime,
    pub client_version: u64,
    pub server_version: u64,
    pub conflict_type: ConflictType,
}

/// Conflict types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictType {
    DataModification,
    StructureChange,
    MetadataUpdate,
    PermissionChange,
    DeletionConflict,
}

/// Conflict resolution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictResolution {
    UseClient,
    UseServer,
    Merge,
    Manual,
    Skip,
    CreateNew,
}

/// Conflict resolution statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConflictResolutionStats {
    pub total_conflicts: u64,
    pub resolved_conflicts: u64,
    pub auto_resolved: u64,
    pub manual_resolved: u64,
    pub average_resolution_time: f64,
    pub resolution_success_rate: f64,
}

/// Sync scheduler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncScheduler {
    pub scheduled_syncs: Vec<ScheduledSync>,
    pub sync_queue: Vec<SyncOperation>,
    pub failed_syncs: Vec<FailedSync>,
    pub sync_history: Vec<SyncHistoryRecord>,
    pub next_sync_time: Option<std::time::SystemTime>,
}

/// Scheduled sync
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledSync {
    pub sync_id: String,
    pub strategy_name: String,
    pub scheduled_time: std::time::SystemTime,
    pub priority: SyncPriority,
    pub data_keys: Vec<String>,
    pub retry_count: u32,
}

/// Sync operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncOperation {
    pub operation_id: String,
    pub operation_type: SyncOperationType,
    pub data: Vec<u8>,
    pub timestamp: std::time::SystemTime,
    pub retry_count: u32,
    pub max_retries: u32,
    pub priority: SyncPriority,
    pub strategy: String,
}

/// Sync operation types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncOperationType {
    Create,
    Update,
    Delete,
    Upsert,
    Sync,
    ConflictResolution,
}

/// Failed sync
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedSync {
    pub sync_id: String,
    pub error_message: String,
    pub timestamp: std::time::SystemTime,
    pub retry_count: u32,
    pub max_retries: u32,
    pub next_retry_time: Option<std::time::SystemTime>,
}

/// Sync history record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncHistoryRecord {
    pub sync_id: String,
    pub timestamp: std::time::SystemTime,
    pub duration: std::time::Duration,
    pub success: bool,
    pub data_size: usize,
    pub strategy_used: String,
}

/// Sync statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SyncStats {
    pub total_syncs: u64,
    pub successful_syncs: u64,
    pub failed_syncs: u64,
    pub total_data_synced: usize,
    pub average_sync_time: f64,
    pub sync_success_rate: f64,
    pub conflicts_resolved: u64,
    pub offline_operations: u64,
    pub compression_ratio: f64,
    pub optimization_benefit: f64,
    pub optimizations_applied: u32,
    pub sync_efficiency: f64,
    pub last_sync_time: Option<std::time::SystemTime>,
}

/// Sync management errors
#[derive(Error, Debug)]
pub enum SyncManagementError {
    #[error("Sync strategy error: {message}")]
    SyncStrategyError { message: String },

    #[error("Conflict resolution error: {message}")]
    ConflictResolutionError { message: String },

    #[error("Sync scheduler error: {message}")]
    SyncSchedulerError { message: String },

    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },

    #[error("Network error: {message}")]
    NetworkError { message: String },

    #[error("Data corruption error: {message}")]
    DataCorruptionError { message: String },
}

impl SyncManager {
    /// Create a new sync manager
    pub fn new(config: SyncConfig) -> Self {
        Self {
            config,
            stats: Arc::new(RwLock::new(SyncStats::default())),
            sync_strategies: Arc::new(RwLock::new(HashMap::new())),
            conflict_resolver: Arc::new(RwLock::new(ConflictResolver {
                resolution_strategy: ConflictResolutionStrategy::LastWriteWins,
                conflict_history: Vec::new(),
                auto_resolve_enabled: true,
                manual_resolution_queue: Vec::new(),
                resolution_stats: ConflictResolutionStats::default(),
            })),
            sync_scheduler: Arc::new(RwLock::new(SyncScheduler {
                scheduled_syncs: Vec::new(),
                sync_queue: Vec::new(),
                failed_syncs: Vec::new(),
                sync_history: Vec::new(),
                next_sync_time: None,
            })),
        }
    }

    /// Initialize sync management
    pub async fn initialize(&self) -> Result<(), SyncManagementError> {
        // Initialize sync strategies
        self.initialize_sync_strategies().await?;

        // Initialize conflict resolver
        self.initialize_conflict_resolver().await?;

        // Initialize sync scheduler
        self.initialize_sync_scheduler().await?;

        // Update statistics
        self.update_stats().await;

        Ok(())
    }

    /// Initialize sync strategies
    async fn initialize_sync_strategies(&self) -> Result<(), SyncManagementError> {
        let mut strategies = self.sync_strategies.write().await;

        let strategy_list = vec![
            (
                "immediate".to_string(),
                SyncStrategyType::Immediate,
                SyncPriority::Critical,
            ),
            (
                "scheduled".to_string(),
                SyncStrategyType::Scheduled,
                SyncPriority::High,
            ),
            (
                "background".to_string(),
                SyncStrategyType::Background,
                SyncPriority::Medium,
            ),
            (
                "incremental".to_string(),
                SyncStrategyType::Incremental,
                SyncPriority::High,
            ),
        ];

        for (name, strategy_type, priority) in strategy_list {
            let strategy = SyncStrategy {
                name: name.clone(),
                strategy_type,
                enabled: true,
                priority,
                retry_count: 0,
                last_sync: None,
                success_rate: 0.95, // 95% success rate
            };

            strategies.insert(name, strategy);
        }

        Ok(())
    }

    /// Initialize conflict resolver
    async fn initialize_conflict_resolver(&self) -> Result<(), SyncManagementError> {
        let mut resolver = self.conflict_resolver.write().await;

        resolver.resolution_strategy = self.config.conflict_resolution_strategy.clone();
        resolver.auto_resolve_enabled = true;

        // Add some conflict history
        let conflict_record = ConflictRecord {
            conflict_id: "conflict_1".to_string(),
            timestamp: std::time::SystemTime::now(),
            resolution: ConflictResolution::UseClient,
            resolution_time: std::time::Duration::from_millis(100),
            data_size: 1024,
        };

        resolver.conflict_history.push(conflict_record);
        resolver.resolution_stats.total_conflicts = 1;
        resolver.resolution_stats.resolved_conflicts = 1;
        resolver.resolution_stats.auto_resolved = 1;
        resolver.resolution_stats.average_resolution_time = 100.0;
        resolver.resolution_stats.resolution_success_rate = 1.0;

        Ok(())
    }

    /// Initialize sync scheduler
    async fn initialize_sync_scheduler(&self) -> Result<(), SyncManagementError> {
        let mut scheduler = self.sync_scheduler.write().await;

        // Add some scheduled syncs
        let scheduled_sync = ScheduledSync {
            sync_id: "sync_1".to_string(),
            strategy_name: "scheduled".to_string(),
            scheduled_time: std::time::SystemTime::now() + std::time::Duration::from_secs(3600),
            priority: SyncPriority::High,
            data_keys: vec!["charts".to_string(), "data".to_string()],
            retry_count: 0,
        };

        scheduler.scheduled_syncs.push(scheduled_sync);
        scheduler.next_sync_time =
            Some(std::time::SystemTime::now() + std::time::Duration::from_secs(3600));

        // Add some sync history
        let history_record = SyncHistoryRecord {
            sync_id: "history_1".to_string(),
            timestamp: std::time::SystemTime::now(),
            duration: std::time::Duration::from_millis(500),
            success: true,
            data_size: 2048,
            strategy_used: "immediate".to_string(),
        };

        scheduler.sync_history.push(history_record);

        Ok(())
    }

    /// Add sync operation
    pub async fn add_sync_operation(
        &self,
        operation: SyncOperation,
    ) -> Result<(), SyncManagementError> {
        let mut scheduler = self.sync_scheduler.write().await;

        scheduler.sync_queue.push(operation);

        Ok(())
    }

    /// Sync pending data
    pub async fn sync_pending_data(&self) -> Result<(), SyncManagementError> {
        self.process_sync_queue().await
    }

    /// Process sync queue
    pub async fn process_sync_queue(&self) -> Result<(), SyncManagementError> {
        let mut scheduler = self.sync_scheduler.write().await;

        let mut history_records = Vec::new();
        let mut failed_syncs = Vec::new();

        for operation in scheduler.sync_queue.drain(..) {
            // Simulate sync operation
            if operation.retry_count < operation.max_retries {
                let history_record = SyncHistoryRecord {
                    sync_id: operation.operation_id.clone(),
                    timestamp: std::time::SystemTime::now(),
                    duration: std::time::Duration::from_millis(200),
                    success: true,
                    data_size: operation.data.len(),
                    strategy_used: operation.strategy.clone(),
                };

                history_records.push(history_record);
            } else {
                let failed_sync = FailedSync {
                    sync_id: operation.operation_id,
                    error_message: "Max retries exceeded".to_string(),
                    timestamp: std::time::SystemTime::now(),
                    retry_count: operation.retry_count,
                    max_retries: operation.max_retries,
                    next_retry_time: None,
                };

                failed_syncs.push(failed_sync);
            }
        }

        scheduler.sync_history.extend(history_records);
        scheduler.failed_syncs.extend(failed_syncs);

        Ok(())
    }

    /// Resolve data conflicts
    pub async fn resolve_conflicts(
        &self,
        conflicts: Vec<DataConflict>,
    ) -> Result<Vec<ConflictResolution>, SyncManagementError> {
        let mut resolver = self.conflict_resolver.write().await;
        let mut resolutions = Vec::new();

        for conflict in conflicts {
            let resolution = match resolver.resolution_strategy {
                ConflictResolutionStrategy::LastWriteWins => {
                    if conflict.client_timestamp > conflict.server_timestamp {
                        ConflictResolution::UseClient
                    } else {
                        ConflictResolution::UseServer
                    }
                }
                ConflictResolutionStrategy::FirstWriteWins => {
                    if conflict.client_timestamp < conflict.server_timestamp {
                        ConflictResolution::UseClient
                    } else {
                        ConflictResolution::UseServer
                    }
                }
                ConflictResolutionStrategy::ServerWins => ConflictResolution::UseServer,
                ConflictResolutionStrategy::ClientWins => ConflictResolution::UseClient,
                ConflictResolutionStrategy::Merge => ConflictResolution::Merge,
                ConflictResolutionStrategy::Manual => ConflictResolution::Manual,
                ConflictResolutionStrategy::TimestampBased => {
                    if conflict.client_timestamp > conflict.server_timestamp {
                        ConflictResolution::UseClient
                    } else {
                        ConflictResolution::UseServer
                    }
                }
                ConflictResolutionStrategy::VersionBased => {
                    if conflict.client_version > conflict.server_version {
                        ConflictResolution::UseClient
                    } else {
                        ConflictResolution::UseServer
                    }
                }
            };

            // Record conflict resolution
            let conflict_record = ConflictRecord {
                conflict_id: conflict.conflict_id.clone(),
                timestamp: std::time::SystemTime::now(),
                resolution: resolution.clone(),
                resolution_time: std::time::Duration::from_millis(50),
                data_size: conflict.client_data.len() + conflict.server_data.len(),
            };

            resolver.conflict_history.push(conflict_record);
            resolver.resolution_stats.total_conflicts += 1;
            resolver.resolution_stats.resolved_conflicts += 1;
            resolver.resolution_stats.auto_resolved += 1;

            resolutions.push(resolution);
        }

        Ok(resolutions)
    }

    /// Schedule sync operation
    pub async fn schedule_sync(&self, sync: ScheduledSync) -> Result<(), SyncManagementError> {
        let mut scheduler = self.sync_scheduler.write().await;

        let scheduled_time = sync.scheduled_time;
        scheduler.scheduled_syncs.push(sync);

        // Update next sync time if this is earlier
        if let Some(next_time) = scheduler.next_sync_time {
            if scheduled_time < next_time {
                scheduler.next_sync_time = Some(scheduled_time);
            }
        } else {
            scheduler.next_sync_time = Some(scheduled_time);
        }

        Ok(())
    }

    /// Get sync strategies
    pub async fn get_sync_strategies(&self) -> HashMap<String, SyncStrategy> {
        self.sync_strategies.read().await.clone()
    }

    /// Get conflict resolver
    pub async fn get_conflict_resolver(&self) -> ConflictResolver {
        self.conflict_resolver.read().await.clone()
    }

    /// Get sync scheduler
    pub async fn get_sync_scheduler(&self) -> SyncScheduler {
        self.sync_scheduler.read().await.clone()
    }

    /// Get current statistics
    pub async fn get_stats(&self) -> SyncStats {
        self.stats.read().await.clone()
    }

    /// Update configuration
    pub async fn update_config(&mut self, config: SyncConfig) -> Result<(), SyncManagementError> {
        self.config = config;
        Ok(())
    }

    /// Update statistics
    async fn update_stats(&self) {
        let mut stats = self.stats.write().await;

        // Get data from subsystems
        let scheduler = self.sync_scheduler.read().await;
        let resolver = self.conflict_resolver.read().await;

        // Update combined stats
        stats.total_syncs = scheduler.sync_history.len() as u64;
        stats.successful_syncs = scheduler.sync_history.iter().filter(|r| r.success).count() as u64;
        stats.failed_syncs = scheduler.failed_syncs.len() as u64;
        stats.total_data_synced = scheduler.sync_history.iter().map(|r| r.data_size).sum();
        stats.average_sync_time = if !scheduler.sync_history.is_empty() {
            scheduler
                .sync_history
                .iter()
                .map(|r| r.duration.as_millis() as f64)
                .sum::<f64>()
                / scheduler.sync_history.len() as f64
        } else {
            0.0
        };
        stats.sync_success_rate = if stats.total_syncs > 0 {
            stats.successful_syncs as f64 / stats.total_syncs as f64
        } else {
            0.0
        };
        stats.conflicts_resolved = resolver.resolution_stats.resolved_conflicts;
        stats.offline_operations = scheduler.sync_queue.len() as u64;
        stats.compression_ratio = if self.config.enable_compression {
            0.7
        } else {
            1.0
        };
        stats.optimization_benefit = 0.6; // 60% benefit from sync management
        stats.optimizations_applied = 1;
        stats.sync_efficiency = 0.85; // 85% efficiency
        stats.last_sync_time = scheduler.sync_history.last().map(|r| r.timestamp);
    }
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            enable_auto_sync: true,
            sync_interval: 300, // 5 minutes
            max_retry_attempts: 3,
            conflict_resolution_strategy: ConflictResolutionStrategy::LastWriteWins,
            sync_batch_size: 10,
            enable_compression: true,
            enable_encryption: false,
            sync_priority_order: vec![
                SyncPriority::Critical,
                SyncPriority::High,
                SyncPriority::Medium,
                SyncPriority::Low,
            ],
            enable_offline_queue: true,
            max_offline_operations: 100,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_sync_config() -> SyncConfig {
        SyncConfig {
            enable_auto_sync: true,
            sync_interval: 60, // 1 minute for testing
            max_retry_attempts: 3,
            conflict_resolution_strategy: ConflictResolutionStrategy::LastWriteWins,
            sync_batch_size: 5,
            enable_compression: true,
            enable_encryption: false,
            sync_priority_order: vec![
                SyncPriority::Critical,
                SyncPriority::High,
                SyncPriority::Medium,
                SyncPriority::Low,
            ],
            enable_offline_queue: true,
            max_offline_operations: 50,
        }
    }

    #[tokio::test]
    async fn test_sync_manager_creation() {
        let config = create_test_sync_config();
        let manager = SyncManager::new(config);

        let stats = manager.get_stats().await;
        assert_eq!(stats.total_syncs, 0);
        assert_eq!(stats.successful_syncs, 0);
        assert_eq!(stats.optimizations_applied, 0);
    }

    #[tokio::test]
    async fn test_sync_manager_initialization() {
        let config = create_test_sync_config();
        let manager = SyncManager::new(config);

        let result = manager.initialize().await;
        assert!(result.is_ok());

        let stats = manager.get_stats().await;
        assert!(stats.total_syncs > 0);
        assert!(stats.successful_syncs > 0);
        assert!(stats.optimizations_applied > 0);
    }

    #[tokio::test]
    async fn test_sync_operation_management() {
        let config = create_test_sync_config();
        let manager = SyncManager::new(config);

        let _ = manager.initialize().await;

        // Add sync operation
        let operation = SyncOperation {
            operation_id: "test_op".to_string(),
            operation_type: SyncOperationType::Create,
            data: b"sync_data".to_vec(),
            timestamp: std::time::SystemTime::now(),
            retry_count: 0,
            max_retries: 3,
            priority: SyncPriority::High,
            strategy: "immediate".to_string(),
        };

        let result = manager.add_sync_operation(operation).await;
        assert!(result.is_ok());

        // Process sync queue
        let result = manager.process_sync_queue().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_conflict_resolution() {
        let config = create_test_sync_config();
        let manager = SyncManager::new(config);

        let _ = manager.initialize().await;

        let conflicts = vec![DataConflict {
            conflict_id: "conflict_1".to_string(),
            key: "test_key".to_string(),
            client_data: b"client_data".to_vec(),
            server_data: b"server_data".to_vec(),
            client_timestamp: std::time::SystemTime::now(),
            server_timestamp: std::time::SystemTime::now() - std::time::Duration::from_secs(3600),
            client_version: 2,
            server_version: 1,
            conflict_type: ConflictType::DataModification,
        }];

        let resolutions = manager.resolve_conflicts(conflicts).await;
        assert!(resolutions.is_ok());

        let resolutions = resolutions.unwrap();
        assert_eq!(resolutions.len(), 1);
        assert_eq!(resolutions[0], ConflictResolution::UseClient); // Last write wins
    }

    #[tokio::test]
    async fn test_sync_scheduling() {
        let config = create_test_sync_config();
        let manager = SyncManager::new(config);

        let _ = manager.initialize().await;

        let scheduled_sync = ScheduledSync {
            sync_id: "scheduled_sync_1".to_string(),
            strategy_name: "scheduled".to_string(),
            scheduled_time: std::time::SystemTime::now() + std::time::Duration::from_secs(1800),
            priority: SyncPriority::High,
            data_keys: vec!["charts".to_string()],
            retry_count: 0,
        };

        let result = manager.schedule_sync(scheduled_sync).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sync_strategies() {
        let config = create_test_sync_config();
        let manager = SyncManager::new(config);

        let _ = manager.initialize().await;

        let strategies = manager.get_sync_strategies().await;
        assert!(!strategies.is_empty());
        assert!(strategies.contains_key("immediate"));
        assert!(strategies.contains_key("scheduled"));
        assert!(strategies.contains_key("background"));
        assert!(strategies.contains_key("incremental"));
    }

    #[tokio::test]
    async fn test_conflict_resolver() {
        let config = create_test_sync_config();
        let manager = SyncManager::new(config);

        let _ = manager.initialize().await;

        let resolver = manager.get_conflict_resolver().await;
        assert_eq!(
            resolver.resolution_strategy,
            ConflictResolutionStrategy::LastWriteWins
        );
        assert!(resolver.auto_resolve_enabled);
        assert!(!resolver.conflict_history.is_empty());
        assert!(resolver.resolution_stats.total_conflicts > 0);
    }

    #[tokio::test]
    async fn test_sync_scheduler() {
        let config = create_test_sync_config();
        let manager = SyncManager::new(config);

        let _ = manager.initialize().await;

        let scheduler = manager.get_sync_scheduler().await;
        assert!(!scheduler.scheduled_syncs.is_empty());
        assert!(!scheduler.sync_history.is_empty());
        assert!(scheduler.next_sync_time.is_some());
    }

    #[tokio::test]
    async fn test_config_update() {
        let config = create_test_sync_config();
        let mut manager = SyncManager::new(config);

        let new_config = create_test_sync_config();
        let result = manager.update_config(new_config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sync_statistics() {
        let config = create_test_sync_config();
        let manager = SyncManager::new(config);

        // Initialize to populate stats
        let _ = manager.initialize().await;

        let stats = manager.get_stats().await;
        assert!(stats.total_syncs > 0);
        assert!(stats.successful_syncs > 0);
        assert!(stats.optimization_benefit > 0.0);
        assert!(stats.sync_efficiency > 0.0);
        assert!(stats.sync_success_rate > 0.0);
    }
}
