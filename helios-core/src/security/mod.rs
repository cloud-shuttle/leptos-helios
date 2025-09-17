//! Enterprise Security & Authentication
//!
//! This module provides comprehensive security and authentication capabilities for Helios,
//! including OAuth, SAML, RBAC, audit logging, and data governance.
//!
//! ## Module Structure
//!
//! - `errors`: Security error types and handling
//! - `types`: Core security types and data structures
//! - `auth`: Authentication providers (OAuth2, SAML, etc.)
//! - `authorization`: Authorization and access control (RBAC, policies)
//! - `audit`: Audit logging and compliance
//! - `data_governance`: Data governance and privacy
//! - `session`: Session management

pub mod auth;
pub mod authorization;
pub mod errors;
pub mod types;

// Re-export main types for convenience
pub use auth::*;
pub use authorization::*;
pub use errors::*;
pub use types::*;

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::{Mutex, RwLock};

/// Main security manager that coordinates all security operations
pub struct SecurityManager {
    auth_providers: HashMap<String, Arc<dyn AuthProvider>>,
    auth_provider: Arc<dyn AuthProvider>,
    authz_provider: Arc<dyn AuthorizationProvider>,
    session_manager: Arc<SessionManager>,
    audit_logger: Arc<AuditLogger>,
}

impl SecurityManager {
    /// Create a new security manager
    pub fn new(
        auth_provider: Arc<dyn AuthProvider>,
        authz_provider: Arc<dyn AuthorizationProvider>,
    ) -> Self {
        Self {
            auth_providers: HashMap::new(),
            auth_provider: auth_provider.clone(),
            authz_provider,
            session_manager: Arc::new(SessionManager::new()),
            audit_logger: Arc::new(AuditLogger::new()),
        }
    }

    /// Add an authentication provider
    pub fn add_auth_provider(&mut self, name: String, provider: Arc<dyn AuthProvider>) {
        self.auth_providers.insert(name, provider);
    }

    /// Authenticate user with credentials
    pub async fn authenticate(
        &self,
        credentials: &Credentials,
    ) -> Result<AuthenticationResult, SecurityError> {
        let result = self.auth_provider.authenticate(credentials).await?;

        if result.success {
            // Log successful authentication
            if let Some(user) = &result.user {
                self.audit_logger
                    .log_auth_event(
                        &user.id,
                        "authentication_success",
                        &format!("User {} authenticated successfully", user.username),
                    )
                    .await;
            }
        } else {
            // Log failed authentication
            self.audit_logger
                .log_auth_event(
                    "unknown",
                    "authentication_failure",
                    &format!(
                        "Authentication failed: {}",
                        result.error_message.as_deref().unwrap_or("Unknown error")
                    ),
                )
                .await;
        }

        Ok(result)
    }

    /// Validate user token
    pub async fn validate_token(&self, token: &str) -> Result<User, SecurityError> {
        let user = self.auth_provider.validate_token(token).await?;

        // Log token validation
        self.audit_logger
            .log_auth_event(
                &user.id,
                "token_validation",
                &format!("Token validated for user {}", user.username),
            )
            .await;

        Ok(user)
    }

    /// Authorize user action
    pub async fn authorize(
        &self,
        user: &User,
        resource: &Resource,
        action: &Action,
    ) -> Result<bool, SecurityError> {
        let context = AuthorizationContext::new(user.clone(), resource.clone(), action.clone());
        let authorized = self.authz_provider.authorize(user, &context).await?;

        // Log authorization decision
        self.audit_logger
            .log_authz_event(
                &user.id,
                if authorized {
                    "authorization_granted"
                } else {
                    "authorization_denied"
                },
                &format!(
                    "User {} {} to {} on {}",
                    user.username,
                    if authorized { "granted" } else { "denied" },
                    action.as_str(),
                    resource.as_string()
                ),
            )
            .await;

        Ok(authorized)
    }

    /// Create a new session
    pub async fn create_session(&self, user: &User) -> Result<Session, SecurityError> {
        let session = self.session_manager.create_session(user).await?;

        // Log session creation
        self.audit_logger
            .log_session_event(
                &user.id,
                "session_created",
                &format!("Session created for user {}", user.username),
            )
            .await;

        Ok(session)
    }

    /// Get session by ID
    pub async fn get_session(&self, session_id: &str) -> Result<Option<Session>, SecurityError> {
        self.session_manager.get_session(session_id).await
    }

    /// Invalidate session
    pub async fn invalidate_session(&self, session_id: &str) -> Result<(), SecurityError> {
        if let Some(session) = self.session_manager.get_session(session_id).await? {
            // Log session invalidation
            self.audit_logger
                .log_session_event(
                    &session.user_id,
                    "session_invalidated",
                    &format!("Session {} invalidated", session_id),
                )
                .await;
        }

        self.session_manager.invalidate_session(session_id).await
    }

    /// Get audit logs
    pub async fn get_audit_logs(
        &self,
        user_id: Option<&str>,
        event_type: Option<&str>,
        limit: Option<usize>,
    ) -> Result<Vec<AuditLogEntry>, SecurityError> {
        self.audit_logger.get_logs(user_id, event_type, limit).await
    }
}

/// Session management
pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<String, Session>>>,
    session_timeout: Duration,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            session_timeout: Duration::from_secs(3600), // 1 hour
        }
    }

    pub async fn create_session(&self, user: &User) -> Result<Session, SecurityError> {
        let session_id = uuid::Uuid::new_v4().to_string();
        let expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + self.session_timeout.as_secs();

        let session = Session {
            id: session_id.clone(),
            user_id: user.id.clone(),
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            expires_at,
            last_activity: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            attributes: HashMap::new(),
        };

        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id, session.clone());

        Ok(session)
    }

    pub async fn get_session(&self, session_id: &str) -> Result<Option<Session>, SecurityError> {
        let sessions = self.sessions.read().await;
        Ok(sessions.get(session_id).cloned())
    }

    pub async fn invalidate_session(&self, session_id: &str) -> Result<(), SecurityError> {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id);
        Ok(())
    }

    pub async fn update_session_activity(&self, session_id: &str) -> Result<(), SecurityError> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.last_activity = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }
        Ok(())
    }

    pub async fn cleanup_expired_sessions(&self) -> Result<usize, SecurityError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut sessions = self.sessions.write().await;
        let initial_count = sessions.len();
        sessions.retain(|_, session| session.expires_at > now);
        let final_count = sessions.len();

        Ok(initial_count - final_count)
    }
}

/// Session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub created_at: u64,
    pub expires_at: u64,
    pub last_activity: u64,
    pub attributes: HashMap<String, String>,
}

impl Session {
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.expires_at <= now
    }

    pub fn set_attribute(&mut self, key: String, value: String) {
        self.attributes.insert(key, value);
    }

    pub fn get_attribute(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
    }
}

/// Audit logging
pub struct AuditLogger {
    logs: Arc<RwLock<Vec<AuditLogEntry>>>,
    max_logs: usize,
}

impl AuditLogger {
    pub fn new() -> Self {
        Self {
            logs: Arc::new(RwLock::new(Vec::new())),
            max_logs: 10000, // Keep last 10,000 log entries
        }
    }

    pub async fn log_auth_event(&self, user_id: &str, event_type: &str, message: &str) {
        self.log_event(user_id, "authentication", event_type, message)
            .await;
    }

    pub async fn log_authz_event(&self, user_id: &str, event_type: &str, message: &str) {
        self.log_event(user_id, "authorization", event_type, message)
            .await;
    }

    pub async fn log_session_event(&self, user_id: &str, event_type: &str, message: &str) {
        self.log_event(user_id, "session", event_type, message)
            .await;
    }

    async fn log_event(&self, user_id: &str, category: &str, event_type: &str, message: &str) {
        let entry = AuditLogEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            user_id: user_id.to_string(),
            category: category.to_string(),
            event_type: event_type.to_string(),
            message: message.to_string(),
            ip_address: None, // Would be populated in real implementation
            user_agent: None, // Would be populated in real implementation
        };

        let mut logs = self.logs.write().await;
        logs.push(entry);

        // Keep only the most recent logs
        if logs.len() > self.max_logs {
            let excess = logs.len() - self.max_logs;
            logs.drain(0..excess);
        }
    }

    pub async fn get_logs(
        &self,
        user_id: Option<&str>,
        event_type: Option<&str>,
        limit: Option<usize>,
    ) -> Result<Vec<AuditLogEntry>, SecurityError> {
        let logs = self.logs.read().await;
        let mut filtered_logs: Vec<AuditLogEntry> = logs
            .iter()
            .filter(|log| {
                if let Some(uid) = user_id {
                    if log.user_id != uid {
                        return false;
                    }
                }
                if let Some(et) = event_type {
                    if log.event_type != et {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();

        // Sort by timestamp (newest first)
        filtered_logs.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        // Apply limit
        if let Some(limit) = limit {
            filtered_logs.truncate(limit);
        }

        Ok(filtered_logs)
    }
}

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
