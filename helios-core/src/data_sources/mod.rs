//! Data Source Adapters
//!
//! This module provides database and data source connectivity for Helios,
//! including PostgreSQL, ClickHouse, JSON, and Parquet adapters.
//!
//! ## Module Structure
//!
//! - `errors`: Data source error types and handling
//! - `types`: Core data source types and data structures
//! - `traits`: Core data source traits and interfaces
//! - `database`: Database-specific adapters (PostgreSQL, ClickHouse)
//! - `file`: File-based data sources (JSON, Parquet, CSV)
//! - `api`: API-based data sources (REST, GraphQL)
//! - `validation`: Data validation and schema checking

pub mod database;
pub mod errors;
pub mod traits;
pub mod types;

// Re-export main types for convenience
pub use database::*;
pub use errors::*;
pub use traits::*;
pub use types::*;

use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main data source manager that coordinates all data source operations
pub struct DataSourceManager {
    adapters: HashMap<String, Box<dyn DataSource>>,
    connections: HashMap<String, Arc<RwLock<Box<dyn Connection>>>>,
    stats: DataSourceStats,
}

impl DataSourceManager {
    /// Create a new data source manager
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
            connections: HashMap::new(),
            stats: DataSourceStats::new(),
        }
    }

    /// Add a data source adapter
    pub fn add_adapter(&mut self, name: String, adapter: Box<dyn DataSource>) {
        self.adapters.insert(name, adapter);
    }

    /// Get a data source adapter
    pub fn get_adapter(&self, name: &str) -> Option<&Box<dyn DataSource>> {
        self.adapters.get(name)
    }

    /// Connect to a data source
    pub async fn connect(
        &mut self,
        adapter_name: &str,
        config: &ConnectionConfig,
    ) -> Result<String, DataSourceError> {
        let adapter = self.adapters.get(adapter_name).ok_or_else(|| {
            DataSourceError::ConnectionFailed(format!("Adapter '{}' not found", adapter_name))
        })?;

        let connection = adapter.connect(config).await?;
        let connection_id = uuid::Uuid::new_v4().to_string();

        self.connections
            .insert(connection_id.clone(), Arc::new(RwLock::new(connection)));
        self.stats.total_connections += 1;
        self.stats.active_connections += 1;
        self.stats.update_activity();

        Ok(connection_id)
    }

    /// Execute a query on a specific connection
    pub async fn execute_query(
        &mut self,
        connection_id: &str,
        query: &str,
    ) -> Result<DataFrame, DataSourceError> {
        let connection = self.connections.get(connection_id).ok_or_else(|| {
            DataSourceError::ConnectionFailed(format!("Connection '{}' not found", connection_id))
        })?;

        let start_time = std::time::Instant::now();
        let result = connection.read().await.execute_query(query).await;
        let execution_time = start_time.elapsed();

        match result {
            Ok(df) => {
                self.stats.total_queries += 1;
                self.stats.update_activity();
                Ok(df)
            }
            Err(e) => {
                self.stats.failed_queries += 1;
                self.stats.update_activity();
                Err(e)
            }
        }
    }

    /// Execute a query with parameters
    pub async fn execute_query_with_params(
        &mut self,
        connection_id: &str,
        query: &str,
        params: &[&dyn ToSql],
    ) -> Result<DataFrame, DataSourceError> {
        let connection = self.connections.get(connection_id).ok_or_else(|| {
            DataSourceError::ConnectionFailed(format!("Connection '{}' not found", connection_id))
        })?;

        let start_time = std::time::Instant::now();
        let result = connection
            .read()
            .await
            .execute_query_with_params(query, params)
            .await;
        let execution_time = start_time.elapsed();

        match result {
            Ok(df) => {
                self.stats.total_queries += 1;
                self.stats.update_activity();
                Ok(df)
            }
            Err(e) => {
                self.stats.failed_queries += 1;
                self.stats.update_activity();
                Err(e)
            }
        }
    }

    /// Get schema for a connection
    pub async fn get_schema(&self, connection_id: &str) -> Result<types::Schema, DataSourceError> {
        let connection = self.connections.get(connection_id).ok_or_else(|| {
            DataSourceError::ConnectionFailed(format!("Connection '{}' not found", connection_id))
        })?;

        connection.read().await.get_schema().await
    }

    /// List all tables for a connection
    pub async fn list_tables(&self, connection_id: &str) -> Result<Vec<String>, DataSourceError> {
        let connection = self.connections.get(connection_id).ok_or_else(|| {
            DataSourceError::ConnectionFailed(format!("Connection '{}' not found", connection_id))
        })?;

        connection.read().await.list_tables().await
    }

    /// Close a connection
    pub async fn close_connection(&mut self, connection_id: &str) -> Result<(), DataSourceError> {
        if let Some(connection) = self.connections.remove(connection_id) {
            connection.read().await.close().await?;
            self.stats.active_connections = self.stats.active_connections.saturating_sub(1);
            self.stats.update_activity();
        }
        Ok(())
    }

    /// Get data source statistics
    pub fn get_stats(&self) -> &DataSourceStats {
        &self.stats
    }

    /// Get all available adapters
    pub fn list_adapters(&self) -> Vec<String> {
        self.adapters.keys().cloned().collect()
    }

    /// Get all active connections
    pub fn list_connections(&self) -> Vec<String> {
        self.connections.keys().cloned().collect()
    }

    /// Test a connection
    pub async fn test_connection(
        &self,
        adapter_name: &str,
        config: &ConnectionConfig,
    ) -> Result<bool, DataSourceError> {
        let adapter = self.adapters.get(adapter_name).ok_or_else(|| {
            DataSourceError::ConnectionFailed(format!("Adapter '{}' not found", adapter_name))
        })?;

        adapter.test_connection(config).await
    }
}

impl Default for DataSourceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Data source factory for creating adapters
pub struct DataSourceFactory;

impl DataSourceFactory {
    /// Create a PostgreSQL adapter
    pub fn create_postgres_adapter(config: ConnectionConfig) -> PostgresAdapter {
        PostgresAdapter::new(config)
    }

    /// Create a ClickHouse adapter
    pub fn create_clickhouse_adapter(config: ConnectionConfig) -> ClickHouseAdapter {
        ClickHouseAdapter::new(config)
    }

    /// Create a data source manager with default adapters
    pub fn create_manager_with_defaults() -> DataSourceManager {
        let mut manager = DataSourceManager::new();

        // Add default adapters (these would be initialized with default configs)
        // manager.add_adapter("postgres".to_string(), Box::new(PostgresAdapter::new(ConnectionConfig::new("".to_string()))));
        // manager.add_adapter("clickhouse".to_string(), Box::new(ClickHouseAdapter::new(ConnectionConfig::new("".to_string()))));

        manager
    }
}

/// Data source configuration builder
pub struct DataSourceConfigBuilder {
    config: ConnectionConfig,
}

impl DataSourceConfigBuilder {
    /// Create a new configuration builder
    pub fn new(connection_string: String) -> Self {
        Self {
            config: ConnectionConfig::new(connection_string),
        }
    }

    /// Set maximum connections
    pub fn max_connections(mut self, max_connections: u32) -> Self {
        self.config.max_connections = Some(max_connections);
        self
    }

    /// Set connection timeout
    pub fn connection_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.config.connection_timeout = Some(timeout);
        self
    }

    /// Set query timeout
    pub fn query_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.config.query_timeout = Some(timeout);
        self
    }

    /// Set SSL mode
    pub fn ssl_mode(mut self, ssl_mode: String) -> Self {
        self.config.ssl_mode = Some(ssl_mode);
        self
    }

    /// Set credentials
    pub fn credentials(mut self, username: String, password: String) -> Self {
        self.config.credentials = Some(Credentials::new(username, password));
        self
    }

    /// Build the configuration
    pub fn build(self) -> ConnectionConfig {
        self.config
    }
}

/// Utility functions for data source operations
pub mod utils {
    use super::*;

    /// Validate a connection string
    pub fn validate_connection_string(connection_string: &str) -> Result<(), DataSourceError> {
        if connection_string.is_empty() {
            return Err(DataSourceError::InvalidConfiguration(
                "Connection string cannot be empty".to_string(),
            ));
        }

        // Basic validation - check for required components
        if !connection_string.contains("://") {
            return Err(DataSourceError::InvalidConfiguration(
                "Connection string must contain protocol (://)".to_string(),
            ));
        }

        Ok(())
    }

    /// Parse connection string into components
    pub fn parse_connection_string(
        connection_string: &str,
    ) -> Result<ConnectionComponents, DataSourceError> {
        validate_connection_string(connection_string)?;

        let parts: Vec<&str> = connection_string.split("://").collect();
        if parts.len() != 2 {
            return Err(DataSourceError::InvalidConfiguration(
                "Invalid connection string format".to_string(),
            ));
        }

        let protocol = parts[0].to_string();
        let rest = parts[1];

        // Parse host, port, database, etc.
        let mut components = ConnectionComponents {
            protocol,
            host: "localhost".to_string(),
            port: None,
            database: None,
            username: None,
            password: None,
            params: HashMap::new(),
        };

        // Simple parsing - in real implementation, use proper URL parsing
        if let Some(at_pos) = rest.find('@') {
            let auth_part = &rest[..at_pos];
            let host_part = &rest[at_pos + 1..];

            if let Some(colon_pos) = auth_part.find(':') {
                components.username = Some(auth_part[..colon_pos].to_string());
                components.password = Some(auth_part[colon_pos + 1..].to_string());
            } else {
                components.username = Some(auth_part.to_string());
            }

            // Parse host part
            if let Some(slash_pos) = host_part.find('/') {
                let host_port = &host_part[..slash_pos];
                components.database = Some(host_part[slash_pos + 1..].to_string());

                if let Some(colon_pos) = host_port.find(':') {
                    components.host = host_port[..colon_pos].to_string();
                    if let Ok(port) = host_port[colon_pos + 1..].parse::<u16>() {
                        components.port = Some(port);
                    }
                } else {
                    components.host = host_port.to_string();
                }
            } else {
                components.host = host_part.to_string();
            }
        }

        Ok(components)
    }

    /// Connection string components
    #[derive(Debug, Clone)]
    pub struct ConnectionComponents {
        pub protocol: String,
        pub host: String,
        pub port: Option<u16>,
        pub database: Option<String>,
        pub username: Option<String>,
        pub password: Option<String>,
        pub params: HashMap<String, String>,
    }
}
