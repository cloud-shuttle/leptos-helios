//! Data analysis and visualization suggestions

use super::types::{DataAnalysis, IntelligenceConfig};
use crate::chart::{ChartSpec, MarkType, Encoding, EncodingDef, DataType};
use polars::prelude::*;
use std::collections::HashMap;

/// Data analyzer for generating visualization suggestions
pub struct DataAnalyzer {
    config: IntelligenceConfig,
}

impl DataAnalyzer {
    /// Create a new data analyzer
    pub fn new(config: IntelligenceConfig) -> Self {
        Self { config }
    }

    /// Analyze data characteristics and suggest visualizations
    pub fn analyze_data_characteristics(&self, data: &DataFrame) -> DataAnalysis {
        let mut column_types = HashMap::new();
        let mut suggested_charts = Vec::new();
        let mut characteristics = Vec::new();

        // Analyze each column
        for (name, dtype) in data.schema().iter() {
            let type_str = match dtype {
                polars::prelude::DataType::Date => "date",
                polars::prelude::DataType::Datetime(_, _) => "datetime",
                polars::prelude::DataType::Int32 | polars::prelude::DataType::Int64 | polars::prelude::DataType::Float32 | polars::prelude::DataType::Float64 => "numeric",
                polars::prelude::DataType::String | polars::prelude::DataType::Categorical(_, _) => "categorical",
                polars::prelude::DataType::Boolean => "boolean",
                _ => "other",
            };
            column_types.insert(name.to_string(), type_str.to_string());
        }

        // Determine data characteristics
        if self.has_date_column(&column_types) {
            characteristics.push("time_series".to_string());
            suggested_charts.push(MarkType::Line { interpolate: None, stroke_width: None, stroke_dash: None });
        }

        if self.has_numeric_columns(&column_types) && self.has_categorical_columns(&column_types) {
            characteristics.push("comparative".to_string());
            suggested_charts.push(MarkType::Bar { width: None, corner_radius: None });
        }

        if self.count_numeric_columns(&column_types) >= 2 {
            characteristics.push("correlational".to_string());
            suggested_charts.push(MarkType::Point { size: None, shape: None, opacity: None });
        }

        if self.has_categorical_columns(&column_types) {
            characteristics.push("categorical".to_string());
            suggested_charts.push(MarkType::Bar { width: None, corner_radius: None });
        }

        DataAnalysis {
            column_types,
            suggested_charts,
            characteristics,
        }
    }

    /// Create time series visualization suggestion
    pub fn create_time_series_suggestion(&self, schema: &Schema) -> ChartSpec {
        let mut spec = ChartSpec::new();
        
        // Find date column
        let date_col = schema.iter()
            .find(|(_, dtype)| matches!(dtype, polars::prelude::DataType::Date | polars::prelude::DataType::Datetime(_, _)))
            .map(|(name, _)| name.to_string())
            .unwrap_or_else(|| schema.iter().next().map(|(name, _)| name.to_string()).unwrap_or_else(|| "date".to_string()));

        // Find numeric column
        let numeric_col = schema.iter()
            .find(|(_, dtype)| matches!(dtype, polars::prelude::DataType::Int32 | polars::prelude::DataType::Int64 | polars::prelude::DataType::Float32 | polars::prelude::DataType::Float64))
            .map(|(name, _)| name.to_string())
            .unwrap_or_else(|| schema.iter().nth(1).map(|(name, _)| name.to_string()).unwrap_or_else(|| "value".to_string()));

        spec.mark = MarkType::Line { interpolate: None, stroke_width: None, stroke_dash: None };
        spec.encoding = Encoding {
            x: Some(EncodingDef {
                field: date_col,
                data_type: DataType::Date,
                scale: None,
                axis: None,
                legend: None,
                bin: None,
                aggregate: None,
                sort: None,
            }),
            y: Some(EncodingDef {
                field: numeric_col,
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
        };

        spec
    }

    /// Create categorical comparison suggestion
    pub fn create_categorical_comparison_suggestion(&self, schema: &Schema) -> ChartSpec {
        let mut spec = ChartSpec::new();
        
        // Find categorical column
        let cat_col = schema.iter()
            .find(|(_, dtype)| matches!(dtype, polars::prelude::DataType::String | polars::prelude::DataType::Categorical(_, _)))
            .map(|(name, _)| name.to_string())
            .unwrap_or_else(|| schema.iter().next().map(|(name, _)| name.to_string()).unwrap_or_else(|| "category".to_string()));

        // Find numeric column
        let numeric_col = schema.iter()
            .find(|(_, dtype)| matches!(dtype, polars::prelude::DataType::Int32 | polars::prelude::DataType::Int64 | polars::prelude::DataType::Float32 | polars::prelude::DataType::Float64))
            .map(|(name, _)| name.to_string())
            .unwrap_or_else(|| schema.iter().nth(1).map(|(name, _)| name.to_string()).unwrap_or_else(|| "value".to_string()));

        spec.mark = MarkType::Bar { width: None, corner_radius: None };
        spec.encoding = self.create_encoding(&cat_col, &numeric_col, DataType::String, DataType::Number);

        spec
    }

    /// Create scatter plot suggestion
    pub fn create_scatter_plot_suggestion(&self, schema: &Schema) -> ChartSpec {
        let mut spec = ChartSpec::new();
        
        // Find two numeric columns
        let numeric_cols: Vec<String> = schema.iter()
            .filter(|(_, dtype)| matches!(dtype, polars::prelude::DataType::Int32 | polars::prelude::DataType::Int64 | polars::prelude::DataType::Float32 | polars::prelude::DataType::Float64))
            .map(|(name, _)| name.to_string())
            .collect();

        if numeric_cols.len() >= 2 {
            spec.mark = MarkType::Point { size: None, shape: None, opacity: None };
            spec.encoding = self.create_encoding(&numeric_cols[0], &numeric_cols[1], DataType::Number, DataType::Number);
        }

        spec
    }

    /// Create treemap suggestion
    pub fn create_treemap_suggestion(&self, schema: &Schema) -> ChartSpec {
        let mut spec = ChartSpec::new();
        
        // Find categorical column for hierarchy
        let cat_col = schema.iter()
            .find(|(_, dtype)| matches!(dtype, polars::prelude::DataType::String | polars::prelude::DataType::Categorical(_, _)))
            .map(|(name, _)| name.to_string())
            .unwrap_or_else(|| schema.iter().next().map(|(name, _)| name.to_string()).unwrap_or_else(|| "category".to_string()));

        // Find numeric column for size
        let numeric_col = schema.iter()
            .find(|(_, dtype)| matches!(dtype, polars::prelude::DataType::Int32 | polars::prelude::DataType::Int64 | polars::prelude::DataType::Float32 | polars::prelude::DataType::Float64))
            .map(|(name, _)| name.to_string())
            .unwrap_or_else(|| schema.iter().nth(1).map(|(name, _)| name.to_string()).unwrap_or_else(|| "value".to_string()));

        spec.mark = MarkType::Rect { stroke: None, stroke_width: None };
        spec.encoding = self.create_encoding(&cat_col, &numeric_col, DataType::String, DataType::Number);

        spec
    }

    /// Check if data has date columns
    fn has_date_column(&self, column_types: &HashMap<String, String>) -> bool {
        column_types.values().any(|t| t == "date" || t == "datetime")
    }

    /// Check if data has numeric columns
    fn has_numeric_columns(&self, column_types: &HashMap<String, String>) -> bool {
        column_types.values().any(|t| t == "numeric")
    }

    /// Check if data has categorical columns
    fn has_categorical_columns(&self, column_types: &HashMap<String, String>) -> bool {
        column_types.values().any(|t| t == "categorical")
    }

    /// Count numeric columns
    fn count_numeric_columns(&self, column_types: &HashMap<String, String>) -> usize {
        column_types.values().filter(|t| t == &"numeric").count()
    }

    /// Helper function to create encoding
    fn create_encoding(&self, x_field: &str, y_field: &str, x_type: DataType, y_type: DataType) -> Encoding {
        Encoding {
            x: Some(EncodingDef {
                field: x_field.to_string(),
                data_type: x_type,
                scale: None,
                axis: None,
                legend: None,
                bin: None,
                aggregate: None,
                sort: None,
            }),
            y: Some(EncodingDef {
                field: y_field.to_string(),
                data_type: y_type,
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
        }
    }
}
