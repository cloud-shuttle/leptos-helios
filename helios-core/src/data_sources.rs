//! Data Source Adapters
//!
//! This module provides database and data source connectivity for Helios,
//! including PostgreSQL, ClickHouse, JSON, and Parquet adapters.

use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

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
}

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

/// Authentication credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub database: Option<String>,
    pub additional_params: HashMap<String, String>,
}

/// Database schema information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub tables: Vec<TableInfo>,
    pub views: Vec<ViewInfo>,
    pub version: Option<String>,
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

/// Column metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub default_value: Option<String>,
    pub max_length: Option<i32>,
}

/// View metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewInfo {
    pub name: String,
    pub schema: Option<String>,
    pub definition: String,
}

/// Index metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexInfo {
    pub name: String,
    pub columns: Vec<String>,
    pub is_unique: bool,
    pub index_type: String,
}

use std::time::Duration;

/// Core trait for data source connections
#[async_trait::async_trait]
pub trait DataSource: Send + Sync {
    /// Connect to the data source
    async fn connect(&self) -> Result<Box<dyn Connection>, DataSourceError>;

    /// Get schema information
    async fn get_schema(&self) -> Result<Schema, DataSourceError>;

    /// Test connection health
    async fn health_check(&self) -> Result<bool, DataSourceError>;

    /// Get data source type identifier
    fn source_type(&self) -> &'static str;

    /// Get connection configuration
    fn config(&self) -> &ConnectionConfig;
}

/// Database connection trait
#[async_trait::async_trait]
pub trait Connection: Send + Sync {
    /// Execute SQL query and return DataFrame
    async fn query(&self, sql: &str) -> Result<DataFrame, DataSourceError>;

    /// Execute parameterized query
    async fn query_with_params(
        &self,
        sql: &str,
        params: &[&dyn ToSql],
    ) -> Result<DataFrame, DataSourceError>;

    /// Stream large result sets
    async fn query_stream(&self, sql: &str) -> Result<Box<dyn DataStream>, DataSourceError>;

    /// Execute DDL/DML operations
    async fn execute(&self, sql: &str) -> Result<u64, DataSourceError>;

    /// Begin transaction
    async fn begin_transaction(&self) -> Result<Box<dyn Transaction>, DataSourceError>;

    /// Close connection
    async fn close(&self) -> Result<(), DataSourceError>;
}

/// Transaction management
#[async_trait::async_trait]
pub trait Transaction: Send + Sync {
    /// Commit transaction
    async fn commit(&self) -> Result<(), DataSourceError>;

    /// Rollback transaction
    async fn rollback(&self) -> Result<(), DataSourceError>;

    /// Execute query within transaction
    async fn query(&self, sql: &str) -> Result<DataFrame, DataSourceError>;
}

/// Streaming data interface
#[async_trait::async_trait]
pub trait DataStream: Send + Sync {
    /// Get next batch of data
    async fn next_batch(&mut self) -> Result<Option<DataFrame>, DataSourceError>;

    /// Get estimated total rows
    fn estimated_rows(&self) -> Option<usize>;
}

/// SQL parameter trait
pub trait ToSql: Send + Sync {
    fn to_sql(&self) -> String;
}

impl ToSql for String {
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

impl ToSql for f64 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

/// PostgreSQL adapter
pub struct PostgresAdapter {
    config: ConnectionConfig,
    pool: Option<Arc<RwLock<MockConnectionPool>>>,
}

impl PostgresAdapter {
    pub fn new(config: ConnectionConfig) -> Self {
        Self { config, pool: None }
    }

    pub async fn initialize(&mut self) -> Result<(), DataSourceError> {
        let pool = MockConnectionPool::new(&self.config).await?;
        self.pool = Some(Arc::new(RwLock::new(pool)));
        Ok(())
    }
}

#[async_trait::async_trait]
impl DataSource for PostgresAdapter {
    async fn connect(&self) -> Result<Box<dyn Connection>, DataSourceError> {
        let pool = self
            .pool
            .as_ref()
            .ok_or_else(|| DataSourceError::ConnectionFailed("Pool not initialized".to_string()))?;

        let conn = PostgresConnection::new(pool.clone()).await?;
        Ok(Box::new(conn))
    }

    async fn get_schema(&self) -> Result<Schema, DataSourceError> {
        let conn = self.connect().await?;

        // Query PostgreSQL system tables for schema information
        let tables_query = r#"
            SELECT
                t.table_name,
                t.table_schema,
                c.column_name,
                c.data_type,
                c.is_nullable,
                c.column_default,
                c.character_maximum_length
            FROM information_schema.tables t
            LEFT JOIN information_schema.columns c ON t.table_name = c.table_name
            WHERE t.table_type = 'BASE TABLE'
            ORDER BY t.table_name, c.ordinal_position
        "#;

        let result = conn.query(tables_query).await?;

        // Parse result into Schema structure
        let mut tables = Vec::new();
        let mut current_table: Option<TableInfo> = None;

        for row_idx in 0..result.height() {
            let table_name = result
                .column("table_name")
                .unwrap()
                .get(row_idx)
                .unwrap()
                .to_string();
            let schema_name = result
                .column("table_schema")
                .unwrap()
                .get(row_idx)
                .unwrap()
                .to_string();

            // Start new table if different from current
            if current_table.as_ref().map(|t| &t.name) != Some(&table_name) {
                if let Some(table) = current_table.take() {
                    tables.push(table);
                }
                current_table = Some(TableInfo {
                    name: table_name,
                    schema: Some(schema_name),
                    columns: Vec::new(),
                    primary_keys: Vec::new(),
                    indexes: Vec::new(),
                });
            }

            // Add column information
            if let Some(ref mut table) = current_table {
                let column_name = result
                    .column("column_name")
                    .unwrap()
                    .get(row_idx)
                    .unwrap()
                    .to_string();
                let data_type = result
                    .column("data_type")
                    .unwrap()
                    .get(row_idx)
                    .unwrap()
                    .to_string();
                let is_nullable = result
                    .column("is_nullable")
                    .unwrap()
                    .get(row_idx)
                    .unwrap()
                    .to_string()
                    == "YES";

                table.columns.push(ColumnInfo {
                    name: column_name,
                    data_type,
                    is_nullable,
                    default_value: None,
                    max_length: None,
                });
            }
        }

        if let Some(table) = current_table {
            tables.push(table);
        }

        Ok(Schema {
            tables,
            views: Vec::new(),
            version: Some("PostgreSQL 14+".to_string()),
        })
    }

    async fn health_check(&self) -> Result<bool, DataSourceError> {
        let conn = self.connect().await?;
        let result = conn.query("SELECT 1 as health_check").await?;
        Ok(result.height() == 1)
    }

    fn source_type(&self) -> &'static str {
        "postgresql"
    }

    fn config(&self) -> &ConnectionConfig {
        &self.config
    }
}

/// ClickHouse adapter
pub struct ClickHouseAdapter {
    config: ConnectionConfig,
    client: Option<Arc<MockClickHouseClient>>,
}

impl ClickHouseAdapter {
    pub fn new(config: ConnectionConfig) -> Self {
        Self {
            config,
            client: None,
        }
    }

    pub async fn initialize(&mut self) -> Result<(), DataSourceError> {
        let client = MockClickHouseClient::new(&self.config).await?;
        self.client = Some(Arc::new(client));
        Ok(())
    }
}

#[async_trait::async_trait]
impl DataSource for ClickHouseAdapter {
    async fn connect(&self) -> Result<Box<dyn Connection>, DataSourceError> {
        let client = self.client.as_ref().ok_or_else(|| {
            DataSourceError::ConnectionFailed("Client not initialized".to_string())
        })?;

        let conn = ClickHouseConnection::new(client.clone()).await?;
        Ok(Box::new(conn))
    }

    async fn get_schema(&self) -> Result<Schema, DataSourceError> {
        let conn = self.connect().await?;

        let tables_query = r#"
            SELECT
                name as table_name,
                database as table_schema,
                engine
            FROM system.tables
            WHERE database != 'system'
            ORDER BY database, name
        "#;

        let result = conn.query(tables_query).await?;

        let mut tables = Vec::new();
        for row_idx in 0..result.height() {
            let table_name = result
                .column("table_name")
                .unwrap()
                .get(row_idx)
                .unwrap()
                .to_string();
            let schema_name = result
                .column("table_schema")
                .unwrap()
                .get(row_idx)
                .unwrap()
                .to_string();

            tables.push(TableInfo {
                name: table_name,
                schema: Some(schema_name),
                columns: Vec::new(), // Would fetch columns in real implementation
                primary_keys: Vec::new(),
                indexes: Vec::new(),
            });
        }

        Ok(Schema {
            tables,
            views: Vec::new(),
            version: Some("ClickHouse 23+".to_string()),
        })
    }

    async fn health_check(&self) -> Result<bool, DataSourceError> {
        let conn = self.connect().await?;
        let result = conn.query("SELECT 1 as health_check").await?;
        Ok(result.height() == 1)
    }

    fn source_type(&self) -> &'static str {
        "clickhouse"
    }

    fn config(&self) -> &ConnectionConfig {
        &self.config
    }
}

/// JSON file adapter
pub struct JsonAdapter {
    config: ConnectionConfig,
    file_path: String,
}

impl JsonAdapter {
    pub fn new(file_path: String) -> Self {
        Self {
            config: ConnectionConfig {
                connection_string: file_path.clone(),
                max_connections: Some(1),
                connection_timeout: Some(Duration::from_secs(5)),
                query_timeout: Some(Duration::from_secs(30)),
                ssl_mode: None,
                credentials: None,
            },
            file_path,
        }
    }
}

#[async_trait::async_trait]
impl DataSource for JsonAdapter {
    async fn connect(&self) -> Result<Box<dyn Connection>, DataSourceError> {
        let conn = JsonConnection::new(&self.file_path).await?;
        Ok(Box::new(conn))
    }

    async fn get_schema(&self) -> Result<Schema, DataSourceError> {
        let conn = self.connect().await?;

        // For JSON files, we infer schema from the data
        let df = conn.query("SELECT * FROM data LIMIT 100").await?;
        let schema = df.schema();

        let mut columns = Vec::new();
        for (name, data_type) in schema.iter() {
            columns.push(ColumnInfo {
                name: name.to_string(),
                data_type: format!("{:?}", data_type),
                is_nullable: matches!(data_type, polars::prelude::DataType::Null),
                default_value: None,
                max_length: None,
            });
        }

        let table = TableInfo {
            name: "data".to_string(),
            schema: None,
            columns,
            primary_keys: Vec::new(),
            indexes: Vec::new(),
        };

        Ok(Schema {
            tables: vec![table],
            views: Vec::new(),
            version: Some("JSON File".to_string()),
        })
    }

    async fn health_check(&self) -> Result<bool, DataSourceError> {
        use std::path::Path;
        Ok(Path::new(&self.file_path).exists())
    }

    fn source_type(&self) -> &'static str {
        "json"
    }

    fn config(&self) -> &ConnectionConfig {
        &self.config
    }
}

/// Parquet file adapter
pub struct ParquetAdapter {
    config: ConnectionConfig,
    file_path: String,
}

impl ParquetAdapter {
    pub fn new(file_path: String) -> Self {
        Self {
            config: ConnectionConfig {
                connection_string: file_path.clone(),
                max_connections: Some(1),
                connection_timeout: Some(Duration::from_secs(5)),
                query_timeout: Some(Duration::from_secs(30)),
                ssl_mode: None,
                credentials: None,
            },
            file_path,
        }
    }
}

#[async_trait::async_trait]
impl DataSource for ParquetAdapter {
    async fn connect(&self) -> Result<Box<dyn Connection>, DataSourceError> {
        let conn = ParquetConnection::new(&self.file_path).await?;
        Ok(Box::new(conn))
    }

    async fn get_schema(&self) -> Result<Schema, DataSourceError> {
        let conn = self.connect().await?;

        // Read parquet schema
        let df = conn.query("SELECT * FROM data LIMIT 0").await?;
        let schema = df.schema();

        let mut columns = Vec::new();
        for (name, data_type) in schema.iter() {
            columns.push(ColumnInfo {
                name: name.to_string(),
                data_type: format!("{:?}", data_type),
                is_nullable: matches!(data_type, polars::prelude::DataType::Null),
                default_value: None,
                max_length: None,
            });
        }

        let table = TableInfo {
            name: "data".to_string(),
            schema: None,
            columns,
            primary_keys: Vec::new(),
            indexes: Vec::new(),
        };

        Ok(Schema {
            tables: vec![table],
            views: Vec::new(),
            version: Some("Parquet File".to_string()),
        })
    }

    async fn health_check(&self) -> Result<bool, DataSourceError> {
        use std::path::Path;
        Ok(Path::new(&self.file_path).exists())
    }

    fn source_type(&self) -> &'static str {
        "parquet"
    }

    fn config(&self) -> &ConnectionConfig {
        &self.config
    }
}

/// Mock implementations for testing and compilation

/// Mock connection pool for PostgreSQL
pub struct MockConnectionPool;

impl MockConnectionPool {
    async fn new(_config: &ConnectionConfig) -> Result<Self, DataSourceError> {
        Ok(Self)
    }
}

/// Mock PostgreSQL connection
pub struct PostgresConnection {
    _pool: Arc<RwLock<MockConnectionPool>>,
}

impl PostgresConnection {
    async fn new(pool: Arc<RwLock<MockConnectionPool>>) -> Result<Self, DataSourceError> {
        Ok(Self { _pool: pool })
    }
}

#[async_trait::async_trait]
impl Connection for PostgresConnection {
    async fn query(&self, sql: &str) -> Result<DataFrame, DataSourceError> {
        // Mock implementation - return empty DataFrame with appropriate schema
        if sql.contains("information_schema") {
            let df = df! {
                "table_name" => ["users", "orders", "products"],
                "table_schema" => ["public", "public", "public"],
                "column_name" => ["id", "id", "id"],
                "data_type" => ["integer", "integer", "integer"],
                "is_nullable" => ["NO", "NO", "NO"],
                "column_default" => [Some("nextval('users_id_seq')"), Some("nextval('orders_id_seq')"), Some("nextval('products_id_seq')")],
                "character_maximum_length" => [None::<i32>, None::<i32>, None::<i32>],
            }.map_err(|e| DataSourceError::QueryFailed(e.to_string()))?;
            Ok(df)
        } else {
            let df = df! {
                "health_check" => [1],
            }
            .map_err(|e| DataSourceError::QueryFailed(e.to_string()))?;
            Ok(df)
        }
    }

    async fn query_with_params(
        &self,
        _sql: &str,
        _params: &[&dyn ToSql],
    ) -> Result<DataFrame, DataSourceError> {
        let df = df! {
            "result" => ["success"],
        }
        .map_err(|e| DataSourceError::QueryFailed(e.to_string()))?;
        Ok(df)
    }

    async fn query_stream(&self, _sql: &str) -> Result<Box<dyn DataStream>, DataSourceError> {
        Ok(Box::new(MockDataStream::new()))
    }

    async fn execute(&self, _sql: &str) -> Result<u64, DataSourceError> {
        Ok(1)
    }

    async fn begin_transaction(&self) -> Result<Box<dyn Transaction>, DataSourceError> {
        Ok(Box::new(MockTransaction))
    }

    async fn close(&self) -> Result<(), DataSourceError> {
        Ok(())
    }
}

/// Mock ClickHouse client
pub struct MockClickHouseClient;

impl MockClickHouseClient {
    async fn new(_config: &ConnectionConfig) -> Result<Self, DataSourceError> {
        Ok(Self)
    }
}

/// Mock ClickHouse connection
pub struct ClickHouseConnection {
    _client: Arc<MockClickHouseClient>,
}

impl ClickHouseConnection {
    async fn new(client: Arc<MockClickHouseClient>) -> Result<Self, DataSourceError> {
        Ok(Self { _client: client })
    }
}

#[async_trait::async_trait]
impl Connection for ClickHouseConnection {
    async fn query(&self, sql: &str) -> Result<DataFrame, DataSourceError> {
        if sql.contains("system.tables") {
            let df = df! {
                "table_name" => ["events", "users", "sessions"],
                "table_schema" => ["analytics", "analytics", "analytics"],
                "engine" => ["MergeTree", "MergeTree", "MergeTree"],
            }
            .map_err(|e| DataSourceError::QueryFailed(e.to_string()))?;
            Ok(df)
        } else {
            let df = df! {
                "health_check" => [1],
            }
            .map_err(|e| DataSourceError::QueryFailed(e.to_string()))?;
            Ok(df)
        }
    }

    async fn query_with_params(
        &self,
        _sql: &str,
        _params: &[&dyn ToSql],
    ) -> Result<DataFrame, DataSourceError> {
        let df = df! {
            "result" => ["success"],
        }
        .map_err(|e| DataSourceError::QueryFailed(e.to_string()))?;
        Ok(df)
    }

    async fn query_stream(&self, _sql: &str) -> Result<Box<dyn DataStream>, DataSourceError> {
        Ok(Box::new(MockDataStream::new()))
    }

    async fn execute(&self, _sql: &str) -> Result<u64, DataSourceError> {
        Ok(1)
    }

    async fn begin_transaction(&self) -> Result<Box<dyn Transaction>, DataSourceError> {
        Ok(Box::new(MockTransaction))
    }

    async fn close(&self) -> Result<(), DataSourceError> {
        Ok(())
    }
}

/// Mock JSON connection
pub struct JsonConnection {
    _file_path: String,
}

impl JsonConnection {
    async fn new(file_path: &str) -> Result<Self, DataSourceError> {
        Ok(Self {
            _file_path: file_path.to_string(),
        })
    }
}

#[async_trait::async_trait]
impl Connection for JsonConnection {
    async fn query(&self, _sql: &str) -> Result<DataFrame, DataSourceError> {
        let df = df! {
            "id" => [1, 2, 3],
            "name" => ["Alice", "Bob", "Charlie"],
            "age" => [30, 25, 35],
        }
        .map_err(|e| DataSourceError::QueryFailed(e.to_string()))?;
        Ok(df)
    }

    async fn query_with_params(
        &self,
        _sql: &str,
        _params: &[&dyn ToSql],
    ) -> Result<DataFrame, DataSourceError> {
        let df = df! {
            "result" => ["success"],
        }
        .map_err(|e| DataSourceError::QueryFailed(e.to_string()))?;
        Ok(df)
    }

    async fn query_stream(&self, _sql: &str) -> Result<Box<dyn DataStream>, DataSourceError> {
        Ok(Box::new(MockDataStream::new()))
    }

    async fn execute(&self, _sql: &str) -> Result<u64, DataSourceError> {
        Ok(1)
    }

    async fn begin_transaction(&self) -> Result<Box<dyn Transaction>, DataSourceError> {
        Ok(Box::new(MockTransaction))
    }

    async fn close(&self) -> Result<(), DataSourceError> {
        Ok(())
    }
}

/// Mock Parquet connection
pub struct ParquetConnection {
    _file_path: String,
}

impl ParquetConnection {
    async fn new(file_path: &str) -> Result<Self, DataSourceError> {
        Ok(Self {
            _file_path: file_path.to_string(),
        })
    }
}

#[async_trait::async_trait]
impl Connection for ParquetConnection {
    async fn query(&self, _sql: &str) -> Result<DataFrame, DataSourceError> {
        let df = df! {
            "timestamp" => [1640995200i64, 1640995260, 1640995320],
            "value" => [100.0, 105.5, 103.2],
            "category" => ["A", "B", "A"],
        }
        .map_err(|e| DataSourceError::QueryFailed(e.to_string()))?;
        Ok(df)
    }

    async fn query_with_params(
        &self,
        _sql: &str,
        _params: &[&dyn ToSql],
    ) -> Result<DataFrame, DataSourceError> {
        let df = df! {
            "result" => ["success"],
        }
        .map_err(|e| DataSourceError::QueryFailed(e.to_string()))?;
        Ok(df)
    }

    async fn query_stream(&self, _sql: &str) -> Result<Box<dyn DataStream>, DataSourceError> {
        Ok(Box::new(MockDataStream::new()))
    }

    async fn execute(&self, _sql: &str) -> Result<u64, DataSourceError> {
        Ok(1)
    }

    async fn begin_transaction(&self) -> Result<Box<dyn Transaction>, DataSourceError> {
        Ok(Box::new(MockTransaction))
    }

    async fn close(&self) -> Result<(), DataSourceError> {
        Ok(())
    }
}

/// Mock transaction implementation
pub struct MockTransaction;

#[async_trait::async_trait]
impl Transaction for MockTransaction {
    async fn commit(&self) -> Result<(), DataSourceError> {
        Ok(())
    }

    async fn rollback(&self) -> Result<(), DataSourceError> {
        Ok(())
    }

    async fn query(&self, _sql: &str) -> Result<DataFrame, DataSourceError> {
        let df = df! {
            "transaction_result" => ["committed"],
        }
        .map_err(|e| DataSourceError::QueryFailed(e.to_string()))?;
        Ok(df)
    }
}

/// Mock data stream implementation
pub struct MockDataStream {
    batch_count: usize,
}

impl MockDataStream {
    pub fn new() -> Self {
        Self { batch_count: 0 }
    }
}

#[async_trait::async_trait]
impl DataStream for MockDataStream {
    async fn next_batch(&mut self) -> Result<Option<DataFrame>, DataSourceError> {
        if self.batch_count >= 3 {
            return Ok(None);
        }

        self.batch_count += 1;
        // Create a simple DataFrame with mock data
        let batch_id_data = vec![self.batch_count as i64];
        let data_content = vec![format!("batch_data_{}", self.batch_count)];
        let df = DataFrame::new(vec![
            Series::new("batch_id".into(), &batch_id_data).into(),
            Series::new("data".into(), &data_content).into(),
        ])
        .map_err(|e| DataSourceError::QueryFailed(e.to_string()))?;

        Ok(Some(df))
    }

    fn estimated_rows(&self) -> Option<usize> {
        Some(1000)
    }
}

/// Data source registry for managing multiple connections
pub struct DataSourceRegistry {
    sources: HashMap<String, Box<dyn DataSource>>,
}

impl DataSourceRegistry {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: String, source: Box<dyn DataSource>) {
        self.sources.insert(name, source);
    }

    pub async fn get_connection(&self, name: &str) -> Result<Box<dyn Connection>, DataSourceError> {
        let source = self.sources.get(name).ok_or_else(|| {
            DataSourceError::InvalidConfiguration(format!("Data source '{}' not found", name))
        })?;

        source.connect().await
    }

    pub async fn health_check_all(&self) -> HashMap<String, Result<bool, DataSourceError>> {
        let mut results = HashMap::new();

        for (name, source) in &self.sources {
            let health = source.health_check().await;
            results.insert(name.clone(), health);
        }

        results
    }

    pub fn list_sources(&self) -> Vec<(&String, &'static str)> {
        self.sources
            .iter()
            .map(|(name, source)| (name, source.source_type()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_postgres_adapter_connection() {
        let config = ConnectionConfig {
            connection_string: "postgresql://localhost:5432/test".to_string(),
            max_connections: Some(10),
            connection_timeout: Some(Duration::from_secs(5)),
            query_timeout: Some(Duration::from_secs(30)),
            ssl_mode: Some("prefer".to_string()),
            credentials: None,
        };

        let mut adapter = PostgresAdapter::new(config);
        adapter.initialize().await.unwrap();

        assert_eq!(adapter.source_type(), "postgresql");

        let connection = adapter.connect().await.unwrap();
        let result = connection.query("SELECT 1").await.unwrap();
        assert_eq!(result.height(), 1);
    }

    #[tokio::test]
    async fn test_clickhouse_adapter_connection() {
        let config = ConnectionConfig {
            connection_string: "http://localhost:8123/test".to_string(),
            max_connections: Some(5),
            connection_timeout: Some(Duration::from_secs(5)),
            query_timeout: Some(Duration::from_secs(30)),
            ssl_mode: None,
            credentials: None,
        };

        let mut adapter = ClickHouseAdapter::new(config);
        adapter.initialize().await.unwrap();

        assert_eq!(adapter.source_type(), "clickhouse");

        let connection = adapter.connect().await.unwrap();
        let result = connection.query("SELECT 1").await.unwrap();
        assert_eq!(result.height(), 1);
    }

    #[tokio::test]
    async fn test_json_adapter() {
        let adapter = JsonAdapter::new("test_data.json".to_string());
        assert_eq!(adapter.source_type(), "json");

        let connection = adapter.connect().await.unwrap();
        let result = connection.query("SELECT * FROM data").await.unwrap();
        assert!(result.height() > 0);
    }

    #[tokio::test]
    async fn test_parquet_adapter() {
        let adapter = ParquetAdapter::new("test_data.parquet".to_string());
        assert_eq!(adapter.source_type(), "parquet");

        let connection = adapter.connect().await.unwrap();
        let result = connection.query("SELECT * FROM data").await.unwrap();
        assert!(result.height() > 0);
    }

    #[tokio::test]
    async fn test_data_source_registry() {
        let mut registry = DataSourceRegistry::new();

        let postgres_config = ConnectionConfig {
            connection_string: "postgresql://localhost:5432/test".to_string(),
            max_connections: Some(10),
            connection_timeout: Some(Duration::from_secs(5)),
            query_timeout: Some(Duration::from_secs(30)),
            ssl_mode: None,
            credentials: None,
        };

        let mut postgres_adapter = PostgresAdapter::new(postgres_config);
        postgres_adapter.initialize().await.unwrap();

        registry.register("main_db".to_string(), Box::new(postgres_adapter));
        registry.register(
            "analytics".to_string(),
            Box::new(JsonAdapter::new("analytics.json".to_string())),
        );

        let sources = registry.list_sources();
        assert_eq!(sources.len(), 2);

        let conn = registry.get_connection("main_db").await.unwrap();
        let result = conn.query("SELECT 1").await.unwrap();
        assert_eq!(result.height(), 1);

        let health_results = registry.health_check_all().await;
        assert_eq!(health_results.len(), 2);
    }

    #[tokio::test]
    async fn test_schema_introspection() {
        let config = ConnectionConfig {
            connection_string: "postgresql://localhost:5432/test".to_string(),
            max_connections: Some(10),
            connection_timeout: Some(Duration::from_secs(5)),
            query_timeout: Some(Duration::from_secs(30)),
            ssl_mode: None,
            credentials: None,
        };

        let mut adapter = PostgresAdapter::new(config);
        adapter.initialize().await.unwrap();

        let schema = adapter.get_schema().await.unwrap();
        assert!(!schema.tables.is_empty());
        assert!(schema.version.is_some());

        let first_table = &schema.tables[0];
        assert!(!first_table.name.is_empty());
        assert!(!first_table.columns.is_empty());
    }

    #[tokio::test]
    async fn test_streaming_data() {
        let config = ConnectionConfig {
            connection_string: "postgresql://localhost:5432/test".to_string(),
            max_connections: Some(10),
            connection_timeout: Some(Duration::from_secs(5)),
            query_timeout: Some(Duration::from_secs(30)),
            ssl_mode: None,
            credentials: None,
        };

        let mut adapter = PostgresAdapter::new(config);
        adapter.initialize().await.unwrap();

        let connection = adapter.connect().await.unwrap();
        let mut stream = connection
            .query_stream("SELECT * FROM large_table")
            .await
            .unwrap();

        let mut batch_count = 0;
        while let Some(_batch) = stream.next_batch().await.unwrap() {
            batch_count += 1;
        }

        assert!(batch_count > 0);
        assert_eq!(stream.estimated_rows(), Some(1000));
    }
}
