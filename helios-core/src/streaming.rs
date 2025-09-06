//! Streaming Data System
//! Real-time data streaming and updates for dynamic visualizations

use crate::chart_config::*;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Data types for streaming
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    TimeSeries,
    Events,
    Metrics,
    Logs,
}

/// Data point for streaming
#[derive(Debug, Clone)]
pub struct DataPoint {
    pub timestamp: Instant,
    pub value: f64,
    pub metadata: Option<String>,
}

/// Stream configuration
#[derive(Debug, Clone)]
pub struct StreamConfig {
    pub stream_id: String,
    pub buffer_size: usize,
    pub update_interval: Duration,
    pub data_type: DataType,
}

/// Stream statistics
#[derive(Debug, Clone)]
pub struct StreamStats {
    pub total_published: usize,
    pub active_subscribers: usize,
    pub buffer_usage: usize,
    pub last_update: Option<Instant>,
}

/// Stream subscriber
pub struct StreamSubscriber {
    stream_id: String,
    buffer: Arc<Mutex<VecDeque<DataPoint>>>,
    is_active: Arc<Mutex<bool>>,
    last_read_index: Arc<Mutex<usize>>,
}

impl StreamSubscriber {
    pub fn new(stream_id: String, buffer: Arc<Mutex<VecDeque<DataPoint>>>) -> Self {
        Self {
            stream_id,
            buffer,
            is_active: Arc::new(Mutex::new(true)),
            last_read_index: Arc::new(Mutex::new(0)),
        }
    }

    pub fn stream_id(&self) -> &str {
        &self.stream_id
    }

    pub fn receive(&self) -> Option<DataPoint> {
        if let (Ok(buffer), Ok(mut last_read_index)) =
            (self.buffer.lock(), self.last_read_index.lock())
        {
            if *last_read_index < buffer.len() {
                let data_point = buffer[*last_read_index].clone();
                *last_read_index += 1;
                Some(data_point)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn is_active(&self) -> bool {
        if let Ok(is_active) = self.is_active.lock() {
            *is_active
        } else {
            false
        }
    }

    pub fn close(&self) {
        if let Ok(mut is_active) = self.is_active.lock() {
            *is_active = false;
        }
    }
}

/// Internal stream data
struct StreamData {
    config: StreamConfig,
    buffer: Arc<Mutex<VecDeque<DataPoint>>>,
    subscribers: Vec<Arc<StreamSubscriber>>,
    stats: StreamStats,
}

impl StreamData {
    fn new(config: StreamConfig) -> Self {
        Self {
            buffer: Arc::new(Mutex::new(VecDeque::new())),
            subscribers: Vec::new(),
            stats: StreamStats {
                total_published: 0,
                active_subscribers: 0,
                buffer_usage: 0,
                last_update: None,
            },
            config,
        }
    }

    fn add_subscriber(&mut self, subscriber: Arc<StreamSubscriber>) {
        self.subscribers.push(subscriber);
        self.stats.active_subscribers = self.subscribers.len();
    }

    fn publish_data(&mut self, data_point: DataPoint) -> Result<(), ChartRenderError> {
        let timestamp = data_point.timestamp;

        if let Ok(mut buffer) = self.buffer.lock() {
            buffer.push_back(data_point);

            // Enforce buffer size limit
            while buffer.len() > self.config.buffer_size {
                buffer.pop_front();
            }

            self.stats.buffer_usage = buffer.len();
            self.stats.total_published += 1;
            self.stats.last_update = Some(timestamp);
        }

        Ok(())
    }

    fn get_stats(&self) -> StreamStats {
        self.stats.clone()
    }
}

/// Streaming manager for handling real-time data
pub struct StreamingManager {
    streams: HashMap<String, StreamData>,
}

impl StreamingManager {
    pub fn new() -> Result<Self, ChartRenderError> {
        Ok(Self {
            streams: HashMap::new(),
        })
    }

    /// Create a new data stream
    pub fn create_stream(&mut self, config: StreamConfig) -> Result<(), ChartRenderError> {
        if self.streams.contains_key(&config.stream_id) {
            return Err(ChartRenderError::InvalidConfig(format!(
                "Stream '{}' already exists",
                config.stream_id
            )));
        }

        let stream_data = StreamData::new(config);
        self.streams
            .insert(stream_data.config.stream_id.clone(), stream_data);

        Ok(())
    }

    /// Subscribe to a data stream
    pub fn subscribe(
        &mut self,
        stream_id: &str,
    ) -> Result<Arc<StreamSubscriber>, ChartRenderError> {
        if let Some(stream_data) = self.streams.get_mut(stream_id) {
            let subscriber = Arc::new(StreamSubscriber::new(
                stream_id.to_string(),
                stream_data.buffer.clone(),
            ));

            stream_data.add_subscriber(subscriber.clone());
            Ok(subscriber)
        } else {
            Err(ChartRenderError::InvalidConfig(format!(
                "Stream '{}' does not exist",
                stream_id
            )))
        }
    }

    /// Publish data to a stream
    pub fn publish(
        &mut self,
        stream_id: &str,
        data_point: DataPoint,
    ) -> Result<(), ChartRenderError> {
        if let Some(stream_data) = self.streams.get_mut(stream_id) {
            stream_data.publish_data(data_point)
        } else {
            Err(ChartRenderError::InvalidConfig(format!(
                "Stream '{}' does not exist",
                stream_id
            )))
        }
    }

    /// Close a data stream
    pub fn close_stream(&mut self, stream_id: &str) -> Result<(), ChartRenderError> {
        if let Some(stream_data) = self.streams.remove(stream_id) {
            // Close all subscribers
            for subscriber in stream_data.subscribers {
                subscriber.close();
            }
            Ok(())
        } else {
            Err(ChartRenderError::InvalidConfig(format!(
                "Stream '{}' does not exist",
                stream_id
            )))
        }
    }

    /// Get stream statistics
    pub fn get_stream_stats(&self, stream_id: &str) -> Result<StreamStats, ChartRenderError> {
        if let Some(stream_data) = self.streams.get(stream_id) {
            Ok(stream_data.get_stats())
        } else {
            Err(ChartRenderError::InvalidConfig(format!(
                "Stream '{}' does not exist",
                stream_id
            )))
        }
    }

    /// List all available streams
    pub fn list_streams(&self) -> Vec<String> {
        self.streams.keys().cloned().collect()
    }

    /// Get stream configuration
    pub fn get_stream_config(&self, stream_id: &str) -> Result<&StreamConfig, ChartRenderError> {
        if let Some(stream_data) = self.streams.get(stream_id) {
            Ok(&stream_data.config)
        } else {
            Err(ChartRenderError::InvalidConfig(format!(
                "Stream '{}' does not exist",
                stream_id
            )))
        }
    }
}

/// Streaming error types
#[derive(Debug, thiserror::Error)]
pub enum StreamingError {
    #[error("Stream not found: {0}")]
    StreamNotFound(String),

    #[error("Stream already exists: {0}")]
    StreamExists(String),

    #[error("Buffer overflow: {0}")]
    BufferOverflow(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Subscriber error: {0}")]
    SubscriberError(String),
}

impl From<StreamingError> for ChartRenderError {
    fn from(error: StreamingError) -> Self {
        match error {
            StreamingError::StreamNotFound(msg) => ChartRenderError::InvalidConfig(msg),
            StreamingError::StreamExists(msg) => ChartRenderError::InvalidConfig(msg),
            StreamingError::BufferOverflow(msg) => ChartRenderError::InvalidConfig(msg),
            StreamingError::InvalidConfig(msg) => ChartRenderError::InvalidConfig(msg),
            StreamingError::SubscriberError(msg) => ChartRenderError::InvalidConfig(msg),
        }
    }
}

/// Real-time data processor for chart updates
pub struct RealtimeDataProcessor {
    streaming_manager: StreamingManager,
    update_callbacks: HashMap<String, Vec<Box<dyn Fn(DataPoint) + Send + Sync>>>,
}

impl RealtimeDataProcessor {
    pub fn new() -> Result<Self, ChartRenderError> {
        Ok(Self {
            streaming_manager: StreamingManager::new()?,
            update_callbacks: HashMap::new(),
        })
    }

    /// Register a callback for stream updates
    pub fn register_callback<F>(
        &mut self,
        stream_id: &str,
        callback: F,
    ) -> Result<(), ChartRenderError>
    where
        F: Fn(DataPoint) + Send + Sync + 'static,
    {
        let callbacks = self
            .update_callbacks
            .entry(stream_id.to_string())
            .or_insert_with(Vec::new);
        callbacks.push(Box::new(callback));
        Ok(())
    }

    /// Process incoming data and trigger callbacks
    pub fn process_data(
        &mut self,
        stream_id: &str,
        data_point: DataPoint,
    ) -> Result<(), ChartRenderError> {
        // Publish to stream
        self.streaming_manager
            .publish(stream_id, data_point.clone())?;

        // Trigger callbacks
        if let Some(callbacks) = self.update_callbacks.get(stream_id) {
            for callback in callbacks {
                callback(data_point.clone());
            }
        }

        Ok(())
    }

    /// Get the streaming manager
    pub fn streaming_manager(&mut self) -> &mut StreamingManager {
        &mut self.streaming_manager
    }
}
