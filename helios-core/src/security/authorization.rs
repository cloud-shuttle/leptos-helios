//! Authorization and access control
//!
//! This module provides authorization providers and RBAC (Role-Based Access Control)
//! implementations for fine-grained access control.

use super::errors::SecurityError;
use super::types::*;
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Core authorization provider trait
#[async_trait]
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

/// Role definition
#[derive(Debug, Clone)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: String,
    pub permissions: HashSet<Permission>,
    pub inherits_from: Vec<String>, // Role IDs that this role inherits from
}

impl Role {
    pub fn new(id: String, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
            permissions: HashSet::new(),
            inherits_from: Vec::new(),
        }
    }

    pub fn add_permission(&mut self, permission: Permission) {
        self.permissions.insert(permission);
    }

    pub fn remove_permission(&mut self, permission: &Permission) {
        self.permissions.remove(permission);
    }

    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions.contains(permission)
    }

    pub fn add_inheritance(&mut self, role_id: String) {
        if !self.inherits_from.contains(&role_id) {
            self.inherits_from.push(role_id);
        }
    }

    pub fn remove_inheritance(&mut self, role_id: &str) {
        self.inherits_from.retain(|id| id != role_id);
    }
}

/// RBAC-based authorization provider
pub struct RBACProvider {
    roles: Arc<RwLock<HashMap<String, Role>>>,
    role_hierarchy: Arc<RwLock<HashMap<String, Vec<String>>>>, // role_id -> child_role_ids
}

impl RBACProvider {
    pub fn new() -> Self {
        Self {
            roles: Arc::new(RwLock::new(HashMap::new())),
            role_hierarchy: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_role(&self, role: Role) {
        let mut roles = self.roles.write().await;
        roles.insert(role.id.clone(), role);
    }

    pub async fn get_role(&self, role_id: &str) -> Option<Role> {
        let roles = self.roles.read().await;
        roles.get(role_id).cloned()
    }

    pub async fn remove_role(&self, role_id: &str) -> Option<Role> {
        let mut roles = self.roles.write().await;
        roles.remove(role_id)
    }

    pub async fn get_all_roles(&self) -> Vec<Role> {
        let roles = self.roles.read().await;
        roles.values().cloned().collect()
    }

    async fn get_effective_permissions(
        &self,
        user: &User,
    ) -> Result<HashSet<Permission>, SecurityError> {
        let mut effective_permissions = HashSet::new();
        let roles = self.roles.read().await;

        // Add direct permissions
        effective_permissions.extend(user.permissions.clone());

        // Add role-based permissions
        for role_id in &user.roles {
            if let Some(role) = roles.get(role_id) {
                effective_permissions.extend(role.permissions.clone());

                // Add inherited permissions
                for inherited_role_id in &role.inherits_from {
                    if let Some(inherited_role) = roles.get(inherited_role_id) {
                        effective_permissions.extend(inherited_role.permissions.clone());
                    }
                }
            }
        }

        Ok(effective_permissions)
    }

    async fn check_resource_permission(
        &self,
        user: &User,
        resource: &Resource,
        action: &Action,
    ) -> Result<bool, SecurityError> {
        let effective_permissions = self.get_effective_permissions(user).await?;

        // Map action to required permissions
        let required_permissions = match (resource, action) {
            (Resource::Chart(_), Action::Read) => vec![Permission::ViewCharts],
            (Resource::Chart(_), Action::Write) => vec![Permission::EditCharts],
            (Resource::Chart(_), Action::Create) => vec![Permission::CreateCharts],
            (Resource::Chart(_), Action::Delete) => vec![Permission::DeleteCharts],

            (Resource::Data(_), Action::Read) => vec![Permission::ViewData],
            (Resource::Data(_), Action::Write) => vec![Permission::EditData],
            (Resource::Data(_), Action::Create) => vec![Permission::EditData],
            (Resource::Data(_), Action::Delete) => vec![Permission::DeleteData],
            (Resource::Data(_), Action::Export) => vec![Permission::ExportData],
            (Resource::Data(_), Action::Import) => vec![Permission::ImportData],

            (Resource::User(_), Action::Read) => vec![Permission::ManageUsers],
            (Resource::User(_), Action::Write) => vec![Permission::ManageUsers],
            (Resource::User(_), Action::Create) => vec![Permission::ManageUsers],
            (Resource::User(_), Action::Delete) => vec![Permission::ManageUsers],

            (Resource::Role(_), Action::Read) => vec![Permission::ManageRoles],
            (Resource::Role(_), Action::Write) => vec![Permission::ManageRoles],
            (Resource::Role(_), Action::Create) => vec![Permission::ManageRoles],
            (Resource::Role(_), Action::Delete) => vec![Permission::ManageRoles],

            (Resource::System, _) => vec![Permission::ManageSystem],

            (Resource::AuditLog, Action::Read) => vec![Permission::ViewAuditLogs],

            (Resource::Export(_), Action::Execute) => match resource {
                Resource::Export(format) => match format.as_str() {
                    "png" => vec![Permission::ExportPNG],
                    "svg" => vec![Permission::ExportSVG],
                    "pdf" => vec![Permission::ExportPDF],
                    "html" => vec![Permission::ExportHTML],
                    _ => vec![Permission::ExportData],
                },
                _ => vec![Permission::ExportData],
            },

            (Resource::Import(_), Action::Execute) => vec![Permission::ImportData],

            _ => vec![], // No specific permissions required
        };

        // Check if user has any of the required permissions
        for required_permission in required_permissions {
            if effective_permissions.contains(&required_permission) {
                return Ok(true);
            }
        }

        // Check for custom permissions
        if let Action::Custom(custom_action) = action {
            if effective_permissions.contains(&Permission::Custom(custom_action.clone())) {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

#[async_trait]
impl AuthorizationProvider for RBACProvider {
    async fn authorize(
        &self,
        user: &User,
        context: &AuthorizationContext,
    ) -> Result<bool, SecurityError> {
        // Check if user is active
        if !user.is_active {
            return Ok(false);
        }

        // Check resource-specific permission
        self.check_resource_permission(user, &context.resource, &context.action)
            .await
    }

    async fn get_permissions(
        &self,
        user: &User,
        _resource: &Resource,
    ) -> Result<HashSet<Permission>, SecurityError> {
        self.get_effective_permissions(user).await
    }

    async fn has_permission(
        &self,
        user: &User,
        permission: &Permission,
    ) -> Result<bool, SecurityError> {
        let effective_permissions = self.get_effective_permissions(user).await?;
        Ok(effective_permissions.contains(permission))
    }
}

/// Policy-based authorization provider
pub struct PolicyProvider {
    policies: Arc<RwLock<Vec<Policy>>>,
}

/// Authorization policy
#[derive(Debug, Clone)]
pub struct Policy {
    pub id: String,
    pub name: String,
    pub description: String,
    pub effect: PolicyEffect,
    pub conditions: Vec<PolicyCondition>,
    pub resources: Vec<Resource>,
    pub actions: Vec<Action>,
    pub subjects: Vec<String>, // User IDs or role IDs
}

/// Policy effect (allow or deny)
#[derive(Debug, Clone, PartialEq)]
pub enum PolicyEffect {
    Allow,
    Deny,
}

/// Policy condition
#[derive(Debug, Clone)]
pub struct PolicyCondition {
    pub attribute: String,
    pub operator: PolicyOperator,
    pub value: String,
}

/// Policy operator
#[derive(Debug, Clone)]
pub enum PolicyOperator {
    Equals,
    NotEquals,
    Contains,
    StartsWith,
    EndsWith,
    GreaterThan,
    LessThan,
    In,
    NotIn,
}

impl PolicyProvider {
    pub fn new() -> Self {
        Self {
            policies: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn add_policy(&self, policy: Policy) {
        let mut policies = self.policies.write().await;
        policies.push(policy);
    }

    pub async fn get_policies(&self) -> Vec<Policy> {
        let policies = self.policies.read().await;
        policies.clone()
    }

    pub async fn remove_policy(&self, policy_id: &str) -> Option<Policy> {
        let mut policies = self.policies.write().await;
        if let Some(pos) = policies.iter().position(|p| p.id == policy_id) {
            Some(policies.remove(pos))
        } else {
            None
        }
    }

    async fn evaluate_condition(
        &self,
        condition: &PolicyCondition,
        context: &AuthorizationContext,
    ) -> bool {
        let attribute_value = match condition.attribute.as_str() {
            "user.id" => &context.user.id,
            "user.username" => &context.user.username,
            "user.email" => &context.user.email,
            "resource.type" => match &context.resource {
                Resource::Chart(_) => "chart",
                Resource::Data(_) => "data",
                Resource::User(_) => "user",
                Resource::Role(_) => "role",
                Resource::System => "system",
                Resource::AuditLog => "audit_log",
                Resource::Export(_) => "export",
                Resource::Import(_) => "import",
                Resource::Custom(name) => name,
            },
            "action.type" => match &context.action {
                Action::Read => "read",
                Action::Write => "write",
                Action::Create => "create",
                Action::Delete => "delete",
                Action::Execute => "execute",
                Action::Export => "export",
                Action::Import => "import",
                Action::Manage => "manage",
                Action::Custom(name) => name,
            },
            _ => {
                // Check environment variables
                if let Some(value) = context.get_environment(&condition.attribute) {
                    value
                } else {
                    return false;
                }
            }
        };

        match condition.operator {
            PolicyOperator::Equals => attribute_value == &condition.value,
            PolicyOperator::NotEquals => attribute_value != &condition.value,
            PolicyOperator::Contains => attribute_value.contains(&condition.value),
            PolicyOperator::StartsWith => attribute_value.starts_with(&condition.value),
            PolicyOperator::EndsWith => attribute_value.ends_with(&condition.value),
            PolicyOperator::GreaterThan => {
                if let (Ok(attr_num), Ok(cond_num)) = (
                    attribute_value.parse::<f64>(),
                    condition.value.parse::<f64>(),
                ) {
                    attr_num > cond_num
                } else {
                    false
                }
            }
            PolicyOperator::LessThan => {
                if let (Ok(attr_num), Ok(cond_num)) = (
                    attribute_value.parse::<f64>(),
                    condition.value.parse::<f64>(),
                ) {
                    attr_num < cond_num
                } else {
                    false
                }
            }
            PolicyOperator::In => {
                let values: Vec<&str> = condition.value.split(',').map(|s| s.trim()).collect();
                values.contains(&attribute_value)
            }
            PolicyOperator::NotIn => {
                let values: Vec<&str> = condition.value.split(',').map(|s| s.trim()).collect();
                !values.contains(&attribute_value)
            }
        }
    }

    async fn evaluate_policy(&self, policy: &Policy, context: &AuthorizationContext) -> bool {
        // Check if policy applies to the subject
        let applies_to_subject = policy.subjects.is_empty()
            || policy.subjects.contains(&context.user.id)
            || policy
                .subjects
                .iter()
                .any(|subject| context.user.has_role(subject));

        if !applies_to_subject {
            return false;
        }

        // Check if policy applies to the resource
        let applies_to_resource =
            policy.resources.is_empty() || policy.resources.contains(&context.resource);

        if !applies_to_resource {
            return false;
        }

        // Check if policy applies to the action
        let applies_to_action =
            policy.actions.is_empty() || policy.actions.contains(&context.action);

        if !applies_to_action {
            return false;
        }

        // Evaluate all conditions
        for condition in &policy.conditions {
            if !self.evaluate_condition(condition, context).await {
                return false;
            }
        }

        true
    }
}

#[async_trait]
impl AuthorizationProvider for PolicyProvider {
    async fn authorize(
        &self,
        _user: &User,
        context: &AuthorizationContext,
    ) -> Result<bool, SecurityError> {
        let policies = self.policies.read().await;
        let mut allow_policies = Vec::new();
        let mut deny_policies = Vec::new();

        // Separate allow and deny policies
        for policy in policies.iter() {
            if self.evaluate_policy(policy, context).await {
                match policy.effect {
                    PolicyEffect::Allow => allow_policies.push(policy),
                    PolicyEffect::Deny => deny_policies.push(policy),
                }
            }
        }

        // Deny policies take precedence
        if !deny_policies.is_empty() {
            return Ok(false);
        }

        // If there are allow policies, user is authorized
        Ok(!allow_policies.is_empty())
    }

    async fn get_permissions(
        &self,
        _user: &User,
        _resource: &Resource,
    ) -> Result<HashSet<Permission>, SecurityError> {
        // Policy-based authorization doesn't work with explicit permissions
        // Return empty set
        Ok(HashSet::new())
    }

    async fn has_permission(
        &self,
        _user: &User,
        _permission: &Permission,
    ) -> Result<bool, SecurityError> {
        // Policy-based authorization doesn't work with explicit permissions
        Ok(false)
    }
}
