//! Enhanced Interactive Features Tests
//! Tests for improved zoom, pan, tooltips, and user interactions

use leptos_helios::*;
use std::time::Duration;

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
fn test_enhanced_tooltip_system() {
    // Given: A chart with enhanced tooltip configuration
    let data = vec![
        (1.0, 2.0, Some("Point A".to_string())),
        (2.0, 3.5, Some("Point B".to_string())),
        (3.0, 1.0, Some("Point C".to_string())),
        (4.0, 4.0, Some("Point D".to_string())),
    ];

    let tooltip_config = TooltipConfig {
        enabled: true,
        show_on_hover: true,
        show_on_click: true,
        delay: Duration::from_millis(100),
        position: TooltipPosition::FollowCursor,
        style: TooltipStyle {
            background_color: "#333333".to_string(),
            text_color: "#ffffff".to_string(),
            border_color: "#666666".to_string(),
            border_width: 1.0,
            border_radius: 4.0,
            padding: 8.0,
            font_size: 12.0,
            font_family: "Arial".to_string(),
        },
        format: TooltipFormat::Custom(Box::new(|data| {
            format!("X: {:.2}, Y: {:.2}", data.x, data.y)
        })),
    };

    let config = LineChartConfig {
        base: create_base_config("Enhanced Tooltips", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let mut interaction_manager = InteractionManager::new().unwrap();
    let chart_state = ChartState::new(&config.base);

    // When: Hovering over a point
    let hover_position = [200.0, 300.0];
    let result = interaction_manager.handle_hover_with_tooltip(
        &chart_state,
        hover_position,
        &data,
        &tooltip_config,
    );

    // Then: Should return enhanced tooltip information
    assert!(result.is_ok(), "Enhanced tooltip hover should succeed");

    let tooltip_info = result.unwrap();
    assert!(tooltip_info.is_some(), "Should return tooltip information");

    if let Some(info) = tooltip_info {
        assert!(
            info.content.contains("X:") && info.content.contains("Y:"),
            "Tooltip should contain formatted data"
        );
        assert_eq!(info.position, TooltipPosition::FollowCursor);
        assert!(info.style.background_color == "#333333");
    }
}

#[test]
fn test_smooth_zoom_animation() {
    // Given: A chart with smooth zoom configuration
    let data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 1.5), (3.0, 3.0)];
    let config = LineChartConfig {
        base: create_base_config("Smooth Zoom", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let zoom_config = ZoomConfig {
        smooth_animation: true,
        animation_duration: Duration::from_millis(300),
        easing: EasingFunction::EaseOutCubic,
        min_zoom: 0.1,
        max_zoom: 10.0,
        zoom_sensitivity: 1.2,
        double_click_zoom: true,
        double_click_zoom_factor: 2.0,
    };

    let mut interaction_manager = InteractionManager::new().unwrap();
    let mut chart_state = ChartState::new(&config.base);

    // When: Performing smooth zoom
    let zoom_center = [400.0, 300.0];
    let zoom_factor = 1.5;
    let result = interaction_manager.smooth_zoom_chart(
        &mut chart_state,
        zoom_center,
        zoom_factor,
        &zoom_config,
    );

    // Then: Should perform smooth zoom
    assert!(result.is_ok(), "Smooth zoom should succeed");

    let viewport = chart_state.viewport();
    assert!(viewport.scale_x > 1.0, "Zoom should be applied");
    assert!(viewport.scale_y > 1.0, "Zoom should be applied");
}

#[test]
fn test_momentum_panning() {
    // Given: A chart with momentum panning
    let data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 1.5), (3.0, 3.0)];
    let config = LineChartConfig {
        base: create_base_config("Momentum Pan", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let pan_config = PanConfig {
        momentum: true,
        momentum_friction: 0.95,
        momentum_threshold: 5.0,
        bounds: Some(PanBounds {
            min_x: -100.0,
            max_x: 100.0,
            min_y: -100.0,
            max_y: 100.0,
        }),
        snap_to_bounds: true,
        snap_threshold: 10.0,
    };

    let mut interaction_manager = InteractionManager::new().unwrap();
    let mut chart_state = ChartState::new(&config.base);

    // When: Starting momentum pan
    let pan_delta = PanDelta { x: 50.0, y: -30.0 };
    let result = interaction_manager.start_momentum_pan(&mut chart_state, pan_delta, &pan_config);

    // Then: Should start momentum panning
    assert!(result.is_ok(), "Momentum pan should succeed");

    // Simulate momentum update
    let momentum_result = interaction_manager.update_momentum_pan(&mut chart_state, &pan_config);
    assert!(momentum_result.is_ok(), "Momentum update should succeed");
}

#[test]
fn test_gesture_recognition() {
    // Given: A chart with gesture recognition
    let data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 1.5), (3.0, 3.0)];
    let config = LineChartConfig {
        base: create_base_config("Gesture Recognition", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let gesture_config = GestureConfig {
        pinch_zoom: true,
        two_finger_pan: true,
        double_tap_zoom: true,
        long_press_context_menu: true,
        swipe_navigation: true,
        gesture_threshold: 10.0,
    };

    let mut interaction_manager = InteractionManager::new().unwrap();
    let mut chart_state = ChartState::new(&config.base);

    // When: Recognizing gestures
    let touch_points = vec![
        TouchPoint {
            x: 100.0,
            y: 200.0,
            id: 1,
        },
        TouchPoint {
            x: 150.0,
            y: 250.0,
            id: 2,
        },
    ];

    let result =
        interaction_manager.handle_gesture(&mut chart_state, &touch_points, &gesture_config);

    // Then: Should recognize gesture
    assert!(result.is_ok(), "Gesture recognition should succeed");

    let gesture = result.unwrap();
    assert!(gesture.is_some(), "Should recognize a gesture");
}

#[test]
fn test_interaction_history() {
    // Given: A chart with interaction history
    let data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 1.5), (3.0, 3.0)];
    let config = LineChartConfig {
        base: create_base_config("Interaction History", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let history_config = InteractionHistoryConfig {
        max_history_size: 50,
        save_zoom: true,
        save_pan: true,
        save_selection: true,
        auto_save_interval: Duration::from_secs(5),
    };

    let mut interaction_manager = InteractionManager::new().unwrap();
    let mut chart_state = ChartState::new(&config.base);

    // When: Performing multiple interactions
    interaction_manager
        .pan_chart(&mut chart_state, PanDelta { x: 50.0, y: -30.0 })
        .unwrap();
    interaction_manager
        .zoom_chart(&mut chart_state, [400.0, 300.0], 1.5)
        .unwrap();
    interaction_manager
        .pan_chart(&mut chart_state, PanDelta { x: 20.0, y: 10.0 })
        .unwrap();

    // Then: Should maintain interaction history
    let history = interaction_manager.get_interaction_history();
    assert!(history.len() >= 3, "Should have interaction history");

    // Test undo/redo
    let undo_result = interaction_manager.undo(&mut chart_state);
    assert!(undo_result.is_ok(), "Undo should succeed");

    let redo_result = interaction_manager.redo(&mut chart_state);
    assert!(redo_result.is_ok(), "Redo should succeed");
}

#[test]
fn test_selection_system() {
    // Given: A chart with selection system
    let data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 1.5), (3.0, 3.0), (4.0, 2.5)];
    let config = LineChartConfig {
        base: create_base_config("Selection System", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let selection_config = SelectionConfig {
        enabled: true,
        selection_type: SelectionType::Rectangle,
        selection_color: "#00d4ff".to_string(),
        selection_opacity: 0.3,
        multi_select: true,
        selection_mode: SelectionMode::Additive,
    };

    let mut interaction_manager = InteractionManager::new().unwrap();
    let chart_state = ChartState::new(&config.base);

    // When: Creating a selection
    let selection_start = [100.0, 100.0];
    let selection_end = [200.0, 200.0];
    let result = interaction_manager.create_selection(
        &chart_state,
        selection_start,
        selection_end,
        &selection_config,
    );

    // Then: Should create selection
    assert!(result.is_ok(), "Selection creation should succeed");

    let selection = result.unwrap();
    assert!(selection.is_some(), "Should create a selection");

    if let Some(sel) = selection {
        assert_eq!(sel.selection_type, SelectionType::Rectangle);
        assert!(sel.selected_points.len() > 0, "Should select some points");
    }
}

#[test]
fn test_keyboard_navigation() {
    // Given: A chart with keyboard navigation
    let data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 1.5), (3.0, 3.0)];
    let config = LineChartConfig {
        base: create_base_config("Keyboard Navigation", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let keyboard_config = KeyboardConfig {
        enabled: true,
        arrow_key_pan: true,
        arrow_key_pan_speed: 10.0,
        plus_minus_zoom: true,
        plus_minus_zoom_factor: 1.2,
        home_reset: true,
        escape_clear_selection: true,
    };

    let mut interaction_manager = InteractionManager::new().unwrap();
    let mut chart_state = ChartState::new(&config.base);

    // When: Using keyboard navigation
    let key_event = KeyEvent {
        key: "ArrowRight".to_string(),
        ctrl_key: false,
        shift_key: false,
        alt_key: false,
    };

    let result =
        interaction_manager.handle_keyboard(&mut chart_state, &key_event, &keyboard_config);

    // Then: Should handle keyboard input
    assert!(result.is_ok(), "Keyboard navigation should succeed");

    let viewport = chart_state.viewport();
    assert_ne!(viewport.x, 0.0, "Pan should be applied via keyboard");
}

#[test]
fn test_interaction_performance() {
    // Given: A chart with many data points for performance testing
    let data: Vec<(f64, f64, Option<String>)> = (0..10000)
        .map(|i| (i as f64 * 0.1, (i as f64 * 0.1).sin(), None))
        .collect();

    let config = ScatterPlotConfig {
        base: create_base_config("Performance Test", 800, 600),
        point_color: "#00d4ff".to_string(),
        point_size: 2.0,
        show_trend_line: false,
        trend_line_color: "#ff6b6b".to_string(),
        trend_line_width: 1.0,
        show_legend: false,
        point_shape: Some(PointShape::Circle),
        opacity: Some(0.8),
        jitter: None,
    };

    let mut interaction_manager = InteractionManager::new().unwrap();
    let mut chart_state = ChartState::new(&config.base);

    // When: Performing many interactions quickly
    let start = std::time::Instant::now();

    for i in 0..100 {
        let pan_delta = PanDelta {
            x: i as f32 * 0.1,
            y: (i as f32 * 0.1).sin(),
        };
        interaction_manager
            .pan_chart(&mut chart_state, pan_delta)
            .unwrap();

        let zoom_center = [400.0, 300.0];
        let zoom_factor = 1.0 + (i as f32) * 0.001;
        interaction_manager
            .zoom_chart(&mut chart_state, zoom_center, zoom_factor)
            .unwrap();

        let hover_position = [200.0 + i as f32, 300.0 + i as f32];
        // Convert data to simple format for performance test
        let simple_data: Vec<(f64, f64)> = data.iter().map(|(x, y, _)| (*x, *y)).collect();
        let _ = interaction_manager.handle_hover(&chart_state, hover_position, &simple_data);
    }

    let duration = start.elapsed();

    // Then: Interactions should be fast
    assert!(
        duration.as_millis() < 200,
        "100 interactions with 10K points should take less than 200ms, took {}ms",
        duration.as_millis()
    );
}

#[test]
fn test_interaction_accessibility() {
    // Given: A chart with accessibility features
    let data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 1.5), (3.0, 3.0)];
    let config = LineChartConfig {
        base: create_base_config("Accessibility Test", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let accessibility_config = AccessibilityConfig {
        screen_reader_support: true,
        high_contrast_mode: true,
        keyboard_navigation: true,
        focus_indicators: true,
        aria_labels: true,
        reduced_motion: true,
    };

    let mut interaction_manager = InteractionManager::new().unwrap();
    let chart_state = ChartState::new(&config.base);

    // When: Using accessibility features
    let result = interaction_manager.get_accessibility_info(&chart_state, &accessibility_config);

    // Then: Should provide accessibility information
    assert!(result.is_ok(), "Accessibility info should be available");

    let accessibility_info = result.unwrap();
    assert!(
        accessibility_info.screen_reader_text.len() > 0,
        "Should have screen reader text"
    );
    assert!(
        accessibility_info.aria_labels.len() > 0,
        "Should have ARIA labels"
    );
}
