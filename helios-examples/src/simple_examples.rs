//! Simple examples for Helios that work with the current implementation
//!
//! This module demonstrates basic usage without complex dependencies.

use leptos_helios::chart::*;

/// Create a simple line chart example
pub fn create_simple_line_chart() -> Result<ChartSpec, Box<dyn std::error::Error>> {

    // Create chart specification using defaults
    let chart_spec = ChartSpecBuilder::default()
        .data(DataReference {
            source: "test_data".to_string(),
            format: DataFormat::Inline,
            schema: None,
        })
        .mark(MarkType::Line {
            interpolate: None,
            stroke_width: None,
            stroke_dash: None,
        })
        .encoding(Encoding {
            x: Some(EncodingDef {
                field: "x".to_string(),
                data_type: DataType::Number,
                scale: None,
                axis: None,
                legend: None,
                bin: None,
                aggregate: None,
                sort: None,
            }),
            y: Some(EncodingDef {
                field: "y".to_string(),
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
        .config(ChartConfig {
            title: "Simple Line Chart".to_string(),
            description: "A simple line chart example".to_string(),
            width: None,
            height: None,
            padding: None,
            background: None,
            viewport: None,
            legend: None,
            axis: None,
            mark: None,
            selection: None,
            scale: None,
            range: None,
            facet: None,
            header: None,
            overlay: None,
            style: None,
            signals: None,
            data: None,
            layout: None,
            projection: None,
            encoding: None,
            resolve: None,
            autosize: None,
            usermeta: None,
        })
        .build()?;

    Ok(chart_spec)
}

/// Create a simple scatter plot example
pub fn create_simple_scatter_plot() -> Result<ChartSpec, Box<dyn std::error::Error>> {

    // Create chart specification using defaults
    let chart_spec = ChartSpecBuilder::default()
        .data(DataReference {
            source: "test_data".to_string(),
            format: DataFormat::Inline,
            schema: None,
        })
        .mark(MarkType::Point {
            size: None,
            shape: None,
            opacity: None,
        })
        .encoding(Encoding {
            x: Some(EncodingDef {
                field: "x".to_string(),
                data_type: DataType::Number,
                scale: None,
                axis: None,
                legend: None,
                bin: None,
                aggregate: None,
                sort: None,
            }),
            y: Some(EncodingDef {
                field: "y".to_string(),
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
        .config(ChartConfig {
            title: "Simple Scatter Plot".to_string(),
            description: "A simple scatter plot example".to_string(),
            width: None,
            height: None,
            padding: None,
            background: None,
            viewport: None,
            legend: None,
            axis: None,
            mark: None,
            selection: None,
            scale: None,
            range: None,
            facet: None,
            header: None,
            overlay: None,
            style: None,
            signals: None,
            data: None,
            layout: None,
            projection: None,
            encoding: None,
            resolve: None,
            autosize: None,
            usermeta: None,
        })
        .build()?;

    Ok(chart_spec)
}

/// Create a simple bar chart example
pub fn create_simple_bar_chart() -> Result<ChartSpec, Box<dyn std::error::Error>> {

    // Create chart specification using defaults
    let chart_spec = ChartSpecBuilder::default()
        .data(DataReference {
            source: "test_data".to_string(),
            format: DataFormat::Inline,
            schema: None,
        })
        .mark(MarkType::Bar {
            width: None,
            corner_radius: None,
        })
        .encoding(Encoding {
            x: Some(EncodingDef {
                field: "category".to_string(),
                data_type: DataType::String,
                scale: None,
                axis: None,
                legend: None,
                bin: None,
                aggregate: None,
                sort: None,
            }),
            y: Some(EncodingDef {
                field: "value".to_string(),
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
        .config(ChartConfig {
            title: "Simple Bar Chart".to_string(),
            description: "A simple bar chart example".to_string(),
            width: None,
            height: None,
            padding: None,
            background: None,
            viewport: None,
            legend: None,
            axis: None,
            mark: None,
            selection: None,
            scale: None,
            range: None,
            facet: None,
            header: None,
            overlay: None,
            style: None,
            signals: None,
            data: None,
            layout: None,
            projection: None,
            encoding: None,
            resolve: None,
            autosize: None,
            usermeta: None,
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
        assert!(!chart_spec.config.title.is_empty());
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
            MarkType::Point { .. } => assert!(true),
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
            MarkType::Point { .. } => assert!(true),
            _ => assert!(false, "Second chart should be a scatter plot"),
        }
        match charts[2].mark {
            MarkType::Bar { .. } => assert!(true),
            _ => assert!(false, "Third chart should be a bar chart"),
        }
    }
}
