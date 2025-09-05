//! TDD Tests for Chart Specification Validation
//!
//! Following RED-GREEN-REFACTOR cycle:
//! 1. RED: Write failing tests first
//! 2. GREEN: Implement minimal code to make tests pass
//! 3. REFACTOR: Improve implementation while keeping tests green

use helios_core::chart::*;
use helios_core::utils::test_utils::*;
use helios_core::DataFrame;

#[test]
fn test_chart_spec_validation_success() {
    // RED: This test should fail initially
    let spec = create_test_chart_spec();

    // Test that valid chart specification passes validation
    let result = spec.validate();
    assert!(result.is_ok(), "Valid chart spec should pass validation");
}

#[test]
fn test_chart_spec_validation_missing_required_encoding() {
    // RED: Test that missing required encodings fail validation
    let mut spec = create_test_chart_spec();

    // Remove required x encoding for point chart
    spec.encoding.x = None;

    let result = spec.validate();
    assert!(
        result.is_err(),
        "Chart spec missing required encoding should fail validation"
    );

    if let Err(ValidationError::MissingRequiredEncoding(msg)) = result {
        assert!(msg.contains("x and y required for points"));
    } else {
        panic!("Expected MissingRequiredEncoding error");
    }
}

#[test]
fn test_chart_spec_validation_empty_dataframe() {
    // RED: Test that empty DataFrame fails validation
    let mut spec = create_test_chart_spec();
    spec.data = DataReference::DataFrame(DataFrame::empty());

    let result = spec.validate();
    assert!(result.is_err(), "Empty DataFrame should fail validation");

    if let Err(ValidationError::DataValidation(msg)) = result {
        assert!(msg.contains("Empty DataFrame"));
    } else {
        panic!("Expected DataValidation error");
    }
}

#[test]
fn test_chart_spec_validation_invalid_url() {
    // RED: Test that invalid URL fails validation
    let mut spec = create_test_chart_spec();
    spec.data = DataReference::Url {
        url: "".to_string(),
        format: DataFormat::Csv,
    };

    let result = spec.validate();
    assert!(result.is_err(), "Empty URL should fail validation");

    if let Err(ValidationError::DataValidation(msg)) = result {
        assert!(msg.contains("Empty URL"));
    } else {
        panic!("Expected DataValidation error");
    }
}

#[test]
fn test_chart_spec_validation_invalid_sql_query() {
    // RED: Test that invalid SQL query fails validation
    let mut spec = create_test_chart_spec();
    spec.data = DataReference::Query {
        sql: "".to_string(),
        dataset: "test".to_string(),
    };

    let result = spec.validate();
    assert!(result.is_err(), "Empty SQL query should fail validation");

    if let Err(ValidationError::DataValidation(msg)) = result {
        assert!(msg.contains("Empty SQL query"));
    } else {
        panic!("Expected DataValidation error");
    }
}

#[test]
fn test_chart_spec_validation_invalid_forecast_config() {
    // RED: Test that invalid forecast configuration fails validation
    let mut spec = create_test_chart_spec();
    spec.intelligence = Some(Intelligence {
        forecast: Some(ForecastConfig {
            periods: 0, // Invalid: periods must be > 0
            confidence: None,
            method: None,
        }),
        anomaly_detection: None,
        trend_analysis: None,
        clustering: None,
    });

    let result = spec.validate();
    assert!(
        result.is_err(),
        "Invalid forecast config should fail validation"
    );

    if let Err(ValidationError::InvalidIntelligence(msg)) = result {
        assert!(msg.contains("Forecast periods must be > 0"));
    } else {
        panic!("Expected InvalidIntelligence error");
    }
}

#[test]
fn test_chart_spec_validation_invalid_confidence_interval() {
    // RED: Test that invalid confidence interval fails validation
    let mut spec = create_test_chart_spec();
    spec.intelligence = Some(Intelligence {
        forecast: Some(ForecastConfig {
            periods: 30,
            confidence: Some(1.5), // Invalid: confidence must be between 0 and 1
            method: None,
        }),
        anomaly_detection: None,
        trend_analysis: None,
        clustering: None,
    });

    let result = spec.validate();
    assert!(
        result.is_err(),
        "Invalid confidence interval should fail validation"
    );

    if let Err(ValidationError::InvalidIntelligence(msg)) = result {
        assert!(msg.contains("Confidence must be between 0 and 1"));
    } else {
        panic!("Expected InvalidIntelligence error");
    }
}

#[test]
fn test_chart_spec_validation_invalid_anomaly_threshold() {
    // RED: Test that invalid anomaly threshold fails validation
    let mut spec = create_test_chart_spec();
    spec.intelligence = Some(Intelligence {
        forecast: None,
        anomaly_detection: Some(AnomalyConfig {
            method: "isolation_forest".to_string(),
            threshold: 1.5, // Invalid: threshold must be between 0 and 1
            sensitivity: None,
        }),
        trend_analysis: None,
        clustering: None,
    });

    let result = spec.validate();
    assert!(
        result.is_err(),
        "Invalid anomaly threshold should fail validation"
    );

    if let Err(ValidationError::InvalidIntelligence(msg)) = result {
        assert!(msg.contains("Anomaly threshold must be between 0 and 1"));
    } else {
        panic!("Expected InvalidIntelligence error");
    }
}

#[test]
fn test_chart_spec_validation_empty_selection_name() {
    // RED: Test that empty selection name fails validation
    let mut spec = create_test_chart_spec();
    spec.selection = vec![Selection {
        name: "".to_string(), // Invalid: empty name
        selection_type: SelectionType::Interval,
        bind: None,
        fields: None,
    }];

    let result = spec.validate();
    assert!(
        result.is_err(),
        "Empty selection name should fail validation"
    );

    if let Err(ValidationError::InvalidSelection(msg)) = result {
        assert!(msg.contains("Empty selection name"));
    } else {
        panic!("Expected InvalidSelection error");
    }
}

#[test]
fn test_chart_spec_validation_empty_filter_expression() {
    // RED: Test that empty filter expression fails validation
    let mut spec = create_test_chart_spec();
    spec.transform = vec![Transform::Filter {
        expression: "".to_string(), // Invalid: empty expression
    }];

    let result = spec.validate();
    assert!(
        result.is_err(),
        "Empty filter expression should fail validation"
    );

    if let Err(ValidationError::InvalidTransform(msg)) = result {
        assert!(msg.contains("Empty filter expression"));
    } else {
        panic!("Expected InvalidTransform error");
    }
}

#[test]
fn test_chart_spec_validation_empty_aggregation_operations() {
    // RED: Test that empty aggregation operations fail validation
    let mut spec = create_test_chart_spec();
    spec.transform = vec![Transform::Aggregate {
        operations: vec![], // Invalid: no operations
        groupby: vec!["category".to_string()],
    }];

    let result = spec.validate();
    assert!(
        result.is_err(),
        "Empty aggregation operations should fail validation"
    );

    if let Err(ValidationError::InvalidTransform(msg)) = result {
        assert!(msg.contains("No aggregation operations"));
    } else {
        panic!("Expected InvalidTransform error");
    }
}

#[test]
fn test_chart_spec_complexity_calculation() {
    // RED: Test complexity calculation for different chart types
    let point_spec = create_test_chart_spec();
    let complexity = point_spec.complexity();

    // Point charts should have base complexity of 1.0
    assert!(
        complexity >= 1.0,
        "Point chart complexity should be at least 1.0"
    );

    // Test line chart complexity
    let mut line_spec = create_test_chart_spec();
    line_spec.mark = MarkType::Line {
        interpolate: None,
        stroke_width: None,
        stroke_dash: None,
    };
    let line_complexity = line_spec.complexity();

    // Line charts should have higher complexity than points
    assert!(
        line_complexity > complexity,
        "Line chart should have higher complexity than point chart"
    );
}

#[test]
fn test_chart_spec_optimization() {
    // RED: Test that optimization doesn't break validation
    let spec = create_test_chart_spec();
    let original_complexity = spec.complexity();
    let optimized_spec = spec.optimize();

    // Optimized spec should still be valid
    let result = optimized_spec.validate();
    assert!(result.is_ok(), "Optimized chart spec should still be valid");

    // Complexity should be the same or better
    let optimized_complexity = optimized_spec.complexity();
    assert!(
        optimized_complexity <= original_complexity,
        "Optimized complexity should be same or better: {} <= {}",
        optimized_complexity,
        original_complexity
    );
}

#[test]
fn test_chart_spec_builder() {
    // RED: Test chart specification builder
    let spec = ChartSpecBuilder::new()
        .data(DataReference::DataFrame(create_test_dataframe()))
        .mark(MarkType::Point {
            size: Some(5.0),
            shape: None,
            opacity: None,
        })
        .encoding(Encoding {
            x: Some(PositionEncoding {
                field: "x".to_string(),
                data_type: DataType::Quantitative,
                scale: None,
                axis: None,
                bin: None,
                sort: None,
            }),
            y: Some(PositionEncoding {
                field: "y".to_string(),
                data_type: DataType::Quantitative,
                scale: None,
                axis: None,
                bin: None,
                sort: None,
            }),
            ..Default::default()
        })
        .build();

    assert!(spec.is_ok(), "Chart spec builder should create valid spec");

    let spec = spec.unwrap();
    assert_eq!(
        spec.mark,
        MarkType::Point {
            size: Some(5.0),
            shape: None,
            opacity: None
        }
    );
    assert!(spec.encoding.x.is_some());
    assert!(spec.encoding.y.is_some());
}

#[test]
fn test_mark_type_complexity() {
    // RED: Test mark type complexity calculation
    let point_complexity = MarkType::Point {
        size: None,
        shape: None,
        opacity: None,
    }
    .complexity();
    let line_complexity = MarkType::Line {
        interpolate: None,
        stroke_width: None,
        stroke_dash: None,
    }
    .complexity();
    let bar_complexity = MarkType::Bar {
        width: None,
        corner_radius: None,
    }
    .complexity();
    let area_complexity = MarkType::Area {
        interpolate: None,
        opacity: None,
    }
    .complexity();

    assert_eq!(point_complexity, 1.0, "Point complexity should be 1.0");
    assert_eq!(line_complexity, 2.0, "Line complexity should be 2.0");
    assert_eq!(bar_complexity, 1.5, "Bar complexity should be 1.5");
    assert_eq!(area_complexity, 3.0, "Area complexity should be 3.0");

    // Test composite mark complexity
    let composite_complexity = MarkType::Composite(vec![
        MarkType::Point {
            size: None,
            shape: None,
            opacity: None,
        },
        MarkType::Line {
            interpolate: None,
            stroke_width: None,
            stroke_dash: None,
        },
    ])
    .complexity();

    assert_eq!(
        composite_complexity, 3.0,
        "Composite complexity should be sum of parts"
    );
}

#[test]
fn test_encoding_complexity() {
    // RED: Test encoding complexity calculation
    let simple_encoding = Encoding {
        x: Some(PositionEncoding {
            field: "x".to_string(),
            data_type: DataType::Quantitative,
            scale: None,
            axis: None,
            bin: None,
            sort: None,
        }),
        y: Some(PositionEncoding {
            field: "y".to_string(),
            data_type: DataType::Quantitative,
            scale: None,
            axis: None,
            bin: None,
            sort: None,
        }),
        ..Default::default()
    };

    let complex_encoding = Encoding {
        x: Some(PositionEncoding {
            field: "x".to_string(),
            data_type: DataType::Quantitative,
            scale: None,
            axis: None,
            bin: None,
            sort: None,
        }),
        y: Some(PositionEncoding {
            field: "y".to_string(),
            data_type: DataType::Quantitative,
            scale: None,
            axis: None,
            bin: None,
            sort: None,
        }),
        color: Some(ColorEncoding {
            field: Some("category".to_string()),
            data_type: Some(DataType::Nominal),
            scale: None,
            condition: None,
        }),
        size: Some(SizeEncoding {
            field: Some("value".to_string()),
            data_type: Some(DataType::Quantitative),
            scale: None,
        }),
        facet: Some(FacetEncoding {
            field: "group".to_string(),
            data_type: DataType::Nominal,
            columns: Some(2),
            rows: Some(2),
        }),
        ..Default::default()
    };

    let simple_complexity = simple_encoding.complexity();
    let complex_complexity = complex_encoding.complexity();

    assert_eq!(
        simple_complexity, 2.0,
        "Simple encoding should have complexity 2.0"
    );
    assert!(
        complex_complexity > simple_complexity,
        "Complex encoding should have higher complexity"
    );
    assert_eq!(
        complex_complexity, 6.5,
        "Complex encoding should have complexity 6.5"
    );
}

#[test]
fn test_intelligence_complexity() {
    // RED: Test intelligence feature complexity calculation
    let no_intelligence = Intelligence {
        forecast: None,
        anomaly_detection: None,
        trend_analysis: None,
        clustering: None,
    };

    let with_forecast = Intelligence {
        forecast: Some(ForecastConfig {
            periods: 30,
            confidence: Some(0.95),
            method: Some("arima".to_string()),
        }),
        anomaly_detection: None,
        trend_analysis: None,
        clustering: None,
    };

    let with_all_features = Intelligence {
        forecast: Some(ForecastConfig {
            periods: 30,
            confidence: Some(0.95),
            method: Some("arima".to_string()),
        }),
        anomaly_detection: Some(AnomalyConfig {
            method: "isolation_forest".to_string(),
            threshold: 0.1,
            sensitivity: Some(0.5),
        }),
        trend_analysis: Some(true),
        clustering: Some(ClusterConfig {
            k: 3,
            method: "kmeans".to_string(),
            features: vec!["x".to_string(), "y".to_string()],
        }),
    };

    let no_complexity = no_intelligence.complexity();
    let forecast_complexity = with_forecast.complexity();
    let all_complexity = with_all_features.complexity();

    assert_eq!(
        no_complexity, 0.0,
        "No intelligence should have complexity 0.0"
    );
    assert_eq!(
        forecast_complexity, 2.0,
        "Forecast should have complexity 2.0"
    );
    assert_eq!(
        all_complexity, 7.0,
        "All features should have complexity 7.0"
    );
}
