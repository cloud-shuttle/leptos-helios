//! Performance Optimization Module
//!
//! This module provides comprehensive performance optimization features for the Helios charting library,
//! with a focus on mobile GPU optimization, memory management, and rendering efficiency.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

pub mod data_streaming;
pub mod memory_pools;
pub mod mobile_gpu;
pub mod rendering_pipeline;

pub use data_streaming::*;
pub use memory_pools::*;
pub use mobile_gpu::*;
pub use rendering_pipeline::*;

/// Performance optimization manager that coordinates all optimization subsystems
#[derive(Debug, Clone)]
pub struct PerformanceOptimizationManager {
    mobile_gpu: Arc<RwLock<MobileGpuOptimizer>>,
    memory_pools: Arc<RwLock<MemoryPoolManager>>,
    rendering_pipeline: Arc<RwLock<RenderingPipelineOptimizer>>,
    data_streaming: Arc<RwLock<DataStreamingOptimizer>>,
    config: PerformanceConfig,
    stats: Arc<RwLock<PerformanceStats>>,
}

/// Configuration for performance optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub mobile_gpu_config: MobileGpuConfig,
    pub memory_pool_config: MemoryPoolConfig,
    pub rendering_config: RenderingPipelineConfig,
    pub data_streaming_config: DataStreamingConfig,
    pub enable_optimizations: bool,
    pub target_fps: u32,
    pub max_memory_usage: usize,
}

/// Performance statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerformanceStats {
    pub frame_rate: f64,
    pub memory_usage: usize,
    pub gpu_memory_usage: usize,
    pub render_time: f64,
    pub data_processing_time: f64,
    pub optimization_benefits: HashMap<String, f64>,
    pub total_optimizations_applied: u32,
}

/// Performance optimization errors
#[derive(Error, Debug)]
pub enum PerformanceOptimizationError {
    #[error("GPU optimization failed: {message}")]
    GpuOptimizationFailed { message: String },

    #[error("Memory pool error: {message}")]
    MemoryPoolError { message: String },

    #[error("Rendering pipeline error: {message}")]
    RenderingPipelineError { message: String },

    #[error("Data streaming error: {message}")]
    DataStreamingError { message: String },

    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },

    #[error("Performance target not met: {target} vs {actual}")]
    PerformanceTargetNotMet { target: f64, actual: f64 },
}

impl From<MobileGpuOptimizationError> for PerformanceOptimizationError {
    fn from(err: MobileGpuOptimizationError) -> Self {
        PerformanceOptimizationError::GpuOptimizationFailed {
            message: err.to_string(),
        }
    }
}

impl From<MemoryPoolError> for PerformanceOptimizationError {
    fn from(err: MemoryPoolError) -> Self {
        PerformanceOptimizationError::MemoryPoolError {
            message: err.to_string(),
        }
    }
}

impl From<RenderingPipelineError> for PerformanceOptimizationError {
    fn from(err: RenderingPipelineError) -> Self {
        PerformanceOptimizationError::RenderingPipelineError {
            message: err.to_string(),
        }
    }
}

impl From<DataStreamingError> for PerformanceOptimizationError {
    fn from(err: DataStreamingError) -> Self {
        PerformanceOptimizationError::DataStreamingError {
            message: err.to_string(),
        }
    }
}

impl PerformanceOptimizationManager {
    /// Create a new performance optimization manager
    pub fn new(config: PerformanceConfig) -> Self {
        Self {
            mobile_gpu: Arc::new(RwLock::new(MobileGpuOptimizer::new(
                config.mobile_gpu_config.clone(),
            ))),
            memory_pools: Arc::new(RwLock::new(MemoryPoolManager::new(
                config.memory_pool_config.clone(),
            ))),
            rendering_pipeline: Arc::new(RwLock::new(RenderingPipelineOptimizer::new(
                config.rendering_config.clone(),
            ))),
            data_streaming: Arc::new(RwLock::new(DataStreamingOptimizer::new(
                config.data_streaming_config.clone(),
            ))),
            config,
            stats: Arc::new(RwLock::new(PerformanceStats::default())),
        }
    }

    /// Optimize performance for current conditions
    pub async fn optimize_performance(&self) -> Result<(), PerformanceOptimizationError> {
        if !self.config.enable_optimizations {
            return Ok(());
        }

        // Run all optimization subsystems
        let gpu_result = self
            .mobile_gpu
            .read()
            .await
            .optimize_gpu_performance()
            .await;
        let memory_result = self.memory_pools.read().await.optimize_memory_usage().await;
        let rendering_result = self
            .rendering_pipeline
            .read()
            .await
            .optimize_rendering()
            .await;
        let streaming_result = self
            .data_streaming
            .read()
            .await
            .optimize_data_streaming()
            .await;

        // Update statistics
        self.update_performance_stats().await;

        // Check if any optimization failed
        if let Err(e) = gpu_result {
            return Err(PerformanceOptimizationError::GpuOptimizationFailed {
                message: e.to_string(),
            });
        }
        if let Err(e) = memory_result {
            return Err(PerformanceOptimizationError::MemoryPoolError {
                message: e.to_string(),
            });
        }
        if let Err(e) = rendering_result {
            return Err(PerformanceOptimizationError::RenderingPipelineError {
                message: e.to_string(),
            });
        }
        if let Err(e) = streaming_result {
            return Err(PerformanceOptimizationError::DataStreamingError {
                message: e.to_string(),
            });
        }

        Ok(())
    }

    /// Get current performance statistics
    pub async fn get_performance_stats(&self) -> PerformanceStats {
        self.stats.read().await.clone()
    }

    /// Update performance configuration
    pub async fn update_config(
        &mut self,
        config: PerformanceConfig,
    ) -> Result<(), PerformanceOptimizationError> {
        self.config = config.clone();

        // Update subsystem configurations
        {
            let mut gpu = self.mobile_gpu.write().await;
            gpu.update_config(config.mobile_gpu_config).await?;
        }

        {
            let mut memory = self.memory_pools.write().await;
            memory.update_config(config.memory_pool_config).await?;
        }

        {
            let mut rendering = self.rendering_pipeline.write().await;
            rendering.update_config(config.rendering_config).await?;
        }

        {
            let mut streaming = self.data_streaming.write().await;
            streaming
                .update_config(config.data_streaming_config)
                .await?;
        }

        Ok(())
    }

    /// Check if performance targets are being met
    pub async fn check_performance_targets(&self) -> Result<(), PerformanceOptimizationError> {
        let stats = self.get_performance_stats().await;

        if stats.frame_rate < self.config.target_fps as f64 {
            return Err(PerformanceOptimizationError::PerformanceTargetNotMet {
                target: self.config.target_fps as f64,
                actual: stats.frame_rate,
            });
        }

        if stats.memory_usage > self.config.max_memory_usage {
            return Err(PerformanceOptimizationError::PerformanceTargetNotMet {
                target: self.config.max_memory_usage as f64,
                actual: stats.memory_usage as f64,
            });
        }

        Ok(())
    }

    /// Update performance statistics
    async fn update_performance_stats(&self) {
        let mut stats = self.stats.write().await;

        // Collect stats from all subsystems
        let gpu_stats = self.mobile_gpu.read().await.get_stats().await;
        let memory_stats = self.memory_pools.read().await.get_stats().await;
        let rendering_stats = self.rendering_pipeline.read().await.get_stats().await;
        let streaming_stats = self.data_streaming.read().await.get_stats().await;

        // Update combined stats
        stats.frame_rate = gpu_stats.frame_rate;
        stats.memory_usage = memory_stats.total_memory_usage;
        stats.gpu_memory_usage = gpu_stats.gpu_memory_usage;
        stats.render_time = rendering_stats.average_render_time;
        stats.data_processing_time = streaming_stats.average_processing_time;

        // Update optimization benefits
        stats.optimization_benefits.insert(
            "gpu_optimization".to_string(),
            gpu_stats.optimization_benefit,
        );
        stats.optimization_benefits.insert(
            "memory_optimization".to_string(),
            memory_stats.optimization_benefit,
        );
        stats.optimization_benefits.insert(
            "rendering_optimization".to_string(),
            rendering_stats.optimization_benefit,
        );
        stats.optimization_benefits.insert(
            "streaming_optimization".to_string(),
            streaming_stats.optimization_benefit,
        );

        stats.total_optimizations_applied = gpu_stats.total_optimizations_applied
            + memory_stats.optimizations_applied
            + rendering_stats.optimizations_applied
            + streaming_stats.optimizations_applied;
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            mobile_gpu_config: MobileGpuConfig::default(),
            memory_pool_config: MemoryPoolConfig::default(),
            rendering_config: RenderingPipelineConfig::default(),
            data_streaming_config: DataStreamingConfig::default(),
            enable_optimizations: true,
            target_fps: 60,
            max_memory_usage: 100 * 1024 * 1024, // 100MB
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_performance_config() -> PerformanceConfig {
        PerformanceConfig {
            mobile_gpu_config: MobileGpuConfig::default(),
            memory_pool_config: MemoryPoolConfig::default(),
            rendering_config: RenderingPipelineConfig::default(),
            data_streaming_config: DataStreamingConfig::default(),
            enable_optimizations: true,
            target_fps: 60,
            max_memory_usage: 50 * 1024 * 1024, // 50MB for testing
        }
    }

    #[tokio::test]
    async fn test_performance_optimization_manager_creation() {
        let config = create_test_performance_config();
        let manager = PerformanceOptimizationManager::new(config);

        let stats = manager.get_performance_stats().await;
        assert_eq!(stats.frame_rate, 0.0);
        assert_eq!(stats.memory_usage, 0);
        assert_eq!(stats.total_optimizations_applied, 0);
    }

    #[tokio::test]
    async fn test_performance_optimization() {
        let config = create_test_performance_config();
        let manager = PerformanceOptimizationManager::new(config);

        let result = manager.optimize_performance().await;
        assert!(result.is_ok());

        let stats = manager.get_performance_stats().await;
        assert!(stats.total_optimizations_applied > 0);
    }

    #[tokio::test]
    async fn test_performance_targets_check() {
        let config = create_test_performance_config();
        let manager = PerformanceOptimizationManager::new(config);

        // Run optimization first to set proper stats
        let _ = manager.optimize_performance().await;

        // Now should pass with optimized stats
        let result = manager.check_performance_targets().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_config_update() {
        let config = create_test_performance_config();
        let mut manager = PerformanceOptimizationManager::new(config);

        let new_config = create_test_performance_config();
        let result = manager.update_config(new_config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_performance_stats_update() {
        let config = create_test_performance_config();
        let manager = PerformanceOptimizationManager::new(config);

        // Run optimization to update stats
        let _ = manager.optimize_performance().await;

        let stats = manager.get_performance_stats().await;
        assert!(stats.optimization_benefits.len() > 0);
    }
}
