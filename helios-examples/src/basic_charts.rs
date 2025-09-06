//! Basic chart examples for Helios
//!
//! This module demonstrates how to create simple charts using the Helios library.

use helios_core::chart::*;
use helios_core::data::*;
use helios_core::DataFrame;
use polars::prelude::*;

/// Create a simple line chart example
pub fn create_line_chart_example() -> Result<ChartSpec, Box<dyn std::error::Error>> {
    // Create sample data
    let x_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y_data = vec![2.0, 4.0, 6.0, 8.0, 10.0];

    // Create DataFrame
    let df = DataFrame::new(vec![
        Series::new("x".into(), x_data),
        Series::new("y".into(), y_data),
    ])?;

    // Create chart specification using defaults
    let chart_spec = ChartSpecBuilder::default()
        .data(DataReference::DataFrame(df))
        .mark(MarkType::Line {
            interpolate: None,
            stroke_width: None,
            stroke_dash: None,
        })
        .encoding(Encoding::default())
        .config(ChartConfig {
            title: Some(TitleConfig {
                text: "Simple Line Chart".to_string(),
                font_size: None,
                font_family: None,
                color: None,
            }),
            ..Default::default()
        })
        .build()?;

    Ok(chart_spec)
}

/// Create a scatter plot example
pub fn create_scatter_plot_example() -> Result<ChartSpec, Box<dyn std::error::Error>> {
    // Create sample data with some randomness
    let x_data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let y_data = vec![2.1, 3.9, 6.2, 7.8, 10.1, 12.3, 14.0, 15.9, 18.2, 19.8];

    // Create DataFrame
    let df = DataFrame::new(vec![
        Series::new("x".into(), x_data),
        Series::new("y".into(), y_data),
    ])?;

    // Create chart specification using defaults
    let chart_spec = ChartSpecBuilder::default()
        .data(DataReference::DataFrame(df))
        .mark(MarkType::Scatter {
            size: None,
            shape: None,
            opacity: None,
            jitter: None,
            trend_line: None,
        })
        .encoding(Encoding::default())
        .config(ChartConfig {
            title: Some(TitleConfig {
                text: "Scatter Plot Example".to_string(),
                font_size: None,
                font_family: None,
                color: None,
            }),
            ..Default::default()
        })
        .build()?;

    Ok(chart_spec)
}

/// Create a bar chart example
pub fn create_bar_chart_example() -> Result<ChartSpec, Box<dyn std::error::Error>> {
    // Create sample data
    let categories = vec!["A", "B", "C", "D", "E"];
    let values = vec![10.0, 25.0, 15.0, 30.0, 20.0];

    // Create DataFrame
    let df = DataFrame::new(vec![
        Series::new("category".into(), categories),
        Series::new("value".into(), values),
    ])?;

    // Create chart specification using defaults
    let chart_spec = ChartSpecBuilder::default()
        .data(DataReference::DataFrame(df))
        .mark(MarkType::Bar {
            width: None,
            corner_radius: None,
        })
        .encoding(Encoding::default())
        .config(ChartConfig {
            title: Some(TitleConfig {
                text: "Bar Chart Example".to_string(),
                font_size: None,
                font_family: None,
                color: None,
            }),
            ..Default::default()
        })
        .build()?;

    Ok(chart_spec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_chart_creation() {
        let chart = create_line_chart_example();
        assert!(chart.is_ok(), "Line chart should be created successfully");

        let chart_spec = chart.unwrap();
        match chart_spec.mark {
            MarkType::Line { .. } => assert!(true),
            _ => assert!(false, "Should be a line chart"),
        }
        assert!(chart_spec.config.title.is_some());
    }

    #[test]
    fn test_scatter_plot_creation() {
        let chart = create_scatter_plot_example();
        assert!(chart.is_ok(), "Scatter plot should be created successfully");

        let chart_spec = chart.unwrap();
        match chart_spec.mark {
            MarkType::Scatter { .. } => assert!(true),
            _ => assert!(false, "Should be a scatter plot"),
        }
    }

    #[test]
    fn test_bar_chart_creation() {
        let chart = create_bar_chart_example();
        assert!(chart.is_ok(), "Bar chart should be created successfully");

        let chart_spec = chart.unwrap();
        match chart_spec.mark {
            MarkType::Bar { .. } => assert!(true),
            _ => assert!(false, "Should be a bar chart"),
        }
    }
}
