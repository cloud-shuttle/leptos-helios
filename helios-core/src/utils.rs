//! Utility functions and helpers

use std::collections::HashMap;

/// Utility functions for data conversion and validation
pub mod conversion {
    use super::*;
    
    /// Convert JavaScript data to Polars DataFrame
    pub fn js_data_to_dataframe(js_data: &serde_json::Value) -> Result<crate::DataFrame, crate::DataError> {
        use polars::prelude::*;
        
        if let Some(array) = js_data.as_array() {
            if array.is_empty() {
                return Ok(DataFrame::empty());
            }
            
            let mut columns = HashMap::new();
            
            // Extract columns from first object
            if let Some(first_obj) = array[0].as_object() {
                for key in first_obj.keys() {
                    columns.insert(key.clone(), Vec::new());
                }
            }
            
            // Collect values for each column
            for item in array {
                if let Some(obj) = item.as_object() {
                    for (key, value) in obj {
                        if let Some(column_vec) = columns.get_mut(key) {
                            column_vec.push(value.clone());
                        }
                    }
                }
            }
            
            // Convert to Polars Series
            let series: Vec<Series> = columns
                .into_iter()
                .map(|(name, values)| {
                    // Convert JSON values to appropriate Polars types
                    let polars_values: Vec<_> = values.iter().map(|v| {
                        match v {
                            serde_json::Value::Number(n) => {
                                if n.is_f64() {
                                    polars::prelude::AnyValue::Float64(n.as_f64().unwrap())
                                } else {
                                    polars::prelude::AnyValue::Int64(n.as_i64().unwrap())
                                }
                            },
                            serde_json::Value::String(s) => polars::prelude::AnyValue::String(s),
                            serde_json::Value::Bool(b) => polars::prelude::AnyValue::Boolean(*b),
                            serde_json::Value::Null => polars::prelude::AnyValue::Null,
                            _ => polars::prelude::AnyValue::String(Box::leak(v.to_string().into_boxed_str())),
                        }
                    }).collect();
                    
                    Series::new((&name).into(), polars_values)
                })
                .collect();
            
            Ok(DataFrame::new(series.into_iter().map(|s| s.into()).collect())?)
        } else {
            Err(crate::DataError::Format("Expected JSON array".to_string()))
        }
    }
    
    /// Convert Polars DataFrame to JavaScript-compatible format
    pub fn dataframe_to_js(df: &crate::DataFrame) -> Result<serde_json::Value, crate::DataError> {
        let mut result = Vec::new();
        
        for row in 0..df.height() {
            let mut obj = serde_json::Map::new();
            
            for col_name in df.get_column_names() {
                let series = df.column(col_name)?;
                let value = series.get(row).map_err(|e| crate::DataError::Processing(e.to_string()))?;
                
                let json_value = match value {
                    polars::prelude::AnyValue::Int32(i) => serde_json::Value::Number(i.into()),
                    polars::prelude::AnyValue::Int64(i) => serde_json::Value::Number(i.into()),
                    polars::prelude::AnyValue::Float32(f) => serde_json::Value::Number(serde_json::Number::from_f64(f as f64).unwrap()),
                    polars::prelude::AnyValue::Float64(f) => serde_json::Value::Number(serde_json::Number::from_f64(f).unwrap()),
                    polars::prelude::AnyValue::String(s) => serde_json::Value::String(s.to_string()),
                    polars::prelude::AnyValue::Boolean(b) => serde_json::Value::Bool(b),
                    polars::prelude::AnyValue::Null => serde_json::Value::Null,
                    _ => serde_json::Value::String(value.to_string()),
                };
                
                obj.insert(col_name.to_string(), json_value);
            }
            
            result.push(serde_json::Value::Object(obj));
        }
        
        Ok(serde_json::Value::Array(result))
    }
}

/// Performance measurement utilities
pub mod performance {
    use std::time::{Duration, Instant};
    
    /// Measure execution time of a function
    pub fn measure_time<F, R>(f: F) -> (R, Duration)
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        (result, duration)
    }
    
    /// Benchmark a function multiple times
    pub fn benchmark<F, R>(f: F, iterations: usize) -> BenchmarkResult
    where
        F: Fn() -> R,
    {
        let mut times = Vec::with_capacity(iterations);
        
        for _ in 0..iterations {
            let (_, duration) = measure_time(&f);
            times.push(duration);
        }
        
        times.sort();
        
        BenchmarkResult {
            min: times[0],
            max: times[iterations - 1],
            mean: times.iter().sum::<Duration>() / iterations as u32,
            median: times[iterations / 2],
            p95: times[(iterations as f64 * 0.95) as usize],
            p99: times[(iterations as f64 * 0.99) as usize],
        }
    }
    
    #[derive(Debug, Clone)]
    pub struct BenchmarkResult {
        pub min: Duration,
        pub max: Duration,
        pub mean: Duration,
        pub median: Duration,
        pub p95: Duration,
        pub p99: Duration,
    }
}

/// Memory utilities
pub mod memory {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    /// Global memory tracker
    pub struct MemoryTracker {
        allocated: AtomicUsize,
        peak: AtomicUsize,
    }
    
    impl MemoryTracker {
        pub const fn new() -> Self {
            Self {
                allocated: AtomicUsize::new(0),
                peak: AtomicUsize::new(0),
            }
        }
        
        pub fn allocated(&self) -> usize {
            self.allocated.load(Ordering::Relaxed)
        }
        
        pub fn peak(&self) -> usize {
            self.peak.load(Ordering::Relaxed)
        }
        
        pub fn record_allocation(&self, size: usize) {
            let current = self.allocated.fetch_add(size, Ordering::Relaxed);
            let new_total = current + size;
            
            let mut peak = self.peak.load(Ordering::Relaxed);
            while peak < new_total {
                match self.peak.compare_exchange_weak(
                    peak,
                    new_total,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => break,
                    Err(p) => peak = p,
                }
            }
        }
        
        pub fn record_deallocation(&self, size: usize) {
            self.allocated.fetch_sub(size, Ordering::Relaxed);
        }
    }
    
    pub static MEMORY_TRACKER: MemoryTracker = MemoryTracker::new();
}

/// Validation utilities
pub mod validation {
    use crate::chart::{ChartSpec, ValidationError};
    
    /// Validate chart specification
    pub fn validate_chart_spec(spec: &ChartSpec) -> Result<(), ValidationError> {
        spec.validate()
    }
    
    /// Validate data types
    pub fn validate_data_types(spec: &ChartSpec) -> Result<(), ValidationError> {
        // Check that encoding data types match actual data
        if let crate::chart::DataReference::DataFrame(df) = &spec.data {
            for (field, encoding) in spec.encoding.get_field_encodings() {
                if let Some(series) = df.column(&field).ok() {
                    let actual_type = series.dtype();
                    let expected_type = encoding.data_type();
                    
                    if !types_compatible(actual_type, expected_type) {
                        return Err(ValidationError::DataValidation(format!(
                            "Type mismatch for field '{}': expected {:?}, got {:?}",
                            field, expected_type, actual_type
                        )));
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn types_compatible(actual: &polars::prelude::DataType, expected: &crate::chart::DataType) -> bool {
        match (actual, expected) {
            (polars::prelude::DataType::Int32 | polars::prelude::DataType::Int64, crate::chart::DataType::Quantitative) => true,
            (polars::prelude::DataType::Float32 | polars::prelude::DataType::Float64, crate::chart::DataType::Quantitative) => true,
            (polars::prelude::DataType::String, crate::chart::DataType::Nominal | crate::chart::DataType::Ordinal) => true,
            (polars::prelude::DataType::Boolean, crate::chart::DataType::Nominal) => true,
            (polars::prelude::DataType::Date | polars::prelude::DataType::Datetime(_, _), crate::chart::DataType::Temporal) => true,
            _ => false,
        }
    }
}

/// Error handling utilities
pub mod error {
    use crate::HeliosError;
    
    /// Convert error to user-friendly message
    pub fn user_friendly_error(error: &HeliosError) -> String {
        error.user_message()
    }
    
    /// Get suggested actions for an error
    pub fn suggested_actions(error: &HeliosError) -> Vec<String> {
        error.suggested_actions()
    }
    
    /// Check if error is recoverable
    pub fn is_recoverable(error: &HeliosError) -> bool {
        error.is_recoverable()
    }
}

/// Testing utilities
pub mod test_utils {
    use super::*;
    use polars::prelude::*;
    
    /// Create test DataFrame
    pub fn create_test_dataframe() -> DataFrame {
        df! {
            "x" => [1, 2, 3, 4, 5],
            "y" => [2, 4, 1, 5, 3],
            "category" => ["A", "B", "A", "C", "B"],
        }.unwrap()
    }
    
    /// Create large test DataFrame
    pub fn create_large_test_dataframe(size: usize) -> DataFrame {
        let x: Vec<i32> = (0..size as i32).collect();
        let y: Vec<f64> = (0..size).map(|i| (i as f64).sin()).collect();
        let category: Vec<String> = (0..size).map(|i| format!("Category_{}", i % 10)).collect();
        
        df! {
            "x" => x,
            "y" => y,
            "category" => category,
        }.unwrap()
    }
    
    /// Create test chart specification
    pub fn create_test_chart_spec() -> crate::chart::ChartSpec {
        crate::chart::ChartSpec {
            data: crate::chart::DataReference::DataFrame(create_test_dataframe()),
            mark: crate::chart::MarkType::Point { 
                size: Some(5.0), 
                shape: None, 
                opacity: None 
            },
            encoding: crate::chart::Encoding {
                x: Some(crate::chart::PositionEncoding {
                    field: "x".to_string(),
                    data_type: crate::chart::DataType::Quantitative,
                    scale: None,
                    axis: None,
                    bin: None,
                    sort: None,
                }),
                y: Some(crate::chart::PositionEncoding {
                    field: "y".to_string(),
                    data_type: crate::chart::DataType::Quantitative,
                    scale: None,
                    axis: None,
                    bin: None,
                    sort: None,
                }),
                color: Some(crate::chart::ColorEncoding {
                    field: Some("category".to_string()),
                    data_type: Some(crate::chart::DataType::Nominal),
                    scale: None,
                    condition: None,
                }),
                ..Default::default()
            },
            transform: Vec::new(),
            selection: Vec::new(),
            intelligence: None,
            config: crate::chart::ChartConfig::default(),
        }
    }
    
    /// Assert two DataFrames are approximately equal
    pub fn assert_dataframes_approx_equal(df1: &DataFrame, df2: &DataFrame, tolerance: f64) {
        assert_eq!(df1.height(), df2.height(), "DataFrame heights differ");
        assert_eq!(df1.width(), df2.width(), "DataFrame widths differ");
        
        for col_name in df1.get_column_names() {
            let series1 = df1.column(col_name).unwrap();
            let series2 = df2.column(col_name).unwrap();
            
            assert_eq!(series1.dtype(), series2.dtype(), "Column '{}' types differ", col_name);
            
            for i in 0..series1.len() {
                let val1 = series1.get(i).unwrap();
                let val2 = series2.get(i).unwrap();
                
                match (val1.clone(), val2.clone()) {
                    (AnyValue::Float64(f1), AnyValue::Float64(f2)) => {
                        assert!((f1 - f2).abs() < tolerance, 
                            "Column '{}' row {}: {} != {} (tolerance: {})", 
                            col_name, i, f1, f2, tolerance);
                    },
                    (AnyValue::Float32(f1), AnyValue::Float32(f2)) => {
                        assert!((f1 - f2).abs() < tolerance as f32, 
                            "Column '{}' row {}: {} != {} (tolerance: {})", 
                            col_name, i, f1, f2, tolerance);
                    },
                    _ => assert_eq!(val1, val2, "Column '{}' row {} values differ", col_name, i),
                }
            }
        }
    }
}

// Add extension trait for Encoding to get field encodings
impl crate::chart::Encoding {
    pub fn get_field_encodings(&self) -> Vec<(String, &crate::chart::PositionEncoding)> {
        let mut encodings = Vec::new();
        
        if let Some(ref x) = self.x {
            encodings.push((x.field.clone(), x));
        }
        if let Some(ref y) = self.y {
            encodings.push((y.field.clone(), y));
        }
        
        encodings
    }
}

// Add extension trait for PositionEncoding to get data type
impl crate::chart::PositionEncoding {
    pub fn data_type(&self) -> &crate::chart::DataType {
        &self.data_type
    }
}
