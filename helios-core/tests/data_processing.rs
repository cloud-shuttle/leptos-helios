//! TDD Tests for Data Processing Pipeline
//!
//! Following RED-GREEN-REFACTOR cycle for data processing functionality

use leptos_helios::*;
use std::time::Duration;

#[test]
fn test_streaming_manager_creation() {
    // RED: Test that streaming manager can be created
    let manager = StreamingManager::new();
    assert!(
        manager.is_ok(),
        "StreamingManager should be created successfully"
    );
}

#[test]
fn test_data_point_creation() {
    // RED: Test that data points can be created
    let data_point = DataPoint {
        timestamp: std::time::Instant::now(),
        value: 42.0,
        metadata: Some("test".to_string()),
    };

    assert_eq!(data_point.value, 42.0);
    assert!(data_point.metadata.is_some());
}

#[test]
fn test_stream_config_creation() {
    // RED: Test that stream configuration can be created
    let config = StreamConfig {
        stream_id: "test_stream".to_string(),
        buffer_size: 1000,
        update_interval: Duration::from_secs(60),
        data_type: DataType::TimeSeries,
    };

    assert_eq!(config.stream_id, "test_stream");
    assert_eq!(config.buffer_size, 1000);
    assert_eq!(config.update_interval, Duration::from_secs(60));
    assert_eq!(config.data_type, DataType::TimeSeries);
}

#[test]
fn test_stream_creation_and_publishing() {
    // RED: Test that streams can be created and data published
    let mut manager = StreamingManager::new().unwrap();

    let config = StreamConfig {
        stream_id: "test_stream".to_string(),
        buffer_size: 100,
        update_interval: Duration::from_secs(30),
        data_type: DataType::TimeSeries,
    };

    // Create a stream
    let result = manager.create_stream(config);
    assert!(result.is_ok(), "Stream creation should succeed");

    // Publish data to the stream
    let data_point = DataPoint {
        timestamp: std::time::Instant::now(),
        value: 123.45,
        metadata: Some("test_data".to_string()),
    };

    let publish_result = manager.publish("test_stream", data_point);
    assert!(publish_result.is_ok(), "Data publishing should succeed");
}

#[test]
fn test_stream_subscription() {
    // RED: Test that streams can be subscribed to
    let mut manager = StreamingManager::new().unwrap();

    let config = StreamConfig {
        stream_id: "subscription_test".to_string(),
        buffer_size: 50,
        update_interval: Duration::from_secs(15),
        data_type: DataType::TimeSeries,
    };

    // Create a stream
    manager.create_stream(config).unwrap();

    // Subscribe to the stream
    let subscriber = manager.subscribe("subscription_test");
    assert!(subscriber.is_ok(), "Stream subscription should succeed");

    let mut subscriber = subscriber.unwrap();

    // Publish some data
    let data_point = DataPoint {
        timestamp: std::time::Instant::now(),
        value: 99.99,
        metadata: Some("subscription_data".to_string()),
    };

    manager.publish("subscription_test", data_point).unwrap();

    // Read data from subscriber
    let data = subscriber.read_new_data();
    assert!(data.is_ok(), "Subscriber should be able to read data");
    let data = data.unwrap();
    assert!(!data.is_empty(), "Subscriber should receive data");
    assert_eq!(data[0].value, 99.99);
}

#[test]
fn test_data_buffer_creation() {
    // RED: Test that data buffers can be created
    let config = DataBufferConfig {
        max_size: 1000,
        eviction_policy: EvictionPolicy::OldestFirst,
        compression_enabled: false,
        compression_threshold: 500,
    };

    let buffer = DataBuffer::new(config);
    assert!(buffer.is_ok(), "DataBuffer should be created successfully");

    let buffer = buffer.unwrap();
    assert_eq!(buffer.size(), 0);
    assert!(!buffer.is_compressed());
}

#[test]
fn test_data_buffer_operations() {
    // RED: Test data buffer operations
    let config = DataBufferConfig {
        max_size: 3,
        eviction_policy: EvictionPolicy::OldestFirst,
        compression_enabled: false,
        compression_threshold: 2,
    };

    let mut buffer = DataBuffer::new(config).unwrap();

    // Add data points
    let data_point1 = DataPoint {
        timestamp: std::time::Instant::now(),
        value: 1.0,
        metadata: Some("first".to_string()),
    };

    let data_point2 = DataPoint {
        timestamp: std::time::Instant::now(),
        value: 2.0,
        metadata: Some("second".to_string()),
    };

    let data_point3 = DataPoint {
        timestamp: std::time::Instant::now(),
        value: 3.0,
        metadata: Some("third".to_string()),
    };

    buffer.add_data_point(data_point1).unwrap();
    assert_eq!(buffer.size(), 1);

    buffer.add_data_point(data_point2).unwrap();
    assert_eq!(buffer.size(), 2);

    buffer.add_data_point(data_point3).unwrap();
    assert_eq!(buffer.size(), 3);

    // Adding one more should trigger eviction
    let data_point4 = DataPoint {
        timestamp: std::time::Instant::now(),
        value: 4.0,
        metadata: Some("fourth".to_string()),
    };

    buffer.add_data_point(data_point4).unwrap();
    assert_eq!(buffer.size(), 3); // Should still be 3 due to eviction
}

#[test]
fn test_transformation_pipeline() {
    // RED: Test data transformation pipeline
    let config = TransformationPipelineConfig {
        transformations: vec![
            Transformation::Filter(FilterConfig {
                field: "value".to_string(),
                operator: FilterOperator::GreaterThan,
                threshold: 10.0,
            }),
            Transformation::Smooth(SmoothConfig {
                window_size: 3,
                method: SmoothingMethod::MovingAverage,
            }),
        ],
    };

    let pipeline = TransformationPipeline::new(config);
    assert!(
        pipeline.is_ok(),
        "TransformationPipeline should be created successfully"
    );

    let pipeline = pipeline.unwrap();

    // Test data processing
    let input_data = vec![
        DataPoint {
            timestamp: std::time::Instant::now(),
            value: 5.0,
            metadata: Some("low".to_string()),
        },
        DataPoint {
            timestamp: std::time::Instant::now(),
            value: 15.0,
            metadata: Some("high".to_string()),
        },
        DataPoint {
            timestamp: std::time::Instant::now(),
            value: 25.0,
            metadata: Some("higher".to_string()),
        },
    ];

    let result = pipeline.process_data(input_data);
    assert!(result.is_ok(), "Data processing should succeed");

    let processed_data = result.unwrap();
    // After filtering, should only have values > 10.0
    assert!(processed_data.len() <= 2);
}

#[test]
fn test_data_quality_monitoring() {
    // RED: Test data quality monitoring
    let config = DataQualityConfig {
        checks: vec![
            QualityCheck::RangeCheck(RangeCheckConfig {
                field: "value".to_string(),
                min_value: 0.0,
                max_value: 100.0,
            }),
            QualityCheck::CompletenessCheck(CompletenessCheckConfig {
                required_fields: vec!["metadata".to_string()],
                threshold: 0.9,
            }),
        ],
        alert_threshold: 0.8,
    };

    let mut monitor = DataQualityMonitor::new(config);
    assert!(
        monitor.is_ok(),
        "DataQualityMonitor should be created successfully"
    );

    let mut monitor = monitor.unwrap();

    // Test with good data
    let good_data = DataPoint {
        timestamp: std::time::Instant::now(),
        value: 50.0,
        metadata: Some("good_data".to_string()),
    };

    let result = monitor.check_data_quality(&good_data);
    assert!(result.is_ok(), "Quality check should succeed for good data");

    // Test with bad data (out of range)
    let bad_data = DataPoint {
        timestamp: std::time::Instant::now(),
        value: 150.0, // Out of range
        metadata: Some("bad_data".to_string()),
    };

    let result = monitor.check_data_quality(&bad_data);
    assert!(
        result.is_ok(),
        "Quality check should succeed even for bad data"
    );

    // Check quality report
    let report = monitor.get_quality_report();
    assert!(report.overall_quality <= 1.0);
    assert!(report.overall_quality >= 0.0);
}

#[test]
fn test_data_cache_operations() {
    // RED: Test data cache operations
    let config = DataCacheConfig {
        cache_size: 2,
        ttl: Duration::from_secs(60),
        eviction_policy: CacheEvictionPolicy::LRU,
        compression: false,
    };

    let mut cache = DataCache::new(config);
    assert!(cache.is_ok(), "DataCache should be created successfully");

    let mut cache = cache.unwrap();

    // Test cache operations
    let test_data = vec![DataPoint {
        timestamp: std::time::Instant::now(),
        value: 1.0,
        metadata: Some("cached_data".to_string()),
    }];

    // Put data in cache
    let result = cache.put("test_key", test_data.clone());
    assert!(result.is_ok(), "Cache put should succeed");

    // Get data from cache
    let retrieved = cache.get("test_key");
    assert!(retrieved.is_ok(), "Cache get should succeed");

    let retrieved_data = retrieved.unwrap();
    assert!(retrieved_data.is_some(), "Cached data should be retrieved");

    // Test hit rate
    let hit_rate = cache.get_hit_rate();
    assert!(hit_rate >= 0.0 && hit_rate <= 1.0);
}
