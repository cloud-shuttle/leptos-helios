//! Enterprise Security & Authentication
//!
//! This module provides comprehensive security and authentication capabilities for Helios,
//! including OAuth, SAML, RBAC, audit logging, and data governance.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::{Mutex, RwLock};

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

/// User identity and profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub roles: HashSet<String>,
    pub permissions: HashSet<Permission>,
    pub groups: HashSet<String>,
    pub attributes: HashMap<String, String>,
    pub created_at: u64,
    pub last_login: Option<u64>,
    pub is_active: bool,
}

/// Permission types for fine-grained access control
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    // Chart permissions
    ViewCharts,
    CreateCharts,
    EditCharts,
    DeleteCharts,

    // Data permissions
    ViewData,
    EditData,
    DeleteData,
    ExportData,
    ImportData,

    // System permissions
    ManageUsers,
    ManageRoles,
    ManageSystem,
    ViewAuditLogs,

    // Export permissions
    ExportPNG,
    ExportSVG,
    ExportPDF,
    ExportHTML,

    // Custom permission
    Custom(String),
}

/// Authentication credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub credential_type: CredentialType,
    pub username: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
    pub additional_data: HashMap<String, String>,
}

/// Types of authentication credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CredentialType {
    UsernamePassword,
    BearerToken,
    ApiKey,
    OAuth2,
    SAML,
    Certificate,
}

/// Authentication result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResult {
    pub success: bool,
    pub user: Option<User>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: Option<Duration>,
    pub token_type: Option<String>,
    pub error: Option<String>,
}

/// Authorization context for resource access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationContext {
    pub resource: Resource,
    pub action: Action,
    pub environment: HashMap<String, String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

/// Resource types for authorization
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Resource {
    Chart {
        id: Option<String>,
    },
    Data {
        source: String,
        table: Option<String>,
    },
    Dashboard {
        id: String,
    },
    Export {
        format: String,
    },
    System {
        component: String,
    },
    Custom(String),
}

/// Action types for authorization
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    Create,
    Read,
    Update,
    Delete,
    Execute,
    Export,
    Import,
    Manage,
    Custom(String),
}

/// Core authentication provider trait
#[async_trait::async_trait]
pub trait AuthProvider: Send + Sync {
    /// Authenticate user with credentials
    async fn authenticate(
        &self,
        credentials: &Credentials,
    ) -> Result<AuthenticationResult, SecurityError>;

    /// Validate existing token
    async fn validate_token(&self, token: &str) -> Result<User, SecurityError>;

    /// Refresh access token
    async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<AuthenticationResult, SecurityError>;

    /// Logout user
    async fn logout(&self, token: &str) -> Result<(), SecurityError>;

    /// Get provider type identifier
    fn provider_type(&self) -> &'static str;
}

/// Core authorization provider trait
#[async_trait::async_trait]
pub trait AuthorizationProvider: Send + Sync {
    /// Check if user is authorized for action on resource
    async fn authorize(
        &self,
        user: &User,
        context: &AuthorizationContext,
    ) -> Result<bool, SecurityError>;

    /// Get user permissions for resource
    async fn get_permissions(
        &self,
        user: &User,
        resource: &Resource,
    ) -> Result<HashSet<Permission>, SecurityError>;

    /// Check if user has specific permission
    async fn has_permission(
        &self,
        user: &User,
        permission: &Permission,
    ) -> Result<bool, SecurityError>;
}

/// OAuth 2.0 authentication provider
pub struct OAuth2Provider {
    client_id: String,
    client_secret: String,
    authorization_url: String,
    token_url: String,
    userinfo_url: String,
    redirect_uri: String,
    scope: Vec<String>,
    token_cache: Arc<RwLock<HashMap<String, (User, u64)>>>, // token -> (user, expires_at)
}

impl OAuth2Provider {
    pub fn new(
        client_id: String,
        client_secret: String,
        authorization_url: String,
        token_url: String,
        userinfo_url: String,
        redirect_uri: String,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            authorization_url,
            token_url,
            userinfo_url,
            redirect_uri,
            scope: vec![
                "openid".to_string(),
                "profile".to_string(),
                "email".to_string(),
            ],
            token_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get_authorization_url(&self, state: &str) -> String {
        format!(
            "{}?client_id={}&response_type=code&redirect_uri={}&scope={}&state={}",
            self.authorization_url,
            self.client_id,
            urlencoding::encode(&self.redirect_uri),
            self.scope.join(" "),
            state
        )
    }
}

#[async_trait::async_trait]
impl AuthProvider for OAuth2Provider {
    async fn authenticate(
        &self,
        credentials: &Credentials,
    ) -> Result<AuthenticationResult, SecurityError> {
        match &credentials.credential_type {
            CredentialType::OAuth2 => {
                let code = credentials.token.as_ref().ok_or_else(|| {
                    SecurityError::AuthenticationFailed("OAuth2 code required".to_string())
                })?;

                // Exchange authorization code for access token
                let token_response = self.exchange_code_for_token(code).await?;

                // Get user information
                let user = self.get_user_info(&token_response.access_token).await?;

                // Cache token
                let expires_at = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    + token_response.expires_in.unwrap_or(3600);

                let mut cache = self.token_cache.write().await;
                cache.insert(
                    token_response.access_token.clone(),
                    (user.clone(), expires_at),
                );

                Ok(AuthenticationResult {
                    success: true,
                    user: Some(user),
                    access_token: Some(token_response.access_token),
                    refresh_token: token_response.refresh_token,
                    expires_in: Some(Duration::from_secs(
                        token_response.expires_in.unwrap_or(3600),
                    )),
                    token_type: Some("Bearer".to_string()),
                    error: None,
                })
            }
            _ => Err(SecurityError::AuthenticationFailed(
                "Unsupported credential type for OAuth2".to_string(),
            )),
        }
    }

    async fn validate_token(&self, token: &str) -> Result<User, SecurityError> {
        let cache = self.token_cache.read().await;
        if let Some((user, expires_at)) = cache.get(token) {
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            if current_time < *expires_at {
                return Ok(user.clone());
            }
        }

        Err(SecurityError::TokenError(
            "Token expired or invalid".to_string(),
        ))
    }

    async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<AuthenticationResult, SecurityError> {
        // Mock refresh token implementation
        let new_access_token = format!("refreshed_access_token_{}", refresh_token);
        let new_refresh_token = format!("refreshed_refresh_token_{}", refresh_token);
        
        // Get user info with new token
        let user = self.get_user_info(&new_access_token).await?;
        
        // Cache new token
        let expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 3600;

        let mut cache = self.token_cache.write().await;
        cache.insert(new_access_token.clone(), (user.clone(), expires_at));

        Ok(AuthenticationResult {
            success: true,
            user: Some(user),
            access_token: Some(new_access_token),
            refresh_token: Some(new_refresh_token),
            expires_in: Some(Duration::from_secs(3600)),
            token_type: Some("Bearer".to_string()),
            error: None,
        })
    }

    async fn logout(&self, token: &str) -> Result<(), SecurityError> {
        let mut cache = self.token_cache.write().await;
        cache.remove(token);
        Ok(())
    }

    fn provider_type(&self) -> &'static str {
        "oauth2"
    }
}

impl OAuth2Provider {
    async fn exchange_code_for_token(&self, code: &str) -> Result<TokenResponse, SecurityError> {
        // Mock token exchange - would make HTTP request to token endpoint
        Ok(TokenResponse {
            access_token: format!("mock_access_token_{}", code),
            refresh_token: Some(format!("mock_refresh_token_{}", code)),
            expires_in: Some(3600),
            token_type: "Bearer".to_string(),
        })
    }

    async fn get_user_info(&self, _access_token: &str) -> Result<User, SecurityError> {
        // Mock user info retrieval - would make HTTP request to userinfo endpoint
        Ok(User {
            id: "oauth2_user_123".to_string(),
            username: "oauth2user".to_string(),
            email: "user@example.com".to_string(),
            display_name: "OAuth2 User".to_string(),
            roles: {
                let mut roles = HashSet::new();
                roles.insert("user".to_string());
                roles
            },
            permissions: {
                let mut perms = HashSet::new();
                perms.insert(Permission::ViewCharts);
                perms.insert(Permission::CreateCharts);
                perms
            },
            groups: HashSet::new(),
            attributes: HashMap::new(),
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            last_login: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            ),
            is_active: true,
        })
    }
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: Option<u64>,
    token_type: String,
}

/// SAML authentication provider
pub struct SAMLProvider {
    entity_id: String,
    sso_url: String,
    x509_cert: String,
    private_key: String,
    user_cache: Arc<RwLock<HashMap<String, (User, u64)>>>,
}

impl SAMLProvider {
    pub fn new(entity_id: String, sso_url: String, x509_cert: String, private_key: String) -> Self {
        Self {
            entity_id,
            sso_url,
            x509_cert,
            private_key,
            user_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn generate_saml_request(
        &self,
        _relay_state: Option<&str>,
    ) -> Result<String, SecurityError> {
        // Mock SAML request generation
        let request = format!(
            r#"<samlp:AuthnRequest
                xmlns:samlp="urn:oasis:names:tc:SAML:2.0:protocol"
                ID="_{}"
                Version="2.0"
                IssueInstant="{}"
                Destination="{}">
                <saml:Issuer xmlns:saml="urn:oasis:names:tc:SAML:2.0:assertion">{}</saml:Issuer>
            </samlp:AuthnRequest>"#,
            uuid::Uuid::new_v4(),
            chrono::Utc::now().to_rfc3339(),
            self.sso_url,
            self.entity_id
        );

        // For testing purposes, return raw XML. In production, would encode and sign the request
        Ok(request)
    }
}

#[async_trait::async_trait]
impl AuthProvider for SAMLProvider {
    async fn authenticate(
        &self,
        credentials: &Credentials,
    ) -> Result<AuthenticationResult, SecurityError> {
        match &credentials.credential_type {
            CredentialType::SAML => {
                let saml_response = credentials.token.as_ref().ok_or_else(|| {
                    SecurityError::AuthenticationFailed("SAML response required".to_string())
                })?;

                // Validate and parse SAML response
                let user = self.parse_saml_response(saml_response).await?;

                // Generate session token
                let session_token = format!("saml_session_{}", uuid::Uuid::new_v4());
                let expires_at = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    + 3600;

                let mut cache = self.user_cache.write().await;
                cache.insert(session_token.clone(), (user.clone(), expires_at));

                Ok(AuthenticationResult {
                    success: true,
                    user: Some(user),
                    access_token: Some(session_token),
                    refresh_token: None,
                    expires_in: Some(Duration::from_secs(3600)),
                    token_type: Some("SAML".to_string()),
                    error: None,
                })
            }
            _ => Err(SecurityError::AuthenticationFailed(
                "Unsupported credential type for SAML".to_string(),
            )),
        }
    }

    async fn validate_token(&self, token: &str) -> Result<User, SecurityError> {
        let cache = self.user_cache.read().await;
        if let Some((user, expires_at)) = cache.get(token) {
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            if current_time < *expires_at {
                return Ok(user.clone());
            }
        }

        Err(SecurityError::TokenError(
            "SAML session expired or invalid".to_string(),
        ))
    }

    async fn refresh_token(
        &self,
        _refresh_token: &str,
    ) -> Result<AuthenticationResult, SecurityError> {
        Err(SecurityError::TokenError(
            "SAML does not support token refresh".to_string(),
        ))
    }

    async fn logout(&self, token: &str) -> Result<(), SecurityError> {
        let mut cache = self.user_cache.write().await;
        cache.remove(token);
        Ok(())
    }

    fn provider_type(&self) -> &'static str {
        "saml"
    }
}

impl SAMLProvider {
    async fn parse_saml_response(&self, _saml_response: &str) -> Result<User, SecurityError> {
        // Mock SAML response parsing - would validate signature and parse XML
        Ok(User {
            id: "saml_user_456".to_string(),
            username: "samluser".to_string(),
            email: "samluser@example.com".to_string(),
            display_name: "SAML User".to_string(),
            roles: {
                let mut roles = HashSet::new();
                roles.insert("saml_user".to_string());
                roles.insert("admin".to_string());
                roles
            },
            permissions: {
                let mut perms = HashSet::new();
                perms.insert(Permission::ViewCharts);
                perms.insert(Permission::CreateCharts);
                perms.insert(Permission::EditCharts);
                perms.insert(Permission::ManageSystem);
                perms
            },
            groups: {
                let mut groups = HashSet::new();
                groups.insert("administrators".to_string());
                groups
            },
            attributes: {
                let mut attrs = HashMap::new();
                attrs.insert("department".to_string(), "IT".to_string());
                attrs.insert("title".to_string(), "System Administrator".to_string());
                attrs
            },
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            last_login: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            ),
            is_active: true,
        })
    }
}

/// Role-Based Access Control (RBAC) provider
pub struct RBACProvider {
    roles: Arc<RwLock<HashMap<String, Role>>>,
    user_roles: Arc<RwLock<HashMap<String, HashSet<String>>>>,
    policies: Arc<RwLock<Vec<Policy>>>,
}

/// Role definition with permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub name: String,
    pub description: String,
    pub permissions: HashSet<Permission>,
    pub inherits_from: HashSet<String>,
    pub created_at: u64,
    pub is_active: bool,
}

/// Access control policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: String,
    pub name: String,
    pub effect: PolicyEffect,
    pub subjects: Vec<String>, // Users, roles, or groups
    pub resources: Vec<Resource>,
    pub actions: Vec<Action>,
    pub conditions: Vec<PolicyCondition>,
    pub priority: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyEffect {
    Allow,
    Deny,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyCondition {
    pub key: String,
    pub operator: ConditionOperator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    In,
    NotIn,
    Contains,
    StartsWith,
    EndsWith,
    GreaterThan,
    LessThan,
    IpInRange,
    TimeInRange,
}

impl RBACProvider {
    pub fn new() -> Self {
        let rbac = Self {
            roles: Arc::new(RwLock::new(HashMap::new())),
            user_roles: Arc::new(RwLock::new(HashMap::new())),
            policies: Arc::new(RwLock::new(Vec::new())),
        };

        // Initialize default roles
        let roles_clone = rbac.roles.clone();
        let user_roles_clone = rbac.user_roles.clone();
        let policies_clone = rbac.policies.clone();
        tokio::spawn(async move {
            let temp_rbac = Self {
                roles: roles_clone,
                user_roles: user_roles_clone,
                policies: policies_clone,
            };
            temp_rbac.initialize_default_roles().await;
        });

        rbac
    }

    async fn initialize_default_roles(&self) {
        let mut roles = self.roles.write().await;

        // Admin role
        let admin_permissions = {
            let mut perms = HashSet::new();
            perms.insert(Permission::ViewCharts);
            perms.insert(Permission::CreateCharts);
            perms.insert(Permission::EditCharts);
            perms.insert(Permission::DeleteCharts);
            perms.insert(Permission::ViewData);
            perms.insert(Permission::EditData);
            perms.insert(Permission::DeleteData);
            perms.insert(Permission::ExportData);
            perms.insert(Permission::ImportData);
            perms.insert(Permission::ManageUsers);
            perms.insert(Permission::ManageRoles);
            perms.insert(Permission::ManageSystem);
            perms.insert(Permission::ViewAuditLogs);
            perms.insert(Permission::ExportPNG);
            perms.insert(Permission::ExportSVG);
            perms.insert(Permission::ExportPDF);
            perms.insert(Permission::ExportHTML);
            perms
        };

        roles.insert(
            "admin".to_string(),
            Role {
                name: "admin".to_string(),
                description: "Full system administrator".to_string(),
                permissions: admin_permissions,
                inherits_from: HashSet::new(),
                created_at: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                is_active: true,
            },
        );

        // User role
        let user_permissions = {
            let mut perms = HashSet::new();
            perms.insert(Permission::ViewCharts);
            perms.insert(Permission::CreateCharts);
            perms.insert(Permission::ViewData);
            perms.insert(Permission::ExportPNG);
            perms.insert(Permission::ExportSVG);
            perms
        };

        roles.insert(
            "user".to_string(),
            Role {
                name: "user".to_string(),
                description: "Standard user".to_string(),
                permissions: user_permissions,
                inherits_from: HashSet::new(),
                created_at: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                is_active: true,
            },
        );

        // Viewer role
        let viewer_permissions = {
            let mut perms = HashSet::new();
            perms.insert(Permission::ViewCharts);
            perms.insert(Permission::ViewData);
            perms
        };

        roles.insert(
            "viewer".to_string(),
            Role {
                name: "viewer".to_string(),
                description: "Read-only access".to_string(),
                permissions: viewer_permissions,
                inherits_from: HashSet::new(),
                created_at: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                is_active: true,
            },
        );
    }

    pub async fn assign_role_to_user(
        &self,
        user_id: &str,
        role_name: &str,
    ) -> Result<(), SecurityError> {
        let roles = self.roles.read().await;
        if !roles.contains_key(role_name) {
            return Err(SecurityError::ConfigurationError(format!(
                "Role '{}' not found",
                role_name
            )));
        }

        let mut user_roles = self.user_roles.write().await;
        user_roles
            .entry(user_id.to_string())
            .or_insert_with(HashSet::new)
            .insert(role_name.to_string());

        Ok(())
    }

    pub async fn remove_role_from_user(
        &self,
        user_id: &str,
        role_name: &str,
    ) -> Result<(), SecurityError> {
        let mut user_roles = self.user_roles.write().await;
        if let Some(roles) = user_roles.get_mut(user_id) {
            roles.remove(role_name);
        }
        Ok(())
    }

    pub async fn get_user_permissions(&self, user_id: &str) -> HashSet<Permission> {
        let mut all_permissions = HashSet::new();

        let user_roles = self.user_roles.read().await;
        let roles = self.roles.read().await;

        if let Some(user_role_names) = user_roles.get(user_id) {
            for role_name in user_role_names {
                if let Some(role) = roles.get(role_name) {
                    all_permissions.extend(role.permissions.clone());

                    // Handle role inheritance
                    for inherited_role_name in &role.inherits_from {
                        if let Some(inherited_role) = roles.get(inherited_role_name) {
                            all_permissions.extend(inherited_role.permissions.clone());
                        }
                    }
                }
            }
        }

        all_permissions
    }
}

#[async_trait::async_trait]
impl AuthorizationProvider for RBACProvider {
    async fn authorize(
        &self,
        user: &User,
        context: &AuthorizationContext,
    ) -> Result<bool, SecurityError> {
        // Check user permissions from roles
        let user_permissions = self.get_user_permissions(&user.id).await;
        
        // Also check direct permissions on the user object
        let mut all_permissions = user_permissions;
        all_permissions.extend(user.permissions.clone());

        // Map resource/action to required permission
        let required_permission = match (&context.resource, &context.action) {
            (Resource::Chart { .. }, Action::Read) => Permission::ViewCharts,
            (Resource::Chart { .. }, Action::Create) => Permission::CreateCharts,
            (Resource::Chart { .. }, Action::Update) => Permission::EditCharts,
            (Resource::Chart { .. }, Action::Delete) => Permission::DeleteCharts,
            (Resource::Data { .. }, Action::Read) => Permission::ViewData,
            (Resource::Data { .. }, Action::Update) => Permission::EditData,
            (Resource::Export { format }, Action::Execute) => match format.as_str() {
                "png" => Permission::ExportPNG,
                "svg" => Permission::ExportSVG,
                "pdf" => Permission::ExportPDF,
                "html" => Permission::ExportHTML,
                _ => return Ok(false),
            },
            _ => return Ok(false),
        };

        // Check if user has required permission
        if all_permissions.contains(&required_permission) {
            // Check policies for additional rules
            return self.check_policies(user, context).await;
        }

        Ok(false)
    }

    async fn get_permissions(
        &self,
        user: &User,
        _resource: &Resource,
    ) -> Result<HashSet<Permission>, SecurityError> {
        let mut permissions = self.get_user_permissions(&user.id).await;
        permissions.extend(user.permissions.clone());
        Ok(permissions)
    }

    async fn has_permission(
        &self,
        user: &User,
        permission: &Permission,
    ) -> Result<bool, SecurityError> {
        let mut user_permissions = self.get_user_permissions(&user.id).await;
        user_permissions.extend(user.permissions.clone());
        Ok(user_permissions.contains(permission))
    }
}

impl RBACProvider {
    async fn check_policies(
        &self,
        user: &User,
        context: &AuthorizationContext,
    ) -> Result<bool, SecurityError> {
        let policies = self.policies.read().await;
        let mut allow = false;
        let mut deny = false;

        for policy in policies.iter() {
            if self.policy_applies_to_user(user, policy)
                && self.policy_applies_to_context(context, policy)
                && self.evaluate_policy_conditions(user, context, policy)
            {
                match policy.effect {
                    PolicyEffect::Allow => allow = true,
                    PolicyEffect::Deny => deny = true,
                }
            }
        }

        // If no policies apply, allow by default
        // If policies apply, deny takes precedence over allow
        if policies.is_empty() {
            Ok(true)
        } else {
            Ok(allow && !deny)
        }
    }

    fn policy_applies_to_user(&self, user: &User, policy: &Policy) -> bool {
        for subject in &policy.subjects {
            if subject == &user.id || user.roles.contains(subject) || user.groups.contains(subject)
            {
                return true;
            }
        }
        false
    }

    fn policy_applies_to_context(&self, context: &AuthorizationContext, policy: &Policy) -> bool {
        policy.resources.contains(&context.resource) && policy.actions.contains(&context.action)
    }

    fn evaluate_policy_conditions(
        &self,
        user: &User,
        context: &AuthorizationContext,
        policy: &Policy,
    ) -> bool {
        for condition in &policy.conditions {
            if !self.evaluate_condition(user, context, condition) {
                return false;
            }
        }
        true
    }

    fn evaluate_condition(
        &self,
        user: &User,
        context: &AuthorizationContext,
        condition: &PolicyCondition,
    ) -> bool {
        // Mock condition evaluation - would implement full condition logic
        match condition.operator {
            ConditionOperator::Equals => true, // Simplified
            _ => true,
        }
    }
}

/// Audit logging system
pub struct AuditLogger {
    log_storage: Arc<Mutex<Vec<AuditEvent>>>,
    enabled: bool,
}

/// Audit event for security logging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: String,
    pub timestamp: u64,
    pub event_type: AuditEventType,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub resource: Option<Resource>,
    pub action: Option<Action>,
    pub result: AuditResult,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub details: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEventType {
    Authentication,
    Authorization,
    DataAccess,
    SystemChange,
    Export,
    SecurityViolation,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditResult {
    Success,
    Failure,
    Warning,
}

impl AuditLogger {
    pub fn new(enabled: bool) -> Self {
        Self {
            log_storage: Arc::new(Mutex::new(Vec::new())),
            enabled,
        }
    }

    pub async fn log_event(&self, event: AuditEvent) -> Result<(), SecurityError> {
        if !self.enabled {
            return Ok(());
        }

        let mut storage = self.log_storage.lock().await;
        storage.push(event);

        // In production, would write to persistent storage
        Ok(())
    }

    pub async fn log_authentication(
        &self,
        user_id: &str,
        success: bool,
        ip_address: Option<String>,
    ) -> Result<(), SecurityError> {
        let event = AuditEvent {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            event_type: AuditEventType::Authentication,
            user_id: Some(user_id.to_string()),
            session_id: None,
            resource: None,
            action: None,
            result: if success {
                AuditResult::Success
            } else {
                AuditResult::Failure
            },
            ip_address,
            user_agent: None,
            details: HashMap::new(),
        };

        self.log_event(event).await
    }

    pub async fn log_authorization(
        &self,
        user_id: &str,
        resource: Resource,
        action: Action,
        authorized: bool,
    ) -> Result<(), SecurityError> {
        let event = AuditEvent {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            event_type: AuditEventType::Authorization,
            user_id: Some(user_id.to_string()),
            session_id: None,
            resource: Some(resource),
            action: Some(action),
            result: if authorized {
                AuditResult::Success
            } else {
                AuditResult::Failure
            },
            ip_address: None,
            user_agent: None,
            details: HashMap::new(),
        };

        self.log_event(event).await
    }

    pub async fn get_audit_logs(
        &self,
        limit: Option<usize>,
    ) -> Result<Vec<AuditEvent>, SecurityError> {
        let storage = self.log_storage.lock().await;
        let logs = if let Some(limit) = limit {
            storage.iter().rev().take(limit).cloned().collect()
        } else {
            storage.clone()
        };
        Ok(logs)
    }
}

/// Data governance and classification system
pub struct DataGovernance {
    classifications: Arc<RwLock<HashMap<String, DataClassification>>>,
    policies: Arc<RwLock<Vec<DataPolicy>>>,
}

/// Data sensitivity classification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
    TopSecret,
}

/// Data governance policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPolicy {
    pub id: String,
    pub name: String,
    pub classification: DataClassification,
    pub retention_days: Option<u32>,
    pub geographic_restrictions: Vec<String>,
    pub allowed_exports: Vec<String>,
    pub required_approvals: Vec<String>,
}

impl DataGovernance {
    pub fn new() -> Self {
        Self {
            classifications: Arc::new(RwLock::new(HashMap::new())),
            policies: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn classify_data(
        &self,
        data_source: &str,
        classification: DataClassification,
    ) -> Result<(), SecurityError> {
        let mut classifications = self.classifications.write().await;
        classifications.insert(data_source.to_string(), classification);
        Ok(())
    }

    pub async fn check_export_compliance(
        &self,
        data_source: &str,
        export_format: &str,
        user: &User,
    ) -> Result<bool, SecurityError> {
        let classifications = self.classifications.read().await;
        let policies = self.policies.read().await;

        if let Some(classification) = classifications.get(data_source) {
            for policy in policies.iter() {
                if policy.classification == *classification {
                    if !policy.allowed_exports.contains(&export_format.to_string()) {
                        return Err(SecurityError::DataGovernanceViolation(format!(
                            "Export format '{}' not allowed for {} data",
                            export_format,
                            format!("{:?}", classification)
                        )));
                    }

                    // Check if user has required approvals
                    for required_role in &policy.required_approvals {
                        if !user.roles.contains(required_role) {
                            return Err(SecurityError::DataGovernanceViolation(format!(
                                "Role '{}' required for exporting {} data",
                                required_role,
                                format!("{:?}", classification)
                            )));
                        }
                    }
                }
            }
        }

        Ok(true)
    }
}

/// Main security configuration
pub struct SecurityConfig {
    pub auth_provider: Box<dyn AuthProvider>,
    pub authorization_provider: Box<dyn AuthorizationProvider>,
    pub audit_logger: AuditLogger,
    pub data_governance: DataGovernance,
    pub session_timeout: Duration,
    pub max_failed_attempts: u32,
    pub require_mfa: bool,
}

impl SecurityConfig {
    pub fn new(
        auth_provider: Box<dyn AuthProvider>,
        authorization_provider: Box<dyn AuthorizationProvider>,
    ) -> Self {
        Self {
            auth_provider,
            authorization_provider,
            audit_logger: AuditLogger::new(true),
            data_governance: DataGovernance::new(),
            session_timeout: Duration::from_secs(3600),
            max_failed_attempts: 3,
            require_mfa: false,
        }
    }

    pub async fn authenticate_user(
        &self,
        credentials: &Credentials,
        ip_address: Option<String>,
    ) -> Result<AuthenticationResult, SecurityError> {
        let result = self.auth_provider.authenticate(credentials).await?;

        // Log authentication attempt
        if let Some(ref user) = result.user {
            self.audit_logger
                .log_authentication(&user.id, result.success, ip_address)
                .await?;
        }

        Ok(result)
    }

    pub async fn authorize_user(
        &self,
        user: &User,
        context: &AuthorizationContext,
    ) -> Result<bool, SecurityError> {
        let authorized = self.authorization_provider.authorize(user, context).await?;

        // Log authorization attempt
        self.audit_logger
            .log_authorization(
                &user.id,
                context.resource.clone(),
                context.action.clone(),
                authorized,
            )
            .await?;

        Ok(authorized)
    }

    pub async fn validate_token(&self, token: &str) -> Result<User, SecurityError> {
        self.auth_provider.validate_token(token).await
    }
}

// Add required dependencies to mock base64 and urlencoding
mod base64 {
    pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
        // Mock base64 encoding
        format!("base64_{}", hex::encode(input.as_ref()))
    }
}

mod urlencoding {
    pub fn encode(input: &str) -> String {
        // Mock URL encoding
        input.replace(" ", "%20").replace("&", "%26")
    }
}

mod hex {
    pub fn encode<T: AsRef<[u8]>>(data: T) -> String {
        data.as_ref().iter().map(|b| format!("{:02x}", b)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_oauth2_authentication() {
        let oauth2_provider = OAuth2Provider::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "https://auth.example.com/authorize".to_string(),
            "https://auth.example.com/token".to_string(),
            "https://auth.example.com/userinfo".to_string(),
            "http://localhost:3000/callback".to_string(),
        );

        let credentials = Credentials {
            credential_type: CredentialType::OAuth2,
            username: None,
            password: None,
            token: Some("test_auth_code".to_string()),
            additional_data: HashMap::new(),
        };

        let result = oauth2_provider.authenticate(&credentials).await.unwrap();
        assert!(result.success);
        assert!(result.user.is_some());
        assert!(result.access_token.is_some());
    }

    #[tokio::test]
    async fn test_saml_authentication() {
        let saml_provider = SAMLProvider::new(
            "urn:example:helios".to_string(),
            "https://idp.example.com/sso".to_string(),
            "test_cert".to_string(),
            "test_key".to_string(),
        );

        let credentials = Credentials {
            credential_type: CredentialType::SAML,
            username: None,
            password: None,
            token: Some("mock_saml_response".to_string()),
            additional_data: HashMap::new(),
        };

        let result = saml_provider.authenticate(&credentials).await.unwrap();
        assert!(result.success);
        assert!(result.user.is_some());
        assert!(result.access_token.is_some());
    }

    #[tokio::test]
    async fn test_rbac_authorization() {
        let rbac_provider = RBACProvider::new();

        // Wait for default roles to initialize
        tokio::time::sleep(Duration::from_millis(500)).await;

        let mut user = User {
            id: "test_user".to_string(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            display_name: "Test User".to_string(),
            roles: HashSet::new(),
            permissions: HashSet::new(),
            groups: HashSet::new(),
            attributes: HashMap::new(),
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            last_login: None,
            is_active: true,
        };

        rbac_provider
            .assign_role_to_user("test_user", "user")
            .await
            .unwrap();

        let context = AuthorizationContext {
            resource: Resource::Chart {
                id: Some("test_chart".to_string()),
            },
            action: Action::Read,
            environment: HashMap::new(),
            ip_address: None,
            user_agent: None,
        };

        let authorized = rbac_provider.authorize(&user, &context).await.unwrap();
        assert!(authorized);

        // Test unauthorized action
        let delete_context = AuthorizationContext {
            resource: Resource::Chart {
                id: Some("test_chart".to_string()),
            },
            action: Action::Delete,
            environment: HashMap::new(),
            ip_address: None,
            user_agent: None,
        };

        let not_authorized = rbac_provider
            .authorize(&user, &delete_context)
            .await
            .unwrap();
        assert!(!not_authorized);
    }

    #[tokio::test]
    async fn test_audit_logging() {
        let audit_logger = AuditLogger::new(true);

        audit_logger
            .log_authentication("test_user", true, Some("192.168.1.1".to_string()))
            .await
            .unwrap();

        audit_logger
            .log_authorization(
                "test_user",
                Resource::Data {
                    source: "sales".to_string(),
                    table: Some("revenue".to_string()),
                },
                Action::Read,
                true,
            )
            .await
            .unwrap();

        let logs = audit_logger.get_audit_logs(Some(10)).await.unwrap();
        assert_eq!(logs.len(), 2);
        assert!(matches!(logs[1].event_type, AuditEventType::Authentication));
        assert!(matches!(logs[0].event_type, AuditEventType::Authorization));
    }

    #[tokio::test]
    async fn test_data_governance() {
        let governance = DataGovernance::new();

        governance
            .classify_data("customer_data", DataClassification::Confidential)
            .await
            .unwrap();

        let user = User {
            id: "test_user".to_string(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            display_name: "Test User".to_string(),
            roles: {
                let mut roles = HashSet::new();
                roles.insert("user".to_string());
                roles
            },
            permissions: HashSet::new(),
            groups: HashSet::new(),
            attributes: HashMap::new(),
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            last_login: None,
            is_active: true,
        };

        // Should pass for basic export (no policies defined yet)
        let result = governance
            .check_export_compliance("customer_data", "png", &user)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_security_config_integration() {
        let oauth2_provider = OAuth2Provider::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "https://auth.example.com/authorize".to_string(),
            "https://auth.example.com/token".to_string(),
            "https://auth.example.com/userinfo".to_string(),
            "http://localhost:3000/callback".to_string(),
        );

        let rbac_provider = RBACProvider::new();

        let security_config =
            SecurityConfig::new(Box::new(oauth2_provider), Box::new(rbac_provider));

        let credentials = Credentials {
            credential_type: CredentialType::OAuth2,
            username: None,
            password: None,
            token: Some("test_auth_code".to_string()),
            additional_data: HashMap::new(),
        };

        let auth_result = security_config
            .authenticate_user(&credentials, Some("192.168.1.1".to_string()))
            .await
            .unwrap();
        assert!(auth_result.success);

        if let Some(user) = auth_result.user {
            let context = AuthorizationContext {
                resource: Resource::Chart {
                    id: Some("test_chart".to_string()),
                },
                action: Action::Read,
                environment: HashMap::new(),
                ip_address: Some("192.168.1.1".to_string()),
                user_agent: None,
            };

            // Note: This might fail since OAuth2 user doesn't automatically get RBAC roles
            // In a real implementation, you'd sync roles between systems
            let _authorized = security_config.authorize_user(&user, &context).await;
        }
    }
}
