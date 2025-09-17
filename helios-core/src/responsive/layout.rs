//! Responsive Layout for leptos-helios
//!
//! This module provides responsive layout management for charts and visualizations,
//! including grid systems, flexible layouts, and adaptive spacing.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use thiserror::Error;
use tokio::sync::RwLock;

use super::breakpoints::{Breakpoint, DeviceType, Orientation};

/// Layout types for responsive design
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum LayoutType {
    Grid,
    Flexbox,
    Absolute,
    Relative,
    Sticky,
    Fixed,
}

/// Layout grid configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutGrid {
    pub columns: usize,
    pub rows: usize,
    pub gap: f64,
    pub column_gap: Option<f64>,
    pub row_gap: Option<f64>,
    pub auto_fit: bool,
    pub auto_fill: bool,
}

impl Default for LayoutGrid {
    fn default() -> Self {
        Self {
            columns: 12,
            rows: 1,
            gap: 16.0,
            column_gap: None,
            row_gap: None,
            auto_fit: false,
            auto_fill: false,
        }
    }
}

/// Layout item configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutItem {
    pub id: String,
    pub layout_type: LayoutType,
    pub grid_column_start: Option<usize>,
    pub grid_column_end: Option<usize>,
    pub grid_row_start: Option<usize>,
    pub grid_row_end: Option<usize>,
    pub flex_grow: Option<f64>,
    pub flex_shrink: Option<f64>,
    pub flex_basis: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub min_width: Option<f64>,
    pub min_height: Option<f64>,
    pub max_width: Option<f64>,
    pub max_height: Option<f64>,
    pub margin: Option<SpacingConfig>,
    pub padding: Option<SpacingConfig>,
    pub responsive_properties: HashMap<String, serde_json::Value>,
}

/// Spacing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingConfig {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl Default for SpacingConfig {
    fn default() -> Self {
        Self {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        }
    }
}

/// Responsive property for different breakpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsiveProperty {
    pub mobile: Option<serde_json::Value>,
    pub tablet: Option<serde_json::Value>,
    pub desktop: Option<serde_json::Value>,
    pub large_desktop: Option<serde_json::Value>,
    pub ultra_wide: Option<serde_json::Value>,
}

/// Responsive layout container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsiveLayout {
    pub id: String,
    pub layout_type: LayoutType,
    pub grid: Option<LayoutGrid>,
    pub items: Vec<LayoutItem>,
    pub container_width: f64,
    pub container_height: f64,
    pub responsive_spacing: bool,
    pub mobile_first: bool,
    pub breakpoint_overrides: HashMap<String, LayoutOverride>,
}

/// Layout override for specific breakpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutOverride {
    pub layout_type: Option<LayoutType>,
    pub grid: Option<LayoutGrid>,
    pub container_width: Option<f64>,
    pub container_height: Option<f64>,
    pub item_overrides: HashMap<String, LayoutItem>,
}

/// Layout manager for responsive design
pub struct LayoutManager {
    config: LayoutConfig,
    layouts: Arc<RwLock<HashMap<String, ResponsiveLayout>>>,
    stats: Arc<RwLock<LayoutStats>>,
}

/// Layout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConfig {
    pub grid_columns: usize,
    pub grid_gap: f64,
    pub container_padding: f64,
    pub responsive_spacing: bool,
    pub mobile_first: bool,
    pub default_layout_type: LayoutType,
    pub auto_resize: bool,
    pub breakpoint_layouts: HashMap<String, LayoutOverride>,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            grid_columns: 12,
            grid_gap: 16.0,
            container_padding: 20.0,
            responsive_spacing: true,
            mobile_first: true,
            default_layout_type: LayoutType::Grid,
            auto_resize: true,
            breakpoint_layouts: HashMap::new(),
        }
    }
}

/// Layout statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutStats {
    pub total_layouts: usize,
    pub responsive_layouts: usize,
    pub mobile_layouts: usize,
    pub tablet_layouts: usize,
    pub desktop_layouts: usize,
    pub grid_layouts: usize,
    pub flexbox_layouts: usize,
    pub absolute_layouts: usize,
    pub layout_calculations: usize,
    pub responsive_adjustments: usize,
}

impl Default for LayoutStats {
    fn default() -> Self {
        Self {
            total_layouts: 0,
            responsive_layouts: 0,
            mobile_layouts: 0,
            tablet_layouts: 0,
            desktop_layouts: 0,
            grid_layouts: 0,
            flexbox_layouts: 0,
            absolute_layouts: 0,
            layout_calculations: 0,
            responsive_adjustments: 0,
        }
    }
}

impl LayoutManager {
    /// Create a new layout manager
    pub fn new(config: LayoutConfig) -> Self {
        Self {
            config,
            layouts: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(LayoutStats::default())),
        }
    }

    /// Create a new responsive layout
    pub async fn create_layout(
        &self,
        id: String,
        layout_type: LayoutType,
        width: f64,
        height: f64,
    ) -> Result<ResponsiveLayout, ResponsiveLayoutError> {
        let grid = if layout_type == LayoutType::Grid {
            Some(LayoutGrid {
                columns: self.config.grid_columns,
                rows: 1,
                gap: self.config.grid_gap,
                column_gap: None,
                row_gap: None,
                auto_fit: false,
                auto_fill: false,
            })
        } else {
            None
        };

        let layout = ResponsiveLayout {
            id: id.clone(),
            layout_type,
            grid,
            items: Vec::new(),
            container_width: width,
            container_height: height,
            responsive_spacing: self.config.responsive_spacing,
            mobile_first: self.config.mobile_first,
            breakpoint_overrides: HashMap::new(),
        };

        // Store layout
        {
            let mut layouts = self.layouts.write().await;
            layouts.insert(id, layout.clone());
        }

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.total_layouts += 1;
            stats.responsive_layouts += 1;

            match layout_type {
                LayoutType::Grid => stats.grid_layouts += 1,
                LayoutType::Flexbox => stats.flexbox_layouts += 1,
                LayoutType::Absolute => stats.absolute_layouts += 1,
                _ => {}
            }
        }

        Ok(layout)
    }

    /// Add item to layout
    pub async fn add_layout_item(
        &self,
        layout_id: &str,
        item: LayoutItem,
    ) -> Result<(), ResponsiveLayoutError> {
        let mut layouts = self.layouts.write().await;

        if let Some(layout) = layouts.get_mut(layout_id) {
            layout.items.push(item);

            // Update stats
            let mut stats = self.stats.write().await;
            stats.layout_calculations += 1;
        } else {
            return Err(ResponsiveLayoutError::LayoutNotFound {
                layout_id: layout_id.to_string(),
            });
        }

        Ok(())
    }

    /// Apply breakpoint-specific layout adjustments
    pub async fn apply_breakpoint_layout(
        &self,
        layout: &mut ResponsiveLayout,
        breakpoint: &Breakpoint,
    ) -> Result<(), ResponsiveLayoutError> {
        // Apply breakpoint-specific overrides
        if let Some(override_config) = layout.breakpoint_overrides.get(breakpoint.name.as_str()) {
            if let Some(override_layout_type) = &override_config.layout_type {
                layout.layout_type = *override_layout_type;
            }

            if let Some(override_grid) = &override_config.grid {
                layout.grid = Some(override_grid.clone());
            }

            if let Some(override_width) = override_config.container_width {
                layout.container_width = override_width;
            }

            if let Some(override_height) = override_config.container_height {
                layout.container_height = override_height;
            }

            // Apply item overrides
            for (item_id, override_item) in &override_config.item_overrides {
                if let Some(item) = layout.items.iter_mut().find(|i| &i.id == item_id) {
                    *item = override_item.clone();
                }
            }
        }

        // Apply responsive spacing
        if layout.responsive_spacing {
            self.apply_responsive_spacing(layout, breakpoint).await?;
        }

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.responsive_adjustments += 1;

            match breakpoint.device_type {
                DeviceType::Mobile => stats.mobile_layouts += 1,
                DeviceType::Tablet => stats.tablet_layouts += 1,
                DeviceType::Desktop => stats.desktop_layouts += 1,
                _ => {}
            }
        }

        Ok(())
    }

    /// Apply responsive spacing based on breakpoint
    async fn apply_responsive_spacing(
        &self,
        layout: &mut ResponsiveLayout,
        breakpoint: &Breakpoint,
    ) -> Result<(), ResponsiveLayoutError> {
        let spacing_multiplier = match breakpoint.device_type {
            DeviceType::Mobile => 0.5,
            DeviceType::Tablet => 0.75,
            DeviceType::Desktop => 1.0,
            DeviceType::LargeDesktop => 1.25,
            DeviceType::UltraWide => 1.5,
        };

        // Adjust grid gap
        if let Some(grid) = &mut layout.grid {
            grid.gap *= spacing_multiplier;
            if let Some(column_gap) = &mut grid.column_gap {
                *column_gap *= spacing_multiplier;
            }
            if let Some(row_gap) = &mut grid.row_gap {
                *row_gap *= spacing_multiplier;
            }
        }

        // Adjust item spacing
        for item in &mut layout.items {
            if let Some(margin) = &mut item.margin {
                margin.top *= spacing_multiplier;
                margin.right *= spacing_multiplier;
                margin.bottom *= spacing_multiplier;
                margin.left *= spacing_multiplier;
            }

            if let Some(padding) = &mut item.padding {
                padding.top *= spacing_multiplier;
                padding.right *= spacing_multiplier;
                padding.bottom *= spacing_multiplier;
                padding.left *= spacing_multiplier;
            }
        }

        Ok(())
    }

    /// Get layout by ID
    pub async fn get_layout(&self, layout_id: &str) -> Option<ResponsiveLayout> {
        let layouts = self.layouts.read().await;
        layouts.get(layout_id).cloned()
    }

    /// Update layout configuration
    pub async fn update_config(
        &mut self,
        config: LayoutConfig,
    ) -> Result<(), ResponsiveLayoutError> {
        self.config = config;
        Ok(())
    }

    /// Get layout statistics
    pub async fn get_stats(&self) -> LayoutStats {
        self.stats.read().await.clone()
    }

    /// Calculate layout dimensions
    pub async fn calculate_layout_dimensions(
        &self,
        layout: &ResponsiveLayout,
    ) -> Result<HashMap<String, (f64, f64)>, ResponsiveLayoutError> {
        let mut dimensions = HashMap::new();

        match layout.layout_type {
            LayoutType::Grid => {
                if let Some(grid) = &layout.grid {
                    let column_width = (layout.container_width
                        - (grid.gap * (grid.columns - 1) as f64))
                        / grid.columns as f64;

                    for item in &layout.items {
                        let item_width = if let (Some(start), Some(end)) =
                            (item.grid_column_start, item.grid_column_end)
                        {
                            let span = end - start;
                            (column_width * span as f64) + (grid.gap * (span - 1) as f64)
                        } else {
                            column_width
                        };

                        let item_height = item.height.unwrap_or(100.0);
                        dimensions.insert(item.id.clone(), (item_width, item_height));
                    }
                }
            }
            LayoutType::Flexbox => {
                let total_flex_grow: f64 = layout
                    .items
                    .iter()
                    .map(|item| item.flex_grow.unwrap_or(1.0))
                    .sum();

                for item in &layout.items {
                    let flex_ratio = item.flex_grow.unwrap_or(1.0) / total_flex_grow;
                    let item_width = layout.container_width * flex_ratio;
                    let item_height = item.height.unwrap_or(100.0);
                    dimensions.insert(item.id.clone(), (item_width, item_height));
                }
            }
            _ => {
                // For absolute, relative, sticky, fixed layouts
                for item in &layout.items {
                    let item_width = item.width.unwrap_or(100.0);
                    let item_height = item.height.unwrap_or(100.0);
                    dimensions.insert(item.id.clone(), (item_width, item_height));
                }
            }
        }

        Ok(dimensions)
    }

    /// Remove layout
    pub async fn remove_layout(&self, layout_id: &str) -> Result<(), ResponsiveLayoutError> {
        let mut layouts = self.layouts.write().await;
        layouts.remove(layout_id);
        Ok(())
    }

    /// Get all layouts
    pub async fn get_all_layouts(&self) -> HashMap<String, ResponsiveLayout> {
        self.layouts.read().await.clone()
    }
}

/// Responsive layout error types
#[derive(Debug, Error)]
pub enum ResponsiveLayoutError {
    #[error("Layout not found: {layout_id}")]
    LayoutNotFound { layout_id: String },

    #[error("Invalid layout configuration: {message}")]
    InvalidConfiguration { message: String },

    #[error("Layout calculation error: {message}")]
    CalculationError { message: String },

    #[error("Item not found: {item_id}")]
    ItemNotFound { item_id: String },

    #[error("Grid configuration error: {message}")]
    GridError { message: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_layout_config() -> LayoutConfig {
        LayoutConfig {
            grid_columns: 12,
            grid_gap: 16.0,
            container_padding: 20.0,
            responsive_spacing: true,
            mobile_first: true,
            default_layout_type: LayoutType::Grid,
            auto_resize: true,
            breakpoint_layouts: HashMap::new(),
        }
    }

    fn create_test_layout_item(id: &str) -> LayoutItem {
        LayoutItem {
            id: id.to_string(),
            layout_type: LayoutType::Grid,
            grid_column_start: Some(1),
            grid_column_end: Some(4),
            grid_row_start: None,
            grid_row_end: None,
            flex_grow: None,
            flex_shrink: None,
            flex_basis: None,
            width: None,
            height: Some(100.0),
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            margin: Some(SpacingConfig::default()),
            padding: Some(SpacingConfig::default()),
            responsive_properties: HashMap::new(),
        }
    }

    #[tokio::test]
    async fn test_layout_manager_creation() {
        let config = create_test_layout_config();
        let manager = LayoutManager::new(config);

        let stats = manager.get_stats().await;
        assert_eq!(stats.total_layouts, 0);
    }

    #[tokio::test]
    async fn test_create_grid_layout() {
        let config = create_test_layout_config();
        let manager = LayoutManager::new(config);

        let layout = manager
            .create_layout("test-layout".to_string(), LayoutType::Grid, 1200.0, 800.0)
            .await
            .unwrap();

        assert_eq!(layout.id, "test-layout");
        assert_eq!(layout.layout_type, LayoutType::Grid);
        assert_eq!(layout.container_width, 1200.0);
        assert_eq!(layout.container_height, 800.0);
        assert!(layout.grid.is_some());

        let stats = manager.get_stats().await;
        assert_eq!(stats.total_layouts, 1);
        assert_eq!(stats.grid_layouts, 1);
    }

    #[tokio::test]
    async fn test_create_flexbox_layout() {
        let config = create_test_layout_config();
        let manager = LayoutManager::new(config);

        let layout = manager
            .create_layout(
                "flex-layout".to_string(),
                LayoutType::Flexbox,
                1200.0,
                800.0,
            )
            .await
            .unwrap();

        assert_eq!(layout.layout_type, LayoutType::Flexbox);
        assert!(layout.grid.is_none());

        let stats = manager.get_stats().await;
        assert_eq!(stats.flexbox_layouts, 1);
    }

    #[tokio::test]
    async fn test_add_layout_item() {
        let config = create_test_layout_config();
        let manager = LayoutManager::new(config);

        let layout = manager
            .create_layout("test-layout".to_string(), LayoutType::Grid, 1200.0, 800.0)
            .await
            .unwrap();
        let item = create_test_layout_item("item-1");

        manager.add_layout_item("test-layout", item).await.unwrap();

        let updated_layout = manager.get_layout("test-layout").await.unwrap();
        assert_eq!(updated_layout.items.len(), 1);
        assert_eq!(updated_layout.items[0].id, "item-1");

        let stats = manager.get_stats().await;
        assert_eq!(stats.layout_calculations, 1);
    }

    #[tokio::test]
    async fn test_apply_breakpoint_layout() {
        let config = create_test_layout_config();
        let manager = LayoutManager::new(config);

        let mut layout = manager
            .create_layout("test-layout".to_string(), LayoutType::Grid, 1200.0, 800.0)
            .await
            .unwrap();
        let item = create_test_layout_item("item-1");
        layout.items.push(item);

        let breakpoint = Breakpoint {
            name: "mobile".to_string(),
            min_width: 0.0,
            max_width: Some(768.0),
            device_type: DeviceType::Mobile,
            orientation: Orientation::Portrait,
        };

        manager
            .apply_breakpoint_layout(&mut layout, &breakpoint)
            .await
            .unwrap();

        let stats = manager.get_stats().await;
        assert_eq!(stats.responsive_adjustments, 1);
        assert_eq!(stats.mobile_layouts, 1);
    }

    #[tokio::test]
    async fn test_responsive_spacing() {
        let config = create_test_layout_config();
        let manager = LayoutManager::new(config);

        let mut layout = manager
            .create_layout("test-layout".to_string(), LayoutType::Grid, 1200.0, 800.0)
            .await
            .unwrap();
        let item = create_test_layout_item("item-1");
        layout.items.push(item);

        let mobile_breakpoint = Breakpoint {
            name: "mobile".to_string(),
            min_width: 0.0,
            max_width: Some(768.0),
            device_type: DeviceType::Mobile,
            orientation: Orientation::Portrait,
        };

        let original_gap = layout.grid.as_ref().unwrap().gap;
        manager
            .apply_breakpoint_layout(&mut layout, &mobile_breakpoint)
            .await
            .unwrap();

        // Mobile spacing should be reduced
        assert!(layout.grid.as_ref().unwrap().gap < original_gap);
    }

    #[tokio::test]
    async fn test_calculate_grid_dimensions() {
        let config = create_test_layout_config();
        let manager = LayoutManager::new(config);

        let mut layout = manager
            .create_layout("test-layout".to_string(), LayoutType::Grid, 1200.0, 800.0)
            .await
            .unwrap();
        let item = create_test_layout_item("item-1");
        layout.items.push(item);

        let dimensions = manager.calculate_layout_dimensions(&layout).await.unwrap();

        assert!(dimensions.contains_key("item-1"));
        let (width, height) = dimensions["item-1"];
        assert!(width > 0.0);
        assert_eq!(height, 100.0);
    }

    #[tokio::test]
    async fn test_calculate_flexbox_dimensions() {
        let config = create_test_layout_config();
        let manager = LayoutManager::new(config);

        let mut layout = manager
            .create_layout(
                "flex-layout".to_string(),
                LayoutType::Flexbox,
                1200.0,
                800.0,
            )
            .await
            .unwrap();

        let mut item1 = create_test_layout_item("item-1");
        item1.flex_grow = Some(2.0);
        layout.items.push(item1);

        let mut item2 = create_test_layout_item("item-2");
        item2.flex_grow = Some(1.0);
        layout.items.push(item2);

        let dimensions = manager.calculate_layout_dimensions(&layout).await.unwrap();

        let (width1, _) = dimensions["item-1"];
        let (width2, _) = dimensions["item-2"];

        // Item 1 should be twice as wide as item 2
        assert!((width1 - width2 * 2.0).abs() < 1.0);
    }

    #[tokio::test]
    async fn test_layout_not_found() {
        let config = create_test_layout_config();
        let manager = LayoutManager::new(config);

        let item = create_test_layout_item("item-1");
        let result = manager.add_layout_item("nonexistent", item).await;

        assert!(matches!(
            result,
            Err(ResponsiveLayoutError::LayoutNotFound { .. })
        ));
    }

    #[tokio::test]
    async fn test_remove_layout() {
        let config = create_test_layout_config();
        let manager = LayoutManager::new(config);

        manager
            .create_layout("test-layout".to_string(), LayoutType::Grid, 1200.0, 800.0)
            .await
            .unwrap();

        let layout = manager.get_layout("test-layout").await;
        assert!(layout.is_some());

        manager.remove_layout("test-layout").await.unwrap();

        let layout = manager.get_layout("test-layout").await;
        assert!(layout.is_none());
    }

    #[tokio::test]
    async fn test_config_update() {
        let mut config = create_test_layout_config();
        config.grid_columns = 16;

        let mut manager = LayoutManager::new(config);

        let new_config = create_test_layout_config();
        manager.update_config(new_config).await.unwrap();

        // Verify config was updated
        let layout = manager
            .create_layout("test-layout".to_string(), LayoutType::Grid, 1200.0, 800.0)
            .await
            .unwrap();
        assert_eq!(layout.grid.as_ref().unwrap().columns, 12);
    }
}
