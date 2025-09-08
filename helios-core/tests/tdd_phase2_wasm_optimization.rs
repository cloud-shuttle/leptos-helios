//! TDD Phase 2: WASM Bundle Size Optimization Tests
//!
//! This module contains comprehensive tests for WASM bundle size optimization:
//! - Bundle size analysis and regression testing
//! - Tree-shaking efficiency validation
//! - Dynamic import optimization
//! - Code splitting and chunking
//! - Feature flag optimization

use leptos_helios::*;
use std::time::Duration;
use proptest::prelude::*;

/// Test WASM bundle size analysis and optimization
#[cfg(test)]
mod bundle_size_tests {
    use super::*;

    #[test]
    fn test_bundle_size_target_met() {
        // TDD: Bundle size should meet target (<120KB)
        let mut analyzer = WasmBundleAnalyzer::new();
        let bundle_size = analyzer.analyze_bundle_size();
        
        let target_size = 120 * 1024; // 120KB target
        assert!(bundle_size < target_size, 
               "Bundle size {} bytes should be < {} bytes (120KB)", 
               bundle_size, target_size);
    }

    #[test]
    fn test_bundle_size_regression_prevention() {
        // TDD: Should prevent bundle size regressions
        let mut analyzer = WasmBundleAnalyzer::new();
        let current_size = analyzer.analyze_bundle_size();
        
        // Simulate size regression check
        let max_allowed_size = 130 * 1024; // 130KB max (10KB buffer)
        assert!(current_size < max_allowed_size, 
               "Bundle size {} bytes exceeds maximum allowed {} bytes", 
               current_size, max_allowed_size);
    }

    #[test]
    fn test_optimized_bundle_size() {
        // TDD: Optimized bundle should be significantly smaller
        let mut optimizer = WasmBundleOptimizer::new();
        let initial_size = optimizer.get_initial_bundle_size();
        
        // Apply optimizations
        optimizer.apply_tree_shaking().unwrap();
        optimizer.apply_feature_flags().unwrap();
        optimizer.apply_dynamic_imports().unwrap();
        
        let optimized_size = optimizer.get_optimized_bundle_size();
        let reduction_ratio = (initial_size - optimized_size) as f64 / initial_size as f64;
        
        // Should achieve at least 40% size reduction
        assert!(reduction_ratio >= 0.4, 
               "Bundle size reduction {}% should be >= 40%", 
               reduction_ratio * 100.0);
    }

    #[test]
    fn test_core_bundle_size() {
        // TDD: Core bundle should be minimal
        let optimizer = WasmBundleOptimizer::new();
        let core_size = optimizer.get_core_bundle_size();
        
        let max_core_size = 100 * 1024; // 100KB max for core
        assert!(core_size < max_core_size, 
               "Core bundle size {} bytes should be < {} bytes", 
               core_size, max_core_size);
    }

    #[test]
    fn test_dynamic_import_optimization() {
        // TDD: Dynamic imports should reduce initial bundle size
        let optimizer = WasmBundleOptimizer::new();
        let initial_size = optimizer.get_initial_bundle_size();
        let dynamic_size = optimizer.get_optimized_bundle_size_with_dynamic_imports();
        
        let reduction = initial_size - dynamic_size;
        let reduction_ratio = reduction as f64 / initial_size as f64;
        
        // Should achieve at least 40% reduction with dynamic imports
        assert!(reduction_ratio >= 0.4, 
               "Dynamic import reduction {}% should be >= 40%", 
               reduction_ratio * 100.0);
    }
}

/// Test tree-shaking efficiency
#[cfg(test)]
mod tree_shaking_tests {
    use super::*;

    #[test]
    fn test_tree_shaking_efficiency() {
        // TDD: Tree-shaking should achieve high efficiency
        let mut analyzer = WasmBundleAnalyzer::new();
        let efficiency = analyzer.calculate_tree_shake_ratio();
        
        // Should achieve at least 50% tree-shaking efficiency
        assert!(efficiency >= 0.5, 
               "Tree-shaking efficiency {}% should be >= 50%", 
               efficiency * 100.0);
    }

    #[test]
    fn test_unused_dependencies_removal() {
        // TDD: Should identify and remove unused dependencies
        let mut analyzer = WasmBundleAnalyzer::new();
        let unused_deps = analyzer.analyze_unused_dependencies();
        
        // After optimization, should have minimal unused dependencies
        assert!(unused_deps.len() <= 5, 
               "Should have <= 5 unused dependencies, found {}", 
               unused_deps.len());
    }

    #[test]
    fn test_tree_shaking_optimization() {
        // TDD: Tree-shaking optimization should improve efficiency
        let mut optimizer = WasmBundleOptimizer::new();
        let initial_efficiency = optimizer.analyzer.get_tree_shake_ratio();
        
        // Apply tree-shaking optimization
        optimizer.apply_tree_shaking().unwrap();
        let optimized_efficiency = optimizer.analyzer.get_tree_shake_ratio();
        
        // Efficiency should improve after optimization
        assert!(optimized_efficiency >= initial_efficiency, 
               "Tree-shaking efficiency should improve from {}% to {}%", 
               initial_efficiency * 100.0, optimized_efficiency * 100.0);
    }
}

/// Test code splitting and chunking
#[cfg(test)]
mod code_splitting_tests {
    use super::*;

    #[test]
    fn test_code_chunk_creation() {
        // TDD: Should create optimized code chunks
        let mut splitter = CodeSplittingOptimizer::new();
        let chunks = splitter.create_optimized_code_chunks();
        
        // Should create multiple chunks
        assert!(chunks.len() >= 4, "Should create at least 4 code chunks");
        
        // Verify chunk sizes are reasonable
        for chunk in &chunks {
            assert!(chunk.size > 0, "Chunk '{}' should have positive size", chunk.name);
            assert!(chunk.size < 50 * 1024, "Chunk '{}' size {} should be < 50KB", 
                   chunk.name, chunk.size);
        }
    }

    #[test]
    fn test_chunk_size_distribution() {
        // TDD: Chunk sizes should be well-distributed
        let mut splitter = CodeSplittingOptimizer::new();
        let chunks = splitter.create_optimized_code_chunks();
        
        let total_size: usize = chunks.iter().map(|c| c.size).sum();
        let avg_size = total_size / chunks.len();
        
        // No chunk should be more than 3x the average size
        for chunk in &chunks {
            assert!(chunk.size <= avg_size * 3, 
                   "Chunk '{}' size {} should be <= 3x average {}", 
                   chunk.name, chunk.size, avg_size);
        }
    }

    #[test]
    fn test_core_chunk_priority() {
        // TDD: Core chunk should be smallest and highest priority
        let mut splitter = CodeSplittingOptimizer::new();
        let chunks = splitter.create_optimized_code_chunks();
        
        // Find core chunk
        let core_chunk = chunks.iter().find(|c| c.name == "core");
        assert!(core_chunk.is_some(), "Should have a 'core' chunk");
        
        let core_size = core_chunk.unwrap().size;
        
        // Core chunk should be among the smallest
        let min_size = chunks.iter().map(|c| c.size).min().unwrap();
        assert!(core_size <= min_size * 2, 
               "Core chunk size {} should be close to minimum {}", 
               core_size, min_size);
    }

    #[test]
    fn test_chunk_dependency_optimization() {
        // TDD: Chunks should be optimized for dependency loading
        let mut splitter = CodeSplittingOptimizer::new();
        let chunks = splitter.create_optimized_code_chunks();
        
        // Should have logical chunk separation
        let chunk_names: Vec<&str> = chunks.iter().map(|c| c.name.as_str()).collect();
        assert!(chunk_names.contains(&"core"), "Should have core chunk");
        assert!(chunk_names.contains(&"gpu"), "Should have GPU chunk");
        assert!(chunk_names.contains(&"data"), "Should have data chunk");
        assert!(chunk_names.contains(&"ui"), "Should have UI chunk");
    }
}

/// Test feature flag optimization
#[cfg(test)]
mod feature_flag_tests {
    use super::*;

    #[test]
    fn test_feature_flag_optimization() {
        // TDD: Feature flags should enable size optimization
        let mut optimizer = WasmBundleOptimizer::new();
        let initial_size = optimizer.get_initial_bundle_size();
        
        // Apply feature flag optimization
        optimizer.apply_feature_flags().unwrap();
        let optimized_size = optimizer.get_optimized_bundle_size();
        
        // Should reduce bundle size
        assert!(optimized_size < initial_size, 
               "Feature flag optimization should reduce bundle size from {} to {}", 
               initial_size, optimized_size);
    }

    #[test]
    fn test_core_vs_full_bundle_size() {
        // TDD: Core bundle should be significantly smaller than full bundle
        let optimizer = WasmBundleOptimizer::new();
        let core_size = optimizer.get_core_bundle_size();
        let full_size = optimizer.get_full_bundle_size();
        
        let size_ratio = core_size as f64 / full_size as f64;
        
        // Core should be at most 60% of full bundle size
        assert!(size_ratio <= 0.6, 
               "Core bundle {} should be <= 60% of full bundle {}", 
               core_size, full_size);
    }

    #[test]
    fn test_feature_flag_application() {
        // TDD: Feature flags should be applied correctly
        let mut optimizer = WasmBundleOptimizer::new();
        
        // Initially not optimized
        assert!(!optimizer.optimization_applied);
        
        // Apply feature flags
        let result = optimizer.apply_feature_flags();
        assert!(result.is_ok());
        assert!(optimizer.optimization_applied);
    }
}

/// Test WASM optimization engine integration
#[cfg(test)]
mod optimization_engine_tests {
    use super::*;

    #[test]
    fn test_full_optimization_pipeline() {
        // TDD: Full optimization pipeline should work end-to-end
        let mut engine = WasmOptimizationEngine::new();
        let initial_size = engine.analyzer.get_bundle_size();
        
        // Run full optimization
        let result = engine.optimize();
        assert!(result.is_ok(), "Full optimization should succeed");
        
        let final_size = engine.get_final_bundle_size();
        let reduction = initial_size - final_size;
        let reduction_ratio = reduction as f64 / initial_size as f64;
        
        // Should achieve significant size reduction
        assert!(reduction_ratio >= 0.4, 
               "Full optimization should achieve >= 40% size reduction, got {}%", 
               reduction_ratio * 100.0);
    }

    #[test]
    fn test_size_target_validation() {
        // TDD: Should validate against size targets
        let engine = WasmOptimizationEngine::new();
        let final_size = engine.get_final_bundle_size();
        
        // Test various size targets
        let targets = vec![
            120 * 1024, // 120KB
            100 * 1024, // 100KB
            80 * 1024,  // 80KB
        ];
        
        for target in targets {
            let meets_target = engine.meets_size_target(target);
            if final_size < target {
                assert!(meets_target, "Should meet target {} bytes", target);
            } else {
                assert!(!meets_target, "Should not meet target {} bytes", target);
            }
        }
    }

    #[test]
    fn test_optimization_consistency() {
        // TDD: Optimization should be consistent across runs
        let mut engine1 = WasmOptimizationEngine::new();
        let mut engine2 = WasmOptimizationEngine::new();
        
        // Run optimization on both engines
        engine1.optimize().unwrap();
        engine2.optimize().unwrap();
        
        let size1 = engine1.get_final_bundle_size();
        let size2 = engine2.get_final_bundle_size();
        
        // Results should be consistent (within 1KB tolerance)
        let difference = (size1 as i64 - size2 as i64).abs() as usize;
        assert!(difference <= 1024, 
               "Optimization results should be consistent, difference: {} bytes", 
               difference);
    }
}

/// Property-based tests for WASM optimization
#[cfg(test)]
mod property_based_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_bundle_size_scaling(
            initial_size in 100000..500000usize,
            optimization_factor in 0.3..0.8f64
        ) {
            let mut analyzer = WasmBundleAnalyzer::new();
            analyzer.bundle_size = initial_size;
            
            let optimized_size = (initial_size as f64 * optimization_factor) as usize;
            analyzer.bundle_size = optimized_size;
            
            let reduction = initial_size - optimized_size;
            let reduction_ratio = reduction as f64 / initial_size as f64;
            
            // Reduction ratio should match optimization factor
            let tolerance = 0.01; // 1% tolerance
            assert!((reduction_ratio - (1.0 - optimization_factor)).abs() < tolerance,
                   "Reduction ratio {} should match optimization factor {}", 
                   reduction_ratio, 1.0 - optimization_factor);
        }

        #[test]
        fn test_chunk_size_distribution(
            chunk_count in 3..10usize,
            total_size in 100000..300000usize
        ) {
            let mut splitter = CodeSplittingOptimizer::new();
            
            // Create chunks with random sizes
            let mut chunks = Vec::new();
            let chunk_names = ["core", "gpu", "data", "ui", "utils", "math", "render"];
            
            for i in 0..chunk_count {
                let name = chunk_names[i % chunk_names.len()].to_string();
                let size = total_size / chunk_count + (i * 1000); // Add some variation
                chunks.push(CodeChunk { name, size });
            }
            
            splitter.chunks = chunks;
            
            let chunks = splitter.get_chunks();
            let total_chunk_size: usize = chunks.iter().map(|c| c.size).sum();
            
            // Total chunk size should be close to target
            let tolerance = total_size / 10; // 10% tolerance
            assert!((total_chunk_size as i64 - total_size as i64).abs() <= tolerance as i64,
                   "Total chunk size {} should be close to target {}", 
                   total_chunk_size, total_size);
        }

        #[test]
        fn test_tree_shaking_efficiency_bounds(
            efficiency in 0.0..1.0f64
        ) {
            let mut analyzer = WasmBundleAnalyzer::new();
            analyzer.tree_shake_ratio = efficiency;
            
            let actual_efficiency = analyzer.calculate_tree_shake_ratio();
            
            // Efficiency should be within bounds
            assert!(actual_efficiency >= 0.0 && actual_efficiency <= 1.0,
                   "Tree-shaking efficiency {} should be between 0 and 1", 
                   actual_efficiency);
        }
    }
}

/// Performance tests for WASM optimization
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_optimization_performance() {
        // TDD: Optimization should complete quickly
        let mut engine = WasmOptimizationEngine::new();
        
        let start = Instant::now();
        let result = engine.optimize();
        let duration = start.elapsed();
        
        assert!(result.is_ok());
        
        // Optimization should complete within 100ms
        assert!(duration < Duration::from_millis(100), 
               "Optimization took {:?}, should be < 100ms", duration);
    }

    #[test]
    fn test_bundle_analysis_performance() {
        // TDD: Bundle analysis should be fast
        let mut analyzer = WasmBundleAnalyzer::new();
        
        let start = Instant::now();
        analyzer.analyze_bundle_size();
        analyzer.analyze_unused_dependencies();
        analyzer.calculate_tree_shake_ratio();
        let duration = start.elapsed();
        
        // Analysis should complete within 50ms
        assert!(duration < Duration::from_millis(50), 
               "Bundle analysis took {:?}, should be < 50ms", duration);
    }

    #[test]
    fn test_chunk_creation_performance() {
        // TDD: Chunk creation should be efficient
        let mut splitter = CodeSplittingOptimizer::new();
        
        let start = Instant::now();
        let chunks = splitter.create_optimized_code_chunks();
        let duration = start.elapsed();
        
        // Chunk creation should complete within 10ms
        assert!(duration < Duration::from_millis(10), 
               "Chunk creation took {:?}, should be < 10ms", duration);
        
        // Should create reasonable number of chunks
        assert!(chunks.len() >= 3 && chunks.len() <= 10, 
               "Should create 3-10 chunks, got {}", chunks.len());
    }
}

/// Integration tests for WASM optimization
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_end_to_end_wasm_optimization() {
        // TDD: End-to-end WASM optimization should work
        let mut engine = WasmOptimizationEngine::new();
        
        // Get initial state
        let initial_size = engine.analyzer.get_bundle_size();
        let initial_efficiency = engine.analyzer.get_tree_shake_ratio();
        
        // Run full optimization
        let result = engine.optimize();
        assert!(result.is_ok(), "End-to-end optimization should succeed");
        
        // Verify improvements
        let final_size = engine.get_final_bundle_size();
        let final_efficiency = engine.analyzer.get_tree_shake_ratio();
        
        assert!(final_size < initial_size, "Bundle size should decrease");
        assert!(final_efficiency >= initial_efficiency, "Tree-shaking efficiency should improve");
        
        // Verify size target
        let target_size = 120 * 1024; // 120KB
        assert!(engine.meets_size_target(target_size), "Should meet size target");
        
        // Verify chunks are created
        let chunks = engine.code_splitter.get_chunks();
        assert!(!chunks.is_empty(), "Should create code chunks");
    }

    #[test]
    fn test_optimization_with_different_configurations() {
        // TDD: Should work with different optimization configurations
        let configurations = vec![
            ("minimal", 80 * 1024),   // 80KB target
            ("standard", 120 * 1024), // 120KB target
            ("full", 200 * 1024),     // 200KB target
        ];
        
        for (config_name, target_size) in configurations {
            let mut engine = WasmOptimizationEngine::new();
            
            // Apply configuration-specific optimization
            match config_name {
                "minimal" => {
                    engine.optimizer.apply_tree_shaking().unwrap();
                    engine.optimizer.apply_feature_flags().unwrap();
                },
                "standard" => {
                    engine.optimize().unwrap();
                },
                "full" => {
                    engine.optimize().unwrap();
                    // Additional optimizations for full config
                },
                _ => {}
            }
            
            let final_size = engine.get_final_bundle_size();
            assert!(final_size <= target_size, 
                   "Configuration '{}' should meet target {} bytes, got {}", 
                   config_name, target_size, final_size);
        }
    }
}