//! Enhanced Styling Tests
//! Tests for visual customization options and theming support

use helios_core::*;
use std::collections::HashMap;

/// Helper function to create a base chart config
fn create_base_config(title: &str, width: u32, height: u32) -> BaseChartConfig {
    BaseChartConfig {
        width,
        height,
        title: title.to_string(),
        x_label: "X Axis".to_string(),
        y_label: "Y Axis".to_string(),
        show_grid: true,
        background_color: "#ffffff".to_string(),
        text_color: "#000000".to_string(),
    }
}

#[test]
fn test_theme_system() {
    // Given: A theme system with predefined themes
    let light_theme = Theme {
        name: "light".to_string(),
        colors: ThemeColors {
            background: "#ffffff".to_string(),
            foreground: "#000000".to_string(),
            primary: "#007acc".to_string(),
            secondary: "#6c757d".to_string(),
            success: "#28a745".to_string(),
            warning: "#ffc107".to_string(),
            error: "#dc3545".to_string(),
            info: "#17a2b8".to_string(),
        },
        typography: Typography {
            font_family: "Inter, sans-serif".to_string(),
            font_size_base: 14.0,
            font_size_small: 12.0,
            font_size_large: 16.0,
            font_weight_normal: 400,
            font_weight_bold: 700,
            line_height: 1.5,
        },
        spacing: Spacing {
            xs: 4.0,
            sm: 8.0,
            md: 16.0,
            lg: 24.0,
            xl: 32.0,
        },
        border_radius: BorderRadius {
            sm: 2.0,
            md: 4.0,
            lg: 8.0,
            xl: 12.0,
        },
        shadows: Shadows {
            sm: "0 1px 2px rgba(0, 0, 0, 0.05)".to_string(),
            md: "0 4px 6px rgba(0, 0, 0, 0.1)".to_string(),
            lg: "0 10px 15px rgba(0, 0, 0, 0.1)".to_string(),
        },
    };

    let dark_theme = Theme {
        name: "dark".to_string(),
        colors: ThemeColors {
            background: "#1a1a1a".to_string(),
            foreground: "#ffffff".to_string(),
            primary: "#4dabf7".to_string(),
            secondary: "#868e96".to_string(),
            success: "#51cf66".to_string(),
            warning: "#ffd43b".to_string(),
            error: "#ff6b6b".to_string(),
            info: "#74c0fc".to_string(),
        },
        typography: light_theme.typography.clone(),
        spacing: light_theme.spacing.clone(),
        border_radius: light_theme.border_radius.clone(),
        shadows: light_theme.shadows.clone(),
    };

    let mut theme_manager = ThemeManager::new().unwrap();
    theme_manager.register_theme(light_theme.clone()).unwrap();
    theme_manager.register_theme(dark_theme.clone()).unwrap();

    // When: Applying themes to charts
    let config = LineChartConfig {
        base: create_base_config("Themed Chart", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let light_styled_config = theme_manager.apply_theme(&config, "light").unwrap();
    let dark_styled_config = theme_manager.apply_theme(&config, "dark").unwrap();

    // Then: Should apply theme styles
    assert_eq!(light_styled_config.base.background_color, "#ffffff");
    assert_eq!(light_styled_config.base.text_color, "#000000");
    assert_eq!(dark_styled_config.base.background_color, "#1a1a1a");
    assert_eq!(dark_styled_config.base.text_color, "#ffffff");
}

#[test]
fn test_custom_color_palettes() {
    // Given: Custom color palettes
    let palette_config = ColorPaletteConfig {
        name: "custom_palette".to_string(),
        colors: vec![
            "#ff6b6b".to_string(),
            "#4ecdc4".to_string(),
            "#45b7d1".to_string(),
            "#96ceb4".to_string(),
            "#feca57".to_string(),
            "#ff9ff3".to_string(),
        ],
        color_blind_safe: true,
        accessibility_contrast: true,
    };

    let mut palette_manager = ColorPaletteManager::new().unwrap();
    palette_manager
        .register_palette(palette_config.clone())
        .unwrap();

    // When: Using custom palette in chart
    let data = vec![
        ("Category A", 25.0),
        ("Category B", 40.0),
        ("Category C", 30.0),
        ("Category D", 35.0),
    ];

    let config = BarChartConfig {
        base: create_base_config("Custom Palette Chart", 800, 600),
        colors: vec![], // Will be filled by palette
        bar_width: 0.8,
        show_values: true,
        horizontal: false,
        show_legend: true,
        corner_radius: Some(4.0),
        spacing: Some(0.1),
    };

    let styled_config = palette_manager
        .apply_palette(&config, "custom_palette")
        .unwrap();

    // Then: Should apply custom colors
    assert_eq!(styled_config.colors.len(), 4);
    assert_eq!(styled_config.colors[0], "#ff6b6b");
    assert_eq!(styled_config.colors[1], "#4ecdc4");
}

#[test]
fn test_advanced_styling_options() {
    // Given: Advanced styling configuration
    let styling_config = AdvancedStylingConfig {
        animations: AnimationConfig {
            enabled: true,
            duration: Duration::from_millis(500),
            easing: EasingType::EaseOutCubic,
            stagger_delay: Duration::from_millis(50),
        },
        effects: EffectsConfig {
            gradients: true,
            shadows: true,
            blur: true,
            glow: true,
        },
        layout: LayoutConfig {
            padding: PaddingConfig {
                top: 20.0,
                right: 20.0,
                bottom: 20.0,
                left: 20.0,
            },
            margin: MarginConfig {
                top: 10.0,
                right: 10.0,
                bottom: 10.0,
                left: 10.0,
            },
            grid: GridConfig {
                show: true,
                color: "#e0e0e0".to_string(),
                width: 1.0,
                style: GridStyle::Solid,
            },
        },
        typography: TypographyConfig {
            title: TitleConfig {
                font_size: 24.0,
                font_weight: 700,
                color: "#333333".to_string(),
                margin_bottom: 16.0,
            },
            axis_labels: AxisLabelConfig {
                font_size: 12.0,
                font_weight: 500,
                color: "#666666".to_string(),
                rotation: 0.0,
            },
            legend: LegendConfig {
                font_size: 11.0,
                font_weight: 400,
                color: "#555555".to_string(),
                position: LegendPosition::TopRight,
            },
        },
    };

    let mut styling_manager = AdvancedStylingManager::new().unwrap();
    styling_manager
        .set_styling_config(styling_config.clone())
        .unwrap();

    // When: Applying advanced styling
    let config = AreaChartConfig {
        base: create_base_config("Advanced Styled Chart", 800, 600),
        fill_color: "#00d4ff".to_string(),
        stroke_color: "#0066cc".to_string(),
        stroke_width: 2.0,
        opacity: 0.7,
        interpolation: InterpolationType::Smooth,
        show_legend: true,
        gradient: Some(GradientConfig {
            start_color: "#00d4ff".to_string(),
            end_color: "#0066cc".to_string(),
            direction: GradientDirection::Vertical,
        }),
    };

    let styled_config = styling_manager.apply_styling(&config).unwrap();

    // Then: Should apply advanced styling
    assert!(styled_config.gradient.is_some());
    assert_eq!(styled_config.opacity, 0.7);
}

#[test]
fn test_responsive_design() {
    // Given: Responsive design configuration
    let responsive_config = ResponsiveConfig {
        breakpoints: vec![
            Breakpoint {
                name: "mobile".to_string(),
                min_width: 0,
                max_width: 768,
            },
            Breakpoint {
                name: "tablet".to_string(),
                min_width: 769,
                max_width: 1024,
            },
            Breakpoint {
                name: "desktop".to_string(),
                min_width: 1025,
                max_width: 0,
            },
        ],
        adaptive_layout: true,
        fluid_typography: true,
        responsive_colors: true,
    };

    let mut responsive_manager = ResponsiveManager::new().unwrap();
    responsive_manager
        .set_responsive_config(responsive_config.clone())
        .unwrap();

    // When: Applying responsive design
    let config = ScatterPlotConfig {
        base: create_base_config("Responsive Chart", 800, 600),
        point_color: "#00d4ff".to_string(),
        point_size: 5.0,
        show_trend_line: true,
        trend_line_color: "#ff6b6b".to_string(),
        trend_line_width: 2.0,
        show_legend: true,
        point_shape: Some(PointShape::Circle),
        opacity: Some(0.8),
        jitter: None,
    };

    let mobile_config = responsive_manager
        .apply_responsive(&config, 400, 300)
        .unwrap();
    let desktop_config = responsive_manager
        .apply_responsive(&config, 1200, 800)
        .unwrap();

    // Then: Should adapt to different screen sizes
    assert_eq!(mobile_config.base.width, 400);
    assert_eq!(mobile_config.base.height, 300);
    assert_eq!(desktop_config.base.width, 1200);
    assert_eq!(desktop_config.base.height, 800);
}

#[test]
fn test_accessibility_styling() {
    // Given: Accessibility styling configuration
    let accessibility_config = AccessibilityStylingConfig {
        high_contrast_mode: true,
        color_blind_support: true,
        reduced_motion: true,
        focus_indicators: true,
        screen_reader_optimized: true,
        minimum_contrast_ratio: 4.5,
    };

    let mut accessibility_manager = AccessibilityStylingManager::new().unwrap();
    accessibility_manager
        .set_accessibility_config(accessibility_config.clone())
        .unwrap();

    // When: Applying accessibility styling
    let config = LineChartConfig {
        base: create_base_config("Accessible Chart", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let accessible_config = accessibility_manager.apply_accessibility(&config).unwrap();

    // Then: Should apply accessibility enhancements
    assert!(accessible_config.line_width >= 2.0); // Ensure minimum line width
    assert!(accessible_config.point_size >= 4.0); // Ensure minimum point size
}

#[test]
fn test_animation_system() {
    // Given: Animation system configuration
    let animation_config = AnimationSystemConfig {
        entrance_animations: EntranceAnimationConfig {
            enabled: true,
            type_: EntranceAnimationType::FadeIn,
            duration: Duration::from_millis(800),
            delay: Duration::from_millis(100),
        },
        data_animations: DataAnimationConfig {
            enabled: true,
            type_: DataAnimationType::Morph,
            duration: Duration::from_millis(600),
            easing: EasingType::EaseInOut,
        },
        interaction_animations: InteractionAnimationConfig {
            hover: HoverAnimationConfig {
                enabled: true,
                scale: 1.1,
                duration: Duration::from_millis(200),
            },
            click: ClickAnimationConfig {
                enabled: true,
                scale: 0.95,
                duration: Duration::from_millis(150),
            },
        },
    };

    let mut animation_manager = AnimationManager::new().unwrap();
    animation_manager
        .set_animation_config(animation_config.clone())
        .unwrap();

    // When: Applying animations
    let config = BarChartConfig {
        base: create_base_config("Animated Chart", 800, 600),
        colors: vec!["#00d4ff".to_string(), "#ff6b6b".to_string()],
        bar_width: 0.8,
        show_values: true,
        horizontal: false,
        show_legend: true,
        corner_radius: Some(4.0),
        spacing: Some(0.1),
    };

    let animated_config = animation_manager.apply_animations(&config).unwrap();

    // Then: Should apply animation settings
    assert!(animated_config.corner_radius.is_some());
}

#[test]
fn test_export_styling() {
    // Given: Export styling configuration
    let export_config = ExportStylingConfig {
        formats: vec![
            ExportFormat::PNG {
                width: 1920,
                height: 1080,
                dpi: 300,
                background: "#ffffff".to_string(),
            },
            ExportFormat::SVG {
                width: 800,
                height: 600,
                include_styles: true,
            },
            ExportFormat::PDF {
                width: 8.5,
                height: 11.0,
                dpi: 300,
                margin: 0.5,
            },
        ],
        styling_presets: vec![
            "print".to_string(),
            "presentation".to_string(),
            "web".to_string(),
        ],
    };

    let mut export_manager = ExportStylingManager::new().unwrap();
    export_manager
        .set_export_config(export_config.clone())
        .unwrap();

    // When: Preparing chart for export
    let config = LineChartConfig {
        base: create_base_config("Export Chart", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let print_config = export_manager.prepare_for_export(&config, "print").unwrap();
    let web_config = export_manager.prepare_for_export(&config, "web").unwrap();

    // Then: Should prepare appropriate export styling
    assert_ne!(
        print_config.base.background_color,
        web_config.base.background_color
    );
}

#[test]
fn test_style_validation() {
    // Given: Style validation configuration
    let validation_config = StyleValidationConfig {
        validate_colors: true,
        validate_contrast: true,
        validate_accessibility: true,
        validate_performance: true,
        warnings_as_errors: false,
    };

    let mut validation_manager = StyleValidationManager::new().unwrap();
    validation_manager
        .set_validation_config(validation_config.clone())
        .unwrap();

    // When: Validating chart styles
    let config = LineChartConfig {
        base: create_base_config("Validation Test", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let validation_result = validation_manager.validate_styles(&config).unwrap();

    // Then: Should validate styles
    assert!(validation_result.is_valid);
    assert!(validation_result.warnings.len() >= 0);
    assert!(validation_result.errors.len() == 0);
}

#[test]
fn test_style_performance() {
    // Given: Performance testing configuration
    let performance_config = StylePerformanceConfig {
        measure_render_time: true,
        measure_memory_usage: true,
        measure_gpu_usage: true,
        target_fps: 60,
        max_memory_mb: 100,
    };

    let mut performance_manager = StylePerformanceManager::new().unwrap();
    performance_manager
        .set_performance_config(performance_config.clone())
        .unwrap();

    // When: Testing style performance
    let config = LineChartConfig {
        base: create_base_config("Performance Test", 1920, 1080),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let performance_result = performance_manager.measure_performance(&config).unwrap();

    // Then: Should measure performance
    assert!(performance_result.render_time_ms > 0.0);
    assert!(performance_result.memory_usage_mb > 0.0);
    assert!(performance_result.meets_target_fps);
}
