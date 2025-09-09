//! Responsive Design System
//!
//! This module provides a comprehensive responsive design framework for Helios visualizations,
//! including breakpoint management, adaptive layouts, and dynamic styling.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use thiserror::Error;

use crate::theme_engine::{Breakpoint, BreakpointSet, ThemeValue};

/// Responsive design errors
#[derive(Debug, Error)]
pub enum ResponsiveError {
    #[error("Breakpoint not found: {name}")]
    BreakpointNotFound { name: String },

    #[error("Invalid breakpoint configuration: {message}")]
    InvalidBreakpointConfiguration { message: String },

    #[error("Layout calculation failed: {message}")]
    LayoutCalculationFailed { message: String },

    #[error("Media query evaluation failed: {message}")]
    MediaQueryEvaluationFailed { message: String },

    #[error("Responsive update failed: {message}")]
    ResponsiveUpdateFailed { message: String },
}

/// Viewport size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewportSize {
    pub width: u32,
    pub height: u32,
}

/// Device type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceType {
    Mobile,
    Tablet,
    Desktop,
    LargeDesktop,
    Custom(String),
}

/// Orientation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Orientation {
    Portrait,
    Landscape,
}

/// Responsive configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsiveConfig {
    pub breakpoints: BreakpointSet,
    pub default_device_type: DeviceType,
    pub enable_auto_resize: bool,
    pub resize_debounce_ms: u64,
    pub enable_orientation_change: bool,
    pub enable_device_detection: bool,
}

impl Default for ResponsiveConfig {
    fn default() -> Self {
        Self {
            breakpoints: BreakpointSet {
                breakpoints: vec![
                    Breakpoint {
                        name: "mobile".to_string(),
                        min_width: None,
                        max_width: Some(767.0),
                        styles: HashMap::new(),
                    },
                    Breakpoint {
                        name: "tablet".to_string(),
                        min_width: Some(768.0),
                        max_width: Some(1023.0),
                        styles: HashMap::new(),
                    },
                    Breakpoint {
                        name: "desktop".to_string(),
                        min_width: Some(1024.0),
                        max_width: Some(1439.0),
                        styles: HashMap::new(),
                    },
                    Breakpoint {
                        name: "large-desktop".to_string(),
                        min_width: Some(1440.0),
                        max_width: None,
                        styles: HashMap::new(),
                    },
                ],
                default: HashMap::new(),
            },
            default_device_type: DeviceType::Desktop,
            enable_auto_resize: true,
            resize_debounce_ms: 100,
            enable_orientation_change: true,
            enable_device_detection: true,
        }
    }
}

/// Responsive state
#[derive(Debug, Clone)]
pub struct ResponsiveState {
    pub viewport: ViewportSize,
    pub device_type: DeviceType,
    pub orientation: Orientation,
    pub active_breakpoint: Option<String>,
    pub media_queries: HashMap<String, bool>,
    pub last_update: std::time::Instant,
}

/// Layout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConfig {
    pub container_width: Option<f64>,
    pub container_height: Option<f64>,
    pub padding: f64,
    pub margin: f64,
    pub grid_columns: u32,
    pub grid_gap: f64,
    pub flex_direction: FlexDirection,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub flex_wrap: FlexWrap,
}

/// Flex direction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

/// Justify content
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

/// Align items
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

/// Flex wrap
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

/// Layout calculation result
#[derive(Debug, Clone)]
pub struct LayoutResult {
    pub element_positions: HashMap<String, ElementPosition>,
    pub container_size: ViewportSize,
    pub grid_layout: Option<GridLayout>,
    pub flex_layout: Option<FlexLayout>,
}

/// Element position
#[derive(Debug, Clone)]
pub struct ElementPosition {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub z_index: i32,
}

/// Grid layout
#[derive(Debug, Clone)]
pub struct GridLayout {
    pub columns: u32,
    pub rows: u32,
    pub cell_width: f64,
    pub cell_height: f64,
    pub gap: f64,
}

/// Flex layout
#[derive(Debug, Clone)]
pub struct FlexLayout {
    pub direction: FlexDirection,
    pub main_axis_size: f64,
    pub cross_axis_size: f64,
    pub item_sizes: Vec<f64>,
    pub item_positions: Vec<f64>,
}

/// Responsive manager
#[derive(Debug)]
pub struct ResponsiveManager {
    config: ResponsiveConfig,
    state: Arc<Mutex<ResponsiveState>>,
    layout_configs: HashMap<DeviceType, LayoutConfig>,
    media_query_listeners: Vec<Box<dyn Fn(&ResponsiveState) + Send + Sync>>,
}

impl ResponsiveManager {
    /// Create a new responsive manager
    pub fn new() -> Self {
        Self {
            config: ResponsiveConfig::default(),
            state: Arc::new(Mutex::new(ResponsiveState {
                viewport: ViewportSize {
                    width: 1024,
                    height: 768,
                },
                device_type: DeviceType::Desktop,
                orientation: Orientation::Landscape,
                active_breakpoint: None,
                media_queries: HashMap::new(),
                last_update: std::time::Instant::now(),
            })),
            layout_configs: HashMap::new(),
            media_query_listeners: Vec::new(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: ResponsiveConfig) -> Self {
        Self {
            config,
            state: Arc::new(Mutex::new(ResponsiveState {
                viewport: ViewportSize {
                    width: 1024,
                    height: 768,
                },
                device_type: DeviceType::Desktop,
                orientation: Orientation::Landscape,
                active_breakpoint: None,
                media_queries: HashMap::new(),
                last_update: std::time::Instant::now(),
            })),
            layout_configs: HashMap::new(),
            media_query_listeners: Vec::new(),
        }
    }

    /// Update viewport size
    pub fn update_viewport(&mut self, viewport: ViewportSize) -> Result<(), ResponsiveError> {
        let mut state = self.state.lock().unwrap();

        // Update viewport
        state.viewport = viewport;

        // Detect device type
        state.device_type = self.detect_device_type(viewport);

        // Detect orientation
        state.orientation = if viewport.width > viewport.height {
            Orientation::Landscape
        } else {
            Orientation::Portrait
        };

        // Find active breakpoint
        state.active_breakpoint = self.find_active_breakpoint(viewport);

        // Update media queries
        self.update_media_queries(&mut state)?;

        // Update timestamp
        state.last_update = std::time::Instant::now();

        // Notify listeners
        self.notify_listeners(&state);

        Ok(())
    }

    /// Get current responsive state
    pub fn get_state(&self) -> ResponsiveState {
        self.state.lock().unwrap().clone()
    }

    /// Get active breakpoint
    pub fn get_active_breakpoint(&self) -> Option<&Breakpoint> {
        let state = self.state.lock().unwrap();
        if let Some(ref name) = state.active_breakpoint {
            self.config
                .breakpoints
                .breakpoints
                .iter()
                .find(|bp| bp.name == *name)
        } else {
            None
        }
    }

    /// Get device-specific layout configuration
    pub fn get_layout_config(&self, device_type: &DeviceType) -> Option<&LayoutConfig> {
        self.layout_configs.get(device_type)
    }

    /// Set device-specific layout configuration
    pub fn set_layout_config(&mut self, device_type: DeviceType, config: LayoutConfig) {
        self.layout_configs.insert(device_type, config);
    }

    /// Calculate layout for current viewport
    pub fn calculate_layout(
        &self,
        elements: &[LayoutElement],
    ) -> Result<LayoutResult, ResponsiveError> {
        let state = self.state.lock().unwrap();
        let layout_config = self.layout_configs.get(&state.device_type).ok_or_else(|| {
            ResponsiveError::LayoutCalculationFailed {
                message: "No layout configuration found for current device type".to_string(),
            }
        })?;

        match layout_config.flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
                self.calculate_flex_layout(elements, layout_config, &state)
            }
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                self.calculate_flex_layout(elements, layout_config, &state)
            }
        }
    }

    /// Add media query listener
    pub fn add_media_query_listener(
        &mut self,
        listener: Box<dyn Fn(&ResponsiveState) + Send + Sync>,
    ) {
        self.media_query_listeners.push(listener);
    }

    /// Check if media query matches
    pub fn matches_media_query(&self, query: &str) -> Result<bool, ResponsiveError> {
        let state = self.state.lock().unwrap();
        self.evaluate_media_query(query, &state)
    }

    /// Get responsive styles for current breakpoint
    pub fn get_responsive_styles(
        &self,
        base_styles: &HashMap<String, ThemeValue>,
    ) -> HashMap<String, ThemeValue> {
        let state = self.state.lock().unwrap();
        let mut styles = base_styles.clone();

        // Apply breakpoint-specific styles
        if let Some(ref breakpoint_name) = state.active_breakpoint {
            if let Some(breakpoint) = self
                .config
                .breakpoints
                .breakpoints
                .iter()
                .find(|bp| bp.name == *breakpoint_name)
            {
                for (key, value) in &breakpoint.styles {
                    styles.insert(key.clone(), value.clone());
                }
            }
        }

        // Apply device-specific styles
        if let Some(layout_config) = self.layout_configs.get(&state.device_type) {
            // Add device-specific style overrides
            styles.insert(
                "device-type".to_string(),
                ThemeValue::String(format!("{:?}", state.device_type)),
            );
            styles.insert(
                "orientation".to_string(),
                ThemeValue::String(format!("{:?}", state.orientation)),
            );
        }

        styles
    }

    /// Detect device type from viewport
    fn detect_device_type(&self, viewport: ViewportSize) -> DeviceType {
        for breakpoint in &self.config.breakpoints.breakpoints {
            let matches = match (breakpoint.min_width, breakpoint.max_width) {
                (Some(min), Some(max)) => {
                    viewport.width as f64 >= min && viewport.width as f64 <= max
                }
                (Some(min), None) => viewport.width as f64 >= min,
                (None, Some(max)) => viewport.width as f64 <= max,
                (None, None) => true,
            };

            if matches {
                match breakpoint.name.as_str() {
                    "mobile" => return DeviceType::Mobile,
                    "tablet" => return DeviceType::Tablet,
                    "desktop" => return DeviceType::Desktop,
                    "large-desktop" => return DeviceType::LargeDesktop,
                    _ => return DeviceType::Custom(breakpoint.name.clone()),
                }
            }
        }

        self.config.default_device_type.clone()
    }

    /// Find active breakpoint
    fn find_active_breakpoint(&self, viewport: ViewportSize) -> Option<String> {
        for breakpoint in &self.config.breakpoints.breakpoints {
            let matches = match (breakpoint.min_width, breakpoint.max_width) {
                (Some(min), Some(max)) => {
                    viewport.width as f64 >= min && viewport.width as f64 <= max
                }
                (Some(min), None) => viewport.width as f64 >= min,
                (None, Some(max)) => viewport.width as f64 <= max,
                (None, None) => true,
            };

            if matches {
                return Some(breakpoint.name.clone());
            }
        }

        None
    }

    /// Update media queries
    fn update_media_queries(&self, state: &mut ResponsiveState) -> Result<(), ResponsiveError> {
        state.media_queries.clear();

        // Add common media queries
        state.media_queries.insert(
            "mobile".to_string(),
            state.device_type == DeviceType::Mobile,
        );
        state.media_queries.insert(
            "tablet".to_string(),
            state.device_type == DeviceType::Tablet,
        );
        state.media_queries.insert(
            "desktop".to_string(),
            state.device_type == DeviceType::Desktop,
        );
        state.media_queries.insert(
            "large-desktop".to_string(),
            state.device_type == DeviceType::LargeDesktop,
        );
        state.media_queries.insert(
            "portrait".to_string(),
            state.orientation == Orientation::Portrait,
        );
        state.media_queries.insert(
            "landscape".to_string(),
            state.orientation == Orientation::Landscape,
        );

        // Add breakpoint-specific media queries
        for breakpoint in &self.config.breakpoints.breakpoints {
            let matches = match (breakpoint.min_width, breakpoint.max_width) {
                (Some(min), Some(max)) => {
                    state.viewport.width as f64 >= min && state.viewport.width as f64 <= max
                }
                (Some(min), None) => state.viewport.width as f64 >= min,
                (None, Some(max)) => state.viewport.width as f64 <= max,
                (None, None) => true,
            };

            state.media_queries.insert(breakpoint.name.clone(), matches);
        }

        Ok(())
    }

    /// Calculate flex layout
    fn calculate_flex_layout(
        &self,
        elements: &[LayoutElement],
        config: &LayoutConfig,
        state: &ResponsiveState,
    ) -> Result<LayoutResult, ResponsiveError> {
        let mut element_positions = HashMap::new();
        let mut item_sizes = Vec::new();
        let mut item_positions = Vec::new();

        // Calculate available space
        let available_width = state.viewport.width as f64 - (config.padding * 2.0);
        let available_height = state.viewport.height as f64 - (config.padding * 2.0);

        // Calculate item sizes based on flex direction
        let (main_axis_size, cross_axis_size) = match config.flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => (available_width, available_height),
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                (available_height, available_width)
            }
        };

        // Distribute space among elements
        let total_flex = elements.iter().map(|e| e.flex_grow).sum::<f64>();
        let mut current_position = 0.0;

        for (i, element) in elements.iter().enumerate() {
            let element_size = if total_flex > 0.0 {
                (element.flex_grow / total_flex) * main_axis_size
            } else {
                element
                    .size
                    .unwrap_or(main_axis_size / elements.len() as f64)
            };

            item_sizes.push(element_size);
            item_positions.push(current_position);

            // Calculate element position
            let (x, y, width, height) = match config.flex_direction {
                FlexDirection::Row | FlexDirection::RowReverse => {
                    let x = if config.flex_direction == FlexDirection::RowReverse {
                        available_width - current_position - element_size
                    } else {
                        current_position
                    };
                    (x, 0.0, element_size, cross_axis_size)
                }
                FlexDirection::Column | FlexDirection::ColumnReverse => {
                    let y = if config.flex_direction == FlexDirection::ColumnReverse {
                        available_height - current_position - element_size
                    } else {
                        current_position
                    };
                    (0.0, y, cross_axis_size, element_size)
                }
            };

            element_positions.insert(
                element.id.clone(),
                ElementPosition {
                    x: x + config.padding,
                    y: y + config.padding,
                    width,
                    height,
                    z_index: element.z_index,
                },
            );

            current_position += element_size + config.grid_gap;
        }

        let flex_layout = FlexLayout {
            direction: config.flex_direction.clone(),
            main_axis_size,
            cross_axis_size,
            item_sizes,
            item_positions,
        };

        Ok(LayoutResult {
            element_positions,
            container_size: state.viewport,
            grid_layout: None,
            flex_layout: Some(flex_layout),
        })
    }

    /// Evaluate media query
    fn evaluate_media_query(
        &self,
        query: &str,
        state: &ResponsiveState,
    ) -> Result<bool, ResponsiveError> {
        // Simple media query evaluation - can be enhanced
        if let Some(result) = state.media_queries.get(query) {
            return Ok(*result);
        }

        // Parse width-based queries
        if query.starts_with("(min-width:") {
            if let Some(width_str) = query
                .strip_prefix("(min-width:")
                .and_then(|s| s.strip_suffix("px)"))
            {
                if let Ok(width) = width_str.parse::<f64>() {
                    return Ok(state.viewport.width as f64 >= width);
                }
            }
        }

        if query.starts_with("(max-width:") {
            if let Some(width_str) = query
                .strip_prefix("(max-width:")
                .and_then(|s| s.strip_suffix("px)"))
            {
                if let Ok(width) = width_str.parse::<f64>() {
                    return Ok(state.viewport.width as f64 <= width);
                }
            }
        }

        // Parse height-based queries
        if query.starts_with("(min-height:") {
            if let Some(height_str) = query
                .strip_prefix("(min-height:")
                .and_then(|s| s.strip_suffix("px)"))
            {
                if let Ok(height) = height_str.parse::<f64>() {
                    return Ok(state.viewport.height as f64 >= height);
                }
            }
        }

        if query.starts_with("(max-height:") {
            if let Some(height_str) = query
                .strip_prefix("(max-height:")
                .and_then(|s| s.strip_suffix("px)"))
            {
                if let Ok(height) = height_str.parse::<f64>() {
                    return Ok(state.viewport.height as f64 <= height);
                }
            }
        }

        // Parse orientation queries
        if query == "(orientation: portrait)" {
            return Ok(state.orientation == Orientation::Portrait);
        }

        if query == "(orientation: landscape)" {
            return Ok(state.orientation == Orientation::Landscape);
        }

        Err(ResponsiveError::MediaQueryEvaluationFailed {
            message: format!("Unsupported media query: {}", query),
        })
    }

    /// Notify media query listeners
    fn notify_listeners(&self, state: &ResponsiveState) {
        for listener in &self.media_query_listeners {
            listener(state);
        }
    }
}

/// Layout element
#[derive(Debug, Clone)]
pub struct LayoutElement {
    pub id: String,
    pub size: Option<f64>,
    pub flex_grow: f64,
    pub flex_shrink: f64,
    pub flex_basis: Option<f64>,
    pub z_index: i32,
    pub min_size: Option<f64>,
    pub max_size: Option<f64>,
}

impl Default for ResponsiveManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Create default layout configurations
pub fn create_default_layout_configs() -> HashMap<DeviceType, LayoutConfig> {
    let mut configs = HashMap::new();

    // Mobile layout
    configs.insert(
        DeviceType::Mobile,
        LayoutConfig {
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
        },
    );

    // Tablet layout
    configs.insert(
        DeviceType::Tablet,
        LayoutConfig {
            container_width: None,
            container_height: None,
            padding: 24.0,
            margin: 16.0,
            grid_columns: 2,
            grid_gap: 16.0,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            flex_wrap: FlexWrap::Wrap,
        },
    );

    // Desktop layout
    configs.insert(
        DeviceType::Desktop,
        LayoutConfig {
            container_width: None,
            container_height: None,
            padding: 32.0,
            margin: 24.0,
            grid_columns: 3,
            grid_gap: 24.0,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::FlexStart,
            flex_wrap: FlexWrap::NoWrap,
        },
    );

    // Large desktop layout
    configs.insert(
        DeviceType::LargeDesktop,
        LayoutConfig {
            container_width: None,
            container_height: None,
            padding: 48.0,
            margin: 32.0,
            grid_columns: 4,
            grid_gap: 32.0,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::FlexStart,
            flex_wrap: FlexWrap::NoWrap,
        },
    );

    configs
}
