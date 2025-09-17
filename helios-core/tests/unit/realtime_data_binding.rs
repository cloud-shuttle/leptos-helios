//! Real-time Data Binding Tests
//! Tests for live data updates, streaming, and reactive chart updates

use leptos_helios::*;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Helper function to create a base chart config
fn create_base_config(title: &str, width: u32, height: u32) -> BaseChartConfig {
    BaseChartConfig {
        width,
        height,
        title: title.to_string(),
        x_label: "X Axis".to_string(),
        y_label: "Y Axis".to_string(),
        show_grid: true,
        background_color: "#ffffff".to_string(),
        text_color: "#000000".to_string(),
    }
}

#[test]
fn test_realtime_data_stream() {
    // Given: A real-time data stream
    let stream_config = StreamConfig {
        stream_id: "test_stream".to_string(),
        buffer_size: 1000,
        update_interval: Duration::from_millis(100),
        data_type: DataType::TimeSeries,
    };

    let mut streaming_manager = StreamingManager::new().unwrap();
    streaming_manager
        .create_stream(stream_config.clone())
        .unwrap();

    // When: Publishing data to the stream
    let data_point = DataPoint {
        timestamp: Instant::now(),
        value: 42.0,
        metadata: Some("test_data".to_string()),
    };

    let result = streaming_manager.publish("test_stream", data_point.clone());

    // Then: Should publish successfully
    assert!(result.is_ok(), "Data publishing should succeed");

    // Verify stream statistics
    let stats = streaming_manager.get_stream_stats("test_stream").unwrap();
    assert_eq!(stats.total_published, 1);
    assert_eq!(stats.buffer_usage, 1);
}

#[test]
fn test_data_stream_subscription() {
    // Given: A data stream with subscriber
    let stream_config = StreamConfig {
        stream_id: "subscription_test".to_string(),
        buffer_size: 100,
        update_interval: Duration::from_millis(50),
        data_type: DataType::Metrics,
    };

    let mut streaming_manager = StreamingManager::new().unwrap();
    streaming_manager.create_stream(stream_config).unwrap();

    // When: Subscribing to the stream
    let subscriber = streaming_manager.subscribe("subscription_test").unwrap();

    // Then: Should create subscriber successfully
    assert!(subscriber.is_active(), "Subscriber should be active");

    // Publish some data
    for i in 0..5 {
        let data_point = DataPoint {
            timestamp: Instant::now(),
            value: i as f64,
            metadata: None,
        };
        streaming_manager
            .publish("subscription_test", data_point)
            .unwrap();
    }

    // Verify subscriber can read data
    let data = subscriber.read_new_data().unwrap();
    assert_eq!(data.len(), 5, "Should have 5 data points");
}

#[test]
fn test_reactive_chart_updates() {
    // Given: A chart with reactive data binding
    let data = Arc::new(Mutex::new(vec![(0.0, 1.0), (1.0, 2.0), (2.0, 1.5)]));
    let config = LineChartConfig {
        base: create_base_config("Reactive Chart", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let mut reactive_chart = ReactiveChart::new(data.clone(), config).unwrap();

    // When: Updating data reactively
    {
        let mut chart_data = data.lock().unwrap();
        chart_data.push((3.0, 3.0));
        chart_data.push((4.0, 2.5));
    }

    let update_result = reactive_chart.update_data();

    // Then: Should update chart data
    assert!(update_result.is_ok(), "Reactive update should succeed");

    let current_data = reactive_chart.get_current_data();
    assert_eq!(
        current_data.len(),
        5,
        "Should have 5 data points after update"
    );
}

#[test]
fn test_websocket_data_binding() {
    // Given: A WebSocket data binding configuration
    let ws_config = WebSocketConfig {
        url: "ws://localhost:8080/data".to_string(),
        reconnect_interval: Duration::from_secs(5),
        max_reconnect_attempts: 10,
        heartbeat_interval: Duration::from_secs(30),
        message_format: MessageFormat::JSON,
    };

    let mut ws_binding = WebSocketDataBinding::new(ws_config).unwrap();

    // When: Simulating WebSocket connection (mock)
    let connection_result = ws_binding.connect();

    // Then: Should handle connection (mock success)
    assert!(
        connection_result.is_ok(),
        "WebSocket connection should be handled"
    );

    // Simulate receiving data
    let mock_data = r#"{"timestamp": 1640995200, "value": 42.5, "label": "test"}"#;
    let parse_result = ws_binding.parse_message(mock_data);

    assert!(parse_result.is_ok(), "Message parsing should succeed");
}

#[test]
fn test_data_transformation_pipeline() {
    // Given: A data transformation pipeline
    let pipeline_config = TransformationPipelineConfig {
        transformations: vec![
            Transformation::Filter(FilterConfig {
                field: "value".to_string(),
                operator: FilterOperator::GreaterThan,
                threshold: 10.0,
            }),
            Transformation::Aggregate(AggregateConfig {
                field: "value".to_string(),
                operation: AggregateOperation::Average,
                window_size: Duration::from_secs(60),
            }),
            Transformation::Smooth(SmoothConfig {
                window_size: 5,
                method: SmoothingMethod::MovingAverage,
            }),
        ],
    };

    let mut pipeline = TransformationPipeline::new(pipeline_config).unwrap();

    // When: Processing data through pipeline
    let input_data = vec![
        DataPoint {
            timestamp: Instant::now(),
            value: 5.0,
            metadata: None,
        },
        DataPoint {
            timestamp: Instant::now(),
            value: 15.0,
            metadata: None,
        },
        DataPoint {
            timestamp: Instant::now(),
            value: 25.0,
            metadata: None,
        },
        DataPoint {
            timestamp: Instant::now(),
            value: 8.0,
            metadata: None,
        },
        DataPoint {
            timestamp: Instant::now(),
            value: 30.0,
            metadata: None,
        },
    ];

    let result = pipeline.process_data(input_data);

    // Then: Should transform data successfully
    assert!(result.is_ok(), "Data transformation should succeed");

    let transformed_data = result.unwrap();
    assert!(transformed_data.len() > 0, "Should have transformed data");
}

#[test]
fn test_data_buffer_management() {
    // Given: A data buffer with size limits
    let buffer_config = DataBufferConfig {
        max_size: 1000,
        eviction_policy: EvictionPolicy::OldestFirst,
        compression_enabled: true,
        compression_threshold: 500,
    };

    let mut data_buffer = DataBuffer::new(buffer_config).unwrap();

    // When: Adding data beyond buffer limit
    for i in 0..1500 {
        let data_point = DataPoint {
            timestamp: Instant::now(),
            value: i as f64,
            metadata: None,
        };
        data_buffer.add_data_point(data_point).unwrap();
    }

    // Then: Should manage buffer size
    assert_eq!(data_buffer.size(), 1000, "Buffer should maintain max size");
    assert!(data_buffer.is_compressed(), "Buffer should be compressed");
}

#[test]
fn test_real_time_performance() {
    // Given: A high-frequency data stream
    let stream_config = StreamConfig {
        stream_id: "performance_test".to_string(),
        buffer_size: 10000,
        update_interval: Duration::from_millis(1), // 1000 updates per second
        data_type: DataType::Events,
    };

    let mut streaming_manager = StreamingManager::new().unwrap();
    streaming_manager.create_stream(stream_config).unwrap();

    // When: Publishing data at high frequency
    let start = Instant::now();
    let mut published_count = 0;

    for i in 0..1000 {
        let data_point = DataPoint {
            timestamp: Instant::now(),
            value: (i as f64).sin(),
            metadata: Some(format!("event_{}", i)),
        };

        if streaming_manager
            .publish("performance_test", data_point)
            .is_ok()
        {
            published_count += 1;
        }
    }

    let duration = start.elapsed();

    // Then: Should handle high-frequency updates efficiently
    assert_eq!(published_count, 1000, "Should publish all data points");
    assert!(
        duration.as_millis() < 100,
        "1000 updates should take less than 100ms, took {}ms",
        duration.as_millis()
    );

    let stats = streaming_manager
        .get_stream_stats("performance_test")
        .unwrap();
    assert_eq!(stats.total_published, 1000);
}

#[test]
fn test_data_synchronization() {
    // Given: Multiple data sources that need synchronization
    let sync_config = DataSynchronizationConfig {
        sources: vec![
            DataSource {
                id: "source1".to_string(),
                stream_id: "stream1".to_string(),
                priority: 1,
                enabled: true,
            },
            DataSource {
                id: "source2".to_string(),
                stream_id: "stream2".to_string(),
                priority: 2,
                enabled: true,
            },
        ],
        sync_strategy: SyncStrategy::TimestampBased,
        max_drift: Duration::from_millis(100),
    };

    let mut sync_manager = DataSynchronizationManager::new(sync_config).unwrap();

    // When: Synchronizing data from multiple sources
    let data1 = DataPoint {
        timestamp: Instant::now(),
        value: 10.0,
        metadata: Some("source1".to_string()),
    };

    let data2 = DataPoint {
        timestamp: Instant::now(),
        value: 20.0,
        metadata: Some("source2".to_string()),
    };

    sync_manager.add_data("source1", data1).unwrap();
    sync_manager.add_data("source2", data2).unwrap();

    let synchronized_data = sync_manager.get_synchronized_data().unwrap();

    // Then: Should synchronize data successfully
    assert_eq!(
        synchronized_data.len(),
        2,
        "Should have synchronized data from both sources"
    );
}

#[test]
fn test_data_quality_monitoring() {
    // Given: A data quality monitoring system
    let quality_config = DataQualityConfig {
        checks: vec![
            QualityCheck::RangeCheck(RangeCheckConfig {
                field: "value".to_string(),
                min_value: 0.0,
                max_value: 100.0,
            }),
            QualityCheck::CompletenessCheck(CompletenessCheckConfig {
                required_fields: vec!["timestamp".to_string(), "value".to_string()],
                threshold: 0.95,
            }),
            QualityCheck::ConsistencyCheck(ConsistencyCheckConfig {
                field: "value".to_string(),
                max_change_rate: 0.5,
            }),
        ],
        alert_threshold: 0.9,
    };

    let mut quality_monitor = DataQualityMonitor::new(quality_config).unwrap();

    // When: Monitoring data quality
    let test_data = vec![
        DataPoint {
            timestamp: Instant::now(),
            value: 50.0,
            metadata: None,
        }, // Good
        DataPoint {
            timestamp: Instant::now(),
            value: 150.0,
            metadata: None,
        }, // Out of range
        DataPoint {
            timestamp: Instant::now(),
            value: 25.0,
            metadata: None,
        }, // Good
    ];

    for data_point in test_data {
        quality_monitor.check_data_quality(&data_point).unwrap();
    }

    let quality_report = quality_monitor.get_quality_report();

    // Then: Should monitor data quality
    assert!(
        quality_report.overall_quality < 1.0,
        "Should detect quality issues"
    );
    assert!(
        quality_report.issues.len() > 0,
        "Should have quality issues"
    );
}

#[test]
fn test_data_caching_strategy() {
    // Given: A data caching system
    let cache_config = DataCacheConfig {
        cache_size: 1000,
        ttl: Duration::from_secs(300),
        eviction_policy: CacheEvictionPolicy::LRU,
        compression: true,
    };

    let mut data_cache = DataCache::new(cache_config).unwrap();

    // When: Caching and retrieving data
    let data_key = "test_data_key".to_string();
    let data_value = vec![
        DataPoint {
            timestamp: Instant::now(),
            value: 1.0,
            metadata: None,
        },
        DataPoint {
            timestamp: Instant::now(),
            value: 2.0,
            metadata: None,
        },
        DataPoint {
            timestamp: Instant::now(),
            value: 3.0,
            metadata: None,
        },
    ];

    data_cache.put(&data_key, data_value.clone()).unwrap();
    let retrieved_data = data_cache.get(&data_key).unwrap();

    // Then: Should cache and retrieve data
    assert!(retrieved_data.is_some(), "Should retrieve cached data");
    assert_eq!(
        retrieved_data.unwrap().len(),
        3,
        "Should retrieve correct data"
    );

    // Test cache hit rate
    let hit_rate = data_cache.get_hit_rate();
    assert!(hit_rate > 0.0, "Should have cache hits");
}
