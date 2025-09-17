//! Data source connection errors
//!
//! This module provides comprehensive error handling for data source connections,
//! including database connectivity, query execution, and schema introspection errors.

use std::time::Duration;

/// Data source connection errors
#[derive(Debug, thiserror::Error)]
pub enum DataSourceError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Query execution failed: {0}")]
    QueryFailed(String),

    #[error("Schema introspection failed: {0}")]
    SchemaFailed(String),

    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),

    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("Timeout error: {0}")]
    Timeout(String),

    #[error("Data type conversion failed: {0}")]
    DataTypeConversion(String),

    #[error("Transaction failed: {0}")]
    TransactionFailed(String),

    #[error("Connection pool exhausted: {0}")]
    PoolExhausted(String),
}

impl DataSourceError {
    /// Create a new connection failed error
    pub fn connection_failed(message: impl Into<String>) -> Self {
        Self::ConnectionFailed(message.into())
    }

    /// Create a new query execution failed error
    pub fn query_failed(message: impl Into<String>) -> Self {
        Self::QueryFailed(message.into())
    }

    /// Create a new schema introspection failed error
    pub fn schema_failed(message: impl Into<String>) -> Self {
        Self::SchemaFailed(message.into())
    }

    /// Create a new unsupported operation error
    pub fn unsupported_operation(message: impl Into<String>) -> Self {
        Self::UnsupportedOperation(message.into())
    }

    /// Create a new authentication failed error
    pub fn authentication_failed(message: impl Into<String>) -> Self {
        Self::AuthenticationFailed(message.into())
    }

    /// Create a new invalid configuration error
    pub fn invalid_configuration(message: impl Into<String>) -> Self {
        Self::InvalidConfiguration(message.into())
    }

    /// Create a new timeout error
    pub fn timeout(message: impl Into<String>) -> Self {
        Self::Timeout(message.into())
    }

    /// Create a new data type conversion error
    pub fn data_type_conversion(message: impl Into<String>) -> Self {
        Self::DataTypeConversion(message.into())
    }

    /// Create a new transaction failed error
    pub fn transaction_failed(message: impl Into<String>) -> Self {
        Self::TransactionFailed(message.into())
    }

    /// Create a new connection pool exhausted error
    pub fn pool_exhausted(message: impl Into<String>) -> Self {
        Self::PoolExhausted(message.into())
    }

    /// Check if this is a connection error
    pub fn is_connection_error(&self) -> bool {
        matches!(
            self,
            DataSourceError::ConnectionFailed(_) | DataSourceError::AuthenticationFailed(_)
        )
    }

    /// Check if this is a query error
    pub fn is_query_error(&self) -> bool {
        matches!(
            self,
            DataSourceError::QueryFailed(_) | DataSourceError::UnsupportedOperation(_)
        )
    }

    /// Check if this is a configuration error
    pub fn is_configuration_error(&self) -> bool {
        matches!(self, DataSourceError::InvalidConfiguration(_))
    }

    /// Check if this is a timeout error
    pub fn is_timeout_error(&self) -> bool {
        matches!(self, DataSourceError::Timeout(_))
    }

    /// Check if this is a transaction error
    pub fn is_transaction_error(&self) -> bool {
        matches!(self, DataSourceError::TransactionFailed(_))
    }

    /// Check if this is a pool error
    pub fn is_pool_error(&self) -> bool {
        matches!(self, DataSourceError::PoolExhausted(_))
    }
}
