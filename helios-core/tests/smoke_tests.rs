//! Smoke tests for basic compilation and API surface stability
//!
//! These tests verify that core types can be instantiated without panicking
//! and that the public API surface remains stable.

use leptos_helios::chart::{ChartSpec, ChartSpecBuilder, MarkType, Encoding, EncodingDef, DataType as HeliosDataType, DataFormat, DataReference};
use polars::prelude::*;

#[test]
fn test_core_crate_compiles() {
    // Verify core types instantiate without panic
    let _spec = ChartSpecBuilder::default()
        .data(DataReference {
            source: "test".to_string(),
            format: DataFormat::Inline,
            schema: None,
        })
        .mark(MarkType::Point { size: None, shape: None, opacity: None })
        .encoding(Encoding {
            x: Some(EncodingDef {
                field: "x".to_string(),
                data_type: HeliosDataType::Number,
                scale: None,
                axis: None,
                legend: None,
                bin: None,
                aggregate: None,
                sort: None,
            }),
            y: Some(EncodingDef {
                field: "y".to_string(),
                data_type: HeliosDataType::Number,
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
        .build()
        .unwrap();
    let _encoding = Encoding::default();
    let _mark_type = MarkType::Point { size: None, shape: None, opacity: None };
    let _data_type = HeliosDataType::String;
    
    // Test that we can create basic chart configurations
    let spec = ChartSpecBuilder::default()
        .data(DataReference {
            source: "test".to_string(),
            format: DataFormat::Inline,
            schema: None,
        })
        .mark(MarkType::Line { interpolate: None, stroke_width: None, stroke_dash: None })
        .encoding(Encoding {
            x: Some(EncodingDef {
                field: "x".to_string(),
                data_type: HeliosDataType::Number,
                scale: None,
                axis: None,
                legend: None,
                bin: None,
                aggregate: None,
                sort: None,
            }),
            y: Some(EncodingDef {
                field: "y".to_string(),
                data_type: HeliosDataType::Number,
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
        .build()
        .unwrap();
    
    assert!(matches!(spec.mark, MarkType::Line { .. }));
}

#[test]
fn test_data_types_instantiate() {
    // Test that data types can be created
    let _string_type = HeliosDataType::String;
    let _number_type = HeliosDataType::Number;
    let _date_type = HeliosDataType::Date;
    let _boolean_type = HeliosDataType::Boolean;
}

#[test]
fn test_mark_types_instantiate() {
    // Test that all mark types can be created
    let _point = MarkType::Point { size: None, shape: None, opacity: None };
    let _line = MarkType::Line { interpolate: None, stroke_width: None, stroke_dash: None };
    let _bar = MarkType::Bar { width: None, corner_radius: None };
    let _area = MarkType::Area { interpolate: None, opacity: None };
    let _rect = MarkType::Rect { stroke: None, stroke_width: None };
}

#[test]
fn test_encoding_def_creation() {
    // Test that encoding definitions can be created
    let encoding_def = EncodingDef {
        field: "test_field".to_string(),
        data_type: HeliosDataType::Number,
        scale: None,
        axis: None,
        legend: None,
        bin: None,
        aggregate: None,
        sort: None,
    };
    
    assert_eq!(encoding_def.field, "test_field");
    assert_eq!(encoding_def.data_type, HeliosDataType::Number);
}

#[test]
fn test_encoding_creation() {
    // Test that encoding can be created
    let encoding = Encoding {
        x: Some(EncodingDef {
            field: "x_field".to_string(),
            data_type: HeliosDataType::Number,
            scale: None,
            axis: None,
            legend: None,
            bin: None,
            aggregate: None,
            sort: None,
        }),
        y: Some(EncodingDef {
            field: "y_field".to_string(),
            data_type: HeliosDataType::Number,
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
fn test_chart_spec_creation() {
    // Test that we can create a chart spec
    let spec = ChartSpecBuilder::default()
        .data(DataReference {
            source: "test".to_string(),
            format: DataFormat::Inline,
            schema: None,
        })
        .mark(MarkType::Bar { width: None, corner_radius: None })
        .encoding(Encoding {
        x: Some(EncodingDef {
            field: "category".to_string(),
                data_type: HeliosDataType::String,
            scale: None,
            axis: None,
            legend: None,
            bin: None,
            aggregate: None,
            sort: None,
        }),
        y: Some(EncodingDef {
            field: "value".to_string(),
            data_type: HeliosDataType::Number,
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
        .build()
        .unwrap();
    
    assert!(matches!(spec.mark, MarkType::Bar { .. }));
    assert!(spec.encoding.x.is_some());
    assert!(spec.encoding.y.is_some());
}

#[test]
fn test_data_reference_creation() {
    // Test that data references can be created
    let df = DataFrame::new(vec![
        Series::new("x".into(), &[1, 2, 3, 4, 5]).into(),
        Series::new("y".into(), &[10, 20, 30, 40, 50]).into(),
    ]).unwrap();
    
    let data_ref = DataReference {
        source: "test".to_string(),
        format: DataFormat::Inline,
        schema: None,
    };
    
    assert_eq!(data_ref.source, "test");
}

#[test]
fn test_security_types_instantiate() {
    // Test that security types can be created without panicking
    let _user = leptos_helios::security::User::new(
        "user1".to_string(),
        "testuser".to_string(),
        "test@example.com".to_string(),
        "Test User".to_string(),
    );
    
    let _credentials = leptos_helios::security::Credentials::username_password(
        "testuser".to_string(),
        "password123".to_string(),
    );
    
    let _permission = leptos_helios::security::Permission::ViewCharts;
}

#[test]
fn test_export_types_instantiate() {
    // Test that export types can be created
    let _export_config = leptos_helios::export_system::ExportConfig::default();
    let _export_format = leptos_helios::export_system::ExportFormat::PNG {
        width: 800,
        height: 600,
        dpi: Some(96),
    };
}

#[test]
fn test_nl_processor_instantiate() {
    // Test that NL processor can be created
    let _processor = leptos_helios::nl_processor::NLProcessor::new();
}

#[test]
fn test_webgpu_renderer_types_instantiate() {
    // Test that WebGPU types can be created (without actually initializing WebGPU)
    let _memory_usage = leptos_helios::webgpu_renderer::MemoryUsage::default();
    let _buffer_stats = leptos_helios::webgpu_renderer::BufferPoolStats::default();
}

#[test]
fn test_public_api_stability() {
    // Contract test for public API surface
    // This test will fail if breaking changes are introduced to core types
    
    // Test that core structs have expected fields
    let spec = ChartSpecBuilder::default()
        .data(DataReference {
            source: "test".to_string(),
            format: DataFormat::Inline,
            schema: None,
        })
        .mark(MarkType::Point { size: None, shape: None, opacity: None })
        .encoding(Encoding {
            x: Some(EncodingDef {
                field: "x".to_string(),
                data_type: HeliosDataType::Number,
                scale: None,
                axis: None,
                legend: None,
                bin: None,
                aggregate: None,
                sort: None,
            }),
            y: Some(EncodingDef {
                field: "y".to_string(),
                data_type: HeliosDataType::Number,
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
        .build()
        .unwrap();
    // These fields should always exist
    let _mark = &spec.mark;
    let _encoding = &spec.encoding;
    
    // Test that enums have expected variants
    match (MarkType::Point { size: None, shape: None, opacity: None }) {
        MarkType::Point { .. } => {},
        MarkType::Line { .. } => {},
        MarkType::Bar { .. } => {},
        MarkType::Area { .. } => {},
        MarkType::Rect { .. } => {},
        _ => {}, // Handle all other variants
    }
    
    match HeliosDataType::String {
        HeliosDataType::String => {},
        HeliosDataType::Number => {},
        HeliosDataType::Date => {},
        HeliosDataType::Boolean => {},
        _ => {}, // Handle all other variants
    }
}
