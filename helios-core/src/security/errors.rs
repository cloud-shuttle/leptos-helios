//! Security system errors
//!
//! This module provides comprehensive error handling for the security system,
//! including authentication, authorization, token management, and audit errors.

/// Security system errors
#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Authorization denied: {0}")]
    AuthorizationDenied(String),

    #[error("Token error: {0}")]
    TokenError(String),

    #[error("Session error: {0}")]
    SessionError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Audit error: {0}")]
    AuditError(String),

    #[error("Data governance violation: {0}")]
    DataGovernanceViolation(String),
}

impl SecurityError {
    /// Create a new authentication failed error
    pub fn authentication_failed(message: impl Into<String>) -> Self {
        Self::AuthenticationFailed(message.into())
    }

    /// Create a new authorization denied error
    pub fn authorization_denied(message: impl Into<String>) -> Self {
        Self::AuthorizationDenied(message.into())
    }

    /// Create a new token error
    pub fn token_error(message: impl Into<String>) -> Self {
        Self::TokenError(message.into())
    }

    /// Create a new session error
    pub fn session_error(message: impl Into<String>) -> Self {
        Self::SessionError(message.into())
    }

    /// Create a new configuration error
    pub fn configuration_error(message: impl Into<String>) -> Self {
        Self::ConfigurationError(message.into())
    }

    /// Create a new audit error
    pub fn audit_error(message: impl Into<String>) -> Self {
        Self::AuditError(message.into())
    }

    /// Create a new data governance violation error
    pub fn data_governance_violation(message: impl Into<String>) -> Self {
        Self::DataGovernanceViolation(message.into())
    }

    /// Check if this is an authentication error
    pub fn is_authentication_error(&self) -> bool {
        matches!(self, SecurityError::AuthenticationFailed(_))
    }

    /// Check if this is an authorization error
    pub fn is_authorization_error(&self) -> bool {
        matches!(self, SecurityError::AuthorizationDenied(_))
    }

    /// Check if this is a token error
    pub fn is_token_error(&self) -> bool {
        matches!(self, SecurityError::TokenError(_))
    }

    /// Check if this is a session error
    pub fn is_session_error(&self) -> bool {
        matches!(self, SecurityError::SessionError(_))
    }

    /// Check if this is a configuration error
    pub fn is_configuration_error(&self) -> bool {
        matches!(self, SecurityError::ConfigurationError(_))
    }

    /// Check if this is an audit error
    pub fn is_audit_error(&self) -> bool {
        matches!(self, SecurityError::AuditError(_))
    }

    /// Check if this is a data governance violation
    pub fn is_data_governance_violation(&self) -> bool {
        matches!(self, SecurityError::DataGovernanceViolation(_))
    }
}
