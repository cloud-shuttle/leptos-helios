//! Theme Engine Tests
//!
//! Comprehensive test suite for the Helios theme engine, including
//! theme validation, CSS compilation, custom components, animations, and responsive design.

use leptos_helios::animation_engine::*;
use leptos_helios::custom_components::*;
use leptos_helios::responsive_design::*;
use leptos_helios::theme_engine::*;
use std::collections::HashMap;

#[test]
fn test_theme_registry_creation() {
    let registry = ThemeRegistry::new();
    assert_eq!(registry.list_themes().len(), 0);
    assert_eq!(registry.get_active_theme().unwrap().id.0, "default");
}

#[test]
fn test_theme_registration() {
    let mut registry = ThemeRegistry::new();
    let theme = create_default_theme();

    let result = registry.register_theme(theme.clone());
    assert!(result.is_ok());

    let registered_theme = registry.get_theme(&theme.id).unwrap();
    assert_eq!(registered_theme.name, theme.name);
}

#[test]
fn test_theme_validation() {
    let mut registry = ThemeRegistry::new();

    // Test invalid theme ID
    let mut invalid_theme = create_default_theme();
    invalid_theme.id = ThemeId("".to_string());

    let result = registry.register_theme(invalid_theme);
    assert!(result.is_err());

    // Test invalid theme name
    let mut invalid_theme = create_default_theme();
    invalid_theme.name = "".to_string();

    let result = registry.register_theme(invalid_theme);
    assert!(result.is_err());

    // Test invalid color format
    let mut invalid_theme = create_default_theme();
    invalid_theme.variables.colors.primary = "invalid-color".to_string();

    let result = registry.register_theme(invalid_theme);
    assert!(result.is_err());
}

#[test]
fn test_theme_activation() {
    let mut registry = ThemeRegistry::new();
    let theme = create_default_theme();

    registry.register_theme(theme.clone()).unwrap();
    registry.set_active_theme(&theme.id).unwrap();

    let active_theme = registry.get_active_theme().unwrap();
    assert_eq!(active_theme.id, theme.id);
}

#[test]
fn test_style_compilation() {
    let mut compiler = StyleCompiler::new();
    let theme = create_default_theme();

    let compiled_style = compiler.compile_theme(&theme).unwrap();

    // Check that CSS variables are generated
    assert!(compiled_style.variables.contains_key("color-primary"));
    assert!(compiled_style.variables.contains_key("font-family"));
    assert!(compiled_style.variables.contains_key("spacing-md"));

    // Check that CSS is generated
    assert!(!compiled_style.css.is_empty());
    assert!(compiled_style.css.contains(":root"));
    assert!(compiled_style.css.contains("--color-primary"));
}

#[test]
fn test_theme_engine_integration() {
    let mut engine = ThemeEngine::new();
    let theme = create_default_theme();

    engine.register_theme(theme.clone()).unwrap();
    engine.set_active_theme(&theme.id).unwrap();

    let compiled_style = engine.compile_active_theme().unwrap();
    assert!(!compiled_style.css.is_empty());

    let cache_stats = engine.get_cache_stats();
    assert!(cache_stats.entry_count > 0);
}

#[test]
fn test_custom_component_registration() {
    let mut factory = ComponentFactory::new();
    let component = create_sample_component();

    let result = factory.register_component(component.clone());
    assert!(result.is_ok());

    let registered_component = factory.get_component(&component.id).unwrap();
    assert_eq!(registered_component.name, component.name);
}

#[test]
fn test_custom_component_validation() {
    let mut factory = ComponentFactory::new();

    // Test invalid component ID
    let mut invalid_component = create_sample_component();
    invalid_component.id = ComponentId("".to_string());

    let result = factory.register_component(invalid_component);
    assert!(result.is_err());

    // Test invalid component name
    let mut invalid_component = create_sample_component();
    invalid_component.name = "".to_string();

    let result = factory.register_component(invalid_component);
    assert!(result.is_err());

    // Test invalid template
    let mut invalid_component = create_sample_component();
    invalid_component.template = "".to_string();

    let result = factory.register_component(invalid_component);
    assert!(result.is_err());
}

#[test]
fn test_custom_component_rendering() {
    let factory = ComponentFactory::new();
    let component = create_sample_component();

    let mut props = HashMap::new();
    props.insert(
        "title".to_string(),
        ThemeValue::String("Test Chart".to_string()),
    );
    props.insert("width".to_string(), ThemeValue::Number(500.0));
    props.insert("height".to_string(), ThemeValue::Number(400.0));
    props.insert(
        "color".to_string(),
        ThemeValue::Color("#ff0000".to_string()),
    );

    let rendered = factory.render_component(&component.id, &props).unwrap();
    assert!(rendered.contains("Test Chart"));
    assert!(rendered.contains("500px"));
    assert!(rendered.contains("400px"));
    assert!(rendered.contains("#ff0000"));
}

#[test]
fn test_animation_registration() {
    let mut scheduler = AnimationScheduler::new();
    let animation = create_sample_animation();

    let result = scheduler.register_animation(animation.clone());
    assert!(result.is_ok());
}

#[test]
fn test_animation_validation() {
    let mut scheduler = AnimationScheduler::new();

    // Test invalid animation ID
    let mut invalid_animation = create_sample_animation();
    invalid_animation.id = AnimationId("".to_string());

    let result = scheduler.register_animation(invalid_animation);
    assert!(result.is_err());

    // Test invalid animation name
    let mut invalid_animation = create_sample_animation();
    invalid_animation.name = "".to_string();

    let result = scheduler.register_animation(invalid_animation);
    assert!(result.is_err());

    // Test zero duration
    let mut invalid_animation = create_sample_animation();
    invalid_animation.duration = std::time::Duration::ZERO;

    let result = scheduler.register_animation(invalid_animation);
    assert!(result.is_err());

    // Test empty keyframes
    let mut invalid_animation = create_sample_animation();
    invalid_animation.keyframes = vec![];

    let result = scheduler.register_animation(invalid_animation);
    assert!(result.is_err());
}

#[test]
fn test_animation_execution() {
    let mut scheduler = AnimationScheduler::new();
    let animation = create_sample_animation();

    scheduler.register_animation(animation.clone()).unwrap();
    scheduler.start_animation(&animation.id).unwrap();

    let state = scheduler.get_animation_state(&animation.id).unwrap();
    assert!(state.is_playing);
    assert!(!state.is_paused);
    assert!(!state.is_finished);

    // Simulate time passing
    scheduler
        .update(std::time::Duration::from_millis(100))
        .unwrap();

    let state = scheduler.get_animation_state(&animation.id).unwrap();
    assert!(state.progress > 0.0);
}

#[test]
fn test_animation_pause_resume() {
    let mut scheduler = AnimationScheduler::new();
    let animation = create_sample_animation();

    scheduler.register_animation(animation.clone()).unwrap();
    scheduler.start_animation(&animation.id).unwrap();

    // Pause animation
    scheduler.pause_animation(&animation.id).unwrap();
    let state = scheduler.get_animation_state(&animation.id).unwrap();
    assert!(!state.is_playing);
    assert!(state.is_paused);

    // Resume animation
    scheduler.resume_animation(&animation.id).unwrap();
    let state = scheduler.get_animation_state(&animation.id).unwrap();
    assert!(state.is_playing);
    assert!(!state.is_paused);
}

#[test]
fn test_easing_functions() {
    let scheduler = AnimationScheduler::new();

    // Test linear easing
    let linear_progress = scheduler.apply_easing(0.5, &EasingFunction::Linear);
    assert_eq!(linear_progress, 0.5);

    // Test ease-in
    let ease_in_progress = scheduler.apply_easing(0.5, &EasingFunction::EaseIn);
    assert!(ease_in_progress < 0.5);

    // Test ease-out
    let ease_out_progress = scheduler.apply_easing(0.5, &EasingFunction::EaseOut);
    assert!(ease_out_progress > 0.5);

    // Test ease-in-out
    let ease_in_out_progress = scheduler.apply_easing(0.5, &EasingFunction::EaseInOut);
    assert_eq!(ease_in_out_progress, 0.5);
}

#[test]
fn test_responsive_manager_creation() {
    let manager = ResponsiveManager::new();
    let state = manager.get_state();

    assert_eq!(state.viewport.width, 1024);
    assert_eq!(state.viewport.height, 768);
    assert_eq!(state.device_type, DeviceType::Desktop);
    assert_eq!(state.orientation, Orientation::Landscape);
}

#[test]
fn test_viewport_update() {
    let mut manager = ResponsiveManager::new();

    // Update to mobile viewport
    let mobile_viewport = ViewportSize {
        width: 375,
        height: 667,
    };
    manager.update_viewport(mobile_viewport).unwrap();

    let state = manager.get_state();
    assert_eq!(state.viewport.width, 375);
    assert_eq!(state.viewport.height, 667);
    assert_eq!(state.device_type, DeviceType::Mobile);
    assert_eq!(state.orientation, Orientation::Portrait);
}

#[test]
fn test_breakpoint_detection() {
    let mut manager = ResponsiveManager::new();

    // Test mobile breakpoint
    let mobile_viewport = ViewportSize {
        width: 375,
        height: 667,
    };
    manager.update_viewport(mobile_viewport).unwrap();

    let active_breakpoint = manager.get_active_breakpoint().unwrap();
    assert_eq!(active_breakpoint.name, "mobile");

    // Test tablet breakpoint
    let tablet_viewport = ViewportSize {
        width: 768,
        height: 1024,
    };
    manager.update_viewport(tablet_viewport).unwrap();

    let active_breakpoint = manager.get_active_breakpoint().unwrap();
    assert_eq!(active_breakpoint.name, "tablet");

    // Test desktop breakpoint
    let desktop_viewport = ViewportSize {
        width: 1024,
        height: 768,
    };
    manager.update_viewport(desktop_viewport).unwrap();

    let active_breakpoint = manager.get_active_breakpoint().unwrap();
    assert_eq!(active_breakpoint.name, "desktop");
}

#[test]
fn test_media_query_evaluation() {
    let mut manager = ResponsiveManager::new();

    // Test mobile viewport
    let mobile_viewport = ViewportSize {
        width: 375,
        height: 667,
    };
    manager.update_viewport(mobile_viewport).unwrap();

    assert!(manager.matches_media_query("mobile").unwrap());
    assert!(!manager.matches_media_query("desktop").unwrap());
    assert!(manager.matches_media_query("portrait").unwrap());
    assert!(!manager.matches_media_query("landscape").unwrap());

    // Test desktop viewport
    let desktop_viewport = ViewportSize {
        width: 1024,
        height: 768,
    };
    manager.update_viewport(desktop_viewport).unwrap();

    assert!(!manager.matches_media_query("mobile").unwrap());
    assert!(manager.matches_media_query("desktop").unwrap());
    assert!(!manager.matches_media_query("portrait").unwrap());
    assert!(manager.matches_media_query("landscape").unwrap());
}

#[test]
fn test_responsive_styles() {
    let mut manager = ResponsiveManager::new();

    let mut base_styles = HashMap::new();
    base_styles.insert(
        "font-size".to_string(),
        ThemeValue::String("16px".to_string()),
    );
    base_styles.insert(
        "color".to_string(),
        ThemeValue::Color("#000000".to_string()),
    );

    // Test mobile styles
    let mobile_viewport = ViewportSize {
        width: 375,
        height: 667,
    };
    manager.update_viewport(mobile_viewport).unwrap();

    let responsive_styles = manager.get_responsive_styles(&base_styles);
    assert_eq!(
        responsive_styles.get("font-size").unwrap(),
        &ThemeValue::String("16px".to_string())
    );
    assert_eq!(
        responsive_styles.get("device-type").unwrap(),
        &ThemeValue::String("Mobile".to_string())
    );
    assert_eq!(
        responsive_styles.get("orientation").unwrap(),
        &ThemeValue::String("Portrait".to_string())
    );
}

#[test]
fn test_layout_calculation() {
    let mut manager = ResponsiveManager::new();

    // Set up layout configuration
    let layout_config = LayoutConfig {
        container_width: None,
        container_height: None,
        padding: 16.0,
        margin: 8.0,
        grid_columns: 1,
        grid_gap: 8.0,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::FlexStart,
        align_items: AlignItems::Stretch,
        flex_wrap: FlexWrap::Wrap,
    };

    manager.set_layout_config(DeviceType::Mobile, layout_config);

    // Create layout elements
    let elements = vec![
        LayoutElement {
            id: "element1".to_string(),
            size: None,
            flex_grow: 1.0,
            flex_shrink: 1.0,
            flex_basis: None,
            z_index: 1,
            min_size: None,
            max_size: None,
        },
        LayoutElement {
            id: "element2".to_string(),
            size: None,
            flex_grow: 1.0,
            flex_shrink: 1.0,
            flex_basis: None,
            z_index: 1,
            min_size: None,
            max_size: None,
        },
    ];

    let layout_result = manager.calculate_layout(&elements).unwrap();

    assert_eq!(layout_result.element_positions.len(), 2);
    assert!(layout_result.element_positions.contains_key("element1"));
    assert!(layout_result.element_positions.contains_key("element2"));

    let element1_pos = layout_result.element_positions.get("element1").unwrap();
    assert!(element1_pos.x >= 0.0);
    assert!(element1_pos.y >= 0.0);
    assert!(element1_pos.width > 0.0);
    assert!(element1_pos.height > 0.0);
}

#[test]
fn test_theme_variables_generation() {
    let theme = create_default_theme();
    let mut compiler = StyleCompiler::new();

    let compiled_style = compiler.compile_theme(&theme).unwrap();

    // Check color variables
    assert_eq!(
        compiled_style.variables.get("color-primary").unwrap(),
        "#3b82f6"
    );
    assert_eq!(
        compiled_style.variables.get("color-secondary").unwrap(),
        "#64748b"
    );
    assert_eq!(
        compiled_style.variables.get("color-accent").unwrap(),
        "#f59e0b"
    );

    // Check typography variables
    assert_eq!(
        compiled_style.variables.get("font-family").unwrap(),
        "Inter, system-ui, sans-serif"
    );
    assert_eq!(
        compiled_style.variables.get("font-size-base").unwrap(),
        "14px"
    );
    assert_eq!(
        compiled_style.variables.get("font-size-small").unwrap(),
        "12px"
    );
    assert_eq!(
        compiled_style.variables.get("font-size-large").unwrap(),
        "16px"
    );

    // Check spacing variables
    assert_eq!(compiled_style.variables.get("spacing-xs").unwrap(), "4px");
    assert_eq!(compiled_style.variables.get("spacing-sm").unwrap(), "8px");
    assert_eq!(compiled_style.variables.get("spacing-md").unwrap(), "16px");
    assert_eq!(compiled_style.variables.get("spacing-lg").unwrap(), "24px");
    assert_eq!(compiled_style.variables.get("spacing-xl").unwrap(), "32px");
    assert_eq!(compiled_style.variables.get("spacing-xxl").unwrap(), "48px");
}

#[test]
fn test_animation_performance_limits() {
    let mut scheduler = AnimationScheduler::with_config(PerformanceConfig {
        max_animations: 2,
        target_fps: 60,
        max_frame_time: std::time::Duration::from_millis(16),
        enable_performance_monitoring: true,
        auto_pause_on_hidden: true,
    });

    // Register first animation
    let animation1 = create_sample_animation();
    scheduler.register_animation(animation1).unwrap();

    // Register second animation
    let mut animation2 = create_sample_animation();
    animation2.id = AnimationId("animation2".to_string());
    scheduler.register_animation(animation2).unwrap();

    // Try to register third animation (should fail)
    let mut animation3 = create_sample_animation();
    animation3.id = AnimationId("animation3".to_string());
    let result = scheduler.register_animation(animation3);
    assert!(result.is_err());

    let stats = scheduler.get_performance_stats();
    assert_eq!(stats.total_animations, 2);
    assert_eq!(stats.target_fps, 60);
}

#[test]
fn test_component_property_validation() {
    let factory = ComponentFactory::new();
    let component = create_sample_component();

    // Test valid props
    let mut valid_props = HashMap::new();
    valid_props.insert("title".to_string(), ThemeValue::String("Test".to_string()));
    valid_props.insert("width".to_string(), ThemeValue::Number(500.0));
    valid_props.insert("height".to_string(), ThemeValue::Number(400.0));
    valid_props.insert(
        "color".to_string(),
        ThemeValue::Color("#ff0000".to_string()),
    );

    let result = factory.render_component(&component.id, &valid_props);
    assert!(result.is_ok());

    // Test invalid width (below minimum)
    let mut invalid_props = valid_props.clone();
    invalid_props.insert("width".to_string(), ThemeValue::Number(50.0)); // Below minimum of 100

    let result = factory.render_component(&component.id, &invalid_props);
    assert!(result.is_err());

    // Test invalid width (above maximum)
    let mut invalid_props = valid_props.clone();
    invalid_props.insert("width".to_string(), ThemeValue::Number(3000.0)); // Above maximum of 2000

    let result = factory.render_component(&component.id, &invalid_props);
    assert!(result.is_err());
}

#[test]
fn test_responsive_breakpoint_priority() {
    let mut manager = ResponsiveManager::new();

    // Test edge case at breakpoint boundary
    let boundary_viewport = ViewportSize {
        width: 768,
        height: 1024,
    };
    manager.update_viewport(boundary_viewport).unwrap();

    let active_breakpoint = manager.get_active_breakpoint().unwrap();
    assert_eq!(active_breakpoint.name, "tablet");

    // Test just below boundary
    let below_boundary = ViewportSize {
        width: 767,
        height: 1024,
    };
    manager.update_viewport(below_boundary).unwrap();

    let active_breakpoint = manager.get_active_breakpoint().unwrap();
    assert_eq!(active_breakpoint.name, "mobile");

    // Test just above boundary
    let above_boundary = ViewportSize {
        width: 769,
        height: 1024,
    };
    manager.update_viewport(above_boundary).unwrap();

    let active_breakpoint = manager.get_active_breakpoint().unwrap();
    assert_eq!(active_breakpoint.name, "tablet");
}

#[test]
fn test_theme_engine_cache_management() {
    let mut engine = ThemeEngine::new();
    let theme = create_default_theme();

    engine.register_theme(theme.clone()).unwrap();
    engine.set_active_theme(&theme.id).unwrap();

    // Compile theme multiple times
    let _style1 = engine.compile_active_theme().unwrap();
    let _style2 = engine.compile_active_theme().unwrap();
    let _style3 = engine.compile_active_theme().unwrap();

    let cache_stats = engine.get_cache_stats();
    assert!(cache_stats.entry_count > 0);
    assert!(cache_stats.total_size > 0);

    // Clear cache
    engine.clear_cache();
    let cache_stats = engine.get_cache_stats();
    assert_eq!(cache_stats.entry_count, 0);
    assert_eq!(cache_stats.total_size, 0);
}
