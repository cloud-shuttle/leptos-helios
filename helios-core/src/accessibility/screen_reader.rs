//! Screen Reader Support
//!
//! This module provides screen reader support and ARIA integration.

use super::{AccessibilityError, DataTable, ScreenReaderSupport};
use crate::chart::{ChartSpec, MarkType};
use polars::prelude::DataFrame;
use std::collections::HashMap;

/// Screen reader manager
pub struct ScreenReaderManager {
    config: ScreenReaderSupport,
    live_regions: HashMap<String, String>,
    announcements: Vec<String>,
}

impl ScreenReaderManager {
    /// Create a new screen reader manager
    pub fn new(config: ScreenReaderSupport) -> Self {
        Self {
            config,
            live_regions: HashMap::new(),
            announcements: Vec::new(),
        }
    }

    /// Generate screen reader description for chart
    pub fn generate_description(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
    ) -> Result<String, AccessibilityError> {
        if !self.config.enabled {
            return Ok(String::new());
        }

        let mut description = String::new();

        // Add chart title and type
        if !spec.config.title.is_empty() {
            description.push_str(&format!("Chart titled: {}. ", spec.config.title));
        }

        description.push_str(&format!(
            "This is a {} chart. ",
            self.get_chart_type_description(&spec.mark)
        ));

        // Add data summary
        if let Ok(summary) = self.generate_data_summary(data) {
            description.push_str(&summary);
        }

        // Add interaction instructions
        description.push_str(" Use arrow keys to navigate data points. Press Enter to select.");

        Ok(description)
    }

    /// Create data table for screen readers
    pub fn create_data_table(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
    ) -> Result<DataTable, AccessibilityError> {
        if !self.config.create_data_tables {
            return Err(AccessibilityError::ScreenReaderError(
                "Data table creation is disabled".to_string(),
            ));
        }

        let title = if spec.config.title.is_empty() {
            "Chart Data".to_string()
        } else {
            format!("{} Data", spec.config.title)
        };

        let summary = self
            .generate_data_summary(data)
            .unwrap_or_else(|_| "Chart data table".to_string());

        // Extract headers from data
        let headers = data.get_column_names();

        // Convert data to string rows
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
            title: title.clone(),
            summary,
            headers: headers.into_iter().map(|h| h.to_string()).collect(),
            rows,
            caption: Some(format!("Data table for {}", title)),
            scope_attributes: HashMap::new(),
        })
    }

    /// Generate ARIA labels for chart elements
    pub fn generate_aria_labels(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
    ) -> Result<HashMap<String, String>, AccessibilityError> {
        if !self.config.aria_labels {
            return Ok(HashMap::new());
        }

        let mut labels = HashMap::new();

        // Chart container
        labels.insert(
            "chart-container".to_string(),
            format!("Interactive {} chart", self.get_chart_type_name(&spec.mark)),
        );

        // Data points
        for (i, row) in data.iter().enumerate() {
            let row_values: Vec<_> = row.iter().map(|v| v.clone()).collect();
            let point_label = self.generate_point_label(&spec.mark, &row_values, i);
            labels.insert(format!("data-point-{}", i), point_label);
        }

        // Legend items
        if let Some(legend) = &spec.config.legend {
            if let Some(title) = &legend.title {
                labels.insert("legend-title".to_string(), format!("Legend: {}", title));
            }
        }

        Ok(labels)
    }

    /// Announce update to screen readers
    pub fn announce_update(&mut self, message: &str) -> Result<(), AccessibilityError> {
        if !self.config.announce_updates {
            return Ok(());
        }

        self.announcements.push(message.to_string());
        Ok(())
    }

    /// Get pending announcements
    pub fn get_pending_announcements(&mut self) -> Vec<String> {
        self.announcements.drain(..).collect()
    }

    /// Generate structured navigation
    pub fn generate_navigation_structure(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
    ) -> Result<NavigationStructure, AccessibilityError> {
        if !self.config.structured_navigation {
            return Err(AccessibilityError::ScreenReaderError(
                "Structured navigation is disabled".to_string(),
            ));
        }

        let mut structure = NavigationStructure {
            landmarks: Vec::new(),
            headings: Vec::new(),
            regions: Vec::new(),
        };

        // Add main chart landmark
        structure.landmarks.push(Landmark {
            role: "main".to_string(),
            label: "Chart Content".to_string(),
            id: "chart-main".to_string(),
        });

        // Add chart heading
        structure.headings.push(Heading {
            level: 1,
            text: spec.config.title.clone(),
            id: "chart-title".to_string(),
        });

        // Add data region
        structure.regions.push(Region {
            role: "region".to_string(),
            label: "Chart Data".to_string(),
            id: "chart-data".to_string(),
            aria_live: Some("polite".to_string()),
        });

        Ok(structure)
    }

    /// Generate data summary
    fn generate_data_summary(&self, data: &DataFrame) -> Result<String, AccessibilityError> {
        let row_count = data.height();
        let col_count = data.width();

        Ok(format!(
            "The chart contains {} data points across {} categories. ",
            row_count, col_count
        ))
    }

    /// Get chart type description
    fn get_chart_type_description(&self, mark: &MarkType) -> &'static str {
        match mark {
            MarkType::Line { .. } => "line",
            MarkType::Bar { .. } => "bar",
            MarkType::Point { .. } => "scatter plot",
            MarkType::Area { .. } => "area",
            MarkType::Text { .. } => "text",
            _ => "data visualization",
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

    /// Generate point label
    fn generate_point_label(
        &self,
        mark: &MarkType,
        _row: &[polars::prelude::AnyValue],
        index: usize,
    ) -> String {
        match mark {
            MarkType::Point { .. } => format!("Data point {}", index + 1),
            MarkType::Bar { .. } => format!("Bar {}", index + 1),
            MarkType::Line { .. } => format!("Line point {}", index + 1),
            _ => format!("Data element {}", index + 1),
        }
    }
}

/// Navigation structure for screen readers
#[derive(Debug, Clone)]
pub struct NavigationStructure {
    pub landmarks: Vec<Landmark>,
    pub headings: Vec<Heading>,
    pub regions: Vec<Region>,
}

/// Landmark for navigation
#[derive(Debug, Clone)]
pub struct Landmark {
    pub role: String,
    pub label: String,
    pub id: String,
}

/// Heading for navigation
#[derive(Debug, Clone)]
pub struct Heading {
    pub level: u32,
    pub text: String,
    pub id: String,
}

/// Region for navigation
#[derive(Debug, Clone)]
pub struct Region {
    pub role: String,
    pub label: String,
    pub id: String,
    pub aria_live: Option<String>,
}
