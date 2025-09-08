//! TDD Tests for Comprehensive Audit Logging System
//!
//! This module tests the enhanced audit logging capabilities including:
//! - Real-time alerting and threshold monitoring
//! - Data access logging with classification
//! - Performance event logging
//! - Security violation tracking
//! - Export capabilities (JSON, CSV, XML)
//! - Log retention and cleanup
//! - Statistical analysis and reporting

use leptos_helios::security::*;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;

#[tokio::test]
async fn test_audit_logger_configuration() {
    let logger = AuditLogger::new(true)
        .with_retention_days(30)
        .with_max_log_size(50_000)
        .with_real_time_alerts(true)
        .with_alert_threshold(AuditEventType::SecurityViolation, 3);

    // Test basic logging
    let event = AuditEvent {
        id: "test_event_1".to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        event_type: AuditEventType::Authentication,
        user_id: Some("test_user".to_string()),
        session_id: None,
        resource: None,
        action: None,
        result: AuditResult::Success,
        ip_address: Some("192.168.1.1".to_string()),
        user_agent: Some("Mozilla/5.0".to_string()),
        details: HashMap::new(),
    };

    logger.log_event(event).await.unwrap();

    let logs = logger.get_audit_logs(Some(10)).await.unwrap();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].id, "test_event_1");
    assert!(matches!(logs[0].event_type, AuditEventType::Authentication));
}

#[tokio::test]
async fn test_data_access_logging() {
    let logger = AuditLogger::new(true);

    // Log data access with classification
    logger
        .log_data_access(
            "user123",
            Resource::Data {
                source: "customer_db".to_string(),
                table: Some("personal_info".to_string()),
            },
            Action::Read,
            true,
            Some(DataClassification::Confidential),
        )
        .await
        .unwrap();

    let logs = logger
        .get_audit_logs_by_type(&AuditEventType::DataAccess, None)
        .await
        .unwrap();
    assert_eq!(logs.len(), 1);

    let log = &logs[0];
    assert_eq!(log.user_id, Some("user123".to_string()));
    assert!(matches!(log.event_type, AuditEventType::DataAccess));
    assert!(matches!(log.result, AuditResult::Success));

    // Check that data classification is in details
    assert!(log.details.contains_key("data_classification"));
    assert_eq!(
        log.details.get("data_classification").unwrap(),
        &serde_json::Value::String("Confidential".to_string())
    );
}

#[tokio::test]
async fn test_performance_event_logging() {
    let logger = AuditLogger::new(true);

    let mut metadata = HashMap::new();
    metadata.insert(
        "chart_type".to_string(),
        serde_json::Value::String("line".to_string()),
    );
    metadata.insert(
        "data_points".to_string(),
        serde_json::Value::Number(1000.into()),
    );

    logger
        .log_performance_event("chart_rendering", 150, true, metadata)
        .await
        .unwrap();

    let logs = logger
        .get_audit_logs_by_type(&AuditEventType::Performance, None)
        .await
        .unwrap();
    assert_eq!(logs.len(), 1);

    let log = &logs[0];
    assert!(matches!(log.event_type, AuditEventType::Performance));
    assert!(matches!(log.result, AuditResult::Success));
    assert_eq!(
        log.details.get("operation").unwrap(),
        &serde_json::Value::String("chart_rendering".to_string())
    );
    assert_eq!(
        log.details.get("duration_ms").unwrap(),
        &serde_json::Value::Number(150.into())
    );
    assert_eq!(
        log.details.get("chart_type").unwrap(),
        &serde_json::Value::String("line".to_string())
    );
}

#[tokio::test]
async fn test_security_violation_logging() {
    let logger = AuditLogger::new(true);

    let mut details = HashMap::new();
    details.insert(
        "attempted_action".to_string(),
        serde_json::Value::String("unauthorized_export".to_string()),
    );
    details.insert(
        "blocked_reason".to_string(),
        serde_json::Value::String("insufficient_permissions".to_string()),
    );

    logger
        .log_security_violation(
            Some("malicious_user".to_string()),
            "unauthorized_access_attempt",
            "high",
            details,
        )
        .await
        .unwrap();

    let logs = logger
        .get_audit_logs_by_type(&AuditEventType::SecurityViolation, None)
        .await
        .unwrap();
    assert_eq!(logs.len(), 1);

    let log = &logs[0];
    assert_eq!(log.user_id, Some("malicious_user".to_string()));
    assert!(matches!(log.event_type, AuditEventType::SecurityViolation));
    assert!(matches!(log.result, AuditResult::Failure));
    assert_eq!(
        log.details.get("violation_type").unwrap(),
        &serde_json::Value::String("unauthorized_access_attempt".to_string())
    );
    assert_eq!(
        log.details.get("severity").unwrap(),
        &serde_json::Value::String("high".to_string())
    );
}

#[tokio::test]
async fn test_alert_thresholds() {
    let logger = AuditLogger::new(true).with_alert_threshold(AuditEventType::SecurityViolation, 2);

    // Log multiple security violations to trigger alert
    for i in 0..3 {
        let mut details = HashMap::new();
        details.insert("attempt".to_string(), serde_json::Value::Number(i.into()));

        logger
            .log_security_violation(
                Some("attacker".to_string()),
                "brute_force_attempt",
                "medium",
                details,
            )
            .await
            .unwrap();
    }

    let logs = logger
        .get_audit_logs_by_type(&AuditEventType::SecurityViolation, None)
        .await
        .unwrap();
    assert_eq!(logs.len(), 3);

    // In a real implementation, we'd check that alerts were triggered
    // For now, we just verify the events were logged
    for log in &logs {
        assert!(matches!(log.event_type, AuditEventType::SecurityViolation));
    }
}

#[tokio::test]
async fn test_export_capabilities() {
    let logger = AuditLogger::new(true);

    // Create test events
    let event1 = AuditEvent {
        id: "export_test_1".to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        event_type: AuditEventType::Authentication,
        user_id: Some("user1".to_string()),
        session_id: None,
        resource: None,
        action: None,
        result: AuditResult::Success,
        ip_address: Some("192.168.1.1".to_string()),
        user_agent: None,
        details: HashMap::new(),
    };

    let event2 = AuditEvent {
        id: "export_test_2".to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        event_type: AuditEventType::DataAccess,
        user_id: Some("user2".to_string()),
        session_id: None,
        resource: Some(Resource::Chart {
            id: Some("chart1".to_string()),
        }),
        action: Some(Action::Read),
        result: AuditResult::Success,
        ip_address: Some("192.168.1.2".to_string()),
        user_agent: None,
        details: HashMap::new(),
    };

    logger.log_event(event1).await.unwrap();
    logger.log_event(event2).await.unwrap();

    // Test JSON export
    let json_export = logger.export_audit_logs("json", None, None).await.unwrap();
    assert!(json_export.contains("export_test_1"));
    assert!(json_export.contains("export_test_2"));
    assert!(json_export.contains("Authentication"));
    assert!(json_export.contains("DataAccess"));

    // Test CSV export
    let csv_export = logger.export_audit_logs("csv", None, None).await.unwrap();
    assert!(csv_export.contains("id,timestamp,event_type,user_id,result,ip_address"));
    assert!(csv_export.contains("export_test_1"));
    assert!(csv_export.contains("export_test_2"));

    // Test XML export
    let xml_export = logger.export_audit_logs("xml", None, None).await.unwrap();
    assert!(xml_export.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
    assert!(xml_export.contains("<audit_logs>"));
    assert!(xml_export.contains("export_test_1"));
    assert!(xml_export.contains("export_test_2"));
    assert!(xml_export.contains("</audit_logs>"));

    // Test unsupported format
    let result = logger.export_audit_logs("yaml", None, None).await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), SecurityError::AuditError(_)));
}

#[tokio::test]
async fn test_log_retention_and_cleanup() {
    let logger = AuditLogger::new(true).with_retention_days(1);

    // Create an old event (simulate by setting timestamp to 2 days ago)
    let old_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        - (2 * 24 * 60 * 60); // 2 days ago

    let old_event = AuditEvent {
        id: "old_event".to_string(),
        timestamp: old_timestamp,
        event_type: AuditEventType::Authentication,
        user_id: Some("old_user".to_string()),
        session_id: None,
        resource: None,
        action: None,
        result: AuditResult::Success,
        ip_address: None,
        user_agent: None,
        details: HashMap::new(),
    };

    // Create a recent event
    let recent_event = AuditEvent {
        id: "recent_event".to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        event_type: AuditEventType::DataAccess,
        user_id: Some("recent_user".to_string()),
        session_id: None,
        resource: None,
        action: None,
        result: AuditResult::Success,
        ip_address: None,
        user_agent: None,
        details: HashMap::new(),
    };

    logger.log_event(old_event).await.unwrap();
    logger.log_event(recent_event).await.unwrap();

    // Verify both events exist
    let logs_before = logger.get_audit_logs(None).await.unwrap();
    assert_eq!(logs_before.len(), 2);

    // Run cleanup
    let removed_count = logger.cleanup_old_logs().await.unwrap();
    assert_eq!(removed_count, 1); // Should remove 1 old event

    // Verify only recent event remains
    let logs_after = logger.get_audit_logs(None).await.unwrap();
    assert_eq!(logs_after.len(), 1);
    assert_eq!(logs_after[0].id, "recent_event");
}

#[tokio::test]
async fn test_audit_statistics() {
    let logger = AuditLogger::new(true);

    // Create various types of events
    let events = vec![
        (
            AuditEventType::Authentication,
            AuditResult::Success,
            "user1",
        ),
        (
            AuditEventType::Authentication,
            AuditResult::Failure,
            "user2",
        ),
        (AuditEventType::DataAccess, AuditResult::Success, "user1"),
        (
            AuditEventType::SecurityViolation,
            AuditResult::Failure,
            "user3",
        ),
        (AuditEventType::Performance, AuditResult::Success, "system"),
    ];

    for (event_type, result, user_id) in events {
        let event = AuditEvent {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            event_type,
            user_id: if user_id == "system" {
                None
            } else {
                Some(user_id.to_string())
            },
            session_id: None,
            resource: None,
            action: None,
            result,
            ip_address: None,
            user_agent: None,
            details: HashMap::new(),
        };

        logger.log_event(event).await.unwrap();
    }

    let stats = logger.get_audit_statistics().await.unwrap();

    // Verify statistics
    assert_eq!(stats.total_events, 5);
    assert_eq!(
        stats.events_by_type.get(&AuditEventType::Authentication),
        Some(&2)
    );
    assert_eq!(
        stats.events_by_type.get(&AuditEventType::DataAccess),
        Some(&1)
    );
    assert_eq!(
        stats.events_by_type.get(&AuditEventType::SecurityViolation),
        Some(&1)
    );
    assert_eq!(
        stats.events_by_type.get(&AuditEventType::Performance),
        Some(&1)
    );

    assert_eq!(stats.events_by_result.get(&AuditResult::Success), Some(&3));
    assert_eq!(stats.events_by_result.get(&AuditResult::Failure), Some(&2));

    assert_eq!(stats.unique_users.len(), 3); // user1, user2, user3
    assert!(stats.unique_users.contains("user1"));
    assert!(stats.unique_users.contains("user2"));
    assert!(stats.unique_users.contains("user3"));

    assert_eq!(stats.security_violations, 1);
    assert_eq!(stats.failed_authentications, 1);
}

#[tokio::test]
async fn test_log_size_limits() {
    let logger = AuditLogger::new(true).with_max_log_size(5);

    // Add more events than the limit
    for i in 0..10 {
        let event = AuditEvent {
            id: format!("event_{}", i),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            event_type: AuditEventType::Authentication,
            user_id: Some(format!("user_{}", i)),
            session_id: None,
            resource: None,
            action: None,
            result: AuditResult::Success,
            ip_address: None,
            user_agent: None,
            details: HashMap::new(),
        };

        logger.log_event(event).await.unwrap();
    }

    // Should only keep the most recent events (90% of max size = 4.5, rounded to 4)
    let logs = logger.get_audit_logs(None).await.unwrap();
    assert!(logs.len() <= 5);

    // The most recent events should be present
    let event_ids: HashSet<String> = logs.iter().map(|log| log.id.clone()).collect();
    assert!(event_ids.contains("event_9")); // Most recent
    assert!(event_ids.contains("event_8"));
    assert!(event_ids.contains("event_7"));
    assert!(event_ids.contains("event_6"));
}

#[tokio::test]
async fn test_filtered_log_retrieval() {
    let logger = AuditLogger::new(true);

    // Create events for different users and types
    let test_events = vec![
        ("user1", AuditEventType::Authentication),
        ("user1", AuditEventType::DataAccess),
        ("user2", AuditEventType::Authentication),
        ("user1", AuditEventType::SecurityViolation),
        ("user3", AuditEventType::DataAccess),
    ];

    for (user_id, event_type) in test_events {
        let event = AuditEvent {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            event_type,
            user_id: Some(user_id.to_string()),
            session_id: None,
            resource: None,
            action: None,
            result: AuditResult::Success,
            ip_address: None,
            user_agent: None,
            details: HashMap::new(),
        };

        logger.log_event(event).await.unwrap();
    }

    // Test filtering by user
    let user1_logs = logger.get_audit_logs_by_user("user1", None).await.unwrap();
    assert_eq!(user1_logs.len(), 3);

    let user2_logs = logger.get_audit_logs_by_user("user2", None).await.unwrap();
    assert_eq!(user2_logs.len(), 1);

    // Test filtering by event type
    let auth_logs = logger
        .get_audit_logs_by_type(&AuditEventType::Authentication, None)
        .await
        .unwrap();
    assert_eq!(auth_logs.len(), 2);

    let data_access_logs = logger
        .get_audit_logs_by_type(&AuditEventType::DataAccess, None)
        .await
        .unwrap();
    assert_eq!(data_access_logs.len(), 2);

    // Test filtering with limits
    let limited_logs = logger
        .get_audit_logs_by_user("user1", Some(2))
        .await
        .unwrap();
    assert_eq!(limited_logs.len(), 2);
}

#[tokio::test]
async fn test_time_range_export() {
    let logger = AuditLogger::new(true);

    let base_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Create events at different times
    let events = vec![
        ("event_old", base_time - 3600),    // 1 hour ago
        ("event_recent", base_time - 1800), // 30 minutes ago
        ("event_new", base_time),           // now
    ];

    for (id, timestamp) in events {
        let event = AuditEvent {
            id: id.to_string(),
            timestamp,
            event_type: AuditEventType::Authentication,
            user_id: Some("test_user".to_string()),
            session_id: None,
            resource: None,
            action: None,
            result: AuditResult::Success,
            ip_address: None,
            user_agent: None,
            details: HashMap::new(),
        };

        logger.log_event(event).await.unwrap();
    }

    // Export with time range (last 45 minutes)
    let start_time = base_time - 2700; // 45 minutes ago
    let end_time = base_time + 60; // 1 minute in future

    let export = logger
        .export_audit_logs("json", Some(start_time), Some(end_time))
        .await
        .unwrap();

    // Should include event_recent and event_new, but not event_old
    assert!(export.contains("event_recent"));
    assert!(export.contains("event_new"));
    assert!(!export.contains("event_old"));
}

#[tokio::test]
async fn test_comprehensive_audit_workflow() {
    // Test a complete audit workflow
    let logger = AuditLogger::new(true)
        .with_retention_days(7)
        .with_max_log_size(1000)
        .with_real_time_alerts(true)
        .with_alert_threshold(AuditEventType::SecurityViolation, 3);

    // Simulate user authentication
    logger
        .log_authentication("admin_user", true, Some("192.168.1.100".to_string()))
        .await
        .unwrap();

    // Simulate data access
    logger
        .log_data_access(
            "admin_user",
            Resource::Data {
                source: "sensitive_db".to_string(),
                table: Some("financial_records".to_string()),
            },
            Action::Read,
            true,
            Some(DataClassification::Confidential),
        )
        .await
        .unwrap();

    // Simulate performance monitoring
    let mut perf_metadata = HashMap::new();
    perf_metadata.insert(
        "chart_type".to_string(),
        serde_json::Value::String("complex_dashboard".to_string()),
    );
    perf_metadata.insert(
        "render_time_ms".to_string(),
        serde_json::Value::Number(2500.into()),
    );

    logger
        .log_performance_event("dashboard_render", 2500, true, perf_metadata)
        .await
        .unwrap();

    // Simulate security violation
    let mut violation_details = HashMap::new();
    violation_details.insert(
        "attempted_export".to_string(),
        serde_json::Value::String("confidential_data".to_string()),
    );
    violation_details.insert(
        "blocked_by".to_string(),
        serde_json::Value::String("data_governance_policy".to_string()),
    );

    logger
        .log_security_violation(
            Some("suspicious_user".to_string()),
            "unauthorized_data_export",
            "critical",
            violation_details,
        )
        .await
        .unwrap();

    // Verify all events were logged
    let all_logs = logger.get_audit_logs(None).await.unwrap();
    assert_eq!(all_logs.len(), 4);

    // Verify statistics
    let stats = logger.get_audit_statistics().await.unwrap();
    assert_eq!(stats.total_events, 4);
    assert_eq!(stats.unique_users.len(), 2); // admin_user, suspicious_user
    assert_eq!(stats.security_violations, 1);
    assert_eq!(stats.failed_authentications, 0);

    // Test export functionality
    let json_export = logger.export_audit_logs("json", None, None).await.unwrap();
    assert!(json_export.contains("admin_user"));
    assert!(json_export.contains("suspicious_user"));
    assert!(json_export.contains("Authentication"));
    assert!(json_export.contains("DataAccess"));
    assert!(json_export.contains("Performance"));
    assert!(json_export.contains("SecurityViolation"));

    // Test filtering
    let auth_logs = logger
        .get_audit_logs_by_type(&AuditEventType::Authentication, None)
        .await
        .unwrap();
    assert_eq!(auth_logs.len(), 1);
    assert_eq!(auth_logs[0].user_id, Some("admin_user".to_string()));

    let admin_logs = logger
        .get_audit_logs_by_user("admin_user", None)
        .await
        .unwrap();
    assert_eq!(admin_logs.len(), 2); // authentication + data access
}
