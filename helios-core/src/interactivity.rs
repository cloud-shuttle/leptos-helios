//! Core Interactivity Features for leptos-helios
//!
//! This module implements the core interactivity features identified in the feature gap analysis:
//! - Zoom & Pan functionality
//! - Rich tooltips with contextual information
//! - Brush selection for data filtering
//! - Cross-filtering between charts
//!
//! Following TDD methodology: Red -> Green -> Refactor

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Viewport management for zoom and pan operations
#[derive(Debug, Clone, PartialEq)]
pub struct Viewport {
    /// X offset in world coordinates
    pub x: f64,
    /// Y offset in world coordinates
    pub y: f64,
    /// Scale factor (1.0 = no zoom)
    pub scale: f64,
    /// Viewport width in screen coordinates
    pub width: f64,
    /// Viewport height in screen coordinates
    pub height: f64,
    /// Minimum X bound
    min_x: Option<f64>,
    /// Maximum X bound
    max_x: Option<f64>,
    /// Minimum Y bound
    min_y: Option<f64>,
    /// Maximum Y bound
    max_y: Option<f64>,
}

impl Viewport {
    /// Create a new viewport with default values
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            scale: 1.0,
            width,
            height,
            min_x: None,
            max_x: None,
            min_y: None,
            max_y: None,
        }
    }

    /// Set bounds for the viewport
    pub fn set_bounds(&mut self, min_x: f64, min_y: f64, max_x: f64, max_y: f64) {
        self.min_x = Some(min_x);
        self.min_y = Some(min_y);
        self.max_x = Some(max_x);
        self.max_y = Some(max_y);
    }

    /// Zoom the viewport by a factor at a specific center point
    pub fn zoom(&mut self, factor: f64, center_x: f64, center_y: f64) {
        let old_scale = self.scale;
        self.scale *= factor;

        // Clamp scale to reasonable bounds
        self.scale = self.scale.max(0.1).min(10.0);

        // Adjust position to keep zoom center fixed
        // The center point in world coordinates should remain the same
        let world_center_x = (center_x - self.x) / old_scale;
        let world_center_y = (center_y - self.y) / old_scale;

        // Update viewport position to keep the world center at the same screen position
        self.x = center_x - world_center_x * self.scale;
        self.y = center_y - world_center_y * self.scale;

        self.clamp_to_bounds();
    }

    /// Pan the viewport by the given offset
    pub fn pan(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
        self.clamp_to_bounds();
    }

    /// Clamp viewport position to bounds
    fn clamp_to_bounds(&mut self) {
        if let Some(min_x) = self.min_x {
            self.x = self.x.max(min_x);
        }
        if let Some(max_x) = self.max_x {
            self.x = self.x.min(max_x);
        }
        if let Some(min_y) = self.min_y {
            self.y = self.y.max(min_y);
        }
        if let Some(max_y) = self.max_y {
            self.y = self.y.min(max_y);
        }
    }

    /// Transform screen coordinates to world coordinates
    pub fn screen_to_world(&self, screen_x: f64, screen_y: f64) -> (f64, f64) {
        let world_x = (screen_x - self.x) / self.scale;
        let world_y = (screen_y - self.y) / self.scale;
        (world_x, world_y)
    }

    /// Transform world coordinates to screen coordinates
    pub fn world_to_screen(&self, world_x: f64, world_y: f64) -> (f64, f64) {
        let screen_x = world_x * self.scale + self.x;
        let screen_y = world_y * self.scale + self.y;
        (screen_x, screen_y)
    }
}

/// Rich tooltip with contextual information
#[derive(Debug, Clone)]
pub struct Tooltip {
    /// Tooltip content
    pub content: String,
    /// Position on screen
    pub position: (f64, f64),
    /// Whether tooltip is visible
    pub visible: bool,
    /// Tooltip style
    pub style: TooltipStyle,
}

#[derive(Debug, Clone)]
pub struct TooltipStyle {
    /// Background color
    pub background_color: String,
    /// Text color
    pub text_color: String,
    /// Border color
    pub border_color: String,
    /// Font size
    pub font_size: f64,
    /// Padding
    pub padding: f64,
}

impl Default for TooltipStyle {
    fn default() -> Self {
        Self {
            background_color: "rgba(0, 0, 0, 0.8)".to_string(),
            text_color: "white".to_string(),
            border_color: "rgba(255, 255, 255, 0.2)".to_string(),
            font_size: 12.0,
            padding: 8.0,
        }
    }
}

impl Tooltip {
    /// Create a new tooltip
    pub fn new(content: String, position: (f64, f64)) -> Self {
        Self {
            content,
            position,
            visible: false,
            style: TooltipStyle::default(),
        }
    }

    /// Show the tooltip
    pub fn show(&mut self) {
        self.visible = true;
    }

    /// Hide the tooltip
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Update tooltip position
    pub fn update_position(&mut self, x: f64, y: f64) {
        self.position = (x, y);
    }

    /// Auto-position tooltip to stay within viewport bounds
    pub fn auto_position(&mut self, viewport: &Viewport) {
        let (x, y) = self.position;
        let tooltip_width = 200.0; // Estimate
        let tooltip_height = 100.0; // Estimate

        let new_x = if x + tooltip_width > viewport.width {
            x - tooltip_width
        } else {
            x
        };

        let new_y = if y + tooltip_height > viewport.height {
            y - tooltip_height
        } else {
            y
        };

        self.position = (new_x.max(0.0), new_y.max(0.0));
    }

    /// Create tooltip from structured data
    pub fn from_data(data: TooltipData, position: (f64, f64)) -> Self {
        let mut content = format!("**{}**\n", data.title);

        for (key, value) in &data.values {
            content.push_str(&format!("{}: {}\n", key, value));
        }

        content.push_str(&format!("\n*{}*", data.timestamp));

        Self::new(content, position)
    }
}

/// Structured data for tooltips
#[derive(Debug, Clone)]
pub struct TooltipData {
    /// Tooltip title
    pub title: String,
    /// Key-value pairs
    pub values: Vec<(String, String)>,
    /// Timestamp
    pub timestamp: String,
}

/// Brush selection for data filtering
#[derive(Debug, Clone, PartialEq)]
pub struct BrushSelection {
    /// Start X coordinate
    pub x1: f64,
    /// Start Y coordinate
    pub y1: f64,
    /// End X coordinate
    pub x2: f64,
    /// End Y coordinate
    pub y2: f64,
}

impl BrushSelection {
    /// Create a new brush selection
    pub fn new(start: (f64, f64), end: (f64, f64)) -> Self {
        Self {
            x1: start.0,
            y1: start.1,
            x2: end.0,
            y2: end.1,
        }
    }

    /// Get normalized brush (ensuring x1,y1 is top-left and x2,y2 is bottom-right)
    pub fn normalized(&self) -> Self {
        Self {
            x1: self.x1.min(self.x2),
            y1: self.y1.min(self.y2),
            x2: self.x1.max(self.x2),
            y2: self.y1.max(self.y2),
        }
    }

    /// Check if a point is contained within the brush
    pub fn contains_point(&self, x: f64, y: f64) -> bool {
        let normalized = self.normalized();
        x >= normalized.x1 && x <= normalized.x2 && y >= normalized.y1 && y <= normalized.y2
    }

    /// Check if brush intersects with a rectangle
    pub fn intersects_rect(&self, rect: (f64, f64, f64, f64)) -> bool {
        let (rect_x, rect_y, rect_width, rect_height) = rect;
        let normalized = self.normalized();

        !(normalized.x2 < rect_x
            || normalized.x1 > rect_x + rect_width
            || normalized.y2 < rect_y
            || normalized.y1 > rect_y + rect_height)
    }

    /// Filter data points based on brush selection
    pub fn filter_data(&self, data_points: &[DataPoint]) -> Vec<DataPoint> {
        data_points
            .iter()
            .filter(|point| self.contains_point(point.x, point.y))
            .cloned()
            .collect()
    }

    /// Check if brush is empty
    pub fn is_empty(&self) -> bool {
        self.x1 == self.x2 && self.y1 == self.y2
    }

    /// Clear the brush selection
    pub fn clear(&mut self) {
        self.x1 = 0.0;
        self.y1 = 0.0;
        self.x2 = 0.0;
        self.y2 = 0.0;
    }
}

/// Cross-filtering between multiple charts
#[derive(Debug)]
pub struct CrossFilter {
    /// Chart IDs that participate in cross-filtering
    pub charts: Vec<String>,
    /// Active filters by chart ID
    pub active_filters: HashMap<String, BrushSelection>,
    /// Filter events to propagate
    filter_events: Vec<FilterEvent>,
}

#[derive(Debug, Clone)]
pub struct FilterEvent {
    /// Target chart ID
    pub target_chart: String,
    /// Filter to apply
    pub filter: Option<BrushSelection>,
}

impl CrossFilter {
    /// Create a new cross-filter
    pub fn new(chart_ids: Vec<String>) -> Self {
        Self {
            charts: chart_ids,
            active_filters: HashMap::new(),
            filter_events: Vec::new(),
        }
    }

    /// Add a filter for a specific chart
    pub fn add_filter(&mut self, chart_id: &str, brush: BrushSelection) {
        self.active_filters
            .insert(chart_id.to_string(), brush.clone());

        // Propagate filter to other charts
        for other_chart in &self.charts {
            if other_chart != chart_id {
                self.filter_events.push(FilterEvent {
                    target_chart: other_chart.clone(),
                    filter: Some(brush.clone()),
                });
            }
        }
    }

    /// Remove filter for a specific chart
    pub fn remove_filter(&mut self, chart_id: &str) {
        self.active_filters.remove(chart_id);

        // Propagate filter removal to other charts
        for other_chart in &self.charts {
            if other_chart != chart_id {
                self.filter_events.push(FilterEvent {
                    target_chart: other_chart.clone(),
                    filter: None,
                });
            }
        }
    }

    /// Check if a chart has an active filter
    pub fn has_filter(&self, chart_id: &str) -> bool {
        self.active_filters.contains_key(chart_id)
    }

    /// Get filter events to propagate
    pub fn get_filter_events(&mut self) -> Vec<FilterEvent> {
        let events = self.filter_events.clone();
        self.filter_events.clear();
        events
    }

    /// Clear all filters
    pub fn clear_all(&mut self) {
        self.active_filters.clear();
        self.filter_events.clear();
    }

    /// Get filtered data based on active filters
    pub fn get_filtered_data(&self, shared_data: &Arc<Mutex<Vec<DataPoint>>>) -> Vec<DataPoint> {
        let data = shared_data.lock().unwrap();
        let mut filtered_data = data.clone();

        // Apply all active filters
        for brush in self.active_filters.values() {
            filtered_data = brush.filter_data(&filtered_data);
        }

        filtered_data
    }
}

/// Data point for testing and filtering
#[derive(Debug, Clone, PartialEq)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
    pub value: f64,
}

/// Interactive chart that combines all interactivity features
#[derive(Debug)]
pub struct InteractiveChart {
    /// Viewport for zoom and pan
    pub viewport: Viewport,
    /// Tooltip for data display
    pub tooltip: Tooltip,
    /// Brush selection for filtering
    pub brush: BrushSelection,
    /// Cross-filter for multi-chart coordination
    pub cross_filter: Option<CrossFilter>,
    /// Chart data
    pub data: Vec<DataPoint>,
    /// Shared data for cross-filtering
    pub shared_data: Option<Arc<Mutex<Vec<DataPoint>>>>,
}

impl InteractiveChart {
    /// Create a new interactive chart
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            viewport: Viewport::new(width, height),
            tooltip: Tooltip::new(String::new(), (0.0, 0.0)),
            brush: BrushSelection::new((0.0, 0.0), (0.0, 0.0)),
            cross_filter: None,
            data: Vec::new(),
            shared_data: None,
        }
    }

    /// Set chart data
    pub fn set_data(&mut self, data: Vec<DataPoint>) {
        self.data = data;
    }

    /// Set shared data for cross-filtering
    pub fn set_shared_data(&mut self, shared_data: Arc<Mutex<Vec<DataPoint>>>) {
        self.shared_data = Some(shared_data);
    }

    /// Zoom the chart
    pub fn zoom(&mut self, factor: f64, center_x: f64, center_y: f64) {
        self.viewport.zoom(factor, center_x, center_y);
    }

    /// Pan the chart
    pub fn pan(&mut self, dx: f64, dy: f64) {
        self.viewport.pan(dx, dy);
    }

    /// Show tooltip at position
    pub fn show_tooltip(&mut self, x: f64, y: f64) {
        self.tooltip.update_position(x, y);
        self.tooltip.auto_position(&self.viewport);
        self.tooltip.show();
    }

    /// Start brush selection
    pub fn start_brush_selection(&mut self, x: f64, y: f64) {
        self.brush = BrushSelection::new((x, y), (x, y));
    }

    /// Update brush selection
    pub fn update_brush_selection(&mut self, x: f64, y: f64) {
        self.brush.x2 = x;
        self.brush.y2 = y;
    }

    /// Finish brush selection
    pub fn finish_brush_selection(&mut self) {
        // Brush selection is complete
    }

    /// Apply brush filter
    pub fn apply_brush_filter(&mut self, brush: BrushSelection) -> Vec<DataPoint> {
        self.brush = brush.clone();

        if let Some(ref mut cross_filter) = self.cross_filter {
            cross_filter.add_filter("current_chart", brush.clone());
        }

        // Use shared data if available, otherwise use local data
        if let Some(ref shared_data) = self.shared_data {
            if let Some(ref cross_filter) = self.cross_filter {
                cross_filter.get_filtered_data(shared_data)
            } else {
                self.brush.filter_data(&shared_data.lock().unwrap())
            }
        } else {
            self.brush.filter_data(&self.data)
        }
    }

    /// Get filtered data
    pub fn get_filtered_data(&self) -> Vec<DataPoint> {
        if let Some(ref shared_data) = self.shared_data {
            if let Some(ref cross_filter) = self.cross_filter {
                cross_filter.get_filtered_data(shared_data)
            } else {
                self.brush.filter_data(&self.data)
            }
        } else {
            self.brush.filter_data(&self.data)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewport_initialization() {
        let viewport = Viewport::new(800.0, 600.0);
        assert_eq!(viewport.x, 0.0);
        assert_eq!(viewport.y, 0.0);
        assert_eq!(viewport.scale, 1.0);
        assert_eq!(viewport.width, 800.0);
        assert_eq!(viewport.height, 600.0);
    }

    #[test]
    fn test_viewport_zoom_in() {
        let mut viewport = Viewport::new(800.0, 600.0);
        viewport.zoom(2.0, 400.0, 300.0);
        assert_eq!(viewport.scale, 2.0);
        // When zooming 2x at center (400, 300), the viewport should be positioned
        // so that the center point remains at the same screen position
        // world_center_x = (400 - 0) / 1.0 = 400
        // world_center_y = (300 - 0) / 1.0 = 300
        // new x = 400 - 400 * 2.0 = -400
        // new y = 300 - 300 * 2.0 = -300
        assert_eq!(viewport.x, -400.0);
        assert_eq!(viewport.y, -300.0);
    }

    #[test]
    fn test_viewport_zoom_out() {
        let mut viewport = Viewport::new(800.0, 600.0);
        viewport.scale = 2.0;
        viewport.x = 100.0;
        viewport.y = 100.0;
        viewport.zoom(0.5, 400.0, 300.0);
        assert_eq!(viewport.scale, 1.0);
    }

    #[test]
    fn test_viewport_pan() {
        let mut viewport = Viewport::new(800.0, 600.0);
        viewport.pan(50.0, 30.0);
        assert_eq!(viewport.x, 50.0);
        assert_eq!(viewport.y, 30.0);
    }

    #[test]
    fn test_viewport_bounds_checking() {
        let mut viewport = Viewport::new(800.0, 600.0);
        viewport.set_bounds(0.0, 0.0, 1600.0, 1200.0);
        viewport.pan(-100.0, -100.0);
        assert_eq!(viewport.x, 0.0);
        assert_eq!(viewport.y, 0.0);
    }

    #[test]
    fn test_viewport_coordinate_transformation() {
        let mut viewport = Viewport::new(800.0, 600.0);
        viewport.x = 100.0;
        viewport.y = 50.0;
        viewport.scale = 2.0;
        let world_pos = viewport.screen_to_world(200.0, 150.0);
        assert_eq!(world_pos.0, 50.0);
        assert_eq!(world_pos.1, 50.0);
    }

    #[test]
    fn test_viewport_world_to_screen_transformation() {
        let mut viewport = Viewport::new(800.0, 600.0);
        viewport.x = 100.0;
        viewport.y = 50.0;
        viewport.scale = 2.0;
        let screen_pos = viewport.world_to_screen(50.0, 25.0);
        assert_eq!(screen_pos.0, 200.0);
        assert_eq!(screen_pos.1, 100.0);
    }

    #[test]
    fn test_tooltip_creation() {
        let content = "Value: 42.5\nCategory: A\nTimestamp: 2025-01-01";
        let position = (100.0, 200.0);
        let tooltip = Tooltip::new(content.to_string(), position);
        assert_eq!(tooltip.content, content);
        assert_eq!(tooltip.position, position);
        assert_eq!(tooltip.visible, false);
    }

    #[test]
    fn test_tooltip_show_hide() {
        let mut tooltip = Tooltip::new("Test".to_string(), (0.0, 0.0));
        tooltip.show();
        assert_eq!(tooltip.visible, true);
        tooltip.hide();
        assert_eq!(tooltip.visible, false);
    }

    #[test]
    fn test_tooltip_position_update() {
        let mut tooltip = Tooltip::new("Test".to_string(), (0.0, 0.0));
        tooltip.update_position(150.0, 250.0);
        assert_eq!(tooltip.position, (150.0, 250.0));
    }

    #[test]
    fn test_tooltip_auto_positioning() {
        let mut tooltip = Tooltip::new("Test".to_string(), (100.0, 100.0));
        let viewport = Viewport::new(800.0, 600.0);
        tooltip.auto_position(&viewport);
        assert!(tooltip.position.0 >= 0.0);
        assert!(tooltip.position.1 >= 0.0);
        assert!(tooltip.position.0 <= viewport.width);
        assert!(tooltip.position.1 <= viewport.height);
    }

    #[test]
    fn test_tooltip_rich_content() {
        let data = TooltipData {
            title: "Stock Price".to_string(),
            values: vec![
                ("Price".to_string(), "125.50".to_string()),
                ("Volume".to_string(), "1,250,000".to_string()),
                ("Change".to_string(), "+2.5%".to_string()),
            ],
            timestamp: "2025-01-01 12:00:00".to_string(),
        };
        let tooltip = Tooltip::from_data(data, (100.0, 100.0));
        assert!(tooltip.content.contains("Stock Price"));
        assert!(tooltip.content.contains("Price: 125.50"));
        assert!(tooltip.content.contains("Volume: 1,250,000"));
        assert!(tooltip.content.contains("Change: +2.5%"));
    }

    #[test]
    fn test_brush_creation() {
        let start = (100.0, 100.0);
        let end = (200.0, 200.0);
        let brush = BrushSelection::new(start, end);
        assert_eq!(brush.x1, 100.0);
        assert_eq!(brush.y1, 100.0);
        assert_eq!(brush.x2, 200.0);
        assert_eq!(brush.y2, 200.0);
    }

    #[test]
    fn test_brush_normalization() {
        let brush = BrushSelection::new((200.0, 200.0), (100.0, 100.0));
        let normalized = brush.normalized();
        assert_eq!(normalized.x1, 100.0);
        assert_eq!(normalized.y1, 100.0);
        assert_eq!(normalized.x2, 200.0);
        assert_eq!(normalized.y2, 200.0);
    }

    #[test]
    fn test_brush_contains_point() {
        let brush = BrushSelection::new((100.0, 100.0), (200.0, 200.0));
        let inside = brush.contains_point(150.0, 150.0);
        let outside = brush.contains_point(50.0, 50.0);
        assert_eq!(inside, true);
        assert_eq!(outside, false);
    }

    #[test]
    fn test_brush_intersects_rect() {
        let brush = BrushSelection::new((100.0, 100.0), (200.0, 200.0));
        let rect = (150.0, 150.0, 100.0, 100.0); // x, y, width, height
        let intersects = brush.intersects_rect(rect);
        assert_eq!(intersects, true);
    }

    #[test]
    fn test_brush_data_filtering() {
        let data_points = vec![
            DataPoint {
                x: 50.0,
                y: 50.0,
                value: 10.0,
            },
            DataPoint {
                x: 150.0,
                y: 150.0,
                value: 20.0,
            },
            DataPoint {
                x: 250.0,
                y: 250.0,
                value: 30.0,
            },
        ];
        let brush = BrushSelection::new((100.0, 100.0), (200.0, 200.0));
        let filtered = brush.filter_data(&data_points);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].value, 20.0);
    }

    #[test]
    fn test_brush_clear() {
        let mut brush = BrushSelection::new((100.0, 100.0), (200.0, 200.0));
        brush.clear();
        assert_eq!(brush.is_empty(), true);
    }

    #[test]
    fn test_cross_filter_initialization() {
        let chart_ids = vec!["chart1".to_string(), "chart2".to_string()];
        let cross_filter = CrossFilter::new(chart_ids);
        assert_eq!(cross_filter.charts.len(), 2);
        assert_eq!(cross_filter.active_filters.len(), 0);
    }

    #[test]
    fn test_cross_filter_add_filter() {
        let mut cross_filter = CrossFilter::new(vec!["chart1".to_string()]);
        let brush = BrushSelection::new((100.0, 100.0), (200.0, 200.0));
        cross_filter.add_filter("chart1", brush);
        assert_eq!(cross_filter.active_filters.len(), 1);
        assert!(cross_filter.has_filter("chart1"));
    }

    #[test]
    fn test_cross_filter_remove_filter() {
        let mut cross_filter = CrossFilter::new(vec!["chart1".to_string()]);
        let brush = BrushSelection::new((100.0, 100.0), (200.0, 200.0));
        cross_filter.add_filter("chart1", brush);
        cross_filter.remove_filter("chart1");
        assert_eq!(cross_filter.active_filters.len(), 0);
        assert!(!cross_filter.has_filter("chart1"));
    }

    #[test]
    fn test_cross_filter_propagate_filter() {
        let mut cross_filter = CrossFilter::new(vec!["chart1".to_string(), "chart2".to_string()]);
        let brush = BrushSelection::new((100.0, 100.0), (200.0, 200.0));
        cross_filter.add_filter("chart1", brush);
        let events = cross_filter.get_filter_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].target_chart, "chart2");
    }

    #[test]
    fn test_cross_filter_clear_all() {
        let mut cross_filter = CrossFilter::new(vec!["chart1".to_string(), "chart2".to_string()]);
        let brush1 = BrushSelection::new((100.0, 100.0), (200.0, 200.0));
        let brush2 = BrushSelection::new((300.0, 300.0), (400.0, 400.0));
        cross_filter.add_filter("chart1", brush1);
        cross_filter.add_filter("chart2", brush2);
        cross_filter.clear_all();
        assert_eq!(cross_filter.active_filters.len(), 0);
    }

    #[test]
    fn test_cross_filter_data_synchronization() {
        let mut cross_filter = CrossFilter::new(vec!["chart1".to_string(), "chart2".to_string()]);
        let shared_data = Arc::new(Mutex::new(vec![
            DataPoint {
                x: 50.0,
                y: 50.0,
                value: 10.0,
            },
            DataPoint {
                x: 150.0,
                y: 150.0,
                value: 20.0,
            },
            DataPoint {
                x: 250.0,
                y: 250.0,
                value: 30.0,
            },
        ]));
        let brush = BrushSelection::new((100.0, 100.0), (200.0, 200.0));
        cross_filter.add_filter("chart1", brush);
        let filtered_data = cross_filter.get_filtered_data(&shared_data);
        assert_eq!(filtered_data.len(), 1);
        assert_eq!(filtered_data[0].value, 20.0);
    }

    #[test]
    fn test_interactive_chart_workflow() {
        let mut chart = InteractiveChart::new(800.0, 600.0);
        let data = vec![
            DataPoint {
                x: 100.0,
                y: 100.0,
                value: 10.0,
            },
            DataPoint {
                x: 200.0,
                y: 200.0,
                value: 20.0,
            },
            DataPoint {
                x: 300.0,
                y: 300.0,
                value: 30.0,
            },
        ];
        chart.set_data(data);
        chart.zoom(2.0, 200.0, 200.0);
        chart.pan(50.0, 50.0);
        chart.show_tooltip(200.0, 200.0);
        chart.start_brush_selection(150.0, 150.0);
        chart.update_brush_selection(250.0, 250.0);
        chart.finish_brush_selection();
        assert_eq!(chart.viewport.scale, 2.0);
        assert_eq!(chart.viewport.x, -150.0);
        assert_eq!(chart.viewport.y, -150.0);
        assert!(chart.tooltip.visible);
        assert!(!chart.brush.is_empty());
    }

    #[test]
    fn test_multi_chart_cross_filtering() {
        let mut chart1 = InteractiveChart::new(400.0, 300.0);
        let mut chart2 = InteractiveChart::new(400.0, 300.0);
        let shared_data = Arc::new(Mutex::new(vec![
            DataPoint {
                x: 100.0,
                y: 100.0,
                value: 10.0,
            },
            DataPoint {
                x: 200.0,
                y: 200.0,
                value: 20.0,
            },
            DataPoint {
                x: 300.0,
                y: 300.0,
                value: 30.0,
            },
        ]));
        chart1.set_shared_data(shared_data.clone());
        chart2.set_shared_data(shared_data.clone());

        // Set up cross-filter
        let cross_filter = CrossFilter::new(vec!["chart1".to_string(), "chart2".to_string()]);
        chart1.cross_filter = Some(cross_filter);

        let brush = BrushSelection::new((150.0, 150.0), (250.0, 250.0));
        let filtered = chart1.apply_brush_filter(brush);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].value, 20.0);
    }

    #[test]
    fn test_performance_with_large_dataset() {
        let mut data = Vec::new();
        for i in 0..10000 {
            data.push(DataPoint {
                x: (i as f64) * 0.1,
                y: (i as f64) * 0.1,
                value: (i as f64) * 0.01,
            });
        }
        let mut chart = InteractiveChart::new(800.0, 600.0);
        chart.set_data(data);
        let start = std::time::Instant::now();
        chart.zoom(2.0, 400.0, 300.0);
        chart.pan(100.0, 100.0);
        let brush = BrushSelection::new((200.0, 200.0), (400.0, 400.0));
        let filtered = chart.apply_brush_filter(brush);
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100);
        assert!(filtered.len() > 0);
    }
}
