//! Responsive Typography for leptos-helios
//!
//! This module provides responsive typography management for charts and visualizations,
//! including font scaling, responsive text sizing, and device-specific typography.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use thiserror::Error;
use tokio::sync::RwLock;

use super::breakpoints::{Breakpoint, DeviceType, Orientation};

/// Typography size scale
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TypographySize {
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XXLarge,
    XXXLarge,
}

/// Typography scale configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographyScale {
    pub base_size: f64,
    pub scale_factor: f64,
    pub line_height: f64,
    pub letter_spacing: f64,
    pub font_weight: f64,
}

impl Default for TypographyScale {
    fn default() -> Self {
        Self {
            base_size: 16.0,
            scale_factor: 1.2,
            line_height: 1.5,
            letter_spacing: 0.0,
            font_weight: 400.0,
        }
    }
}

/// Font scale configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontScale {
    pub xsmall: f64,
    pub small: f64,
    pub medium: f64,
    pub large: f64,
    pub xlarge: f64,
    pub xxlarge: f64,
    pub xxxlarge: f64,
}

impl Default for FontScale {
    fn default() -> Self {
        Self {
            xsmall: 12.0,
            small: 14.0,
            medium: 16.0,
            large: 18.0,
            xlarge: 20.0,
            xxlarge: 24.0,
            xxxlarge: 32.0,
        }
    }
}

/// Responsive typography configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsiveTypography {
    pub id: String,
    pub font_family: String,
    pub font_scale: FontScale,
    pub responsive_scales: HashMap<String, TypographyScale>,
    pub breakpoint_overrides: HashMap<String, TypographyOverride>,
    pub accessibility_enabled: bool,
    pub high_contrast_mode: bool,
}

/// Typography override for specific breakpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographyOverride {
    pub font_scale: Option<FontScale>,
    pub base_size: Option<f64>,
    pub scale_factor: Option<f64>,
    pub line_height: Option<f64>,
    pub letter_spacing: Option<f64>,
    pub font_weight: Option<f64>,
}

/// Typography manager for responsive design
pub struct TypographyManager {
    config: TypographyConfig,
    typography_systems: Arc<RwLock<HashMap<String, ResponsiveTypography>>>,
    stats: Arc<RwLock<TypographyStats>>,
}

/// Typography configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographyConfig {
    pub default_font_family: String,
    pub default_font_scale: FontScale,
    pub responsive_scaling: bool,
    pub accessibility_scaling: bool,
    pub high_contrast_support: bool,
    pub mobile_scale_factor: f64,
    pub tablet_scale_factor: f64,
    pub desktop_scale_factor: f64,
    pub large_desktop_scale_factor: f64,
    pub ultra_wide_scale_factor: f64,
}

impl Default for TypographyConfig {
    fn default() -> Self {
        Self {
            default_font_family: "system-ui, -apple-system, sans-serif".to_string(),
            default_font_scale: FontScale::default(),
            responsive_scaling: true,
            accessibility_scaling: true,
            high_contrast_support: true,
            mobile_scale_factor: 0.875,
            tablet_scale_factor: 1.0,
            desktop_scale_factor: 1.125,
            large_desktop_scale_factor: 1.25,
            ultra_wide_scale_factor: 1.375,
        }
    }
}

/// Typography statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographyStats {
    pub total_typography_systems: usize,
    pub responsive_typography: usize,
    pub mobile_typography: usize,
    pub tablet_typography: usize,
    pub desktop_typography: usize,
    pub accessibility_applications: usize,
    pub high_contrast_applications: usize,
    pub font_scale_calculations: usize,
    pub responsive_adjustments: usize,
}

impl Default for TypographyStats {
    fn default() -> Self {
        Self {
            total_typography_systems: 0,
            responsive_typography: 0,
            mobile_typography: 0,
            tablet_typography: 0,
            desktop_typography: 0,
            accessibility_applications: 0,
            high_contrast_applications: 0,
            font_scale_calculations: 0,
            responsive_adjustments: 0,
        }
    }
}

impl TypographyManager {
    /// Create a new typography manager
    pub fn new(config: TypographyConfig) -> Self {
        Self {
            config,
            typography_systems: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(TypographyStats::default())),
        }
    }

    /// Create a new responsive typography system
    pub async fn create_typography_system(
        &self,
        id: String,
        font_family: String,
    ) -> Result<ResponsiveTypography, TypographyError> {
        let typography = ResponsiveTypography {
            id: id.clone(),
            font_family,
            font_scale: self.config.default_font_scale.clone(),
            responsive_scales: HashMap::new(),
            breakpoint_overrides: HashMap::new(),
            accessibility_enabled: self.config.accessibility_scaling,
            high_contrast_mode: false,
        };

        // Store typography system
        {
            let mut systems = self.typography_systems.write().await;
            systems.insert(id, typography.clone());
        }

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.total_typography_systems += 1;
            stats.responsive_typography += 1;
        }

        Ok(typography)
    }

    /// Apply responsive typography to a layout
    pub async fn apply_responsive_typography(
        &self,
        layout: &mut super::layout::ResponsiveLayout,
        breakpoint: &Breakpoint,
    ) -> Result<(), TypographyError> {
        // Apply responsive scaling based on breakpoint
        let scale_factor = self.get_scale_factor_for_breakpoint(breakpoint).await;

        // Update typography systems in the layout
        for item in &mut layout.items {
            if let Some(typography_id) = item.responsive_properties.get("typography_id") {
                if let Some(typography_id_str) = typography_id.as_str() {
                    if let Some(typography) = self.get_typography_system(typography_id_str).await {
                        let scaled_typography = self
                            .scale_typography_for_breakpoint(&typography, breakpoint, scale_factor)
                            .await;

                        // Apply scaled typography to item
                        self.apply_typography_to_item(item, &scaled_typography, breakpoint)
                            .await?;
                    }
                }
            }
        }

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.responsive_adjustments += 1;

            match breakpoint.device_type {
                DeviceType::Mobile => stats.mobile_typography += 1,
                DeviceType::Tablet => stats.tablet_typography += 1,
                DeviceType::Desktop => stats.desktop_typography += 1,
                _ => {}
            }
        }

        Ok(())
    }

    /// Get scale factor for breakpoint
    async fn get_scale_factor_for_breakpoint(&self, breakpoint: &Breakpoint) -> f64 {
        match breakpoint.device_type {
            DeviceType::Mobile => self.config.mobile_scale_factor,
            DeviceType::Tablet => self.config.tablet_scale_factor,
            DeviceType::Desktop => self.config.desktop_scale_factor,
            DeviceType::LargeDesktop => self.config.large_desktop_scale_factor,
            DeviceType::UltraWide => self.config.ultra_wide_scale_factor,
        }
    }

    /// Scale typography for breakpoint
    async fn scale_typography_for_breakpoint(
        &self,
        typography: &ResponsiveTypography,
        breakpoint: &Breakpoint,
        scale_factor: f64,
    ) -> ResponsiveTypography {
        let mut scaled_typography = typography.clone();

        // Apply breakpoint-specific overrides
        if let Some(override_config) = typography
            .breakpoint_overrides
            .get(breakpoint.name.as_str())
        {
            if let Some(override_scale) = &override_config.font_scale {
                scaled_typography.font_scale = override_scale.clone();
            }
        }

        // Apply scale factor
        scaled_typography.font_scale.xsmall *= scale_factor;
        scaled_typography.font_scale.small *= scale_factor;
        scaled_typography.font_scale.medium *= scale_factor;
        scaled_typography.font_scale.large *= scale_factor;
        scaled_typography.font_scale.xlarge *= scale_factor;
        scaled_typography.font_scale.xxlarge *= scale_factor;
        scaled_typography.font_scale.xxxlarge *= scale_factor;

        scaled_typography
    }

    /// Apply typography to layout item
    async fn apply_typography_to_item(
        &self,
        item: &mut super::layout::LayoutItem,
        typography: &ResponsiveTypography,
        breakpoint: &Breakpoint,
    ) -> Result<(), TypographyError> {
        // Apply typography properties to item
        if let Some(typography_size) = item.responsive_properties.get("typography_size") {
            if let Some(size_str) = typography_size.as_str() {
                let size = match size_str {
                    "xsmall" => TypographySize::XSmall,
                    "small" => TypographySize::Small,
                    "medium" => TypographySize::Medium,
                    "large" => TypographySize::Large,
                    "xlarge" => TypographySize::XLarge,
                    "xxlarge" => TypographySize::XXLarge,
                    "xxxlarge" => TypographySize::XXXLarge,
                    _ => TypographySize::Medium,
                };

                let font_size =
                    self.get_font_size_for_typography_size(&typography.font_scale, size);

                // Update item properties with font size
                item.responsive_properties.insert(
                    "font_size".to_string(),
                    serde_json::Value::Number(serde_json::Number::from_f64(font_size).unwrap()),
                );
            }
        }

        // Apply accessibility scaling if enabled
        if typography.accessibility_enabled && self.config.accessibility_scaling {
            self.apply_accessibility_scaling(item, breakpoint).await?;
        }

        // Apply high contrast mode if enabled
        if typography.high_contrast_mode && self.config.high_contrast_support {
            self.apply_high_contrast_mode(item).await?;
        }

        Ok(())
    }

    /// Get font size for typography size
    fn get_font_size_for_typography_size(
        &self,
        font_scale: &FontScale,
        size: TypographySize,
    ) -> f64 {
        match size {
            TypographySize::XSmall => font_scale.xsmall,
            TypographySize::Small => font_scale.small,
            TypographySize::Medium => font_scale.medium,
            TypographySize::Large => font_scale.large,
            TypographySize::XLarge => font_scale.xlarge,
            TypographySize::XXLarge => font_scale.xxlarge,
            TypographySize::XXXLarge => font_scale.xxxlarge,
        }
    }

    /// Apply accessibility scaling
    async fn apply_accessibility_scaling(
        &self,
        item: &mut super::layout::LayoutItem,
        breakpoint: &Breakpoint,
    ) -> Result<(), TypographyError> {
        // Increase font size for better accessibility
        let accessibility_factor = 1.2;

        if let Some(font_size) = item.responsive_properties.get("font_size") {
            if let Some(size_value) = font_size.as_f64() {
                let new_size = size_value * accessibility_factor;
                item.responsive_properties.insert(
                    "font_size".to_string(),
                    serde_json::Value::Number(serde_json::Number::from_f64(new_size).unwrap()),
                );
            }
        }

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.accessibility_applications += 1;
        }

        Ok(())
    }

    /// Apply high contrast mode
    async fn apply_high_contrast_mode(
        &self,
        item: &mut super::layout::LayoutItem,
    ) -> Result<(), TypographyError> {
        // Apply high contrast styling
        item.responsive_properties
            .insert("high_contrast".to_string(), serde_json::Value::Bool(true));
        item.responsive_properties.insert(
            "font_weight".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(600.0).unwrap()),
        );

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.high_contrast_applications += 1;
        }

        Ok(())
    }

    /// Get typography system by ID
    pub async fn get_typography_system(&self, id: &str) -> Option<ResponsiveTypography> {
        let systems = self.typography_systems.read().await;
        systems.get(id).cloned()
    }

    /// Update typography system
    pub async fn update_typography_system(
        &self,
        id: &str,
        typography: ResponsiveTypography,
    ) -> Result<(), TypographyError> {
        let mut systems = self.typography_systems.write().await;
        systems.insert(id.to_string(), typography);
        Ok(())
    }

    /// Update typography configuration
    pub async fn update_config(&mut self, config: TypographyConfig) -> Result<(), TypographyError> {
        self.config = config;
        Ok(())
    }

    /// Get typography statistics
    pub async fn get_stats(&self) -> TypographyStats {
        self.stats.read().await.clone()
    }

    /// Calculate font scale for breakpoint
    pub async fn calculate_font_scale(&self, breakpoint: &Breakpoint) -> FontScale {
        let scale_factor = self.get_scale_factor_for_breakpoint(breakpoint).await;

        FontScale {
            xsmall: self.config.default_font_scale.xsmall * scale_factor,
            small: self.config.default_font_scale.small * scale_factor,
            medium: self.config.default_font_scale.medium * scale_factor,
            large: self.config.default_font_scale.large * scale_factor,
            xlarge: self.config.default_font_scale.xlarge * scale_factor,
            xxlarge: self.config.default_font_scale.xxlarge * scale_factor,
            xxxlarge: self.config.default_font_scale.xxxlarge * scale_factor,
        }
    }

    /// Remove typography system
    pub async fn remove_typography_system(&self, id: &str) -> Result<(), TypographyError> {
        let mut systems = self.typography_systems.write().await;
        systems.remove(id);
        Ok(())
    }

    /// Get all typography systems
    pub async fn get_all_typography_systems(&self) -> HashMap<String, ResponsiveTypography> {
        self.typography_systems.read().await.clone()
    }
}

/// Typography error types
#[derive(Debug, Error)]
pub enum TypographyError {
    #[error("Typography system not found: {id}")]
    TypographySystemNotFound { id: String },

    #[error("Invalid typography configuration: {message}")]
    InvalidConfiguration { message: String },

    #[error("Font scale calculation error: {message}")]
    FontScaleError { message: String },

    #[error("Accessibility scaling error: {message}")]
    AccessibilityError { message: String },

    #[error("High contrast mode error: {message}")]
    HighContrastError { message: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_typography_config() -> TypographyConfig {
        TypographyConfig {
            default_font_family: "Arial, sans-serif".to_string(),
            default_font_scale: FontScale::default(),
            responsive_scaling: true,
            accessibility_scaling: true,
            high_contrast_support: true,
            mobile_scale_factor: 0.875,
            tablet_scale_factor: 1.0,
            desktop_scale_factor: 1.125,
            large_desktop_scale_factor: 1.25,
            ultra_wide_scale_factor: 1.375,
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
    async fn test_typography_manager_creation() {
        let config = create_test_typography_config();
        let manager = TypographyManager::new(config);

        let stats = manager.get_stats().await;
        assert_eq!(stats.total_typography_systems, 0);
    }

    #[tokio::test]
    async fn test_create_typography_system() {
        let config = create_test_typography_config();
        let manager = TypographyManager::new(config);

        let typography = manager
            .create_typography_system(
                "test-typography".to_string(),
                "Arial, sans-serif".to_string(),
            )
            .await
            .unwrap();

        assert_eq!(typography.id, "test-typography");
        assert_eq!(typography.font_family, "Arial, sans-serif");
        assert!(typography.accessibility_enabled);

        let stats = manager.get_stats().await;
        assert_eq!(stats.total_typography_systems, 1);
        assert_eq!(stats.responsive_typography, 1);
    }

    #[tokio::test]
    async fn test_get_typography_system() {
        let config = create_test_typography_config();
        let manager = TypographyManager::new(config);

        manager
            .create_typography_system(
                "test-typography".to_string(),
                "Arial, sans-serif".to_string(),
            )
            .await
            .unwrap();

        let typography = manager.get_typography_system("test-typography").await;
        assert!(typography.is_some());
        assert_eq!(typography.unwrap().id, "test-typography");
    }

    #[tokio::test]
    async fn test_calculate_font_scale() {
        let config = create_test_typography_config();
        let manager = TypographyManager::new(config);

        let breakpoint = create_test_breakpoint();
        let font_scale = manager.calculate_font_scale(&breakpoint).await;

        // Mobile scale factor is 0.875
        assert_eq!(font_scale.medium, 16.0 * 0.875);
        assert_eq!(font_scale.large, 18.0 * 0.875);
    }

    #[tokio::test]
    async fn test_scale_factor_for_breakpoint() {
        let config = create_test_typography_config();
        let manager = TypographyManager::new(config);

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

        let mobile_scale = manager
            .get_scale_factor_for_breakpoint(&mobile_breakpoint)
            .await;
        let desktop_scale = manager
            .get_scale_factor_for_breakpoint(&desktop_breakpoint)
            .await;

        assert_eq!(mobile_scale, 0.875);
        assert_eq!(desktop_scale, 1.125);
    }

    #[tokio::test]
    async fn test_typography_size_enum() {
        let config = create_test_typography_config();
        let manager = TypographyManager::new(config);

        let font_scale = FontScale::default();

        let xsmall_size =
            manager.get_font_size_for_typography_size(&font_scale, TypographySize::XSmall);
        let medium_size =
            manager.get_font_size_for_typography_size(&font_scale, TypographySize::Medium);
        let xxxlarge_size =
            manager.get_font_size_for_typography_size(&font_scale, TypographySize::XXXLarge);

        assert_eq!(xsmall_size, 12.0);
        assert_eq!(medium_size, 16.0);
        assert_eq!(xxxlarge_size, 32.0);
    }

    #[tokio::test]
    async fn test_update_typography_system() {
        let config = create_test_typography_config();
        let manager = TypographyManager::new(config);

        let mut typography = manager
            .create_typography_system(
                "test-typography".to_string(),
                "Arial, sans-serif".to_string(),
            )
            .await
            .unwrap();
        typography.font_family = "Helvetica, sans-serif".to_string();

        manager
            .update_typography_system("test-typography", typography)
            .await
            .unwrap();

        let updated_typography = manager
            .get_typography_system("test-typography")
            .await
            .unwrap();
        assert_eq!(updated_typography.font_family, "Helvetica, sans-serif");
    }

    #[tokio::test]
    async fn test_remove_typography_system() {
        let config = create_test_typography_config();
        let manager = TypographyManager::new(config);

        manager
            .create_typography_system(
                "test-typography".to_string(),
                "Arial, sans-serif".to_string(),
            )
            .await
            .unwrap();

        let typography = manager.get_typography_system("test-typography").await;
        assert!(typography.is_some());

        manager
            .remove_typography_system("test-typography")
            .await
            .unwrap();

        let typography = manager.get_typography_system("test-typography").await;
        assert!(typography.is_none());
    }

    #[tokio::test]
    async fn test_config_update() {
        let mut config = create_test_typography_config();
        config.mobile_scale_factor = 0.9;

        let mut manager = TypographyManager::new(config);

        let new_config = create_test_typography_config();
        manager.update_config(new_config).await.unwrap();

        // Verify config was updated
        let breakpoint = create_test_breakpoint();
        let font_scale = manager.calculate_font_scale(&breakpoint).await;
        assert_eq!(font_scale.medium, 16.0 * 0.875); // Should use new config
    }

    #[tokio::test]
    async fn test_typography_stats() {
        let config = create_test_typography_config();
        let manager = TypographyManager::new(config);

        manager
            .create_typography_system(
                "test-typography".to_string(),
                "Arial, sans-serif".to_string(),
            )
            .await
            .unwrap();

        let stats = manager.get_stats().await;
        assert_eq!(stats.total_typography_systems, 1);
        assert_eq!(stats.responsive_typography, 1);
    }
}
