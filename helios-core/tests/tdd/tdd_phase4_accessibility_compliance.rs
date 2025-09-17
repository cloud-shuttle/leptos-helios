//! TDD Tests for WCAG 2.1 AA Accessibility Compliance
//!
//! This module tests comprehensive accessibility features including:
//! - WCAG 2.1 AA compliance validation
//! - Screen reader support and ARIA implementation
//! - Keyboard navigation and focus management
//! - Color vision support and contrast ratios
//! - Motion preferences and reduced motion support
//! - Alternative formats and data table generation
//! - Performance accessibility budgets

use leptos_helios::accessibility::*;
use leptos_helios::chart::{ChartConfig, ChartSpec, DataReference, Encoding, MarkType};
use polars::prelude::*;
use std::collections::HashMap;

#[tokio::test]
async fn test_wcag_2_1_aa_compliance_validation() {
    let performance_config = PerformanceConfig::default();
    let accessibility_system =
        AccessibilitySystem::new(AccessibilityConfig::default(), performance_config);

    // Create test data
    let data = df! [
        "month" => ["Jan", "Feb", "Mar"],
        "sales" => [1000, 1200, 1100]
    ]
    .unwrap();

    // Test WCAG 2.1 AA compliance validation
    let chart_spec = ChartSpec {
        transform: None,
        selection: None,
        intelligence: None,
    };

    let compliance_result = accessibility_system
        .validate_wcag_compliance(&chart_spec, &data)
        .unwrap();

    assert!(compliance_result.is_compliant);
    assert_eq!(compliance_result.level, WCAGLevel::AA);
    assert!(compliance_result.violations.is_empty());
    assert!(compliance_result.recommendations.is_empty());
}

#[tokio::test]
async fn test_screen_reader_support() {
    let mut config = AccessibilityConfig::default();
    config.screen_reader.enabled = true;
    config.screen_reader.generate_alt_text = true;
    config.screen_reader.create_data_tables = true;
    config.screen_reader.aria_labels = true;

    let performance_config = PerformanceConfig::default();
    let accessibility_system = AccessibilitySystem::new(config, performance_config);

    // Create test data
    let data = df! [
        "month" => ["Jan", "Feb", "Mar"],
        "sales" => [1000, 1200, 1100],
        "target" => [900, 1000, 1050]
    ]
    .unwrap();

    let chart_spec = ChartSpec {
        transform: None,
        selection: None,
        intelligence: None,
    };

    // Test alt text generation
    let alt_text = accessibility_system
        .generate_alt_text(&chart_spec, &data)
        .unwrap();

    assert!(!alt_text.is_empty());
    assert!(alt_text.contains("chart"));

    // Test data table generation
    let data_table = accessibility_system
        .create_data_table(&chart_spec, &data)
        .unwrap();

    assert_eq!(data_table.rows.len(), 3);
    assert_eq!(data_table.columns.len(), 3);
    assert!(data_table.columns.contains(&"month".to_string()));
    assert!(data_table.columns.contains(&"sales".to_string()));
    assert!(data_table.columns.contains(&"target".to_string()));
}

#[tokio::test]
async fn test_keyboard_navigation_support() {
    let mut config = AccessibilityConfig::default();
    config.keyboard_nav.enabled = true;
    config.keyboard_nav.tab_order = true;
    config.keyboard_nav.arrow_key_navigation = true;
    config.keyboard_nav.focus_indicators = true;
    config.keyboard_nav.skip_links = true;

    let performance_config = PerformanceConfig::default();
    let accessibility_system = AccessibilitySystem::new(config, performance_config);

    let chart_spec = ChartSpec {
        transform: None,
        selection: None,
        intelligence: None,
    };

    // Test keyboard navigation setup
    let keyboard_map = accessibility_system.generate_keyboard_map(&chart_spec);

    assert!(keyboard_map.contains_key("zoom_in"));
    assert!(keyboard_map.contains_key("zoom_out"));
    assert!(keyboard_map.contains_key("reset_view"));
    assert!(keyboard_map.contains_key("toggle_data_table"));

    assert_eq!(keyboard_map.get("zoom_in").unwrap(), "ctrl+plus");
    assert_eq!(keyboard_map.get("zoom_out").unwrap(), "ctrl+minus");
    assert_eq!(keyboard_map.get("reset_view").unwrap(), "ctrl+0");
    assert_eq!(keyboard_map.get("toggle_data_table").unwrap(), "ctrl+t");
}

#[tokio::test]
async fn test_color_vision_support() {
    let mut config = AccessibilityConfig::default();
    config.color_vision.enabled = true;
    config.color_vision.high_contrast_mode = true;
    config.color_vision.colorblind_friendly_palette = true;
    config.color_vision.pattern_support = true;
    config.color_vision.minimum_contrast_ratio = 4.5;

    let performance_config = PerformanceConfig::default();
    let accessibility_system = AccessibilitySystem::new(config, performance_config);

    // Test color palette validation
    let color_palette = vec![
        "#FF6B6B".to_string(), // Red
        "#4ECDC4".to_string(), // Teal
        "#45B7D1".to_string(), // Blue
        "#96CEB4".to_string(), // Green
        "#FFEAA7".to_string(), // Yellow
    ];

    let validation_result = accessibility_system
        .validate_color_palette(&color_palette)
        .unwrap();

    assert!(validation_result.is_colorblind_friendly);
    assert!(validation_result.meets_contrast_requirements);
    assert!(validation_result.recommendations.is_empty());

    // Test contrast ratio calculation
    let contrast_ratio = accessibility_system
        .calculate_contrast_ratio("#000000", "#FFFFFF")
        .unwrap();

    assert_eq!(contrast_ratio, 21.0); // Perfect contrast

    // Test high contrast mode
    let high_contrast_palette = accessibility_system
        .generate_high_contrast_palette()
        .unwrap();

    assert_eq!(high_contrast_palette.len(), 5);
    assert!(high_contrast_palette.contains(&"#000000".to_string())); // Black
    assert!(high_contrast_palette.contains(&"#FFFFFF".to_string())); // White
}

#[tokio::test]
async fn test_motion_preferences_support() {
    let mut config = AccessibilityConfig::default();
    config.motion.respect_prefers_reduced_motion = true;
    config.motion.reduce_animations = false;
    config.motion.static_alternative = false;

    let performance_config = PerformanceConfig::default();
    let accessibility_system = AccessibilitySystem::new(config, performance_config);

    // Test motion preference detection
    let motion_preference = accessibility_system.detect_motion_preference().unwrap();

    assert!(matches!(motion_preference, MotionPreference::NoPreference));

    // Test reduced motion adaptations
    let adaptations = accessibility_system
        .get_motion_adaptations(MotionPreference::Reduce)
        .unwrap();

    assert!(adaptations.reduce_animations);
    assert!(adaptations.simplify_transitions);
    assert!(adaptations.disable_parallax);
    assert!(adaptations.reduce_motion_duration);

    // Test animation configuration
    let animation_config = accessibility_system
        .configure_animations(MotionPreference::Reduce)
        .unwrap();

    assert_eq!(animation_config.duration, 0.0); // No animations
    assert!(!animation_config.enable_transitions);
    assert!(!animation_config.enable_parallax);
}

#[tokio::test]
async fn test_focus_management() {
    let mut config = AccessibilityConfig::default();
    config.focus_management.enabled = true;
    config.focus_management.visible_focus_indicator = true;
    config.focus_management.focus_trap = true;
    config.focus_management.restore_focus = true;

    let performance_config = PerformanceConfig::default();
    let accessibility_system = AccessibilitySystem::new(config, performance_config);

    let chart_spec = ChartSpec {
        transform: None,
        selection: None,
        intelligence: None,
    };

    // Test focus management setup
    let focus_config = accessibility_system
        .setup_focus_management(&chart_spec)
        .unwrap();

    assert!(focus_config.visible_focus);
    assert!(focus_config.focus_trapping);
    assert!(focus_config.focus_restoration);
    assert!(focus_config.tab_index >= 0);

    // Test focus indicators
    let focus_indicators = accessibility_system.generate_focus_indicators().unwrap();

    assert!(focus_indicators.contains("outline"));
    assert!(focus_indicators.contains("box-shadow"));
    assert!(focus_indicators.contains("2px solid"));
}

#[tokio::test]
async fn test_alternative_formats() {
    let mut config = AccessibilityConfig::default();
    config.alternative_formats.data_tables = true;
    config.alternative_formats.text_descriptions = true;
    config.alternative_formats.sonification = true;

    let performance_config = PerformanceConfig::default();
    let accessibility_system = AccessibilitySystem::new(config, performance_config);

    let data = df! [
        "category" => ["A", "B", "C", "D"],
        "value" => [25, 30, 20, 25],
        "percentage" => [25.0, 30.0, 20.0, 25.0]
    ]
    .unwrap();

    let chart_spec = ChartSpec {
        transform: None,
        selection: None,
        intelligence: None,
    };

    // Test data table generation
    let data_table = accessibility_system
        .create_data_table(&chart_spec, &data)
        .unwrap();

    assert_eq!(data_table.rows.len(), 4);
    assert_eq!(data_table.columns.len(), 3);
    assert!(data_table.columns.contains(&"category".to_string()));
    assert!(data_table.columns.contains(&"value".to_string()));
    assert!(data_table.columns.contains(&"percentage".to_string()));

    // Test text summary generation
    let text_summary = accessibility_system
        .generate_text_summary(&chart_spec, &data)
        .unwrap();

    assert!(!text_summary.is_empty());
    assert!(text_summary.contains("4 categories"));
    assert!(text_summary.contains("highest value"));
    assert!(text_summary.contains("lowest value"));

    // Test audio description generation
    let audio_description = accessibility_system
        .generate_audio_description(&chart_spec, &data)
        .unwrap();

    assert!(!audio_description.is_empty());
    assert!(audio_description.contains("chart"));
    assert!(audio_description.contains("4 segments"));
}

#[tokio::test]
async fn test_performance_accessibility_budget() {
    let mut config = AccessibilityConfig::default();
    let performance_config = PerformanceConfig {
        max_render_time: 100.0,  // 100ms
        max_memory_usage: 50.0,  // 50MB
        max_bundle_size: 1000.0, // 1MB
        enable_monitoring: true,
    };

    let accessibility_system = AccessibilitySystem::new(config, performance_config);

    // Test performance budget validation
    let performance_metrics = PerformanceMetrics {
        fps: 60.0,
        interaction_delay_ms: 80.0,
        cache_hit_rate: 0.95,
        budget_compliance: true,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    let budget_result = accessibility_system
        .validate_performance_budget(&performance_metrics)
        .unwrap();

    assert!(budget_result.within_budget);
    assert!(budget_result.render_time_ok);
    assert!(budget_result.memory_usage_ok);
    assert!(budget_result.bundle_size_ok);
    assert!(budget_result.recommendations.is_empty());

    // Test performance optimization suggestions
    let optimization_suggestions = accessibility_system
        .get_performance_optimizations(&performance_metrics)
        .unwrap();

    assert!(!optimization_suggestions.is_empty());
    assert!(optimization_suggestions
        .iter()
        .any(|s| s.contains("lazy loading")));
    assert!(optimization_suggestions
        .iter()
        .any(|s| s.contains("code splitting")));
}

#[tokio::test]
async fn test_comprehensive_accessibility_workflow() {
    // Create comprehensive accessibility configuration
    let config = AccessibilityConfig {
        wcag_level: WCAGLevel::AA,
        screen_reader: ScreenReaderSupport {
            enabled: true,
            generate_alt_text: true,
            create_data_tables: true,
            provide_summaries: true,
            announce_updates: true,
            aria_labels: true,
            structured_navigation: true,
        },
        keyboard_nav: KeyboardNavigation {
            enabled: true,
            tab_order: true,
            arrow_key_navigation: true,
            skip_links: true,
            focus_indicators: true,
            escape_handling: true,
            custom_shortcuts: {
                let mut shortcuts = HashMap::new();
                shortcuts.insert("zoom_in".to_string(), "ctrl+plus".to_string());
                shortcuts.insert("zoom_out".to_string(), "ctrl+minus".to_string());
                shortcuts.insert("reset_view".to_string(), "ctrl+0".to_string());
                shortcuts.insert("toggle_data_table".to_string(), "ctrl+t".to_string());
                shortcuts
            },
        },
        color_vision: ColorVisionSupport {
            enabled: true,
            high_contrast_mode: true,
            colorblind_friendly_palette: true,
            pattern_support: true,
            texture_support: false,
            minimum_contrast_ratio: 4.5,
        },
        motion: MotionPreferences {
            respect_prefers_reduced_motion: true,
            reduce_animations: false,
            static_alternative: false,
            disable_auto_play: true,
            animation_duration_multiplier: 1.0,
        },
        focus_management: FocusManagement {
            enabled: true,
            visible_focus_indicator: true,
            focus_trap: true,
            restore_focus: true,
            focus_order: vec![
                "chart".to_string(),
                "controls".to_string(),
                "data_table".to_string(),
            ],
        },
        alternative_formats: AlternativeFormats {
            data_tables: true,
            text_descriptions: true,
            sonification: true,
            tactile_graphics: false,
            high_contrast_version: true,
        },
    };

    let performance_config = PerformanceConfig {
        max_render_time: 100.0,
        max_memory_usage: 50.0,
        max_bundle_size: 1000.0,
        enable_monitoring: true,
    };

    let accessibility_system = AccessibilitySystem::new(config, performance_config);

    // Create test data and chart
    let data = df! [
        "quarter" => ["Q1", "Q2", "Q3", "Q4"],
        "revenue" => [100000, 120000, 110000, 130000],
        "profit" => [20000, 25000, 22000, 28000]
    ]
    .unwrap();

    let chart_spec = ChartSpec {
        transform: None,
        selection: None,
        intelligence: None,
    };

    // Step 1: Validate WCAG 2.1 AA compliance
    let compliance_result = accessibility_system
        .validate_wcag_compliance(&chart_spec, &data)
        .unwrap();

    assert!(compliance_result.is_compliant);
    assert_eq!(compliance_result.level, WCAGLevel::AA);

    // Step 2: Generate screen reader support
    let alt_text = accessibility_system
        .generate_alt_text(&chart_spec, &data)
        .unwrap();

    let data_table = accessibility_system
        .create_data_table(&chart_spec, &data)
        .unwrap();

    assert!(!alt_text.is_empty());
    assert_eq!(data_table.rows.len(), 4);

    // Step 3: Setup keyboard navigation
    let keyboard_map = accessibility_system.generate_keyboard_map(&chart_spec);

    assert!(keyboard_map.contains_key("zoom_in"));
    assert!(keyboard_map.contains_key("zoom_out"));

    // Step 4: Validate color accessibility
    let color_palette = vec![
        "#1f77b4".to_string(), // Blue
        "#ff7f0e".to_string(), // Orange
        "#2ca02c".to_string(), // Green
        "#d62728".to_string(), // Red
    ];

    let color_validation = accessibility_system
        .validate_color_palette(&color_palette)
        .unwrap();

    assert!(color_validation.is_colorblind_friendly);

    // Step 5: Setup focus management
    let focus_config = accessibility_system
        .setup_focus_management(&chart_spec)
        .unwrap();

    assert!(focus_config.visible_focus);
    assert!(focus_config.focus_trapping);

    // Step 6: Generate alternative formats
    let text_summary = accessibility_system
        .generate_text_summary(&chart_spec, &data)
        .unwrap();

    let audio_description = accessibility_system
        .generate_audio_description(&chart_spec, &data)
        .unwrap();

    assert!(!text_summary.is_empty());
    assert!(!audio_description.is_empty());

    // Step 7: Validate performance budget
    let performance_metrics = PerformanceMetrics {
        fps: 60.0,
        interaction_delay_ms: 85.0,
        cache_hit_rate: 0.95,
        budget_compliance: true,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    let budget_result = accessibility_system
        .validate_performance_budget(&performance_metrics)
        .unwrap();

    assert!(budget_result.within_budget);
    assert!(budget_result.render_time_ok);
    assert!(budget_result.memory_usage_ok);
    assert!(budget_result.bundle_size_ok);

    // Step 8: Generate comprehensive accessibility report
    let accessibility_report = accessibility_system
        .generate_accessibility_report(&chart_spec, &data)
        .unwrap();

    assert!(accessibility_report.contains("WCAG 2.1 AA"));
    assert!(accessibility_report.contains("Screen Reader"));
    assert!(accessibility_report.contains("Keyboard Navigation"));
    assert!(accessibility_report.contains("Color Vision"));
    assert!(accessibility_report.contains("Performance"));
}
