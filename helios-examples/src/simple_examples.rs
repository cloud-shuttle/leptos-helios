//! Simple examples for Helios that work with the current implementation
//!
//! This module demonstrates basic usage without complex dependencies.

use leptos_helios::chart::*;
// use leptos_helios::data::*;
use leptos_helios::DataFrame;

/// Create a simple line chart example using empty DataFrame
pub fn create_simple_line_chart() -> Result<ChartSpec, Box<dyn std::error::Error>> {
    // Create empty DataFrame for testing
    let df = DataFrame::empty();

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

/// Create a simple scatter plot example
pub fn create_simple_scatter_plot() -> Result<ChartSpec, Box<dyn std::error::Error>> {
    // Create empty DataFrame for testing
    let df = DataFrame::empty();

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
                text: "Simple Scatter Plot".to_string(),
                font_size: None,
                font_family: None,
                color: None,
            }),
            ..Default::default()
        })
        .build()?;

    Ok(chart_spec)
}

/// Create a simple bar chart example
pub fn create_simple_bar_chart() -> Result<ChartSpec, Box<dyn std::error::Error>> {
    // Create empty DataFrame for testing
    let df = DataFrame::empty();

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
                text: "Simple Bar Chart".to_string(),
                font_size: None,
                font_family: None,
                color: None,
            }),
            ..Default::default()
        })
        .build()?;

    Ok(chart_spec)
}

/// Create a simple dashboard with multiple charts
pub fn create_simple_dashboard() -> Result<Vec<ChartSpec>, Box<dyn std::error::Error>> {
    let charts = vec![
        create_simple_line_chart()?,
        create_simple_scatter_plot()?,
        create_simple_bar_chart()?,
    ];

    Ok(charts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_line_chart_creation() {
        let chart = create_simple_line_chart();
        assert!(
            chart.is_ok(),
            "Simple line chart should be created successfully"
        );

        let chart_spec = chart.unwrap();
        match chart_spec.mark {
            MarkType::Line { .. } => assert!(true),
            _ => assert!(false, "Should be a line chart"),
        }
        assert!(chart_spec.config.title.is_some());
    }

    #[test]
    fn test_simple_scatter_plot_creation() {
        let chart = create_simple_scatter_plot();
        assert!(
            chart.is_ok(),
            "Simple scatter plot should be created successfully"
        );

        let chart_spec = chart.unwrap();
        match chart_spec.mark {
            MarkType::Scatter { .. } => assert!(true),
            _ => assert!(false, "Should be a scatter plot"),
        }
    }

    #[test]
    fn test_simple_bar_chart_creation() {
        let chart = create_simple_bar_chart();
        assert!(
            chart.is_ok(),
            "Simple bar chart should be created successfully"
        );

        let chart_spec = chart.unwrap();
        match chart_spec.mark {
            MarkType::Bar { .. } => assert!(true),
            _ => assert!(false, "Should be a bar chart"),
        }
    }

    #[test]
    fn test_simple_dashboard_creation() {
        let dashboard = create_simple_dashboard();
        assert!(
            dashboard.is_ok(),
            "Simple dashboard should be created successfully"
        );

        let charts = dashboard.unwrap();
        assert_eq!(charts.len(), 3, "Dashboard should have 3 charts");

        // Check chart types
        match charts[0].mark {
            MarkType::Line { .. } => assert!(true),
            _ => assert!(false, "First chart should be a line chart"),
        }
        match charts[1].mark {
            MarkType::Scatter { .. } => assert!(true),
            _ => assert!(false, "Second chart should be a scatter plot"),
        }
        match charts[2].mark {
            MarkType::Bar { .. } => assert!(true),
            _ => assert!(false, "Third chart should be a bar chart"),
        }
    }
}
