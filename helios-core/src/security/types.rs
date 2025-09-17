//! Core security types and data structures
//!
//! This module provides the fundamental types used throughout the security system,
//! including users, permissions, credentials, and authorization contexts.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

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

impl User {
    /// Create a new user
    pub fn new(id: String, username: String, email: String, display_name: String) -> Self {
        Self {
            id,
            username,
            email,
            display_name,
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
        }
    }

    /// Add a role to the user
    pub fn add_role(&mut self, role: String) {
        self.roles.insert(role);
    }

    /// Remove a role from the user
    pub fn remove_role(&mut self, role: &str) {
        self.roles.remove(role);
    }

    /// Check if the user has a specific role
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.contains(role)
    }

    /// Add a permission to the user
    pub fn add_permission(&mut self, permission: Permission) {
        self.permissions.insert(permission);
    }

    /// Remove a permission from the user
    pub fn remove_permission(&mut self, permission: &Permission) {
        self.permissions.remove(permission);
    }

    /// Check if the user has a specific permission
    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions.contains(permission)
    }

    /// Add a group to the user
    pub fn add_group(&mut self, group: String) {
        self.groups.insert(group);
    }

    /// Remove a group from the user
    pub fn remove_group(&mut self, group: &str) {
        self.groups.remove(group);
    }

    /// Check if the user belongs to a specific group
    pub fn in_group(&self, group: &str) -> bool {
        self.groups.contains(group)
    }

    /// Set a user attribute
    pub fn set_attribute(&mut self, key: String, value: String) {
        self.attributes.insert(key, value);
    }

    /// Get a user attribute
    pub fn get_attribute(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
    }

    /// Update last login time
    pub fn update_last_login(&mut self) {
        self.last_login = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
    }

    /// Activate the user
    pub fn activate(&mut self) {
        self.is_active = true;
    }

    /// Deactivate the user
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
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

impl Permission {
    /// Get the string representation of the permission
    pub fn as_str(&self) -> &str {
        match self {
            Permission::ViewCharts => "view_charts",
            Permission::CreateCharts => "create_charts",
            Permission::EditCharts => "edit_charts",
            Permission::DeleteCharts => "delete_charts",
            Permission::ViewData => "view_data",
            Permission::EditData => "edit_data",
            Permission::DeleteData => "delete_data",
            Permission::ExportData => "export_data",
            Permission::ImportData => "import_data",
            Permission::ManageUsers => "manage_users",
            Permission::ManageRoles => "manage_roles",
            Permission::ManageSystem => "manage_system",
            Permission::ViewAuditLogs => "view_audit_logs",
            Permission::ExportPNG => "export_png",
            Permission::ExportSVG => "export_svg",
            Permission::ExportPDF => "export_pdf",
            Permission::ExportHTML => "export_html",
            Permission::Custom(name) => name,
        }
    }

    /// Create a custom permission
    pub fn custom(name: impl Into<String>) -> Self {
        Self::Custom(name.into())
    }
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

impl Credentials {
    /// Create new username/password credentials
    pub fn username_password(username: String, password: String) -> Self {
        Self {
            credential_type: CredentialType::UsernamePassword,
            username: Some(username),
            password: Some(password),
            token: None,
            additional_data: HashMap::new(),
        }
    }

    /// Create new bearer token credentials
    pub fn bearer_token(token: String) -> Self {
        Self {
            credential_type: CredentialType::BearerToken,
            username: None,
            password: None,
            token: Some(token),
            additional_data: HashMap::new(),
        }
    }

    /// Create new API key credentials
    pub fn api_key(api_key: String) -> Self {
        Self {
            credential_type: CredentialType::ApiKey,
            username: None,
            password: None,
            token: Some(api_key),
            additional_data: HashMap::new(),
        }
    }

    /// Add additional data to credentials
    pub fn add_data(&mut self, key: String, value: String) {
        self.additional_data.insert(key, value);
    }

    /// Get additional data from credentials
    pub fn get_data(&self, key: &str) -> Option<&String> {
        self.additional_data.get(key)
    }
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
    Biometric,
}

impl CredentialType {
    /// Get the string representation of the credential type
    pub fn as_str(&self) -> &str {
        match self {
            CredentialType::UsernamePassword => "username_password", // pragma: allowlist secret
            CredentialType::BearerToken => "bearer_token",           // pragma: allowlist secret
            CredentialType::ApiKey => "api_key",                     // pragma: allowlist secret
            CredentialType::OAuth2 => "oauth2",
            CredentialType::SAML => "saml",
            CredentialType::Certificate => "certificate",
            CredentialType::Biometric => "biometric",
        }
    }
}

/// Authentication result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResult {
    pub success: bool,
    pub user: Option<User>,
    pub token: Option<String>,
    pub expires_at: Option<u64>,
    pub error_message: Option<String>,
}

impl AuthenticationResult {
    /// Create a successful authentication result
    pub fn success(user: User, token: String, expires_at: u64) -> Self {
        Self {
            success: true,
            user: Some(user),
            token: Some(token),
            expires_at: Some(expires_at),
            error_message: None,
        }
    }

    /// Create a failed authentication result
    pub fn failure(error_message: String) -> Self {
        Self {
            success: false,
            user: None,
            token: None,
            expires_at: None,
            error_message: Some(error_message),
        }
    }
}

/// Authorization context for access control decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationContext {
    pub user: User,
    pub resource: Resource,
    pub action: Action,
    pub environment: HashMap<String, String>,
}

impl AuthorizationContext {
    /// Create a new authorization context
    pub fn new(user: User, resource: Resource, action: Action) -> Self {
        Self {
            user,
            resource,
            action,
            environment: HashMap::new(),
        }
    }

    /// Add environment variable to context
    pub fn add_environment(&mut self, key: String, value: String) {
        self.environment.insert(key, value);
    }

    /// Get environment variable from context
    pub fn get_environment(&self, key: &str) -> Option<&String> {
        self.environment.get(key)
    }
}

/// Resources that can be accessed
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Resource {
    Chart(String),
    Data(String),
    User(String),
    Role(String),
    System,
    AuditLog,
    Export(String),
    Import(String),
    Custom(String),
}

impl Resource {
    /// Get the string representation of the resource
    pub fn as_string(&self) -> String {
        match self {
            Resource::Chart(id) => format!("chart:{}", id),
            Resource::Data(id) => format!("data:{}", id),
            Resource::User(id) => format!("user:{}", id),
            Resource::Role(id) => format!("role:{}", id),
            Resource::System => "system".to_string(),
            Resource::AuditLog => "audit_log".to_string(),
            Resource::Export(id) => format!("export:{}", id),
            Resource::Import(id) => format!("import:{}", id),
            Resource::Custom(name) => name.clone(),
        }
    }

    /// Create a custom resource
    pub fn custom(name: impl Into<String>) -> Self {
        Self::Custom(name.into())
    }
}

/// Actions that can be performed on resources
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    Read,
    Write,
    Create,
    Delete,
    Execute,
    Export,
    Import,
    Manage,
    Custom(String),
}

impl Action {
    /// Get the string representation of the action
    pub fn as_str(&self) -> &str {
        match self {
            Action::Read => "read",
            Action::Write => "write",
            Action::Create => "create",
            Action::Delete => "delete",
            Action::Execute => "execute",
            Action::Export => "export",
            Action::Import => "import",
            Action::Manage => "manage",
            Action::Custom(name) => name,
        }
    }

    /// Create a custom action
    pub fn custom(name: impl Into<String>) -> Self {
        Self::Custom(name.into())
    }
}
