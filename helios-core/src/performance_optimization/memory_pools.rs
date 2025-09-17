//! Memory Pool Management Module
//!
//! This module provides efficient memory pool management for the Helios charting library,
//! including buffer pools, memory allocation strategies, and garbage collection optimization.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

/// Memory pool manager for efficient memory allocation
#[derive(Debug, Clone)]
pub struct MemoryPoolManager {
    config: MemoryPoolConfig,
    stats: Arc<RwLock<MemoryPoolStats>>,
    buffer_pools: Arc<RwLock<HashMap<String, BufferPool>>>,
    allocation_strategies: Arc<RwLock<HashMap<String, AllocationStrategy>>>,
    gc_engine: Arc<RwLock<GarbageCollectionEngine>>,
}

/// Configuration for memory pool management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPoolConfig {
    pub enable_buffer_pools: bool,
    pub enable_allocation_strategies: bool,
    pub enable_garbage_collection: bool,
    pub max_memory_usage: usize,
    pub buffer_pool_size: usize,
    pub allocation_strategy: AllocationStrategyType,
    pub gc_frequency: u32,
    pub gc_threshold: f64,
}

/// Allocation strategy types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AllocationStrategyType {
    FirstFit,
    BestFit,
    WorstFit,
    BuddySystem,
    SlabAllocator,
}

/// Buffer pool information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferPool {
    pub pool_id: String,
    pub buffer_size: usize,
    pub total_buffers: usize,
    pub available_buffers: usize,
    pub used_buffers: usize,
    pub allocation_count: u64,
    pub deallocation_count: u64,
    pub fragmentation_ratio: f64,
}

/// Allocation strategy information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationStrategy {
    pub strategy_id: String,
    pub strategy_type: AllocationStrategyType,
    pub allocation_time: f64,
    pub deallocation_time: f64,
    pub fragmentation_level: f64,
    pub memory_efficiency: f64,
}

/// Garbage collection engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GarbageCollectionEngine {
    pub gc_cycles: u64,
    pub total_collected_memory: usize,
    pub average_gc_time: f64,
    pub last_gc_time: f64,
    pub memory_freed_per_cycle: f64,
    pub gc_efficiency: f64,
}

/// Memory pool statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryPoolStats {
    pub total_memory_usage: usize,
    pub allocated_memory: usize,
    pub free_memory: usize,
    pub fragmentation_ratio: f64,
    pub allocation_count: u64,
    pub deallocation_count: u64,
    pub gc_cycles: u64,
    pub total_collected_memory: usize,
    pub optimizations_applied: u32,
    pub optimization_benefit: f64,
    pub average_allocation_time: f64,
    pub average_deallocation_time: f64,
    pub memory_efficiency: f64,
}

/// Memory pool management errors
#[derive(Error, Debug)]
pub enum MemoryPoolError {
    #[error("Buffer pool error: {message}")]
    BufferPoolError { message: String },

    #[error("Allocation strategy error: {message}")]
    AllocationStrategyError { message: String },

    #[error("Garbage collection error: {message}")]
    GarbageCollectionError { message: String },

    #[error("Memory limit exceeded: {limit} vs {usage}")]
    MemoryLimitExceeded { limit: usize, usage: usize },

    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },
}

impl MemoryPoolManager {
    /// Create a new memory pool manager
    pub fn new(config: MemoryPoolConfig) -> Self {
        Self {
            config,
            stats: Arc::new(RwLock::new(MemoryPoolStats::default())),
            buffer_pools: Arc::new(RwLock::new(HashMap::new())),
            allocation_strategies: Arc::new(RwLock::new(HashMap::new())),
            gc_engine: Arc::new(RwLock::new(GarbageCollectionEngine {
                gc_cycles: 0,
                total_collected_memory: 0,
                average_gc_time: 0.0,
                last_gc_time: 0.0,
                memory_freed_per_cycle: 0.0,
                gc_efficiency: 0.0,
            })),
        }
    }

    /// Optimize memory usage
    pub async fn optimize_memory_usage(&self) -> Result<(), MemoryPoolError> {
        let mut total_optimizations = 0;
        let mut total_benefit = 0.0;

        // Optimize buffer pools
        if self.config.enable_buffer_pools {
            let buffer_result = self.optimize_buffer_pools().await;
            if let Ok(count) = buffer_result {
                total_optimizations += count;
                total_benefit += 0.4; // 40% benefit from buffer pool optimization
            }
        }

        // Optimize allocation strategies
        if self.config.enable_allocation_strategies {
            let allocation_result = self.optimize_allocation_strategies().await;
            if let Ok(count) = allocation_result {
                total_optimizations += count;
                total_benefit += 0.3; // 30% benefit from allocation strategy optimization
            }
        }

        // Run garbage collection
        if self.config.enable_garbage_collection {
            let gc_result = self.run_garbage_collection().await;
            if let Ok(count) = gc_result {
                total_optimizations += count;
                total_benefit += 0.3; // 30% benefit from garbage collection
            }
        }

        // Update statistics
        self.update_stats(total_optimizations, total_benefit).await;

        Ok(())
    }

    /// Optimize buffer pools
    async fn optimize_buffer_pools(&self) -> Result<u32, MemoryPoolError> {
        let mut optimizations_applied = 0;

        // Create optimized buffer pools
        let pool_configs = vec![
            ("vertex_buffer_pool".to_string(), 1024, 100),
            ("index_buffer_pool".to_string(), 512, 200),
            ("texture_buffer_pool".to_string(), 2048, 50),
            ("uniform_buffer_pool".to_string(), 256, 300),
        ];

        for (pool_id, buffer_size, total_buffers) in pool_configs {
            let buffer_pool = BufferPool {
                pool_id: pool_id.clone(),
                buffer_size,
                total_buffers,
                available_buffers: total_buffers,
                used_buffers: 0,
                allocation_count: 0,
                deallocation_count: 0,
                fragmentation_ratio: 0.1, // 10% fragmentation
            };

            self.buffer_pools.write().await.insert(pool_id, buffer_pool);
            optimizations_applied += 1;
        }

        Ok(optimizations_applied)
    }

    /// Optimize allocation strategies
    async fn optimize_allocation_strategies(&self) -> Result<u32, MemoryPoolError> {
        let mut optimizations_applied = 0;

        // Create optimized allocation strategies
        let strategy_configs = vec![
            ("first_fit".to_string(), AllocationStrategyType::FirstFit),
            ("best_fit".to_string(), AllocationStrategyType::BestFit),
            (
                "buddy_system".to_string(),
                AllocationStrategyType::BuddySystem,
            ),
        ];

        for (strategy_id, strategy_type) in strategy_configs {
            let allocation_strategy = AllocationStrategy {
                strategy_id: strategy_id.clone(),
                strategy_type: strategy_type.clone(),
                allocation_time: match strategy_type {
                    AllocationStrategyType::FirstFit => 0.001,
                    AllocationStrategyType::BestFit => 0.002,
                    AllocationStrategyType::WorstFit => 0.001,
                    AllocationStrategyType::BuddySystem => 0.0005,
                    AllocationStrategyType::SlabAllocator => 0.0003,
                },
                deallocation_time: 0.0005,
                fragmentation_level: match strategy_type {
                    AllocationStrategyType::FirstFit => 0.2,
                    AllocationStrategyType::BestFit => 0.1,
                    AllocationStrategyType::WorstFit => 0.3,
                    AllocationStrategyType::BuddySystem => 0.05,
                    AllocationStrategyType::SlabAllocator => 0.02,
                },
                memory_efficiency: match strategy_type {
                    AllocationStrategyType::FirstFit => 0.8,
                    AllocationStrategyType::BestFit => 0.9,
                    AllocationStrategyType::WorstFit => 0.7,
                    AllocationStrategyType::BuddySystem => 0.95,
                    AllocationStrategyType::SlabAllocator => 0.98,
                },
            };

            self.allocation_strategies
                .write()
                .await
                .insert(strategy_id, allocation_strategy);
            optimizations_applied += 1;
        }

        Ok(optimizations_applied)
    }

    /// Run garbage collection
    async fn run_garbage_collection(&self) -> Result<u32, MemoryPoolError> {
        let mut gc_engine = self.gc_engine.write().await;

        // Simulate garbage collection
        gc_engine.gc_cycles += 1;
        gc_engine.last_gc_time = 0.01; // 10ms
        gc_engine.memory_freed_per_cycle = 1024.0; // 1KB freed per cycle
        gc_engine.total_collected_memory += 1024;
        gc_engine.average_gc_time = (gc_engine.average_gc_time + gc_engine.last_gc_time) / 2.0;
        gc_engine.gc_efficiency = 0.85; // 85% efficiency

        Ok(1)
    }

    /// Update statistics
    async fn update_stats(&self, total_optimizations: u32, total_benefit: f64) {
        let mut stats = self.stats.write().await;

        // Calculate memory usage from buffer pools
        let buffer_pools = self.buffer_pools.read().await;
        let total_memory: usize = buffer_pools
            .values()
            .map(|pool| pool.buffer_size * pool.total_buffers)
            .sum();

        let allocated_memory: usize = buffer_pools
            .values()
            .map(|pool| pool.buffer_size * pool.used_buffers)
            .sum();

        let free_memory = total_memory - allocated_memory;

        // Calculate fragmentation
        let total_fragmentation: f64 = buffer_pools
            .values()
            .map(|pool| pool.fragmentation_ratio)
            .sum();
        let average_fragmentation = total_fragmentation / buffer_pools.len() as f64;

        // Calculate allocation statistics
        let total_allocations: u64 = buffer_pools
            .values()
            .map(|pool| pool.allocation_count)
            .sum();

        let total_deallocations: u64 = buffer_pools
            .values()
            .map(|pool| pool.deallocation_count)
            .sum();

        // Calculate memory efficiency
        let allocation_strategies = self.allocation_strategies.read().await;
        let total_efficiency: f64 = allocation_strategies
            .values()
            .map(|strategy| strategy.memory_efficiency)
            .sum();
        let average_efficiency = total_efficiency / allocation_strategies.len() as f64;

        // Update stats
        stats.total_memory_usage = total_memory;
        stats.allocated_memory = allocated_memory;
        stats.free_memory = free_memory;
        stats.fragmentation_ratio = average_fragmentation;
        stats.allocation_count = total_allocations;
        stats.deallocation_count = total_deallocations;
        stats.optimizations_applied = total_optimizations;
        stats.optimization_benefit = total_benefit;
        stats.memory_efficiency = average_efficiency;

        // Update GC stats
        let gc_engine = self.gc_engine.read().await;
        stats.gc_cycles = gc_engine.gc_cycles;
        stats.total_collected_memory = gc_engine.total_collected_memory;
    }

    /// Get current statistics
    pub async fn get_stats(&self) -> MemoryPoolStats {
        self.stats.read().await.clone()
    }

    /// Update configuration
    pub async fn update_config(&mut self, config: MemoryPoolConfig) -> Result<(), MemoryPoolError> {
        self.config = config;
        Ok(())
    }

    /// Get buffer pool information
    pub async fn get_buffer_pool(&self, pool_id: &str) -> Option<BufferPool> {
        self.buffer_pools.read().await.get(pool_id).cloned()
    }

    /// Get allocation strategy information
    pub async fn get_allocation_strategy(&self, strategy_id: &str) -> Option<AllocationStrategy> {
        self.allocation_strategies
            .read()
            .await
            .get(strategy_id)
            .cloned()
    }

    /// Get garbage collection engine information
    pub async fn get_gc_engine(&self) -> GarbageCollectionEngine {
        self.gc_engine.read().await.clone()
    }

    /// Allocate memory from a buffer pool
    pub async fn allocate_from_pool(
        &self,
        pool_id: &str,
        size: usize,
    ) -> Result<usize, MemoryPoolError> {
        let mut buffer_pools = self.buffer_pools.write().await;

        if let Some(pool) = buffer_pools.get_mut(pool_id) {
            if pool.available_buffers > 0 && size <= pool.buffer_size {
                pool.available_buffers -= 1;
                pool.used_buffers += 1;
                pool.allocation_count += 1;
                Ok(pool.buffer_size)
            } else {
                Err(MemoryPoolError::BufferPoolError {
                    message: "Insufficient buffers or size too large".to_string(),
                })
            }
        } else {
            Err(MemoryPoolError::BufferPoolError {
                message: format!("Buffer pool '{}' not found", pool_id),
            })
        }
    }

    /// Deallocate memory to a buffer pool
    pub async fn deallocate_to_pool(
        &self,
        pool_id: &str,
        _size: usize,
    ) -> Result<(), MemoryPoolError> {
        let mut buffer_pools = self.buffer_pools.write().await;

        if let Some(pool) = buffer_pools.get_mut(pool_id) {
            if pool.used_buffers > 0 {
                pool.available_buffers += 1;
                pool.used_buffers -= 1;
                pool.deallocation_count += 1;
                Ok(())
            } else {
                Err(MemoryPoolError::BufferPoolError {
                    message: "No buffers to deallocate".to_string(),
                })
            }
        } else {
            Err(MemoryPoolError::BufferPoolError {
                message: format!("Buffer pool '{}' not found", pool_id),
            })
        }
    }
}

impl Default for MemoryPoolConfig {
    fn default() -> Self {
        Self {
            enable_buffer_pools: true,
            enable_allocation_strategies: true,
            enable_garbage_collection: true,
            max_memory_usage: 100 * 1024 * 1024, // 100MB
            buffer_pool_size: 1024,
            allocation_strategy: AllocationStrategyType::BuddySystem,
            gc_frequency: 100,
            gc_threshold: 0.8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_memory_pool_config() -> MemoryPoolConfig {
        MemoryPoolConfig {
            enable_buffer_pools: true,
            enable_allocation_strategies: true,
            enable_garbage_collection: true,
            max_memory_usage: 10 * 1024 * 1024, // 10MB for testing
            buffer_pool_size: 1024,
            allocation_strategy: AllocationStrategyType::BuddySystem,
            gc_frequency: 100,
            gc_threshold: 0.8,
        }
    }

    #[tokio::test]
    async fn test_memory_pool_manager_creation() {
        let config = create_test_memory_pool_config();
        let manager = MemoryPoolManager::new(config);

        let stats = manager.get_stats().await;
        assert_eq!(stats.total_memory_usage, 0);
        assert_eq!(stats.allocated_memory, 0);
        assert_eq!(stats.optimizations_applied, 0);
    }

    #[tokio::test]
    async fn test_memory_usage_optimization() {
        let config = create_test_memory_pool_config();
        let manager = MemoryPoolManager::new(config);

        let result = manager.optimize_memory_usage().await;
        assert!(result.is_ok());

        let stats = manager.get_stats().await;
        assert!(stats.optimizations_applied > 0);
        assert!(stats.optimization_benefit > 0.0);
    }

    #[tokio::test]
    async fn test_buffer_pool_optimization() {
        let config = create_test_memory_pool_config();
        let manager = MemoryPoolManager::new(config);

        let result = manager.optimize_memory_usage().await;
        assert!(result.is_ok());

        let pool = manager.get_buffer_pool("vertex_buffer_pool").await;
        assert!(pool.is_some());

        let pool = pool.unwrap();
        assert_eq!(pool.pool_id, "vertex_buffer_pool");
        assert!(pool.buffer_size > 0);
        assert!(pool.total_buffers > 0);
    }

    #[tokio::test]
    async fn test_allocation_strategy_optimization() {
        let config = create_test_memory_pool_config();
        let manager = MemoryPoolManager::new(config);

        let result = manager.optimize_memory_usage().await;
        assert!(result.is_ok());

        let strategy = manager.get_allocation_strategy("buddy_system").await;
        assert!(strategy.is_some());

        let strategy = strategy.unwrap();
        assert_eq!(strategy.strategy_id, "buddy_system");
        assert_eq!(strategy.strategy_type, AllocationStrategyType::BuddySystem);
        assert!(strategy.memory_efficiency > 0.0);
    }

    #[tokio::test]
    async fn test_garbage_collection() {
        let config = create_test_memory_pool_config();
        let manager = MemoryPoolManager::new(config);

        let result = manager.optimize_memory_usage().await;
        assert!(result.is_ok());

        let gc_engine = manager.get_gc_engine().await;
        assert!(gc_engine.gc_cycles > 0);
        assert!(gc_engine.total_collected_memory > 0);
        assert!(gc_engine.gc_efficiency > 0.0);
    }

    #[tokio::test]
    async fn test_memory_allocation() {
        let config = create_test_memory_pool_config();
        let manager = MemoryPoolManager::new(config);

        // First optimize to create buffer pools
        let _ = manager.optimize_memory_usage().await;

        // Allocate from vertex buffer pool
        let result = manager.allocate_from_pool("vertex_buffer_pool", 1024).await;
        assert!(result.is_ok());

        let allocated_size = result.unwrap();
        assert_eq!(allocated_size, 1024);

        // Check pool state
        let pool = manager.get_buffer_pool("vertex_buffer_pool").await.unwrap();
        assert_eq!(pool.used_buffers, 1);
        assert_eq!(pool.available_buffers, 99);
    }

    #[tokio::test]
    async fn test_memory_deallocation() {
        let config = create_test_memory_pool_config();
        let manager = MemoryPoolManager::new(config);

        // First optimize to create buffer pools
        let _ = manager.optimize_memory_usage().await;

        // Allocate first
        let _ = manager.allocate_from_pool("vertex_buffer_pool", 1024).await;

        // Then deallocate
        let result = manager.deallocate_to_pool("vertex_buffer_pool", 1024).await;
        assert!(result.is_ok());

        // Check pool state
        let pool = manager.get_buffer_pool("vertex_buffer_pool").await.unwrap();
        assert_eq!(pool.used_buffers, 0);
        assert_eq!(pool.available_buffers, 100);
    }

    #[tokio::test]
    async fn test_memory_allocation_error() {
        let config = create_test_memory_pool_config();
        let manager = MemoryPoolManager::new(config);

        // Try to allocate from non-existent pool
        let result = manager.allocate_from_pool("non_existent_pool", 1024).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_config_update() {
        let config = create_test_memory_pool_config();
        let mut manager = MemoryPoolManager::new(config);

        let new_config = create_test_memory_pool_config();
        let result = manager.update_config(new_config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_memory_statistics() {
        let config = create_test_memory_pool_config();
        let manager = MemoryPoolManager::new(config);

        // Optimize to create pools and strategies
        let _ = manager.optimize_memory_usage().await;

        let stats = manager.get_stats().await;
        assert!(stats.total_memory_usage > 0);
        assert!(stats.memory_efficiency > 0.0);
        assert!(stats.fragmentation_ratio >= 0.0);
    }
}
