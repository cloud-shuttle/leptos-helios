//! Alternative Text Generation
//!
//! This module provides alternative text generation for charts and visualizations.

use super::{AccessibilityError, DataTable};
use crate::chart::{ChartSpec, MarkType};
use polars::prelude::DataFrame;
use std::collections::HashMap;

/// Alternative text generator
pub struct AltTextGenerator {
    config: super::AlternativeFormats,
    templates: HashMap<String, String>,
}

impl AltTextGenerator {
    /// Create a new alt text generator
    pub fn new(config: super::AlternativeFormats) -> Self {
        let mut generator = Self {
            config,
            templates: HashMap::new(),
        };
        generator.initialize_templates();
        generator
    }

    /// Generate alternative text for a chart
    pub fn generate_alt_text(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
    ) -> Result<String, AccessibilityError> {
        if !self.config.text_descriptions {
            return Err(AccessibilityError::ScreenReaderError(
                "Text descriptions are disabled".to_string(),
            ));
        }

        let mut alt_text = String::new();

        // Add chart title
        if !spec.config.title.is_empty() {
            alt_text.push_str(&format!("Chart: {}. ", spec.config.title));
        }

        // Add chart type description
        alt_text.push_str(&self.get_chart_type_description(&spec.mark));

        // Add data summary
        alt_text.push_str(&self.generate_data_summary(data)?);

        // Add key insights
        if let Ok(insights) = self.generate_key_insights(spec, data) {
            alt_text.push_str(&insights);
        }

        Ok(alt_text)
    }

    /// Generate detailed description
    pub fn generate_detailed_description(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
    ) -> Result<String, AccessibilityError> {
        let mut description = String::new();

        // Chart overview
        description.push_str(&format!(
            "This {} chart",
            self.get_chart_type_name(&spec.mark)
        ));

        if !spec.config.title.is_empty() {
            description.push_str(&format!(" titled '{}'", spec.config.title));
        }

        description.push_str(" displays the following data:\n\n");

        // Data breakdown
        description.push_str(&self.generate_data_breakdown(data)?);

        // Visual elements
        description.push_str(&self.describe_visual_elements(spec, data)?);

        // Interaction instructions
        description.push_str("\n\nInteraction: This chart is interactive. Use keyboard navigation to explore data points.");

        Ok(description)
    }

    /// Generate data table alternative
    pub fn generate_data_table(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
    ) -> Result<DataTable, AccessibilityError> {
        if !self.config.data_tables {
            return Err(AccessibilityError::ScreenReaderError(
                "Data table generation is disabled".to_string(),
            ));
        }

        let title = if spec.config.title.is_empty() {
            "Chart Data".to_string()
        } else {
            format!("{} - Data Table", spec.config.title)
        };

        let summary = format!(
            "Tabular representation of {} chart data",
            self.get_chart_type_name(&spec.mark)
        );

        // Get column names
        let headers = data.get_column_names();

        // Convert data to string representation
        let mut rows = Vec::new();
        for i in 0..data.height() {
            let mut row = Vec::new();
            for col in &headers {
                if let Ok(series) = data.column(col) {
                    let value = series
                        .get(i)
                        .map(|v| format!("{}", v))
                        .unwrap_or_else(|_| "N/A".to_string());
                    row.push(value);
                }
            }
            rows.push(row);
        }

        Ok(DataTable {
            title,
            summary,
            headers: headers.into_iter().map(|h| h.to_string()).collect(),
            rows,
            caption: Some(format!(
                "Data table for {} chart",
                self.get_chart_type_name(&spec.mark)
            )),
            scope_attributes: self.generate_scope_attributes(
                &headers
                    .into_iter()
                    .map(|h| h.to_string())
                    .collect::<Vec<_>>(),
            ),
        })
    }

    /// Generate high contrast version description
    pub fn generate_high_contrast_description(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
    ) -> Result<String, AccessibilityError> {
        if !self.config.high_contrast_version {
            return Err(AccessibilityError::ScreenReaderError(
                "High contrast version is disabled".to_string(),
            ));
        }

        let mut description = String::new();
        description.push_str("High contrast version: ");
        description.push_str(&self.generate_alt_text(spec, data)?);
        description.push_str(
            " This version uses high contrast colors and patterns for better visibility.",
        );

        Ok(description)
    }

    /// Initialize text templates
    fn initialize_templates(&mut self) {
        self.templates.insert("line_chart".to_string(),
            "Line chart showing trends over time. The x-axis represents time or categories, and the y-axis shows values.".to_string());

        self.templates.insert("bar_chart".to_string(),
            "Bar chart comparing values across categories. Each bar represents a different category with its corresponding value.".to_string());

        self.templates.insert("scatter_plot".to_string(),
            "Scatter plot showing the relationship between two variables. Each point represents a data observation.".to_string());

        self.templates.insert("area_chart".to_string(),
            "Area chart displaying cumulative values over time. The filled area shows the total magnitude of values.".to_string());
    }

    /// Get chart type description
    fn get_chart_type_description(&self, mark: &MarkType) -> String {
        match mark {
            MarkType::Line { .. } => {
                "Line chart showing trends and patterns over time or categories.".to_string()
            }
            MarkType::Bar { .. } => {
                "Bar chart comparing values across different categories.".to_string()
            }
            MarkType::Point { .. } => {
                "Scatter plot showing relationships between two variables.".to_string()
            }
            MarkType::Area { .. } => {
                "Area chart displaying cumulative values with filled areas.".to_string()
            }
            MarkType::Text { .. } => "Text visualization displaying textual data.".to_string(),
            _ => "Data visualization chart.".to_string(),
        }
    }

    /// Get chart type name
    fn get_chart_type_name(&self, mark: &MarkType) -> &'static str {
        match mark {
            MarkType::Line { .. } => "line",
            MarkType::Bar { .. } => "bar",
            MarkType::Point { .. } => "scatter",
            MarkType::Area { .. } => "area",
            MarkType::Text { .. } => "text",
            _ => "data",
        }
    }

    /// Generate data summary
    fn generate_data_summary(&self, data: &DataFrame) -> Result<String, AccessibilityError> {
        let row_count = data.height();
        let col_count = data.width();

        if row_count == 0 {
            return Ok("The chart contains no data.".to_string());
        }

        let mut summary = format!("The chart contains {} data points", row_count);

        if col_count > 1 {
            summary.push_str(&format!(" across {} categories", col_count));
        }

        summary.push_str(". ");

        // Add range information if possible
        if let Some(first_col_name) = data.get_column_names().first() {
            if let Ok(first_col) = data.column(first_col_name) {
                // Note: Column doesn't have min/max methods directly, would need to convert to Series
                // For now, just add basic info
                summary.push_str(&format!("First column: {}.", first_col_name));
            }
        }

        Ok(summary)
    }

    /// Generate key insights
    fn generate_key_insights(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
    ) -> Result<String, AccessibilityError> {
        let mut insights = String::new();

        // Basic statistical insights
        if data.height() > 0 {
            if let Some(first_col_name) = data.get_column_names().first() {
                if let Ok(_first_col) = data.column(first_col_name) {
                    // Note: Would need to convert to Series for statistical operations
                    insights.push_str(&format!("Data contains {} rows. ", data.height()));
                }
            }
        }

        Ok(insights)
    }

    /// Generate data breakdown
    fn generate_data_breakdown(&self, data: &DataFrame) -> Result<String, AccessibilityError> {
        let mut breakdown = String::new();
        let headers = data.get_column_names();

        for (i, header) in headers.iter().enumerate() {
            breakdown.push_str(&format!("Column {}: {}. ", i + 1, header));

            if let Ok(series) = data.column(header) {
                let count = series.len();
                breakdown.push_str(&format!("Contains {} values. ", count));
            }
        }

        Ok(breakdown)
    }

    /// Describe visual elements
    fn describe_visual_elements(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
    ) -> Result<String, AccessibilityError> {
        let mut description = String::new();

        // Describe axes
        if let Some(x_encoding) = &spec.encoding.x {
            description.push_str(&format!("X-axis represents: {}. ", x_encoding.field));
        }

        if let Some(y_encoding) = &spec.encoding.y {
            description.push_str(&format!("Y-axis represents: {}. ", y_encoding.field));
        }

        // Describe color encoding
        if let Some(color_encoding) = &spec.encoding.color {
            description.push_str(&format!("Colors represent: {}. ", color_encoding.field));
        }

        // Describe size encoding
        if let Some(size_encoding) = &spec.encoding.size {
            description.push_str(&format!("Sizes represent: {}. ", size_encoding.field));
        }

        Ok(description)
    }

    /// Generate scope attributes for data table
    fn generate_scope_attributes(&self, headers: &[String]) -> HashMap<String, String> {
        let mut attributes = HashMap::new();

        for (i, header) in headers.iter().enumerate() {
            attributes.insert(format!("col-{}", i), header.clone());
        }

        attributes
    }
}
