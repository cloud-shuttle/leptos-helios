//! TDD Phase 4: Enterprise Security Tests
//!
//! This module implements comprehensive Test-Driven Development tests for enterprise
//! security features, including OAuth2, SAML, RBAC, and audit logging.
//!
//! ## Test Coverage
//!
//! - **OAuth2 Authentication**: Authorization code flow, token validation, user info retrieval
//! - **SAML Authentication**: SSO flow, assertion validation, user mapping
//! - **RBAC Authorization**: Role-based access control, permission checking, policy evaluation
//! - **Audit Logging**: Security event logging, compliance tracking, log retrieval
//! - **Data Governance**: Data classification, access policies, compliance validation
//! - **Session Management**: Token lifecycle, session security, timeout handling
//! - **Security Policies**: Access control policies, condition evaluation, policy enforcement
//!
//! ## TDD Methodology
//!
//! 1. **RED**: Write failing tests first
//! 2. **GREEN**: Implement minimal code to pass tests
//! 3. **REFACTOR**: Improve code quality while maintaining test coverage

use leptos_helios::security::*;
use std::collections::{HashMap, HashSet};
use tokio::time::{sleep, Duration};

/// Test suite for OAuth2 authentication
mod oauth2_tests {
    use super::*;

    #[tokio::test]
    async fn test_oauth2_provider_creation() {
        // RED: Test OAuth2 provider creation
        let provider = OAuth2Provider::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "https://auth.example.com/authorize".to_string(),
            "https://auth.example.com/token".to_string(),
            "https://auth.example.com/userinfo".to_string(),
            "https://app.example.com/callback".to_string(),
        );

        // GREEN: Verify provider configuration
        assert_eq!(provider.provider_type(), "oauth2");
    }

    #[tokio::test]
    async fn test_oauth2_authentication_flow() {
        // RED: Test OAuth2 authentication with authorization code
        let provider = OAuth2Provider::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "https://auth.example.com/authorize".to_string(),
            "https://auth.example.com/token".to_string(),
            "https://auth.example.com/userinfo".to_string(),
            "https://app.example.com/callback".to_string(),
        );

        let credentials = Credentials {
            credential_type: CredentialType::OAuth2,
            username: None,
            password: None,
            token: Some("test_auth_code".to_string()),
            additional_data: HashMap::new(),
        };

        // GREEN: Test authentication
        let result = provider.authenticate(&credentials).await;
        assert!(result.is_ok());

        let auth_result = result.unwrap();
        assert!(auth_result.success);
        assert!(auth_result.user.is_some());
        assert!(auth_result.access_token.is_some());
        assert_eq!(auth_result.token_type, Some("Bearer".to_string()));

        let user = auth_result.user.unwrap();
        assert_eq!(user.id, "oauth2_user_123");
        assert_eq!(user.username, "oauth2user");
        assert_eq!(user.email, "user@example.com");
    }

    #[tokio::test]
    async fn test_oauth2_token_validation() {
        // RED: Test OAuth2 token validation
        let provider = OAuth2Provider::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "https://auth.example.com/authorize".to_string(),
            "https://auth.example.com/token".to_string(),
            "https://auth.example.com/userinfo".to_string(),
            "https://app.example.com/callback".to_string(),
        );

        // First authenticate to get a token
        let credentials = Credentials {
            credential_type: CredentialType::OAuth2,
            username: None,
            password: None,
            token: Some("test_auth_code".to_string()),
            additional_data: HashMap::new(),
        };

        let auth_result = provider.authenticate(&credentials).await.unwrap();
        let token = auth_result.access_token.unwrap();

        // GREEN: Test token validation
        let user = provider.validate_token(&token).await;
        assert!(user.is_ok());

        let validated_user = user.unwrap();
        assert_eq!(validated_user.id, "oauth2_user_123");
        assert_eq!(validated_user.username, "oauth2user");
    }

    #[tokio::test]
    async fn test_oauth2_token_refresh() {
        // RED: Test OAuth2 token refresh
        let provider = OAuth2Provider::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "https://auth.example.com/authorize".to_string(),
            "https://auth.example.com/token".to_string(),
            "https://auth.example.com/userinfo".to_string(),
            "https://app.example.com/callback".to_string(),
        );

        // GREEN: Test token refresh
        let refresh_result = provider.refresh_token("test_refresh_token").await;
        assert!(refresh_result.is_ok());

        let refresh_auth_result = refresh_result.unwrap();
        assert!(refresh_auth_result.success);
        assert!(refresh_auth_result.access_token.is_some());
    }

    #[tokio::test]
    async fn test_oauth2_logout() {
        // RED: Test OAuth2 logout
        let provider = OAuth2Provider::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "https://auth.example.com/authorize".to_string(),
            "https://auth.example.com/token".to_string(),
            "https://auth.example.com/userinfo".to_string(),
            "https://app.example.com/callback".to_string(),
        );

        // GREEN: Test logout
        let logout_result = provider.logout("test_access_token").await;
        assert!(logout_result.is_ok());
    }

    #[tokio::test]
    async fn test_oauth2_invalid_credentials() {
        // RED: Test OAuth2 with invalid credentials
        let provider = OAuth2Provider::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "https://auth.example.com/authorize".to_string(),
            "https://auth.example.com/token".to_string(),
            "https://auth.example.com/userinfo".to_string(),
            "https://app.example.com/callback".to_string(),
        );

        let invalid_credentials = Credentials {
            credential_type: CredentialType::UsernamePassword, // Wrong type
            username: Some("user".to_string()),
            password: Some("pass".to_string()),
            token: None,
            additional_data: HashMap::new(),
        };

        // GREEN: Test invalid credentials handling
        let result = provider.authenticate(&invalid_credentials).await;
        assert!(result.is_err());

        if let Err(SecurityError::AuthenticationFailed(msg)) = result {
            assert!(msg.contains("Unsupported credential type"));
        }
    }
}

/// Test suite for SAML authentication
mod saml_tests {
    use super::*;

    #[tokio::test]
    async fn test_saml_provider_creation() {
        // RED: Test SAML provider creation
        let provider = SAMLProvider::new(
            "test_entity_id".to_string(),
            "https://saml.example.com/sso".to_string(),
            "test_x509_cert".to_string(),
            "test_private_key".to_string(),
        );

        // GREEN: Verify provider configuration
        assert_eq!(provider.provider_type(), "saml");
    }

    #[tokio::test]
    async fn test_saml_authentication_flow() {
        // RED: Test SAML authentication with SAML response
        let provider = SAMLProvider::new(
            "test_entity_id".to_string(),
            "https://saml.example.com/sso".to_string(),
            "test_x509_cert".to_string(),
            "test_private_key".to_string(),
        );

        let credentials = Credentials {
            credential_type: CredentialType::SAML,
            username: None,
            password: None,
            token: Some("test_saml_response".to_string()),
            additional_data: HashMap::new(),
        };

        // GREEN: Test authentication
        let result = provider.authenticate(&credentials).await;
        assert!(result.is_ok());

        let auth_result = result.unwrap();
        assert!(auth_result.success);
        assert!(auth_result.user.is_some());
        assert!(auth_result.access_token.is_some());
        assert_eq!(auth_result.token_type, Some("SAML".to_string()));

        let user = auth_result.user.unwrap();
        assert_eq!(user.id, "saml_user_456");
        assert_eq!(user.username, "samluser");
        assert_eq!(user.email, "samluser@example.com");
    }

    #[tokio::test]
    async fn test_saml_token_validation() {
        // RED: Test SAML token validation
        let provider = SAMLProvider::new(
            "test_entity_id".to_string(),
            "https://saml.example.com/sso".to_string(),
            "test_x509_cert".to_string(),
            "test_private_key".to_string(),
        );

        // First authenticate to get a token
        let credentials = Credentials {
            credential_type: CredentialType::SAML,
            username: None,
            password: None,
            token: Some("test_saml_response".to_string()),
            additional_data: HashMap::new(),
        };

        let auth_result = provider.authenticate(&credentials).await.unwrap();
        let token = auth_result.access_token.unwrap();

        // GREEN: Test token validation
        let user = provider.validate_token(&token).await;
        assert!(user.is_ok());

        let validated_user = user.unwrap();
        assert_eq!(validated_user.id, "saml_user_456");
        assert_eq!(validated_user.username, "samluser");
    }

    #[tokio::test]
    async fn test_saml_authn_request_generation() {
        // RED: Test SAML authentication request generation
        let provider = SAMLProvider::new(
            "test_entity_id".to_string(),
            "https://saml.example.com/sso".to_string(),
            "test_x509_cert".to_string(),
            "test_private_key".to_string(),
        );

        // GREEN: Test authn request generation
        let request = provider.generate_saml_request(Some("test_relay_state"));
        assert!(request.is_ok());

        let authn_request = request.unwrap();
        assert!(authn_request.contains("test_entity_id"));
    }
}

/// Test suite for RBAC authorization
mod rbac_tests {
    use super::*;

    #[tokio::test]
    async fn test_rbac_provider_creation() {
        // RED: Test RBAC provider creation
        let _rbac = RBACProvider::new();

        // GREEN: Verify provider is created
        // Note: Initialization happens asynchronously, so we wait a bit
        sleep(Duration::from_millis(100)).await;
    }

    #[tokio::test]
    async fn test_rbac_role_assignment() {
        // RED: Test RBAC role assignment
        let rbac = RBACProvider::new();
        sleep(Duration::from_millis(100)).await; // Wait for initialization

        // GREEN: Test user role assignment
        let assignment_result = rbac.assign_role_to_user("user123", "user").await;
        assert!(assignment_result.is_ok());

        // Test role removal
        let removal_result = rbac.remove_role_from_user("user123", "user").await;
        assert!(removal_result.is_ok());
    }

    #[tokio::test]
    async fn test_rbac_authorization() {
        // RED: Test RBAC authorization
        let rbac = RBACProvider::new();
        sleep(Duration::from_millis(100)).await; // Wait for initialization

        // Create a test user with admin role
        let user = User {
            id: "admin_user".to_string(),
            username: "admin".to_string(),
            email: "admin@example.com".to_string(),
            display_name: "Admin User".to_string(),
            roles: {
                let mut roles = HashSet::new();
                roles.insert("admin".to_string());
                roles
            },
            permissions: HashSet::new(),
            groups: HashSet::new(),
            attributes: HashMap::new(),
            created_at: 1234567890,
            last_login: None,
            is_active: true,
        };

        // Assign admin role to user in RBAC provider
        rbac.assign_role_to_user("admin_user", "admin")
            .await
            .unwrap();

        let context = AuthorizationContext {
            resource: Resource::Chart {
                id: Some("chart_123".to_string()),
            },
            action: Action::Read,
            environment: HashMap::new(),
            ip_address: None,
            user_agent: None,
        };

        // GREEN: Test authorization
        let authorized = rbac.authorize(&user, &context).await;
        assert!(authorized.is_ok());

        let is_authorized = authorized.unwrap();
        assert!(is_authorized); // Admin should have access
    }
}

/// Test suite for audit logging
mod audit_logging_tests {
    use super::*;

    #[tokio::test]
    async fn test_audit_logger_creation() {
        // RED: Test audit logger creation
        let _logger = AuditLogger::new(true);

        // GREEN: Verify logger is created and enabled
        // Note: We can't directly test the enabled state, but we can test logging
    }

    #[tokio::test]
    async fn test_audit_authentication_logging() {
        // RED: Test audit authentication logging
        let logger = AuditLogger::new(true);

        // GREEN: Test authentication logging
        let log_result = logger
            .log_authentication("user123", true, Some("192.168.1.1".to_string()))
            .await;
        assert!(log_result.is_ok());

        // Test failed authentication logging
        let failed_log_result = logger
            .log_authentication("user456", false, Some("192.168.1.2".to_string()))
            .await;
        assert!(failed_log_result.is_ok());
    }

    #[tokio::test]
    async fn test_audit_authorization_logging() {
        // RED: Test audit authorization logging
        let logger = AuditLogger::new(true);

        let resource = Resource::Chart {
            id: Some("chart_123".to_string()),
        };

        let action = Action::Read;

        // GREEN: Test authorization logging
        let log_result = logger
            .log_authorization("user123", resource.clone(), action.clone(), true)
            .await;
        assert!(log_result.is_ok());

        // Test failed authorization logging
        let failed_log_result = logger
            .log_authorization("user456", resource, action, false)
            .await;
        assert!(failed_log_result.is_ok());
    }

    #[tokio::test]
    async fn test_audit_log_retrieval() {
        // RED: Test audit log retrieval
        let logger = AuditLogger::new(true);

        // Log some events first
        logger
            .log_authentication("user123", true, Some("192.168.1.1".to_string()))
            .await
            .unwrap();
        logger
            .log_authentication("user456", false, Some("192.168.1.2".to_string()))
            .await
            .unwrap();

        // GREEN: Test log retrieval
        let logs = logger.get_audit_logs(Some(10)).await;
        assert!(logs.is_ok());

        let audit_logs = logs.unwrap();
        assert_eq!(audit_logs.len(), 2);

        // Verify log content
        let auth_log = &audit_logs[0]; // Most recent first
        assert_eq!(auth_log.user_id, Some("user456".to_string()));
        assert!(matches!(
            auth_log.event_type,
            AuditEventType::Authentication
        ));
        assert!(matches!(auth_log.result, AuditResult::Failure));
    }

    #[tokio::test]
    async fn test_audit_custom_event_logging() {
        // RED: Test audit custom event logging
        let logger = AuditLogger::new(true);

        let mut details = HashMap::new();
        details.insert(
            "custom_field".to_string(),
            serde_json::Value::String("custom_value".to_string()),
        );

        let event = AuditEvent {
            id: "custom_event_123".to_string(),
            timestamp: 1234567890,
            event_type: AuditEventType::Custom("CustomEvent".to_string()),
            user_id: Some("user123".to_string()),
            session_id: Some("session_456".to_string()),
            resource: None,
            action: None,
            result: AuditResult::Success,
            ip_address: Some("192.168.1.1".to_string()),
            user_agent: Some("Mozilla/5.0".to_string()),
            details,
        };

        // GREEN: Test custom event logging
        let log_result = logger.log_event(event).await;
        assert!(log_result.is_ok());
    }
}

/// Test suite for data governance
mod data_governance_tests {
    use super::*;

    #[tokio::test]
    async fn test_data_governance_creation() {
        // RED: Test data governance system creation
        let _governance = DataGovernance::new();

        // GREEN: Verify governance system is created
        // Note: We can't directly test internal state, but we can test operations
    }

    #[tokio::test]
    async fn test_data_classification() {
        // RED: Test data classification
        let governance = DataGovernance::new();

        // GREEN: Test data classification
        let classification_result = governance
            .classify_data("dataset_123", DataClassification::Confidential)
            .await;

        assert!(classification_result.is_ok());
    }

    #[tokio::test]
    async fn test_data_export_compliance() {
        // RED: Test data export compliance
        let governance = DataGovernance::new();

        // First classify some data
        governance
            .classify_data("sensitive_data", DataClassification::Confidential)
            .await
            .unwrap();

        // Create a test user
        let user = User {
            id: "admin_user".to_string(),
            username: "admin".to_string(),
            email: "admin@example.com".to_string(),
            display_name: "Admin User".to_string(),
            roles: {
                let mut roles = HashSet::new();
                roles.insert("admin".to_string());
                roles
            },
            permissions: HashSet::new(),
            groups: HashSet::new(),
            attributes: HashMap::new(),
            created_at: 1234567890,
            last_login: None,
            is_active: true,
        };

        // GREEN: Test export compliance check
        let compliance_result = governance
            .check_export_compliance("sensitive_data", "png", &user)
            .await;
        assert!(compliance_result.is_ok());
    }
}

/// Test suite for security configuration
mod security_config_tests {
    use super::*;

    #[tokio::test]
    async fn test_security_config_creation() {
        // RED: Test security configuration creation
        let oauth2_provider = OAuth2Provider::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "https://auth.example.com/authorize".to_string(),
            "https://auth.example.com/token".to_string(),
            "https://auth.example.com/userinfo".to_string(),
            "https://app.example.com/callback".to_string(),
        );

        let rbac_provider = RBACProvider::new();
        sleep(Duration::from_millis(100)).await; // Wait for RBAC initialization

        let _config = SecurityConfig::new(Box::new(oauth2_provider), Box::new(rbac_provider));

        // GREEN: Verify configuration is created
        // Note: We can't directly test internal state, but we can test operations
    }

    #[tokio::test]
    async fn test_security_config_authentication() {
        // RED: Test security configuration authentication
        let oauth2_provider = OAuth2Provider::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "https://auth.example.com/authorize".to_string(),
            "https://auth.example.com/token".to_string(),
            "https://auth.example.com/userinfo".to_string(),
            "https://app.example.com/callback".to_string(),
        );

        let rbac_provider = RBACProvider::new();
        sleep(Duration::from_millis(100)).await; // Wait for RBAC initialization

        let config = SecurityConfig::new(Box::new(oauth2_provider), Box::new(rbac_provider));

        let credentials = Credentials {
            credential_type: CredentialType::OAuth2,
            username: None,
            password: None,
            token: Some("test_auth_code".to_string()),
            additional_data: HashMap::new(),
        };

        // GREEN: Test authentication through config
        let result = config
            .authenticate_user(&credentials, Some("192.168.1.1".to_string()))
            .await;
        assert!(result.is_ok());

        let auth_result = result.unwrap();
        assert!(auth_result.success);
        assert!(auth_result.user.is_some());
    }

    #[tokio::test]
    async fn test_security_config_authorization() {
        // RED: Test security configuration authorization
        let oauth2_provider = OAuth2Provider::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "https://auth.example.com/authorize".to_string(),
            "https://auth.example.com/token".to_string(),
            "https://auth.example.com/userinfo".to_string(),
            "https://app.example.com/callback".to_string(),
        );

        let rbac_provider = RBACProvider::new();
        sleep(Duration::from_millis(100)).await; // Wait for RBAC initialization

        // Assign admin role to user in RBAC provider before moving it
        rbac_provider
            .assign_role_to_user("admin_user", "admin")
            .await
            .unwrap();

        let config = SecurityConfig::new(Box::new(oauth2_provider), Box::new(rbac_provider));

        let user = User {
            id: "admin_user".to_string(),
            username: "admin".to_string(),
            email: "admin@example.com".to_string(),
            display_name: "Admin User".to_string(),
            roles: {
                let mut roles = HashSet::new();
                roles.insert("admin".to_string());
                roles
            },
            permissions: HashSet::new(),
            groups: HashSet::new(),
            attributes: HashMap::new(),
            created_at: 1234567890,
            last_login: None,
            is_active: true,
        };

        let context = AuthorizationContext {
            resource: Resource::Chart {
                id: Some("chart_123".to_string()),
            },
            action: Action::Read,
            environment: HashMap::new(),
            ip_address: None,
            user_agent: None,
        };

        // GREEN: Test authorization through config
        let result = config.authorize_user(&user, &context).await;
        assert!(result.is_ok());

        let authorized = result.unwrap();
        assert!(authorized); // Admin should have access
    }

    #[tokio::test]
    async fn test_security_config_token_validation() {
        // RED: Test security configuration token validation
        let oauth2_provider = OAuth2Provider::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "https://auth.example.com/authorize".to_string(),
            "https://auth.example.com/token".to_string(),
            "https://auth.example.com/userinfo".to_string(),
            "https://app.example.com/callback".to_string(),
        );

        let rbac_provider = RBACProvider::new();
        sleep(Duration::from_millis(100)).await; // Wait for RBAC initialization

        let config = SecurityConfig::new(Box::new(oauth2_provider), Box::new(rbac_provider));

        // First authenticate to create a valid token
        let credentials = Credentials {
            credential_type: CredentialType::OAuth2,
            username: None,
            password: None,
            token: Some("test_auth_code".to_string()),
            additional_data: HashMap::new(),
        };

        let auth_result = config.authenticate_user(&credentials, None).await.unwrap();
        assert!(auth_result.success);
        let token = auth_result.access_token.unwrap();

        // GREEN: Test token validation through config
        let result = config.validate_token(&token).await;
        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.id, "oauth2_user_123");
        assert_eq!(user.username, "oauth2user");
    }
}

/// Integration tests for complete security workflow
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_complete_oauth2_workflow() {
        // RED: Test complete OAuth2 workflow
        let oauth2_provider = OAuth2Provider::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "https://auth.example.com/authorize".to_string(),
            "https://auth.example.com/token".to_string(),
            "https://auth.example.com/userinfo".to_string(),
            "https://app.example.com/callback".to_string(),
        );

        let rbac_provider = RBACProvider::new();
        sleep(Duration::from_millis(100)).await; // Wait for RBAC initialization

        let config = SecurityConfig::new(Box::new(oauth2_provider), Box::new(rbac_provider));

        // GREEN: Complete workflow
        // 1. Authenticate user
        let credentials = Credentials {
            credential_type: CredentialType::OAuth2,
            username: None,
            password: None,
            token: Some("test_auth_code".to_string()),
            additional_data: HashMap::new(),
        };

        let auth_result = config
            .authenticate_user(&credentials, Some("192.168.1.1".to_string()))
            .await
            .unwrap();
        assert!(auth_result.success);

        let user = auth_result.user.unwrap();
        let token = auth_result.access_token.unwrap();

        // 2. Validate token
        let validated_user = config.validate_token(&token).await.unwrap();
        assert_eq!(validated_user.id, user.id);

        // 3. Authorize action
        let context = AuthorizationContext {
            resource: Resource::Chart {
                id: Some("chart_123".to_string()),
            },
            action: Action::Read,
            environment: HashMap::new(),
            ip_address: None,
            user_agent: None,
        };

        let authorized = config.authorize_user(&user, &context).await.unwrap();
        assert!(authorized);

        // 4. Logout
        // Note: Logout is handled by the provider, not the config
    }

    #[tokio::test]
    async fn test_complete_saml_workflow() {
        // RED: Test complete SAML workflow
        let saml_provider = SAMLProvider::new(
            "test_entity_id".to_string(),
            "https://saml.example.com/sso".to_string(),
            "test_x509_cert".to_string(),
            "test_private_key".to_string(),
        );

        // Generate authn request before moving saml_provider
        let authn_request = saml_provider
            .generate_saml_request(Some("test_relay_state"))
            .unwrap();
        assert!(authn_request.contains("test_entity_id"));

        let rbac_provider = RBACProvider::new();
        sleep(Duration::from_millis(100)).await; // Wait for RBAC initialization

        let config = SecurityConfig::new(Box::new(saml_provider), Box::new(rbac_provider));

        // GREEN: Complete SAML workflow

        // 2. Authenticate with SAML response
        let credentials = Credentials {
            credential_type: CredentialType::SAML,
            username: None,
            password: None,
            token: Some("test_saml_response".to_string()),
            additional_data: HashMap::new(),
        };

        let auth_result = config
            .authenticate_user(&credentials, Some("192.168.1.1".to_string()))
            .await
            .unwrap();
        assert!(auth_result.success);

        let user = auth_result.user.unwrap();
        assert_eq!(user.id, "saml_user_456");
    }

    #[tokio::test]
    async fn test_security_audit_trail() {
        // RED: Test complete security audit trail
        let oauth2_provider = OAuth2Provider::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "https://auth.example.com/authorize".to_string(),
            "https://auth.example.com/token".to_string(),
            "https://auth.example.com/userinfo".to_string(),
            "https://app.example.com/callback".to_string(),
        );

        let rbac_provider = RBACProvider::new();
        sleep(Duration::from_millis(100)).await; // Wait for RBAC initialization

        let config = SecurityConfig::new(Box::new(oauth2_provider), Box::new(rbac_provider));

        // GREEN: Test audit trail
        // 1. Authenticate (should log authentication)
        let credentials = Credentials {
            credential_type: CredentialType::OAuth2,
            username: None,
            password: None,
            token: Some("test_auth_code".to_string()),
            additional_data: HashMap::new(),
        };

        let auth_result = config
            .authenticate_user(&credentials, Some("192.168.1.1".to_string()))
            .await
            .unwrap();
        let user = auth_result.user.unwrap();

        // 2. Authorize (should log authorization)
        let context = AuthorizationContext {
            resource: Resource::Chart {
                id: Some("chart_123".to_string()),
            },
            action: Action::Read,
            environment: HashMap::new(),
            ip_address: None,
            user_agent: None,
        };

        let _authorized = config.authorize_user(&user, &context).await.unwrap();

        // 3. Retrieve audit logs
        let audit_logs = config.audit_logger.get_audit_logs(Some(10)).await.unwrap();
        assert!(audit_logs.len() >= 2); // Should have at least authentication and authorization logs

        // Verify audit log content
        let auth_logs: Vec<_> = audit_logs
            .iter()
            .filter(|log| matches!(log.event_type, AuditEventType::Authentication))
            .collect();
        assert!(!auth_logs.is_empty());

        let authz_logs: Vec<_> = audit_logs
            .iter()
            .filter(|log| matches!(log.event_type, AuditEventType::Authorization))
            .collect();
        assert!(!authz_logs.is_empty());
    }
}
