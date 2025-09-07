//! TDD Implementation: Data Processing Pipeline for Helios v1.0
//!
//! RED-GREEN-REFACTOR cycle for high-performance data processing
//! Target: 500K rows processed in <100ms

use std::collections::HashMap;
use std::time::{Duration, Instant};

// =============================================================================
// Data Structures for TDD
// =============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
    pub category: Option<String>,
    pub timestamp: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct Dataset {
    pub points: Vec<DataPoint>,
    pub metadata: DatasetMetadata,
}

#[derive(Debug, Clone)]
pub struct DatasetMetadata {
    pub name: String,
    pub size: usize,
    pub schema: Vec<ColumnInfo>,
}

#[derive(Debug, Clone)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: DataType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Float64,
    String,
    Timestamp,
}

#[derive(Debug, Clone)]
pub struct ProcessingResult {
    pub processed_count: usize,
    pub processing_time: Duration,
    pub aggregations: HashMap<String, f64>,
    pub success: bool,
}

#[derive(Debug, Clone)]
pub enum ProcessingOperation {
    Filter(FilterCondition),
    Aggregate(AggregateOperation),
    Sort(SortOperation),
    GroupBy(String),
}

#[derive(Debug, Clone)]
pub struct FilterCondition {
    pub column: String,
    pub operator: ComparisonOperator,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonOperator {
    GreaterThan,
    LessThan,
    Equal,
}

#[derive(Debug, Clone)]
pub struct AggregateOperation {
    pub column: String,
    pub function: AggregateFunction,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AggregateFunction {
    Sum,
    Average,
    Count,
    Max,
    Min,
}

#[derive(Debug, Clone)]
pub struct SortOperation {
    pub column: String,
    pub ascending: bool,
}

// =============================================================================
// TDD Implementation: Data Processing Pipeline
// =============================================================================

pub struct DataPipeline {
    name: String,
    operations: Vec<ProcessingOperation>,
}

impl DataPipeline {
    /// Create new data processing pipeline
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            operations: Vec::new(),
        }
    }

    /// Add processing operation to pipeline
    pub fn add_operation(&mut self, operation: ProcessingOperation) {
        self.operations.push(operation);
    }

    /// Process dataset through pipeline
    pub fn process(&self, dataset: &Dataset) -> Result<ProcessingResult, String> {
        let start = Instant::now();

        // Clone data for processing
        let mut working_data = dataset.points.clone();
        let mut aggregations = HashMap::new();

        // Apply each operation in sequence
        for operation in &self.operations {
            working_data = self.apply_operation(working_data, operation, &mut aggregations)?;
        }

        let processing_time = start.elapsed();

        Ok(ProcessingResult {
            processed_count: working_data.len(),
            processing_time,
            aggregations,
            success: true,
        })
    }

    /// Apply single operation to data
    fn apply_operation(
        &self,
        data: Vec<DataPoint>,
        operation: &ProcessingOperation,
        aggregations: &mut HashMap<String, f64>,
    ) -> Result<Vec<DataPoint>, String> {
        match operation {
            ProcessingOperation::Filter(filter) => self.apply_filter(data, filter),
            ProcessingOperation::Aggregate(agg) => {
                let result = self.apply_aggregation(&data, agg)?;
                aggregations.insert(format!("{}_{:?}", agg.column, agg.function), result);
                Ok(data) // Return original data, store aggregation result
            }
            ProcessingOperation::Sort(sort) => self.apply_sort(data, sort),
            ProcessingOperation::GroupBy(_column) => {
                // Simplified group by - just return data for now
                Ok(data)
            }
        }
    }

    /// Apply filter operation
    fn apply_filter(
        &self,
        data: Vec<DataPoint>,
        filter: &FilterCondition,
    ) -> Result<Vec<DataPoint>, String> {
        let filtered: Vec<DataPoint> = data
            .into_iter()
            .filter(|point| {
                let value = match filter.column.as_str() {
                    "x" => point.x,
                    "y" => point.y,
                    _ => return false,
                };

                match filter.operator {
                    ComparisonOperator::GreaterThan => value > filter.value,
                    ComparisonOperator::LessThan => value < filter.value,
                    ComparisonOperator::Equal => (value - filter.value).abs() < f64::EPSILON,
                }
            })
            .collect();

        Ok(filtered)
    }

    /// Apply aggregation operation
    fn apply_aggregation(
        &self,
        data: &[DataPoint],
        agg: &AggregateOperation,
    ) -> Result<f64, String> {
        if data.is_empty() {
            return Ok(0.0);
        }

        let values: Vec<f64> = data
            .iter()
            .map(|point| match agg.column.as_str() {
                "x" => point.x,
                "y" => point.y,
                _ => 0.0,
            })
            .collect();

        let result = match agg.function {
            AggregateFunction::Sum => values.iter().sum(),
            AggregateFunction::Average => values.iter().sum::<f64>() / values.len() as f64,
            AggregateFunction::Count => values.len() as f64,
            AggregateFunction::Max => values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)),
            AggregateFunction::Min => values.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
        };

        Ok(result)
    }

    /// Apply sort operation
    fn apply_sort(
        &self,
        mut data: Vec<DataPoint>,
        sort: &SortOperation,
    ) -> Result<Vec<DataPoint>, String> {
        data.sort_by(|a, b| {
            let val_a = match sort.column.as_str() {
                "x" => a.x,
                "y" => a.y,
                _ => 0.0,
            };
            let val_b = match sort.column.as_str() {
                "x" => b.x,
                "y" => b.y,
                _ => 0.0,
            };

            if sort.ascending {
                val_a
                    .partial_cmp(&val_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            } else {
                val_b
                    .partial_cmp(&val_a)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }
        });

        Ok(data)
    }

    /// Get pipeline name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get operation count
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }
}

#[cfg(test)]
mod data_pipeline_tdd {
    use super::*;

    // =============================================================================
    // RED PHASE: Write failing tests first
    // =============================================================================

    #[test]
    fn test_data_pipeline_creation() {
        // RED: Pipeline should be creatable
        let pipeline = DataPipeline::new("test_pipeline");

        assert_eq!(pipeline.name(), "test_pipeline");
        assert_eq!(
            pipeline.operation_count(),
            0,
            "New pipeline should have no operations"
        );
    }

    #[test]
    fn test_add_operations_to_pipeline() {
        // RED: Should be able to add operations
        let mut pipeline = DataPipeline::new("filter_pipeline");

        let filter_op = ProcessingOperation::Filter(FilterCondition {
            column: "x".to_string(),
            operator: ComparisonOperator::GreaterThan,
            value: 5.0,
        });

        pipeline.add_operation(filter_op);

        assert_eq!(
            pipeline.operation_count(),
            1,
            "Pipeline should have 1 operation after adding"
        );
    }

    #[test]
    fn test_filter_operation() {
        // RED: Filter should work on dataset
        let dataset = create_test_dataset(100);
        let mut pipeline = DataPipeline::new("filter_test");

        // Add filter: x > 50
        pipeline.add_operation(ProcessingOperation::Filter(FilterCondition {
            column: "x".to_string(),
            operator: ComparisonOperator::GreaterThan,
            value: 50.0,
        }));

        let result = pipeline.process(&dataset);

        assert!(result.is_ok(), "Filter processing should succeed");

        let result = result.unwrap();
        assert!(result.success, "Processing should be successful");
        assert!(
            result.processed_count < dataset.points.len(),
            "Filter should reduce data size"
        );

        println!(
            "Filtered {} points to {} points",
            dataset.points.len(),
            result.processed_count
        );
    }

    #[test]
    fn test_aggregation_operation() {
        // RED: Aggregation should work correctly
        let dataset = create_test_dataset(1000);
        let mut pipeline = DataPipeline::new("aggregation_test");

        // Add aggregation: SUM of y values
        pipeline.add_operation(ProcessingOperation::Aggregate(AggregateOperation {
            column: "y".to_string(),
            function: AggregateFunction::Sum,
        }));

        let result = pipeline.process(&dataset);

        assert!(result.is_ok(), "Aggregation processing should succeed");

        let result = result.unwrap();
        assert!(
            result.aggregations.contains_key("y_Sum"),
            "Should contain sum aggregation"
        );

        let sum_value = result.aggregations.get("y_Sum").unwrap();
        assert!(*sum_value > 0.0, "Sum should be positive");

        println!("Sum of y values: {}", sum_value);
    }

    #[test]
    fn test_performance_requirement_500k_rows() {
        // RED: Performance test for 500K rows in <100ms
        let dataset = create_performance_test_dataset(500_000);
        let mut pipeline = DataPipeline::new("performance_test");

        // Add multiple operations to test pipeline performance
        pipeline.add_operation(ProcessingOperation::Filter(FilterCondition {
            column: "x".to_string(),
            operator: ComparisonOperator::GreaterThan,
            value: 100_000.0,
        }));

        pipeline.add_operation(ProcessingOperation::Aggregate(AggregateOperation {
            column: "y".to_string(),
            function: AggregateFunction::Average,
        }));

        let start = Instant::now();
        let result = pipeline.process(&dataset);
        let total_time = start.elapsed();

        assert!(result.is_ok(), "Performance test should succeed");

        let result = result.unwrap();
        assert!(result.success, "Processing should be successful");

        // Performance requirement: 500K rows in <100ms
        assert!(
            total_time < Duration::from_millis(100),
            "Processing 500K rows took {:?}, expected <100ms",
            total_time
        );

        println!(
            "GREEN: Processed {} rows in {:?} (pipeline time: {:?})",
            dataset.points.len(),
            total_time,
            result.processing_time
        );
    }

    #[test]
    fn test_sort_operation() {
        // RED: Sort should work correctly
        let mut dataset = create_test_dataset(10);
        // Shuffle the data to make sorting meaningful
        dataset.points.reverse();

        let mut pipeline = DataPipeline::new("sort_test");

        pipeline.add_operation(ProcessingOperation::Sort(SortOperation {
            column: "x".to_string(),
            ascending: true,
        }));

        let result = pipeline.process(&dataset);

        assert!(result.is_ok(), "Sort processing should succeed");
        assert_eq!(result.unwrap().processed_count, dataset.points.len());
    }

    #[test]
    fn test_complex_pipeline() {
        // RED: Complex pipeline with multiple operations
        let dataset = create_test_dataset(10_000);
        let mut pipeline = DataPipeline::new("complex_pipeline");

        // Filter -> Aggregate -> Sort
        pipeline.add_operation(ProcessingOperation::Filter(FilterCondition {
            column: "y".to_string(),
            operator: ComparisonOperator::GreaterThan,
            value: 50.0,
        }));

        pipeline.add_operation(ProcessingOperation::Aggregate(AggregateOperation {
            column: "x".to_string(),
            function: AggregateFunction::Max,
        }));

        pipeline.add_operation(ProcessingOperation::Sort(SortOperation {
            column: "y".to_string(),
            ascending: false,
        }));

        let result = pipeline.process(&dataset);

        assert!(result.is_ok(), "Complex pipeline should succeed");

        let result = result.unwrap();
        assert!(result.success, "Complex processing should be successful");
        assert!(
            result.aggregations.contains_key("x_Max"),
            "Should contain max aggregation"
        );

        println!(
            "Complex pipeline processed {} -> {} points with {} aggregations",
            dataset.points.len(),
            result.processed_count,
            result.aggregations.len()
        );
    }
}

// =============================================================================
// GREEN PHASE: Helper functions for testing
// =============================================================================

/// Create test dataset with specified number of points
fn create_test_dataset(count: usize) -> Dataset {
    let points: Vec<DataPoint> = (0..count)
        .map(|i| DataPoint {
            x: i as f64,
            y: (i as f64).sin() * 100.0 + 100.0, // Ensure positive values for sum test
            category: Some(format!("category_{}", i % 5)),
            timestamp: Some(1000 + i as u64),
        })
        .collect();

    let metadata = DatasetMetadata {
        name: "test_dataset".to_string(),
        size: count,
        schema: vec![
            ColumnInfo {
                name: "x".to_string(),
                data_type: DataType::Float64,
            },
            ColumnInfo {
                name: "y".to_string(),
                data_type: DataType::Float64,
            },
            ColumnInfo {
                name: "category".to_string(),
                data_type: DataType::String,
            },
            ColumnInfo {
                name: "timestamp".to_string(),
                data_type: DataType::Timestamp,
            },
        ],
    };

    Dataset { points, metadata }
}

/// Create large dataset for performance testing
fn create_performance_test_dataset(count: usize) -> Dataset {
    let points: Vec<DataPoint> = (0..count)
        .map(|i| DataPoint {
            x: i as f64,
            y: ((i as f64) / 1000.0).cos() * 500.0,
            category: Some(format!("perf_cat_{}", i % 10)),
            timestamp: Some(2000 + i as u64),
        })
        .collect();

    let metadata = DatasetMetadata {
        name: "performance_dataset".to_string(),
        size: count,
        schema: vec![
            ColumnInfo {
                name: "x".to_string(),
                data_type: DataType::Float64,
            },
            ColumnInfo {
                name: "y".to_string(),
                data_type: DataType::Float64,
            },
        ],
    };

    Dataset { points, metadata }
}

#[cfg(test)]
mod tdd_validation {
    use super::*;

    #[test]
    fn validate_data_structures() {
        let point = DataPoint {
            x: 1.0,
            y: 2.0,
            category: Some("test".to_string()),
            timestamp: Some(1234),
        };

        assert_eq!(point.x, 1.0);
        assert_eq!(point.category, Some("test".to_string()));
    }

    #[test]
    fn validate_test_dataset_creation() {
        let dataset = create_test_dataset(5);
        assert_eq!(dataset.points.len(), 5);
        assert_eq!(dataset.metadata.size, 5);
        assert_eq!(dataset.metadata.schema.len(), 4);
    }
}
