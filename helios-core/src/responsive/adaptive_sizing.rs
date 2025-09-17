//! Adaptive Sizing for leptos-helios
//!
//! This module provides adaptive sizing management for responsive design,
//! including size constraints, adaptive scaling, and device-specific sizing strategies.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use thiserror::Error;
use tokio::sync::RwLock;

use super::breakpoints::{Breakpoint, DeviceType, Orientation};

/// Size constraint types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SizeConstraintType {
    MinWidth,
    MaxWidth,
    MinHeight,
    MaxHeight,
    AspectRatio,
    FixedWidth,
    FixedHeight,
    Flexible,
}

/// Size constraint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeConstraint {
    pub constraint_type: SizeConstraintType,
    pub value: f64,
    pub breakpoint_specific: bool,
    pub breakpoint_values: HashMap<String, f64>,
    pub priority: u8,
}

/// Sizing strategy for adaptive design
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SizingStrategy {
    Fixed,
    Flexible,
    Responsive,
    Adaptive,
    Fluid,
    Elastic,
}

/// Adaptive sizing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveSizing {
    pub id: String,
    pub base_width: f64,
    pub base_height: f64,
    pub sizing_strategy: SizingStrategy,
    pub constraints: Vec<SizeConstraint>,
    pub breakpoint_overrides: HashMap<String, SizingOverride>,
    pub aspect_ratio: Option<f64>,
    pub min_size: (f64, f64),
    pub max_size: (f64, f64),
}

/// Sizing override for specific breakpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizingOverride {
    pub sizing_strategy: Option<SizingStrategy>,
    pub base_width: Option<f64>,
    pub base_height: Option<f64>,
    pub aspect_ratio: Option<f64>,
    pub min_size: Option<(f64, f64)>,
    pub max_size: Option<(f64, f64)>,
    pub constraint_overrides: HashMap<String, SizeConstraint>,
}

/// Adaptive sizing manager for responsive design
pub struct AdaptiveSizingManager {
    config: AdaptiveSizingConfig,
    sizing_systems: Arc<RwLock<HashMap<String, AdaptiveSizing>>>,
    stats: Arc<RwLock<AdaptiveSizingStats>>,
}

/// Adaptive sizing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveSizingConfig {
    pub default_sizing_strategy: SizingStrategy,
    pub mobile_sizing_strategy: SizingStrategy,
    pub tablet_sizing_strategy: SizingStrategy,
    pub desktop_sizing_strategy: SizingStrategy,
    pub fluid_sizing_enabled: bool,
    pub elastic_sizing_enabled: bool,
    pub aspect_ratio_preservation: bool,
    pub min_size_constraints: (f64, f64),
    pub max_size_constraints: (f64, f64),
    pub breakpoint_scaling_factors: HashMap<String, f64>,
}

impl Default for AdaptiveSizingConfig {
    fn default() -> Self {
        let mut breakpoint_factors = HashMap::new();
        breakpoint_factors.insert("mobile".to_string(), 0.8);
        breakpoint_factors.insert("tablet".to_string(), 1.0);
        breakpoint_factors.insert("desktop".to_string(), 1.2);
        breakpoint_factors.insert("large-desktop".to_string(), 1.4);
        breakpoint_factors.insert("ultra-wide".to_string(), 1.6);

        Self {
            default_sizing_strategy: SizingStrategy::Responsive,
            mobile_sizing_strategy: SizingStrategy::Fixed,
            tablet_sizing_strategy: SizingStrategy::Flexible,
            desktop_sizing_strategy: SizingStrategy::Adaptive,
            fluid_sizing_enabled: true,
            elastic_sizing_enabled: true,
            aspect_ratio_preservation: true,
            min_size_constraints: (100.0, 100.0),
            max_size_constraints: (2000.0, 2000.0),
            breakpoint_scaling_factors: breakpoint_factors,
        }
    }
}

/// Adaptive sizing statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveSizingStats {
    pub total_sizing_operations: usize,
    pub responsive_sizing: usize,
    pub mobile_sizing: usize,
    pub tablet_sizing: usize,
    pub desktop_sizing: usize,
    pub fixed_sizing: usize,
    pub flexible_sizing: usize,
    pub adaptive_sizing: usize,
    pub fluid_sizing: usize,
    pub elastic_sizing: usize,
    pub constraint_applications: usize,
    pub aspect_ratio_preservations: usize,
    pub scaling_operations: usize,
}

impl Default for AdaptiveSizingStats {
    fn default() -> Self {
        Self {
            total_sizing_operations: 0,
            responsive_sizing: 0,
            mobile_sizing: 0,
            tablet_sizing: 0,
            desktop_sizing: 0,
            fixed_sizing: 0,
            flexible_sizing: 0,
            adaptive_sizing: 0,
            fluid_sizing: 0,
            elastic_sizing: 0,
            constraint_applications: 0,
            aspect_ratio_preservations: 0,
            scaling_operations: 0,
        }
    }
}

impl AdaptiveSizingManager {
    /// Create a new adaptive sizing manager
    pub fn new(config: AdaptiveSizingConfig) -> Self {
        Self {
            config,
            sizing_systems: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(AdaptiveSizingStats::default())),
        }
    }

    /// Create a new adaptive sizing system
    pub async fn create_adaptive_sizing(
        &self,
        id: String,
        base_width: f64,
        base_height: f64,
        strategy: SizingStrategy,
    ) -> Result<AdaptiveSizing, AdaptiveSizingError> {
        let sizing = AdaptiveSizing {
            id: id.clone(),
            base_width,
            base_height,
            sizing_strategy: strategy,
            constraints: Vec::new(),
            breakpoint_overrides: HashMap::new(),
            aspect_ratio: Some(base_width / base_height),
            min_size: self.config.min_size_constraints,
            max_size: self.config.max_size_constraints,
        };

        // Store sizing system
        {
            let mut systems = self.sizing_systems.write().await;
            systems.insert(id, sizing.clone());
        }

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.total_sizing_operations += 1;

            match strategy {
                SizingStrategy::Fixed => stats.fixed_sizing += 1,
                SizingStrategy::Flexible => stats.flexible_sizing += 1,
                SizingStrategy::Responsive => stats.responsive_sizing += 1,
                SizingStrategy::Adaptive => stats.adaptive_sizing += 1,
                SizingStrategy::Fluid => stats.fluid_sizing += 1,
                SizingStrategy::Elastic => stats.elastic_sizing += 1,
            }
        }

        Ok(sizing)
    }

    /// Add size constraint
    pub async fn add_size_constraint(
        &self,
        sizing_id: &str,
        constraint: SizeConstraint,
    ) -> Result<(), AdaptiveSizingError> {
        let mut systems = self.sizing_systems.write().await;

        if let Some(sizing) = systems.get_mut(sizing_id) {
            sizing.constraints.push(constraint);

            // Update stats
            let mut stats = self.stats.write().await;
            stats.constraint_applications += 1;
        } else {
            return Err(AdaptiveSizingError::SizingSystemNotFound {
                id: sizing_id.to_string(),
            });
        }

        Ok(())
    }

    /// Apply adaptive sizing to a layout
    pub async fn apply_adaptive_sizing(
        &self,
        layout: &mut super::layout::ResponsiveLayout,
        breakpoint: &Breakpoint,
    ) -> Result<(), AdaptiveSizingError> {
        // Determine sizing strategy for breakpoint
        let sizing_strategy = self.get_sizing_strategy_for_breakpoint(breakpoint).await;

        // Apply sizing adjustments to layout items
        for item in &mut layout.items {
            if let Some(sizing_id) = item.responsive_properties.get("sizing_id") {
                if let Some(sizing_id_str) = sizing_id.as_str() {
                    if let Some(sizing) = self.get_adaptive_sizing(sizing_id_str).await {
                        let adaptive_sizing = self
                            .adapt_sizing_for_breakpoint(&sizing, breakpoint, sizing_strategy)
                            .await;

                        // Apply sizing properties to item
                        self.apply_sizing_to_item(item, &adaptive_sizing, breakpoint)
                            .await?;
                    }
                }
            }
        }

        // Update stats
        {
            let mut stats = self.stats.write().await;

            match breakpoint.device_type {
                DeviceType::Mobile => stats.mobile_sizing += 1,
                DeviceType::Tablet => stats.tablet_sizing += 1,
                DeviceType::Desktop => stats.desktop_sizing += 1,
                _ => {}
            }
        }

        Ok(())
    }

    /// Get sizing strategy for breakpoint
    async fn get_sizing_strategy_for_breakpoint(&self, breakpoint: &Breakpoint) -> SizingStrategy {
        match breakpoint.device_type {
            DeviceType::Mobile => self.config.mobile_sizing_strategy,
            DeviceType::Tablet => self.config.tablet_sizing_strategy,
            DeviceType::Desktop => self.config.desktop_sizing_strategy,
            _ => self.config.default_sizing_strategy,
        }
    }

    /// Adapt sizing for breakpoint
    async fn adapt_sizing_for_breakpoint(
        &self,
        sizing: &AdaptiveSizing,
        breakpoint: &Breakpoint,
        strategy: SizingStrategy,
    ) -> AdaptiveSizing {
        let mut adapted_sizing = sizing.clone();

        // Apply breakpoint-specific overrides
        if let Some(override_config) = sizing.breakpoint_overrides.get(breakpoint.name.as_str()) {
            if let Some(override_strategy) = override_config.sizing_strategy {
                adapted_sizing.sizing_strategy = override_strategy;
            }

            if let Some(override_width) = override_config.base_width {
                adapted_sizing.base_width = override_width;
            }

            if let Some(override_height) = override_config.base_height {
                adapted_sizing.base_height = override_height;
            }

            if let Some(override_aspect_ratio) = override_config.aspect_ratio {
                adapted_sizing.aspect_ratio = Some(override_aspect_ratio);
            }
        } else {
            // Apply default sizing strategy for breakpoint
            adapted_sizing.sizing_strategy = strategy;
        }

        // Apply scaling factor for breakpoint
        if let Some(scaling_factor) = self
            .config
            .breakpoint_scaling_factors
            .get(breakpoint.name.as_str())
        {
            adapted_sizing.base_width *= scaling_factor;
            adapted_sizing.base_height *= scaling_factor;

            // Update stats
            {
                let mut stats = self.stats.write().await;
                stats.scaling_operations += 1;
            }
        }

        // Apply aspect ratio preservation
        if self.config.aspect_ratio_preservation && adapted_sizing.aspect_ratio.is_some() {
            let aspect_ratio = adapted_sizing.aspect_ratio.unwrap();

            // Adjust height to maintain aspect ratio
            adapted_sizing.base_height = adapted_sizing.base_width / aspect_ratio;

            // Update stats
            {
                let mut stats = self.stats.write().await;
                stats.aspect_ratio_preservations += 1;
            }
        }

        adapted_sizing
    }

    /// Apply sizing to layout item
    async fn apply_sizing_to_item(
        &self,
        item: &mut super::layout::LayoutItem,
        sizing: &AdaptiveSizing,
        breakpoint: &Breakpoint,
    ) -> Result<(), AdaptiveSizingError> {
        // Apply sizing strategy
        item.responsive_properties.insert(
            "sizing_strategy".to_string(),
            serde_json::Value::String(format!("{:?}", sizing.sizing_strategy)),
        );

        // Apply base dimensions
        item.width = Some(sizing.base_width);
        item.height = Some(sizing.base_height);

        // Apply constraints
        for constraint in &sizing.constraints {
            self.apply_constraint_to_item(item, constraint, breakpoint)
                .await?;
        }

        // Apply min/max size constraints
        item.min_width = Some(sizing.min_size.0);
        item.min_height = Some(sizing.min_size.1);
        item.max_width = Some(sizing.max_size.0);
        item.max_height = Some(sizing.max_size.1);

        // Apply aspect ratio if specified
        if let Some(aspect_ratio) = sizing.aspect_ratio {
            item.responsive_properties.insert(
                "aspect_ratio".to_string(),
                serde_json::Value::Number(serde_json::Number::from_f64(aspect_ratio).unwrap()),
            );
        }

        Ok(())
    }

    /// Apply constraint to layout item
    async fn apply_constraint_to_item(
        &self,
        item: &mut super::layout::LayoutItem,
        constraint: &SizeConstraint,
        breakpoint: &Breakpoint,
    ) -> Result<(), AdaptiveSizingError> {
        let constraint_value = if constraint.breakpoint_specific {
            constraint
                .breakpoint_values
                .get(breakpoint.name.as_str())
                .unwrap_or(&constraint.value)
        } else {
            &constraint.value
        };

        match constraint.constraint_type {
            SizeConstraintType::MinWidth => {
                item.min_width = Some(item.min_width.unwrap_or(0.0).max(*constraint_value));
            }
            SizeConstraintType::MaxWidth => {
                item.max_width = Some(
                    item.max_width
                        .unwrap_or(f64::INFINITY)
                        .min(*constraint_value),
                );
            }
            SizeConstraintType::MinHeight => {
                item.min_height = Some(item.min_height.unwrap_or(0.0).max(*constraint_value));
            }
            SizeConstraintType::MaxHeight => {
                item.max_height = Some(
                    item.max_height
                        .unwrap_or(f64::INFINITY)
                        .min(*constraint_value),
                );
            }
            SizeConstraintType::FixedWidth => {
                item.width = Some(*constraint_value);
            }
            SizeConstraintType::FixedHeight => {
                item.height = Some(*constraint_value);
            }
            SizeConstraintType::AspectRatio => {
                item.responsive_properties.insert(
                    "aspect_ratio".to_string(),
                    serde_json::Value::Number(
                        serde_json::Number::from_f64(*constraint_value).unwrap(),
                    ),
                );
            }
            SizeConstraintType::Flexible => {
                item.responsive_properties
                    .insert("flexible".to_string(), serde_json::Value::Bool(true));
            }
        }

        Ok(())
    }

    /// Get adaptive sizing by ID
    pub async fn get_adaptive_sizing(&self, id: &str) -> Option<AdaptiveSizing> {
        let systems = self.sizing_systems.read().await;
        systems.get(id).cloned()
    }

    /// Update adaptive sizing
    pub async fn update_adaptive_sizing(
        &self,
        id: &str,
        sizing: AdaptiveSizing,
    ) -> Result<(), AdaptiveSizingError> {
        let mut systems = self.sizing_systems.write().await;
        systems.insert(id.to_string(), sizing);
        Ok(())
    }

    /// Update adaptive sizing configuration
    pub async fn update_config(
        &mut self,
        config: AdaptiveSizingConfig,
    ) -> Result<(), AdaptiveSizingError> {
        self.config = config;
        Ok(())
    }

    /// Get adaptive sizing statistics
    pub async fn get_stats(&self) -> AdaptiveSizingStats {
        self.stats.read().await.clone()
    }

    /// Remove adaptive sizing
    pub async fn remove_adaptive_sizing(&self, id: &str) -> Result<(), AdaptiveSizingError> {
        let mut systems = self.sizing_systems.write().await;
        systems.remove(id);
        Ok(())
    }

    /// Get all adaptive sizing systems
    pub async fn get_all_adaptive_sizing(&self) -> HashMap<String, AdaptiveSizing> {
        self.sizing_systems.read().await.clone()
    }
}

/// Adaptive sizing error types
#[derive(Debug, Error)]
pub enum AdaptiveSizingError {
    #[error("Sizing system not found: {id}")]
    SizingSystemNotFound { id: String },

    #[error("Invalid sizing configuration: {message}")]
    InvalidConfiguration { message: String },

    #[error("Size constraint error: {message}")]
    SizeConstraintError { message: String },

    #[error("Scaling operation error: {message}")]
    ScalingError { message: String },

    #[error("Aspect ratio calculation error: {message}")]
    AspectRatioError { message: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_sizing_config() -> AdaptiveSizingConfig {
        AdaptiveSizingConfig {
            default_sizing_strategy: SizingStrategy::Responsive,
            mobile_sizing_strategy: SizingStrategy::Fixed,
            tablet_sizing_strategy: SizingStrategy::Flexible,
            desktop_sizing_strategy: SizingStrategy::Adaptive,
            fluid_sizing_enabled: true,
            elastic_sizing_enabled: true,
            aspect_ratio_preservation: true,
            min_size_constraints: (100.0, 100.0),
            max_size_constraints: (2000.0, 2000.0),
            breakpoint_scaling_factors: {
                let mut factors = HashMap::new();
                factors.insert("mobile".to_string(), 0.8);
                factors.insert("tablet".to_string(), 1.0);
                factors.insert("desktop".to_string(), 1.2);
                factors
            },
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

    fn create_test_size_constraint() -> SizeConstraint {
        SizeConstraint {
            constraint_type: SizeConstraintType::MinWidth,
            value: 200.0,
            breakpoint_specific: false,
            breakpoint_values: HashMap::new(),
            priority: 1,
        }
    }

    #[tokio::test]
    async fn test_adaptive_sizing_manager_creation() {
        let config = create_test_sizing_config();
        let manager = AdaptiveSizingManager::new(config);

        let stats = manager.get_stats().await;
        assert_eq!(stats.total_sizing_operations, 0);
    }

    #[tokio::test]
    async fn test_create_adaptive_sizing() {
        let config = create_test_sizing_config();
        let manager = AdaptiveSizingManager::new(config);

        let sizing = manager
            .create_adaptive_sizing(
                "test-sizing".to_string(),
                800.0,
                600.0,
                SizingStrategy::Responsive,
            )
            .await
            .unwrap();

        assert_eq!(sizing.id, "test-sizing");
        assert_eq!(sizing.base_width, 800.0);
        assert_eq!(sizing.base_height, 600.0);
        assert_eq!(sizing.sizing_strategy, SizingStrategy::Responsive);
        assert_eq!(sizing.aspect_ratio, Some(800.0 / 600.0));

        let stats = manager.get_stats().await;
        assert_eq!(stats.total_sizing_operations, 1);
        assert_eq!(stats.responsive_sizing, 1);
    }

    #[tokio::test]
    async fn test_add_size_constraint() {
        let config = create_test_sizing_config();
        let manager = AdaptiveSizingManager::new(config);

        manager
            .create_adaptive_sizing(
                "test-sizing".to_string(),
                800.0,
                600.0,
                SizingStrategy::Responsive,
            )
            .await
            .unwrap();
        let constraint = create_test_size_constraint();

        manager
            .add_size_constraint("test-sizing", constraint)
            .await
            .unwrap();

        let sizing = manager.get_adaptive_sizing("test-sizing").await.unwrap();
        assert_eq!(sizing.constraints.len(), 1);
        assert_eq!(
            sizing.constraints[0].constraint_type,
            SizeConstraintType::MinWidth
        );
        assert_eq!(sizing.constraints[0].value, 200.0);

        let stats = manager.get_stats().await;
        assert_eq!(stats.constraint_applications, 1);
    }

    #[tokio::test]
    async fn test_sizing_strategy_for_breakpoint() {
        let config = create_test_sizing_config();
        let manager = AdaptiveSizingManager::new(config);

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

        let mobile_strategy = manager
            .get_sizing_strategy_for_breakpoint(&mobile_breakpoint)
            .await;
        let desktop_strategy = manager
            .get_sizing_strategy_for_breakpoint(&desktop_breakpoint)
            .await;

        assert_eq!(mobile_strategy, SizingStrategy::Fixed);
        assert_eq!(desktop_strategy, SizingStrategy::Adaptive);
    }

    #[tokio::test]
    async fn test_adapt_sizing_for_breakpoint() {
        let config = create_test_sizing_config();
        let manager = AdaptiveSizingManager::new(config);

        let sizing = manager
            .create_adaptive_sizing(
                "test-sizing".to_string(),
                800.0,
                600.0,
                SizingStrategy::Responsive,
            )
            .await
            .unwrap();
        let mobile_breakpoint = create_test_breakpoint();

        let adapted_sizing = manager
            .adapt_sizing_for_breakpoint(&sizing, &mobile_breakpoint, SizingStrategy::Fixed)
            .await;

        assert_eq!(adapted_sizing.sizing_strategy, SizingStrategy::Fixed);
        assert_eq!(adapted_sizing.base_width, 800.0 * 0.8); // Mobile scaling factor
        assert_eq!(adapted_sizing.base_height, 600.0 * 0.8);

        let stats = manager.get_stats().await;
        assert_eq!(stats.scaling_operations, 1);
        assert_eq!(stats.aspect_ratio_preservations, 1);
    }

    #[tokio::test]
    async fn test_aspect_ratio_preservation() {
        let config = create_test_sizing_config();
        let manager = AdaptiveSizingManager::new(config);

        let sizing = manager
            .create_adaptive_sizing(
                "test-sizing".to_string(),
                800.0,
                600.0,
                SizingStrategy::Responsive,
            )
            .await
            .unwrap();
        let mobile_breakpoint = create_test_breakpoint();

        let adapted_sizing = manager
            .adapt_sizing_for_breakpoint(&sizing, &mobile_breakpoint, SizingStrategy::Fixed)
            .await;

        // Aspect ratio should be preserved
        let expected_aspect_ratio = 800.0 / 600.0;
        let actual_aspect_ratio = adapted_sizing.base_width / adapted_sizing.base_height;
        assert!((actual_aspect_ratio - expected_aspect_ratio).abs() < 0.001);
    }

    #[tokio::test]
    async fn test_get_adaptive_sizing() {
        let config = create_test_sizing_config();
        let manager = AdaptiveSizingManager::new(config);

        manager
            .create_adaptive_sizing(
                "test-sizing".to_string(),
                800.0,
                600.0,
                SizingStrategy::Responsive,
            )
            .await
            .unwrap();

        let sizing = manager.get_adaptive_sizing("test-sizing").await;
        assert!(sizing.is_some());
        assert_eq!(sizing.unwrap().id, "test-sizing");
    }

    #[tokio::test]
    async fn test_remove_adaptive_sizing() {
        let config = create_test_sizing_config();
        let manager = AdaptiveSizingManager::new(config);

        manager
            .create_adaptive_sizing(
                "test-sizing".to_string(),
                800.0,
                600.0,
                SizingStrategy::Responsive,
            )
            .await
            .unwrap();

        let sizing = manager.get_adaptive_sizing("test-sizing").await;
        assert!(sizing.is_some());

        manager.remove_adaptive_sizing("test-sizing").await.unwrap();

        let sizing = manager.get_adaptive_sizing("test-sizing").await;
        assert!(sizing.is_none());
    }

    #[tokio::test]
    async fn test_sizing_system_not_found() {
        let config = create_test_sizing_config();
        let manager = AdaptiveSizingManager::new(config);

        let constraint = create_test_size_constraint();
        let result = manager.add_size_constraint("nonexistent", constraint).await;

        assert!(matches!(
            result,
            Err(AdaptiveSizingError::SizingSystemNotFound { .. })
        ));
    }

    #[tokio::test]
    async fn test_config_update() {
        let mut config = create_test_sizing_config();
        config.mobile_sizing_strategy = SizingStrategy::Flexible;

        let mut manager = AdaptiveSizingManager::new(config);

        let new_config = create_test_sizing_config();
        manager.update_config(new_config).await.unwrap();

        // Verify config was updated
        let mobile_breakpoint = create_test_breakpoint();
        let strategy = manager
            .get_sizing_strategy_for_breakpoint(&mobile_breakpoint)
            .await;
        assert_eq!(strategy, SizingStrategy::Fixed); // Should use new config
    }

    #[tokio::test]
    async fn test_adaptive_sizing_stats() {
        let config = create_test_sizing_config();
        let manager = AdaptiveSizingManager::new(config);

        manager
            .create_adaptive_sizing(
                "test-sizing".to_string(),
                800.0,
                600.0,
                SizingStrategy::Responsive,
            )
            .await
            .unwrap();

        let stats = manager.get_stats().await;
        assert_eq!(stats.total_sizing_operations, 1);
        assert_eq!(stats.responsive_sizing, 1);
    }
}
