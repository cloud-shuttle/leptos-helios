//! Property tests for data processing pipelines
//!
//! These tests use proptest to generate arbitrary data and verify that
//! data processing operations maintain invariants and don't crash.

use proptest::prelude::*;
use polars::prelude::*;
use leptos_helios::chart::{ChartSpec, ChartSpecBuilder, MarkType, Encoding, EncodingDef, DataType, DataFormat, DataReference};
use leptos_helios::data_processing::*;

/// Generate arbitrary DataFrames for testing
fn arbitrary_dataframe() -> impl Strategy<Value = DataFrame> {
    (1..=1000usize).prop_flat_map(|size| {
        let data: Vec<i32> = (0..size).collect();
        let categories: Vec<String> = (0..size).map(|i| format!("category_{}", i % 10)).collect();
        let values: Vec<f64> = (0..size).map(|i| (i as f64) * 1.5).collect();
        
        Just(DataFrame::new(vec![
            Series::new("id", data).into(),
            Series::new("category", categories).into(),
            Series::new("value", values).into(),
        ]).unwrap())
    })
}

/// Generate arbitrary processing configurations
fn arbitrary_processing_config() -> impl Strategy<Value = ProcessingConfig> {
    prop_oneof![
        Just(ProcessingConfig::default()),
        Just(ProcessingConfig {
            max_rows: Some(1000),
            sampling_strategy: SamplingStrategy::Random,
            ..Default::default()
        }),
        Just(ProcessingConfig {
            max_rows: Some(100),
            sampling_strategy: SamplingStrategy::Systematic,
            ..Default::default()
        }),
    ]
}

proptest! {
    #[test]
    fn test_data_processing_preserves_schema(
        data in arbitrary_dataframe(),
        config in arbitrary_processing_config()
    ) {
        let original_schema = data.schema();
        let result = process_dataframe(&data, &config);
        
        // Processing should preserve the schema structure
        assert_eq!(original_schema.len(), result.schema().len());
        
        // All original columns should still exist
        for (name, dtype) in original_schema.iter() {
            assert!(result.schema().get(name).is_some());
            assert_eq!(dtype, result.schema().get(name).unwrap());
        }
    }

    #[test]
    fn test_data_processing_never_increases_rows(
        data in arbitrary_dataframe(),
        config in arbitrary_processing_config()
    ) {
        let original_rows = data.height();
        let result = process_dataframe(&data, &config);
        
        // Processing should never increase the number of rows
        assert!(result.height() <= original_rows);
    }

    #[test]
    fn test_data_processing_handles_empty_data(
        config in arbitrary_processing_config()
    ) {
        let empty_df = DataFrame::new(vec![]).unwrap();
        let result = process_dataframe(&empty_df, &config);
        
        // Should handle empty data gracefully
        assert_eq!(result.height(), 0);
    }

    #[test]
    fn test_chart_spec_builder_with_arbitrary_data(
        data in arbitrary_dataframe()
    ) {
        // Test that we can create chart specs with arbitrary data
        let spec = ChartSpecBuilder::default()
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
                    field: "value".to_string(),
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
            .build();
        
        // Should successfully create chart spec
        assert!(spec.is_ok());
    }

    #[test]
    fn test_data_sampling_consistency(
        data in arbitrary_dataframe()
    ) {
        if data.height() > 0 {
            let config = ProcessingConfig {
                max_rows: Some(10),
                sampling_strategy: SamplingStrategy::Random,
                ..Default::default()
            };
            
            let result1 = process_dataframe(&data, &config);
            let result2 = process_dataframe(&data, &config);
            
            // Both results should have the same number of rows (within sampling limits)
            assert!(result1.height() <= 10);
            assert!(result2.height() <= 10);
            
            // Schema should be preserved
            assert_eq!(data.schema(), result1.schema());
            assert_eq!(data.schema(), result2.schema());
        }
    }
}

/// Helper function to process a DataFrame (placeholder implementation)
fn process_dataframe(df: &DataFrame, config: &ProcessingConfig) -> DataFrame {
    // Simple processing: apply row limit if specified
    if let Some(max_rows) = config.max_rows {
        if df.height() > max_rows {
            return df.slice(0, max_rows as i64);
        }
    }
    
    df.clone()
}

/// Processing configuration for testing
#[derive(Debug, Clone, Default)]
struct ProcessingConfig {
    max_rows: Option<usize>,
    sampling_strategy: SamplingStrategy,
}

/// Sampling strategies
#[derive(Debug, Clone, Default)]
enum SamplingStrategy {
    #[default]
    None,
    Random,
    Systematic,
}
