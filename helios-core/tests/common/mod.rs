//! Common test utilities and fixtures
//!
//! This module provides shared utilities for testing across the test suite,
//! including mock data, sample configurations, and helper functions.

use leptos_helios::*;
use polars::prelude::*;
use std::collections::HashMap;

/// Create a mock DataFrame for testing
pub fn mock_dataframe() -> DataFrame {
    DataFrame::new(vec![
        Series::new("x", &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
        Series::new("y", &[10, 20, 15, 25, 30, 35, 40, 45, 50, 55]),
        Series::new("category", &["A", "B", "A", "C", "B", "A", "C", "B", "A", "C"]),
        Series::new("date", &[
            "2023-01-01", "2023-01-02", "2023-01-03", "2023-01-04", "2023-01-05",
            "2023-01-06", "2023-01-07", "2023-01-08", "2023-01-09", "2023-01-10"
        ]),
    ]).unwrap()
}

/// Create a large mock DataFrame for performance testing
pub fn mock_large_dataframe(size: usize) -> DataFrame {
    let x_values: Vec<i32> = (1..=size as i32).collect();
    let y_values: Vec<i32> = (1..=size as i32).map(|i| i * 2).collect();
    let categories: Vec<String> = (1..=size)
        .map(|i| format!("Category_{}", i % 10))
        .collect();
    
    DataFrame::new(vec![
        Series::new("x", x_values),
        Series::new("y", y_values),
        Series::new("category", categories),
    ]).unwrap()
}

/// Create a sample chart specification
pub fn sample_chart_spec() -> ChartSpec {
    ChartSpec::new()
        .with_mark(MarkType::Line)
        .with_title("Sample Chart")
        .with_description("A sample chart for testing")
        .with_width(800)
        .with_height(600)
        .with_encoding(Encoding {
            x: Some(EncodingDef {
                field: "x".to_string(),
                data_type: DataType::Number,
                scale: None,
                axis: None,
                legend: None,
                bin: None,
                aggregate: None,
                sort: None,
            }),
            y: Some(EncodingDef {
                field: "y".to_string(),
                data_type: DataType::Number,
                scale: None,
                axis: None,
                legend: None,
                bin: None,
                aggregate: None,
                sort: None,
            }),
            color: Some(EncodingDef {
                field: "category".to_string(),
                data_type: DataType::String,
                scale: None,
                axis: None,
                legend: None,
                bin: None,
                aggregate: None,
                sort: None,
            }),
            size: None,
            shape: None,
            opacity: None,
            text: None,
            tooltip: None,
            detail: None,
            order: None,
            row: None,
            column: None,
        })
}

/// Create a bar chart specification
pub fn sample_bar_chart_spec() -> ChartSpec {
    ChartSpec::new()
        .with_mark(MarkType::Bar { width: None, corner_radius: None })
        .with_title("Sample Bar Chart")
        .with_encoding(Encoding {
            x: Some(EncodingDef {
                field: "category".to_string(),
                data_type: DataType::String,
                scale: None,
                axis: None,
                legend: None,
                bin: None,
                aggregate: None,
                sort: None,
            }),
            y: Some(EncodingDef {
                field: "y".to_string(),
                data_type: DataType::Number,
                scale: None,
                axis: None,
                legend: None,
                bin: None,
                aggregate: None,
                sort: None,
            }),
            color: None,
            size: None,
            shape: None,
            opacity: None,
            text: None,
            tooltip: None,
            detail: None,
            order: None,
            row: None,
            column: None,
        })
}

/// Create a scatter plot specification
pub fn sample_scatter_plot_spec() -> ChartSpec {
    ChartSpec::new()
        .with_mark(MarkType::Point)
        .with_title("Sample Scatter Plot")
        .with_encoding(Encoding {
            x: Some(EncodingDef {
                field: "x".to_string(),
                data_type: DataType::Number,
                scale: None,
                axis: None,
                legend: None,
                bin: None,
                aggregate: None,
                sort: None,
            }),
            y: Some(EncodingDef {
                field: "y".to_string(),
                data_type: DataType::Number,
                scale: None,
                axis: None,
                legend: None,
                bin: None,
                aggregate: None,
                sort: None,
            }),
            color: Some(EncodingDef {
                field: "category".to_string(),
                data_type: DataType::String,
                scale: None,
                axis: None,
                legend: None,
                bin: None,
                aggregate: None,
                sort: None,
            }),
            size: None,
            shape: None,
            opacity: None,
            text: None,
            tooltip: None,
            detail: None,
            order: None,
            row: None,
            column: None,
        })
}

/// Create a test renderer (mock implementation)
pub fn test_renderer() -> Box<dyn Renderer> {
    // For now, return a mock renderer
    // This would be replaced with actual renderer implementations in real tests
    Box::new(MockRenderer::new())
}

/// Mock renderer for testing
pub struct MockRenderer {
    name: String,
}

impl MockRenderer {
    pub fn new() -> Self {
        Self {
            name: "MockRenderer".to_string(),
        }
    }
}

impl Renderer for MockRenderer {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn render(&mut self, _spec: &ChartSpec, _data: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
        // Mock implementation - always succeeds
        Ok(())
    }
    
    fn supports_format(&self, _format: &str) -> bool {
        true
    }
}

/// Create a sample export configuration
pub fn sample_export_config() -> ExportConfig {
    ExportConfig {
        format: ExportFormat::PNG {
            width: 800,
            height: 600,
            dpi: Some(96),
        },
        quality: Some(90),
        background_color: Some("#ffffff".to_string()),
        include_metadata: true,
        custom_styles: None,
        template_variables: None,
        title: Some("Test Export".to_string()),
        description: Some("Test export for unit testing".to_string()),
        author: Some("Test Suite".to_string()),
    }
}

/// Create a sample user for security testing
pub fn sample_user() -> User {
    User::new(
        "test_user_1".to_string(),
        "testuser".to_string(),
        "test@example.com".to_string(),
        "Test User".to_string(),
    )
}

/// Create sample credentials for authentication testing
pub fn sample_credentials() -> Credentials {
    Credentials::username_password(
        "testuser".to_string(),
        "testpassword".to_string(),
    )
}

/// Create a sample NL processor configuration
pub fn sample_nl_config() -> NLConfig {
    NLConfig::default()
}

/// Assert that two DataFrames have the same schema
pub fn assert_same_schema(df1: &DataFrame, df2: &DataFrame) {
    assert_eq!(df1.schema(), df2.schema(), "DataFrames should have the same schema");
}

/// Assert that a DataFrame has expected columns
pub fn assert_has_columns(df: &DataFrame, expected_columns: &[&str]) {
    let actual_columns: Vec<&str> = df.get_column_names().iter().map(|s| s.as_str()).collect();
    for expected_col in expected_columns {
        assert!(
            actual_columns.contains(expected_col),
            "DataFrame should contain column '{}', but has columns: {:?}",
            expected_col,
            actual_columns
        );
    }
}

/// Create a test chart with data
pub fn create_test_chart() -> (ChartSpec, DataFrame) {
    let spec = sample_chart_spec();
    let data = mock_dataframe();
    (spec, data)
}

/// Measure execution time of a function
pub fn measure_time<F, R>(f: F) -> (R, std::time::Duration)
where
    F: FnOnce() -> R,
{
    let start = std::time::Instant::now();
    let result = f();
    let duration = start.elapsed();
    (result, duration)
}

/// Assert that execution time is within acceptable limits
pub fn assert_execution_time_acceptable<F>(f: F, max_duration: std::time::Duration)
where
    F: FnOnce(),
{
    let (_, duration) = measure_time(f);
    assert!(
        duration <= max_duration,
        "Execution time {:?} exceeded maximum allowed duration {:?}",
        duration,
        max_duration
    );
}

/// Create a mock data source for testing
pub fn mock_data_source() -> MockDataSource {
    MockDataSource::new()
}

/// Mock data source for testing
pub struct MockDataSource {
    data: DataFrame,
}

impl MockDataSource {
    pub fn new() -> Self {
        Self {
            data: mock_dataframe(),
        }
    }
    
    pub fn get_data(&self) -> &DataFrame {
        &self.data
    }
}

impl DataSource for MockDataSource {
    fn name(&self) -> &str {
        "MockDataSource"
    }
    
    fn load_data(&self) -> Result<DataFrame, Box<dyn std::error::Error>> {
        Ok(self.data.clone())
    }
    
    fn supports_query(&self, _query: &str) -> bool {
        true
    }
}