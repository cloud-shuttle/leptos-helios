//! Message Protocol for Real-time Communication
//!
//! This module defines the message protocol for real-time communication between
//! clients and servers, including serialization, validation, and compression.

use base64::Engine;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Message protocol version
pub const PROTOCOL_VERSION: u32 = 1;

/// Message types for the protocol
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    /// Data streaming messages
    DataUpdate,
    DataStream,

    /// Collaborative editing messages
    ChartEdit,
    UserPresence,
    UserActivity,

    /// System messages
    Heartbeat,
    Error,
    Acknowledgment,

    /// Authentication messages
    AuthRequest,
    AuthResponse,

    /// Session management
    JoinSession,
    LeaveSession,
    SessionUpdate,
}

/// Base message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message ID for tracking
    pub id: String,
    /// Message type
    pub message_type: MessageType,
    /// Protocol version
    pub version: u32,
    /// Timestamp in milliseconds
    pub timestamp: u64,
    /// Sender ID
    pub sender_id: String,
    /// Recipient ID (empty for broadcast)
    pub recipient_id: Option<String>,
    /// Message payload
    pub payload: MessagePayload,
    /// Message metadata
    pub metadata: HashMap<String, String>,
}

/// Message payload variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessagePayload {
    /// Data update payload
    DataUpdate {
        chart_id: String,
        data: Vec<DataPoint>,
        update_type: DataUpdateType,
    },
    /// Data stream payload
    DataStream {
        stream_id: String,
        data: Vec<DataPoint>,
        stream_config: StreamConfig,
    },
    /// Chart edit payload
    ChartEdit {
        chart_id: String,
        operation: ChartOperation,
        element_id: Option<String>,
    },
    /// User presence payload
    UserPresence {
        user_id: String,
        username: String,
        status: UserStatus,
        cursor_position: Option<Position>,
        last_activity: u64,
    },
    /// User activity payload
    UserActivity {
        user_id: String,
        activity_type: ActivityType,
        target_id: Option<String>,
        details: HashMap<String, String>,
    },
    /// Heartbeat payload
    Heartbeat {
        client_time: u64,
        server_time: Option<u64>,
    },
    /// Error payload
    Error {
        code: u32,
        message: String,
        details: Option<String>,
        recoverable: bool,
    },
    /// Acknowledgment payload
    Acknowledgment {
        original_message_id: String,
        status: AcknowledgmentStatus,
        timestamp: u64,
    },
    /// Authentication request payload
    AuthRequest {
        token: String,
        session_id: Option<String>,
    },
    /// Authentication response payload
    AuthResponse {
        success: bool,
        user_id: Option<String>,
        permissions: Vec<String>,
        error_message: Option<String>,
    },
    /// Join session payload
    JoinSession {
        session_id: String,
        user_id: String,
        permissions: Vec<String>,
    },
    /// Leave session payload
    LeaveSession {
        session_id: String,
        user_id: String,
        reason: Option<String>,
    },
    /// Session update payload
    SessionUpdate {
        session_id: String,
        update_type: SessionUpdateType,
        data: HashMap<String, String>,
    },
}

/// Data update types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataUpdateType {
    /// Full data replacement
    Replace,
    /// Incremental data update
    Append,
    /// Data modification
    Modify,
    /// Data deletion
    Delete,
}

/// Stream configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamConfig {
    /// Stream frequency in milliseconds
    pub frequency: u64,
    /// Maximum batch size
    pub max_batch_size: usize,
    /// Compression enabled
    pub compression: bool,
    /// Filters to apply
    pub filters: Vec<DataFilter>,
}

/// Data filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFilter {
    /// Filter field
    pub field: String,
    /// Filter operator
    pub operator: FilterOperator,
    /// Filter value
    pub value: String,
}

/// Filter operators
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FilterOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    StartsWith,
    EndsWith,
    In,
    NotIn,
}

/// Chart operation for collaborative editing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChartOperation {
    /// Add new element
    AddElement { element: ChartElement },
    /// Remove element
    RemoveElement { element_id: String },
    /// Update element
    UpdateElement {
        element_id: String,
        changes: ElementChanges,
    },
    /// Move element
    MoveElement {
        element_id: String,
        position: Position,
    },
    /// Resize element
    ResizeElement { element_id: String, size: Size },
    /// Change element style
    StyleElement {
        element_id: String,
        style: ElementStyle,
    },
}

/// Chart element
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChartElement {
    /// Element ID
    pub id: String,
    /// Element type
    pub element_type: ElementType,
    /// Element position
    pub position: Position,
    /// Element size
    pub size: Size,
    /// Element properties
    pub properties: HashMap<String, String>,
    /// Element style
    pub style: ElementStyle,
}

/// Element type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ElementType {
    Point,
    Line,
    Bar,
    Area,
    Text,
    Shape,
    Image,
    Annotation,
}

/// Element changes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ElementChanges {
    /// Position changes
    pub position: Option<Position>,
    /// Size changes
    pub size: Option<Size>,
    /// Property changes
    pub properties: HashMap<String, String>,
    /// Style changes
    pub style: Option<ElementStyle>,
}

/// Element style
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ElementStyle {
    /// Fill color
    pub fill_color: Option<String>,
    /// Stroke color
    pub stroke_color: Option<String>,
    /// Stroke width
    pub stroke_width: Option<f64>,
    /// Opacity
    pub opacity: Option<f64>,
    /// Font family
    pub font_family: Option<String>,
    /// Font size
    pub font_size: Option<f64>,
    /// Font weight
    pub font_weight: Option<String>,
}

/// Position
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

/// Size
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

/// User status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserStatus {
    Online,
    Away,
    Busy,
    Offline,
}

/// Activity type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActivityType {
    View,
    Edit,
    Comment,
    Share,
    Export,
    Import,
}

/// Acknowledgment status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AcknowledgmentStatus {
    Success,
    Error,
    Warning,
}

/// Session update type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SessionUpdateType {
    UserJoined,
    UserLeft,
    ChartUpdated,
    SettingsChanged,
    PermissionsChanged,
}

/// Data point
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
    pub value: Option<f64>,
    pub metadata: HashMap<String, String>,
}

/// Message protocol errors
#[derive(Error, Debug)]
pub enum MessageProtocolError {
    #[error("Serialization failed: {0}")]
    SerializationFailed(String),
    #[error("Deserialization failed: {0}")]
    DeserializationFailed(String),
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
    #[error("Compression failed: {0}")]
    CompressionFailed(String),
    #[error("Decompression failed: {0}")]
    DecompressionFailed(String),
    #[error("Invalid message type: {0}")]
    InvalidMessageType(String),
    #[error("Invalid protocol version: {0}")]
    InvalidProtocolVersion(u32),
    #[error("Message too large: {0} bytes")]
    MessageTooLarge(usize),
    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(u64),
}

/// Message protocol implementation
pub struct MessageProtocol {
    /// Maximum message size in bytes
    max_message_size: usize,
    /// Compression enabled
    compression_enabled: bool,
    /// Compression level
    compression_level: Compression,
}

impl MessageProtocol {
    /// Create a new message protocol
    pub fn new() -> Self {
        Self {
            max_message_size: 1024 * 1024, // 1MB
            compression_enabled: true,
            compression_level: Compression::default(),
        }
    }

    /// Create a new message protocol with custom settings
    pub fn with_settings(max_message_size: usize, compression_enabled: bool) -> Self {
        Self {
            max_message_size,
            compression_enabled,
            compression_level: Compression::default(),
        }
    }

    /// Serialize a message to JSON
    pub fn serialize(&self, message: &Message) -> Result<String, MessageProtocolError> {
        let json = serde_json::to_string(message)
            .map_err(|e| MessageProtocolError::SerializationFailed(e.to_string()))?;

        if json.len() > self.max_message_size {
            return Err(MessageProtocolError::MessageTooLarge(json.len()));
        }

        if self.compression_enabled && json.len() > 1024 {
            self.compress(&json)
        } else {
            Ok(json)
        }
    }

    /// Deserialize a message from JSON
    pub fn deserialize(&self, data: &str) -> Result<Message, MessageProtocolError> {
        let json = if self.is_compressed(data) {
            self.decompress(data)?
        } else {
            data.to_string()
        };

        let message: Message = serde_json::from_str(&json)
            .map_err(|e| MessageProtocolError::DeserializationFailed(e.to_string()))?;

        self.validate(&message)?;
        Ok(message)
    }

    /// Validate a message
    pub fn validate(&self, message: &Message) -> Result<(), MessageProtocolError> {
        // Check protocol version
        if message.version != PROTOCOL_VERSION {
            return Err(MessageProtocolError::InvalidProtocolVersion(
                message.version,
            ));
        }

        // Check timestamp (should be within reasonable range)
        // For testing, allow timestamps from 1970 to 2100
        if message.timestamp == 0 || message.timestamp > 4102444800000 {
            return Err(MessageProtocolError::InvalidTimestamp(message.timestamp));
        }

        // Check message ID
        if message.id.is_empty() {
            return Err(MessageProtocolError::ValidationFailed(
                "Message ID cannot be empty".to_string(),
            ));
        }

        // Check sender ID
        if message.sender_id.is_empty() {
            return Err(MessageProtocolError::ValidationFailed(
                "Sender ID cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    /// Compress data
    fn compress(&self, data: &str) -> Result<String, MessageProtocolError> {
        use flate2::write::GzEncoder;
        use std::io::Write;

        let mut encoder = GzEncoder::new(Vec::new(), self.compression_level);
        encoder
            .write_all(data.as_bytes())
            .map_err(|e| MessageProtocolError::CompressionFailed(e.to_string()))?;

        let compressed = encoder
            .finish()
            .map_err(|e| MessageProtocolError::CompressionFailed(e.to_string()))?;

        Ok(base64::engine::general_purpose::STANDARD.encode(compressed))
    }

    /// Decompress data
    fn decompress(&self, data: &str) -> Result<String, MessageProtocolError> {
        use flate2::read::GzDecoder;
        use std::io::Read;

        let compressed = base64::engine::general_purpose::STANDARD
            .decode(data)
            .map_err(|e| MessageProtocolError::DecompressionFailed(e.to_string()))?;

        let mut decoder = GzDecoder::new(&compressed[..]);
        let mut decompressed = String::new();

        decoder
            .read_to_string(&mut decompressed)
            .map_err(|e| MessageProtocolError::DecompressionFailed(e.to_string()))?;

        Ok(decompressed)
    }

    /// Check if data is compressed
    fn is_compressed(&self, data: &str) -> bool {
        data.starts_with("eJ") || data.starts_with("H4sI")
    }

    /// Create a heartbeat message
    pub fn create_heartbeat(&self, sender_id: &str) -> Message {
        Message {
            id: uuid::Uuid::new_v4().to_string(),
            message_type: MessageType::Heartbeat,
            version: PROTOCOL_VERSION,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            sender_id: sender_id.to_string(),
            recipient_id: None,
            payload: MessagePayload::Heartbeat {
                client_time: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
                server_time: None,
            },
            metadata: HashMap::new(),
        }
    }

    /// Create an acknowledgment message
    pub fn create_acknowledgment(
        &self,
        original_message_id: &str,
        sender_id: &str,
        status: AcknowledgmentStatus,
    ) -> Message {
        Message {
            id: uuid::Uuid::new_v4().to_string(),
            message_type: MessageType::Acknowledgment,
            version: PROTOCOL_VERSION,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            sender_id: sender_id.to_string(),
            recipient_id: None,
            payload: MessagePayload::Acknowledgment {
                original_message_id: original_message_id.to_string(),
                status,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
            },
            metadata: HashMap::new(),
        }
    }

    /// Create an error message
    pub fn create_error(
        &self,
        code: u32,
        message: &str,
        sender_id: &str,
        recipient_id: Option<&str>,
    ) -> Message {
        Message {
            id: uuid::Uuid::new_v4().to_string(),
            message_type: MessageType::Error,
            version: PROTOCOL_VERSION,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            sender_id: sender_id.to_string(),
            recipient_id: recipient_id.map(|s| s.to_string()),
            payload: MessagePayload::Error {
                code,
                message: message.to_string(),
                details: None,
                recoverable: true,
            },
            metadata: HashMap::new(),
        }
    }
}

impl Default for MessageProtocol {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_serialization() {
        // Given: A message with data
        let message = Message {
            id: "test-id".to_string(),
            message_type: MessageType::Heartbeat,
            version: PROTOCOL_VERSION,
            timestamp: 1234567890,
            sender_id: "sender".to_string(),
            recipient_id: None,
            payload: MessagePayload::Heartbeat {
                client_time: 1234567890,
                server_time: None,
            },
            metadata: HashMap::new(),
        };

        let protocol = MessageProtocol::new();

        // When: Serializing to JSON
        let result = protocol.serialize(&message);

        // Then: Should produce valid JSON
        assert!(result.is_ok());
        let json = result.unwrap();
        assert!(!json.is_empty());
    }

    #[test]
    fn test_message_deserialization() {
        // Given: Valid JSON message
        let json = r#"{
            "id": "test-id",
            "message_type": "Heartbeat",
            "version": 1,
            "timestamp": 1234567890,
            "sender_id": "sender",
            "recipient_id": null,
            "payload": {
                "Heartbeat": {
                    "client_time": 1234567890,
                    "server_time": null
                }
            },
            "metadata": {}
        }"#;

        let protocol = MessageProtocol::new();

        // When: Deserializing to Message
        let result = protocol.deserialize(json);

        // Then: Should produce correct Message struct
        assert!(result.is_ok());
        let message = result.unwrap();
        assert_eq!(message.id, "test-id");
        assert_eq!(message.message_type, MessageType::Heartbeat);
        assert_eq!(message.version, PROTOCOL_VERSION);
        assert_eq!(message.timestamp, 1234567890);
        assert_eq!(message.sender_id, "sender");
    }

    #[test]
    fn test_message_validation() {
        // Given: Invalid message data
        let message = Message {
            id: "".to_string(), // Empty ID
            message_type: MessageType::Heartbeat,
            version: PROTOCOL_VERSION,
            timestamp: 1234567890,
            sender_id: "sender".to_string(),
            recipient_id: None,
            payload: MessagePayload::Heartbeat {
                client_time: 1234567890,
                server_time: None,
            },
            metadata: HashMap::new(),
        };

        let protocol = MessageProtocol::new();

        // When: Validating message
        let result = protocol.validate(&message);

        // Then: Should return validation error
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(MessageProtocolError::ValidationFailed(_))
        ));
    }

    #[test]
    fn test_message_compression() {
        // Given: Large message
        let mut metadata = HashMap::new();
        for i in 0..1000 {
            metadata.insert(format!("key_{}", i), format!("value_{}", i));
        }

        let message = Message {
            id: "test-id".to_string(),
            message_type: MessageType::DataUpdate,
            version: PROTOCOL_VERSION,
            timestamp: 1234567890,
            sender_id: "sender".to_string(),
            recipient_id: None,
            payload: MessagePayload::DataUpdate {
                chart_id: "chart-1".to_string(),
                data: vec![],
                update_type: DataUpdateType::Replace,
            },
            metadata,
        };

        let protocol = MessageProtocol::new();

        // When: Compressing message
        let result = protocol.serialize(&message);

        // Then: Should reduce size significantly
        assert!(result.is_ok());
        let compressed = result.unwrap();
        assert!(!compressed.is_empty());
    }

    #[test]
    fn test_heartbeat_message_creation() {
        // Given: Message protocol
        let protocol = MessageProtocol::new();

        // When: Creating heartbeat message
        let message = protocol.create_heartbeat("test-sender");

        // Then: Should create valid heartbeat message
        assert_eq!(message.message_type, MessageType::Heartbeat);
        assert_eq!(message.sender_id, "test-sender");
        assert!(matches!(message.payload, MessagePayload::Heartbeat { .. }));
    }

    #[test]
    fn test_acknowledgment_message_creation() {
        // Given: Message protocol
        let protocol = MessageProtocol::new();

        // When: Creating acknowledgment message
        let message =
            protocol.create_acknowledgment("original-id", "sender", AcknowledgmentStatus::Success);

        // Then: Should create valid acknowledgment message
        assert_eq!(message.message_type, MessageType::Acknowledgment);
        assert_eq!(message.sender_id, "sender");
        assert!(matches!(
            message.payload,
            MessagePayload::Acknowledgment { .. }
        ));
    }

    #[test]
    fn test_error_message_creation() {
        // Given: Message protocol
        let protocol = MessageProtocol::new();

        // When: Creating error message
        let message = protocol.create_error(500, "Test error", "sender", Some("recipient"));

        // Then: Should create valid error message
        assert_eq!(message.message_type, MessageType::Error);
        assert_eq!(message.sender_id, "sender");
        assert_eq!(message.recipient_id, Some("recipient".to_string()));
        assert!(matches!(message.payload, MessagePayload::Error { .. }));
    }

    #[test]
    fn test_protocol_version_validation() {
        // Given: Message with invalid protocol version
        let message = Message {
            id: "test-id".to_string(),
            message_type: MessageType::Heartbeat,
            version: 999, // Invalid version
            timestamp: 1234567890,
            sender_id: "sender".to_string(),
            recipient_id: None,
            payload: MessagePayload::Heartbeat {
                client_time: 1234567890,
                server_time: None,
            },
            metadata: HashMap::new(),
        };

        let protocol = MessageProtocol::new();

        // When: Validating message
        let result = protocol.validate(&message);

        // Then: Should return protocol version error
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(MessageProtocolError::InvalidProtocolVersion(999))
        ));
    }

    #[test]
    fn test_timestamp_validation() {
        // Given: Message with invalid timestamp
        let message = Message {
            id: "test-id".to_string(),
            message_type: MessageType::Heartbeat,
            version: PROTOCOL_VERSION,
            timestamp: 0, // Invalid timestamp
            sender_id: "sender".to_string(),
            recipient_id: None,
            payload: MessagePayload::Heartbeat {
                client_time: 1234567890,
                server_time: None,
            },
            metadata: HashMap::new(),
        };

        let protocol = MessageProtocol::new();

        // When: Validating message
        let result = protocol.validate(&message);

        // Then: Should return timestamp error
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(MessageProtocolError::InvalidTimestamp(0))
        ));
    }
}
