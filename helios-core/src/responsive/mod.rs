//! Responsive Design for leptos-helios
//!
//! This module provides responsive design capabilities for charts and visualizations,
//! including mobile-first layouts, adaptive sizing, responsive typography, and
//! device-specific optimizations.

pub mod adaptive_sizing;
pub mod breakpoints;
pub mod data_visualization;
pub mod layout;
pub mod navigation;
pub mod typography;

// Re-export main types for convenience
pub use breakpoints::{
    Breakpoint, BreakpointConfig, BreakpointManager, BreakpointType, DeviceType, Orientation,
};

pub use layout::{
    LayoutGrid, LayoutItem, LayoutManager, LayoutType, ResponsiveLayout, ResponsiveLayoutError,
    SpacingConfig,
};

pub use typography::{
    FontScale, ResponsiveTypography, TypographyConfig, TypographyManager, TypographyScale,
    TypographySize,
};

pub use navigation::{
    NavigationConfig, NavigationManager, NavigationType, ResponsiveNavigation,
    ResponsiveNavigationError,
};

pub use data_visualization::{
    AdaptiveChart, ChartResponsiveConfig, DataVisualizationManager, ResponsiveChart,
    VisualizationBreakpoint,
};

pub use adaptive_sizing::{
    AdaptiveSizing, AdaptiveSizingConfig, AdaptiveSizingManager, SizeConstraint,
    SizeConstraintType, SizingStrategy,
};

/// Main responsive design manager that coordinates all responsive features
pub struct ResponsiveDesignManager {
    breakpoint_manager: BreakpointManager,
    layout_manager: LayoutManager,
    typography_manager: TypographyManager,
    navigation_manager: NavigationManager,
    data_visualization_manager: DataVisualizationManager,
    adaptive_sizing_manager: AdaptiveSizingManager,
    config: ResponsiveConfig,
}

impl ResponsiveDesignManager {
    /// Create a new responsive design manager
    pub fn new(config: ResponsiveConfig) -> Self {
        Self {
            breakpoint_manager: BreakpointManager::new(config.breakpoints.clone()),
            layout_manager: LayoutManager::new(layout::LayoutConfig::default()),
            typography_manager: TypographyManager::new(config.typography.clone()),
            navigation_manager: NavigationManager::new(config.navigation.clone()),
            data_visualization_manager: DataVisualizationManager::new(
                config.data_visualization.clone(),
            ),
            adaptive_sizing_manager: AdaptiveSizingManager::new(config.adaptive_sizing.clone()),
            config,
        }
    }

    /// Get current breakpoint
    pub async fn get_current_breakpoint(&self) -> Breakpoint {
        self.breakpoint_manager.get_current_breakpoint().await
    }

    /// Update responsive configuration
    pub async fn update_config(
        &mut self,
        config: ResponsiveConfig,
    ) -> Result<(), ResponsiveDesignError> {
        self.config = config.clone();
        self.breakpoint_manager
            .update_config(config.breakpoints)
            .await?;
        self.layout_manager.update_config(config.layout).await?;
        self.typography_manager
            .update_config(config.typography)
            .await?;
        self.navigation_manager
            .update_config(config.navigation)
            .await?;
        self.data_visualization_manager
            .update_config(config.data_visualization)
            .await?;
        self.adaptive_sizing_manager
            .update_config(config.adaptive_sizing)
            .await?;
        Ok(())
    }

    /// Apply responsive design to a layout
    pub async fn apply_responsive_layout(
        &self,
        layout: &mut ResponsiveLayout,
    ) -> Result<(), ResponsiveDesignError> {
        let breakpoint = self.get_current_breakpoint().await;

        // Apply breakpoint-specific layout adjustments
        self.layout_manager
            .apply_breakpoint_layout(layout, &breakpoint)
            .await?;

        // Apply responsive typography
        self.typography_manager
            .apply_responsive_typography(layout, &breakpoint)
            .await?;

        // Apply adaptive sizing
        self.adaptive_sizing_manager
            .apply_adaptive_sizing(layout, &breakpoint)
            .await?;

        // Apply responsive navigation
        self.navigation_manager
            .apply_responsive_navigation(layout, &breakpoint)
            .await?;

        // Apply responsive data visualization
        self.data_visualization_manager
            .apply_responsive_visualization(layout, &breakpoint)
            .await?;

        Ok(())
    }

    /// Get responsive statistics
    pub async fn get_responsive_stats(&self) -> ResponsiveStats {
        ResponsiveStats {
            current_breakpoint: self.get_current_breakpoint().await,
            layout_stats: self.layout_manager.get_stats().await,
            typography_stats: self.typography_manager.get_stats().await,
            navigation_stats: self.navigation_manager.get_stats().await,
            data_visualization_stats: self.data_visualization_manager.get_stats().await,
            adaptive_sizing_stats: self.adaptive_sizing_manager.get_stats().await,
        }
    }
}

/// Responsive design error types
#[derive(Debug, thiserror::Error)]
pub enum ResponsiveDesignError {
    #[error("Layout error: {0}")]
    Layout(#[from] ResponsiveLayoutError),

    #[error("Navigation error: {0}")]
    Navigation(#[from] ResponsiveNavigationError),

    #[error("Breakpoint error: {0}")]
    Breakpoint(#[from] breakpoints::BreakpointError),

    #[error("Typography error: {0}")]
    Typography(#[from] typography::TypographyError),

    #[error("Data visualization error: {0}")]
    DataVisualization(#[from] data_visualization::DataVisualizationError),

    #[error("Adaptive sizing error: {0}")]
    AdaptiveSizing(#[from] adaptive_sizing::AdaptiveSizingError),

    #[error("Configuration error: {0}")]
    Configuration(String),
}

/// Responsive design statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResponsiveStats {
    pub current_breakpoint: Breakpoint,
    pub layout_stats: layout::LayoutStats,
    pub typography_stats: typography::TypographyStats,
    pub navigation_stats: navigation::NavigationStats,
    pub data_visualization_stats: data_visualization::DataVisualizationStats,
    pub adaptive_sizing_stats: adaptive_sizing::AdaptiveSizingStats,
}

// Statistics structs are defined in their respective modules

/// Main responsive configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResponsiveConfig {
    pub breakpoints: BreakpointConfig,
    pub layout: layout::LayoutConfig,
    pub typography: TypographyConfig,
    pub navigation: NavigationConfig,
    pub data_visualization: ChartResponsiveConfig,
    pub adaptive_sizing: AdaptiveSizingConfig,
}

// Layout configuration is defined in layout module
