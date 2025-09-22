//! Stream management and subscriber handling

use super::types::*;
use crate::chart_config::*;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

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

    pub fn read_new_data(&self) -> Result<Vec<DataPoint>, ChartRenderError> {
        if !self.is_active() {
            return Err(ChartRenderError::InvalidConfig(
                "Subscriber is not active".to_string(),
            ));
        }

        let buffer = self.buffer.lock().unwrap();
        let last_index = *self.last_read_index.lock().unwrap();

        if last_index < buffer.len() {
            let new_data: Vec<DataPoint> = buffer.iter().skip(last_index).cloned().collect();

            *self.last_read_index.lock().unwrap() = buffer.len();
            Ok(new_data)
        } else {
            Ok(vec![])
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
    pub fn subscribe(&mut self, stream_id: &str) -> Result<Arc<StreamSubscriber>, ChartRenderError> {
        if let Some(stream_data) = self.streams.get_mut(stream_id) {
            let subscriber = Arc::new(StreamSubscriber::new(
                stream_id.to_string(),
                stream_data.buffer.clone(),
            ));
            stream_data.add_subscriber(subscriber.clone());
            Ok(subscriber)
        } else {
            Err(ChartRenderError::InvalidConfig(format!(
                "Stream '{}' not found",
                stream_id
            )))
        }
    }

    /// Publish data to a stream
    pub fn publish(&mut self, stream_id: &str, data_point: DataPoint) -> Result<(), ChartRenderError> {
        if let Some(stream_data) = self.streams.get_mut(stream_id) {
            stream_data.publish_data(data_point)
        } else {
            Err(ChartRenderError::InvalidConfig(format!(
                "Stream '{}' not found",
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
                "Stream '{}' not found",
                stream_id
            )))
        }
    }

    /// Remove a stream
    pub fn remove_stream(&mut self, stream_id: &str) -> Result<(), ChartRenderError> {
        if self.streams.remove(stream_id).is_some() {
            Ok(())
        } else {
            Err(ChartRenderError::InvalidConfig(format!(
                "Stream '{}' not found",
                stream_id
            )))
        }
    }

    /// List all available streams
    pub fn list_streams(&self) -> Vec<String> {
        self.streams.keys().cloned().collect()
    }
}
