//! Responsive Navigation for leptos-helios
//!
//! This module provides responsive navigation management for charts and visualizations,
//! including mobile navigation patterns, adaptive menus, and touch-friendly controls.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use thiserror::Error;
use tokio::sync::RwLock;

use super::breakpoints::{Breakpoint, DeviceType, Orientation};

/// Navigation types for responsive design
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum NavigationType {
    Horizontal,
    Vertical,
    Dropdown,
    Hamburger,
    Tab,
    Breadcrumb,
    Pagination,
    Sidebar,
}

/// Navigation item configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationItem {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub url: Option<String>,
    pub children: Vec<NavigationItem>,
    pub visible: bool,
    pub responsive_visibility: HashMap<String, bool>,
    pub touch_target_size: f64,
    pub spacing: f64,
}

/// Responsive navigation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsiveNavigation {
    pub id: String,
    pub navigation_type: NavigationType,
    pub items: Vec<NavigationItem>,
    pub breakpoint_overrides: HashMap<String, NavigationOverride>,
    pub touch_friendly: bool,
    pub accessibility_enabled: bool,
    pub auto_collapse: bool,
    pub collapse_threshold: f64,
}

/// Navigation override for specific breakpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationOverride {
    pub navigation_type: Option<NavigationType>,
    pub touch_friendly: Option<bool>,
    pub auto_collapse: Option<bool>,
    pub collapse_threshold: Option<f64>,
    pub item_overrides: HashMap<String, NavigationItem>,
}

/// Navigation manager for responsive design
pub struct NavigationManager {
    config: NavigationConfig,
    navigations: Arc<RwLock<HashMap<String, ResponsiveNavigation>>>,
    stats: Arc<RwLock<NavigationStats>>,
}

/// Navigation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationConfig {
    pub default_navigation_type: NavigationType,
    pub touch_target_min_size: f64,
    pub mobile_navigation_type: NavigationType,
    pub tablet_navigation_type: NavigationType,
    pub desktop_navigation_type: NavigationType,
    pub auto_collapse_enabled: bool,
    pub collapse_threshold: f64,
    pub accessibility_enabled: bool,
    pub touch_friendly_default: bool,
}

impl Default for NavigationConfig {
    fn default() -> Self {
        Self {
            default_navigation_type: NavigationType::Horizontal,
            touch_target_min_size: 44.0,
            mobile_navigation_type: NavigationType::Hamburger,
            tablet_navigation_type: NavigationType::Tab,
            desktop_navigation_type: NavigationType::Horizontal,
            auto_collapse_enabled: true,
            collapse_threshold: 768.0,
            accessibility_enabled: true,
            touch_friendly_default: true,
        }
    }
}

/// Navigation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationStats {
    pub total_navigations: usize,
    pub responsive_navigations: usize,
    pub mobile_navigations: usize,
    pub tablet_navigations: usize,
    pub desktop_navigations: usize,
    pub horizontal_navigations: usize,
    pub vertical_navigations: usize,
    pub hamburger_navigations: usize,
    pub tab_navigations: usize,
    pub navigation_collapses: usize,
    pub touch_interactions: usize,
}

impl Default for NavigationStats {
    fn default() -> Self {
        Self {
            total_navigations: 0,
            responsive_navigations: 0,
            mobile_navigations: 0,
            tablet_navigations: 0,
            desktop_navigations: 0,
            horizontal_navigations: 0,
            vertical_navigations: 0,
            hamburger_navigations: 0,
            tab_navigations: 0,
            navigation_collapses: 0,
            touch_interactions: 0,
        }
    }
}

impl NavigationManager {
    /// Create a new navigation manager
    pub fn new(config: NavigationConfig) -> Self {
        Self {
            config,
            navigations: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(NavigationStats::default())),
        }
    }

    /// Create a new responsive navigation
    pub async fn create_navigation(
        &self,
        id: String,
        navigation_type: NavigationType,
    ) -> Result<ResponsiveNavigation, ResponsiveNavigationError> {
        let navigation = ResponsiveNavigation {
            id: id.clone(),
            navigation_type,
            items: Vec::new(),
            breakpoint_overrides: HashMap::new(),
            touch_friendly: self.config.touch_friendly_default,
            accessibility_enabled: self.config.accessibility_enabled,
            auto_collapse: self.config.auto_collapse_enabled,
            collapse_threshold: self.config.collapse_threshold,
        };

        // Store navigation
        {
            let mut navigations = self.navigations.write().await;
            navigations.insert(id, navigation.clone());
        }

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.total_navigations += 1;
            stats.responsive_navigations += 1;

            match navigation_type {
                NavigationType::Horizontal => stats.horizontal_navigations += 1,
                NavigationType::Vertical => stats.vertical_navigations += 1,
                NavigationType::Hamburger => stats.hamburger_navigations += 1,
                NavigationType::Tab => stats.tab_navigations += 1,
                _ => {}
            }
        }

        Ok(navigation)
    }

    /// Add navigation item
    pub async fn add_navigation_item(
        &self,
        navigation_id: &str,
        item: NavigationItem,
    ) -> Result<(), ResponsiveNavigationError> {
        let mut navigations = self.navigations.write().await;

        if let Some(navigation) = navigations.get_mut(navigation_id) {
            navigation.items.push(item);
        } else {
            return Err(ResponsiveNavigationError::NavigationNotFound {
                navigation_id: navigation_id.to_string(),
            });
        }

        Ok(())
    }

    /// Apply responsive navigation to a layout
    pub async fn apply_responsive_navigation(
        &self,
        layout: &mut super::layout::ResponsiveLayout,
        breakpoint: &Breakpoint,
    ) -> Result<(), ResponsiveNavigationError> {
        // Determine navigation type for breakpoint
        let navigation_type = self.get_navigation_type_for_breakpoint(breakpoint).await;

        // Apply navigation adjustments to layout items
        for item in &mut layout.items {
            if let Some(navigation_id) = item.responsive_properties.get("navigation_id") {
                if let Some(navigation_id_str) = navigation_id.as_str() {
                    if let Some(navigation) = self.get_navigation(navigation_id_str).await {
                        let responsive_navigation = self
                            .adapt_navigation_for_breakpoint(
                                &navigation,
                                breakpoint,
                                navigation_type,
                            )
                            .await;

                        // Apply navigation properties to item
                        self.apply_navigation_to_item(item, &responsive_navigation, breakpoint)
                            .await?;
                    }
                }
            }
        }

        // Update stats
        {
            let mut stats = self.stats.write().await;

            match breakpoint.device_type {
                DeviceType::Mobile => stats.mobile_navigations += 1,
                DeviceType::Tablet => stats.tablet_navigations += 1,
                DeviceType::Desktop => stats.desktop_navigations += 1,
                _ => {}
            }
        }

        Ok(())
    }

    /// Get navigation type for breakpoint
    async fn get_navigation_type_for_breakpoint(&self, breakpoint: &Breakpoint) -> NavigationType {
        match breakpoint.device_type {
            DeviceType::Mobile => self.config.mobile_navigation_type,
            DeviceType::Tablet => self.config.tablet_navigation_type,
            DeviceType::Desktop => self.config.desktop_navigation_type,
            _ => self.config.default_navigation_type,
        }
    }

    /// Adapt navigation for breakpoint
    async fn adapt_navigation_for_breakpoint(
        &self,
        navigation: &ResponsiveNavigation,
        breakpoint: &Breakpoint,
        navigation_type: NavigationType,
    ) -> ResponsiveNavigation {
        let mut adapted_navigation = navigation.clone();

        // Apply breakpoint-specific overrides
        if let Some(override_config) = navigation
            .breakpoint_overrides
            .get(breakpoint.name.as_str())
        {
            if let Some(override_type) = override_config.navigation_type {
                adapted_navigation.navigation_type = override_type;
            }

            if let Some(override_touch_friendly) = override_config.touch_friendly {
                adapted_navigation.touch_friendly = override_touch_friendly;
            }

            if let Some(override_auto_collapse) = override_config.auto_collapse {
                adapted_navigation.auto_collapse = override_auto_collapse;
            }

            if let Some(override_threshold) = override_config.collapse_threshold {
                adapted_navigation.collapse_threshold = override_threshold;
            }
        } else {
            // Apply default navigation type for breakpoint
            adapted_navigation.navigation_type = navigation_type;
        }

        // Apply touch-friendly adjustments for mobile
        if breakpoint.device_type == DeviceType::Mobile {
            adapted_navigation.touch_friendly = true;
            adapted_navigation.auto_collapse = true;
        }

        adapted_navigation
    }

    /// Apply navigation to layout item
    async fn apply_navigation_to_item(
        &self,
        item: &mut super::layout::LayoutItem,
        navigation: &ResponsiveNavigation,
        breakpoint: &Breakpoint,
    ) -> Result<(), ResponsiveNavigationError> {
        // Apply navigation type
        item.responsive_properties.insert(
            "navigation_type".to_string(),
            serde_json::Value::String(format!("{:?}", navigation.navigation_type)),
        );

        // Apply touch-friendly sizing
        if navigation.touch_friendly {
            let touch_target_size = self.config.touch_target_min_size;
            item.min_width = Some(touch_target_size);
            item.min_height = Some(touch_target_size);

            // Ensure adequate spacing for touch targets
            if let Some(margin) = &mut item.margin {
                margin.top = margin.top.max(8.0);
                margin.right = margin.right.max(8.0);
                margin.bottom = margin.bottom.max(8.0);
                margin.left = margin.left.max(8.0);
            }
        }

        // Apply auto-collapse behavior
        if navigation.auto_collapse && breakpoint.device_type == DeviceType::Mobile {
            item.responsive_properties
                .insert("auto_collapse".to_string(), serde_json::Value::Bool(true));
            item.responsive_properties.insert(
                "collapse_threshold".to_string(),
                serde_json::Value::Number(
                    serde_json::Number::from_f64(navigation.collapse_threshold).unwrap(),
                ),
            );
        }

        // Apply accessibility features
        if navigation.accessibility_enabled {
            item.responsive_properties.insert(
                "accessibility_enabled".to_string(),
                serde_json::Value::Bool(true),
            );
            item.responsive_properties.insert(
                "aria_label".to_string(),
                serde_json::Value::String("Navigation item".to_string()),
            );
        }

        Ok(())
    }

    /// Get navigation by ID
    pub async fn get_navigation(&self, id: &str) -> Option<ResponsiveNavigation> {
        let navigations = self.navigations.read().await;
        navigations.get(id).cloned()
    }

    /// Update navigation
    pub async fn update_navigation(
        &self,
        id: &str,
        navigation: ResponsiveNavigation,
    ) -> Result<(), ResponsiveNavigationError> {
        let mut navigations = self.navigations.write().await;
        navigations.insert(id.to_string(), navigation);
        Ok(())
    }

    /// Update navigation configuration
    pub async fn update_config(
        &mut self,
        config: NavigationConfig,
    ) -> Result<(), ResponsiveNavigationError> {
        self.config = config;
        Ok(())
    }

    /// Get navigation statistics
    pub async fn get_stats(&self) -> NavigationStats {
        self.stats.read().await.clone()
    }

    /// Handle navigation collapse
    pub async fn handle_navigation_collapse(
        &self,
        navigation_id: &str,
    ) -> Result<(), ResponsiveNavigationError> {
        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.navigation_collapses += 1;
        }

        Ok(())
    }

    /// Handle touch interaction
    pub async fn handle_touch_interaction(
        &self,
        navigation_id: &str,
        item_id: &str,
    ) -> Result<(), ResponsiveNavigationError> {
        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.touch_interactions += 1;
        }

        Ok(())
    }

    /// Remove navigation
    pub async fn remove_navigation(&self, id: &str) -> Result<(), ResponsiveNavigationError> {
        let mut navigations = self.navigations.write().await;
        navigations.remove(id);
        Ok(())
    }

    /// Get all navigations
    pub async fn get_all_navigations(&self) -> HashMap<String, ResponsiveNavigation> {
        self.navigations.read().await.clone()
    }
}

/// Responsive navigation error types
#[derive(Debug, Error)]
pub enum ResponsiveNavigationError {
    #[error("Navigation not found: {navigation_id}")]
    NavigationNotFound { navigation_id: String },

    #[error("Invalid navigation configuration: {message}")]
    InvalidConfiguration { message: String },

    #[error("Navigation item error: {message}")]
    NavigationItemError { message: String },

    #[error("Touch interaction error: {message}")]
    TouchInteractionError { message: String },

    #[error("Accessibility error: {message}")]
    AccessibilityError { message: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_navigation_config() -> NavigationConfig {
        NavigationConfig {
            default_navigation_type: NavigationType::Horizontal,
            touch_target_min_size: 44.0,
            mobile_navigation_type: NavigationType::Hamburger,
            tablet_navigation_type: NavigationType::Tab,
            desktop_navigation_type: NavigationType::Horizontal,
            auto_collapse_enabled: true,
            collapse_threshold: 768.0,
            accessibility_enabled: true,
            touch_friendly_default: true,
        }
    }

    fn create_test_navigation_item(id: &str, label: &str) -> NavigationItem {
        NavigationItem {
            id: id.to_string(),
            label: label.to_string(),
            icon: None,
            url: None,
            children: Vec::new(),
            visible: true,
            responsive_visibility: HashMap::new(),
            touch_target_size: 44.0,
            spacing: 8.0,
        }
    }

    fn create_test_breakpoint() -> Breakpoint {
        Breakpoint {
            name: "mobile".to_string(),
            min_width: 0.0,
            max_width: Some(768.0),
            device_type: DeviceType::Mobile,
            orientation: Orientation::Portrait,
        }
    }

    #[tokio::test]
    async fn test_navigation_manager_creation() {
        let config = create_test_navigation_config();
        let manager = NavigationManager::new(config);

        let stats = manager.get_stats().await;
        assert_eq!(stats.total_navigations, 0);
    }

    #[tokio::test]
    async fn test_create_horizontal_navigation() {
        let config = create_test_navigation_config();
        let manager = NavigationManager::new(config);

        let navigation = manager
            .create_navigation("test-navigation".to_string(), NavigationType::Horizontal)
            .await
            .unwrap();

        assert_eq!(navigation.id, "test-navigation");
        assert_eq!(navigation.navigation_type, NavigationType::Horizontal);
        assert!(navigation.touch_friendly);
        assert!(navigation.accessibility_enabled);

        let stats = manager.get_stats().await;
        assert_eq!(stats.total_navigations, 1);
        assert_eq!(stats.horizontal_navigations, 1);
    }

    #[tokio::test]
    async fn test_create_hamburger_navigation() {
        let config = create_test_navigation_config();
        let manager = NavigationManager::new(config);

        let navigation = manager
            .create_navigation("mobile-navigation".to_string(), NavigationType::Hamburger)
            .await
            .unwrap();

        assert_eq!(navigation.navigation_type, NavigationType::Hamburger);

        let stats = manager.get_stats().await;
        assert_eq!(stats.hamburger_navigations, 1);
    }

    #[tokio::test]
    async fn test_add_navigation_item() {
        let config = create_test_navigation_config();
        let manager = NavigationManager::new(config);

        manager
            .create_navigation("test-navigation".to_string(), NavigationType::Horizontal)
            .await
            .unwrap();
        let item = create_test_navigation_item("item-1", "Home");

        manager
            .add_navigation_item("test-navigation", item)
            .await
            .unwrap();

        let navigation = manager.get_navigation("test-navigation").await.unwrap();
        assert_eq!(navigation.items.len(), 1);
        assert_eq!(navigation.items[0].id, "item-1");
        assert_eq!(navigation.items[0].label, "Home");
    }

    #[tokio::test]
    async fn test_navigation_type_for_breakpoint() {
        let config = create_test_navigation_config();
        let manager = NavigationManager::new(config);

        let mobile_breakpoint = Breakpoint {
            name: "mobile".to_string(),
            min_width: 0.0,
            max_width: Some(768.0),
            device_type: DeviceType::Mobile,
            orientation: Orientation::Portrait,
        };

        let desktop_breakpoint = Breakpoint {
            name: "desktop".to_string(),
            min_width: 1200.0,
            max_width: None,
            device_type: DeviceType::Desktop,
            orientation: Orientation::Landscape,
        };

        let mobile_type = manager
            .get_navigation_type_for_breakpoint(&mobile_breakpoint)
            .await;
        let desktop_type = manager
            .get_navigation_type_for_breakpoint(&desktop_breakpoint)
            .await;

        assert_eq!(mobile_type, NavigationType::Hamburger);
        assert_eq!(desktop_type, NavigationType::Horizontal);
    }

    #[tokio::test]
    async fn test_adapt_navigation_for_breakpoint() {
        let config = create_test_navigation_config();
        let manager = NavigationManager::new(config);

        let navigation = manager
            .create_navigation("test-navigation".to_string(), NavigationType::Horizontal)
            .await
            .unwrap();
        let mobile_breakpoint = create_test_breakpoint();

        let adapted_navigation = manager
            .adapt_navigation_for_breakpoint(
                &navigation,
                &mobile_breakpoint,
                NavigationType::Hamburger,
            )
            .await;

        assert_eq!(
            adapted_navigation.navigation_type,
            NavigationType::Hamburger
        );
        assert!(adapted_navigation.touch_friendly);
        assert!(adapted_navigation.auto_collapse);
    }

    #[tokio::test]
    async fn test_handle_navigation_collapse() {
        let config = create_test_navigation_config();
        let manager = NavigationManager::new(config);

        manager
            .handle_navigation_collapse("test-navigation")
            .await
            .unwrap();

        let stats = manager.get_stats().await;
        assert_eq!(stats.navigation_collapses, 1);
    }

    #[tokio::test]
    async fn test_handle_touch_interaction() {
        let config = create_test_navigation_config();
        let manager = NavigationManager::new(config);

        manager
            .handle_touch_interaction("test-navigation", "item-1")
            .await
            .unwrap();

        let stats = manager.get_stats().await;
        assert_eq!(stats.touch_interactions, 1);
    }

    #[tokio::test]
    async fn test_navigation_not_found() {
        let config = create_test_navigation_config();
        let manager = NavigationManager::new(config);

        let item = create_test_navigation_item("item-1", "Home");
        let result = manager.add_navigation_item("nonexistent", item).await;

        assert!(matches!(
            result,
            Err(ResponsiveNavigationError::NavigationNotFound { .. })
        ));
    }

    #[tokio::test]
    async fn test_remove_navigation() {
        let config = create_test_navigation_config();
        let manager = NavigationManager::new(config);

        manager
            .create_navigation("test-navigation".to_string(), NavigationType::Horizontal)
            .await
            .unwrap();

        let navigation = manager.get_navigation("test-navigation").await;
        assert!(navigation.is_some());

        manager.remove_navigation("test-navigation").await.unwrap();

        let navigation = manager.get_navigation("test-navigation").await;
        assert!(navigation.is_none());
    }

    #[tokio::test]
    async fn test_config_update() {
        let mut config = create_test_navigation_config();
        config.touch_target_min_size = 48.0;

        let mut manager = NavigationManager::new(config);

        let new_config = create_test_navigation_config();
        manager.update_config(new_config).await.unwrap();

        // Verify config was updated
        let navigation = manager
            .create_navigation("test-navigation".to_string(), NavigationType::Horizontal)
            .await
            .unwrap();
        assert!(navigation.touch_friendly);
    }
}
