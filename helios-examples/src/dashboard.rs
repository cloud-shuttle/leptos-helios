//! Dashboard example for Helios
//!
//! This module demonstrates how to create a dashboard with multiple charts.

use helios_core::chart::*;
use helios_core::data::*;
use helios_core::DataFrame;
use polars::prelude::*;

/// Create a sample dashboard with multiple charts
pub fn create_sample_dashboard() -> Result<Vec<ChartSpec>, Box<dyn std::error::Error>> {
    let mut charts = Vec::new();

    // Chart 1: Sales over time (Line chart)
    let sales_data = DataFrame::new(vec![
        Series::new(
            "month".into(),
            vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
        ),
        Series::new(
            "sales".into(),
            vec![100.0, 120.0, 110.0, 140.0, 160.0, 150.0],
        ),
    ])?;

    let sales_chart = ChartSpecBuilder::default()
        .data(DataReference::DataFrame(sales_data))
        .mark(MarkType::Line {
            interpolate: None,
            stroke_width: None,
            stroke_dash: None,
        })
        .encoding(Encoding::default())
        .config(ChartConfig {
            title: Some(TitleConfig {
                text: "Sales Over Time".to_string(),
                font_size: None,
                font_family: None,
                color: None,
            }),
            ..Default::default()
        })
        .build()?;

    charts.push(sales_chart);

    // Chart 2: Product categories (Bar chart)
    let category_data = DataFrame::new(vec![
        Series::new(
            "category".into(),
            vec!["Electronics", "Clothing", "Books", "Home"],
        ),
        Series::new("revenue".into(), vec![250.0, 180.0, 90.0, 120.0]),
    ])?;

    let category_chart = ChartSpecBuilder::default()
        .data(DataReference::DataFrame(category_data))
        .mark(MarkType::Bar {
            width: None,
            corner_radius: None,
        })
        .encoding(Encoding::default())
        .config(ChartConfig {
            title: Some(TitleConfig {
                text: "Revenue by Category".to_string(),
                font_size: None,
                font_family: None,
                color: None,
            }),
            ..Default::default()
        })
        .build()?;

    charts.push(category_chart);

    // Chart 3: Customer satisfaction (Scatter plot)
    let satisfaction_data = DataFrame::new(vec![
        Series::new(
            "price".into(),
            vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0],
        ),
        Series::new(
            "satisfaction".into(),
            vec![8.5, 7.8, 8.2, 7.5, 7.0, 6.8, 6.5, 6.2],
        ),
    ])?;

    let satisfaction_chart = ChartSpecBuilder::default()
        .data(DataReference::DataFrame(satisfaction_data))
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
                text: "Price vs Satisfaction".to_string(),
                font_size: None,
                font_family: None,
                color: None,
            }),
            ..Default::default()
        })
        .build()?;

    charts.push(satisfaction_chart);

    Ok(charts)
}

/// Create a simple data processing example
pub fn create_data_processing_example() -> Result<DataFrame, Box<dyn std::error::Error>> {
    // Create sample data
    let raw_data = DataFrame::new(vec![
        Series::new("id".into(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
        Series::new(
            "value".into(),
            vec![10.5, 15.2, 8.7, 22.1, 18.9, 12.3, 25.6, 19.8, 14.2, 21.4],
        ),
        Series::new(
            "category".into(),
            vec!["A", "B", "A", "C", "B", "A", "C", "B", "A", "C"],
        ),
    ])?;

    // This would normally include data transformations
    // For now, we'll just return the raw data
    Ok(raw_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dashboard_creation() {
        let dashboard = create_sample_dashboard();
        assert!(
            dashboard.is_ok(),
            "Dashboard should be created successfully"
        );

        let charts = dashboard.unwrap();
        assert_eq!(charts.len(), 3, "Dashboard should have 3 charts");

        // Check chart types
        match charts[0].mark {
            MarkType::Line { .. } => assert!(true),
            _ => assert!(false, "First chart should be a line chart"),
        }
        match charts[1].mark {
            MarkType::Bar { .. } => assert!(true),
            _ => assert!(false, "Second chart should be a bar chart"),
        }
        match charts[2].mark {
            MarkType::Scatter { .. } => assert!(true),
            _ => assert!(false, "Third chart should be a scatter plot"),
        }
    }

    #[test]
    fn test_data_processing() {
        let data = create_data_processing_example();
        assert!(data.is_ok(), "Data processing should succeed");

        let df = data.unwrap();
        assert_eq!(df.height(), 10, "Should have 10 rows");
        assert_eq!(df.width(), 3, "Should have 3 columns");
    }
}
