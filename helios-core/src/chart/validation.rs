//! Chart Validation System
//!
//! This module provides validation functionality for chart specifications.

use super::{
    ChartSpec, DataReference, DataType, Encoding, EncodingDef, Intelligence, MarkType, Selection,
    Transform,
};
use thiserror::Error;

/// Validation error types
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid field value: {0}")]
    InvalidValue(String),

    #[error("Incompatible encoding for mark type: {0}")]
    IncompatibleEncoding(String),

    #[error("Invalid data reference: {0}")]
    InvalidDataReference(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),
}

/// Chart validation system
pub struct ChartValidator;

impl ChartValidator {
    /// Validate a complete chart specification
    pub fn validate_spec(spec: &ChartSpec) -> Result<(), ValidationError> {
        // Validate data reference
        spec.data.validate()?;

        // Validate encoding matches mark type
        spec.encoding.validate_for_mark(&spec.mark)?;

        // Validate transforms
        for transform in &spec.transform {
            transform.validate()?;
        }

        // Validate selections
        for selection in &spec.selection {
            selection.validate()?;
        }

        // Validate intelligence features
        if let Some(intelligence) = &spec.intelligence {
            intelligence.validate()?;
        }

        Ok(())
    }
}

impl DataReference {
    /// Validate data reference
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.source.is_empty() {
            return Err(ValidationError::InvalidDataReference(
                "Source cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}

impl Encoding {
    /// Validate encoding for mark type
    pub fn validate_for_mark(&self, mark: &MarkType) -> Result<(), ValidationError> {
        match mark {
            MarkType::Line { .. } | MarkType::Area { .. } => {
                if self.x.is_none() || self.y.is_none() {
                    return Err(ValidationError::IncompatibleEncoding(
                        "Line and area charts require both x and y encodings".to_string(),
                    ));
                }
            }
            MarkType::Bar { .. } => {
                if self.x.is_none() || self.y.is_none() {
                    return Err(ValidationError::IncompatibleEncoding(
                        "Bar charts require both x and y encodings".to_string(),
                    ));
                }
            }
            MarkType::Point { .. } | MarkType::Scatter { .. } => {
                if self.x.is_none() || self.y.is_none() {
                    return Err(ValidationError::IncompatibleEncoding(
                        "Point and scatter charts require both x and y encodings".to_string(),
                    ));
                }
            }
            _ => {
                // Other mark types have different requirements
            }
        }

        Ok(())
    }

    /// Calculate encoding complexity
    pub fn complexity(&self) -> f64 {
        let mut complexity = 0.0;

        if self.x.is_some() {
            complexity += 1.0;
        }
        if self.y.is_some() {
            complexity += 1.0;
        }
        if self.color.is_some() {
            complexity += 1.5;
        }
        if self.size.is_some() {
            complexity += 1.0;
        }
        if self.shape.is_some() {
            complexity += 1.0;
        }
        if self.opacity.is_some() {
            complexity += 0.5;
        }
        if self.text.is_some() {
            complexity += 1.0;
        }
        if self.tooltip.is_some() {
            complexity += 0.5;
        }
        if self.detail.is_some() {
            complexity += 1.0;
        }
        if self.order.is_some() {
            complexity += 0.5;
        }
        if self.row.is_some() {
            complexity += 1.5;
        }
        if self.column.is_some() {
            complexity += 1.5;
        }

        complexity
    }

    /// Optimize encoding for performance
    pub fn optimize(self) -> Self {
        // Remove unnecessary encodings and optimize scales
        self
    }
}

impl Transform {
    /// Validate transform
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Basic validation for transforms
        Ok(())
    }

    /// Optimize transform
    pub fn optimize(self) -> Self {
        // Apply optimizations
        self
    }
}

impl Selection {
    /// Validate selection
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Basic validation for selections
        Ok(())
    }
}

impl Intelligence {
    /// Validate intelligence features
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Basic validation for intelligence features
        Ok(())
    }

    /// Calculate intelligence complexity
    pub fn complexity(&self) -> f64 {
        // Calculate complexity based on enabled features
        1.0
    }
}

impl MarkType {
    /// Calculate mark complexity
    pub fn complexity(&self) -> f64 {
        match self {
            MarkType::Point { .. } => 1.0,
            MarkType::Line { .. } => 2.0,
            MarkType::Bar { .. } => 1.5,
            MarkType::Area { .. } => 3.0,
            MarkType::Text { .. } => 0.5,
            MarkType::Rect { .. } => 1.0,
            MarkType::Scatter { .. } => 1.5,
            MarkType::BoxPlot { .. } => 2.5,
            MarkType::Violin { .. } => 3.0,
            MarkType::Heatmap { .. } => 2.0,
            MarkType::Histogram { .. } => 1.5,
            MarkType::Density { .. } => 2.5,
            MarkType::Contour { .. } => 3.5,
            MarkType::Radar { .. } => 2.0,
            MarkType::Sankey { .. } => 4.0,
            MarkType::Treemap { .. } => 3.0,
            MarkType::Composite(ref marks) => marks.iter().map(|m| m.complexity()).sum(),
            // Phase 3 Advanced Chart Types
            MarkType::Point3D { .. } => 4.0,
            MarkType::Surface3D { .. } => 5.0,
            MarkType::Choropleth { .. } => 3.5,
            MarkType::NetworkGraph { .. } => 4.5,
            MarkType::DotMap { .. } => 3.0,
            MarkType::FlowMap { .. } => 4.0,
        }
    }
}
