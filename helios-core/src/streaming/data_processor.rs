//! Real-time data processing and reactive charts

use super::{types::*, stream_manager::*};
use crate::chart_config::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Real-time data processor
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
