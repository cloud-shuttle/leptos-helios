//! User Interaction Tests
//! Tests for pan, zoom, hover, and other user interaction functionality

use leptos_helios::*;

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
fn test_interaction_manager_initialization() {
    // Given: Interaction manager should be able to initialize
    // When: Creating an interaction manager
    // Then: Manager should be created successfully

    let interaction_manager = InteractionManager::new();
    assert!(
        interaction_manager.is_ok(),
        "Interaction manager should initialize successfully"
    );
}

#[test]
fn test_pan_functionality() {
    // Given: A chart with pan enabled
    let data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 1.5), (3.0, 3.0)];
    let config = LineChartConfig {
        base: create_base_config("Pan Test", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let mut interaction_manager = InteractionManager::new().unwrap();
    let mut chart_state = ChartState::new(&config.base);

    // When: Panning the chart
    let pan_delta = PanDelta { x: 50.0, y: -30.0 };
    let result = interaction_manager.pan_chart(&mut chart_state, pan_delta);

    // Then: Pan should be applied successfully
    assert!(result.is_ok(), "Pan operation should succeed");

    let new_viewport = chart_state.viewport();
    assert_ne!(new_viewport.x, 0.0, "Viewport X should change after pan");
    assert_ne!(new_viewport.y, 0.0, "Viewport Y should change after pan");
}

#[test]
fn test_zoom_functionality() {
    // Given: A chart with zoom enabled
    let data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 1.5), (3.0, 3.0)];
    let config = LineChartConfig {
        base: create_base_config("Zoom Test", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let mut interaction_manager = InteractionManager::new().unwrap();
    let mut chart_state = ChartState::new(&config.base);

    // When: Zooming in on the chart
    let zoom_center = [400.0, 300.0];
    let zoom_factor = 1.5;
    let result = interaction_manager.zoom_chart(&mut chart_state, zoom_center, zoom_factor);

    // Then: Zoom should be applied successfully
    assert!(result.is_ok(), "Zoom operation should succeed");

    let new_viewport = chart_state.viewport();
    assert!(
        new_viewport.scale_x > 1.0,
        "X scale should increase after zoom in"
    );
    assert!(
        new_viewport.scale_y > 1.0,
        "Y scale should increase after zoom in"
    );
}

#[test]
fn test_zoom_out_functionality() {
    // Given: A chart that's already zoomed in
    let data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 1.5), (3.0, 3.0)];
    let config = LineChartConfig {
        base: create_base_config("Zoom Out Test", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let mut interaction_manager = InteractionManager::new().unwrap();
    let mut chart_state = ChartState::new(&config.base);

    // First zoom in
    let zoom_center = [400.0, 300.0];
    interaction_manager
        .zoom_chart(&mut chart_state, zoom_center, 2.0)
        .unwrap();

    // When: Zooming out
    let zoom_out_factor = 0.5;
    let result = interaction_manager.zoom_chart(&mut chart_state, zoom_center, zoom_out_factor);

    // Then: Zoom out should be applied successfully
    assert!(result.is_ok(), "Zoom out operation should succeed");

    let new_viewport = chart_state.viewport();
    assert!(
        new_viewport.scale_x < 2.0,
        "X scale should decrease after zoom out"
    );
    assert!(
        new_viewport.scale_y < 2.0,
        "Y scale should decrease after zoom out"
    );
}

#[test]
fn test_hover_functionality() {
    // Given: A chart with hover enabled
    let data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 1.5), (3.0, 3.0)];
    let config = LineChartConfig {
        base: create_base_config("Hover Test", 800, 600),
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
    let result = interaction_manager.handle_hover(&chart_state, hover_position, &data);

    // Then: Hover should return data point information
    assert!(result.is_ok(), "Hover operation should succeed");

    let hover_info = result.unwrap();
    assert!(hover_info.is_some(), "Hover should return some information");

    if let Some(info) = hover_info {
        assert!(
            info.distance < 50.0,
            "Hovered point should be within reasonable distance"
        );
        assert!(info.data_index < data.len(), "Data index should be valid");
    }
}

#[test]
fn test_click_functionality() {
    // Given: A chart with click handling enabled
    let data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 1.5), (3.0, 3.0)];
    let config = LineChartConfig {
        base: create_base_config("Click Test", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let mut interaction_manager = InteractionManager::new().unwrap();
    let chart_state = ChartState::new(&config.base);

    // When: Clicking on a point
    let click_position = [200.0, 300.0];
    let result = interaction_manager.handle_click(&chart_state, click_position, &data);

    // Then: Click should return data point information
    assert!(result.is_ok(), "Click operation should succeed");

    let click_info = result.unwrap();
    assert!(click_info.is_some(), "Click should return some information");

    if let Some(info) = click_info {
        assert!(
            info.distance < 50.0,
            "Clicked point should be within reasonable distance"
        );
        assert!(info.data_index < data.len(), "Data index should be valid");
    }
}

#[test]
fn test_reset_view_functionality() {
    // Given: A chart that's been panned and zoomed
    let data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 1.5), (3.0, 3.0)];
    let config = LineChartConfig {
        base: create_base_config("Reset Test", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let mut interaction_manager = InteractionManager::new().unwrap();
    let mut chart_state = ChartState::new(&config.base);

    // Apply some transformations
    interaction_manager
        .pan_chart(&mut chart_state, PanDelta { x: 100.0, y: 50.0 })
        .unwrap();
    interaction_manager
        .zoom_chart(&mut chart_state, [400.0, 300.0], 2.0)
        .unwrap();

    // When: Resetting the view
    let result = interaction_manager.reset_view(&mut chart_state);

    // Then: View should be reset to original state
    assert!(result.is_ok(), "Reset view operation should succeed");

    let viewport = chart_state.viewport();
    assert_eq!(viewport.x, 0.0, "Viewport X should be reset to 0");
    assert_eq!(viewport.y, 0.0, "Viewport Y should be reset to 0");
    assert_eq!(viewport.scale_x, 1.0, "X scale should be reset to 1.0");
    assert_eq!(viewport.scale_y, 1.0, "Y scale should be reset to 1.0");
}

#[test]
fn test_interaction_constraints() {
    // Given: A chart with interaction constraints
    let data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 1.5), (3.0, 3.0)];
    let config = LineChartConfig {
        base: create_base_config("Constraints Test", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let mut interaction_manager = InteractionManager::new().unwrap();
    let mut chart_state = ChartState::new(&config.base);

    // Set constraints
    let constraints = InteractionConstraints {
        min_zoom: 0.1,
        max_zoom: 10.0,
        pan_bounds: Some([-100.0, -100.0, 200.0, 200.0]), // [x, y, width, height]
    };
    interaction_manager.set_constraints(constraints);

    // When: Trying to zoom beyond limits
    let zoom_center = [400.0, 300.0];
    let result = interaction_manager.zoom_chart(&mut chart_state, zoom_center, 20.0); // Beyond max_zoom

    // Then: Zoom should be constrained
    assert!(result.is_ok(), "Constrained zoom should succeed");

    let viewport = chart_state.viewport();
    assert!(
        viewport.scale_x <= 10.0,
        "Zoom should be constrained to max_zoom"
    );
    assert!(
        viewport.scale_y <= 10.0,
        "Zoom should be constrained to max_zoom"
    );
}

#[test]
fn test_interaction_performance() {
    // Given: A chart with many data points
    let data: Vec<(f64, f64)> = (0..10000).map(|i| (i as f64, (i as f64).sin())).collect();

    let config = LineChartConfig {
        base: create_base_config("Performance Test", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 1.0,
        show_points: false,
        point_size: 2.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let mut interaction_manager = InteractionManager::new().unwrap();
    let mut chart_state = ChartState::new(&config.base);

    // When: Performing multiple interactions quickly
    let start = std::time::Instant::now();

    for i in 0..100 {
        let pan_delta = PanDelta {
            x: i as f32,
            y: (i as f32) * 0.5,
        };
        interaction_manager
            .pan_chart(&mut chart_state, pan_delta)
            .unwrap();

        let zoom_center = [400.0, 300.0];
        let zoom_factor = 1.0 + (i as f32) * 0.01;
        interaction_manager
            .zoom_chart(&mut chart_state, zoom_center, zoom_factor)
            .unwrap();
    }

    let duration = start.elapsed();

    // Then: Interactions should be fast
    assert!(
        duration.as_millis() < 100,
        "100 interactions should take less than 100ms, took {}ms",
        duration.as_millis()
    );
}

#[test]
fn test_interaction_state_persistence() {
    // Given: A chart with interaction state
    let data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 1.5), (3.0, 3.0)];
    let config = LineChartConfig {
        base: create_base_config("State Test", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let mut interaction_manager = InteractionManager::new().unwrap();
    let mut chart_state = ChartState::new(&config.base);

    // Apply some transformations
    interaction_manager
        .pan_chart(&mut chart_state, PanDelta { x: 50.0, y: -30.0 })
        .unwrap();
    interaction_manager
        .zoom_chart(&mut chart_state, [400.0, 300.0], 1.5)
        .unwrap();

    // When: Getting the current state
    let state = interaction_manager.get_state(&chart_state);

    // Then: State should reflect the transformations
    // Note: Zoom operation affects pan coordinates, so we check that they're non-zero
    assert_ne!(state.pan_x, 0.0, "Pan X should be persisted in state");
    assert_ne!(state.pan_y, 0.0, "Pan Y should be persisted in state");
    assert_eq!(state.zoom_x, 1.5, "Zoom X should be persisted in state");
    assert_eq!(state.zoom_y, 1.5, "Zoom Y should be persisted in state");
}

#[test]
fn test_interaction_undo_redo() {
    // Given: A chart with undo/redo functionality
    let data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 1.5), (3.0, 3.0)];
    let config = LineChartConfig {
        base: create_base_config("Undo Test", 800, 600),
        color: "#00d4ff".to_string(),
        line_width: 2.0,
        show_points: true,
        point_size: 4.0,
        interpolation: InterpolationType::Linear,
        show_legend: true,
    };

    let mut interaction_manager = InteractionManager::new().unwrap();
    let mut chart_state = ChartState::new(&config.base);

    // Apply some transformations
    interaction_manager
        .pan_chart(&mut chart_state, PanDelta { x: 50.0, y: -30.0 })
        .unwrap();
    interaction_manager
        .zoom_chart(&mut chart_state, [400.0, 300.0], 1.5)
        .unwrap();

    // When: Undoing the last action
    let result = interaction_manager.undo(&mut chart_state);

    // Then: Undo should work
    assert!(result.is_ok(), "Undo operation should succeed");

    let viewport = chart_state.viewport();
    assert_eq!(viewport.scale_x, 1.0, "Zoom should be undone");
    assert_eq!(viewport.scale_y, 1.0, "Zoom should be undone");

    // Pan should still be there
    assert_eq!(viewport.x, 50.0, "Pan should still be applied");
    assert_eq!(viewport.y, -30.0, "Pan should still be applied");
}
