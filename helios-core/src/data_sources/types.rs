//! Core data source types and data structures
//!
//! This module provides the fundamental types used throughout the data source system,
//! including connection configurations, schema information, and metadata structures.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Database connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub connection_string: String,
    pub max_connections: Option<u32>,
    pub connection_timeout: Option<Duration>,
    pub query_timeout: Option<Duration>,
    pub ssl_mode: Option<String>,
    pub credentials: Option<Credentials>,
}

impl ConnectionConfig {
    /// Create a new connection configuration
    pub fn new(connection_string: String) -> Self {
        Self {
            connection_string,
            max_connections: None,
            connection_timeout: None,
            query_timeout: None,
            ssl_mode: None,
            credentials: None,
        }
    }

    /// Set maximum connections
    pub fn with_max_connections(mut self, max_connections: u32) -> Self {
        self.max_connections = Some(max_connections);
        self
    }

    /// Set connection timeout
    pub fn with_connection_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = Some(timeout);
        self
    }

    /// Set query timeout
    pub fn with_query_timeout(mut self, timeout: Duration) -> Self {
        self.query_timeout = Some(timeout);
        self
    }

    /// Set SSL mode
    pub fn with_ssl_mode(mut self, ssl_mode: String) -> Self {
        self.ssl_mode = Some(ssl_mode);
        self
    }

    /// Set credentials
    pub fn with_credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }
}

/// Authentication credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub database: Option<String>,
    pub additional_params: HashMap<String, String>,
}

impl Credentials {
    /// Create new credentials
    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password,
            database: None,
            additional_params: HashMap::new(),
        }
    }

    /// Set database name
    pub fn with_database(mut self, database: String) -> Self {
        self.database = Some(database);
        self
    }

    /// Add additional parameter
    pub fn add_param(&mut self, key: String, value: String) {
        self.additional_params.insert(key, value);
    }

    /// Get additional parameter
    pub fn get_param(&self, key: &str) -> Option<&String> {
        self.additional_params.get(key)
    }
}

/// Database schema information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub tables: Vec<TableInfo>,
    pub views: Vec<ViewInfo>,
    pub version: Option<String>,
}

impl Schema {
    /// Create a new schema
    pub fn new() -> Self {
        Self {
            tables: Vec::new(),
            views: Vec::new(),
            version: None,
        }
    }

    /// Add a table to the schema
    pub fn add_table(&mut self, table: TableInfo) {
        self.tables.push(table);
    }

    /// Add a view to the schema
    pub fn add_view(&mut self, view: ViewInfo) {
        self.views.push(view);
    }

    /// Set schema version
    pub fn set_version(&mut self, version: String) {
        self.version = Some(version);
    }

    /// Find table by name
    pub fn find_table(&self, name: &str) -> Option<&TableInfo> {
        self.tables.iter().find(|t| t.name == name)
    }

    /// Find view by name
    pub fn find_view(&self, name: &str) -> Option<&ViewInfo> {
        self.views.iter().find(|v| v.name == name)
    }
}

/// Table metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableInfo {
    pub name: String,
    pub schema: Option<String>,
    pub columns: Vec<ColumnInfo>,
    pub primary_keys: Vec<String>,
    pub indexes: Vec<IndexInfo>,
}

impl TableInfo {
    /// Create a new table info
    pub fn new(name: String) -> Self {
        Self {
            name,
            schema: None,
            columns: Vec::new(),
            primary_keys: Vec::new(),
            indexes: Vec::new(),
        }
    }

    /// Set schema name
    pub fn with_schema(mut self, schema: String) -> Self {
        self.schema = Some(schema);
        self
    }

    /// Add a column
    pub fn add_column(&mut self, column: ColumnInfo) {
        self.columns.push(column);
    }

    /// Add a primary key
    pub fn add_primary_key(&mut self, column_name: String) {
        if !self.primary_keys.contains(&column_name) {
            self.primary_keys.push(column_name);
        }
    }

    /// Add an index
    pub fn add_index(&mut self, index: IndexInfo) {
        self.indexes.push(index);
    }

    /// Find column by name
    pub fn find_column(&self, name: &str) -> Option<&ColumnInfo> {
        self.columns.iter().find(|c| c.name == name)
    }

    /// Check if column is primary key
    pub fn is_primary_key(&self, column_name: &str) -> bool {
        self.primary_keys.contains(&column_name.to_string())
    }
}

/// Column metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub default_value: Option<String>,
    pub max_length: Option<i32>,
}

impl ColumnInfo {
    /// Create a new column info
    pub fn new(name: String, data_type: String) -> Self {
        Self {
            name,
            data_type,
            is_nullable: true,
            default_value: None,
            max_length: None,
        }
    }

    /// Set nullable flag
    pub fn with_nullable(mut self, nullable: bool) -> Self {
        self.is_nullable = nullable;
        self
    }

    /// Set default value
    pub fn with_default_value(mut self, default_value: String) -> Self {
        self.default_value = Some(default_value);
        self
    }

    /// Set maximum length
    pub fn with_max_length(mut self, max_length: i32) -> Self {
        self.max_length = Some(max_length);
        self
    }
}

/// View metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewInfo {
    pub name: String,
    pub schema: Option<String>,
    pub definition: String,
}

impl ViewInfo {
    /// Create a new view info
    pub fn new(name: String, definition: String) -> Self {
        Self {
            name,
            schema: None,
            definition,
        }
    }

    /// Set schema name
    pub fn with_schema(mut self, schema: String) -> Self {
        self.schema = Some(schema);
        self
    }
}

/// Index metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexInfo {
    pub name: String,
    pub columns: Vec<String>,
    pub is_unique: bool,
    pub is_primary: bool,
    pub is_clustered: bool,
}

impl IndexInfo {
    /// Create a new index info
    pub fn new(name: String, columns: Vec<String>) -> Self {
        Self {
            name,
            columns,
            is_unique: false,
            is_primary: false,
            is_clustered: false,
        }
    }

    /// Set unique flag
    pub fn with_unique(mut self, unique: bool) -> Self {
        self.is_unique = unique;
        self
    }

    /// Set primary flag
    pub fn with_primary(mut self, primary: bool) -> Self {
        self.is_primary = primary;
        self
    }

    /// Set clustered flag
    pub fn with_clustered(mut self, clustered: bool) -> Self {
        self.is_clustered = clustered;
        self
    }

    /// Add a column to the index
    pub fn add_column(&mut self, column: String) {
        if !self.columns.contains(&column) {
            self.columns.push(column);
        }
    }
}

/// Query result metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub data_types: Vec<String>,
    pub row_count: usize,
    pub execution_time: Duration,
}

impl QueryResult {
    /// Create a new query result
    pub fn new(
        columns: Vec<String>,
        data_types: Vec<String>,
        row_count: usize,
        execution_time: Duration,
    ) -> Self {
        Self {
            columns,
            data_types,
            row_count,
            execution_time,
        }
    }

    /// Get column count
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Check if result is empty
    pub fn is_empty(&self) -> bool {
        self.row_count == 0
    }
}

/// Data source statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceStats {
    pub total_connections: u32,
    pub active_connections: u32,
    pub total_queries: u64,
    pub failed_queries: u64,
    pub average_query_time: Duration,
    pub last_activity: Option<std::time::SystemTime>,
}

impl DataSourceStats {
    /// Create new statistics
    pub fn new() -> Self {
        Self {
            total_connections: 0,
            active_connections: 0,
            total_queries: 0,
            failed_queries: 0,
            average_query_time: Duration::from_millis(0),
            last_activity: None,
        }
    }

    /// Get success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_queries == 0 {
            1.0
        } else {
            (self.total_queries - self.failed_queries) as f64 / self.total_queries as f64
        }
    }

    /// Get failure rate
    pub fn failure_rate(&self) -> f64 {
        1.0 - self.success_rate()
    }

    /// Update last activity
    pub fn update_activity(&mut self) {
        self.last_activity = Some(std::time::SystemTime::now());
    }
}
