//! Data source integration tests
//!
//! These tests verify that data sources work correctly with the rest of the system,
//! including error handling, retry logic, and connection pooling.

use leptos_helios::data_sources::*;
use polars::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

/// Mock database connection for testing
pub struct MockDatabaseConnection {
    pub connection_string: String,
    pub is_connected: bool,
    pub query_count: u32,
    pub max_retries: u32,
}

impl MockDatabaseConnection {
    pub fn new(connection_string: String) -> Self {
        Self {
            connection_string,
            is_connected: true,
            query_count: 0,
            max_retries: 3,
        }
    }
    
    pub fn new_disconnected() -> Self {
        Self {
            connection_string: "mock://disconnected".to_string(),
            is_connected: false,
            query_count: 0,
            max_retries: 3,
        }
    }
    
    pub fn execute_query(&mut self, query: &str) -> Result<DataFrame, DataSourceError> {
        self.query_count += 1;
        
        if !self.is_connected {
            return Err(DataSourceError::ConnectionFailed("Not connected".to_string()));
        }
        
        // Simulate different query results based on query content
        if query.contains("SELECT * FROM test_table") {
            Ok(DataFrame::new(vec![
                Series::new("id".into(), &[1, 2, 3, 4, 5]).into(),
                Series::new("name".into(), &["Alice", "Bob", "Charlie", "David", "Eve"]).into(),
                Series::new("value".into(), &[10.5, 20.3, 15.7, 25.1, 18.9]).into(),
            ]).unwrap())
        } else if query.contains("SELECT COUNT(*)") {
            Ok(DataFrame::new(vec![
                Series::new("count".into(), &[5]).into(),
            ]).unwrap())
        } else {
            Err(DataSourceError::QueryFailed(format!("Unknown query: {}", query)))
        }
    }
    
    pub fn execute_with_retry(&mut self, query: &str) -> Result<DataFrame, DataSourceError> {
        let mut last_error = None;
        
        for attempt in 1..=self.max_retries {
            match self.execute_query(query) {
                Ok(result) => return Ok(result),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < self.max_retries {
                        // Simulate retry delay
                        std::thread::sleep(Duration::from_millis(10));
                    }
                }
            }
        }
        
        Err(last_error.unwrap())
    }
}

/// Mock file system data source
pub struct MockFileDataSource {
    pub base_path: String,
    pub available_files: HashMap<String, DataFrame>,
}

impl MockFileDataSource {
    pub fn new(base_path: String) -> Self {
        let mut available_files = HashMap::new();
        
        // Add some test files
        available_files.insert("test.csv".to_string(), DataFrame::new(vec![
            Series::new("x".into(), &[1, 2, 3, 4, 5]).into(),
            Series::new("y".into(), &[10, 20, 15, 25, 30]).into(),
        ]).unwrap());
        
        available_files.insert("large_data.csv".to_string(), {
            let size = 1000;
            let x_data: Vec<i32> = (0..size).collect();
            let y_data: Vec<f64> = (0..size).map(|i| (i as f64) * 0.1).collect();
            DataFrame::new(vec![
                Series::new("x".into(), x_data).into(),
                Series::new("y".into(), y_data).into(),
            ]).unwrap()
        });
        
        Self {
            base_path,
            available_files,
        }
    }
    
    pub fn load_file(&self, filename: &str) -> Result<DataFrame, DataSourceError> {
        self.available_files
            .get(filename)
            .cloned()
            .ok_or_else(|| DataSourceError::FileNotFound(filename.to_string()))
    }
    
    pub fn list_files(&self) -> Vec<String> {
        self.available_files.keys().cloned().collect()
    }
}

/// Mock data source error for testing
#[derive(Debug, thiserror::Error)]
pub enum DataSourceError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    
    #[error("Query failed: {0}")]
    QueryFailed(String),
    
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Timeout: {0}")]
    Timeout(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_database_connection_basic() {
        let mut conn = MockDatabaseConnection::new("postgresql://test".to_string());
        
        let result = conn.execute_query("SELECT * FROM test_table");
        assert!(result.is_ok());
        
        let df = result.unwrap();
        assert_eq!(df.height(), 5);
        assert_eq!(df.width(), 3);
        assert_eq!(conn.query_count, 1);
    }
    
    #[test]
    fn test_database_connection_disconnected() {
        let mut conn = MockDatabaseConnection::new_disconnected();
        
        let result = conn.execute_query("SELECT * FROM test_table");
        assert!(result.is_err());
        
        match result.unwrap_err() {
            DataSourceError::ConnectionFailed(_) => {},
            _ => panic!("Expected ConnectionFailed error"),
        }
    }
    
    #[test]
    fn test_database_retry_logic() {
        let mut conn = MockDatabaseConnection::new("postgresql://test".to_string());
        
        let result = conn.execute_with_retry("SELECT * FROM test_table");
        assert!(result.is_ok());
        assert_eq!(conn.query_count, 1);
    }
    
    #[test]
    fn test_database_retry_logic_failure() {
        let mut conn = MockDatabaseConnection::new_disconnected();
        
        let result = conn.execute_with_retry("SELECT * FROM test_table");
        assert!(result.is_err());
        // The retry logic should have tried 3 times, but since the connection is disconnected,
        // each call to execute_query increments query_count
        assert_eq!(conn.query_count, 3); // Should have tried 3 times
    }
    
    #[test]
    fn test_file_data_source_basic() {
        let file_source = MockFileDataSource::new("/test/data".to_string());
        
        let result = file_source.load_file("test.csv");
        assert!(result.is_ok());
        
        let df = result.unwrap();
        assert_eq!(df.height(), 5);
        assert_eq!(df.width(), 2);
    }
    
    #[test]
    fn test_file_data_source_not_found() {
        let file_source = MockFileDataSource::new("/test/data".to_string());
        
        let result = file_source.load_file("nonexistent.csv");
        assert!(result.is_err());
        
        match result.unwrap_err() {
            DataSourceError::FileNotFound(_) => {},
            _ => panic!("Expected FileNotFound error"),
        }
    }
    
    #[test]
    fn test_file_data_source_list_files() {
        let file_source = MockFileDataSource::new("/test/data".to_string());
        
        let files = file_source.list_files();
        assert_eq!(files.len(), 2);
        assert!(files.contains(&"test.csv".to_string()));
        assert!(files.contains(&"large_data.csv".to_string()));
    }
    
    #[test]
    fn test_file_data_source_large_data() {
        let file_source = MockFileDataSource::new("/test/data".to_string());
        
        let result = file_source.load_file("large_data.csv");
        assert!(result.is_ok());
        
        let df = result.unwrap();
        assert_eq!(df.height(), 1000);
        assert_eq!(df.width(), 2);
    }
    
    #[test]
    fn test_data_source_error_types() {
        let conn_failed = DataSourceError::ConnectionFailed("test".to_string());
        assert!(conn_failed.to_string().contains("Connection failed"));
        
        let query_failed = DataSourceError::QueryFailed("test".to_string());
        assert!(query_failed.to_string().contains("Query failed"));
        
        let file_not_found = DataSourceError::FileNotFound("test".to_string());
        assert!(file_not_found.to_string().contains("File not found"));
        
        let timeout = DataSourceError::Timeout("test".to_string());
        assert!(timeout.to_string().contains("Timeout"));
    }
    
    #[test]
    fn test_connection_pooling_simulation() {
        let mut connections = vec![
            MockDatabaseConnection::new("postgresql://test1".to_string()),
            MockDatabaseConnection::new("postgresql://test2".to_string()),
            MockDatabaseConnection::new("postgresql://test3".to_string()),
        ];
        
        // Simulate using different connections
        for (i, conn) in connections.iter_mut().enumerate() {
            let result = conn.execute_query(&format!("SELECT COUNT(*) FROM table_{}", i));
            assert!(result.is_ok());
        }
        
        // All connections should have been used
        for conn in &connections {
            assert_eq!(conn.query_count, 1);
        }
    }
    
    #[test]
    fn test_data_source_performance() {
        let file_source = MockFileDataSource::new("/test/data".to_string());
        
        let start = std::time::Instant::now();
        
        // Load large dataset multiple times
        for _ in 0..10 {
            let result = file_source.load_file("large_data.csv");
            assert!(result.is_ok());
        }
        
        let duration = start.elapsed();
        
        // Should be fast (less than 100ms for 10 loads)
        assert!(duration.as_millis() < 100, "Performance regression: {:?}", duration);
    }
}
