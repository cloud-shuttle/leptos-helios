//! Streaming Data Tests
//! Tests for real-time data updates and streaming functionality

use leptos_helios::streaming::DataPoint;
use leptos_helios::streaming::DataType;
use leptos_helios::streaming::StreamConfig;
use leptos_helios::*;
use std::time::{Duration, Instant};

/// Helper function to create a base chart config
fn create_base_config(title: &str, width: u32, height: u32) -> ChartConfig {
    ChartConfig {
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
fn test_streaming_manager_initialization() {
    // Given: Streaming manager should be able to initialize
    // When: Creating a streaming manager
    // Then: Manager should be created successfully

    let streaming_manager = StreamingManager::new();
    assert!(
        streaming_manager.is_ok(),
        "Streaming manager should initialize successfully"
    );
}

#[test]
fn test_data_stream_creation() {
    // Given: A streaming manager
    let mut streaming_manager = StreamingManager::new().unwrap();

    // When: Creating a data stream
    let stream_config = StreamConfig {
        stream_id: "test_stream".to_string(),
        buffer_size: 1000,
        update_interval: Duration::from_millis(100),
        data_type: DataType::TimeSeries,
    };

    let result = streaming_manager.create_stream(stream_config);

    // Then: Stream should be created successfully
    assert!(result.is_ok(), "Data stream creation should succeed");
}

#[test]
fn test_data_stream_subscription() {
    // Given: A streaming manager with a data stream
    let mut streaming_manager = StreamingManager::new().unwrap();
    let stream_config = StreamConfig {
        stream_id: "test_stream".to_string(),
        buffer_size: 1000,
        update_interval: Duration::from_millis(100),
        data_type: DataType::TimeSeries,
    };
    streaming_manager.create_stream(stream_config).unwrap();

    // When: Subscribing to the stream
    let subscriber = streaming_manager.subscribe("test_stream");

    // Then: Subscription should be successful
    assert!(subscriber.is_ok(), "Stream subscription should succeed");

    let subscriber = subscriber.unwrap();
    assert_eq!(subscriber.stream_id(), "test_stream");
}

#[test]
fn test_data_stream_publishing() {
    // Given: A streaming manager with a data stream and subscriber
    let mut streaming_manager = StreamingManager::new().unwrap();
    let stream_config = StreamConfig {
        stream_id: "test_stream".to_string(),
        buffer_size: 1000,
        update_interval: Duration::from_millis(100),
        data_type: DataType::TimeSeries,
    };
    streaming_manager.create_stream(stream_config).unwrap();

    let subscriber = streaming_manager.subscribe("test_stream").unwrap();

    // When: Publishing data to the stream
    let data_point = DataPoint {
        timestamp: Instant::now(),
        value: 42.0,
        metadata: Some("test_data".to_string()),
    };

    let result = streaming_manager.publish("test_stream", data_point);

    // Then: Publishing should succeed
    assert!(result.is_ok(), "Data publishing should succeed");
}

#[test]
fn test_data_stream_receiving() {
    // Given: A streaming manager with a data stream and subscriber
    let mut streaming_manager = StreamingManager::new().unwrap();
    let stream_config = StreamConfig {
        stream_id: "test_stream".to_string(),
        buffer_size: 1000,
        update_interval: Duration::from_millis(100),
        data_type: DataType::TimeSeries,
    };
    streaming_manager.create_stream(stream_config).unwrap();

    let subscriber = streaming_manager.subscribe("test_stream").unwrap();

    // When: Publishing data and then receiving it
    let data_point = DataPoint {
        timestamp: Instant::now(),
        value: 42.0,
        metadata: Some("test_data".to_string()),
    };

    streaming_manager
        .publish("test_stream", data_point)
        .unwrap();

    // Give a small delay for the data to be processed
    std::thread::sleep(Duration::from_millis(10));

    let received_data = subscriber.receive();

    // Then: Data should be received
    assert!(
        received_data.is_some(),
        "Data should be received from stream"
    );

    let received_data = received_data.unwrap();
    assert_eq!(received_data.value, 42.0);
    assert_eq!(received_data.metadata, Some("test_data".to_string()));
}

#[test]
fn test_multiple_subscribers() {
    // Given: A streaming manager with a data stream
    let mut streaming_manager = StreamingManager::new().unwrap();
    let stream_config = StreamConfig {
        stream_id: "test_stream".to_string(),
        buffer_size: 1000,
        update_interval: Duration::from_millis(100),
        data_type: DataType::TimeSeries,
    };
    streaming_manager.create_stream(stream_config).unwrap();

    // When: Creating multiple subscribers
    let subscriber1 = streaming_manager.subscribe("test_stream").unwrap();
    let subscriber2 = streaming_manager.subscribe("test_stream").unwrap();
    let subscriber3 = streaming_manager.subscribe("test_stream").unwrap();

    // And: Publishing data
    let data_point = DataPoint {
        timestamp: Instant::now(),
        value: 42.0,
        metadata: Some("test_data".to_string()),
    };

    streaming_manager
        .publish("test_stream", data_point)
        .unwrap();

    // Give a small delay for the data to be processed
    std::thread::sleep(Duration::from_millis(10));

    // Then: All subscribers should receive the data
    assert!(
        subscriber1.receive().is_some(),
        "Subscriber 1 should receive data"
    );
    assert!(
        subscriber2.receive().is_some(),
        "Subscriber 2 should receive data"
    );
    assert!(
        subscriber3.receive().is_some(),
        "Subscriber 3 should receive data"
    );
}

#[test]
fn test_stream_buffer_overflow() {
    // Given: A streaming manager with a small buffer
    let mut streaming_manager = StreamingManager::new().unwrap();
    let stream_config = StreamConfig {
        stream_id: "test_stream".to_string(),
        buffer_size: 5, // Small buffer
        update_interval: Duration::from_millis(100),
        data_type: DataType::TimeSeries,
    };
    streaming_manager.create_stream(stream_config).unwrap();

    let subscriber = streaming_manager.subscribe("test_stream").unwrap();

    // When: Publishing more data than the buffer can hold
    for i in 0..10 {
        let data_point = DataPoint {
            timestamp: Instant::now(),
            value: i as f64,
            metadata: Some(format!("data_{}", i)),
        };
        streaming_manager
            .publish("test_stream", data_point)
            .unwrap();
    }

    // Give a small delay for the data to be processed
    std::thread::sleep(Duration::from_millis(10));

    // Then: Only the most recent data should be available
    let mut received_count = 0;
    while subscriber.receive().is_some() {
        received_count += 1;
    }

    // Should have at most 5 items (buffer size)
    assert!(
        received_count <= 5,
        "Should not exceed buffer size, got {}",
        received_count
    );
}

#[test]
fn test_stream_performance() {
    // Given: A streaming manager with a data stream
    let mut streaming_manager = StreamingManager::new().unwrap();
    let stream_config = StreamConfig {
        stream_id: "test_stream".to_string(),
        buffer_size: 10000,
        update_interval: Duration::from_millis(1),
        data_type: DataType::TimeSeries,
    };
    streaming_manager.create_stream(stream_config).unwrap();

    let subscriber = streaming_manager.subscribe("test_stream").unwrap();

    // When: Publishing many data points quickly
    let start = Instant::now();

    for i in 0..1000 {
        let data_point = DataPoint {
            timestamp: Instant::now(),
            value: i as f64,
            metadata: Some(format!("data_{}", i)),
        };
        streaming_manager
            .publish("test_stream", data_point)
            .unwrap();
    }

    let publish_duration = start.elapsed();

    // Then: Publishing should be fast
    assert!(
        publish_duration.as_millis() < 100,
        "Publishing 1000 points should take less than 100ms, took {}ms",
        publish_duration.as_millis()
    );

    // And: Data should be received
    std::thread::sleep(Duration::from_millis(10));
    let received_data = subscriber.receive();
    assert!(
        received_data.is_some(),
        "Data should be received after high-volume publishing"
    );
}

#[test]
fn test_stream_error_handling() {
    // Given: A streaming manager
    let mut streaming_manager = StreamingManager::new().unwrap();

    // When: Trying to subscribe to a non-existent stream
    let result = streaming_manager.subscribe("non_existent_stream");

    // Then: Should return an error
    assert!(
        result.is_err(),
        "Subscribing to non-existent stream should fail"
    );

    // When: Trying to publish to a non-existent stream
    let data_point = DataPoint {
        timestamp: Instant::now(),
        value: 42.0,
        metadata: Some("test_data".to_string()),
    };

    let result = streaming_manager.publish("non_existent_stream", data_point);

    // Then: Should return an error
    assert!(
        result.is_err(),
        "Publishing to non-existent stream should fail"
    );
}

#[test]
fn test_stream_cleanup() {
    // Given: A streaming manager with a data stream and subscriber
    let mut streaming_manager = StreamingManager::new().unwrap();
    let stream_config = StreamConfig {
        stream_id: "test_stream".to_string(),
        buffer_size: 1000,
        update_interval: Duration::from_millis(100),
        data_type: DataType::TimeSeries,
    };
    streaming_manager.create_stream(stream_config).unwrap();

    let subscriber = streaming_manager.subscribe("test_stream").unwrap();

    // When: Closing the stream
    let result = streaming_manager.close_stream("test_stream");

    // Then: Stream closure should succeed
    assert!(result.is_ok(), "Stream closure should succeed");

    // And: Publishing to closed stream should fail
    let data_point = DataPoint {
        timestamp: Instant::now(),
        value: 42.0,
        metadata: Some("test_data".to_string()),
    };

    let result = streaming_manager.publish("test_stream", data_point);
    assert!(result.is_err(), "Publishing to closed stream should fail");
}

#[test]
fn test_stream_statistics() {
    // Given: A streaming manager with a data stream
    let mut streaming_manager = StreamingManager::new().unwrap();
    let stream_config = StreamConfig {
        stream_id: "test_stream".to_string(),
        buffer_size: 1000,
        update_interval: Duration::from_millis(100),
        data_type: DataType::TimeSeries,
    };
    streaming_manager.create_stream(stream_config).unwrap();

    // When: Publishing some data
    for i in 0..10 {
        let data_point = DataPoint {
            timestamp: Instant::now(),
            value: i as f64,
            metadata: Some(format!("data_{}", i)),
        };
        streaming_manager
            .publish("test_stream", data_point)
            .unwrap();
    }

    // Then: Statistics should be available
    let stats = streaming_manager.get_stream_stats("test_stream");
    assert!(stats.is_ok(), "Stream statistics should be available");

    let stats = stats.unwrap();
    assert_eq!(stats.total_published, 10);
    assert_eq!(stats.active_subscribers, 0); // No subscribers created yet
}

#[test]
fn test_stream_data_types() {
    // Given: A streaming manager
    let mut streaming_manager = StreamingManager::new().unwrap();

    // When: Creating streams with different data types
    let time_series_config = StreamConfig {
        stream_id: "time_series".to_string(),
        buffer_size: 1000,
        update_interval: Duration::from_millis(100),
        data_type: DataType::TimeSeries,
    };

    let event_config = StreamConfig {
        stream_id: "events".to_string(),
        buffer_size: 1000,
        update_interval: Duration::from_millis(100),
        data_type: DataType::Events,
    };

    let result1 = streaming_manager.create_stream(time_series_config);
    let result2 = streaming_manager.create_stream(event_config);

    // Then: Both streams should be created successfully
    assert!(result1.is_ok(), "Time series stream should be created");
    assert!(result2.is_ok(), "Event stream should be created");
}
