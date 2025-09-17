//! Chart Specification System
//!
//! This module provides the core chart specification functionality.

use super::{
    ChartConfig, ChartSpec, DataReference, DataType, Encoding, EncodingDef, Intelligence, MarkType,
    Selection, Transform,
};
use crate::chart::validation::ValidationError;

/// Chart specification builder
pub struct ChartSpecBuilder {
    data: Option<DataReference>,
    mark: Option<MarkType>,
    encoding: Option<Encoding>,
    transform: Vec<Transform>,
    selection: Vec<Selection>,
    intelligence: Option<Intelligence>,
    config: ChartConfig,
}

impl Default for ChartSpecBuilder {
    fn default() -> Self {
        Self {
            data: None,
            mark: None,
            encoding: None,
            transform: Vec::new(),
            selection: Vec::new(),
            intelligence: None,
            config: ChartConfig::default(),
        }
    }
}

impl ChartSpecBuilder {
    /// Create a new chart spec builder
    pub fn new() -> Self {
        Self::default()
    }
}

impl ChartSpecBuilder {
    /// Set the data reference
    pub fn data(mut self, data: DataReference) -> Self {
        self.data = Some(data);
        self
    }

    /// Set the mark type
    pub fn mark(mut self, mark: MarkType) -> Self {
        self.mark = Some(mark);
        self
    }

    /// Set the encoding
    pub fn encoding(mut self, encoding: Encoding) -> Self {
        self.encoding = Some(encoding);
        self
    }

    /// Add a transform
    pub fn transform(mut self, transform: Transform) -> Self {
        self.transform.push(transform);
        self
    }

    /// Add a selection
    pub fn selection(mut self, selection: Selection) -> Self {
        self.selection.push(selection);
        self
    }

    /// Set intelligence features
    pub fn intelligence(mut self, intelligence: Intelligence) -> Self {
        self.intelligence = Some(intelligence);
        self
    }

    /// Set chart configuration
    pub fn config(mut self, config: ChartConfig) -> Self {
        self.config = config;
        self
    }

    /// Build the chart specification
    pub fn build(self) -> Result<ChartSpec, ValidationError> {
        let spec = ChartSpec {
            data: self
                .data
                .ok_or_else(|| ValidationError::MissingField("data".to_string()))?,
            mark: self
                .mark
                .ok_or_else(|| ValidationError::MissingField("mark".to_string()))?,
            encoding: self
                .encoding
                .ok_or_else(|| ValidationError::MissingField("encoding".to_string()))?,
            transform: self.transform,
            selection: self.selection,
            intelligence: self.intelligence,
            config: self.config,
        };

        spec.validate()?;
        Ok(spec)
    }
}

/// Chart specification utilities
pub struct ChartSpecUtils;

impl ChartSpecUtils {
    /// Create a simple line chart specification
    pub fn line_chart(data: DataReference, x_field: &str, y_field: &str) -> ChartSpec {
        ChartSpecBuilder::default()
            .data(data)
            .mark(MarkType::Line {
                interpolate: None,
                stroke_width: None,
                stroke_dash: None,
            })
            .encoding(Encoding {
                x: Some(EncodingDef {
                    field: x_field.to_string(),
                    data_type: DataType::String,
                    scale: None,
                    axis: None,
                    legend: None,
                    bin: None,
                    aggregate: None,
                    sort: None,
                }),
                y: Some(EncodingDef {
                    field: y_field.to_string(),
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
            .build()
            .expect("Failed to build line chart")
    }

    /// Create a simple bar chart specification
    pub fn bar_chart(data: DataReference, x_field: &str, y_field: &str) -> ChartSpec {
        ChartSpecBuilder::default()
            .data(data)
            .mark(MarkType::Bar {
                width: None,
                corner_radius: None,
            })
            .encoding(Encoding {
                x: Some(EncodingDef {
                    field: x_field.to_string(),
                    data_type: DataType::String,
                    scale: None,
                    axis: None,
                    legend: None,
                    bin: None,
                    aggregate: None,
                    sort: None,
                }),
                y: Some(EncodingDef {
                    field: y_field.to_string(),
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
            .build()
            .expect("Failed to build bar chart")
    }

    /// Create a simple scatter plot specification
    pub fn scatter_plot(data: DataReference, x_field: &str, y_field: &str) -> ChartSpec {
        ChartSpecBuilder::default()
            .data(data)
            .mark(MarkType::Point {
                size: None,
                shape: None,
                opacity: None,
            })
            .encoding(Encoding {
                x: Some(EncodingDef {
                    field: x_field.to_string(),
                    data_type: DataType::Number,
                    scale: None,
                    axis: None,
                    legend: None,
                    bin: None,
                    aggregate: None,
                    sort: None,
                }),
                y: Some(EncodingDef {
                    field: y_field.to_string(),
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
            .build()
            .expect("Failed to build scatter plot")
    }
}
