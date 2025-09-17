//! Chart Builder System
//!
//! This module provides the chart builder pattern for creating chart specifications.

use super::{
    ChartConfig, ChartSpec, DataReference, DataType, Encoding, EncodingDef, Intelligence, MarkType,
    Selection, Transform,
};
use crate::chart::validation::ValidationError;

/// Chart builder for creating chart specifications
pub struct ChartBuilder {
    data: Option<DataReference>,
    mark: Option<MarkType>,
    encoding: Option<Encoding>,
    transforms: Vec<Transform>,
    selections: Vec<Selection>,
    intelligence: Option<Intelligence>,
    config: ChartConfig,
}

impl Default for ChartBuilder {
    fn default() -> Self {
        Self {
            data: None,
            mark: None,
            encoding: None,
            transforms: Vec::new(),
            selections: Vec::new(),
            intelligence: None,
            config: ChartConfig::default(),
        }
    }
}

impl ChartBuilder {
    /// Create a new chart builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the data source
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
        self.transforms.push(transform);
        self
    }

    /// Add a selection
    pub fn selection(mut self, selection: Selection) -> Self {
        self.selections.push(selection);
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
            transform: self.transforms,
            selection: self.selections,
            intelligence: self.intelligence,
            config: self.config,
        };

        spec.validate()?;
        Ok(spec)
    }
}

/// Encoding builder for creating encodings
pub struct EncodingBuilder {
    x: Option<EncodingDef>,
    y: Option<EncodingDef>,
    color: Option<EncodingDef>,
    size: Option<EncodingDef>,
    shape: Option<EncodingDef>,
    opacity: Option<EncodingDef>,
    text: Option<EncodingDef>,
    tooltip: Option<EncodingDef>,
    detail: Option<EncodingDef>,
    order: Option<EncodingDef>,
    row: Option<EncodingDef>,
    column: Option<EncodingDef>,
}

impl Default for EncodingBuilder {
    fn default() -> Self {
        Self {
            x: None,
            y: None,
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

impl EncodingBuilder {
    /// Create a new encoding builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set x encoding
    pub fn x(mut self, field: &str, data_type: DataType) -> Self {
        self.x = Some(EncodingDef {
            field: field.to_string(),
            data_type,
            scale: None,
            axis: None,
            legend: None,
            bin: None,
            aggregate: None,
            sort: None,
        });
        self
    }

    /// Set y encoding
    pub fn y(mut self, field: &str, data_type: DataType) -> Self {
        self.y = Some(EncodingDef {
            field: field.to_string(),
            data_type,
            scale: None,
            axis: None,
            legend: None,
            bin: None,
            aggregate: None,
            sort: None,
        });
        self
    }

    /// Set color encoding
    pub fn color(mut self, field: &str, data_type: DataType) -> Self {
        self.color = Some(EncodingDef {
            field: field.to_string(),
            data_type,
            scale: None,
            axis: None,
            legend: None,
            bin: None,
            aggregate: None,
            sort: None,
        });
        self
    }

    /// Set size encoding
    pub fn size(mut self, field: &str, data_type: DataType) -> Self {
        self.size = Some(EncodingDef {
            field: field.to_string(),
            data_type,
            scale: None,
            axis: None,
            legend: None,
            bin: None,
            aggregate: None,
            sort: None,
        });
        self
    }

    /// Set shape encoding
    pub fn shape(mut self, field: &str, data_type: DataType) -> Self {
        self.shape = Some(EncodingDef {
            field: field.to_string(),
            data_type,
            scale: None,
            axis: None,
            legend: None,
            bin: None,
            aggregate: None,
            sort: None,
        });
        self
    }

    /// Set opacity encoding
    pub fn opacity(mut self, field: &str, data_type: DataType) -> Self {
        self.opacity = Some(EncodingDef {
            field: field.to_string(),
            data_type,
            scale: None,
            axis: None,
            legend: None,
            bin: None,
            aggregate: None,
            sort: None,
        });
        self
    }

    /// Set text encoding
    pub fn text(mut self, field: &str, data_type: DataType) -> Self {
        self.text = Some(EncodingDef {
            field: field.to_string(),
            data_type,
            scale: None,
            axis: None,
            legend: None,
            bin: None,
            aggregate: None,
            sort: None,
        });
        self
    }

    /// Set tooltip encoding
    pub fn tooltip(mut self, field: &str, data_type: DataType) -> Self {
        self.tooltip = Some(EncodingDef {
            field: field.to_string(),
            data_type,
            scale: None,
            axis: None,
            legend: None,
            bin: None,
            aggregate: None,
            sort: None,
        });
        self
    }

    /// Set detail encoding
    pub fn detail(mut self, field: &str, data_type: DataType) -> Self {
        self.detail = Some(EncodingDef {
            field: field.to_string(),
            data_type,
            scale: None,
            axis: None,
            legend: None,
            bin: None,
            aggregate: None,
            sort: None,
        });
        self
    }

    /// Set order encoding
    pub fn order(mut self, field: &str, data_type: DataType) -> Self {
        self.order = Some(EncodingDef {
            field: field.to_string(),
            data_type,
            scale: None,
            axis: None,
            legend: None,
            bin: None,
            aggregate: None,
            sort: None,
        });
        self
    }

    /// Set row encoding
    pub fn row(mut self, field: &str, data_type: DataType) -> Self {
        self.row = Some(EncodingDef {
            field: field.to_string(),
            data_type,
            scale: None,
            axis: None,
            legend: None,
            bin: None,
            aggregate: None,
            sort: None,
        });
        self
    }

    /// Set column encoding
    pub fn column(mut self, field: &str, data_type: DataType) -> Self {
        self.column = Some(EncodingDef {
            field: field.to_string(),
            data_type,
            scale: None,
            axis: None,
            legend: None,
            bin: None,
            aggregate: None,
            sort: None,
        });
        self
    }

    /// Build the encoding
    pub fn build(self) -> Encoding {
        Encoding {
            x: self.x,
            y: self.y,
            color: self.color,
            size: self.size,
            shape: self.shape,
            opacity: self.opacity,
            text: self.text,
            tooltip: self.tooltip,
            detail: self.detail,
            order: self.order,
            row: self.row,
            column: self.column,
        }
    }
}

/// Chart builder utilities
pub struct ChartBuilderUtils;

impl ChartBuilderUtils {
    /// Create a simple line chart
    pub fn line_chart(
        data: DataReference,
        x_field: &str,
        y_field: &str,
    ) -> Result<ChartSpec, ValidationError> {
        ChartBuilder::new()
            .data(data)
            .mark(MarkType::Line {
                interpolate: None,
                stroke_width: None,
                stroke_dash: None,
            })
            .encoding(
                EncodingBuilder::new()
                    .x(x_field, DataType::String)
                    .y(y_field, DataType::Number)
                    .build(),
            )
            .build()
    }

    /// Create a simple bar chart
    pub fn bar_chart(
        data: DataReference,
        x_field: &str,
        y_field: &str,
    ) -> Result<ChartSpec, ValidationError> {
        ChartBuilder::new()
            .data(data)
            .mark(MarkType::Bar {
                width: None,
                corner_radius: None,
            })
            .encoding(
                EncodingBuilder::new()
                    .x(x_field, DataType::String)
                    .y(y_field, DataType::Number)
                    .build(),
            )
            .build()
    }

    /// Create a simple scatter plot
    pub fn scatter_plot(
        data: DataReference,
        x_field: &str,
        y_field: &str,
    ) -> Result<ChartSpec, ValidationError> {
        ChartBuilder::new()
            .data(data)
            .mark(MarkType::Point {
                size: None,
                shape: None,
                opacity: None,
            })
            .encoding(
                EncodingBuilder::new()
                    .x(x_field, DataType::Number)
                    .y(y_field, DataType::Number)
                    .build(),
            )
            .build()
    }

    /// Create a simple area chart
    pub fn area_chart(
        data: DataReference,
        x_field: &str,
        y_field: &str,
    ) -> Result<ChartSpec, ValidationError> {
        ChartBuilder::new()
            .data(data)
            .mark(MarkType::Area {
                interpolate: None,
                opacity: None,
            })
            .encoding(
                EncodingBuilder::new()
                    .x(x_field, DataType::String)
                    .y(y_field, DataType::Number)
                    .build(),
            )
            .build()
    }
}
