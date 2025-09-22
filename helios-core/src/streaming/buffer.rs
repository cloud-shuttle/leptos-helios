//! Data buffering and management for streaming data

use super::types::*;
use crate::chart_config::*;
use std::collections::VecDeque;

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
