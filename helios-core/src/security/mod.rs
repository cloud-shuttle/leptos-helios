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
//! - `session`: Session management
//! - `manager`: Main security manager

pub mod auth;
pub mod authorization;
pub mod errors;
pub mod types;
pub mod audit;
pub mod session;
pub mod manager;

// Re-export main types for convenience
pub use auth::*;
pub use authorization::*;
pub use errors::*;
pub use types::*;
pub use audit::*;
pub use session::*;
pub use manager::*;