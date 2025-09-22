//! Data synchronization for multiple streaming sources

use super::types::*;
use crate::chart_config::*;
use std::collections::HashMap;
use std::time::Duration;

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
