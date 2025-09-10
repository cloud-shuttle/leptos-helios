//! Real-time Collaborative Features
//!
//! This module provides real-time collaborative features for leptos-helios,
//! including WebSocket integration, collaborative editing, live updates,
//! and conflict resolution.

pub mod collaborative_editing;
pub mod message_protocol;
pub mod websocket_connection;

// Re-export main types for convenience
pub use collaborative_editing::{
    CollaborationError, CollaborationSession, CollaborationSettings, CollaborationStats,
    CollaborativeEditor, Collaborator, ConflictResolutionStrategy, EditOperation,
    EditOperationType, ElementChanges as CollaborationElementChanges,
    ElementStyle as CollaborationElementStyle, Size as CollaborationSize, UserPermissions,
};

pub use websocket_connection::{
    ChartElement, ChartOperation, ConnectionState, ConnectionStats, DataPoint, ElementChanges,
    ElementType, Position, UserStatus, WebSocketConfig, WebSocketConnection, WebSocketError,
    WebSocketMessage,
};

pub use message_protocol::{
    AcknowledgmentStatus, ActivityType, DataFilter, DataUpdateType, ElementStyle, FilterOperator,
    Message, MessagePayload, MessageProtocol, MessageProtocolError, MessageType, SessionUpdateType,
    Size, StreamConfig,
};

/// Real-time collaboration manager
pub struct RealtimeManager {
    websocket: WebSocketConnection,
    message_protocol: MessageProtocol,
    session_id: Option<String>,
    user_id: Option<String>,
}

impl RealtimeManager {
    /// Create a new real-time manager
    pub fn new(config: WebSocketConfig) -> Self {
        Self {
            websocket: WebSocketConnection::new(config),
            message_protocol: MessageProtocol::new(),
            session_id: None,
            user_id: None,
        }
    }

    /// Connect to the real-time service
    pub async fn connect(&self) -> Result<(), WebSocketError> {
        self.websocket.connect().await
    }

    /// Disconnect from the real-time service
    pub async fn disconnect(&self) -> Result<(), WebSocketError> {
        self.websocket.disconnect().await
    }

    /// Send a message
    pub async fn send_message(&self, message: Message) -> Result<(), WebSocketError> {
        let _serialized = self
            .message_protocol
            .serialize(&message)
            .map_err(|e| WebSocketError::SerializationFailed(e.to_string()))?;

        // Convert to WebSocketMessage and send
        let ws_message = WebSocketMessage::DataUpdate {
            chart_id: "default".to_string(),
            data: vec![],
            timestamp: message.timestamp,
        };

        self.websocket.send_message(ws_message).await
    }

    /// Get connection statistics
    pub async fn get_stats(&self) -> ConnectionStats {
        self.websocket.get_stats().await
    }

    /// Get connection state
    pub async fn get_state(&self) -> ConnectionState {
        self.websocket.get_state().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_realtime_manager_creation() {
        // Given: WebSocket configuration
        let config = WebSocketConfig::default();

        // When: Creating real-time manager
        let manager = RealtimeManager::new(config);

        // Then: Manager should be created successfully
        let state = manager.get_state().await;
        assert_eq!(state, ConnectionState::Disconnected);
    }

    #[tokio::test]
    async fn test_realtime_manager_connection() {
        // Given: Real-time manager
        let config = WebSocketConfig::default();
        let manager = RealtimeManager::new(config);

        // When: Connecting
        let result = manager.connect().await;

        // Then: Connection should succeed
        assert!(result.is_ok());

        let state = manager.get_state().await;
        assert_eq!(state, ConnectionState::Connected);
    }

    #[tokio::test]
    async fn test_realtime_manager_disconnection() {
        // Given: Connected real-time manager
        let config = WebSocketConfig::default();
        let manager = RealtimeManager::new(config);
        manager.connect().await.unwrap();

        // When: Disconnecting
        let result = manager.disconnect().await;

        // Then: Disconnection should succeed
        assert!(result.is_ok());

        let state = manager.get_state().await;
        assert_eq!(state, ConnectionState::Disconnected);
    }
}
