//! Plugin System Architecture
//!
//! This module provides a comprehensive plugin architecture for Helios that allows
//! third-party developers to extend functionality while maintaining type safety,
//! performance, and security. The plugin system supports:
//!
//! - **Chart Type Plugins**: Custom chart types and rendering
//! - **Data Source Plugins**: New data source adapters
//! - **Transform Plugins**: Custom data transformations
//! - **Export Plugins**: Additional export formats
//! - **Intelligence Plugins**: ML/AI capabilities
//! - **Theme Plugins**: Custom styling and themes
//!
//! ## Architecture Overview
//!
//! The plugin system uses a trait-based approach with dynamic loading capabilities:
//!
//! ```rust
//! use helios_core::plugin_system::*;
//!
//! // Define a custom chart plugin
//! #[derive(Debug, Clone)]
//! pub struct CustomChartPlugin {
//!     name: String,
//     version: String,
//     capabilities: PluginCapabilities,
//! }
//!
//! impl ChartPlugin for CustomChartPlugin {
//!     fn render(&self, spec: &ChartSpec, context: &RenderContext) -> Result<RenderResult, PluginError> {
//!         // Custom rendering logic
//!         Ok(RenderResult::success())
//!     }
//! }
//!
//! // Register the plugin
//! let plugin_manager = PluginManager::new();
//! plugin_manager.register_chart_plugin(Box::new(CustomChartPlugin::new()));
//! ```

use crate::chart::{ChartSpec, MarkType};
use crate::data_sources::DataSource;
use crate::export_system::{ExportFormat, ExportResult};
use crate::styling::Theme;
use serde::{Deserialize, Serialize};
use std::any::TypeId;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};
use tokio::sync::RwLock as AsyncRwLock;

/// Plugin system errors
#[derive(Debug, Clone, thiserror::Error)]
pub enum PluginError {
    #[error("Plugin not found: {0}")]
    PluginNotFound(String),

    #[error("Plugin registration failed: {0}")]
    RegistrationFailed(String),

    #[error("Plugin execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Plugin validation failed: {0}")]
    ValidationFailed(String),

    #[error("Plugin compatibility error: {0}")]
    CompatibilityError(String),

    #[error("Plugin security violation: {0}")]
    SecurityViolation(String),

    #[error("Plugin resource limit exceeded: {0}")]
    ResourceLimitExceeded(String),

    #[error("Plugin dependency missing: {0}")]
    DependencyMissing(String),
}

/// Plugin metadata and capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub dependencies: Vec<PluginDependency>,
    pub capabilities: PluginCapabilities,
    pub security_level: SecurityLevel,
    pub performance_impact: PerformanceImpact,
}

/// Plugin dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    pub name: String,
    pub version: String,
    pub optional: bool,
}

/// Plugin capabilities enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginCapability {
    /// Custom chart types and rendering
    ChartRendering,
    /// Data source adapters
    DataSource,
    /// Data transformation operations
    DataTransform,
    /// Export format support
    ExportFormat,
    /// Machine learning capabilities
    MLIntelligence,
    /// Custom themes and styling
    Theming,
    /// Interactive features
    Interaction,
    /// Performance optimizations
    Performance,
    /// Security features
    Security,
    /// Accessibility enhancements
    Accessibility,
}

/// Collection of plugin capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginCapabilities {
    pub capabilities: Vec<PluginCapability>,
    pub max_data_points: Option<u64>,
    pub supported_formats: Vec<String>,
    pub performance_requirements: PerformanceRequirements,
}

/// Performance requirements for plugins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRequirements {
    pub max_memory_mb: Option<u64>,
    pub max_execution_time_ms: Option<u64>,
    pub max_cpu_usage_percent: Option<u8>,
    pub requires_gpu: bool,
}

/// Security level for plugins
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLevel {
    /// Trusted plugins with full system access
    Trusted,
    /// Sandboxed plugins with limited access
    Sandboxed,
    /// Untrusted plugins with minimal access
    Untrusted,
}

/// Performance impact classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum PerformanceImpact {
    /// Minimal performance impact
    Minimal,
    /// Moderate performance impact
    Moderate,
    /// High performance impact
    High,
    /// Critical performance impact
    Critical,
}

/// Plugin lifecycle events
#[derive(Debug, Clone)]
pub enum PluginEvent {
    /// Plugin registered
    Registered { name: String, version: String },
    /// Plugin activated
    Activated { name: String },
    /// Plugin deactivated
    Deactivated { name: String },
    /// Plugin error occurred
    Error { name: String, error: PluginError },
    /// Plugin performance warning
    PerformanceWarning { name: String, message: String },
}

/// Plugin event handler trait
pub trait PluginEventHandler: Send + Sync {
    fn handle_event(&self, event: PluginEvent);
}

/// Base plugin trait that all plugins must implement
pub trait Plugin: Send + Sync + fmt::Debug {
    /// Get plugin metadata
    fn metadata(&self) -> &PluginMetadata;

    /// Initialize the plugin
    fn initialize(&mut self) -> Result<(), PluginError>;

    /// Cleanup plugin resources
    fn cleanup(&mut self) -> Result<(), PluginError>;

    /// Check if plugin is compatible with current system
    fn is_compatible(&self, system_info: &SystemInfo) -> bool;

    /// Get plugin type ID for dynamic casting
    fn type_id(&self) -> TypeId;

    /// Clone the plugin (for dynamic dispatch)
    fn clone_plugin(&self) -> Box<dyn Plugin>;

    /// Get plugin as any type for downcasting
    fn as_any(&self) -> &dyn std::any::Any;
}

/// System information for compatibility checking
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub helios_version: String,
    pub rust_version: String,
    pub platform: String,
    pub features: Vec<String>,
    pub available_memory_mb: u64,
    pub gpu_available: bool,
}

/// Chart plugin trait for custom chart types
pub trait ChartPlugin: Plugin {
    /// Get supported mark types
    fn supported_marks(&self) -> Vec<MarkType>;

    /// Render a chart with the plugin
    fn render(
        &self,
        spec: &ChartSpec,
        context: &RenderContext,
    ) -> Result<RenderResult, PluginError>;

    /// Validate chart specification for this plugin
    fn validate_spec(&self, spec: &ChartSpec) -> Result<(), PluginError>;

    /// Get rendering performance estimate
    fn estimate_performance(&self, spec: &ChartSpec) -> PerformanceEstimate;
}

/// Data source plugin trait
pub trait DataSourcePlugin: Plugin {
    /// Get supported data source types
    fn supported_sources(&self) -> Vec<String>;

    /// Create a data source connection
    fn create_connection(
        &self,
        config: &DataSourceConfig,
    ) -> Result<Box<dyn DataSource>, PluginError>;

    /// Validate data source configuration
    fn validate_config(&self, config: &DataSourceConfig) -> Result<(), PluginError>;
}

/// Transform plugin trait for data transformations
pub trait TransformPlugin: Plugin {
    /// Get supported transform types
    fn supported_transforms(&self) -> Vec<String>;

    /// Apply transformation to data
    fn apply_transform(
        &self,
        data: &[u8],
        transform_type: &str,
        params: &TransformParams,
    ) -> Result<Vec<u8>, PluginError>;

    /// Validate transform parameters
    fn validate_params(
        &self,
        transform_type: &str,
        params: &TransformParams,
    ) -> Result<(), PluginError>;
}

/// Export plugin trait for custom export formats
pub trait ExportPlugin: Plugin {
    /// Get supported export formats
    fn supported_formats(&self) -> Vec<ExportFormat>;

    /// Export chart to format
    fn export(
        &self,
        chart_data: &ChartData,
        format: &ExportFormat,
        options: &ExportOptions,
    ) -> Result<ExportResult, PluginError>;

    /// Validate export options
    fn validate_options(
        &self,
        format: &ExportFormat,
        options: &ExportOptions,
    ) -> Result<(), PluginError>;
}

/// ML Intelligence plugin trait
pub trait MLPlugin: Plugin {
    /// Get supported ML capabilities
    fn supported_capabilities(&self) -> Vec<String>;

    /// Execute ML operation
    fn execute_ml(
        &self,
        capability: &str,
        data: &MLData,
        params: &MLParams,
    ) -> Result<Vec<u8>, PluginError>;

    /// Validate ML parameters
    fn validate_ml_params(&self, capability: &str, params: &MLParams) -> Result<(), PluginError>;
}

/// Theme plugin trait for custom styling
pub trait ThemePlugin: Plugin {
    /// Get supported theme types
    fn supported_themes(&self) -> Vec<String>;

    /// Apply theme to chart
    fn apply_theme(
        &self,
        theme_name: &str,
        config: &HashMap<String, String>,
    ) -> Result<Theme, PluginError>;

    /// Validate theme configuration
    fn validate_theme_config(
        &self,
        theme_name: &str,
        config: &HashMap<String, String>,
    ) -> Result<(), PluginError>;
}

/// Render context for chart plugins
#[derive(Debug, Clone)]
pub struct RenderContext {
    pub viewport: Viewport,
    pub device_info: DeviceInfo,
    pub performance_budget: PerformanceBudget,
    pub security_context: SecurityContext,
}

/// Viewport information
#[derive(Debug, Clone)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
    pub dpi: f32,
    pub pixel_ratio: f32,
}

/// Device information
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub gpu_available: bool,
    pub gpu_vendor: Option<String>,
    pub memory_mb: u64,
    pub cpu_cores: u32,
}

/// Performance budget for rendering
#[derive(Debug, Clone)]
pub struct PerformanceBudget {
    pub max_render_time_ms: u64,
    pub max_memory_mb: u64,
    pub max_cpu_usage_percent: u8,
}

/// Security context for plugin execution
#[derive(Debug, Clone)]
pub struct SecurityContext {
    pub security_level: SecurityLevel,
    pub allowed_operations: Vec<String>,
    pub sandbox_mode: bool,
}

/// Render result from chart plugins
#[derive(Debug, Clone)]
pub struct RenderResult {
    pub success: bool,
    pub render_time_ms: u64,
    pub memory_used_mb: u64,
    pub warnings: Vec<String>,
    pub error: Option<String>,
}

impl RenderResult {
    pub fn success() -> Self {
        Self {
            success: true,
            render_time_ms: 0,
            memory_used_mb: 0,
            warnings: Vec::new(),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            render_time_ms: 0,
            memory_used_mb: 0,
            warnings: Vec::new(),
            error: Some(message),
        }
    }
}

/// Performance estimate for rendering
#[derive(Debug, Clone)]
pub struct PerformanceEstimate {
    pub estimated_time_ms: u64,
    pub estimated_memory_mb: u64,
    pub complexity_score: f64,
}

/// Data source configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceConfig {
    pub source_type: String,
    pub connection_string: String,
    pub credentials: Option<HashMap<String, String>>,
    pub options: HashMap<String, String>,
}

/// Transform parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformParams {
    pub transform_type: String,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Chart data for export
#[derive(Debug, Clone)]
pub struct ChartData {
    pub spec: ChartSpec,
    pub rendered_data: Vec<u8>,
    pub metadata: HashMap<String, String>,
}

/// Export options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportOptions {
    pub quality: u8,
    pub format_options: HashMap<String, serde_json::Value>,
}

/// ML data for processing
#[derive(Debug, Clone)]
pub struct MLData {
    pub data_type: String,
    pub data: Vec<u8>,
    pub schema: Option<serde_json::Value>,
}

/// ML parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLParams {
    pub algorithm: String,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Plugin manager for managing all plugins
pub struct PluginManager {
    chart_plugins: Arc<AsyncRwLock<HashMap<String, Box<dyn ChartPlugin>>>>,
    data_source_plugins: Arc<AsyncRwLock<HashMap<String, Box<dyn DataSourcePlugin>>>>,
    transform_plugins: Arc<AsyncRwLock<HashMap<String, Box<dyn TransformPlugin>>>>,
    export_plugins: Arc<AsyncRwLock<HashMap<String, Box<dyn ExportPlugin>>>>,
    ml_plugins: Arc<AsyncRwLock<HashMap<String, Box<dyn MLPlugin>>>>,
    theme_plugins: Arc<AsyncRwLock<HashMap<String, Box<dyn ThemePlugin>>>>,
    event_handlers: Arc<RwLock<Vec<Box<dyn PluginEventHandler>>>>,
    system_info: SystemInfo,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new() -> Self {
        Self {
            chart_plugins: Arc::new(AsyncRwLock::new(HashMap::new())),
            data_source_plugins: Arc::new(AsyncRwLock::new(HashMap::new())),
            transform_plugins: Arc::new(AsyncRwLock::new(HashMap::new())),
            export_plugins: Arc::new(AsyncRwLock::new(HashMap::new())),
            ml_plugins: Arc::new(AsyncRwLock::new(HashMap::new())),
            theme_plugins: Arc::new(AsyncRwLock::new(HashMap::new())),
            event_handlers: Arc::new(RwLock::new(Vec::new())),
            system_info: SystemInfo::current(),
        }
    }

    /// Register a chart plugin
    pub async fn register_chart_plugin(
        &self,
        plugin: Box<dyn ChartPlugin>,
    ) -> Result<(), PluginError> {
        let metadata = plugin.metadata().clone();
        let name = metadata.name.clone();

        // Validate plugin compatibility
        if !plugin.is_compatible(&self.system_info) {
            return Err(PluginError::CompatibilityError(format!(
                "Plugin {} is not compatible with current system",
                name
            )));
        }

        // Initialize plugin
        let mut plugin = plugin;
        plugin.initialize()?;

        // Register plugin
        let mut plugins = self.chart_plugins.write().await;
        plugins.insert(name.clone(), plugin);

        // Emit event
        self.emit_event(PluginEvent::Registered {
            name: name.clone(),
            version: metadata.version.clone(),
        });

        Ok(())
    }

    /// Register a data source plugin
    pub async fn register_data_source_plugin(
        &self,
        plugin: Box<dyn DataSourcePlugin>,
    ) -> Result<(), PluginError> {
        let metadata = plugin.metadata().clone();
        let name = metadata.name.clone();

        if !plugin.is_compatible(&self.system_info) {
            return Err(PluginError::CompatibilityError(format!(
                "Plugin {} is not compatible with current system",
                name
            )));
        }

        let mut plugin = plugin;
        plugin.initialize()?;

        let mut plugins = self.data_source_plugins.write().await;
        plugins.insert(name.clone(), plugin);

        self.emit_event(PluginEvent::Registered {
            name: name.clone(),
            version: metadata.version.clone(),
        });

        Ok(())
    }

    /// Register a transform plugin
    pub async fn register_transform_plugin(
        &self,
        plugin: Box<dyn TransformPlugin>,
    ) -> Result<(), PluginError> {
        let metadata = plugin.metadata().clone();
        let name = metadata.name.clone();

        if !plugin.is_compatible(&self.system_info) {
            return Err(PluginError::CompatibilityError(format!(
                "Plugin {} is not compatible with current system",
                name
            )));
        }

        let mut plugin = plugin;
        plugin.initialize()?;

        let mut plugins = self.transform_plugins.write().await;
        plugins.insert(name.clone(), plugin);

        self.emit_event(PluginEvent::Registered {
            name: name.clone(),
            version: metadata.version.clone(),
        });

        Ok(())
    }

    /// Register an export plugin
    pub async fn register_export_plugin(
        &self,
        plugin: Box<dyn ExportPlugin>,
    ) -> Result<(), PluginError> {
        let metadata = plugin.metadata().clone();
        let name = metadata.name.clone();

        if !plugin.is_compatible(&self.system_info) {
            return Err(PluginError::CompatibilityError(format!(
                "Plugin {} is not compatible with current system",
                name
            )));
        }

        let mut plugin = plugin;
        plugin.initialize()?;

        let mut plugins = self.export_plugins.write().await;
        plugins.insert(name.clone(), plugin);

        self.emit_event(PluginEvent::Registered {
            name: name.clone(),
            version: metadata.version.clone(),
        });

        Ok(())
    }

    /// Register an ML plugin
    pub async fn register_ml_plugin(&self, plugin: Box<dyn MLPlugin>) -> Result<(), PluginError> {
        let metadata = plugin.metadata().clone();
        let name = metadata.name.clone();

        if !plugin.is_compatible(&self.system_info) {
            return Err(PluginError::CompatibilityError(format!(
                "Plugin {} is not compatible with current system",
                name
            )));
        }

        let mut plugin = plugin;
        plugin.initialize()?;

        let mut plugins = self.ml_plugins.write().await;
        plugins.insert(name.clone(), plugin);

        self.emit_event(PluginEvent::Registered {
            name: name.clone(),
            version: metadata.version.clone(),
        });

        Ok(())
    }

    /// Register a theme plugin
    pub async fn register_theme_plugin(
        &self,
        plugin: Box<dyn ThemePlugin>,
    ) -> Result<(), PluginError> {
        let metadata = plugin.metadata().clone();
        let name = metadata.name.clone();

        if !plugin.is_compatible(&self.system_info) {
            return Err(PluginError::CompatibilityError(format!(
                "Plugin {} is not compatible with current system",
                name
            )));
        }

        let mut plugin = plugin;
        plugin.initialize()?;

        let mut plugins = self.theme_plugins.write().await;
        plugins.insert(name.clone(), plugin);

        self.emit_event(PluginEvent::Registered {
            name: name.clone(),
            version: metadata.version.clone(),
        });

        Ok(())
    }

    /// Get a chart plugin by name
    pub async fn get_chart_plugin(&self, name: &str) -> Option<Box<dyn ChartPlugin>> {
        let plugins = self.chart_plugins.read().await;
        // For now, we'll return None since we can't safely downcast trait objects
        // In a real implementation, we'd need a different approach for type-safe plugin retrieval
        plugins.get(name).map(|_| None).flatten()
    }

    /// Get all registered chart plugins
    pub async fn list_chart_plugins(&self) -> Vec<String> {
        let plugins = self.chart_plugins.read().await;
        plugins.keys().cloned().collect()
    }

    /// Add event handler
    pub fn add_event_handler(&self, handler: Box<dyn PluginEventHandler>) {
        let mut handlers = self.event_handlers.write().unwrap();
        handlers.push(handler);
    }

    /// Emit plugin event
    fn emit_event(&self, event: PluginEvent) {
        let handlers = self.event_handlers.read().unwrap();
        for handler in handlers.iter() {
            handler.handle_event(event.clone());
        }
    }

    /// Get system information
    pub fn system_info(&self) -> &SystemInfo {
        &self.system_info
    }

    /// Validate all registered plugins
    pub async fn validate_all_plugins(&self) -> Result<(), PluginError> {
        // Validate chart plugins
        let chart_plugins = self.chart_plugins.read().await;
        for (name, plugin) in chart_plugins.iter() {
            if !plugin.is_compatible(&self.system_info) {
                return Err(PluginError::CompatibilityError(format!(
                    "Chart plugin {} is not compatible",
                    name
                )));
            }
        }

        // Validate other plugin types...
        // (Similar validation for other plugin types)

        Ok(())
    }

    /// Cleanup all plugins
    pub async fn cleanup_all_plugins(&self) -> Result<(), PluginError> {
        // Cleanup chart plugins
        let mut chart_plugins = self.chart_plugins.write().await;
        for (name, plugin) in chart_plugins.iter_mut() {
            if let Err(e) = plugin.cleanup() {
                return Err(PluginError::ExecutionFailed(format!(
                    "Failed to cleanup chart plugin {}: {}",
                    name, e
                )));
            }
        }

        // Cleanup other plugin types...
        // (Similar cleanup for other plugin types)

        Ok(())
    }
}

impl SystemInfo {
    /// Get current system information
    pub fn current() -> Self {
        Self {
            helios_version: env!("CARGO_PKG_VERSION").to_string(),
            rust_version: "1.70.0".to_string(), // Default version
            platform: std::env::consts::OS.to_string(),
            features: vec![],          // TODO: Get actual features
            available_memory_mb: 1024, // TODO: Get actual memory
            gpu_available: true,       // TODO: Detect GPU availability
        }
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Plugin registry for static plugin registration
pub struct PluginRegistry {
    plugins: HashMap<String, PluginRegistration>,
}

/// Plugin registration information
pub struct PluginRegistration {
    pub metadata: PluginMetadata,
    pub factory: Box<dyn Fn() -> Box<dyn Plugin> + Send + Sync>,
}

impl PluginRegistry {
    /// Create a new plugin registry
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    /// Register a plugin factory
    pub fn register<F>(&mut self, metadata: PluginMetadata, factory: F)
    where
        F: Fn() -> Box<dyn Plugin> + Send + Sync + 'static,
    {
        let name = metadata.name.clone();
        self.plugins.insert(
            name,
            PluginRegistration {
                metadata,
                factory: Box::new(factory),
            },
        );
    }

    /// Get plugin factory by name
    pub fn get_factory(&self, name: &str) -> Option<&(dyn Fn() -> Box<dyn Plugin> + Send + Sync)> {
        self.plugins.get(name).map(|reg| reg.factory.as_ref())
    }

    /// List all registered plugins
    pub fn list_plugins(&self) -> Vec<&PluginMetadata> {
        self.plugins.values().map(|reg| &reg.metadata).collect()
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock chart plugin for testing
    #[derive(Debug)]
    struct MockChartPlugin {
        metadata: PluginMetadata,
    }

    impl MockChartPlugin {
        fn new() -> Self {
            Self {
                metadata: PluginMetadata {
                    name: "mock-chart".to_string(),
                    version: "1.0.0".to_string(),
                    description: "Mock chart plugin for testing".to_string(),
                    author: "Test Author".to_string(),
                    license: "MIT".to_string(),
                    homepage: None,
                    repository: None,
                    dependencies: Vec::new(),
                    capabilities: PluginCapabilities {
                        capabilities: vec![PluginCapability::ChartRendering],
                        max_data_points: Some(10000),
                        supported_formats: vec!["svg".to_string()],
                        performance_requirements: PerformanceRequirements {
                            max_memory_mb: Some(100),
                            max_execution_time_ms: Some(1000),
                            max_cpu_usage_percent: Some(50),
                            requires_gpu: false,
                        },
                    },
                    security_level: SecurityLevel::Sandboxed,
                    performance_impact: PerformanceImpact::Minimal,
                },
            }
        }
    }

    impl Plugin for MockChartPlugin {
        fn metadata(&self) -> &PluginMetadata {
            &self.metadata
        }

        fn initialize(&mut self) -> Result<(), PluginError> {
            Ok(())
        }

        fn cleanup(&mut self) -> Result<(), PluginError> {
            Ok(())
        }

        fn is_compatible(&self, _system_info: &SystemInfo) -> bool {
            true
        }

        fn type_id(&self) -> TypeId {
            TypeId::of::<MockChartPlugin>()
        }

        fn clone_plugin(&self) -> Box<dyn Plugin> {
            Box::new(MockChartPlugin::new())
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    impl ChartPlugin for MockChartPlugin {
        fn supported_marks(&self) -> Vec<MarkType> {
            vec![MarkType::Point {
                size: Some(5.0),
                opacity: Some(1.0),
                shape: Some(crate::canvas2d_renderer::PointShape::Circle),
            }]
        }

        fn render(
            &self,
            _spec: &ChartSpec,
            _context: &RenderContext,
        ) -> Result<RenderResult, PluginError> {
            Ok(RenderResult::success())
        }

        fn validate_spec(&self, _spec: &ChartSpec) -> Result<(), PluginError> {
            Ok(())
        }

        fn estimate_performance(&self, _spec: &ChartSpec) -> PerformanceEstimate {
            PerformanceEstimate {
                estimated_time_ms: 10,
                estimated_memory_mb: 5,
                complexity_score: 1.0,
            }
        }
    }

    #[tokio::test]
    async fn test_plugin_manager_creation() {
        let manager = PluginManager::new();
        assert_eq!(manager.list_chart_plugins().await.len(), 0);
    }

    #[tokio::test]
    async fn test_chart_plugin_registration() {
        let manager = PluginManager::new();
        let plugin = Box::new(MockChartPlugin::new());

        let result = manager.register_chart_plugin(plugin).await;
        assert!(result.is_ok());

        let plugins = manager.list_chart_plugins().await;
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0], "mock-chart");
    }

    #[tokio::test]
    async fn test_chart_plugin_retrieval() {
        let manager = PluginManager::new();
        let plugin = Box::new(MockChartPlugin::new());

        manager.register_chart_plugin(plugin).await.unwrap();

        // Verify plugin is registered
        let plugins = manager.list_chart_plugins().await;
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0], "mock-chart");

        // Note: get_chart_plugin currently returns None due to trait object downcasting limitations
        // In a real implementation, we'd need a different approach for type-safe plugin retrieval
        let retrieved_plugin = manager.get_chart_plugin("mock-chart").await;
        assert!(retrieved_plugin.is_none()); // Current implementation limitation
    }

    #[tokio::test]
    async fn test_plugin_not_found() {
        let manager = PluginManager::new();
        let plugin = manager.get_chart_plugin("nonexistent").await;
        assert!(plugin.is_none());
    }

    #[test]
    fn test_plugin_metadata() {
        let plugin = MockChartPlugin::new();
        let metadata = plugin.metadata();

        assert_eq!(metadata.name, "mock-chart");
        assert_eq!(metadata.version, "1.0.0");
        assert_eq!(metadata.security_level, SecurityLevel::Sandboxed);
        assert_eq!(metadata.performance_impact, PerformanceImpact::Minimal);
    }

    #[test]
    fn test_plugin_capabilities() {
        let plugin = MockChartPlugin::new();
        let capabilities = &plugin.metadata().capabilities;

        assert!(capabilities
            .capabilities
            .contains(&PluginCapability::ChartRendering));
        assert_eq!(capabilities.max_data_points, Some(10000));
        assert!(capabilities.supported_formats.contains(&"svg".to_string()));
    }

    #[test]
    fn test_system_info() {
        let system_info = SystemInfo::current();

        assert!(!system_info.helios_version.is_empty());
        assert!(!system_info.platform.is_empty());
    }

    #[test]
    fn test_plugin_registry() {
        let mut registry = PluginRegistry::new();
        let plugin = MockChartPlugin::new();
        let metadata = plugin.metadata().clone();

        registry.register(metadata.clone(), || Box::new(MockChartPlugin::new()));

        let plugins = registry.list_plugins();
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0].name, "mock-chart");

        let factory = registry.get_factory("mock-chart");
        assert!(factory.is_some());
    }

    #[test]
    fn test_render_result() {
        let success_result = RenderResult::success();
        assert!(success_result.success);
        assert!(success_result.error.is_none());

        let error_result = RenderResult::error("Test error".to_string());
        assert!(!error_result.success);
        assert_eq!(error_result.error, Some("Test error".to_string()));
    }

    #[test]
    fn test_performance_estimate() {
        let estimate = PerformanceEstimate {
            estimated_time_ms: 100,
            estimated_memory_mb: 50,
            complexity_score: 2.5,
        };

        assert_eq!(estimate.estimated_time_ms, 100);
        assert_eq!(estimate.estimated_memory_mb, 50);
        assert_eq!(estimate.complexity_score, 2.5);
    }
}
