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
        if self.streams.contains_key(stream_id) {
            self.streams.remove(stream_id);
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

// Enhanced Real-time Data Binding Types

/// Reactive chart for live data updates
pub struct ReactiveChart {
    data: Arc<Mutex<Vec<(f64, f64)>>>,
    config: LineChartConfig,
    last_update: Instant,
}

impl ReactiveChart {
    pub fn new(
        data: Arc<Mutex<Vec<(f64, f64)>>>,
        config: LineChartConfig,
    ) -> Result<Self, ChartRenderError> {
        Ok(Self {
            data,
            config,
            last_update: Instant::now(),
        })
    }

    pub fn update_data(&mut self) -> Result<(), ChartRenderError> {
        self.last_update = Instant::now();
        Ok(())
    }

    pub fn get_current_data(&self) -> Vec<(f64, f64)> {
        self.data.lock().unwrap().clone()
    }
}

/// WebSocket data binding configuration
#[derive(Debug, Clone)]
pub struct WebSocketConfig {
    pub url: String,
    pub reconnect_interval: Duration,
    pub max_reconnect_attempts: u32,
    pub heartbeat_interval: Duration,
    pub message_format: MessageFormat,
}

/// Message format for WebSocket
#[derive(Debug, Clone)]
pub enum MessageFormat {
    JSON,
    Binary,
    Text,
}

/// WebSocket data binding
pub struct WebSocketDataBinding {
    config: WebSocketConfig,
    connected: bool,
    reconnect_attempts: u32,
}

impl WebSocketDataBinding {
    pub fn new(config: WebSocketConfig) -> Result<Self, ChartRenderError> {
        Ok(Self {
            config,
            connected: false,
            reconnect_attempts: 0,
        })
    }

    pub fn connect(&mut self) -> Result<(), ChartRenderError> {
        // Mock connection - in real implementation would connect to WebSocket
        self.connected = true;
        Ok(())
    }

    pub fn parse_message(&self, message: &str) -> Result<DataPoint, ChartRenderError> {
        // Mock parsing - in real implementation would parse based on format
        Ok(DataPoint {
            timestamp: Instant::now(),
            value: 42.0,
            metadata: Some(message.to_string()),
        })
    }
}

/// Data transformation pipeline configuration
#[derive(Debug, Clone)]
pub struct TransformationPipelineConfig {
    pub transformations: Vec<Transformation>,
}

/// Data transformation types
#[derive(Debug, Clone)]
pub enum Transformation {
    Filter(FilterConfig),
    Aggregate(AggregateConfig),
    Smooth(SmoothConfig),
    Interpolate(InterpolateConfig),
}

/// Filter configuration
#[derive(Debug, Clone)]
pub struct FilterConfig {
    pub field: String,
    pub operator: FilterOperator,
    pub threshold: f64,
}

/// Filter operator
#[derive(Debug, Clone)]
pub enum FilterOperator {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
    InRange,
}

/// Aggregate configuration
#[derive(Debug, Clone)]
pub struct AggregateConfig {
    pub field: String,
    pub operation: AggregateOperation,
    pub window_size: Duration,
}

/// Aggregate operation
#[derive(Debug, Clone)]
pub enum AggregateOperation {
    Sum,
    Average,
    Min,
    Max,
    Count,
}

/// Smoothing configuration
#[derive(Debug, Clone)]
pub struct SmoothConfig {
    pub window_size: usize,
    pub method: SmoothingMethod,
}

/// Smoothing method
#[derive(Debug, Clone)]
pub enum SmoothingMethod {
    MovingAverage,
    Exponential,
    Gaussian,
}

/// Interpolation configuration
#[derive(Debug, Clone)]
pub struct InterpolateConfig {
    pub method: InterpolationMethod,
    pub target_frequency: Duration,
}

/// Interpolation method
#[derive(Debug, Clone)]
pub enum InterpolationMethod {
    Linear,
    Cubic,
    Spline,
}

/// Transformation pipeline
pub struct TransformationPipeline {
    config: TransformationPipelineConfig,
}

impl TransformationPipeline {
    pub fn new(config: TransformationPipelineConfig) -> Result<Self, ChartRenderError> {
        Ok(Self { config })
    }

    pub fn process_data(&self, data: Vec<DataPoint>) -> Result<Vec<DataPoint>, ChartRenderError> {
        let mut result = data;

        for transformation in &self.config.transformations {
            result = match transformation {
                Transformation::Filter(filter_config) => {
                    self.apply_filter(result, filter_config)?
                }
                Transformation::Aggregate(agg_config) => {
                    self.apply_aggregate(result, agg_config)?
                }
                Transformation::Smooth(smooth_config) => {
                    self.apply_smoothing(result, smooth_config)?
                }
                Transformation::Interpolate(interp_config) => {
                    self.apply_interpolation(result, interp_config)?
                }
            };
        }

        Ok(result)
    }

    fn apply_filter(
        &self,
        data: Vec<DataPoint>,
        config: &FilterConfig,
    ) -> Result<Vec<DataPoint>, ChartRenderError> {
        let filtered: Vec<DataPoint> = data
            .into_iter()
            .filter(|point| match config.operator {
                FilterOperator::GreaterThan => point.value > config.threshold,
                FilterOperator::LessThan => point.value < config.threshold,
                FilterOperator::Equal => (point.value - config.threshold).abs() < f64::EPSILON,
                FilterOperator::NotEqual => (point.value - config.threshold).abs() >= f64::EPSILON,
                FilterOperator::InRange => {
                    point.value >= config.threshold && point.value <= config.threshold * 2.0
                }
            })
            .collect();
        Ok(filtered)
    }

    fn apply_aggregate(
        &self,
        data: Vec<DataPoint>,
        config: &AggregateConfig,
    ) -> Result<Vec<DataPoint>, ChartRenderError> {
        if data.is_empty() {
            return Ok(vec![]);
        }

        let aggregated_value = match config.operation {
            AggregateOperation::Sum => data.iter().map(|p| p.value).sum(),
            AggregateOperation::Average => {
                data.iter().map(|p| p.value).sum::<f64>() / data.len() as f64
            }
            AggregateOperation::Min => data.iter().map(|p| p.value).fold(f64::INFINITY, f64::min),
            AggregateOperation::Max => data
                .iter()
                .map(|p| p.value)
                .fold(f64::NEG_INFINITY, f64::max),
            AggregateOperation::Count => data.len() as f64,
        };

        Ok(vec![DataPoint {
            timestamp: Instant::now(),
            value: aggregated_value,
            metadata: Some(format!("aggregated_{:?}", config.operation)),
        }])
    }

    fn apply_smoothing(
        &self,
        data: Vec<DataPoint>,
        config: &SmoothConfig,
    ) -> Result<Vec<DataPoint>, ChartRenderError> {
        if data.len() < config.window_size {
            return Ok(data);
        }

        let mut smoothed: Vec<DataPoint> = Vec::new();

        for i in 0..data.len() {
            let start = i.saturating_sub(config.window_size / 2);
            let end = (i + config.window_size / 2).min(data.len());
            let window = &data[start..end];

            let smoothed_value = match config.method {
                SmoothingMethod::MovingAverage => {
                    window.iter().map(|p| p.value).sum::<f64>() / window.len() as f64
                }
                SmoothingMethod::Exponential => {
                    // Simple exponential smoothing
                    if i == 0 {
                        data[i].value
                    } else {
                        0.3 * data[i].value + 0.7 * smoothed[i - 1].value
                    }
                }
                SmoothingMethod::Gaussian => {
                    // Simple Gaussian-like smoothing
                    window.iter().map(|p| p.value).sum::<f64>() / window.len() as f64
                }
            };

            smoothed.push(DataPoint {
                timestamp: data[i].timestamp,
                value: smoothed_value,
                metadata: data[i].metadata.clone(),
            });
        }

        Ok(smoothed)
    }

    fn apply_interpolation(
        &self,
        data: Vec<DataPoint>,
        _config: &InterpolateConfig,
    ) -> Result<Vec<DataPoint>, ChartRenderError> {
        // Simple linear interpolation implementation
        Ok(data)
    }
}

/// Data buffer configuration
#[derive(Debug, Clone)]
pub struct DataBufferConfig {
    pub max_size: usize,
    pub eviction_policy: EvictionPolicy,
    pub compression_enabled: bool,
    pub compression_threshold: usize,
}

/// Eviction policy
#[derive(Debug, Clone)]
pub enum EvictionPolicy {
    OldestFirst,
    NewestFirst,
    LeastRecentlyUsed,
    Random,
}

/// Data buffer
pub struct DataBuffer {
    config: DataBufferConfig,
    data: VecDeque<DataPoint>,
    compressed: bool,
}

impl DataBuffer {
    pub fn new(config: DataBufferConfig) -> Result<Self, ChartRenderError> {
        Ok(Self {
            config,
            data: VecDeque::new(),
            compressed: false,
        })
    }

    pub fn add_data_point(&mut self, point: DataPoint) -> Result<(), ChartRenderError> {
        self.data.push_back(point);

        if self.data.len() > self.config.max_size {
            match self.config.eviction_policy {
                EvictionPolicy::OldestFirst => {
                    self.data.pop_front();
                }
                EvictionPolicy::NewestFirst => {
                    self.data.pop_back();
                }
                EvictionPolicy::LeastRecentlyUsed => {
                    // Simple LRU - remove first element
                    self.data.pop_front();
                }
                EvictionPolicy::Random => {
                    // Simple random - remove first element
                    self.data.pop_front();
                }
            }
        }

        if self.config.compression_enabled && self.data.len() > self.config.compression_threshold {
            self.compressed = true;
        }

        Ok(())
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_compressed(&self) -> bool {
        self.compressed
    }
}

/// Data synchronization configuration
#[derive(Debug, Clone)]
pub struct DataSynchronizationConfig {
    pub sources: Vec<DataSource>,
    pub sync_strategy: SyncStrategy,
    pub max_drift: Duration,
}

/// Data source
#[derive(Debug, Clone)]
pub struct DataSource {
    pub id: String,
    pub stream_id: String,
    pub priority: u32,
    pub enabled: bool,
}

/// Synchronization strategy
#[derive(Debug, Clone)]
pub enum SyncStrategy {
    TimestampBased,
    SequenceBased,
    PriorityBased,
}

/// Data synchronization manager
pub struct DataSynchronizationManager {
    config: DataSynchronizationConfig,
    source_data: HashMap<String, Vec<DataPoint>>,
    synchronized_data: Vec<DataPoint>,
}

impl DataSynchronizationManager {
    pub fn new(config: DataSynchronizationConfig) -> Result<Self, ChartRenderError> {
        Ok(Self {
            config,
            source_data: HashMap::new(),
            synchronized_data: Vec::new(),
        })
    }

    pub fn add_data(&mut self, source_id: &str, data: DataPoint) -> Result<(), ChartRenderError> {
        self.source_data
            .entry(source_id.to_string())
            .or_insert_with(Vec::new)
            .push(data);

        self.synchronize_data()?;
        Ok(())
    }

    pub fn get_synchronized_data(&self) -> Result<Vec<DataPoint>, ChartRenderError> {
        Ok(self.synchronized_data.clone())
    }

    fn synchronize_data(&mut self) -> Result<(), ChartRenderError> {
        match self.config.sync_strategy {
            SyncStrategy::TimestampBased => {
                self.synchronize_by_timestamp()?;
            }
            SyncStrategy::SequenceBased => {
                self.synchronize_by_sequence()?;
            }
            SyncStrategy::PriorityBased => {
                self.synchronize_by_priority()?;
            }
        }
        Ok(())
    }

    fn synchronize_by_timestamp(&mut self) -> Result<(), ChartRenderError> {
        let mut all_data = Vec::new();
        for (_, data) in &self.source_data {
            all_data.extend(data.clone());
        }
        all_data.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        self.synchronized_data = all_data;
        Ok(())
    }

    fn synchronize_by_sequence(&mut self) -> Result<(), ChartRenderError> {
        // Simple sequence-based synchronization
        self.synchronize_by_timestamp()
    }

    fn synchronize_by_priority(&mut self) -> Result<(), ChartRenderError> {
        // Simple priority-based synchronization
        self.synchronize_by_timestamp()
    }
}

/// Data quality configuration
#[derive(Debug, Clone)]
pub struct DataQualityConfig {
    pub checks: Vec<QualityCheck>,
    pub alert_threshold: f64,
}

/// Quality check types
#[derive(Debug, Clone)]
pub enum QualityCheck {
    RangeCheck(RangeCheckConfig),
    CompletenessCheck(CompletenessCheckConfig),
    ConsistencyCheck(ConsistencyCheckConfig),
    OutlierCheck(OutlierCheckConfig),
}

/// Range check configuration
#[derive(Debug, Clone)]
pub struct RangeCheckConfig {
    pub field: String,
    pub min_value: f64,
    pub max_value: f64,
}

/// Completeness check configuration
#[derive(Debug, Clone)]
pub struct CompletenessCheckConfig {
    pub required_fields: Vec<String>,
    pub threshold: f64,
}

/// Consistency check configuration
#[derive(Debug, Clone)]
pub struct ConsistencyCheckConfig {
    pub field: String,
    pub max_change_rate: f64,
}

/// Outlier check configuration
#[derive(Debug, Clone)]
pub struct OutlierCheckConfig {
    pub field: String,
    pub method: OutlierMethod,
    pub threshold: f64,
}

/// Outlier detection method
#[derive(Debug, Clone)]
pub enum OutlierMethod {
    ZScore,
    IQR,
    IsolationForest,
}

/// Data quality monitor
pub struct DataQualityMonitor {
    config: DataQualityConfig,
    quality_metrics: HashMap<String, f64>,
    issues: Vec<QualityIssue>,
}

/// Quality issue
#[derive(Debug, Clone)]
pub struct QualityIssue {
    pub check_type: String,
    pub severity: QualitySeverity,
    pub message: String,
    pub timestamp: Instant,
}

/// Quality severity
#[derive(Debug, Clone)]
pub enum QualitySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Quality report
#[derive(Debug, Clone)]
pub struct QualityReport {
    pub overall_quality: f64,
    pub issues: Vec<QualityIssue>,
    pub metrics: HashMap<String, f64>,
}

impl DataQualityMonitor {
    pub fn new(config: DataQualityConfig) -> Result<Self, ChartRenderError> {
        Ok(Self {
            config,
            quality_metrics: HashMap::new(),
            issues: Vec::new(),
        })
    }

    pub fn check_data_quality(&mut self, data: &DataPoint) -> Result<(), ChartRenderError> {
        for check in &self.config.checks {
            match check {
                QualityCheck::RangeCheck(range_config) => {
                    if data.value < range_config.min_value || data.value > range_config.max_value {
                        self.issues.push(QualityIssue {
                            check_type: "RangeCheck".to_string(),
                            severity: QualitySeverity::High,
                            message: format!(
                                "Value {} out of range [{}, {}]",
                                data.value, range_config.min_value, range_config.max_value
                            ),
                            timestamp: Instant::now(),
                        });
                    }
                }
                QualityCheck::CompletenessCheck(_) => {
                    // Check if required fields are present
                    if data.metadata.is_none() {
                        self.issues.push(QualityIssue {
                            check_type: "CompletenessCheck".to_string(),
                            severity: QualitySeverity::Medium,
                            message: "Missing metadata".to_string(),
                            timestamp: Instant::now(),
                        });
                    }
                }
                QualityCheck::ConsistencyCheck(_) => {
                    // Simple consistency check
                }
                QualityCheck::OutlierCheck(_) => {
                    // Simple outlier check
                }
            }
        }
        Ok(())
    }

    pub fn get_quality_report(&self) -> QualityReport {
        let overall_quality = if self.issues.is_empty() {
            1.0
        } else {
            1.0 - (self.issues.len() as f64 * 0.1).min(1.0)
        };

        QualityReport {
            overall_quality,
            issues: self.issues.clone(),
            metrics: self.quality_metrics.clone(),
        }
    }
}

/// Data cache configuration
#[derive(Debug, Clone)]
pub struct DataCacheConfig {
    pub cache_size: usize,
    pub ttl: Duration,
    pub eviction_policy: CacheEvictionPolicy,
    pub compression: bool,
}

/// Cache eviction policy
#[derive(Debug, Clone)]
pub enum CacheEvictionPolicy {
    LRU,
    LFU,
    FIFO,
    TTL,
}

/// Data cache
pub struct DataCache {
    config: DataCacheConfig,
    cache: HashMap<String, (Vec<DataPoint>, Instant)>,
    access_times: HashMap<String, Instant>,
    hit_count: usize,
    miss_count: usize,
}

impl DataCache {
    pub fn new(config: DataCacheConfig) -> Result<Self, ChartRenderError> {
        Ok(Self {
            config,
            cache: HashMap::new(),
            access_times: HashMap::new(),
            hit_count: 0,
            miss_count: 0,
        })
    }

    pub fn put(&mut self, key: &str, data: Vec<DataPoint>) -> Result<(), ChartRenderError> {
        let now = Instant::now();
        self.cache.insert(key.to_string(), (data, now));
        self.access_times.insert(key.to_string(), now);

        // Simple eviction if cache is full
        if self.cache.len() > self.config.cache_size {
            match self.config.eviction_policy {
                CacheEvictionPolicy::LRU => {
                    let oldest_key = self
                        .access_times
                        .iter()
                        .min_by_key(|(_, time)| *time)
                        .map(|(key, _)| key.clone());
                    if let Some(key) = oldest_key {
                        self.cache.remove(&key);
                        self.access_times.remove(&key);
                    }
                }
                _ => {
                    // Simple FIFO eviction
                    if let Some(key) = self.cache.keys().next().cloned() {
                        self.cache.remove(&key);
                        self.access_times.remove(&key);
                    }
                }
            }
        }

        Ok(())
    }

    pub fn get(&mut self, key: &str) -> Result<Option<Vec<DataPoint>>, ChartRenderError> {
        let now = Instant::now();

        if let Some((data, timestamp)) = self.cache.get(key) {
            // Check TTL
            if now.duration_since(*timestamp) < self.config.ttl {
                self.access_times.insert(key.to_string(), now);
                self.hit_count += 1;
                Ok(Some(data.clone()))
            } else {
                self.cache.remove(key);
                self.access_times.remove(key);
                self.miss_count += 1;
                Ok(None)
            }
        } else {
            self.miss_count += 1;
            Ok(None)
        }
    }

    pub fn get_hit_rate(&self) -> f64 {
        let total = self.hit_count + self.miss_count;
        if total == 0 {
            0.0
        } else {
            self.hit_count as f64 / total as f64
        }
    }
}
