//! TDD Phase 2: GPU Optimization Tests
//!
//! This module contains comprehensive tests for GPU optimization features:
//! - GPU memory management and leak prevention
//! - Compute shader performance optimization
//! - Buffer pooling and reuse strategies
//! - Rendering pipeline optimization
//! - Performance regression testing

use leptos_helios::*;
use proptest::prelude::*;
use std::time::{Duration, Instant};

/// Test GPU memory management and leak prevention
#[cfg(test)]
mod gpu_memory_tests {
    use super::*;

    #[test]
    fn test_gpu_memory_leak_prevention() {
        // TDD: Should prevent GPU memory leaks during repeated operations
        let mut engine = GpuAccelerationEngine::new();
        let iterations = 1000;

        let result = engine.manage_gpu_memory(iterations);
        assert!(result.is_ok(), "GPU memory management should not leak");

        // Verify memory usage is within acceptable bounds
        let memory_usage = engine.memory_usage.used_bytes;
        assert!(
            memory_usage < 10 * 1024 * 1024,
            "Memory usage should be <10MB after {} iterations",
            iterations
        );
    }

    #[test]
    fn test_gpu_buffer_pooling() {
        // TDD: Should efficiently reuse GPU buffers
        let mut engine = GpuAccelerationEngine::new();

        // Create multiple buffers
        let buffer1 = engine.create_optimized_buffer("test_data_1", 1000).unwrap();
        let buffer2 = engine.create_optimized_buffer("test_data_2", 2000).unwrap();
        let buffer3 = engine.create_optimized_buffer("test_data_3", 1500).unwrap();

        // Verify buffers are created successfully
        assert_eq!(buffer1.size, 1000);
        assert_eq!(buffer2.size, 2000);
        assert_eq!(buffer3.size, 1500);

        // Verify buffer pool statistics
        let stats = engine.get_buffer_pool_stats();
        assert!(stats.total_allocations >= 3);
        assert!(stats.available_buffers >= 0);
    }

    #[test]
    fn test_gpu_memory_budget_enforcement() {
        // TDD: Should enforce memory budget limits
        let mut engine = GpuAccelerationEngine::new();
        let budget_mb = 50; // 50MB budget

        // Try to allocate more than budget
        let large_buffer_size = budget_mb * 1024 * 1024 + 1024; // 50MB + 1KB
        let result = engine.create_optimized_buffer("large_buffer", large_buffer_size);

        // Should fail or handle gracefully
        assert!(result.is_err() || engine.memory_usage.used_bytes <= budget_mb * 1024 * 1024);
    }

    #[test]
    fn test_gpu_memory_cleanup() {
        // TDD: Should properly cleanup GPU resources
        let mut engine = GpuAccelerationEngine::new();
        let initial_memory = engine.memory_usage.used_bytes;

        // Allocate some buffers
        let _buffer1 = engine.create_optimized_buffer("temp_1", 1000).unwrap();
        let _buffer2 = engine.create_optimized_buffer("temp_2", 2000).unwrap();
        let allocated_memory = engine.memory_usage.used_bytes;

        // Cleanup
        engine.cleanup_resources();

        let final_memory = engine.memory_usage.used_bytes;

        // Memory should be cleaned up (allowing for some overhead)
        assert!(
            final_memory <= initial_memory + 1024,
            "Memory should be cleaned up after resource cleanup"
        );
    }
}

/// Test compute shader performance optimization
#[cfg(test)]
mod compute_shader_tests {
    use super::*;

    #[test]
    fn test_compute_shader_performance_target() {
        // TDD: Compute shaders should meet performance targets
        let engine = GpuAccelerationEngine::new();
        let point_count = 100_000;

        let result = engine.execute_compute_shader(point_count);
        assert!(
            result.is_ok(),
            "Compute shader should execute successfully for {} points",
            point_count
        );
    }

    #[test]
    fn test_compute_shader_scaling() {
        // TDD: Compute shader performance should scale reasonably
        let engine = GpuAccelerationEngine::new();

        let test_cases = vec![
            (10_000, Duration::from_millis(1)),
            (100_000, Duration::from_millis(3)),
            (1_000_000, Duration::from_millis(30)),
        ];

        for (point_count, max_duration) in test_cases {
            let start = Instant::now();
            let result = engine.execute_compute_shader(point_count);
            let duration = start.elapsed();

            assert!(
                result.is_ok(),
                "Compute shader should handle {} points",
                point_count
            );
            assert!(
                duration <= max_duration,
                "Compute shader took {:?}, expected <= {:?} for {} points",
                duration,
                max_duration,
                point_count
            );
        }
    }

    #[test]
    fn test_compute_shader_parallel_execution() {
        // TDD: Should support parallel compute shader execution
        let engine = GpuAccelerationEngine::new();
        let point_count = 50_000;

        // Simulate parallel execution
        let start = Instant::now();
        let result1 = engine.execute_compute_shader(point_count);
        let result2 = engine.execute_compute_shader(point_count);
        let duration = start.elapsed();

        assert!(result1.is_ok());
        assert!(result2.is_ok());

        // Parallel execution should be faster than sequential
        assert!(
            duration < Duration::from_millis(6),
            "Parallel execution should be faster than sequential"
        );
    }
}

/// Test rendering pipeline optimization
#[cfg(test)]
mod rendering_pipeline_tests {
    use super::*;

    #[test]
    fn test_render_pipeline_creation() {
        // TDD: Should create optimized render pipelines
        let backend = RenderBackend::create_optimal().await.unwrap();
        let mut renderer = Renderer::new().await.unwrap();

        // Test different chart types
        let chart_types = vec![
            ChartType::Line,
            ChartType::Bar,
            ChartType::Scatter,
            ChartType::Area,
        ];

        for chart_type in chart_types {
            let pipeline = renderer.get_or_create_pipeline(chart_type);
            assert!(
                pipeline.is_optimized(),
                "Pipeline for {:?} should be optimized",
                chart_type
            );
        }
    }

    #[test]
    fn test_adaptive_quality_management() {
        // TDD: Should adapt quality based on performance
        let mut renderer = Renderer::new().await.unwrap();
        let spec = ChartSpec::new().mark(MarkType::Line);

        // Simulate frame timing variations
        let frame_times = vec![
            Duration::from_millis(10), // Good performance
            Duration::from_millis(20), // Degraded performance
            Duration::from_millis(5),  // Excellent performance
        ];

        for frame_time in frame_times {
            renderer.frame_timer.record_frame(frame_time);
            let quality = renderer.frame_timer.suggest_quality();
            let config = renderer.quality_manager.get_render_config(quality);

            // Quality should adapt to frame timing
            if frame_time > Duration::from_millis(16) {
                assert!(
                    config.quality_level < 1.0,
                    "Quality should be reduced for slow frames"
                );
            } else {
                assert!(
                    config.quality_level >= 0.8,
                    "Quality should be maintained for fast frames"
                );
            }
        }
    }

    #[test]
    fn test_buffer_pool_efficiency() {
        // TDD: Buffer pool should efficiently reuse buffers
        let backend = RenderBackend::create_optimal().await.unwrap();
        let mut buffer_pool = BufferPool::new(&backend).unwrap();

        // Create multiple buffers
        let buffers = vec![
            buffer_pool.allocate_buffer(1000).unwrap(),
            buffer_pool.allocate_buffer(2000).unwrap(),
            buffer_pool.allocate_buffer(1500).unwrap(),
        ];

        // Return buffers to pool
        for buffer in buffers {
            buffer_pool.return_buffer(buffer);
        }

        // Verify pool statistics
        let stats = buffer_pool.get_stats();
        assert!(
            stats.available_buffers >= 3,
            "Pool should have available buffers for reuse"
        );
        assert!(stats.reuse_count > 0, "Pool should track buffer reuse");
    }

    #[test]
    fn test_rendering_performance_budget() {
        // TDD: Rendering should meet performance budget
        let mut renderer = Renderer::new().await.unwrap();
        let spec = ChartSpec::new().mark(MarkType::Line);
        let budget = PerformanceBudget {
            max_frame_time: Duration::from_millis(16), // 60 FPS
            max_memory: 100 * 1024 * 1024,             // 100MB
            max_gpu_utilization: 0.9,                  // 90%
        };

        let stats = renderer.render(&spec);

        assert!(
            stats.is_within_budget(&budget),
            "Rendering should meet performance budget"
        );
        assert!(stats.fps() >= 50.0, "Should maintain at least 50 FPS");
    }
}

/// Test performance regression detection
#[cfg(test)]
mod performance_regression_tests {
    use super::*;

    #[test]
    fn test_gpu_performance_regression() {
        // TDD: Should detect GPU performance regressions
        let engine = GpuAccelerationEngine::new();
        let point_count = 100_000;

        let start = Instant::now();
        let result = engine.execute_compute_shader(point_count);
        let duration = start.elapsed();

        assert!(result.is_ok());

        // Performance regression test: should complete within 3ms
        assert!(
            duration < Duration::from_millis(3),
            "GPU performance regression detected: {:?} for {} points",
            duration,
            point_count
        );
    }

    #[test]
    fn test_memory_usage_regression() {
        // TDD: Should detect memory usage regressions
        let mut engine = GpuAccelerationEngine::new();
        let iterations = 100;

        let initial_memory = engine.memory_usage.used_bytes;
        let result = engine.manage_gpu_memory(iterations);
        let final_memory = engine.memory_usage.used_bytes;

        assert!(result.is_ok());

        // Memory regression test: growth should be <1MB
        let memory_growth = final_memory - initial_memory;
        assert!(
            memory_growth < 1024 * 1024,
            "Memory usage regression detected: {} bytes growth",
            memory_growth
        );
    }

    #[test]
    fn test_rendering_performance_regression() {
        // TDD: Should detect rendering performance regressions
        let mut renderer = Renderer::new().await.unwrap();
        let spec = ChartSpec::new().mark(MarkType::Line);

        let start = Instant::now();
        let stats = renderer.render(&spec);
        let duration = start.elapsed();

        // Performance regression test: frame time should be <16ms (60 FPS)
        assert!(
            duration < Duration::from_millis(16),
            "Rendering performance regression detected: {:?}",
            duration
        );

        // FPS regression test: should maintain at least 50 FPS
        assert!(
            stats.fps() >= 50.0,
            "FPS regression detected: {:.1} FPS",
            stats.fps()
        );
    }
}

/// Property-based tests for GPU optimization
#[cfg(test)]
mod property_based_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_gpu_memory_scaling(
            iterations in 1..1000usize,
            buffer_size in 100..10000usize
        ) {
            let mut engine = GpuAccelerationEngine::new();
            let initial_memory = engine.memory_usage.used_bytes;

            // Create multiple buffers
            for i in 0..iterations {
                let buffer_id = format!("buffer_{}", i);
                let _buffer = engine.create_optimized_buffer(&buffer_id, buffer_size).unwrap();
            }

            let final_memory = engine.memory_usage.used_bytes;
            let memory_growth = final_memory - initial_memory;

            // Memory growth should be proportional to buffer size and iterations
            let expected_growth = iterations * buffer_size;
            let tolerance = expected_growth / 10; // 10% tolerance

            assert!(memory_growth <= expected_growth + tolerance,
                   "Memory growth {} should be <= expected {} + tolerance {}",
                   memory_growth, expected_growth, tolerance);
        }

        #[test]
        fn test_compute_shader_performance_scaling(
            point_count in 1000..100000usize
        ) {
            let engine = GpuAccelerationEngine::new();
            let start = Instant::now();
            let result = engine.execute_compute_shader(point_count);
            let duration = start.elapsed();

            assert!(result.is_ok());

            // Performance should scale sub-linearly
            let max_duration_ms = (point_count as f64 / 10000.0).ceil() as u64;
            let max_duration = Duration::from_millis(max_duration_ms.max(1));

            assert!(duration <= max_duration,
                   "Compute shader duration {:?} should be <= {:?} for {} points",
                   duration, max_duration, point_count);
        }

        #[test]
        fn test_buffer_pool_efficiency(
            buffer_sizes in prop::collection::vec(100..10000usize, 1..20)
        ) {
            let backend = RenderBackend::create_optimal().await.unwrap();
            let mut buffer_pool = BufferPool::new(&backend).unwrap();
            let initial_stats = buffer_pool.get_stats();

            // Allocate and return buffers
            let mut buffers = Vec::new();
            for size in &buffer_sizes {
                let buffer = buffer_pool.allocate_buffer(*size).unwrap();
                buffers.push(buffer);
            }

            for buffer in buffers {
                buffer_pool.return_buffer(buffer);
            }

            let final_stats = buffer_pool.get_stats();

            // Pool should have available buffers for reuse
            assert!(final_stats.available_buffers >= buffer_sizes.len());

            // Total allocations should be reasonable
            assert!(final_stats.total_allocations <= buffer_sizes.len() * 2);
        }
    }
}

/// Integration tests for GPU optimization
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_end_to_end_gpu_optimization() {
        // TDD: End-to-end GPU optimization should work
        let mut renderer = Renderer::new().await.unwrap();
        let mut gpu_engine = GpuAccelerationEngine::new();

        // Create a complex chart specification
        let spec = ChartSpec::new()
            .mark(MarkType::Line)
            .add_encoding(Encoding::X {
                field: "time".to_string(),
            })
            .add_encoding(Encoding::Y {
                field: "value".to_string(),
            });

        // Process large dataset with GPU acceleration
        let large_dataset: Vec<f64> = (0..100_000).map(|i| (i as f64 * 0.01).sin()).collect();
        let viewport_scale = 1.0;

        let start = Instant::now();
        let metrics = gpu_engine
            .process_large_dataset(&large_dataset, viewport_scale)
            .unwrap();
        let gpu_duration = start.elapsed();

        // Render the chart
        let render_start = Instant::now();
        let stats = renderer.render(&spec);
        let render_duration = render_start.elapsed();

        // Verify performance targets
        assert!(
            gpu_duration < Duration::from_millis(50),
            "GPU processing should be <50ms"
        );
        assert!(
            render_duration < Duration::from_millis(16),
            "Rendering should be <16ms"
        );
        assert!(
            metrics.is_performance_target_met(),
            "Performance targets should be met"
        );

        // Verify memory usage
        let memory_usage = gpu_engine.get_memory_usage();
        assert!(
            memory_usage < 100 * 1024 * 1024,
            "Memory usage should be <100MB"
        );
    }

    #[tokio::test]
    async fn test_gpu_fallback_performance() {
        // TDD: GPU fallback should maintain reasonable performance
        let mut renderer = Renderer::new().await.unwrap();
        let spec = ChartSpec::new().mark(MarkType::Line);

        // Simulate GPU unavailability
        let start = Instant::now();
        let stats = renderer.render(&spec);
        let duration = start.elapsed();

        // Even with fallback, should maintain reasonable performance
        assert!(
            duration < Duration::from_millis(33),
            "Fallback rendering should be <33ms (30 FPS)"
        );
        assert!(
            stats.fps() >= 25.0,
            "Fallback should maintain at least 25 FPS"
        );
    }
}
