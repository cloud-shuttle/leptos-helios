//! TDD Tests for Data Processing Pipeline
//!
//! Following RED-GREEN-REFACTOR cycle for data processing functionality

use helios_core::data::*;
use helios_core::utils::test_utils::*;
use polars::prelude::*;
use std::time::Duration as StdDuration;

#[test]
fn test_data_processor_creation() {
    // RED: Test that data processor can be created
    let processor = DataProcessor::new();
    assert!(
        processor.is_ok(),
        "DataProcessor should be created successfully"
    );
}

#[test]
fn test_strategy_selector_creation() {
    // RED: Test that strategy selector can be created
    let selector = StrategySelector::new();

    // Should have default device capabilities
    let capabilities = selector.device_capabilities();
    assert!(capabilities.cpu_cores > 0, "Should detect CPU cores");
    assert!(capabilities.memory_gb > 0.0, "Should detect memory");
}

#[test]
fn test_strategy_selection_for_small_data() {
    // RED: Test strategy selection for small datasets
    let selector = StrategySelector::new();
    let spec = DataSpec {
        source: DataSource::DataFrame(create_test_dataframe()),
        transforms: vec![],
        filters: vec![],
        aggregations: vec![],
        output_format: OutputFormat::DataFrame,
    };

    let strategy = selector.select(&spec);

    // Small datasets should use CPU strategy
    match strategy {
        ProcessingStrategy::CPU(_) => {
            // Expected for small datasets
        }
        _ => panic!("Small datasets should use CPU strategy"),
    }
}

#[test]
fn test_strategy_selection_for_large_data() {
    // RED: Test strategy selection for large datasets
    let selector = StrategySelector::new();
    let spec = DataSpec {
        source: DataSource::DataFrame(create_large_test_dataframe(1_000_000)),
        transforms: vec![],
        filters: vec![],
        aggregations: vec![],
        output_format: OutputFormat::DataFrame,
    };

    let strategy = selector.select(&spec);

    // Large datasets should use GPU strategy if available
    match strategy {
        ProcessingStrategy::GPU(_) | ProcessingStrategy::CPU(_) => {
            // Either GPU or CPU is acceptable depending on device capabilities
        }
        _ => panic!("Large datasets should use GPU or CPU strategy"),
    }
}

#[test]
fn test_strategy_selection_for_streaming_data() {
    // RED: Test strategy selection for streaming data
    let selector = StrategySelector::new();
    let spec = DataSpec {
        source: DataSource::Stream {
            stream_id: "test_stream".to_string(),
        },
        transforms: vec![],
        filters: vec![],
        aggregations: vec![],
        output_format: OutputFormat::DataFrame,
    };

    let strategy = selector.select(&spec);

    // Streaming data should always use streaming strategy
    match strategy {
        ProcessingStrategy::Streaming(_) => {
            // Expected for streaming data
        }
        _ => panic!("Streaming data should use streaming strategy"),
    }
}

#[test]
fn test_data_spec_complexity_calculation() {
    // RED: Test complexity calculation for data specifications
    let simple_spec = DataSpec {
        source: DataSource::DataFrame(create_test_dataframe()),
        transforms: vec![],
        filters: vec![],
        aggregations: vec![],
        output_format: OutputFormat::DataFrame,
    };

    let complex_spec = DataSpec {
        source: DataSource::DataFrame(create_test_dataframe()),
        transforms: vec![
            DataTransform::Select {
                columns: vec!["x".to_string(), "y".to_string()],
            },
            DataTransform::Rename {
                mappings: [("x".to_string(), "x_coord".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
            },
        ],
        filters: vec![
            Filter::Range {
                column: "x".to_string(),
                min: Some(0.0),
                max: Some(100.0),
            },
            Filter::Values {
                column: "category".to_string(),
                values: vec![serde_json::Value::String("A".to_string())],
            },
        ],
        aggregations: vec![
            Aggregation::GroupBy {
                columns: vec!["category".to_string()],
            },
            Aggregation::Aggregate {
                operations: vec![
                    AggOp::Sum {
                        column: "y".to_string(),
                        alias: Some("total_y".to_string()),
                    },
                    AggOp::Mean {
                        column: "y".to_string(),
                        alias: Some("avg_y".to_string()),
                    },
                ],
            },
        ],
        output_format: OutputFormat::DataFrame,
    };

    let simple_complexity = simple_spec.complexity();
    let complex_complexity = complex_spec.complexity();

    assert_eq!(
        simple_complexity, 1.0,
        "Simple spec should have complexity 1.0"
    );
    assert!(
        complex_complexity > simple_complexity,
        "Complex spec should have higher complexity"
    );
}

#[test]
fn test_data_spec_size_estimation() {
    // RED: Test data size estimation
    let small_spec = DataSpec {
        source: DataSource::DataFrame(create_test_dataframe()),
        transforms: vec![],
        filters: vec![],
        aggregations: vec![],
        output_format: OutputFormat::DataFrame,
    };

    let large_spec = DataSpec {
        source: DataSource::DataFrame(create_large_test_dataframe(100_000)),
        transforms: vec![],
        filters: vec![],
        aggregations: vec![],
        output_format: OutputFormat::DataFrame,
    };

    let small_size = small_spec.estimated_size();
    let large_size = large_spec.estimated_size();

    assert!(
        small_size < large_size,
        "Large spec should have larger estimated size"
    );
    assert_eq!(small_size, 5, "Small spec should have size 5");
    assert_eq!(large_size, 100_000, "Large spec should have size 100,000");
}

#[test]
fn test_data_spec_streaming_detection() {
    // RED: Test streaming data detection
    let streaming_spec = DataSpec {
        source: DataSource::Stream {
            stream_id: "test".to_string(),
        },
        transforms: vec![],
        filters: vec![],
        aggregations: vec![],
        output_format: OutputFormat::DataFrame,
    };

    let non_streaming_spec = DataSpec {
        source: DataSource::DataFrame(create_test_dataframe()),
        transforms: vec![],
        filters: vec![],
        aggregations: vec![],
        output_format: OutputFormat::DataFrame,
    };

    assert!(
        streaming_spec.is_streaming(),
        "Streaming spec should be detected as streaming"
    );
    assert!(
        !non_streaming_spec.is_streaming(),
        "Non-streaming spec should not be detected as streaming"
    );
}

#[test]
fn test_data_spec_hashing() {
    // RED: Test that data spec hashing works consistently
    let spec1 = DataSpec {
        source: DataSource::DataFrame(create_test_dataframe()),
        transforms: vec![],
        filters: vec![],
        aggregations: vec![],
        output_format: OutputFormat::DataFrame,
    };

    let spec2 = DataSpec {
        source: DataSource::DataFrame(create_test_dataframe()),
        transforms: vec![],
        filters: vec![],
        aggregations: vec![],
        output_format: OutputFormat::DataFrame,
    };

    let spec3 = DataSpec {
        source: DataSource::DataFrame(create_test_dataframe()),
        transforms: vec![DataTransform::Select {
            columns: vec!["x".to_string()],
        }],
        filters: vec![],
        aggregations: vec![],
        output_format: OutputFormat::DataFrame,
    };

    let hash1 = spec1.hash();
    let hash2 = spec2.hash();
    let hash3 = spec3.hash();

    assert_eq!(hash1, hash2, "Identical specs should have same hash");
    assert_ne!(hash1, hash3, "Different specs should have different hashes");
}

#[tokio::test]
async fn test_data_processor_cpu_processing() {
    // RED: Test CPU data processing
    let processor = DataProcessor::new().unwrap();
    let spec = DataSpec {
        source: DataSource::DataFrame(create_test_dataframe()),
        transforms: vec![],
        filters: vec![],
        aggregations: vec![],
        output_format: OutputFormat::DataFrame,
    };

    // This test will fail initially because process_cpu is not implemented
    // Following TDD: write test first, then implement
    let result = processor
        .process_cpu(
            &spec,
            &RayonConfig {
                num_threads: None,
                chunk_size: None,
                enable_simd: false,
            },
        )
        .await;
    assert!(result.is_ok(), "CPU processing should succeed");

    let processed_data = result.unwrap();
    assert_eq!(
        processed_data.data.height(),
        5,
        "Processed data should have 5 rows"
    );
    assert_eq!(
        processed_data.data.width(),
        3,
        "Processed data should have 3 columns"
    );
}

#[tokio::test]
async fn test_data_processor_with_filters() {
    // RED: Test data processing with filters
    let processor = DataProcessor::new().unwrap();
    let spec = DataSpec {
        source: DataSource::DataFrame(create_test_dataframe()),
        transforms: vec![],
        filters: vec![Filter::Range {
            column: "x".to_string(),
            min: Some(2.0),
            max: Some(4.0),
        }],
        aggregations: vec![],
        output_format: OutputFormat::DataFrame,
    };

    let result = processor
        .process_cpu(
            &spec,
            &RayonConfig {
                num_threads: None,
                chunk_size: None,
                enable_simd: false,
            },
        )
        .await;
    assert!(result.is_ok(), "Filtered processing should succeed");

    let processed_data = result.unwrap();
    assert!(
        processed_data.data.height() < 5,
        "Filtered data should have fewer rows"
    );
}

#[tokio::test]
async fn test_data_processor_with_transforms() {
    // RED: Test data processing with transforms
    let processor = DataProcessor::new().unwrap();
    let spec = DataSpec {
        source: DataSource::DataFrame(create_test_dataframe()),
        transforms: vec![DataTransform::Select {
            columns: vec!["x".to_string(), "y".to_string()],
        }],
        filters: vec![],
        aggregations: vec![],
        output_format: OutputFormat::DataFrame,
    };

    let result = processor
        .process_cpu(
            &spec,
            &RayonConfig {
                num_threads: None,
                chunk_size: None,
                enable_simd: false,
            },
        )
        .await;
    assert!(result.is_ok(), "Transform processing should succeed");

    let processed_data = result.unwrap();
    assert_eq!(
        processed_data.data.width(),
        2,
        "Transformed data should have 2 columns"
    );
    let column_names: Vec<String> = processed_data
        .data
        .get_column_names()
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert!(column_names.contains(&"x".to_string()));
    assert!(column_names.contains(&"y".to_string()));
    assert!(!column_names.contains(&"category".to_string()));
}

#[tokio::test]
async fn test_data_processor_with_aggregations() {
    // RED: Test data processing with aggregations
    let processor = DataProcessor::new().unwrap();
    let spec = DataSpec {
        source: DataSource::DataFrame(create_test_dataframe()),
        transforms: vec![],
        filters: vec![],
        aggregations: vec![
            Aggregation::GroupBy {
                columns: vec!["category".to_string()],
            },
            Aggregation::Aggregate {
                operations: vec![
                    AggOp::Sum {
                        column: "y".to_string(),
                        alias: Some("total_y".to_string()),
                    },
                    AggOp::Count {
                        column: "y".to_string(),
                        alias: Some("count".to_string()),
                    },
                ],
            },
        ],
        output_format: OutputFormat::DataFrame,
    };

    let result = processor
        .process_cpu(
            &spec,
            &RayonConfig {
                num_threads: None,
                chunk_size: None,
                enable_simd: false,
            },
        )
        .await;
    if let Err(e) = &result {
        println!("Aggregation processing failed: {:?}", e);
    }
    assert!(result.is_ok(), "Aggregation processing should succeed");

    let processed_data = result.unwrap();
    assert!(
        processed_data.data.height() <= 3,
        "Aggregated data should have fewer rows (grouped by category)"
    );
    let column_names: Vec<String> = processed_data
        .data
        .get_column_names()
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert!(column_names.contains(&"total_y".to_string()));
    assert!(column_names.contains(&"count".to_string()));
}

#[test]
fn test_stream_buffer_creation() {
    // RED: Test stream buffer creation
    let buffer = StreamBuffer::new(1000);
    let health = buffer.health_metrics();

    assert_eq!(
        health.buffer_utilization, 0.0,
        "New buffer should have 0% utilization"
    );
    assert_eq!(
        health.dropped_messages, 0,
        "New buffer should have 0 dropped messages"
    );
    assert_eq!(
        health.current_size, 0,
        "New buffer should have 0 current size"
    );
}

#[test]
fn test_stream_buffer_push_and_process() {
    // RED: Test stream buffer push and process operations
    let mut buffer = StreamBuffer::new(10);

    // Push some data
    buffer.push(create_test_dataframe());
    buffer.push(create_test_dataframe());

    let health = buffer.health_metrics();
    assert_eq!(health.current_size, 2, "Buffer should have 2 items");
    assert_eq!(
        health.buffer_utilization, 0.2,
        "Buffer should have 20% utilization"
    );

    // Process batch
    let result = buffer.process_batch(1);
    assert!(result.is_ok(), "Batch processing should succeed");

    let processed_data = result.unwrap();
    assert_eq!(
        processed_data.data.height(),
        5,
        "Processed batch should have 5 rows"
    );
}

#[test]
fn test_stream_buffer_overflow() {
    // RED: Test stream buffer overflow handling
    let mut buffer = StreamBuffer::new(2);

    // Push more data than buffer can hold
    buffer.push(create_test_dataframe());
    buffer.push(create_test_dataframe());
    buffer.push(create_test_dataframe()); // This should cause overflow

    let health = buffer.health_metrics();
    assert_eq!(health.current_size, 2, "Buffer should maintain max size");
    assert_eq!(
        health.dropped_messages, 1,
        "Buffer should have dropped 1 message"
    );
    assert_eq!(
        health.buffer_utilization, 1.0,
        "Buffer should be at 100% utilization"
    );
}

#[test]
fn test_data_metadata_creation() {
    // RED: Test data metadata creation
    let df = create_test_dataframe();
    let metadata = DataMetadata::from_dataframe(&df);

    assert_eq!(
        metadata.row_count, 5,
        "Metadata should have correct row count"
    );
    assert_eq!(
        metadata.column_count, 3,
        "Metadata should have correct column count"
    );
    assert_eq!(
        metadata.column_types.len(),
        3,
        "Metadata should have 3 column types"
    );
    assert!(
        metadata.memory_usage > 0,
        "Metadata should have positive memory usage"
    );

    // Check column types
    assert!(metadata.column_types.contains_key("x"));
    assert!(metadata.column_types.contains_key("y"));
    assert!(metadata.column_types.contains_key("category"));
}

#[test]
fn test_processing_stats() {
    // RED: Test processing statistics
    let stats = ProcessingStats {
        cpu_time: StdDuration::from_millis(100),
        gpu_time: StdDuration::from_millis(50),
        memory_peak: 1024 * 1024, // 1MB
        cache_hits: 10,
        cache_misses: 2,
    };

    assert_eq!(stats.cpu_time, StdDuration::from_millis(100));
    assert_eq!(stats.gpu_time, StdDuration::from_millis(50));
    assert_eq!(stats.memory_peak, 1024 * 1024);
    assert_eq!(stats.cache_hits, 10);
    assert_eq!(stats.cache_misses, 2);
}

#[test]
fn test_device_capabilities_detection() {
    // RED: Test device capabilities detection
    let capabilities = DeviceCapabilities::detect();

    assert!(
        capabilities.cpu_cores > 0,
        "Should detect at least 1 CPU core"
    );
    assert!(
        capabilities.memory_gb > 0.0,
        "Should detect positive memory"
    );

    // GPU and SIMD detection may vary by platform
    // Just ensure the detection doesn't panic
    let _gpu_available = capabilities.gpu_available;
    let _simd_available = capabilities.simd_available;
}

#[tokio::test]
async fn test_data_processor_caching() {
    // RED: Test that data processor caches results
    let mut processor = DataProcessor::new().unwrap();
    let spec = DataSpec {
        source: DataSource::DataFrame(create_test_dataframe()),
        transforms: vec![],
        filters: vec![],
        aggregations: vec![],
        output_format: OutputFormat::DataFrame,
    };

    // Process the same spec twice
    let result1 = processor.process(&spec).await;
    let result2 = processor.process(&spec).await;

    assert!(result1.is_ok(), "First processing should succeed");
    assert!(result2.is_ok(), "Second processing should succeed");

    // Results should be identical (cached)
    let data1 = result1.unwrap();
    let data2 = result2.unwrap();

    assert_eq!(data1.data.height(), data2.data.height());
    assert_eq!(data1.data.width(), data2.data.width());
    // Note: We can't directly compare DataFrames, but we can compare metadata
    assert_eq!(data1.metadata.row_count, data2.metadata.row_count);
    assert_eq!(data1.metadata.column_count, data2.metadata.column_count);
}

// Integration tests that will fail initially (RED phase)
#[tokio::test]
async fn test_end_to_end_data_processing() {
    // RED: Test complete data processing pipeline
    let mut processor = DataProcessor::new().unwrap();

    // Create a complex data processing specification
    let spec = DataSpec {
        source: DataSource::DataFrame(create_large_test_dataframe(1000)),
        transforms: vec![
            DataTransform::Select {
                columns: vec!["x".to_string(), "y".to_string(), "category".to_string()],
            },
            DataTransform::Cast {
                column: "x".to_string(),
                data_type: helios_core::data::DataType::Float64,
            },
        ],
        filters: vec![Filter::Range {
            column: "x".to_string(),
            min: Some(0.0),
            max: Some(100.0),
        }],
        aggregations: vec![
            Aggregation::GroupBy {
                columns: vec!["category".to_string()],
            },
            Aggregation::Aggregate {
                operations: vec![
                    AggOp::Mean {
                        column: "y".to_string(),
                        alias: Some("avg_y".to_string()),
                    },
                    AggOp::Count {
                        column: "y".to_string(),
                        alias: Some("count".to_string()),
                    },
                ],
            },
        ],
        output_format: OutputFormat::DataFrame,
    };

    let result = processor.process(&spec).await;
    assert!(result.is_ok(), "End-to-end processing should succeed");

    let processed_data = result.unwrap();
    assert!(
        processed_data.data.height() > 0,
        "Processed data should not be empty"
    );
    assert!(
        processed_data.data.height() <= 10,
        "Aggregated data should have fewer rows"
    );
    assert!(
        processed_data.processing_time > StdDuration::from_millis(0),
        "Should have processing time"
    );

    // Verify metadata
    assert!(processed_data.metadata.row_count > 0);
    assert!(processed_data.metadata.column_count >= 3); // category + avg_y + count
    assert!(processed_data.metadata.memory_usage > 0);
}
