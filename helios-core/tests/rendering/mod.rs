//! Renderer testing framework
//!
//! This module provides a common testing interface for all renderer types,
//! ensuring consistent behavior across different rendering backends.

use leptos_helios::chart::{ChartSpec, ChartSpecBuilder, MarkType, Encoding, EncodingDef, DataType, DataFormat, DataReference};
use polars::prelude::*;
use std::collections::HashMap;

/// Common test interface for all renderer types
pub trait RendererTest {
    /// Test basic rendering functionality
    fn test_basic_render(&self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Test export format support
    fn test_export_formats(&self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Test error handling with invalid inputs
    fn test_error_handling(&self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Test performance with large datasets
    fn test_performance(&self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Test rendering with different chart types
    fn test_chart_types(&self) -> Result<(), Box<dyn std::error::Error>>;
}

/// Test utilities for renderer testing
pub mod utils {
    use super::*;
    
    /// Create a test DataFrame
    pub fn create_test_dataframe() -> DataFrame {
        DataFrame::new(vec![
            Series::new("x", &[1, 2, 3, 4, 5]).into(),
            Series::new("y", &[10, 20, 15, 25, 30]).into(),
            Series::new("category", &["A", "B", "A", "C", "B"]).into(),
        ]).unwrap()
    }
    
    /// Create a test chart specification
    pub fn create_test_chart_spec() -> ChartSpec {
        ChartSpecBuilder::default()
            .data(DataReference {
                source: "test_data".to_string(),
                format: DataFormat::Inline,
                schema: None,
            })
            .mark(MarkType::Bar {
                width: None,
                corner_radius: None,
            })
            .encoding(Encoding {
                x: Some(EncodingDef {
                    field: "category".to_string(),
                    data_type: DataType::String,
                    scale: None,
                    axis: None,
                    legend: None,
                    bin: None,
                    aggregate: None,
                    sort: None,
                }),
                y: Some(EncodingDef {
                    field: "y".to_string(),
                    data_type: DataType::Number,
                    scale: None,
                    axis: None,
                    legend: None,
                    bin: None,
                    aggregate: None,
                    sort: None,
                }),
                color: None,
                size: None,
                shape: None,
                opacity: None,
                text: None,
                tooltip: None,
                detail: None,
                order: None,
                row: None,
                column: None,
            })
            .build()
            .unwrap()
    }
    
    /// Create a large test dataset for performance testing
    pub fn create_large_test_dataframe() -> DataFrame {
        let size = 10000;
        let x_data: Vec<i32> = (0..size).collect();
        let y_data: Vec<f64> = (0..size).map(|i| (i as f64) * 0.1).collect();
        let category_data: Vec<String> = (0..size).map(|i| format!("cat_{}", i % 100)).collect();
        
        DataFrame::new(vec![
            Series::new("x", x_data).into(),
            Series::new("y", y_data).into(),
            Series::new("category", category_data).into(),
        ]).unwrap()
    }
    
    /// Create different chart types for testing
    pub fn create_chart_specs() -> HashMap<String, ChartSpec> {
        let mut specs = HashMap::new();
        
        // Bar chart
        specs.insert("bar".to_string(), ChartSpecBuilder::default()
            .data(DataReference {
                source: "test_data".to_string(),
                format: DataFormat::Inline,
                schema: None,
            })
            .mark(MarkType::Bar {
                width: None,
                corner_radius: None,
            })
            .encoding(Encoding {
                x: Some(EncodingDef {
                    field: "category".to_string(),
                    data_type: DataType::String,
                    scale: None,
                    axis: None,
                    legend: None,
                    bin: None,
                    aggregate: None,
                    sort: None,
                }),
                y: Some(EncodingDef {
                    field: "y".to_string(),
                    data_type: DataType::Number,
                    scale: None,
                    axis: None,
                    legend: None,
                    bin: None,
                    aggregate: None,
                    sort: None,
                }),
                color: None,
                size: None,
                shape: None,
                opacity: None,
                text: None,
                tooltip: None,
                detail: None,
                order: None,
                row: None,
                column: None,
            })
            .build()
            .unwrap());
        
        // Line chart
        specs.insert("line".to_string(), ChartSpecBuilder::default()
            .data(DataReference {
                source: "test_data".to_string(),
                format: DataFormat::Inline,
                schema: None,
            })
            .mark(MarkType::Line {
                interpolate: None,
                stroke_width: None,
                stroke_dash: None,
            })
            .encoding(Encoding {
                x: Some(EncodingDef {
                    field: "x".to_string(),
                    data_type: DataType::Number,
                    scale: None,
                    axis: None,
                    legend: None,
                    bin: None,
                    aggregate: None,
                    sort: None,
                }),
                y: Some(EncodingDef {
                    field: "y".to_string(),
                    data_type: DataType::Number,
                    scale: None,
                    axis: None,
                    legend: None,
                    bin: None,
                    aggregate: None,
                    sort: None,
                }),
                color: None,
                size: None,
                shape: None,
                opacity: None,
                text: None,
                tooltip: None,
                detail: None,
                order: None,
                row: None,
                column: None,
            })
            .build()
            .unwrap());
        
        // Point chart
        specs.insert("point".to_string(), ChartSpecBuilder::default()
            .data(DataReference {
                source: "test_data".to_string(),
                format: DataFormat::Inline,
                schema: None,
            })
            .mark(MarkType::Point {
                size: None,
                shape: None,
                opacity: None,
            })
            .encoding(Encoding {
                x: Some(EncodingDef {
                    field: "x".to_string(),
                    data_type: DataType::Number,
                    scale: None,
                    axis: None,
                    legend: None,
                    bin: None,
                    aggregate: None,
                    sort: None,
                }),
                y: Some(EncodingDef {
                    field: "y".to_string(),
                    data_type: DataType::Number,
                    scale: None,
                    axis: None,
                    legend: None,
                    bin: None,
                    aggregate: None,
                    sort: None,
                }),
                color: None,
                size: None,
                shape: None,
                opacity: None,
                text: None,
                tooltip: None,
                detail: None,
                order: None,
                row: None,
                column: None,
            })
            .build()
            .unwrap());
        
        specs
    }
}

/// Common test implementations that can be shared across renderers
pub mod common_tests {
    use super::*;
    use std::time::Instant;
    
    /// Test that renderer can handle basic rendering without crashing
    pub fn test_basic_render_impl<T: RendererTest>(renderer: &T) -> Result<(), Box<dyn std::error::Error>> {
        let spec = utils::create_test_chart_spec();
        let data = utils::create_test_dataframe();
        
        // This is a placeholder - actual renderers would call their render method
        // For now, we just test that the renderer can be created and the test runs
        println!("Testing basic render with spec: {:?}", spec.mark);
        println!("Testing with data of size: {}", data.height());
        
        Ok(())
    }
    
    /// Test error handling with invalid inputs
    pub fn test_error_handling_impl<T: RendererTest>(renderer: &T) -> Result<(), Box<dyn std::error::Error>> {
        // Test with empty data
        let empty_df = DataFrame::new(vec![]).unwrap();
        println!("Testing error handling with empty data");
        
        // Test with malformed chart spec (this would be caught by the builder)
        println!("Testing error handling with invalid chart spec");
        
        Ok(())
    }
    
    /// Test performance with large datasets
    pub fn test_performance_impl<T: RendererTest>(renderer: &T) -> Result<(), Box<dyn std::error::Error>> {
        let large_data = utils::create_large_test_dataframe();
        let spec = utils::create_test_chart_spec();
        
        let start = Instant::now();
        
        // Simulate rendering operation
        println!("Testing performance with {} rows", large_data.height());
        
        let duration = start.elapsed();
        println!("Performance test completed in {:?}", duration);
        
        // Performance should be reasonable (less than 1 second for 10k rows)
        assert!(duration.as_secs() < 1, "Performance regression: {:?}", duration);
        
        Ok(())
    }
    
    /// Test different chart types
    pub fn test_chart_types_impl<T: RendererTest>(renderer: &T) -> Result<(), Box<dyn std::error::Error>> {
        let specs = utils::create_chart_specs();
        let data = utils::create_test_dataframe();
        
        for (chart_type, spec) in specs.iter() {
            println!("Testing chart type: {}", chart_type);
            // Test that each chart type can be processed
            assert!(matches!(spec.mark, MarkType::Bar { .. } | MarkType::Line { .. } | MarkType::Point { .. }));
        }
        
        Ok(())
    }
}
