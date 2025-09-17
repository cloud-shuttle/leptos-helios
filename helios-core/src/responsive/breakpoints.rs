//! Responsive Breakpoints for leptos-helios
//!
//! This module provides responsive breakpoint management for mobile-first design,
//! including device detection, orientation handling, and breakpoint-based styling.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use thiserror::Error;
use tokio::sync::RwLock;

/// Represents a responsive breakpoint
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Breakpoint {
    pub name: String,
    pub min_width: f64,
    pub max_width: Option<f64>,
    pub device_type: DeviceType,
    pub orientation: Orientation,
}

/// Device types for responsive design
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DeviceType {
    Mobile,
    Tablet,
    Desktop,
    LargeDesktop,
    UltraWide,
}

/// Screen orientation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Orientation {
    Portrait,
    Landscape,
}

/// Breakpoint types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BreakpointType {
    Mobile,
    Tablet,
    Desktop,
    LargeDesktop,
    UltraWide,
}

/// Breakpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakpointConfig {
    pub mobile_breakpoint: f64,
    pub tablet_breakpoint: f64,
    pub desktop_breakpoint: f64,
    pub large_desktop_breakpoint: f64,
    pub ultra_wide_breakpoint: f64,
    pub orientation_detection: bool,
    pub device_detection: bool,
    pub custom_breakpoints: HashMap<String, Breakpoint>,
}

impl Default for BreakpointConfig {
    fn default() -> Self {
        Self {
            mobile_breakpoint: 768.0,
            tablet_breakpoint: 1024.0,
            desktop_breakpoint: 1200.0,
            large_desktop_breakpoint: 1440.0,
            ultra_wide_breakpoint: 1920.0,
            orientation_detection: true,
            device_detection: true,
            custom_breakpoints: HashMap::new(),
        }
    }
}

/// Breakpoint manager for responsive design
pub struct BreakpointManager {
    config: BreakpointConfig,
    current_breakpoint: Arc<RwLock<Breakpoint>>,
    viewport_size: Arc<RwLock<(f64, f64)>>,
    orientation: Arc<RwLock<Orientation>>,
    device_type: Arc<RwLock<DeviceType>>,
    breakpoint_history: Arc<RwLock<Vec<Breakpoint>>>,
    stats: Arc<RwLock<BreakpointStats>>,
}

/// Breakpoint statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakpointStats {
    pub total_breakpoint_changes: usize,
    pub mobile_views: usize,
    pub tablet_views: usize,
    pub desktop_views: usize,
    pub large_desktop_views: usize,
    pub ultra_wide_views: usize,
    pub portrait_views: usize,
    pub landscape_views: usize,
    pub last_breakpoint_change: Option<u64>,
    pub average_viewport_width: f64,
    pub average_viewport_height: f64,
}

impl Default for BreakpointStats {
    fn default() -> Self {
        Self {
            total_breakpoint_changes: 0,
            mobile_views: 0,
            tablet_views: 0,
            desktop_views: 0,
            large_desktop_views: 0,
            ultra_wide_views: 0,
            portrait_views: 0,
            landscape_views: 0,
            last_breakpoint_change: None,
            average_viewport_width: 0.0,
            average_viewport_height: 0.0,
        }
    }
}

impl BreakpointManager {
    /// Create a new breakpoint manager
    pub fn new(config: BreakpointConfig) -> Self {
        let default_breakpoint = Self::get_default_mobile_breakpoint();

        Self {
            config,
            current_breakpoint: Arc::new(RwLock::new(default_breakpoint)),
            viewport_size: Arc::new(RwLock::new((375.0, 667.0))), // Default mobile size
            orientation: Arc::new(RwLock::new(Orientation::Portrait)),
            device_type: Arc::new(RwLock::new(DeviceType::Mobile)),
            breakpoint_history: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(RwLock::new(BreakpointStats::default())),
        }
    }

    /// Update viewport size and recalculate breakpoint
    pub async fn update_viewport_size(
        &self,
        width: f64,
        height: f64,
    ) -> Result<Breakpoint, BreakpointError> {
        // Validate viewport size
        if width <= 0.0 || height <= 0.0 {
            return Err(BreakpointError::InvalidViewportSize { width, height });
        }

        // Update viewport size
        {
            let mut viewport = self.viewport_size.write().await;
            *viewport = (width, height);
        }

        // Update orientation
        {
            let mut orientation = self.orientation.write().await;
            *orientation = if width > height {
                Orientation::Landscape
            } else {
                Orientation::Portrait
            };
        }

        // Determine device type
        let device_type = self.determine_device_type(width).await;
        {
            let mut device = self.device_type.write().await;
            *device = device_type;
        }

        // Calculate new breakpoint
        let new_breakpoint = self.calculate_breakpoint(width, height, device_type).await;

        // Check if breakpoint changed
        let current_breakpoint = self.current_breakpoint.read().await;
        if *current_breakpoint != new_breakpoint {
            drop(current_breakpoint);

            // Update current breakpoint
            {
                let mut current = self.current_breakpoint.write().await;
                *current = new_breakpoint.clone();
            }

            // Add to history
            {
                let mut history = self.breakpoint_history.write().await;
                history.push(new_breakpoint.clone());
                if history.len() > 100 {
                    history.remove(0);
                }
            }

            // Update stats
            self.update_stats(&new_breakpoint, width, height).await;
        }

        Ok(new_breakpoint)
    }

    /// Get current breakpoint
    pub async fn get_current_breakpoint(&self) -> Breakpoint {
        self.current_breakpoint.read().await.clone()
    }

    /// Get current viewport size
    pub async fn get_viewport_size(&self) -> (f64, f64) {
        self.viewport_size.read().await.clone()
    }

    /// Get current orientation
    pub async fn get_orientation(&self) -> Orientation {
        self.orientation.read().await.clone()
    }

    /// Get current device type
    pub async fn get_device_type(&self) -> DeviceType {
        self.device_type.read().await.clone()
    }

    /// Get breakpoint history
    pub async fn get_breakpoint_history(&self) -> Vec<Breakpoint> {
        self.breakpoint_history.read().await.clone()
    }

    /// Get breakpoint statistics
    pub async fn get_stats(&self) -> BreakpointStats {
        self.stats.read().await.clone()
    }

    /// Update breakpoint configuration
    pub async fn update_config(&mut self, config: BreakpointConfig) -> Result<(), BreakpointError> {
        self.config = config;

        // Recalculate current breakpoint with new config
        let (width, height) = self.get_viewport_size().await;
        let device_type = self.get_device_type().await;
        let new_breakpoint = self.calculate_breakpoint(width, height, device_type).await;

        {
            let mut current = self.current_breakpoint.write().await;
            *current = new_breakpoint;
        }

        Ok(())
    }

    /// Check if current breakpoint matches a specific type
    pub async fn is_breakpoint(&self, breakpoint_type: BreakpointType) -> bool {
        let current = self.get_current_breakpoint().await;
        match breakpoint_type {
            BreakpointType::Mobile => current.device_type == DeviceType::Mobile,
            BreakpointType::Tablet => current.device_type == DeviceType::Tablet,
            BreakpointType::Desktop => current.device_type == DeviceType::Desktop,
            BreakpointType::LargeDesktop => current.device_type == DeviceType::LargeDesktop,
            BreakpointType::UltraWide => current.device_type == DeviceType::UltraWide,
        }
    }

    /// Get breakpoint for a specific width
    pub async fn get_breakpoint_for_width(&self, width: f64) -> Breakpoint {
        let device_type = self.determine_device_type(width).await;
        let (_, height) = self.get_viewport_size().await;
        self.calculate_breakpoint(width, height, device_type).await
    }

    /// Determine device type based on width
    async fn determine_device_type(&self, width: f64) -> DeviceType {
        if width < self.config.mobile_breakpoint {
            DeviceType::Mobile
        } else if width < self.config.tablet_breakpoint {
            DeviceType::Tablet
        } else if width < self.config.desktop_breakpoint {
            DeviceType::Desktop
        } else if width < self.config.large_desktop_breakpoint {
            DeviceType::LargeDesktop
        } else {
            DeviceType::UltraWide
        }
    }

    /// Calculate breakpoint based on dimensions and device type
    async fn calculate_breakpoint(
        &self,
        width: f64,
        height: f64,
        device_type: DeviceType,
    ) -> Breakpoint {
        let orientation = if width > height {
            Orientation::Landscape
        } else {
            Orientation::Portrait
        };

        match device_type {
            DeviceType::Mobile => Breakpoint {
                name: "mobile".to_string(),
                min_width: 0.0,
                max_width: Some(self.config.mobile_breakpoint),
                device_type,
                orientation,
            },
            DeviceType::Tablet => Breakpoint {
                name: "tablet".to_string(),
                min_width: self.config.mobile_breakpoint,
                max_width: Some(self.config.tablet_breakpoint),
                device_type,
                orientation,
            },
            DeviceType::Desktop => Breakpoint {
                name: "desktop".to_string(),
                min_width: self.config.tablet_breakpoint,
                max_width: Some(self.config.desktop_breakpoint),
                device_type,
                orientation,
            },
            DeviceType::LargeDesktop => Breakpoint {
                name: "large-desktop".to_string(),
                min_width: self.config.desktop_breakpoint,
                max_width: Some(self.config.large_desktop_breakpoint),
                device_type,
                orientation,
            },
            DeviceType::UltraWide => Breakpoint {
                name: "ultra-wide".to_string(),
                min_width: self.config.large_desktop_breakpoint,
                max_width: None,
                device_type,
                orientation,
            },
        }
    }

    /// Update breakpoint statistics
    async fn update_stats(&self, breakpoint: &Breakpoint, width: f64, height: f64) {
        let mut stats = self.stats.write().await;
        stats.total_breakpoint_changes += 1;
        stats.last_breakpoint_change = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );

        match breakpoint.device_type {
            DeviceType::Mobile => stats.mobile_views += 1,
            DeviceType::Tablet => stats.tablet_views += 1,
            DeviceType::Desktop => stats.desktop_views += 1,
            DeviceType::LargeDesktop => stats.large_desktop_views += 1,
            DeviceType::UltraWide => stats.ultra_wide_views += 1,
        }

        match breakpoint.orientation {
            Orientation::Portrait => stats.portrait_views += 1,
            Orientation::Landscape => stats.landscape_views += 1,
        }

        // Update average viewport dimensions
        let total_changes = stats.total_breakpoint_changes as f64;
        stats.average_viewport_width =
            (stats.average_viewport_width * (total_changes - 1.0) + width) / total_changes;
        stats.average_viewport_height =
            (stats.average_viewport_height * (total_changes - 1.0) + height) / total_changes;
    }

    /// Get default mobile breakpoint
    fn get_default_mobile_breakpoint() -> Breakpoint {
        Breakpoint {
            name: "mobile".to_string(),
            min_width: 0.0,
            max_width: Some(768.0),
            device_type: DeviceType::Mobile,
            orientation: Orientation::Portrait,
        }
    }
}

/// Breakpoint error types
#[derive(Debug, Error)]
pub enum BreakpointError {
    #[error("Invalid viewport size: width={width}, height={height}")]
    InvalidViewportSize { width: f64, height: f64 },

    #[error("Invalid breakpoint configuration: {message}")]
    InvalidConfiguration { message: String },

    #[error("Breakpoint calculation error: {message}")]
    CalculationError { message: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_breakpoint_config() -> BreakpointConfig {
        BreakpointConfig {
            mobile_breakpoint: 768.0,
            tablet_breakpoint: 1024.0,
            desktop_breakpoint: 1200.0,
            large_desktop_breakpoint: 1440.0,
            ultra_wide_breakpoint: 1920.0,
            orientation_detection: true,
            device_detection: true,
            custom_breakpoints: HashMap::new(),
        }
    }

    #[tokio::test]
    async fn test_breakpoint_manager_creation() {
        let config = create_test_breakpoint_config();
        let manager = BreakpointManager::new(config);

        let breakpoint = manager.get_current_breakpoint().await;
        assert_eq!(breakpoint.device_type, DeviceType::Mobile);
        assert_eq!(breakpoint.orientation, Orientation::Portrait);
    }

    #[tokio::test]
    async fn test_mobile_breakpoint() {
        let config = create_test_breakpoint_config();
        let manager = BreakpointManager::new(config);

        let breakpoint = manager.update_viewport_size(375.0, 667.0).await.unwrap();
        assert_eq!(breakpoint.device_type, DeviceType::Mobile);
        assert_eq!(breakpoint.orientation, Orientation::Portrait);
    }

    #[tokio::test]
    async fn test_tablet_breakpoint() {
        let config = create_test_breakpoint_config();
        let manager = BreakpointManager::new(config);

        let breakpoint = manager.update_viewport_size(768.0, 1024.0).await.unwrap();
        assert_eq!(breakpoint.device_type, DeviceType::Tablet);
        assert_eq!(breakpoint.orientation, Orientation::Portrait);
    }

    #[tokio::test]
    async fn test_desktop_breakpoint() {
        let config = create_test_breakpoint_config();
        let manager = BreakpointManager::new(config);

        let breakpoint = manager.update_viewport_size(1100.0, 800.0).await.unwrap();
        assert_eq!(breakpoint.device_type, DeviceType::Desktop);
        assert_eq!(breakpoint.orientation, Orientation::Landscape);
    }

    #[tokio::test]
    async fn test_orientation_detection() {
        let config = create_test_breakpoint_config();
        let manager = BreakpointManager::new(config);

        // Portrait
        let breakpoint = manager.update_viewport_size(375.0, 667.0).await.unwrap();
        assert_eq!(breakpoint.orientation, Orientation::Portrait);

        // Landscape
        let breakpoint = manager.update_viewport_size(667.0, 375.0).await.unwrap();
        assert_eq!(breakpoint.orientation, Orientation::Landscape);
    }

    #[tokio::test]
    async fn test_breakpoint_history() {
        let config = create_test_breakpoint_config();
        let manager = BreakpointManager::new(config);

        // Start with tablet to trigger the first change from default mobile
        manager.update_viewport_size(768.0, 1024.0).await.unwrap();
        manager.update_viewport_size(1100.0, 800.0).await.unwrap();
        manager.update_viewport_size(1300.0, 900.0).await.unwrap();

        let history = manager.get_breakpoint_history().await;
        assert_eq!(history.len(), 3);
        assert_eq!(history[0].device_type, DeviceType::Tablet);
        assert_eq!(history[1].device_type, DeviceType::Desktop);
        assert_eq!(history[2].device_type, DeviceType::LargeDesktop);
    }

    #[tokio::test]
    async fn test_breakpoint_stats() {
        let config = create_test_breakpoint_config();
        let manager = BreakpointManager::new(config);

        // Start with tablet to trigger the first change from default mobile
        manager.update_viewport_size(768.0, 1024.0).await.unwrap();
        manager.update_viewport_size(1100.0, 800.0).await.unwrap();
        manager.update_viewport_size(1300.0, 900.0).await.unwrap();

        let stats = manager.get_stats().await;
        assert_eq!(stats.total_breakpoint_changes, 3);
        assert_eq!(stats.tablet_views, 1);
        assert_eq!(stats.desktop_views, 1);
        assert_eq!(stats.large_desktop_views, 1);
    }

    #[tokio::test]
    async fn test_breakpoint_type_check() {
        let config = create_test_breakpoint_config();
        let manager = BreakpointManager::new(config);

        manager.update_viewport_size(375.0, 667.0).await.unwrap();
        assert!(manager.is_breakpoint(BreakpointType::Mobile).await);
        assert!(!manager.is_breakpoint(BreakpointType::Tablet).await);

        manager.update_viewport_size(768.0, 1024.0).await.unwrap();
        assert!(manager.is_breakpoint(BreakpointType::Tablet).await);
        assert!(!manager.is_breakpoint(BreakpointType::Mobile).await);
    }

    #[tokio::test]
    async fn test_breakpoint_for_width() {
        let config = create_test_breakpoint_config();
        let manager = BreakpointManager::new(config);

        let mobile_breakpoint = manager.get_breakpoint_for_width(375.0).await;
        assert_eq!(mobile_breakpoint.device_type, DeviceType::Mobile);

        let tablet_breakpoint = manager.get_breakpoint_for_width(768.0).await;
        assert_eq!(tablet_breakpoint.device_type, DeviceType::Tablet);

        let desktop_breakpoint = manager.get_breakpoint_for_width(1100.0).await;
        assert_eq!(desktop_breakpoint.device_type, DeviceType::Desktop);
    }

    #[tokio::test]
    async fn test_config_update() {
        let mut config = create_test_breakpoint_config();
        config.mobile_breakpoint = 600.0;

        let mut manager = BreakpointManager::new(config);

        // Update with new config
        let new_config = create_test_breakpoint_config();
        manager.update_config(new_config).await.unwrap();

        // Verify breakpoint is recalculated
        let breakpoint = manager.get_current_breakpoint().await;
        assert_eq!(breakpoint.device_type, DeviceType::Mobile);
    }

    #[tokio::test]
    async fn test_invalid_viewport_size() {
        let config = create_test_breakpoint_config();
        let manager = BreakpointManager::new(config);

        // Test with negative width
        let result = manager.update_viewport_size(-100.0, 600.0).await;
        assert!(result.is_err());

        // Test with zero height
        let result = manager.update_viewport_size(800.0, 0.0).await;
        assert!(result.is_err());
    }
}
