//! Main security manager

use super::audit::AuditLogger;
use super::session::SessionManager;
use super::{AuthProvider, AuthorizationProvider, SecurityError, User, Credentials, Permission};
// use super::Resource; // Currently unused
use std::collections::HashMap;
use std::sync::Arc;

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

    /// Add an additional authentication provider
    pub fn add_auth_provider(&mut self, name: String, provider: Arc<dyn AuthProvider>) {
        self.auth_providers.insert(name, provider);
    }

    /// Authenticate a user
    pub async fn authenticate(&self, username: &str, password: &str) -> Result<User, SecurityError> {
        let credentials = Credentials::username_password(username.to_string(), password.to_string());
        
        let auth_result = self.auth_provider.authenticate(&credentials).await?;
        
        if auth_result.success {
            let user = auth_result.user.ok_or_else(|| SecurityError::AuthenticationFailed("No user in successful auth result".to_string()))?;
            
            // Log authentication event
            self.audit_logger
                .log_auth_event(&user.id, "login", "User authenticated successfully")
                .await;
            
            Ok(user)
        } else {
            Err(SecurityError::AuthenticationFailed(auth_result.error_message.unwrap_or_else(|| "Authentication failed".to_string())))
        }
    }

    /// Authenticate with a specific provider
    pub async fn authenticate_with_provider(
        &self,
        provider_name: &str,
        username: &str,
        password: &str,
    ) -> Result<User, SecurityError> {
        let provider = self
            .auth_providers
            .get(provider_name)
            .ok_or_else(|| SecurityError::AuthenticationFailed("Provider not found".to_string()))?;
        
        let credentials = Credentials::username_password(username.to_string(), password.to_string());
        
        let auth_result = provider.authenticate(&credentials).await?;
        
        if auth_result.success {
            let user = auth_result.user.ok_or_else(|| SecurityError::AuthenticationFailed("No user in successful auth result".to_string()))?;
            
            // Log authentication event
            self.audit_logger
                .log_auth_event(&user.id, "login", &format!("User authenticated with provider: {}", provider_name))
                .await;
            
            Ok(user)
        } else {
            Err(SecurityError::AuthenticationFailed(auth_result.error_message.unwrap_or_else(|| "Authentication failed".to_string())))
        }
    }

    /// Check if a user has permission for a resource
    pub async fn check_permission(
        &self,
        user: &User,
        resource: &str,
        action: &str,
    ) -> Result<bool, SecurityError> {
        // For now, use a simple permission check - this would need to be expanded
        // based on the actual resource and action mapping logic
        let permission = Permission::ViewCharts; // Default permission for testing
        
        let has_permission = self
            .authz_provider
            .has_permission(user, &permission)
            .await?;
        
        // Log authorization check
        self.audit_logger
            .log_authz_event(
                &user.id,
                "permission_check",
                &format!("Permission check for {} on {}: {}", action, resource, has_permission),
            )
            .await;
        
        Ok(has_permission)
    }

    /// Create a session for a user
    pub async fn create_session(&self, user: &User) -> Result<super::session::Session, SecurityError> {
        let session = self.session_manager.create_session(user).await?;
        
        // Log session creation
        self.audit_logger
            .log_session_event(&user.id, "session_created", "New session created")
            .await;
        
        Ok(session)
    }

    /// Get a session by ID
    pub async fn get_session(&self, session_id: &str) -> Result<Option<super::session::Session>, SecurityError> {
        self.session_manager.get_session(session_id).await
    }

    /// Invalidate a session
    pub async fn invalidate_session(&self, session_id: &str) -> Result<(), SecurityError> {
        if let Some(session) = self.session_manager.get_session(session_id).await? {
            // Log session invalidation
            self.audit_logger
                .log_session_event(&session.user_id, "session_invalidated", "Session invalidated")
                .await;
        }
        
        self.session_manager.invalidate_session(session_id).await
    }

    /// Update session activity
    pub async fn update_session_activity(&self, session_id: &str) -> Result<(), SecurityError> {
        self.session_manager.update_session_activity(session_id).await
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired_sessions(&self) -> Result<usize, SecurityError> {
        let cleaned_count = self.session_manager.cleanup_expired_sessions().await?;
        
        if cleaned_count > 0 {
            // Log cleanup event
            self.audit_logger
                .log_session_event("system", "cleanup", &format!("Cleaned up {} expired sessions", cleaned_count))
                .await;
        }
        
        Ok(cleaned_count)
    }

    /// Get audit logs for a user
    pub async fn get_user_audit_logs(&self, user_id: &str) -> Vec<super::audit::AuditLogEntry> {
        self.audit_logger.get_user_logs(user_id).await
    }

    /// Get audit logs for a category
    pub async fn get_category_audit_logs(&self, category: &str) -> Vec<super::audit::AuditLogEntry> {
        self.audit_logger.get_category_logs(category).await
    }

    /// Export audit logs to JSON
    pub async fn export_audit_logs_json(&self) -> Result<String, serde_json::Error> {
        self.audit_logger.export_logs_json().await
    }

    /// Export audit logs to CSV
    pub async fn export_audit_logs_csv(&self) -> String {
        self.audit_logger.export_logs_csv().await
    }

    /// Get session manager reference
    pub fn session_manager(&self) -> Arc<SessionManager> {
        self.session_manager.clone()
    }

    /// Get audit logger reference
    pub fn audit_logger(&self) -> Arc<AuditLogger> {
        self.audit_logger.clone()
    }

    /// Get authentication provider reference
    pub fn auth_provider(&self) -> Arc<dyn AuthProvider> {
        self.auth_provider.clone()
    }

    /// Get authorization provider reference
    pub fn authz_provider(&self) -> Arc<dyn AuthorizationProvider> {
        self.authz_provider.clone()
    }
}
