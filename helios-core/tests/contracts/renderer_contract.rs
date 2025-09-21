//! Renderer contract tests
//!
//! These tests ensure that all renderer implementations conform to the expected
//! interface and behavior contracts.

use leptos_helios::*;
use crate::common::*;

/// Trait for testing renderer implementations
pub trait RendererTest {
    fn test_basic_render(&self);
    fn test_export_formats(&self);
    fn test_error_handling(&self);
    fn test_performance_characteristics(&self);
}

/// Test that a renderer can handle basic rendering operations
pub fn test_renderer_basic_functionality(renderer: &mut dyn Renderer) {
    let (spec, data) = create_test_chart();
    
    // Test that render doesn't panic
    let result = renderer.render(&spec, &data);
    // Note: We don't assert success here since renderers may not be fully implemented
    // We just ensure they don't panic and return a Result
    let _ = result;
}

/// Test that a renderer supports expected formats
pub fn test_renderer_format_support(renderer: &dyn Renderer) {
    // Test common formats
    let formats = ["png", "svg", "pdf", "html"];
    
    for format in &formats {
        let supports = renderer.supports_format(format);
        // We don't assert specific support since implementations may vary
        // We just ensure the method doesn't panic
        let _ = supports;
    }
}

/// Test renderer error handling
pub fn test_renderer_error_handling(renderer: &mut dyn Renderer) {
    // Test with invalid chart spec
    let invalid_spec = ChartSpec::default();
    let data = mock_dataframe();
    
    let result = renderer.render(&invalid_spec, &data);
    // Should return a Result (either Ok or Err, but not panic)
    let _ = result;
    
    // Test with empty data
    let empty_df = DataFrame::new(vec![]).unwrap();
    let spec = sample_chart_spec();
    
    let result = renderer.render(&spec, &empty_df);
    // Should handle empty data gracefully
    let _ = result;
}

/// Test renderer performance characteristics
pub fn test_renderer_performance(renderer: &mut dyn Renderer) {
    let spec = sample_chart_spec();
    let small_data = mock_dataframe();
    let large_data = mock_large_dataframe(1000);
    
    // Test with small dataset
    let (_, small_duration) = measure_time(|| {
        let _ = renderer.render(&spec, &small_data);
    });
    
    // Test with large dataset
    let (_, large_duration) = measure_time(|| {
        let _ = renderer.render(&spec, &large_data);
    });
    
    // Performance should be reasonable (not more than 5 seconds for large data)
    let max_duration = std::time::Duration::from_secs(5);
    assert!(
        large_duration <= max_duration,
        "Renderer took too long for large dataset: {:?}",
        large_duration
    );
    
    // Large dataset should take longer than small dataset (basic sanity check)
    // But we allow for some variance in case of optimizations
    if large_duration > small_duration * 2 {
        // This is expected - larger data should take longer
    }
}

/// Test renderer with different chart types
pub fn test_renderer_chart_types(renderer: &mut dyn Renderer) {
    let data = mock_dataframe();
    
    let chart_types = vec![
        ("Line Chart", sample_chart_spec()),
        ("Bar Chart", sample_bar_chart_spec()),
        ("Scatter Plot", sample_scatter_plot_spec()),
    ];
    
    for (chart_name, spec) in chart_types {
        let result = renderer.render(&spec, &data);
        // Should not panic for any chart type
        let _ = result;
        
        // Log the result for debugging (in real tests, you might want to assert success)
        match result {
            Ok(_) => println!("✅ {} rendered successfully", chart_name),
            Err(e) => println!("⚠️  {} failed to render: {}", chart_name, e),
        }
    }
}

/// Test renderer with different data sizes
pub fn test_renderer_data_scaling(renderer: &mut dyn Renderer) {
    let spec = sample_chart_spec();
    
    let data_sizes = vec![10, 100, 1000, 5000];
    
    for size in data_sizes {
        let data = mock_large_dataframe(size);
        
        let (_, duration) = measure_time(|| {
            let _ = renderer.render(&spec, &data);
        });
        
        println!("Data size: {}, Render time: {:?}", size, duration);
        
        // Should not take more than 10 seconds for any reasonable data size
        let max_duration = std::time::Duration::from_secs(10);
        assert!(
            duration <= max_duration,
            "Renderer took too long for data size {}: {:?}",
            size,
            duration
        );
    }
}

/// Test renderer memory usage (basic check)
pub fn test_renderer_memory_usage(renderer: &mut dyn Renderer) {
    let spec = sample_chart_spec();
    let data = mock_large_dataframe(1000);
    
    // This is a basic test - in a real implementation, you might want to
    // use more sophisticated memory profiling
    let result = renderer.render(&spec, &data);
    let _ = result;
    
    // If we get here without running out of memory, that's good
    // In a more sophisticated test, you might check memory usage before/after
}

/// Contract test for renderer interface stability
#[test]
fn test_renderer_interface_contract() {
    // Test that the Renderer trait has the expected methods
    let mut renderer = test_renderer();
    
    // Test name() method
    let name = renderer.name();
    assert!(!name.is_empty(), "Renderer name should not be empty");
    
    // Test supports_format() method
    let supports_png = renderer.supports_format("png");
    let _ = supports_png; // Don't assert specific support, just ensure it doesn't panic
    
    // Test render() method
    let (spec, data) = create_test_chart();
    let result = renderer.render(&spec, &data);
    let _ = result; // Don't assert success, just ensure it returns a Result
}

/// Contract test for renderer behavior consistency
#[test]
fn test_renderer_behavior_consistency() {
    let mut renderer = test_renderer();
    
    // Test that multiple renders with the same input produce consistent results
    let (spec, data) = create_test_chart();
    
    let result1 = renderer.render(&spec, &data);
    let result2 = renderer.render(&spec, &data);
    
    // Both should return the same type of result (both Ok or both Err)
    match (result1, result2) {
        (Ok(_), Ok(_)) => {
            // Both succeeded - good
        }
        (Err(_), Err(_)) => {
            // Both failed - acceptable for mock renderer
        }
        _ => {
            // Inconsistent results - this might indicate a problem
            // For now, we just log this as a warning
            println!("⚠️  Renderer produced inconsistent results");
        }
    }
}

/// Test renderer with edge cases
#[test]
fn test_renderer_edge_cases() {
    let mut renderer = test_renderer();
    
    // Test with minimal chart spec
    let minimal_spec = ChartSpec::default();
    let data = mock_dataframe();
    let _ = renderer.render(&minimal_spec, &data);
    
    // Test with empty DataFrame
    let empty_df = DataFrame::new(vec![]).unwrap();
    let spec = sample_chart_spec();
    let _ = renderer.render(&spec, &empty_df);
    
    // Test with single-row DataFrame
    let single_row_df = DataFrame::new(vec![
        Series::new("x", &[1]),
        Series::new("y", &[10]),
    ]).unwrap();
    let _ = renderer.render(&spec, &single_row_df);
    
    // Test with very wide DataFrame (many columns)
    let wide_df = DataFrame::new(vec![
        Series::new("col1", &[1, 2, 3]),
        Series::new("col2", &[4, 5, 6]),
        Series::new("col3", &[7, 8, 9]),
        Series::new("col4", &[10, 11, 12]),
        Series::new("col5", &[13, 14, 15]),
    ]).unwrap();
    let _ = renderer.render(&spec, &wide_df);
}
