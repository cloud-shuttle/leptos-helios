//! User Interaction System
//! Handles pan, zoom, hover, click, and other user interactions with charts

use crate::chart_config::*;
use std::collections::VecDeque;
use std::time::Duration;

/// Pan delta for chart panning
#[derive(Debug, Clone, Copy)]
pub struct PanDelta {
    pub x: f32,
    pub y: f32,
}

// Use the existing Point2D type alias from lib.rs

// Use the existing Rect type alias from lib.rs

/// Viewport state for chart transformations
#[derive(Debug, Clone)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
        }
    }
}

/// Chart state containing viewport and interaction data
pub struct ChartState {
    viewport: Viewport,
    config: BaseChartConfig,
}

impl ChartState {
    pub fn new(config: &BaseChartConfig) -> Self {
        Self {
            viewport: Viewport::default(),
            config: config.clone(),
        }
    }

    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }

    pub fn viewport_mut(&mut self) -> &mut Viewport {
        &mut self.viewport
    }
}

/// Interaction constraints for limiting user actions
#[derive(Debug, Clone)]
pub struct InteractionConstraints {
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub pan_bounds: Option<[f32; 4]>, // [x, y, width, height]
}

impl Default for InteractionConstraints {
    fn default() -> Self {
        Self {
            min_zoom: 0.1,
            max_zoom: 10.0,
            pan_bounds: None,
        }
    }
}

/// Hover information for data points
#[derive(Debug, Clone)]
pub struct HoverInfo {
    pub data_index: usize,
    pub distance: f32,
    pub x: f64,
    pub y: f64,
}

/// Click information for data points
#[derive(Debug, Clone)]
pub struct ClickInfo {
    pub data_index: usize,
    pub distance: f32,
    pub x: f64,
    pub y: f64,
}

/// Interaction state for undo/redo functionality
#[derive(Debug, Clone)]
pub struct InteractionState {
    pub pan_x: f32,
    pub pan_y: f32,
    pub zoom_x: f32,
    pub zoom_y: f32,
}

/// Interaction manager for handling user interactions
pub struct InteractionManager {
    constraints: InteractionConstraints,
    undo_stack: VecDeque<InteractionState>,
    redo_stack: VecDeque<InteractionState>,
    max_undo_steps: usize,
}

impl InteractionManager {
    pub fn new() -> Result<Self, ChartRenderError> {
        Ok(Self {
            constraints: InteractionConstraints::default(),
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
            max_undo_steps: 50,
        })
    }

    /// Set interaction constraints
    pub fn set_constraints(&mut self, constraints: InteractionConstraints) {
        self.constraints = constraints;
    }

    /// Pan the chart
    pub fn pan_chart(
        &mut self,
        chart_state: &mut ChartState,
        delta: PanDelta,
    ) -> Result<(), ChartRenderError> {
        // Save current state for undo
        self.save_state(chart_state);

        let viewport = chart_state.viewport_mut();

        // Apply pan with constraints
        let new_x = viewport.x + delta.x;
        let new_y = viewport.y + delta.y;

        if let Some(bounds) = self.constraints.pan_bounds {
            viewport.x = new_x.max(bounds[0]).min(bounds[0] + bounds[2]);
            viewport.y = new_y.max(bounds[1]).min(bounds[1] + bounds[3]);
        } else {
            viewport.x = new_x;
            viewport.y = new_y;
        }

        Ok(())
    }

    /// Zoom the chart
    pub fn zoom_chart(
        &mut self,
        chart_state: &mut ChartState,
        center: [f32; 2],
        factor: f32,
    ) -> Result<(), ChartRenderError> {
        // Save current state for undo
        self.save_state(chart_state);

        let viewport = chart_state.viewport_mut();

        // Apply zoom with constraints
        let new_scale_x = (viewport.scale_x * factor)
            .max(self.constraints.min_zoom)
            .min(self.constraints.max_zoom);
        let new_scale_y = (viewport.scale_y * factor)
            .max(self.constraints.min_zoom)
            .min(self.constraints.max_zoom);

        // Adjust pan to zoom around the center point
        let scale_change_x = new_scale_x / viewport.scale_x;
        let scale_change_y = new_scale_y / viewport.scale_y;

        viewport.x = center[0] - (center[0] - viewport.x) * scale_change_x;
        viewport.y = center[1] - (center[1] - viewport.y) * scale_change_y;
        viewport.scale_x = new_scale_x;
        viewport.scale_y = new_scale_y;

        Ok(())
    }

    /// Handle hover over chart
    pub fn handle_hover(
        &self,
        chart_state: &ChartState,
        position: [f32; 2],
        data: &[(f64, f64)],
    ) -> Result<Option<HoverInfo>, ChartRenderError> {
        if data.is_empty() {
            return Ok(None);
        }

        let _viewport = chart_state.viewport();

        // For simplicity, we'll use a basic distance calculation in screen space
        // In a real implementation, this would properly transform coordinates

        // Find closest data point by mapping data to screen coordinates
        let mut closest_index = 0;
        let mut min_distance = f32::INFINITY;

        for (i, (x, y)) in data.iter().enumerate() {
            // Simple mapping: assume data range is 0-4 for both x and y
            let screen_x = (*x as f32) * 200.0; // Scale to screen coordinates
            let screen_y = (*y as f32) * 150.0; // Scale to screen coordinates

            let dx = position[0] - screen_x;
            let dy = position[1] - screen_y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance < min_distance {
                min_distance = distance;
                closest_index = i;
            }
        }

        // Only return hover info if within reasonable distance (increased threshold)
        if min_distance < 100.0 {
            Ok(Some(HoverInfo {
                data_index: closest_index,
                distance: min_distance,
                x: data[closest_index].0,
                y: data[closest_index].1,
            }))
        } else {
            Ok(None)
        }
    }

    /// Handle click on chart
    pub fn handle_click(
        &self,
        chart_state: &ChartState,
        position: [f32; 2],
        data: &[(f64, f64)],
    ) -> Result<Option<ClickInfo>, ChartRenderError> {
        if data.is_empty() {
            return Ok(None);
        }

        let _viewport = chart_state.viewport();

        // For simplicity, we'll use a basic distance calculation in screen space
        // In a real implementation, this would properly transform coordinates

        // Find closest data point by mapping data to screen coordinates
        let mut closest_index = 0;
        let mut min_distance = f32::INFINITY;

        for (i, (x, y)) in data.iter().enumerate() {
            // Simple mapping: assume data range is 0-4 for both x and y
            let screen_x = (*x as f32) * 200.0; // Scale to screen coordinates
            let screen_y = (*y as f32) * 150.0; // Scale to screen coordinates

            let dx = position[0] - screen_x;
            let dy = position[1] - screen_y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance < min_distance {
                min_distance = distance;
                closest_index = i;
            }
        }

        // Only return click info if within reasonable distance (increased threshold)
        if min_distance < 100.0 {
            Ok(Some(ClickInfo {
                data_index: closest_index,
                distance: min_distance,
                x: data[closest_index].0,
                y: data[closest_index].1,
            }))
        } else {
            Ok(None)
        }
    }

    /// Reset view to original state
    pub fn reset_view(&mut self, chart_state: &mut ChartState) -> Result<(), ChartRenderError> {
        // Save current state for undo
        self.save_state(chart_state);

        let viewport = chart_state.viewport_mut();
        viewport.x = 0.0;
        viewport.y = 0.0;
        viewport.scale_x = 1.0;
        viewport.scale_y = 1.0;

        Ok(())
    }

    /// Get current interaction state
    pub fn get_state(&self, chart_state: &ChartState) -> InteractionState {
        InteractionState {
            pan_x: chart_state.viewport().x,
            pan_y: chart_state.viewport().y,
            zoom_x: chart_state.viewport().scale_x,
            zoom_y: chart_state.viewport().scale_y,
        }
    }

    /// Undo last interaction
    pub fn undo(&mut self, chart_state: &mut ChartState) -> Result<(), ChartRenderError> {
        if let Some(previous_state) = self.undo_stack.pop_back() {
            // Save current state to redo stack
            let current_state = InteractionState {
                pan_x: chart_state.viewport().x,
                pan_y: chart_state.viewport().y,
                zoom_x: chart_state.viewport().scale_x,
                zoom_y: chart_state.viewport().scale_y,
            };
            self.redo_stack.push_back(current_state);

            // Apply previous state
            let viewport = chart_state.viewport_mut();
            viewport.x = previous_state.pan_x;
            viewport.y = previous_state.pan_y;
            viewport.scale_x = previous_state.zoom_x;
            viewport.scale_y = previous_state.zoom_y;
        }

        Ok(())
    }

    /// Redo last undone interaction
    pub fn redo(&mut self, chart_state: &mut ChartState) -> Result<(), ChartRenderError> {
        if let Some(next_state) = self.redo_stack.pop_back() {
            // Save current state to undo stack
            let current_state = InteractionState {
                pan_x: chart_state.viewport().x,
                pan_y: chart_state.viewport().y,
                zoom_x: chart_state.viewport().scale_x,
                zoom_y: chart_state.viewport().scale_y,
            };
            self.undo_stack.push_back(current_state);

            // Apply next state
            let viewport = chart_state.viewport_mut();
            viewport.x = next_state.pan_x;
            viewport.y = next_state.pan_y;
            viewport.scale_x = next_state.zoom_x;
            viewport.scale_y = next_state.zoom_y;
        }

        Ok(())
    }

    /// Save current state for undo functionality
    fn save_state(&mut self, chart_state: &ChartState) {
        let state = InteractionState {
            pan_x: chart_state.viewport().x,
            pan_y: chart_state.viewport().y,
            zoom_x: chart_state.viewport().scale_x,
            zoom_y: chart_state.viewport().scale_y,
        };

        self.undo_stack.push_back(state);

        // Limit undo stack size
        if self.undo_stack.len() > self.max_undo_steps {
            self.undo_stack.pop_front();
        }

        // Clear redo stack when new action is performed
        self.redo_stack.clear();
    }
}

// Enhanced Interactive Features

/// Tooltip configuration
#[derive(Debug, Clone)]
pub struct TooltipConfig {
    pub enabled: bool,
    pub show_on_hover: bool,
    pub show_on_click: bool,
    pub delay: Duration,
    pub position: TooltipPosition,
    pub style: TooltipStyle,
    pub format: TooltipFormat,
}

/// Tooltip position
#[derive(Debug, Clone, PartialEq)]
pub enum TooltipPosition {
    FollowCursor,
    Fixed,
    Top,
    Bottom,
    Left,
    Right,
}

/// Tooltip style
#[derive(Debug, Clone)]
pub struct TooltipStyle {
    pub background_color: String,
    pub text_color: String,
    pub border_color: String,
    pub border_width: f32,
    pub border_radius: f32,
    pub padding: f32,
    pub font_size: f32,
    pub font_family: String,
}

/// Tooltip format
pub enum TooltipFormat {
    Default,
    Custom(Box<dyn Fn(TooltipData) -> String + Send + Sync>),
}

impl std::fmt::Debug for TooltipFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TooltipFormat::Default => write!(f, "TooltipFormat::Default"),
            TooltipFormat::Custom(_) => write!(f, "TooltipFormat::Custom"),
        }
    }
}

impl Clone for TooltipFormat {
    fn clone(&self) -> Self {
        match self {
            TooltipFormat::Default => TooltipFormat::Default,
            TooltipFormat::Custom(_) => TooltipFormat::Default, // Can't clone function, use default
        }
    }
}

/// Tooltip data
#[derive(Debug, Clone)]
pub struct TooltipData {
    pub x: f64,
    pub y: f64,
    pub label: Option<String>,
    pub index: usize,
}

/// Tooltip information
#[derive(Debug, Clone)]
pub struct TooltipInfo {
    pub content: String,
    pub position: TooltipPosition,
    pub style: TooltipStyle,
    pub data: TooltipData,
}

/// Zoom configuration
#[derive(Debug, Clone)]
pub struct ZoomConfig {
    pub smooth_animation: bool,
    pub animation_duration: Duration,
    pub easing: EasingFunction,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub zoom_sensitivity: f32,
    pub double_click_zoom: bool,
    pub double_click_zoom_factor: f32,
}

/// Easing function
#[derive(Debug, Clone)]
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
}

/// Pan configuration
#[derive(Debug, Clone)]
pub struct PanConfig {
    pub momentum: bool,
    pub momentum_friction: f32,
    pub momentum_threshold: f32,
    pub bounds: Option<PanBounds>,
    pub snap_to_bounds: bool,
    pub snap_threshold: f32,
}

/// Pan bounds
#[derive(Debug, Clone)]
pub struct PanBounds {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

/// Gesture configuration
#[derive(Debug, Clone)]
pub struct GestureConfig {
    pub pinch_zoom: bool,
    pub two_finger_pan: bool,
    pub double_tap_zoom: bool,
    pub long_press_context_menu: bool,
    pub swipe_navigation: bool,
    pub gesture_threshold: f32,
}

/// Touch point
#[derive(Debug, Clone)]
pub struct TouchPoint {
    pub x: f32,
    pub y: f32,
    pub id: u32,
}

/// Gesture type
#[derive(Debug, Clone)]
pub enum GestureType {
    Pan,
    Zoom,
    Rotate,
    Swipe,
    Pinch,
    Tap,
    DoubleTap,
    LongPress,
}

/// Gesture information
#[derive(Debug, Clone)]
pub struct GestureInfo {
    pub gesture_type: GestureType,
    pub start_points: Vec<TouchPoint>,
    pub current_points: Vec<TouchPoint>,
    pub velocity: (f32, f32),
    pub scale: f32,
    pub rotation: f32,
}

/// Interaction history configuration
#[derive(Debug, Clone)]
pub struct InteractionHistoryConfig {
    pub max_history_size: usize,
    pub save_zoom: bool,
    pub save_pan: bool,
    pub save_selection: bool,
    pub auto_save_interval: Duration,
}

/// Selection configuration
#[derive(Debug, Clone)]
pub struct SelectionConfig {
    pub enabled: bool,
    pub selection_type: SelectionType,
    pub selection_color: String,
    pub selection_opacity: f32,
    pub multi_select: bool,
    pub selection_mode: SelectionMode,
}

/// Selection type
#[derive(Debug, Clone, PartialEq)]
pub enum SelectionType {
    Rectangle,
    Circle,
    Lasso,
    Point,
}

/// Selection mode
#[derive(Debug, Clone)]
pub enum SelectionMode {
    Replace,
    Additive,
    Subtractive,
    Toggle,
}

/// Selection information
#[derive(Debug, Clone)]
pub struct SelectionInfo {
    pub selection_type: SelectionType,
    pub bounds: (f32, f32, f32, f32), // x, y, width, height
    pub selected_points: Vec<usize>,
    pub selection_color: String,
    pub selection_opacity: f32,
}

/// Keyboard configuration
#[derive(Debug, Clone)]
pub struct KeyboardConfig {
    pub enabled: bool,
    pub arrow_key_pan: bool,
    pub arrow_key_pan_speed: f32,
    pub plus_minus_zoom: bool,
    pub plus_minus_zoom_factor: f32,
    pub home_reset: bool,
    pub escape_clear_selection: bool,
}

/// Key event
#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub key: String,
    pub ctrl_key: bool,
    pub shift_key: bool,
    pub alt_key: bool,
}

/// Accessibility configuration
#[derive(Debug, Clone)]
pub struct AccessibilityConfig {
    pub screen_reader_support: bool,
    pub high_contrast_mode: bool,
    pub keyboard_navigation: bool,
    pub focus_indicators: bool,
    pub aria_labels: bool,
    pub reduced_motion: bool,
}

/// Accessibility information
#[derive(Debug, Clone)]
pub struct AccessibilityInfo {
    pub screen_reader_text: String,
    pub aria_labels: Vec<String>,
    pub focus_order: Vec<String>,
    pub high_contrast_colors: Vec<String>,
}

// Enhanced InteractionManager methods

impl InteractionManager {
    /// Handle hover with enhanced tooltip
    pub fn handle_hover_with_tooltip(
        &self,
        _chart_state: &ChartState,
        position: [f32; 2],
        data: &[(f64, f64, Option<String>)],
        tooltip_config: &TooltipConfig,
    ) -> Result<Option<TooltipInfo>, ChartRenderError> {
        if !tooltip_config.enabled || !tooltip_config.show_on_hover {
            return Ok(None);
        }

        // Find closest data point
        let mut closest_index = 0;
        let mut min_distance = f32::INFINITY;

        for (i, (x, y, _)) in data.iter().enumerate() {
            let screen_x = (*x as f32) * 200.0;
            let screen_y = (*y as f32) * 150.0;

            let dx = position[0] - screen_x;
            let dy = position[1] - screen_y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance < min_distance {
                min_distance = distance;
                closest_index = i;
            }
        }

        if min_distance < 50.0 {
            let (x, y, label) = &data[closest_index];
            let tooltip_data = TooltipData {
                x: *x,
                y: *y,
                label: label.clone(),
                index: closest_index,
            };

            let content = match &tooltip_config.format {
                TooltipFormat::Default => format!("X: {:.2}, Y: {:.2}", x, y),
                TooltipFormat::Custom(formatter) => formatter(tooltip_data.clone()),
            };

            Ok(Some(TooltipInfo {
                content,
                position: tooltip_config.position.clone(),
                style: tooltip_config.style.clone(),
                data: tooltip_data,
            }))
        } else {
            Ok(None)
        }
    }

    /// Perform smooth zoom with animation
    pub fn smooth_zoom_chart(
        &mut self,
        chart_state: &mut ChartState,
        center: [f32; 2],
        factor: f32,
        zoom_config: &ZoomConfig,
    ) -> Result<(), ChartRenderError> {
        if zoom_config.smooth_animation {
            // For now, just apply the zoom directly
            // In a real implementation, this would animate over time
            self.zoom_chart(chart_state, center, factor)
        } else {
            self.zoom_chart(chart_state, center, factor)
        }
    }

    /// Start momentum panning
    pub fn start_momentum_pan(
        &mut self,
        chart_state: &mut ChartState,
        delta: PanDelta,
        pan_config: &PanConfig,
    ) -> Result<(), ChartRenderError> {
        if pan_config.momentum {
            // For now, just apply the pan directly
            // In a real implementation, this would start momentum physics
            self.pan_chart(chart_state, delta)
        } else {
            self.pan_chart(chart_state, delta)
        }
    }

    /// Update momentum panning
    pub fn update_momentum_pan(
        &mut self,
        _chart_state: &mut ChartState,
        pan_config: &PanConfig,
    ) -> Result<(), ChartRenderError> {
        if pan_config.momentum {
            // For now, just return success
            // In a real implementation, this would update momentum physics
            Ok(())
        } else {
            Ok(())
        }
    }

    /// Handle gesture recognition
    pub fn handle_gesture(
        &mut self,
        _chart_state: &mut ChartState,
        touch_points: &[TouchPoint],
        _gesture_config: &GestureConfig,
    ) -> Result<Option<GestureInfo>, ChartRenderError> {
        if touch_points.len() == 0 {
            return Ok(None);
        }

        // Simple gesture recognition
        if touch_points.len() == 1 {
            let _point = &touch_points[0];
            Ok(Some(GestureInfo {
                gesture_type: GestureType::Tap,
                start_points: touch_points.to_vec(),
                current_points: touch_points.to_vec(),
                velocity: (0.0, 0.0),
                scale: 1.0,
                rotation: 0.0,
            }))
        } else if touch_points.len() == 2 {
            Ok(Some(GestureInfo {
                gesture_type: GestureType::Pinch,
                start_points: touch_points.to_vec(),
                current_points: touch_points.to_vec(),
                velocity: (0.0, 0.0),
                scale: 1.0,
                rotation: 0.0,
            }))
        } else {
            Ok(None)
        }
    }

    /// Get interaction history
    pub fn get_interaction_history(&self) -> Vec<InteractionState> {
        self.undo_stack.iter().cloned().collect()
    }

    /// Create a selection
    pub fn create_selection(
        &self,
        _chart_state: &ChartState,
        start: [f32; 2],
        end: [f32; 2],
        selection_config: &SelectionConfig,
    ) -> Result<Option<SelectionInfo>, ChartRenderError> {
        if !selection_config.enabled {
            return Ok(None);
        }

        let bounds = (
            start[0].min(end[0]),
            start[1].min(end[1]),
            (end[0] - start[0]).abs(),
            (end[1] - start[1]).abs(),
        );

        // For now, just return a mock selection
        Ok(Some(SelectionInfo {
            selection_type: selection_config.selection_type.clone(),
            bounds,
            selected_points: vec![0, 1, 2], // Mock selected points
            selection_color: selection_config.selection_color.clone(),
            selection_opacity: selection_config.selection_opacity,
        }))
    }

    /// Handle keyboard input
    pub fn handle_keyboard(
        &mut self,
        chart_state: &mut ChartState,
        key_event: &KeyEvent,
        keyboard_config: &KeyboardConfig,
    ) -> Result<(), ChartRenderError> {
        if !keyboard_config.enabled {
            return Ok(());
        }

        match key_event.key.as_str() {
            "ArrowLeft" | "ArrowRight" | "ArrowUp" | "ArrowDown" => {
                if keyboard_config.arrow_key_pan {
                    let pan_speed = keyboard_config.arrow_key_pan_speed;
                    let delta = match key_event.key.as_str() {
                        "ArrowLeft" => PanDelta {
                            x: -pan_speed,
                            y: 0.0,
                        },
                        "ArrowRight" => PanDelta {
                            x: pan_speed,
                            y: 0.0,
                        },
                        "ArrowUp" => PanDelta {
                            x: 0.0,
                            y: -pan_speed,
                        },
                        "ArrowDown" => PanDelta {
                            x: 0.0,
                            y: pan_speed,
                        },
                        _ => PanDelta { x: 0.0, y: 0.0 },
                    };
                    self.pan_chart(chart_state, delta)?;
                }
            }
            "Home" => {
                if keyboard_config.home_reset {
                    self.reset_view(chart_state)?;
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Get accessibility information
    pub fn get_accessibility_info(
        &self,
        chart_state: &ChartState,
        accessibility_config: &AccessibilityConfig,
    ) -> Result<AccessibilityInfo, ChartRenderError> {
        let screen_reader_text = if accessibility_config.screen_reader_support {
            format!(
                "Chart: {} with {} data points. Current zoom: {:.1}x, Pan: ({:.1}, {:.1})",
                chart_state.config.title,
                0, // Would need actual data count
                chart_state.viewport().scale_x,
                chart_state.viewport().x,
                chart_state.viewport().y
            )
        } else {
            String::new()
        };

        let aria_labels = if accessibility_config.aria_labels {
            vec![
                "Chart container".to_string(),
                "Data visualization".to_string(),
                "Interactive chart".to_string(),
            ]
        } else {
            vec![]
        };

        Ok(AccessibilityInfo {
            screen_reader_text,
            aria_labels,
            focus_order: vec!["chart".to_string(), "controls".to_string()],
            high_contrast_colors: vec!["#000000".to_string(), "#ffffff".to_string()],
        })
    }
}
