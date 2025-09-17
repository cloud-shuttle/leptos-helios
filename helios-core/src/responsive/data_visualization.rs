//! Responsive Data Visualization for leptos-helios
//!
//! This module provides responsive data visualization management for charts and visualizations,
//! including adaptive chart sizing, responsive data display, and device-specific optimizations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use thiserror::Error;
use tokio::sync::RwLock;

use super::breakpoints::{Breakpoint, DeviceType, Orientation};

/// Chart types for responsive visualization
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ChartType {
    Line,
    Bar,
    Area,
    Scatter,
    Pie,
    Donut,
    Heatmap,
    Histogram,
    BoxPlot,
    Violin,
    Radar,
    Sankey,
    Treemap,
    Network,
    Map,
}

/// Visualization breakpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationBreakpoint {
    pub breakpoint_name: String,
    pub chart_type: ChartType,
    pub min_width: f64,
    pub max_width: Option<f64>,
    pub min_height: f64,
    pub max_height: Option<f64>,
    pub data_density: f64,
    pub animation_enabled: bool,
    pub interactive_features: bool,
}

/// Responsive chart configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsiveChart {
    pub id: String,
    pub chart_type: ChartType,
    pub data: Vec<serde_json::Value>,
    pub width: f64,
    pub height: f64,
    pub responsive: bool,
    pub breakpoint_configs: HashMap<String, VisualizationBreakpoint>,
    pub adaptive_sizing: bool,
    pub data_sampling: bool,
    pub animation_config: AnimationConfig,
}

/// Animation configuration for responsive charts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationConfig {
    pub enabled: bool,
    pub duration: Duration,
    pub easing: String,
    pub delay: Duration,
    pub mobile_optimized: bool,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            duration: Duration::from_millis(500),
            easing: "ease-in-out".to_string(),
            delay: Duration::from_millis(0),
            mobile_optimized: true,
        }
    }
}

/// Adaptive chart configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveChart {
    pub id: String,
    pub base_chart: ResponsiveChart,
    pub adaptive_breakpoints: Vec<VisualizationBreakpoint>,
    pub data_sampling_strategy: DataSamplingStrategy,
    pub performance_optimization: bool,
    pub memory_optimization: bool,
}

/// Data sampling strategy for responsive charts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataSamplingStrategy {
    None,
    Random,
    Systematic,
    Stratified,
    Adaptive,
}

/// Data visualization manager for responsive design
pub struct DataVisualizationManager {
    config: ChartResponsiveConfig,
    charts: Arc<RwLock<HashMap<String, ResponsiveChart>>>,
    adaptive_charts: Arc<RwLock<HashMap<String, AdaptiveChart>>>,
    stats: Arc<RwLock<DataVisualizationStats>>,
}

/// Chart responsive configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartResponsiveConfig {
    pub default_chart_type: ChartType,
    pub mobile_chart_type: ChartType,
    pub tablet_chart_type: ChartType,
    pub desktop_chart_type: ChartType,
    pub responsive_sizing: bool,
    pub data_sampling_enabled: bool,
    pub animation_enabled: bool,
    pub mobile_animation_disabled: bool,
    pub performance_optimization: bool,
    pub memory_optimization: bool,
    pub touch_interactions: bool,
    pub accessibility_enabled: bool,
}

impl Default for ChartResponsiveConfig {
    fn default() -> Self {
        Self {
            default_chart_type: ChartType::Line,
            mobile_chart_type: ChartType::Bar,
            tablet_chart_type: ChartType::Line,
            desktop_chart_type: ChartType::Line,
            responsive_sizing: true,
            data_sampling_enabled: true,
            animation_enabled: true,
            mobile_animation_disabled: true,
            performance_optimization: true,
            memory_optimization: true,
            touch_interactions: true,
            accessibility_enabled: true,
        }
    }
}

/// Data visualization statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataVisualizationStats {
    pub total_charts: usize,
    pub responsive_charts: usize,
    pub mobile_charts: usize,
    pub tablet_charts: usize,
    pub desktop_charts: usize,
    pub line_charts: usize,
    pub bar_charts: usize,
    pub area_charts: usize,
    pub scatter_charts: usize,
    pub pie_charts: usize,
    pub adaptive_charts: usize,
    pub data_sampling_applications: usize,
    pub performance_optimizations: usize,
    pub memory_optimizations: usize,
}

impl Default for DataVisualizationStats {
    fn default() -> Self {
        Self {
            total_charts: 0,
            responsive_charts: 0,
            mobile_charts: 0,
            tablet_charts: 0,
            desktop_charts: 0,
            line_charts: 0,
            bar_charts: 0,
            area_charts: 0,
            scatter_charts: 0,
            pie_charts: 0,
            adaptive_charts: 0,
            data_sampling_applications: 0,
            performance_optimizations: 0,
            memory_optimizations: 0,
        }
    }
}

impl DataVisualizationManager {
    /// Create a new data visualization manager
    pub fn new(config: ChartResponsiveConfig) -> Self {
        Self {
            config,
            charts: Arc::new(RwLock::new(HashMap::new())),
            adaptive_charts: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(DataVisualizationStats::default())),
        }
    }

    /// Create a new responsive chart
    pub async fn create_responsive_chart(
        &self,
        id: String,
        chart_type: ChartType,
        width: f64,
        height: f64,
    ) -> Result<ResponsiveChart, DataVisualizationError> {
        let chart = ResponsiveChart {
            id: id.clone(),
            chart_type,
            data: Vec::new(),
            width,
            height,
            responsive: true,
            breakpoint_configs: HashMap::new(),
            adaptive_sizing: self.config.responsive_sizing,
            data_sampling: self.config.data_sampling_enabled,
            animation_config: AnimationConfig::default(),
        };

        // Store chart
        {
            let mut charts = self.charts.write().await;
            charts.insert(id, chart.clone());
        }

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.total_charts += 1;
            stats.responsive_charts += 1;

            match chart_type {
                ChartType::Line => stats.line_charts += 1,
                ChartType::Bar => stats.bar_charts += 1,
                ChartType::Area => stats.area_charts += 1,
                ChartType::Scatter => stats.scatter_charts += 1,
                ChartType::Pie => stats.pie_charts += 1,
                _ => {}
            }
        }

        Ok(chart)
    }

    /// Create an adaptive chart
    pub async fn create_adaptive_chart(
        &self,
        id: String,
        base_chart: ResponsiveChart,
        sampling_strategy: DataSamplingStrategy,
    ) -> Result<AdaptiveChart, DataVisualizationError> {
        let adaptive_chart = AdaptiveChart {
            id: id.clone(),
            base_chart: base_chart.clone(),
            adaptive_breakpoints: Vec::new(),
            data_sampling_strategy: sampling_strategy,
            performance_optimization: self.config.performance_optimization,
            memory_optimization: self.config.memory_optimization,
        };

        // Store adaptive chart
        {
            let mut adaptive_charts = self.adaptive_charts.write().await;
            adaptive_charts.insert(id, adaptive_chart.clone());
        }

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.adaptive_charts += 1;
        }

        Ok(adaptive_chart)
    }

    /// Apply responsive visualization to a layout
    pub async fn apply_responsive_visualization(
        &self,
        layout: &mut super::layout::ResponsiveLayout,
        breakpoint: &Breakpoint,
    ) -> Result<(), DataVisualizationError> {
        // Determine chart type for breakpoint
        let chart_type = self.get_chart_type_for_breakpoint(breakpoint).await;

        // Apply visualization adjustments to layout items
        for item in &mut layout.items {
            if let Some(chart_id) = item.responsive_properties.get("chart_id") {
                if let Some(chart_id_str) = chart_id.as_str() {
                    if let Some(chart) = self.get_responsive_chart(chart_id_str).await {
                        let responsive_chart = self
                            .adapt_chart_for_breakpoint(&chart, breakpoint, chart_type)
                            .await;

                        // Apply chart properties to item
                        self.apply_chart_to_item(item, &responsive_chart, breakpoint)
                            .await?;
                    }
                }
            }
        }

        // Update stats
        {
            let mut stats = self.stats.write().await;

            match breakpoint.device_type {
                DeviceType::Mobile => stats.mobile_charts += 1,
                DeviceType::Tablet => stats.tablet_charts += 1,
                DeviceType::Desktop => stats.desktop_charts += 1,
                _ => {}
            }
        }

        Ok(())
    }

    /// Get chart type for breakpoint
    async fn get_chart_type_for_breakpoint(&self, breakpoint: &Breakpoint) -> ChartType {
        match breakpoint.device_type {
            DeviceType::Mobile => self.config.mobile_chart_type,
            DeviceType::Tablet => self.config.tablet_chart_type,
            DeviceType::Desktop => self.config.desktop_chart_type,
            _ => self.config.default_chart_type,
        }
    }

    /// Adapt chart for breakpoint
    async fn adapt_chart_for_breakpoint(
        &self,
        chart: &ResponsiveChart,
        breakpoint: &Breakpoint,
        chart_type: ChartType,
    ) -> ResponsiveChart {
        let mut adapted_chart = chart.clone();

        // Apply breakpoint-specific configuration
        if let Some(breakpoint_config) = chart.breakpoint_configs.get(breakpoint.name.as_str()) {
            adapted_chart.chart_type = breakpoint_config.chart_type;
            adapted_chart.width = breakpoint_config.min_width;
            adapted_chart.height = breakpoint_config.min_height;
            adapted_chart.animation_config.enabled = breakpoint_config.animation_enabled;
        } else {
            // Apply default chart type for breakpoint
            adapted_chart.chart_type = chart_type;
        }

        // Apply mobile-specific optimizations
        if breakpoint.device_type == DeviceType::Mobile {
            adapted_chart.animation_config.enabled = !self.config.mobile_animation_disabled;
            adapted_chart.animation_config.mobile_optimized = true;
            adapted_chart.data_sampling = true;
        }

        // Apply performance optimizations
        if self.config.performance_optimization {
            adapted_chart = self
                .optimize_chart_performance(adapted_chart, breakpoint)
                .await;
        }

        // Apply memory optimizations
        if self.config.memory_optimization {
            adapted_chart = self.optimize_chart_memory(adapted_chart, breakpoint).await;
        }

        adapted_chart
    }

    /// Optimize chart performance
    async fn optimize_chart_performance(
        &self,
        mut chart: ResponsiveChart,
        breakpoint: &Breakpoint,
    ) -> ResponsiveChart {
        // Reduce animation duration for mobile
        if breakpoint.device_type == DeviceType::Mobile {
            chart.animation_config.duration = Duration::from_millis(250);
        }

        // Enable data sampling for large datasets
        if chart.data.len() > 1000 {
            chart.data_sampling = true;
        }

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.performance_optimizations += 1;
        }

        chart
    }

    /// Optimize chart memory usage
    async fn optimize_chart_memory(
        &self,
        mut chart: ResponsiveChart,
        breakpoint: &Breakpoint,
    ) -> ResponsiveChart {
        // Limit data points for mobile devices
        if breakpoint.device_type == DeviceType::Mobile && chart.data.len() > 500 {
            chart.data = chart.data.into_iter().take(500).collect();
        }

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.memory_optimizations += 1;
        }

        chart
    }

    /// Apply chart to layout item
    async fn apply_chart_to_item(
        &self,
        item: &mut super::layout::LayoutItem,
        chart: &ResponsiveChart,
        breakpoint: &Breakpoint,
    ) -> Result<(), DataVisualizationError> {
        // Apply chart type
        item.responsive_properties.insert(
            "chart_type".to_string(),
            serde_json::Value::String(format!("{:?}", chart.chart_type)),
        );

        // Apply chart dimensions
        item.width = Some(chart.width);
        item.height = Some(chart.height);

        // Apply animation configuration
        item.responsive_properties.insert(
            "animation_enabled".to_string(),
            serde_json::Value::Bool(chart.animation_config.enabled),
        );
        item.responsive_properties.insert(
            "animation_duration".to_string(),
            serde_json::Value::Number(
                serde_json::Number::from_f64(chart.animation_config.duration.as_millis() as f64)
                    .unwrap(),
            ),
        );

        // Apply data sampling
        if chart.data_sampling {
            item.responsive_properties
                .insert("data_sampling".to_string(), serde_json::Value::Bool(true));

            // Update stats
            {
                let mut stats = self.stats.write().await;
                stats.data_sampling_applications += 1;
            }
        }

        // Apply touch interactions for mobile
        if breakpoint.device_type == DeviceType::Mobile && self.config.touch_interactions {
            item.responsive_properties.insert(
                "touch_interactions".to_string(),
                serde_json::Value::Bool(true),
            );
        }

        // Apply accessibility features
        if self.config.accessibility_enabled {
            item.responsive_properties.insert(
                "accessibility_enabled".to_string(),
                serde_json::Value::Bool(true),
            );
            item.responsive_properties.insert(
                "aria_label".to_string(),
                serde_json::Value::String("Data visualization chart".to_string()),
            );
        }

        Ok(())
    }

    /// Get responsive chart by ID
    pub async fn get_responsive_chart(&self, id: &str) -> Option<ResponsiveChart> {
        let charts = self.charts.read().await;
        charts.get(id).cloned()
    }

    /// Get adaptive chart by ID
    pub async fn get_adaptive_chart(&self, id: &str) -> Option<AdaptiveChart> {
        let adaptive_charts = self.adaptive_charts.read().await;
        adaptive_charts.get(id).cloned()
    }

    /// Update responsive chart
    pub async fn update_responsive_chart(
        &self,
        id: &str,
        chart: ResponsiveChart,
    ) -> Result<(), DataVisualizationError> {
        let mut charts = self.charts.write().await;
        charts.insert(id.to_string(), chart);
        Ok(())
    }

    /// Update adaptive chart
    pub async fn update_adaptive_chart(
        &self,
        id: &str,
        chart: AdaptiveChart,
    ) -> Result<(), DataVisualizationError> {
        let mut adaptive_charts = self.adaptive_charts.write().await;
        adaptive_charts.insert(id.to_string(), chart);
        Ok(())
    }

    /// Update chart responsive configuration
    pub async fn update_config(
        &mut self,
        config: ChartResponsiveConfig,
    ) -> Result<(), DataVisualizationError> {
        self.config = config;
        Ok(())
    }

    /// Get data visualization statistics
    pub async fn get_stats(&self) -> DataVisualizationStats {
        self.stats.read().await.clone()
    }

    /// Remove responsive chart
    pub async fn remove_responsive_chart(&self, id: &str) -> Result<(), DataVisualizationError> {
        let mut charts = self.charts.write().await;
        charts.remove(id);
        Ok(())
    }

    /// Remove adaptive chart
    pub async fn remove_adaptive_chart(&self, id: &str) -> Result<(), DataVisualizationError> {
        let mut adaptive_charts = self.adaptive_charts.write().await;
        adaptive_charts.remove(id);
        Ok(())
    }

    /// Get all responsive charts
    pub async fn get_all_responsive_charts(&self) -> HashMap<String, ResponsiveChart> {
        self.charts.read().await.clone()
    }

    /// Get all adaptive charts
    pub async fn get_all_adaptive_charts(&self) -> HashMap<String, AdaptiveChart> {
        self.adaptive_charts.read().await.clone()
    }
}

/// Data visualization error types
#[derive(Debug, Error)]
pub enum DataVisualizationError {
    #[error("Chart not found: {id}")]
    ChartNotFound { id: String },

    #[error("Invalid chart configuration: {message}")]
    InvalidConfiguration { message: String },

    #[error("Chart adaptation error: {message}")]
    ChartAdaptationError { message: String },

    #[error("Performance optimization error: {message}")]
    PerformanceOptimizationError { message: String },

    #[error("Memory optimization error: {message}")]
    MemoryOptimizationError { message: String },

    #[error("Data sampling error: {message}")]
    DataSamplingError { message: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_chart_config() -> ChartResponsiveConfig {
        ChartResponsiveConfig {
            default_chart_type: ChartType::Line,
            mobile_chart_type: ChartType::Bar,
            tablet_chart_type: ChartType::Line,
            desktop_chart_type: ChartType::Line,
            responsive_sizing: true,
            data_sampling_enabled: true,
            animation_enabled: true,
            mobile_animation_disabled: true,
            performance_optimization: true,
            memory_optimization: true,
            touch_interactions: true,
            accessibility_enabled: true,
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
    async fn test_data_visualization_manager_creation() {
        let config = create_test_chart_config();
        let manager = DataVisualizationManager::new(config);

        let stats = manager.get_stats().await;
        assert_eq!(stats.total_charts, 0);
    }

    #[tokio::test]
    async fn test_create_responsive_chart() {
        let config = create_test_chart_config();
        let manager = DataVisualizationManager::new(config);

        let chart = manager
            .create_responsive_chart("test-chart".to_string(), ChartType::Line, 800.0, 600.0)
            .await
            .unwrap();

        assert_eq!(chart.id, "test-chart");
        assert_eq!(chart.chart_type, ChartType::Line);
        assert_eq!(chart.width, 800.0);
        assert_eq!(chart.height, 600.0);
        assert!(chart.responsive);
        assert!(chart.adaptive_sizing);

        let stats = manager.get_stats().await;
        assert_eq!(stats.total_charts, 1);
        assert_eq!(stats.responsive_charts, 1);
        assert_eq!(stats.line_charts, 1);
    }

    #[tokio::test]
    async fn test_create_adaptive_chart() {
        let config = create_test_chart_config();
        let manager = DataVisualizationManager::new(config);

        let base_chart = manager
            .create_responsive_chart("base-chart".to_string(), ChartType::Line, 800.0, 600.0)
            .await
            .unwrap();
        let adaptive_chart = manager
            .create_adaptive_chart(
                "adaptive-chart".to_string(),
                base_chart,
                DataSamplingStrategy::Adaptive,
            )
            .await
            .unwrap();

        assert_eq!(adaptive_chart.id, "adaptive-chart");
        assert_eq!(
            adaptive_chart.data_sampling_strategy,
            DataSamplingStrategy::Adaptive
        );
        assert!(adaptive_chart.performance_optimization);

        let stats = manager.get_stats().await;
        assert_eq!(stats.adaptive_charts, 1);
    }

    #[tokio::test]
    async fn test_chart_type_for_breakpoint() {
        let config = create_test_chart_config();
        let manager = DataVisualizationManager::new(config);

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
            .get_chart_type_for_breakpoint(&mobile_breakpoint)
            .await;
        let desktop_type = manager
            .get_chart_type_for_breakpoint(&desktop_breakpoint)
            .await;

        assert_eq!(mobile_type, ChartType::Bar);
        assert_eq!(desktop_type, ChartType::Line);
    }

    #[tokio::test]
    async fn test_adapt_chart_for_breakpoint() {
        let config = create_test_chart_config();
        let manager = DataVisualizationManager::new(config);

        let chart = manager
            .create_responsive_chart("test-chart".to_string(), ChartType::Line, 800.0, 600.0)
            .await
            .unwrap();
        let mobile_breakpoint = create_test_breakpoint();

        let adapted_chart = manager
            .adapt_chart_for_breakpoint(&chart, &mobile_breakpoint, ChartType::Bar)
            .await;

        assert_eq!(adapted_chart.chart_type, ChartType::Bar);
        assert!(!adapted_chart.animation_config.enabled); // Mobile animations disabled
        assert!(adapted_chart.animation_config.mobile_optimized);
        assert!(adapted_chart.data_sampling);
    }

    #[tokio::test]
    async fn test_optimize_chart_performance() {
        let config = create_test_chart_config();
        let manager = DataVisualizationManager::new(config);

        let mut chart = manager
            .create_responsive_chart("test-chart".to_string(), ChartType::Line, 800.0, 600.0)
            .await
            .unwrap();
        chart.data = vec![serde_json::Value::Null; 1500]; // Large dataset

        let mobile_breakpoint = create_test_breakpoint();
        let optimized_chart = manager
            .optimize_chart_performance(chart, &mobile_breakpoint)
            .await;

        assert_eq!(
            optimized_chart.animation_config.duration,
            Duration::from_millis(250)
        );
        assert!(optimized_chart.data_sampling);

        let stats = manager.get_stats().await;
        assert_eq!(stats.performance_optimizations, 1);
    }

    #[tokio::test]
    async fn test_optimize_chart_memory() {
        let config = create_test_chart_config();
        let manager = DataVisualizationManager::new(config);

        let mut chart = manager
            .create_responsive_chart("test-chart".to_string(), ChartType::Line, 800.0, 600.0)
            .await
            .unwrap();
        chart.data = vec![serde_json::Value::Null; 1000]; // Large dataset

        let mobile_breakpoint = create_test_breakpoint();
        let optimized_chart = manager
            .optimize_chart_memory(chart, &mobile_breakpoint)
            .await;

        assert_eq!(optimized_chart.data.len(), 500); // Limited for mobile

        let stats = manager.get_stats().await;
        assert_eq!(stats.memory_optimizations, 1);
    }

    #[tokio::test]
    async fn test_get_responsive_chart() {
        let config = create_test_chart_config();
        let manager = DataVisualizationManager::new(config);

        manager
            .create_responsive_chart("test-chart".to_string(), ChartType::Line, 800.0, 600.0)
            .await
            .unwrap();

        let chart = manager.get_responsive_chart("test-chart").await;
        assert!(chart.is_some());
        assert_eq!(chart.unwrap().id, "test-chart");
    }

    #[tokio::test]
    async fn test_remove_responsive_chart() {
        let config = create_test_chart_config();
        let manager = DataVisualizationManager::new(config);

        manager
            .create_responsive_chart("test-chart".to_string(), ChartType::Line, 800.0, 600.0)
            .await
            .unwrap();

        let chart = manager.get_responsive_chart("test-chart").await;
        assert!(chart.is_some());

        manager.remove_responsive_chart("test-chart").await.unwrap();

        let chart = manager.get_responsive_chart("test-chart").await;
        assert!(chart.is_none());
    }

    #[tokio::test]
    async fn test_config_update() {
        let mut config = create_test_chart_config();
        config.mobile_chart_type = ChartType::Pie;

        let mut manager = DataVisualizationManager::new(config);

        let new_config = create_test_chart_config();
        manager.update_config(new_config).await.unwrap();

        // Verify config was updated
        let mobile_breakpoint = create_test_breakpoint();
        let chart_type = manager
            .get_chart_type_for_breakpoint(&mobile_breakpoint)
            .await;
        assert_eq!(chart_type, ChartType::Bar); // Should use new config
    }

    #[tokio::test]
    async fn test_data_visualization_stats() {
        let config = create_test_chart_config();
        let manager = DataVisualizationManager::new(config);

        manager
            .create_responsive_chart("test-chart".to_string(), ChartType::Line, 800.0, 600.0)
            .await
            .unwrap();

        let stats = manager.get_stats().await;
        assert_eq!(stats.total_charts, 1);
        assert_eq!(stats.responsive_charts, 1);
        assert_eq!(stats.line_charts, 1);
    }
}
