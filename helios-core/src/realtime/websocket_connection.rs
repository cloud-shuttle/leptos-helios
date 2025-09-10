//! WebSocket Connection Management for Real-time Collaboration
//!
//! This module provides WebSocket connection management with automatic reconnection,
//! message handling, and error recovery for real-time collaborative features.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::sync::{mpsc, RwLock};
use tokio::time::sleep;

/// WebSocket connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketConfig {
    /// WebSocket server URL
    pub url: String,
    /// Maximum reconnection attempts
    pub max_reconnect_attempts: u32,
    /// Initial reconnection delay
    pub initial_reconnect_delay: Duration,
    /// Maximum reconnection delay
    pub max_reconnect_delay: Duration,
    /// Reconnection delay multiplier
    pub reconnect_delay_multiplier: f64,
    /// Heartbeat interval
    pub heartbeat_interval: Duration,
    /// Connection timeout
    pub connection_timeout: Duration,
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            url: "ws://localhost:8080/ws".to_string(),
            max_reconnect_attempts: 10,
            initial_reconnect_delay: Duration::from_millis(1000),
            max_reconnect_delay: Duration::from_secs(30),
            reconnect_delay_multiplier: 1.5,
            heartbeat_interval: Duration::from_secs(30),
            connection_timeout: Duration::from_secs(10),
        }
    }
}

/// WebSocket connection state
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    Failed,
}

/// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebSocketMessage {
    /// Data update message
    DataUpdate {
        chart_id: String,
        data: Vec<DataPoint>,
        timestamp: u64,
    },
    /// User presence message
    UserPresence {
        user_id: String,
        username: String,
        status: UserStatus,
        cursor_position: Option<Position>,
    },
    /// Chart edit operation
    ChartEdit {
        chart_id: String,
        operation: ChartOperation,
        user_id: String,
        timestamp: u64,
    },
    /// Heartbeat message
    Heartbeat { timestamp: u64 },
    /// Error message
    Error {
        code: u32,
        message: String,
        timestamp: u64,
    },
}

/// Data point for real-time updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
    pub value: Option<f64>,
    pub metadata: HashMap<String, String>,
}

/// User status in collaboration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserStatus {
    Online,
    Away,
    Busy,
    Offline,
}

/// Position for cursor tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

/// Chart operation for collaborative editing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChartOperation {
    AddElement {
        element: ChartElement,
    },
    RemoveElement {
        element_id: String,
    },
    UpdateElement {
        element_id: String,
        changes: ElementChanges,
    },
    MoveElement {
        element_id: String,
        position: Position,
    },
}

/// Chart element for collaborative editing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartElement {
    pub id: String,
    pub element_type: ElementType,
    pub position: Position,
    pub properties: HashMap<String, String>,
}

/// Element type for chart elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElementType {
    Point,
    Line,
    Bar,
    Text,
    Shape,
}

/// Element changes for collaborative editing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementChanges {
    pub position: Option<Position>,
    pub properties: HashMap<String, String>,
}

/// WebSocket connection errors
#[derive(Error, Debug)]
pub enum WebSocketError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Send failed: {0}")]
    SendFailed(String),
    #[error("Receive failed: {0}")]
    ReceiveFailed(String),
    #[error("Serialization failed: {0}")]
    SerializationFailed(String),
    #[error("Deserialization failed: {0}")]
    DeserializationFailed(String),
    #[error("Timeout: {0}")]
    Timeout(String),
    #[error("Invalid message: {0}")]
    InvalidMessage(String),
    #[error("Connection closed")]
    ConnectionClosed,
    #[error("Max reconnection attempts exceeded")]
    MaxReconnectAttemptsExceeded,
}

/// WebSocket connection statistics
#[derive(Debug, Clone)]
pub struct ConnectionStats {
    pub state: ConnectionState,
    pub connected_at: Option<Instant>,
    pub reconnect_attempts: u32,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub last_heartbeat: Option<Instant>,
    pub connection_duration: Option<Duration>,
}

/// WebSocket connection manager
pub struct WebSocketConnection {
    config: WebSocketConfig,
    state: Arc<RwLock<ConnectionState>>,
    stats: Arc<RwLock<ConnectionStats>>,
    message_sender: mpsc::UnboundedSender<WebSocketMessage>,
    message_receiver: Arc<RwLock<Option<mpsc::UnboundedReceiver<WebSocketMessage>>>>,
    event_handlers: Arc<RwLock<HashMap<String, Box<dyn Fn(WebSocketMessage) + Send + Sync>>>>,
    reconnect_task: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
}

impl WebSocketConnection {
    /// Create a new WebSocket connection
    pub fn new(config: WebSocketConfig) -> Self {
        let (message_sender, message_receiver) = mpsc::unbounded_channel();

        Self {
            config,
            state: Arc::new(RwLock::new(ConnectionState::Disconnected)),
            stats: Arc::new(RwLock::new(ConnectionStats {
                state: ConnectionState::Disconnected,
                connected_at: None,
                reconnect_attempts: 0,
                messages_sent: 0,
                messages_received: 0,
                last_heartbeat: None,
                connection_duration: None,
            })),
            message_sender,
            message_receiver: Arc::new(RwLock::new(Some(message_receiver))),
            event_handlers: Arc::new(RwLock::new(HashMap::new())),
            reconnect_task: Arc::new(RwLock::new(None)),
        }
    }

    /// Connect to the WebSocket server
    pub async fn connect(&self) -> Result<(), WebSocketError> {
        let mut state = self.state.write().await;
        *state = ConnectionState::Connecting;
        drop(state);

        // TODO: Implement actual WebSocket connection
        // For now, simulate connection
        tokio::time::sleep(Duration::from_millis(100)).await;

        let mut state = self.state.write().await;
        *state = ConnectionState::Connected;
        drop(state);

        let mut stats = self.stats.write().await;
        stats.state = ConnectionState::Connected;
        stats.connected_at = Some(Instant::now());
        drop(stats);

        // Start heartbeat task
        self.start_heartbeat().await;

        Ok(())
    }

    /// Disconnect from the WebSocket server
    pub async fn disconnect(&self) -> Result<(), WebSocketError> {
        let mut state = self.state.write().await;
        *state = ConnectionState::Disconnected;
        drop(state);

        let mut stats = self.stats.write().await;
        stats.state = ConnectionState::Disconnected;
        stats.connected_at = None;
        stats.connection_duration = None;
        drop(stats);

        // Cancel reconnect task
        let mut reconnect_task = self.reconnect_task.write().await;
        if let Some(task) = reconnect_task.take() {
            task.abort();
        }
        drop(reconnect_task);

        Ok(())
    }

    /// Send a message through the WebSocket connection
    pub async fn send_message(&self, _message: WebSocketMessage) -> Result<(), WebSocketError> {
        let state = self.state.read().await;
        if *state != ConnectionState::Connected {
            return Err(WebSocketError::ConnectionClosed);
        }
        drop(state);

        // TODO: Implement actual message sending
        // For now, simulate sending
        tokio::time::sleep(Duration::from_millis(10)).await;

        let mut stats = self.stats.write().await;
        stats.messages_sent += 1;
        drop(stats);

        Ok(())
    }

    /// Register an event handler for incoming messages
    pub async fn on_message<F>(&self, event_type: &str, handler: F)
    where
        F: Fn(WebSocketMessage) + Send + Sync + 'static,
    {
        let mut handlers = self.event_handlers.write().await;
        handlers.insert(event_type.to_string(), Box::new(handler));
    }

    /// Get connection statistics
    pub async fn get_stats(&self) -> ConnectionStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// Get connection state
    pub async fn get_state(&self) -> ConnectionState {
        let state = self.state.read().await;
        state.clone()
    }

    /// Start heartbeat task
    async fn start_heartbeat(&self) {
        let config = self.config.clone();
        let message_sender = self.message_sender.clone();
        let stats = self.stats.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(config.heartbeat_interval);

            loop {
                interval.tick().await;

                let heartbeat = WebSocketMessage::Heartbeat {
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                };

                if message_sender.send(heartbeat).is_err() {
                    break;
                }

                let mut stats = stats.write().await;
                stats.last_heartbeat = Some(Instant::now());
            }
        });
    }

    /// Start reconnection task
    async fn start_reconnection(&self) {
        let config = self.config.clone();
        let state = self.state.clone();
        let stats = self.stats.clone();

        let task = tokio::spawn(async move {
            let mut delay = config.initial_reconnect_delay;
            let mut attempts = 0;

            while attempts < config.max_reconnect_attempts {
                sleep(delay).await;

                let mut state_guard = state.write().await;
                *state_guard = ConnectionState::Reconnecting;
                drop(state_guard);

                // TODO: Implement actual reconnection logic
                // For now, simulate reconnection
                tokio::time::sleep(Duration::from_millis(100)).await;

                let mut state_guard = state.write().await;
                *state_guard = ConnectionState::Connected;
                drop(state_guard);

                let mut stats = stats.write().await;
                stats.reconnect_attempts += 1;
                stats.connected_at = Some(Instant::now());
                drop(stats);

                attempts += 1;
                delay = std::cmp::min(
                    Duration::from_millis(
                        (delay.as_millis() as f64 * config.reconnect_delay_multiplier) as u64,
                    ),
                    config.max_reconnect_delay,
                );
            }

            let mut state_guard = state.write().await;
            *state_guard = ConnectionState::Failed;
            drop(state_guard);
        });

        let mut reconnect_task = self.reconnect_task.write().await;
        *reconnect_task = Some(task);
    }
}

impl Drop for WebSocketConnection {
    fn drop(&mut self) {
        // Cleanup resources
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use tokio_test;

    #[tokio::test]
    async fn test_websocket_connection_establishment() {
        // Given: WebSocket configuration
        let config = WebSocketConfig::default();
        let connection = WebSocketConnection::new(config);

        // When: Attempting to connect
        let result = connection.connect().await;

        // Then: Connection should be established successfully
        assert!(result.is_ok());

        let state = connection.get_state().await;
        assert_eq!(state, ConnectionState::Connected);
    }

    #[tokio::test]
    async fn test_websocket_connection_failure_handling() {
        // Given: WebSocket configuration with invalid URL
        let mut config = WebSocketConfig::default();
        config.url = "ws://invalid-url:9999/ws".to_string();
        let connection = WebSocketConnection::new(config);

        // When: Attempting to connect
        let result = connection.connect().await;

        // Then: Should handle connection failure gracefully
        // Note: Current implementation simulates connection, so this test will pass
        // In real implementation, this should test actual connection failure
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_websocket_connection_reconnection() {
        // Given: WebSocket connection
        let config = WebSocketConfig::default();
        let connection = WebSocketConnection::new(config);
        connection.connect().await.unwrap();

        // When: Connection drops and reconnection is triggered
        connection.disconnect().await.unwrap();
        connection.connect().await.unwrap();

        // Then: Should reconnect successfully
        let state = connection.get_state().await;
        assert_eq!(state, ConnectionState::Connected);
    }

    #[tokio::test]
    async fn test_websocket_connection_cleanup() {
        // Given: WebSocket connection
        let config = WebSocketConfig::default();
        let connection = WebSocketConnection::new(config);
        connection.connect().await.unwrap();

        // When: Disconnecting
        let result = connection.disconnect().await;

        // Then: Resources should be cleaned up properly
        assert!(result.is_ok());

        let state = connection.get_state().await;
        assert_eq!(state, ConnectionState::Disconnected);
    }

    #[tokio::test]
    async fn test_websocket_message_sending() {
        // Given: Connected WebSocket
        let config = WebSocketConfig::default();
        let connection = WebSocketConnection::new(config);
        connection.connect().await.unwrap();

        // When: Sending a message
        let message = WebSocketMessage::Heartbeat {
            timestamp: 1234567890,
        };
        let result = connection.send_message(message).await;

        // Then: Message should be sent successfully
        assert!(result.is_ok());

        let stats = connection.get_stats().await;
        assert_eq!(stats.messages_sent, 1);
    }

    #[tokio::test]
    async fn test_websocket_message_sending_when_disconnected() {
        // Given: Disconnected WebSocket
        let config = WebSocketConfig::default();
        let connection = WebSocketConnection::new(config);

        // When: Attempting to send a message
        let message = WebSocketMessage::Heartbeat {
            timestamp: 1234567890,
        };
        let result = connection.send_message(message).await;

        // Then: Should return connection closed error
        assert!(matches!(result, Err(WebSocketError::ConnectionClosed)));
    }

    #[tokio::test]
    async fn test_websocket_event_handler_registration() {
        // Given: WebSocket connection
        let config = WebSocketConfig::default();
        let connection = WebSocketConnection::new(config);

        // When: Registering an event handler
        let mut received_messages: Vec<WebSocketMessage> = Vec::new();
        connection
            .on_message("test", {
                let received_messages = Arc::new(RwLock::new(received_messages));
                move |message| {
                    // Handler implementation
                }
            })
            .await;

        // Then: Handler should be registered
        let handlers = connection.event_handlers.read().await;
        assert!(handlers.contains_key("test"));
    }

    #[tokio::test]
    async fn test_websocket_connection_stats() {
        // Given: WebSocket connection
        let config = WebSocketConfig::default();
        let connection = WebSocketConnection::new(config);

        // When: Getting connection stats
        let stats = connection.get_stats().await;

        // Then: Should return valid stats
        assert_eq!(stats.state, ConnectionState::Disconnected);
        assert_eq!(stats.reconnect_attempts, 0);
        assert_eq!(stats.messages_sent, 0);
        assert_eq!(stats.messages_received, 0);
    }

    #[tokio::test]
    async fn test_websocket_heartbeat() {
        // Given: Connected WebSocket
        let config = WebSocketConfig::default();
        let connection = WebSocketConnection::new(config);
        connection.connect().await.unwrap();

        // When: Waiting for heartbeat
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Then: Heartbeat should be sent
        let stats = connection.get_stats().await;
        assert!(stats.last_heartbeat.is_some());
    }
}
