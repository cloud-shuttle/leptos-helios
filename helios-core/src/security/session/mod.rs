//! Session management for security

use crate::security::SecurityError;
use crate::security::User;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

/// Session data structure
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
    /// Check if session is expired
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now > self.expires_at
    }

    /// Update last activity timestamp
    pub fn update_activity(&mut self) {
        self.last_activity = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// Get session attribute
    pub fn get_attribute(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
    }

    /// Set session attribute
    pub fn set_attribute(&mut self, key: String, value: String) {
        self.attributes.insert(key, value);
    }
}

/// Session management
pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<String, Session>>>,
    session_timeout: Duration,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            session_timeout: Duration::from_secs(3600), // 1 hour
        }
    }

    /// Create a new session for a user
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

    /// Get a session by ID
    pub async fn get_session(&self, session_id: &str) -> Result<Option<Session>, SecurityError> {
        let sessions = self.sessions.read().await;
        Ok(sessions.get(session_id).cloned())
    }

    /// Update session activity
    pub async fn update_session_activity(&self, session_id: &str) -> Result<(), SecurityError> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.update_activity();
        }
        Ok(())
    }

    /// Invalidate a session
    pub async fn invalidate_session(&self, session_id: &str) -> Result<(), SecurityError> {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id);
        Ok(())
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired_sessions(&self) -> Result<usize, SecurityError> {
        let mut sessions = self.sessions.write().await;
        let initial_count = sessions.len();
        
        sessions.retain(|_, session| !session.is_expired());
        
        let final_count = sessions.len();
        Ok(initial_count - final_count)
    }

    /// Get all active sessions for a user
    pub async fn get_user_sessions(&self, user_id: &str) -> Result<Vec<Session>, SecurityError> {
        let sessions = self.sessions.read().await;
        let user_sessions: Vec<Session> = sessions
            .values()
            .filter(|session| session.user_id == user_id && !session.is_expired())
            .cloned()
            .collect();
        Ok(user_sessions)
    }

    /// Set session timeout
    pub fn set_session_timeout(&mut self, timeout: Duration) {
        self.session_timeout = timeout;
    }

    /// Get session timeout
    pub fn get_session_timeout(&self) -> Duration {
        self.session_timeout
    }

    /// Get total number of active sessions
    pub async fn get_session_count(&self) -> Result<usize, SecurityError> {
        let sessions = self.sessions.read().await;
        Ok(sessions.len())
    }
}
