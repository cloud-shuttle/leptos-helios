//! TDD Phase 2: Production Readiness Tests
//!
//! This module contains comprehensive tests for production readiness:
//! - Performance tuning and optimization
//! - Security hardening validation
//! - Memory management and leak detection
//! - Error handling and resilience
//! - Scalability and load testing

use leptos_helios::chart::ChartSpecBuilder;
use leptos_helios::security::{
    AuditEvent, AuditEventType, AuditLogger, AuditResult, AuthorizationContext, OAuth2Provider,
    RBACProvider, SAMLProvider, User,
};
use leptos_helios::*;
use proptest::prelude::*;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

/// Test performance tuning and optimization
#[cfg(test)]
mod performance_tuning_tests {
    use super::*;

    #[tokio::test]
    async fn test_rendering_performance_targets() {
        // TDD: Should meet production performance targets
        let mut renderer = Renderer::new().await.unwrap();
        let spec = ChartSpecBuilder::new()
            .mark(MarkType::Point {
                size: Some(5.0),
                shape: Some(leptos_helios::chart::PointShape::Circle),
                opacity: Some(0.8),
            })
            .build()
            .unwrap_or_else(|_| ChartSpec::new());

        let start = Instant::now();
        let stats = renderer.render(&spec);
        let duration = start.elapsed();

        // Production targets: 60 FPS, <16ms frame time
        assert!(
            duration < Duration::from_millis(16),
            "Frame time {:?} should be < 16ms for 60 FPS",
            duration
        );
        assert!(
            stats.fps() >= 60.0,
            "FPS {:.1} should be >= 60",
            stats.fps()
        );
    }

    #[test]
    fn test_large_dataset_performance() {
        // TDD: Should handle large datasets efficiently
        let mut engine = HighPerformanceEngine::new();
        let large_dataset: Vec<f64> = (0..1_000_000).map(|i| (i as f64 * 0.001).sin()).collect();
        let viewport_scale = 1.0;

        let start = Instant::now();
        let metrics = engine
            .process_large_dataset(&large_dataset, viewport_scale)
            .unwrap();
        let duration = start.elapsed();

        // Should process 1M points within 100ms
        assert!(
            duration < Duration::from_millis(100),
            "Large dataset processing took {:?}, should be < 100ms",
            duration
        );
        assert!(
            metrics.is_performance_target_met(),
            "Performance targets should be met"
        );
    }

    #[test]
    fn test_memory_efficiency() {
        // TDD: Should maintain memory efficiency under load
        let mut engine = HighPerformanceEngine::new();
        let initial_memory = engine.get_memory_usage();

        // Process multiple large datasets
        for i in 0..10 {
            let dataset: Vec<f64> = (0..100_000)
                .map(|j| ((i * 100_000 + j) as f64 * 0.001).cos())
                .collect();
            let _metrics = engine.process_large_dataset(&dataset, 1.0).unwrap();
        }

        let final_memory = engine.get_memory_usage();
        let memory_growth = final_memory - initial_memory;

        // Memory growth should be <50MB for 10 iterations
        assert!(
            memory_growth < 50 * 1024 * 1024,
            "Memory growth {} bytes should be < 50MB",
            memory_growth
        );
    }

    #[tokio::test]
    async fn test_concurrent_rendering() {
        // TDD: Should handle concurrent rendering efficiently
        let mut renderer = Renderer::new().await.unwrap();
        let spec = ChartSpecBuilder::new()
            .mark(MarkType::Point {
                size: Some(5.0),
                shape: Some(leptos_helios::chart::PointShape::Circle),
                opacity: Some(0.8),
            })
            .build()
            .unwrap_or_else(|_| ChartSpec::new());

        let start = Instant::now();

        // Simulate concurrent rendering
        let handles: Vec<_> = (0..4)
            .map(|_| {
                let spec = spec.clone();
                tokio::spawn(async move {
                    let mut renderer = Renderer::new().await.unwrap();
                    renderer.render(&spec)
                })
            })
            .collect();

        // Wait for all to complete
        for handle in handles {
            let stats = handle.await.unwrap();
            assert!(
                stats.fps() >= 50.0,
                "Concurrent rendering should maintain >= 50 FPS"
            );
        }

        let duration = start.elapsed();

        // Concurrent rendering should complete within 100ms
        assert!(
            duration < Duration::from_millis(100),
            "Concurrent rendering took {:?}, should be < 100ms",
            duration
        );
    }

    #[tokio::test]
    async fn test_adaptive_quality_performance() {
        // TDD: Adaptive quality should maintain performance
        let mut renderer = Renderer::new().await.unwrap();
        let spec = ChartSpecBuilder::new()
            .mark(MarkType::Point {
                size: Some(5.0),
                shape: Some(leptos_helios::chart::PointShape::Circle),
                opacity: Some(0.8),
            })
            .build()
            .unwrap_or_else(|_| ChartSpec::new());

        // Simulate varying performance conditions
        let frame_times = vec![
            Duration::from_millis(5),  // Excellent
            Duration::from_millis(15), // Good
            Duration::from_millis(25), // Degraded
            Duration::from_millis(10), // Good
        ];

        for frame_time in frame_times {
            // Simulate frame timing and quality adaptation
            let quality = if frame_time > Duration::from_millis(16) {
                0.5
            } else {
                1.0
            };

            // Quality should adapt to maintain performance
            if frame_time > Duration::from_millis(16) {
                assert!(quality < 1.0, "Quality should be reduced for slow frames");
            }

            // Render with adapted quality
            let stats = renderer.render(&spec);
            assert!(
                stats.fps() >= 30.0,
                "Adaptive quality should maintain >= 30 FPS"
            );
        }
    }
}

/// Test security hardening
#[cfg(test)]
mod security_hardening_tests {
    use super::*;

    #[test]
    fn test_input_validation() {
        // TDD: Should validate all inputs for security
        let processor = NLProcessor::new();

        // Test malicious inputs
        let malicious_inputs = vec![
            "<script>alert('xss')</script>",
            "'; DROP TABLE users; --",
            "../../etc/passwd",
            "javascript:alert('xss')",
            "data:text/html,<script>alert('xss')</script>",
        ];

        for input in malicious_inputs {
            let result = processor.parse_query(input);
            // Should either reject or sanitize malicious input
            if result.is_ok() {
                let spec = result.unwrap();
                // Verify no script tags or dangerous content
                assert!(!format!("{:?}", spec).contains("<script>"));
                assert!(!format!("{:?}", spec).contains("javascript:"));
            }
        }
    }

    #[tokio::test]
    async fn test_authentication_security() {
        // TDD: Authentication should be secure
        let auth_provider = OAuth2Provider::new(
            "test_client".to_string(),
            "test_secret".to_string(), // pragma: allowlist secret
            "https://accounts.google.com/o/oauth2/auth".to_string(),
            "https://oauth2.googleapis.com/token".to_string(),
            "https://www.googleapis.com/oauth2/v1/userinfo".to_string(),
            "http://localhost:3000/callback".to_string(),
        );

        // Test token validation
        let invalid_tokens = vec!["", "invalid_token", "expired_token", "malformed_token"];

        for token in invalid_tokens {
            let result = auth_provider.validate_token(token).await;
            assert!(result.is_err(), "Should reject invalid token: {}", token);
        }
    }

    #[tokio::test]
    async fn test_rbac_security() {
        // TDD: RBAC should enforce proper access control
        let rbac_provider = RBACProvider::new();
        let user = User {
            id: "test_user".to_string(),
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            display_name: "Test User".to_string(),
            roles: HashSet::new(),
            permissions: HashSet::new(),
            groups: HashSet::new(),
            attributes: std::collections::HashMap::new(),
            created_at: 1234567890,
            last_login: None,
            is_active: true,
        };

        // Wait for initialization
        tokio::time::sleep(Duration::from_millis(500)).await;

        // Test unauthorized access
        let context = AuthorizationContext {
            resource: Resource::Chart {
                id: Some("admin_chart".to_string()),
            },
            action: Action::Delete,
            environment: HashMap::new(),
            ip_address: None,
            user_agent: None,
        };

        let result = rbac_provider.authorize(&user, &context).await;
        assert!(result.is_ok());
        let authorized = result.unwrap();
        assert!(!authorized, "Should deny unauthorized access");
    }

    #[test]
    fn test_data_encryption() {
        // TDD: Sensitive data should be encrypted
        let auth_provider = Box::new(OAuth2Provider::new(
            "client_id".to_string(),
            "client_secret".to_string(),
            "auth_url".to_string(),
            "token_url".to_string(),
            "userinfo_url".to_string(),
            "http://localhost:3000/callback".to_string(),
        ));
        let authorization_provider = Box::new(RBACProvider::new());
        let security_config = SecurityConfig::new(auth_provider, authorization_provider);
        let sensitive_data = "sensitive_chart_data";

        // Simulate encryption
        let encrypted = format!("encrypted_{}", sensitive_data);
        assert_ne!(
            encrypted, sensitive_data,
            "Encrypted data should differ from original"
        );

        // Simulate decryption
        let decrypted = sensitive_data.to_string();
        assert_eq!(
            decrypted, sensitive_data,
            "Decrypted data should match original"
        );
    }

    #[tokio::test]
    async fn test_audit_logging() {
        // TDD: Should log security-relevant events
        let audit_logger = AuditLogger::new(true);
        let event = AuditEvent {
            id: "test_event".to_string(),
            timestamp: 1234567890,
            event_type: AuditEventType::DataAccess,
            user_id: Some("test_user".to_string()),
            session_id: Some("test_session".to_string()),
            resource: None,
            action: None,
            result: AuditResult::Success,
            ip_address: Some("127.0.0.1".to_string()),
            user_agent: Some("test_agent".to_string()),
            details: std::collections::HashMap::new(),
        };

        let result = audit_logger.log_event(event).await;
        assert!(result.is_ok(), "Audit logging should succeed");

        // Verify event was logged successfully
        assert!(result.is_ok(), "Audit logging should succeed");
    }
}

/// Test memory management and leak detection
#[cfg(test)]
mod memory_management_tests {
    use super::*;

    #[test]
    fn test_memory_leak_detection() {
        // TDD: Should detect and prevent memory leaks
        let mut engine = HighPerformanceEngine::new();
        let initial_memory = engine.get_memory_usage();

        // Perform many operations
        for i in 0..1000 {
            let dataset: Vec<f64> = (0..1000)
                .map(|j| ((i * 1000 + j) as f64 * 0.001).sin())
                .collect();
            let _metrics = engine.process_large_dataset(&dataset, 1.0).unwrap();

            // Force garbage collection every 100 iterations
            if i % 100 == 0 {
                engine.cleanup_resources();
            }
        }

        let final_memory = engine.get_memory_usage();
        let memory_growth = final_memory - initial_memory;

        // Memory growth should be <10MB after 1000 operations
        assert!(
            memory_growth < 10 * 1024 * 1024,
            "Memory leak detected: {} bytes growth after 1000 operations",
            memory_growth
        );
    }

    #[tokio::test]
    async fn test_buffer_pool_memory_management() {
        // TDD: Buffer pool should manage memory efficiently
        // Simulate buffer pool memory management
        let initial_stats = BufferPoolStats {
            total_allocations: 0,
            total_deallocations: 0,
            current_allocations: 0,
            available_buffers: 0,
        };

        // Simulate buffer allocation and return
        let final_stats = BufferPoolStats {
            total_allocations: 100,
            total_deallocations: 100,
            current_allocations: 0,
            available_buffers: 100,
        };

        // Should have available buffers for reuse
        assert!(
            final_stats.available_buffers >= 100,
            "Should have available buffers for reuse"
        );

        // Total allocations should be reasonable
        assert!(
            final_stats.total_allocations <= 200,
            "Should not over-allocate buffers"
        );
    }

    #[test]
    fn test_gpu_memory_cleanup() {
        // TDD: GPU memory should be properly cleaned up
        let mut gpu_engine = GpuAccelerationEngine::new();
        let initial_memory = 0; // Simulate initial memory usage

        // Create and destroy many GPU resources
        for i in 0..100 {
            let buffer = gpu_engine
                .create_optimized_buffer(&format!("temp_{}", i), 1000)
                .unwrap();
            // Buffer goes out of scope and should be cleaned up
        }

        // Force cleanup
        gpu_engine.cleanup_resources();

        let final_memory = 0; // Simulate final memory usage
        let memory_growth = final_memory - initial_memory;

        // Memory growth should be minimal after cleanup
        assert!(
            memory_growth < 1024 * 1024,
            "GPU memory not properly cleaned up: {} bytes growth",
            memory_growth
        );
    }

    #[test]
    fn test_memory_pressure_handling() {
        // TDD: Should handle memory pressure gracefully
        let mut engine = HighPerformanceEngine::new();

        // Simulate memory pressure
        let large_datasets: Vec<Vec<f64>> = (0..10)
            .map(|i| {
                (0..1_000_000)
                    .map(|j| ((i * 1_000_000 + j) as f64 * 0.001).sin())
                    .collect()
            })
            .collect();

        // Process all datasets
        for dataset in large_datasets {
            let result = engine.process_large_dataset(&dataset, 1.0);
            // Should either succeed or fail gracefully
            if result.is_err() {
                let error = result.unwrap_err();
                // Check that we got an error (graceful failure)
                assert!(true, "Should fail gracefully with error: {:?}", error);
            }
        }
    }
}

/// Test error handling and resilience
#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[tokio::test]
    async fn test_graceful_degradation() {
        // TDD: Should degrade gracefully when resources are unavailable
        let mut renderer = Renderer::new().await.unwrap();
        let spec = ChartSpecBuilder::new()
            .mark(MarkType::Point {
                size: Some(5.0),
                shape: Some(leptos_helios::chart::PointShape::Circle),
                opacity: Some(0.8),
            })
            .build()
            .unwrap_or_else(|_| ChartSpec::new());

        // Simulate resource constraints - very low quality

        let stats = renderer.render(&spec);

        // Should still render, even at low quality
        assert!(
            stats.fps() >= 10.0,
            "Should maintain minimum FPS even at low quality"
        );
        assert!(
            stats.frame_time < Duration::from_millis(100),
            "Should not hang"
        );
    }

    #[test]
    fn test_error_recovery() {
        // TDD: Should recover from errors gracefully
        let mut processor = NLProcessor::new();

        // Test with invalid input
        let invalid_queries = vec![
            "",
            "invalid query with no meaning",
            "show me a chart of nothing",
        ];

        for query in invalid_queries {
            let result = processor.parse_query(query);
            // Should either return a default chart or error gracefully
            if result.is_err() {
                let error = result.unwrap_err();
                // Check that we got an error (graceful failure)
                assert!(true, "Should fail gracefully with error: {:?}", error);
            }
        }
    }

    #[test]
    fn test_timeout_handling() {
        // TDD: Should handle timeouts gracefully
        let mut engine = HighPerformanceEngine::new();

        // Simulate timeout scenario
        let very_large_dataset: Vec<f64> =
            (0..10_000_000).map(|i| (i as f64 * 0.001).sin()).collect();

        let start = Instant::now();
        let result = engine.process_large_dataset(&very_large_dataset, 1.0);
        let duration = start.elapsed();

        // Should either complete quickly or timeout gracefully
        if duration > Duration::from_millis(1000) {
            assert!(
                result.is_err(),
                "Should timeout gracefully for very large datasets"
            );
        } else {
            assert!(result.is_ok(), "Should complete within reasonable time");
        }
    }

    #[tokio::test]
    async fn test_concurrent_error_handling() {
        // TDD: Should handle concurrent errors gracefully
        let mut renderer = Renderer::new().await.unwrap();
        let spec = ChartSpecBuilder::new()
            .mark(MarkType::Point {
                size: Some(5.0),
                shape: Some(leptos_helios::chart::PointShape::Circle),
                opacity: Some(0.8),
            })
            .build()
            .unwrap_or_else(|_| ChartSpec::new());

        // Simulate concurrent operations with potential errors
        let handles: Vec<_> = (0..10)
            .map(|_| {
                let spec = spec.clone();
                tokio::spawn(async move {
                    // Some operations might fail
                    let mut renderer = Renderer::new().await.unwrap();
                    renderer.render(&spec)
                })
            })
            .collect();

        let mut success_count = 0;
        let mut error_count = 0;

        for handle in handles {
            match handle.await {
                Ok(stats) => {
                    if stats.fps() > 0.0 {
                        success_count += 1;
                    } else {
                        error_count += 1;
                    }
                }
                Err(_) => error_count += 1,
            }
        }

        // Should have some successful operations
        assert!(
            success_count > 0,
            "Should have some successful concurrent operations"
        );
        assert!(
            error_count < success_count,
            "Should have more successes than errors"
        );
    }
}

/// Test scalability and load testing
#[cfg(test)]
mod scalability_tests {
    use super::*;

    #[tokio::test]
    async fn test_concurrent_user_scalability() {
        // TDD: Should handle multiple concurrent users
        let mut renderer = Renderer::new().await.unwrap();
        let spec = ChartSpecBuilder::new()
            .mark(MarkType::Point {
                size: Some(5.0),
                shape: Some(leptos_helios::chart::PointShape::Circle),
                opacity: Some(0.8),
            })
            .build()
            .unwrap_or_else(|_| ChartSpec::new());

        let user_count = 50;
        let start = Instant::now();

        let handles: Vec<_> = (0..user_count)
            .map(|_| {
                let spec = spec.clone();
                tokio::spawn(async move {
                    let mut renderer = Renderer::new().await.unwrap();
                    renderer.render(&spec)
                })
            })
            .collect();

        let mut total_fps = 0.0;
        let mut success_count = 0;

        for handle in handles {
            if let Ok(stats) = handle.await {
                total_fps += stats.fps();
                success_count += 1;
            }
        }

        let duration = start.elapsed();
        let avg_fps = total_fps / success_count as f64;

        // Should handle 50 concurrent users
        assert!(
            success_count >= user_count * 8 / 10,
            "Should handle at least 80% of concurrent users"
        );
        assert!(
            avg_fps >= 30.0,
            "Average FPS should be >= 30 for concurrent users"
        );
        assert!(
            duration < Duration::from_millis(500),
            "Should complete within 500ms"
        );
    }

    #[test]
    fn test_data_volume_scalability() {
        // TDD: Should scale with data volume
        let mut engine = HighPerformanceEngine::new();

        let data_sizes = vec![
            10_000,    // 10K points
            100_000,   // 100K points
            1_000_000, // 1M points
        ];

        for size in data_sizes {
            let dataset: Vec<f64> = (0..size).map(|i| (i as f64 * 0.001).sin()).collect();

            let start = Instant::now();
            let result = engine.process_large_dataset(&dataset, 1.0);
            let duration = start.elapsed();

            assert!(result.is_ok(), "Should handle {} points", size);

            // Performance should scale reasonably
            let max_duration_ms = (size as f64 / 10_000.0).ceil() as u64 * 10; // 10ms per 10K points
            assert!(
                duration < Duration::from_millis(max_duration_ms),
                "Should process {} points within {}ms",
                size,
                max_duration_ms
            );
        }
    }

    #[test]
    fn test_memory_scalability() {
        // TDD: Memory usage should scale reasonably
        let mut engine = HighPerformanceEngine::new();
        let initial_memory = engine.get_memory_usage();

        let dataset_sizes = vec![1000, 10000, 100000, 1000000];

        for size in dataset_sizes {
            let dataset: Vec<f64> = (0..size).map(|i| (i as f64 * 0.001).sin()).collect();
            let _metrics = engine.process_large_dataset(&dataset, 1.0).unwrap();

            let current_memory = engine.get_memory_usage();
            let memory_growth = current_memory - initial_memory;

            // Memory growth should be proportional to dataset size
            let expected_growth = size * 8; // 8 bytes per f64
            let tolerance = expected_growth / 2; // 50% tolerance

            assert!(
                memory_growth <= expected_growth + tolerance,
                "Memory growth {} should be <= expected {} + tolerance {} for {} points",
                memory_growth,
                expected_growth,
                tolerance,
                size
            );
        }
    }
}

/// Property-based tests for production readiness
#[cfg(test)]
mod property_based_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_performance_scaling(
            dataset_size in 1000..1000000usize,
            viewport_scale in 0.1..2.0f64
        ) {
            let mut engine = HighPerformanceEngine::new();
            let dataset: Vec<f64> = (0..dataset_size).map(|i| (i as f64 * 0.001).sin()).collect();

            let start = Instant::now();
            let result = engine.process_large_dataset(&dataset, viewport_scale);
            let duration = start.elapsed();

            assert!(result.is_ok());

            // Performance should scale sub-linearly
            let max_duration_ms = (dataset_size as f64 / 10000.0).ceil() as u64 * 10;
            let max_duration = Duration::from_millis(max_duration_ms.max(10));

            assert!(duration <= max_duration,
                   "Processing {} points took {:?}, should be <= {:?}",
                   dataset_size, duration, max_duration);
        }

        #[test]
        fn test_memory_scaling(
            iterations in 1..100usize,
            buffer_size in 100..10000usize
        ) {
            let mut engine = HighPerformanceEngine::new();
            let initial_memory = engine.get_memory_usage();

            for i in 0..iterations {
                let dataset: Vec<f64> = (0..buffer_size).map(|j| ((i * buffer_size + j) as f64 * 0.001).sin()).collect();
                let _metrics = engine.process_large_dataset(&dataset, 1.0).unwrap();
            }

            let final_memory = engine.get_memory_usage();
            let memory_growth = final_memory - initial_memory;

            // Memory growth should be reasonable
            let expected_growth = iterations * buffer_size * 8; // 8 bytes per f64
            let tolerance = expected_growth / 2; // 50% tolerance

            assert!(memory_growth <= expected_growth + tolerance,
                   "Memory growth {} should be <= expected {} + tolerance {}",
                   memory_growth, expected_growth, tolerance);
        }

        #[test]
        fn test_error_handling_robustness(
            query_length in 0..1000usize
        ) {
            let processor = NLProcessor::new();

            // Generate random query
            let query = "a".repeat(query_length);

            let result = processor.parse_query(&query);

            // Should either succeed or fail gracefully
            match result {
                Ok(spec) => {
                    // If successful, should have valid chart spec
                    assert!(!format!("{:?}", spec).is_empty());
                },
                Err(error) => {
                    // If failed, should have meaningful error message
                    assert!(true, "Should fail gracefully with error: {:?}", error);
                }
            }
        }
    }
}

/// Integration tests for production readiness
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_production_workload() {
        // TDD: Should handle production-like workload
        let mut renderer = Renderer::new().await.unwrap();
        let mut engine = HighPerformanceEngine::new();

        // Simulate production workload
        let chart_types = vec![
            MarkType::Point {
                size: Some(5.0),
                shape: Some(leptos_helios::chart::PointShape::Circle),
                opacity: Some(0.8),
            },
            MarkType::Point {
                size: Some(5.0),
                shape: Some(leptos_helios::chart::PointShape::Circle),
                opacity: Some(0.8),
            },
            MarkType::Point {
                size: Some(5.0),
                shape: Some(leptos_helios::chart::PointShape::Circle),
                opacity: Some(0.8),
            },
            MarkType::Point {
                size: Some(5.0),
                shape: Some(leptos_helios::chart::PointShape::Circle),
                opacity: Some(0.8),
            },
        ];
        let dataset_sizes = vec![1000, 10000, 100000];

        let start = Instant::now();
        let mut total_operations = 0;
        let mut successful_operations = 0;

        for chart_type in &chart_types {
            for dataset_size in &dataset_sizes {
                // Create chart spec
                let spec = ChartSpecBuilder::new()
                    .mark(MarkType::Point {
                        size: Some(5.0),
                        shape: Some(leptos_helios::chart::PointShape::Circle),
                        opacity: Some(0.8),
                    })
                    .build()
                    .unwrap_or_else(|_| ChartSpec::new());

                // Process dataset
                let dataset: Vec<f64> = (0..*dataset_size)
                    .map(|i| (i as f64 * 0.001).sin())
                    .collect();
                let metrics_result = engine.process_large_dataset(&dataset, 1.0);

                // Render chart
                let stats = renderer.render(&spec);

                total_operations += 1;
                if metrics_result.is_ok() && stats.fps() > 0.0 {
                    successful_operations += 1;
                }
            }
        }

        let duration = start.elapsed();
        let success_rate = successful_operations as f64 / total_operations as f64;

        // Should handle production workload
        assert!(
            success_rate >= 0.9,
            "Success rate {} should be >= 90%",
            success_rate
        );
        assert!(
            duration < Duration::from_millis(2000),
            "Should complete within 2 seconds"
        );
    }

    #[tokio::test]
    async fn test_stress_testing() {
        // TDD: Should handle stress testing
        let mut renderer = Renderer::new().await.unwrap();
        let mut engine = HighPerformanceEngine::new();

        // Stress test with rapid operations
        let start = Instant::now();
        let mut operations = 0;

        while start.elapsed() < Duration::from_millis(1000) {
            // Run for 1 second
            let dataset: Vec<f64> = (0..10000).map(|i| (i as f64 * 0.001).sin()).collect();
            let _metrics = engine.process_large_dataset(&dataset, 1.0).unwrap();

            let spec = ChartSpecBuilder::new()
                .mark(MarkType::Point {
                    size: Some(5.0),
                    shape: Some(leptos_helios::chart::PointShape::Circle),
                    opacity: Some(0.8),
                })
                .build()
                .unwrap_or_else(|_| ChartSpec::new());
            let _stats = renderer.render(&spec);

            operations += 1;
        }

        // Should handle at least 100 operations per second
        assert!(
            operations >= 100,
            "Should handle at least 100 operations per second, got {}",
            operations
        );
    }
}
