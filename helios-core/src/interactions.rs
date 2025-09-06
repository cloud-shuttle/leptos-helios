//! User Interaction System
//! Handles pan, zoom, hover, click, and other user interactions with charts

use crate::chart_config::*;
use std::collections::VecDeque;

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

        let viewport = chart_state.viewport();

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

        let viewport = chart_state.viewport();

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
