//! Audit logging for security

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub id: String,
    pub timestamp: u64,
    pub user_id: String,
    pub category: String,
    pub event_type: String,
    pub message: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

impl AuditLogEntry {
    /// Create a new audit log entry
    pub fn new(
        user_id: String,
        category: String,
        event_type: String,
        message: String,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            user_id,
            category,
            event_type,
            message,
            ip_address: None,
            user_agent: None,
        }
    }

    /// Set IP address for the log entry
    pub fn with_ip_address(mut self, ip_address: String) -> Self {
        self.ip_address = Some(ip_address);
        self
    }

    /// Set user agent for the log entry
    pub fn with_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = Some(user_agent);
        self
    }
}

/// Audit logging
pub struct AuditLogger {
    logs: Arc<RwLock<Vec<AuditLogEntry>>>,
    max_logs: usize,
}

impl AuditLogger {
    /// Create a new audit logger
    pub fn new() -> Self {
        Self {
            logs: Arc::new(RwLock::new(Vec::new())),
            max_logs: 10000, // Keep last 10,000 log entries
        }
    }

    /// Log an authentication event
    pub async fn log_auth_event(&self, user_id: &str, event_type: &str, message: &str) {
        self.log_event(user_id, "authentication", event_type, message)
            .await;
    }

    /// Log an authorization event
    pub async fn log_authz_event(&self, user_id: &str, event_type: &str, message: &str) {
        self.log_event(user_id, "authorization", event_type, message)
            .await;
    }

    /// Log a session event
    pub async fn log_session_event(&self, user_id: &str, event_type: &str, message: &str) {
        self.log_event(user_id, "session", event_type, message)
            .await;
    }

    /// Log a data access event
    pub async fn log_data_access_event(&self, user_id: &str, event_type: &str, message: &str) {
        self.log_event(user_id, "data_access", event_type, message)
            .await;
    }

    /// Log a configuration change event
    pub async fn log_config_event(&self, user_id: &str, event_type: &str, message: &str) {
        self.log_event(user_id, "configuration", event_type, message)
            .await;
    }

    /// Internal method to log an event
    async fn log_event(&self, user_id: &str, category: &str, event_type: &str, message: &str) {
        let entry = AuditLogEntry::new(
            user_id.to_string(),
            category.to_string(),
            event_type.to_string(),
            message.to_string(),
        );

        let mut logs = self.logs.write().await;
        logs.push(entry);

        // Keep only the most recent logs
        if logs.len() > self.max_logs {
            let excess = logs.len() - self.max_logs;
            logs.drain(0..excess);
        }
    }

    /// Get audit logs for a specific user
    pub async fn get_user_logs(&self, user_id: &str) -> Vec<AuditLogEntry> {
        let logs = self.logs.read().await;
        logs.iter()
            .filter(|entry| entry.user_id == user_id)
            .cloned()
            .collect()
    }

    /// Get audit logs for a specific category
    pub async fn get_category_logs(&self, category: &str) -> Vec<AuditLogEntry> {
        let logs = self.logs.read().await;
        logs.iter()
            .filter(|entry| entry.category == category)
            .cloned()
            .collect()
    }

    /// Get audit logs within a time range
    pub async fn get_logs_in_range(&self, start_time: u64, end_time: u64) -> Vec<AuditLogEntry> {
        let logs = self.logs.read().await;
        logs.iter()
            .filter(|entry| entry.timestamp >= start_time && entry.timestamp <= end_time)
            .cloned()
            .collect()
    }

    /// Get all audit logs (use with caution for large datasets)
    pub async fn get_all_logs(&self) -> Vec<AuditLogEntry> {
        let logs = self.logs.read().await;
        logs.clone()
    }

    /// Clear all audit logs
    pub async fn clear_logs(&self) {
        let mut logs = self.logs.write().await;
        logs.clear();
    }

    /// Set maximum number of logs to keep
    pub fn set_max_logs(&mut self, max_logs: usize) {
        self.max_logs = max_logs;
    }

    /// Get current number of log entries
    pub async fn get_log_count(&self) -> usize {
        let logs = self.logs.read().await;
        logs.len()
    }

    /// Export logs to JSON format
    pub async fn export_logs_json(&self) -> Result<String, serde_json::Error> {
        let logs = self.logs.read().await;
        serde_json::to_string_pretty(&*logs)
    }

    /// Export logs to CSV format
    pub async fn export_logs_csv(&self) -> String {
        let logs = self.logs.read().await;
        let mut csv = String::from("id,timestamp,user_id,category,event_type,message,ip_address,user_agent\n");
        
        for entry in logs.iter() {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                entry.id,
                entry.timestamp,
                entry.user_id,
                entry.category,
                entry.event_type,
                entry.message,
                entry.ip_address.as_deref().unwrap_or(""),
                entry.user_agent.as_deref().unwrap_or("")
            ));
        }
        
        csv
    }
}
