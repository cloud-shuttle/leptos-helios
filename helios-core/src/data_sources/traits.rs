//! Core data source traits and interfaces
//!
//! This module provides the fundamental traits for data source connectivity,
//! including data source management, connections, transactions, and data streaming.

use super::errors::DataSourceError;
use super::types::*;
use async_trait::async_trait;
use polars::prelude::*;
use serde::{Deserialize, Serialize};
// use std::collections::HashMap; // Currently unused

/// Core data source trait
#[async_trait]
pub trait DataSource: Send + Sync {
    /// Get the data source name
    fn name(&self) -> &str;

    /// Get the data source type
    fn source_type(&self) -> &str;

    /// Connect to the data source
    async fn connect(
        &self,
        config: &ConnectionConfig,
    ) -> Result<Box<dyn Connection>, DataSourceError>;

    /// Test the connection
    async fn test_connection(&self, config: &ConnectionConfig) -> Result<bool, DataSourceError>;

    /// Get supported features
    fn supported_features(&self) -> Vec<String>;

    /// Get connection limits
    fn connection_limits(&self) -> ConnectionLimits;

    /// Get data source statistics
    async fn get_stats(&self) -> Result<DataSourceStats, DataSourceError>;
}

/// Database connection trait
#[async_trait]
pub trait Connection: Send + Sync {
    /// Execute a query and return results as DataFrame
    async fn execute_query(&self, query: &str) -> Result<DataFrame, DataSourceError>;

    /// Execute a query with parameters
    async fn execute_query_with_params(
        &self,
        query: &str,
        params: &[&dyn ToSql],
    ) -> Result<DataFrame, DataSourceError>;

    /// Execute a non-query statement (INSERT, UPDATE, DELETE)
    async fn execute_statement(&self, statement: &str) -> Result<u64, DataSourceError>;

    /// Execute a statement with parameters
    async fn execute_statement_with_params(
        &self,
        statement: &str,
        params: &[&dyn ToSql],
    ) -> Result<u64, DataSourceError>;

    /// Begin a transaction
    async fn begin_transaction(&self) -> Result<Box<dyn Transaction>, DataSourceError>;

    /// Get database schema
    async fn get_schema(&self) -> Result<super::types::Schema, DataSourceError>;

    /// Get table information
    async fn get_table_info(&self, table_name: &str) -> Result<TableInfo, DataSourceError>;

    /// List all tables
    async fn list_tables(&self) -> Result<Vec<String>, DataSourceError>;

    /// List all views
    async fn list_views(&self) -> Result<Vec<String>, DataSourceError>;

    /// Check if table exists
    async fn table_exists(&self, table_name: &str) -> Result<bool, DataSourceError>;

    /// Get connection statistics
    async fn get_stats(&self) -> Result<ConnectionStats, DataSourceError>;

    /// Close the connection
    async fn close(&self) -> Result<(), DataSourceError>;
}

/// Database transaction trait
#[async_trait]
pub trait Transaction: Send + Sync {
    /// Execute a query within the transaction
    async fn execute_query(&self, query: &str) -> Result<DataFrame, DataSourceError>;

    /// Execute a query with parameters within the transaction
    async fn execute_query_with_params(
        &self,
        query: &str,
        params: &[&dyn ToSql],
    ) -> Result<DataFrame, DataSourceError>;

    /// Execute a statement within the transaction
    async fn execute_statement(&self, statement: &str) -> Result<u64, DataSourceError>;

    /// Execute a statement with parameters within the transaction
    async fn execute_statement_with_params(
        &self,
        statement: &str,
        params: &[&dyn ToSql],
    ) -> Result<u64, DataSourceError>;

    /// Commit the transaction
    async fn commit(&self) -> Result<(), DataSourceError>;

    /// Rollback the transaction
    async fn rollback(&self) -> Result<(), DataSourceError>;

    /// Check if transaction is active
    fn is_active(&self) -> bool;
}

/// Data streaming trait
#[async_trait]
pub trait DataStream: Send + Sync {
    /// Get the next batch of data
    async fn next_batch(&mut self) -> Result<Option<DataFrame>, DataSourceError>;

    /// Check if stream has more data
    fn has_more(&self) -> bool;

    /// Get stream statistics
    fn get_stats(&self) -> StreamStats;

    /// Close the stream
    async fn close(&mut self) -> Result<(), DataSourceError>;
}

/// SQL parameter binding trait
pub trait ToSql: Send + Sync {
    /// Convert to SQL parameter
    fn to_sql(&self) -> String;
}

// Implement ToSql for common types
impl ToSql for String {
    fn to_sql(&self) -> String {
        format!("'{}'", self.replace("'", "''"))
    }
}

impl ToSql for &str {
    fn to_sql(&self) -> String {
        format!("'{}'", self.replace("'", "''"))
    }
}

impl ToSql for i32 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl ToSql for i64 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl ToSql for f32 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl ToSql for f64 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl ToSql for bool {
    fn to_sql(&self) -> String {
        if *self { "TRUE" } else { "FALSE" }.to_string()
    }
}

/// Connection limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionLimits {
    pub max_connections: u32,
    pub max_query_time: std::time::Duration,
    pub max_result_size: usize,
    pub max_parameters: u32,
}

impl ConnectionLimits {
    /// Create new connection limits
    pub fn new() -> Self {
        Self {
            max_connections: 100,
            max_query_time: std::time::Duration::from_secs(300), // 5 minutes
            max_result_size: 100_000_000,                        // 100MB
            max_parameters: 1000,
        }
    }

    /// Set maximum connections
    pub fn with_max_connections(mut self, max_connections: u32) -> Self {
        self.max_connections = max_connections;
        self
    }

    /// Set maximum query time
    pub fn with_max_query_time(mut self, max_query_time: std::time::Duration) -> Self {
        self.max_query_time = max_query_time;
        self
    }

    /// Set maximum result size
    pub fn with_max_result_size(mut self, max_result_size: usize) -> Self {
        self.max_result_size = max_result_size;
        self
    }

    /// Set maximum parameters
    pub fn with_max_parameters(mut self, max_parameters: u32) -> Self {
        self.max_parameters = max_parameters;
        self
    }
}

/// Connection statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStats {
    pub connection_id: String,
    pub connected_at: std::time::SystemTime,
    pub total_queries: u64,
    pub failed_queries: u64,
    pub total_execution_time: std::time::Duration,
    pub last_query_time: Option<std::time::SystemTime>,
}

impl ConnectionStats {
    /// Create new connection statistics
    pub fn new(connection_id: String) -> Self {
        Self {
            connection_id,
            connected_at: std::time::SystemTime::now(),
            total_queries: 0,
            failed_queries: 0,
            total_execution_time: std::time::Duration::from_millis(0),
            last_query_time: None,
        }
    }

    /// Record a successful query
    pub fn record_successful_query(&mut self, execution_time: std::time::Duration) {
        self.total_queries += 1;
        self.total_execution_time += execution_time;
        self.last_query_time = Some(std::time::SystemTime::now());
    }

    /// Record a failed query
    pub fn record_failed_query(&mut self) {
        self.failed_queries += 1;
        self.last_query_time = Some(std::time::SystemTime::now());
    }

    /// Get average execution time
    pub fn average_execution_time(&self) -> std::time::Duration {
        if self.total_queries == 0 {
            std::time::Duration::from_millis(0)
        } else {
            self.total_execution_time / self.total_queries as u32
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
}

/// Stream statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamStats {
    pub total_rows: u64,
    pub batches_processed: u64,
    pub bytes_processed: u64,
    pub start_time: std::time::SystemTime,
    pub last_batch_time: Option<std::time::SystemTime>,
}

impl StreamStats {
    /// Create new stream statistics
    pub fn new() -> Self {
        Self {
            total_rows: 0,
            batches_processed: 0,
            bytes_processed: 0,
            start_time: std::time::SystemTime::now(),
            last_batch_time: None,
        }
    }

    /// Record a batch
    pub fn record_batch(&mut self, rows: usize, bytes: usize) {
        self.total_rows += rows as u64;
        self.batches_processed += 1;
        self.bytes_processed += bytes as u64;
        self.last_batch_time = Some(std::time::SystemTime::now());
    }

    /// Get processing rate (rows per second)
    pub fn rows_per_second(&self) -> f64 {
        let elapsed = self
            .start_time
            .elapsed()
            .unwrap_or(std::time::Duration::from_secs(1));
        if elapsed.as_secs() == 0 {
            0.0
        } else {
            self.total_rows as f64 / elapsed.as_secs_f64()
        }
    }

    /// Get bytes per second
    pub fn bytes_per_second(&self) -> f64 {
        let elapsed = self
            .start_time
            .elapsed()
            .unwrap_or(std::time::Duration::from_secs(1));
        if elapsed.as_secs() == 0 {
            0.0
        } else {
            self.bytes_processed as f64 / elapsed.as_secs_f64()
        }
    }
}
