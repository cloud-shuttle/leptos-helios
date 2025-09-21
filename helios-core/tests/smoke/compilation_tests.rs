//! Smoke tests to ensure basic compilation and API surface stability
//!
//! These tests verify that core types can be instantiated without panicking
//! and that the public API surface remains stable.

use leptos_helios::*;
use polars::prelude::*;

#[test]
fn test_core_crate_compiles() {
    // Verify core types instantiate without panic
    let _spec = ChartSpec::default();
    let _encoding = Encoding::default();
    let _mark_type = MarkType::Point;
    let _data_type = DataType::String;
    
    // Test that we can create basic chart configurations
    let mut spec = ChartSpec::new();
    spec.mark = MarkType::Line;
    spec.title = Some("Test Chart".to_string());
    
    assert_eq!(spec.mark, MarkType::Line);
    assert_eq!(spec.title, Some("Test Chart".to_string()));
}

#[test]
fn test_data_types_instantiate() {
    // Test that data types can be created
    let _string_type = DataType::String;
    let _number_type = DataType::Number;
    let _date_type = DataType::Date;
    let _boolean_type = DataType::Boolean;
}

#[test]
fn test_mark_types_instantiate() {
    // Test that all mark types can be created
    let _point = MarkType::Point;
    let _line = MarkType::Line;
    let _bar = MarkType::Bar { width: None, corner_radius: None };
    let _area = MarkType::Area { interpolate: None, opacity: None };
    let _rect = MarkType::Rect;
}

#[test]
fn test_encoding_def_creation() {
    // Test that encoding definitions can be created
    let encoding_def = EncodingDef {
        field: "test_field".to_string(),
        data_type: DataType::Number,
        scale: None,
        axis: None,
        legend: None,
        bin: None,
        aggregate: None,
        sort: None,
    };
    
    assert_eq!(encoding_def.field, "test_field");
    assert_eq!(encoding_def.data_type, DataType::Number);
}

#[test]
fn test_encoding_creation() {
    // Test that encoding can be created
    let encoding = Encoding {
        x: Some(EncodingDef {
            field: "x_field".to_string(),
            data_type: DataType::Number,
            scale: None,
            axis: None,
            legend: None,
            bin: None,
            aggregate: None,
            sort: None,
        }),
        y: Some(EncodingDef {
            field: "y_field".to_string(),
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
    };
    
    assert!(encoding.x.is_some());
    assert!(encoding.y.is_some());
    assert_eq!(encoding.x.as_ref().unwrap().field, "x_field");
    assert_eq!(encoding.y.as_ref().unwrap().field, "y_field");
}

#[test]
fn test_chart_spec_builder_pattern() {
    // Test that we can build a chart spec using the builder pattern
    let spec = ChartSpec::new()
        .with_mark(MarkType::Bar { width: None, corner_radius: None })
        .with_title("Test Bar Chart")
        .with_encoding(Encoding {
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
        });
    
    assert_eq!(spec.mark, MarkType::Bar { width: None, corner_radius: None });
    assert_eq!(spec.title, Some("Test Bar Chart".to_string()));
    assert!(spec.encoding.x.is_some());
    assert!(spec.encoding.y.is_some());
}

#[test]
fn test_data_reference_creation() {
    // Test that data references can be created
    let df = DataFrame::new(vec![
        Series::new("x", &[1, 2, 3, 4, 5]),
        Series::new("y", &[10, 20, 30, 40, 50]),
    ]).unwrap();
    
    let data_ref = DataReference::DataFrame(df);
    
    match data_ref {
        DataReference::DataFrame(_) => {
            // Success - we can create a DataFrame reference
        }
        _ => panic!("Expected DataFrame reference"),
    }
}

#[test]
fn test_security_types_instantiate() {
    // Test that security types can be created without panicking
    let _user = User::new(
        "user1".to_string(),
        "testuser".to_string(),
        "test@example.com".to_string(),
        "Test User".to_string(),
    );
    
    let _credentials = Credentials::username_password(
        "testuser".to_string(),
        "password123".to_string(),
    );
    
    let _permission = Permission::ViewCharts;
}

#[test]
fn test_export_types_instantiate() {
    // Test that export types can be created
    let _export_config = ExportConfig::default();
    let _export_format = ExportFormat::PNG {
        width: 800,
        height: 600,
        dpi: Some(96),
    };
}

#[test]
fn test_nl_processor_instantiate() {
    // Test that NL processor can be created
    let _processor = NLProcessor::new();
}

#[test]
fn test_webgpu_renderer_types_instantiate() {
    // Test that WebGPU types can be created (without actually initializing WebGPU)
    let _memory_usage = MemoryUsage::default();
    let _buffer_stats = BufferPoolStats::default();
}

#[test]
fn test_public_api_stability() {
    // Contract test for public API surface
    // This test will fail if breaking changes are introduced to core types
    
    // Test that core structs have expected fields
    let spec = ChartSpec::default();
    // These fields should always exist
    let _mark = &spec.mark;
    let _encoding = &spec.encoding;
    let _title = &spec.title;
    let _description = &spec.description;
    let _width = &spec.width;
    let _height = &spec.height;
    
    // Test that enums have expected variants
    match MarkType::Point {
        MarkType::Point => {},
        MarkType::Line => {},
        MarkType::Bar { .. } => {},
        MarkType::Area { .. } => {},
        MarkType::Rect => {},
    }
    
    match DataType::String {
        DataType::String => {},
        DataType::Number => {},
        DataType::Date => {},
        DataType::Boolean => {},
    }
}
