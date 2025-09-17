//! Chart Module
//!
//! This module provides chart specification system with compile-time validation.

pub mod builder;
pub mod rendering;
pub mod specification;
pub mod types;
pub mod validation;

// Re-export main types and functions
pub use builder::*;
pub use rendering::*;
pub use specification::*;
pub use types::*;
pub use validation::*;

// Common types used across all chart features
use serde::{Deserialize, Serialize};

/// Core chart specification structure with compile-time validation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChartSpec {
    pub data: DataReference,
    pub mark: MarkType,
    pub encoding: Encoding,
    pub transform: Vec<Transform>,
    pub selection: Vec<Selection>,
    pub intelligence: Option<Intelligence>,
    pub config: ChartConfig,
}

impl ChartSpec {
    /// Create a new chart specification
    pub fn new() -> Self {
        ChartSpecBuilder::default()
            .build()
            .expect("Failed to build default chart spec")
    }

    /// Validate the specification
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Validate data reference
        self.data.validate()?;

        // Validate encoding matches mark type
        self.encoding.validate_for_mark(&self.mark)?;

        // Validate transforms
        for transform in &self.transform {
            transform.validate()?;
        }

        // Validate selections
        for selection in &self.selection {
            selection.validate()?;
        }

        // Validate intelligence features
        if let Some(intelligence) = &self.intelligence {
            intelligence.validate()?;
        }

        Ok(())
    }

    /// Optimize for performance
    pub fn optimize(self) -> Self {
        // Apply performance optimizations
        let optimized_encoding = self.encoding.optimize();
        let optimized_transforms = self.transform.into_iter().map(|t| t.optimize()).collect();

        Self {
            encoding: optimized_encoding,
            transform: optimized_transforms,
            ..self
        }
    }

    /// Estimate render complexity
    pub fn complexity(&self) -> f64 {
        let base_complexity = match self.mark {
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
        };

        let encoding_complexity = self.encoding.complexity();
        let transform_complexity = self.transform.len() as f64 * 0.5;
        let intelligence_complexity = self.intelligence.as_ref().map_or(0.0, |i| i.complexity());

        base_complexity + encoding_complexity + transform_complexity + intelligence_complexity
    }
}
