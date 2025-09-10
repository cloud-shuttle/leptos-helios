//! Conflict Resolution for leptos-helios
//!
//! This module provides operational transformation and conflict resolution
//! capabilities for collaborative editing, including operation transformation,
//! conflict detection, and resolution strategies.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use thiserror::Error;
use tokio::sync::{mpsc, RwLock};

use super::collaborative_editing::ElementChanges;
use super::collaborative_editing::{EditOperation, EditOperationType};

/// Represents a conflict between two operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Conflict {
    pub conflict_id: String,
    pub operation1_id: String,
    pub operation2_id: String,
    pub conflict_type: ConflictType,
    pub severity: ConflictSeverity,
    pub timestamp: u64,
    pub resolution: Option<ConflictResolution>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Types of conflicts that can occur
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictType {
    /// Two operations modify the same element
    ElementModification,
    /// Two operations try to delete the same element
    ElementDeletion,
    /// Two operations try to create elements with the same ID
    ElementCreation,
    /// Two operations modify overlapping regions
    RegionOverlap,
    /// Two operations have conflicting style changes
    StyleConflict,
    /// Two operations have conflicting data changes
    DataConflict,
    /// Custom conflict type
    Custom(String),
}

/// Severity levels for conflicts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Resolution strategies for conflicts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictResolution {
    /// Last write wins - the most recent operation takes precedence
    LastWriteWins,
    /// First write wins - the first operation takes precedence
    FirstWriteWins,
    /// User priority - operations from higher priority users win
    UserPriority { user_id: String },
    /// Manual resolution required
    Manual,
    /// Automatic merge using operational transformation
    OperationalTransform { transformed_ops: Vec<EditOperation> },
    /// Custom resolution
    Custom {
        strategy: String,
        parameters: HashMap<String, serde_json::Value>,
    },
}

/// Operational transformation context
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransformContext {
    pub base_version: u64,
    pub operations: Vec<EditOperation>,
    pub user_id: String,
    pub timestamp: u64,
}

/// Transformation result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransformResult {
    pub transformed_operations: Vec<EditOperation>,
    pub conflicts: Vec<Conflict>,
    pub success: bool,
    pub error_message: Option<String>,
}

/// Conflict resolution strategy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResolutionStrategy {
    pub strategy_id: String,
    pub name: String,
    pub description: String,
    pub conflict_types: Vec<ConflictType>,
    pub auto_resolve: bool,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Conflict resolution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictStats {
    pub total_conflicts: u64,
    pub resolved_conflicts: u64,
    pub unresolved_conflicts: u64,
    pub conflicts_by_type: HashMap<String, u64>,
    pub conflicts_by_severity: HashMap<String, u64>,
    pub average_resolution_time: Duration,
    pub last_conflict_time: Option<u64>,
}

/// Errors that can occur during conflict resolution
#[derive(Debug, Error)]
pub enum ConflictResolutionError {
    #[error("Operation not found: {operation_id}")]
    OperationNotFound { operation_id: String },

    #[error("Conflict not found: {conflict_id}")]
    ConflictNotFound { conflict_id: String },

    #[error("Invalid transformation: {reason}")]
    InvalidTransformation { reason: String },

    #[error("Resolution strategy not found: {strategy_id}")]
    ResolutionStrategyNotFound { strategy_id: String },

    #[error("Circular dependency detected in operations")]
    CircularDependency,

    #[error("Operation version mismatch: expected {expected}, got {actual}")]
    VersionMismatch { expected: u64, actual: u64 },

    #[error("Transform context invalid: {reason}")]
    InvalidTransformContext { reason: String },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Collaboration error: {0}")]
    Collaboration(#[from] super::CollaborationError),
}

/// Manager for conflict resolution and operational transformation
pub struct ConflictResolutionManager {
    conflicts: Arc<RwLock<HashMap<String, Conflict>>>,
    resolution_strategies: Arc<RwLock<HashMap<String, ResolutionStrategy>>>,
    operation_history: Arc<RwLock<VecDeque<EditOperation>>>,
    stats: Arc<RwLock<ConflictStats>>,
    conflict_sender: mpsc::UnboundedSender<Conflict>,
    conflict_receiver: Arc<RwLock<Option<mpsc::UnboundedReceiver<Conflict>>>>,
}

impl ConflictResolutionManager {
    /// Create a new conflict resolution manager
    pub fn new() -> Self {
        let (conflict_sender, conflict_receiver) = mpsc::unbounded_channel();

        Self {
            conflicts: Arc::new(RwLock::new(HashMap::new())),
            resolution_strategies: Arc::new(RwLock::new(HashMap::new())),
            operation_history: Arc::new(RwLock::new(VecDeque::new())),
            stats: Arc::new(RwLock::new(ConflictStats {
                total_conflicts: 0,
                resolved_conflicts: 0,
                unresolved_conflicts: 0,
                conflicts_by_type: HashMap::new(),
                conflicts_by_severity: HashMap::new(),
                average_resolution_time: Duration::from_millis(0),
                last_conflict_time: None,
            })),
            conflict_sender,
            conflict_receiver: Arc::new(RwLock::new(Some(conflict_receiver))),
        }
    }

    /// Register a resolution strategy
    pub async fn register_strategy(
        &self,
        strategy: ResolutionStrategy,
    ) -> Result<(), ConflictResolutionError> {
        let mut strategies = self.resolution_strategies.write().await;
        strategies.insert(strategy.strategy_id.clone(), strategy);
        Ok(())
    }

    /// Add an operation to the history and check for conflicts
    pub async fn add_operation(
        &self,
        operation: EditOperation,
    ) -> Result<Vec<Conflict>, ConflictResolutionError> {
        let mut history = self.operation_history.write().await;
        let mut conflicts = self.conflicts.write().await;
        let mut stats = self.stats.write().await;

        // Check for conflicts with existing operations
        let mut new_conflicts = Vec::new();

        for existing_op in history.iter() {
            if let Some(conflict) = self.detect_conflict(&operation, existing_op).await {
                let conflict_id = format!(
                    "conflict_{}_{}",
                    operation.operation_id, existing_op.operation_id
                );

                let mut conflict = conflict;
                conflict.conflict_id = conflict_id.clone();

                conflicts.insert(conflict_id.clone(), conflict.clone());
                new_conflicts.push(conflict.clone());

                // Update stats
                stats.total_conflicts += 1;
                stats.unresolved_conflicts += 1;
                stats.last_conflict_time = Some(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                );

                // Update conflict type stats
                let type_key = format!("{:?}", conflict.conflict_type);
                *stats.conflicts_by_type.entry(type_key).or_insert(0) += 1;

                // Update severity stats
                let severity_key = format!("{:?}", conflict.severity);
                *stats.conflicts_by_severity.entry(severity_key).or_insert(0) += 1;

                // Send conflict for processing
                if let Err(_) = self.conflict_sender.send(conflict) {
                    // Receiver dropped, ignore
                }
            }
        }

        // Add operation to history
        history.push_back(operation);

        // Maintain history limit
        while history.len() > 1000 {
            history.pop_front();
        }

        Ok(new_conflicts)
    }

    /// Resolve a conflict using a specific strategy
    pub async fn resolve_conflict(
        &self,
        conflict_id: &str,
        strategy: ResolutionStrategy,
    ) -> Result<ConflictResolution, ConflictResolutionError> {
        let mut conflicts = self.conflicts.write().await;
        let mut stats = self.stats.write().await;

        let conflict = conflicts.get_mut(conflict_id).ok_or_else(|| {
            ConflictResolutionError::ConflictNotFound {
                conflict_id: conflict_id.to_string(),
            }
        })?;

        let start_time = Instant::now();

        // Apply resolution strategy
        let resolution = match strategy.name.as_str() {
            "LastWriteWins" => {
                // Get the later operation
                let op1_time = self
                    .get_operation_timestamp(&conflict.operation1_id)
                    .await?;
                let op2_time = self
                    .get_operation_timestamp(&conflict.operation2_id)
                    .await?;

                if op1_time > op2_time {
                    ConflictResolution::LastWriteWins
                } else {
                    ConflictResolution::FirstWriteWins
                }
            }
            "FirstWriteWins" => ConflictResolution::FirstWriteWins,
            "UserPriority" => {
                let user_id = strategy
                    .parameters
                    .get("user_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                ConflictResolution::UserPriority { user_id }
            }
            "Manual" => ConflictResolution::Manual,
            "OperationalTransform" => {
                // Perform operational transformation
                let transformed_ops = self
                    .perform_operational_transform(&conflict.operation1_id, &conflict.operation2_id)
                    .await?;
                ConflictResolution::OperationalTransform { transformed_ops }
            }
            _ => ConflictResolution::Custom {
                strategy: strategy.name.clone(),
                parameters: strategy.parameters.clone(),
            },
        };

        // Update conflict with resolution
        conflict.resolution = Some(resolution.clone());

        // Update stats
        stats.resolved_conflicts += 1;
        stats.unresolved_conflicts = stats.unresolved_conflicts.saturating_sub(1);

        let resolution_time = start_time.elapsed();
        stats.average_resolution_time = resolution_time;

        Ok(resolution)
    }

    /// Perform operational transformation on two operations
    pub async fn perform_operational_transform(
        &self,
        op1_id: &str,
        op2_id: &str,
    ) -> Result<Vec<EditOperation>, ConflictResolutionError> {
        let history = self.operation_history.read().await;

        let op1 = history
            .iter()
            .find(|op| op.operation_id == op1_id)
            .ok_or_else(|| ConflictResolutionError::OperationNotFound {
                operation_id: op1_id.to_string(),
            })?;

        let op2 = history
            .iter()
            .find(|op| op.operation_id == op2_id)
            .ok_or_else(|| ConflictResolutionError::OperationNotFound {
                operation_id: op2_id.to_string(),
            })?;

        // Simple operational transformation logic
        // In a real implementation, this would be much more sophisticated
        let mut transformed_ops = Vec::new();

        match (&op1.operation_type, &op2.operation_type) {
            (EditOperationType::Create, EditOperationType::Create) => {
                // Both create operations - transform the second one
                let mut transformed_op2 = op2.clone();
                transformed_op2.element_id = Some(format!(
                    "{}_transformed",
                    op2.element_id.as_deref().unwrap_or("element")
                ));
                transformed_ops.push(transformed_op2);
            }
            (EditOperationType::Update, EditOperationType::Update) => {
                // Both update operations - merge the changes
                let mut merged_op = op1.clone();
                merged_op.timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                transformed_ops.push(merged_op);
            }
            (EditOperationType::Delete, EditOperationType::Update) => {
                // Delete vs Update - the delete wins
                transformed_ops.push(op1.clone());
            }
            (EditOperationType::Update, EditOperationType::Delete) => {
                // Update vs Delete - the delete wins
                transformed_ops.push(op2.clone());
            }
            _ => {
                // Default: keep both operations
                transformed_ops.push(op1.clone());
                transformed_ops.push(op2.clone());
            }
        }

        Ok(transformed_ops)
    }

    /// Detect conflicts between two operations
    async fn detect_conflict(&self, op1: &EditOperation, op2: &EditOperation) -> Option<Conflict> {
        // Skip if same operation
        if op1.operation_id == op2.operation_id {
            return None;
        }

        // Skip if same user (no self-conflict)
        if op1.user_id == op2.user_id {
            return None;
        }

        // Check for element conflicts
        if let (Some(id1), Some(id2)) = (&op1.element_id, &op2.element_id) {
            if id1 == id2 {
                let conflict_type = match (&op1.operation_type, &op2.operation_type) {
                    (EditOperationType::Create, EditOperationType::Create) => {
                        ConflictType::ElementCreation
                    }
                    (EditOperationType::Delete, EditOperationType::Delete) => {
                        ConflictType::ElementDeletion
                    }
                    (EditOperationType::Update, EditOperationType::Update) => {
                        ConflictType::ElementModification
                    }
                    (EditOperationType::StyleChange, EditOperationType::StyleChange) => {
                        ConflictType::StyleConflict
                    }
                    _ => ConflictType::ElementModification,
                };

                let severity = if op1.timestamp.abs_diff(op2.timestamp) < 5 {
                    ConflictSeverity::High
                } else {
                    ConflictSeverity::Medium
                };

                return Some(Conflict {
                    conflict_id: String::new(), // Will be set by caller
                    operation1_id: op1.operation_id.clone(),
                    operation2_id: op2.operation_id.clone(),
                    conflict_type,
                    severity,
                    timestamp: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    resolution: None,
                    metadata: HashMap::new(),
                });
            }
        }

        None
    }

    /// Get operation timestamp
    async fn get_operation_timestamp(
        &self,
        operation_id: &str,
    ) -> Result<u64, ConflictResolutionError> {
        let history = self.operation_history.read().await;
        let operation = history
            .iter()
            .find(|op| op.operation_id == operation_id)
            .ok_or_else(|| ConflictResolutionError::OperationNotFound {
                operation_id: operation_id.to_string(),
            })?;
        Ok(operation.timestamp)
    }

    /// Get all conflicts
    pub async fn get_conflicts(&self) -> Vec<Conflict> {
        let conflicts = self.conflicts.read().await;
        conflicts.values().cloned().collect()
    }

    /// Get unresolved conflicts
    pub async fn get_unresolved_conflicts(&self) -> Vec<Conflict> {
        let conflicts = self.conflicts.read().await;
        conflicts
            .values()
            .filter(|conflict| conflict.resolution.is_none())
            .cloned()
            .collect()
    }

    /// Get conflict resolution statistics
    pub async fn get_stats(&self) -> ConflictStats {
        self.stats.read().await.clone()
    }

    /// Start processing conflicts
    pub async fn start_processing(&self) {
        let receiver = self.conflict_receiver.write().await.take();
        if let Some(mut receiver) = receiver {
            let conflicts = self.conflicts.clone();
            let stats = self.stats.clone();

            tokio::spawn(async move {
                while let Some(conflict) = receiver.recv().await {
                    // Process conflict (in a real implementation, this would apply auto-resolution)
                    let start_time = Instant::now();

                    // Here you would implement automatic conflict resolution
                    // For now, we'll just log the conflict
                    println!("Processing conflict: {:?}", conflict.conflict_type);

                    // Update processing stats
                    let mut stats_guard = stats.write().await;
                    let processing_time = start_time.elapsed();
                    stats_guard.average_resolution_time = processing_time;
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn create_test_operation(
        operation_id: &str,
        user_id: &str,
        operation_type: EditOperationType,
        element_id: Option<&str>,
    ) -> EditOperation {
        EditOperation {
            operation_id: operation_id.to_string(),
            user_id: user_id.to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            operation_type,
            element_id: element_id.map(|s| s.to_string()),
            changes: ElementChanges::default(),
            dependencies: vec![],
        }
    }

    fn create_test_strategy(strategy_id: &str, name: &str) -> ResolutionStrategy {
        ResolutionStrategy {
            strategy_id: strategy_id.to_string(),
            name: name.to_string(),
            description: format!("Test strategy: {}", name),
            conflict_types: vec![
                ConflictType::ElementModification,
                ConflictType::ElementDeletion,
                ConflictType::ElementCreation,
            ],
            auto_resolve: false,
            parameters: HashMap::new(),
        }
    }

    #[tokio::test]
    async fn test_register_strategy() {
        let manager = ConflictResolutionManager::new();
        let strategy = create_test_strategy("strategy1", "LastWriteWins");

        let result = manager.register_strategy(strategy.clone()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_operation_no_conflict() {
        let manager = ConflictResolutionManager::new();
        let operation =
            create_test_operation("op1", "user1", EditOperationType::Create, Some("element1"));

        let conflicts = manager.add_operation(operation).await.unwrap();
        assert_eq!(conflicts.len(), 0);
    }

    #[tokio::test]
    async fn test_add_operation_with_conflict() {
        let manager = ConflictResolutionManager::new();

        // Add first operation
        let op1 =
            create_test_operation("op1", "user1", EditOperationType::Create, Some("element1"));
        manager.add_operation(op1).await.unwrap();

        // Add conflicting operation
        let op2 =
            create_test_operation("op2", "user2", EditOperationType::Create, Some("element1"));
        let conflicts = manager.add_operation(op2).await.unwrap();

        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].conflict_type, ConflictType::ElementCreation);
    }

    #[tokio::test]
    async fn test_resolve_conflict_last_write_wins() {
        let manager = ConflictResolutionManager::new();

        // Register strategy
        let strategy = create_test_strategy("strategy1", "LastWriteWins");
        manager.register_strategy(strategy.clone()).await.unwrap();

        // Add conflicting operations
        let op1 =
            create_test_operation("op1", "user1", EditOperationType::Create, Some("element1"));
        manager.add_operation(op1).await.unwrap();

        let op2 =
            create_test_operation("op2", "user2", EditOperationType::Create, Some("element1"));
        let conflicts = manager.add_operation(op2).await.unwrap();

        // Resolve conflict
        let resolution = manager
            .resolve_conflict(&conflicts[0].conflict_id, strategy)
            .await
            .unwrap();
        assert!(matches!(
            resolution,
            ConflictResolution::LastWriteWins | ConflictResolution::FirstWriteWins
        ));
    }

    #[tokio::test]
    async fn test_resolve_conflict_manual() {
        let manager = ConflictResolutionManager::new();

        // Register strategy
        let strategy = create_test_strategy("strategy1", "Manual");
        manager.register_strategy(strategy.clone()).await.unwrap();

        // Add conflicting operations
        let op1 =
            create_test_operation("op1", "user1", EditOperationType::Create, Some("element1"));
        manager.add_operation(op1).await.unwrap();

        let op2 =
            create_test_operation("op2", "user2", EditOperationType::Create, Some("element1"));
        let conflicts = manager.add_operation(op2).await.unwrap();

        // Resolve conflict
        let resolution = manager
            .resolve_conflict(&conflicts[0].conflict_id, strategy)
            .await
            .unwrap();
        assert!(matches!(resolution, ConflictResolution::Manual));
    }

    #[tokio::test]
    async fn test_operational_transform() {
        let manager = ConflictResolutionManager::new();

        // Add operations
        let op1 =
            create_test_operation("op1", "user1", EditOperationType::Create, Some("element1"));
        manager.add_operation(op1).await.unwrap();

        let op2 =
            create_test_operation("op2", "user2", EditOperationType::Create, Some("element1"));
        manager.add_operation(op2).await.unwrap();

        // Perform transformation
        let transformed_ops = manager
            .perform_operational_transform("op1", "op2")
            .await
            .unwrap();
        assert!(!transformed_ops.is_empty());
    }

    #[tokio::test]
    async fn test_conflict_detection_different_elements() {
        let manager = ConflictResolutionManager::new();

        // Add operations for different elements
        let op1 =
            create_test_operation("op1", "user1", EditOperationType::Create, Some("element1"));
        manager.add_operation(op1).await.unwrap();

        let op2 =
            create_test_operation("op2", "user2", EditOperationType::Create, Some("element2"));
        let conflicts = manager.add_operation(op2).await.unwrap();

        assert_eq!(conflicts.len(), 0);
    }

    #[tokio::test]
    async fn test_conflict_detection_same_user() {
        let manager = ConflictResolutionManager::new();

        // Add operations from same user
        let op1 =
            create_test_operation("op1", "user1", EditOperationType::Create, Some("element1"));
        manager.add_operation(op1).await.unwrap();

        let op2 =
            create_test_operation("op2", "user1", EditOperationType::Create, Some("element1"));
        let conflicts = manager.add_operation(op2).await.unwrap();

        assert_eq!(conflicts.len(), 0);
    }

    #[tokio::test]
    async fn test_conflict_stats() {
        let manager = ConflictResolutionManager::new();

        // Add conflicting operations
        let op1 =
            create_test_operation("op1", "user1", EditOperationType::Create, Some("element1"));
        manager.add_operation(op1).await.unwrap();

        let op2 =
            create_test_operation("op2", "user2", EditOperationType::Create, Some("element1"));
        manager.add_operation(op2).await.unwrap();

        let stats = manager.get_stats().await;
        assert_eq!(stats.total_conflicts, 1);
        assert_eq!(stats.unresolved_conflicts, 1);
    }

    #[tokio::test]
    async fn test_get_unresolved_conflicts() {
        let manager = ConflictResolutionManager::new();

        // Add conflicting operations
        let op1 =
            create_test_operation("op1", "user1", EditOperationType::Create, Some("element1"));
        manager.add_operation(op1).await.unwrap();

        let op2 =
            create_test_operation("op2", "user2", EditOperationType::Create, Some("element1"));
        manager.add_operation(op2).await.unwrap();

        let unresolved = manager.get_unresolved_conflicts().await;
        assert_eq!(unresolved.len(), 1);
    }

    #[tokio::test]
    async fn test_operation_not_found() {
        let manager = ConflictResolutionManager::new();

        let result = manager
            .perform_operational_transform("nonexistent1", "nonexistent2")
            .await;
        assert!(matches!(
            result,
            Err(ConflictResolutionError::OperationNotFound { .. })
        ));
    }

    #[tokio::test]
    async fn test_conflict_not_found() {
        let manager = ConflictResolutionManager::new();
        let strategy = create_test_strategy("strategy1", "Manual");

        let result = manager.resolve_conflict("nonexistent", strategy).await;
        assert!(matches!(
            result,
            Err(ConflictResolutionError::ConflictNotFound { .. })
        ));
    }
}
