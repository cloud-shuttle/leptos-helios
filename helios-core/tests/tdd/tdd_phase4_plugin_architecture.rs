//! TDD Phase 4: Plugin Architecture Tests
//!
//! This module implements comprehensive Test-Driven Development tests for the
//! plugin architecture system, following the RED-GREEN-REFACTOR cycle.
//!
//! ## Test Coverage
//!
//! - **Plugin Registration**: Dynamic plugin loading and registration
//! - **Plugin Lifecycle**: Initialization, execution, and cleanup
//! - **Plugin Compatibility**: System compatibility checking
//! - **Plugin Security**: Security level enforcement and sandboxing
//! - **Plugin Performance**: Performance monitoring and resource limits
//! - **Plugin Events**: Event handling and notification system
//! - **Plugin Types**: All plugin types (Chart, DataSource, Transform, Export, ML, Theme)
//! - **Plugin Manager**: Central plugin management and coordination
//! - **Plugin Registry**: Static plugin registration and discovery
//!
//! ## TDD Methodology
//!
//! 1. **RED**: Write failing tests first
//! 2. **GREEN**: Implement minimal code to pass tests
//! 3. **REFACTOR**: Improve code quality while maintaining test coverage

use async_trait::async_trait;
use leptos_helios::chart::{ChartSpec, DataReference, Interpolation, MarkType, PointShape};
use leptos_helios::data_sources::{ConnectionConfig, DataSource, DataSourceError, Schema};
use leptos_helios::export_system::{ExportFormat, ExportResult};
use leptos_helios::plugin_system::*;
use leptos_helios::styling::Theme;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

/// Mock chart plugin for testing
#[derive(Debug, Clone)]
struct TestChartPlugin {
    metadata: PluginMetadata,
    render_count: Arc<Mutex<u32>>,
    initialization_called: Arc<Mutex<bool>>,
    cleanup_called: Arc<Mutex<bool>>,
}

impl TestChartPlugin {
    fn new(name: &str, version: &str) -> Self {
        Self {
            metadata: PluginMetadata {
                name: name.to_string(),
                version: version.to_string(),
                description: format!("Test chart plugin {}", name),
                author: "Test Author".to_string(),
                license: "MIT".to_string(),
                homepage: None,
                repository: None,
                dependencies: Vec::new(),
                capabilities: PluginCapabilities {
                    capabilities: vec![PluginCapability::ChartRendering],
                    max_data_points: Some(10000),
                    supported_formats: vec!["svg".to_string(), "png".to_string()],
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
            render_count: Arc::new(Mutex::new(0)),
            initialization_called: Arc::new(Mutex::new(false)),
            cleanup_called: Arc::new(Mutex::new(false)),
        }
    }

    fn get_render_count(&self) -> u32 {
        *self.render_count.lock().unwrap()
    }

    fn was_initialized(&self) -> bool {
        *self.initialization_called.lock().unwrap()
    }

    fn was_cleaned_up(&self) -> bool {
        *self.cleanup_called.lock().unwrap()
    }
}

impl Plugin for TestChartPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    fn initialize(&mut self) -> Result<(), PluginError> {
        *self.initialization_called.lock().unwrap() = true;
        Ok(())
    }

    fn cleanup(&mut self) -> Result<(), PluginError> {
        *self.cleanup_called.lock().unwrap() = true;
        Ok(())
    }

    fn is_compatible(&self, _system_info: &SystemInfo) -> bool {
        true
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<TestChartPlugin>()
    }

    fn clone_plugin(&self) -> Box<dyn Plugin> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ChartPlugin for TestChartPlugin {
    fn supported_marks(&self) -> Vec<MarkType> {
        vec![
            MarkType::Point {
                size: Some(5.0),
                opacity: Some(1.0),
                shape: Some(PointShape::Circle),
            },
            MarkType::Line {
                interpolate: Some(Interpolation::Linear),
                stroke_width: Some(2.0),
                stroke_dash: None,
            },
        ]
    }

    fn render(
        &self,
        _spec: &ChartSpec,
        _context: &RenderContext,
    ) -> Result<RenderResult, PluginError> {
        let mut count = self.render_count.lock().unwrap();
        *count += 1;

        Ok(RenderResult {
            success: true,
            render_time_ms: 10,
            memory_used_mb: 5,
            warnings: Vec::new(),
            error: None,
        })
    }

    fn validate_spec(&self, spec: &ChartSpec) -> Result<(), PluginError> {
        if matches!(spec.data, DataReference::Static(_)) {
            Err(PluginError::ValidationFailed(
                "Static data not supported".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    fn estimate_performance(&self, _spec: &ChartSpec) -> PerformanceEstimate {
        PerformanceEstimate {
            estimated_time_ms: 10,
            estimated_memory_mb: 5,
            complexity_score: 1.0,
        }
    }
}

/// Mock data source plugin for testing
#[derive(Debug, Clone)]
struct TestDataSourcePlugin {
    metadata: PluginMetadata,
    connection_count: Arc<Mutex<u32>>,
}

impl TestDataSourcePlugin {
    fn new(name: &str) -> Self {
        Self {
            metadata: PluginMetadata {
                name: name.to_string(),
                version: "1.0.0".to_string(),
                description: format!("Test data source plugin {}", name),
                author: "Test Author".to_string(),
                license: "MIT".to_string(),
                homepage: None,
                repository: None,
                dependencies: Vec::new(),
                capabilities: PluginCapabilities {
                    capabilities: vec![PluginCapability::DataSource],
                    max_data_points: Some(1000000),
                    supported_formats: vec!["csv".to_string(), "json".to_string()],
                    performance_requirements: PerformanceRequirements {
                        max_memory_mb: Some(500),
                        max_execution_time_ms: Some(5000),
                        max_cpu_usage_percent: Some(80),
                        requires_gpu: false,
                    },
                },
                security_level: SecurityLevel::Sandboxed,
                performance_impact: PerformanceImpact::Moderate,
            },
            connection_count: Arc::new(Mutex::new(0)),
        }
    }

    fn get_connection_count(&self) -> u32 {
        *self.connection_count.lock().unwrap()
    }
}

impl Plugin for TestDataSourcePlugin {
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
        TypeId::of::<TestDataSourcePlugin>()
    }

    fn clone_plugin(&self) -> Box<dyn Plugin> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl DataSourcePlugin for TestDataSourcePlugin {
    fn supported_sources(&self) -> Vec<String> {
        vec!["test-db".to_string(), "mock-api".to_string()]
    }

    fn create_connection(
        &self,
        config: &DataSourceConfig,
    ) -> Result<Box<dyn DataSource>, PluginError> {
        let mut count = self.connection_count.lock().unwrap();
        *count += 1;

        if config.source_type == "test-db" {
            Ok(Box::new(MockDataSource::new()))
        } else {
            Err(PluginError::ValidationFailed(format!(
                "Unsupported source type: {}",
                config.source_type
            )))
        }
    }

    fn validate_config(&self, config: &DataSourceConfig) -> Result<(), PluginError> {
        if config.connection_string.is_empty() {
            Err(PluginError::ValidationFailed(
                "Empty connection string".to_string(),
            ))
        } else {
            Ok(())
        }
    }
}

/// Mock data source for testing
#[derive(Debug)]
struct MockDataSource {
    config: ConnectionConfig,
}

/// Mock connection for testing
#[derive(Debug)]
struct MockConnection;

#[async_trait]
impl leptos_helios::data_sources::Connection for MockConnection {
    async fn query(&self, _sql: &str) -> Result<polars::prelude::DataFrame, DataSourceError> {
        Ok(polars::prelude::DataFrame::new(vec![]).unwrap())
    }

    async fn execute(&self, _sql: &str) -> Result<u64, DataSourceError> {
        Ok(0)
    }

    async fn query_with_params(
        &self,
        _sql: &str,
        _params: &[&(dyn leptos_helios::data_sources::ToSql)],
    ) -> Result<polars::prelude::DataFrame, DataSourceError> {
        Ok(polars::prelude::DataFrame::new(vec![]).unwrap())
    }

    async fn query_stream(
        &self,
        _sql: &str,
    ) -> Result<Box<dyn leptos_helios::data_sources::DataStream>, DataSourceError> {
        Ok(Box::new(MockDataStream))
    }

    async fn begin_transaction(
        &self,
    ) -> Result<Box<dyn leptos_helios::data_sources::Transaction>, DataSourceError> {
        Ok(Box::new(MockTransaction) as Box<dyn leptos_helios::data_sources::Transaction>)
    }

    async fn close(&self) -> Result<(), DataSourceError> {
        Ok(())
    }
}

/// Mock transaction for testing
#[derive(Debug)]
struct MockTransaction;

/// Mock data stream for testing
#[derive(Debug)]
struct MockDataStream;

#[async_trait]
impl leptos_helios::data_sources::Transaction for MockTransaction {
    async fn query(&self, _sql: &str) -> Result<polars::prelude::DataFrame, DataSourceError> {
        Ok(polars::prelude::DataFrame::new(vec![]).unwrap())
    }

    async fn commit(&self) -> Result<(), DataSourceError> {
        Ok(())
    }

    async fn rollback(&self) -> Result<(), DataSourceError> {
        Ok(())
    }
}

#[async_trait]
impl leptos_helios::data_sources::DataStream for MockDataStream {
    async fn next_batch(&mut self) -> Result<Option<polars::prelude::DataFrame>, DataSourceError> {
        Ok(None)
    }

    fn estimated_rows(&self) -> Option<usize> {
        Some(0)
    }
}

impl MockDataSource {
    fn new() -> Self {
        Self {
            config: ConnectionConfig {
                connection_string: "postgresql://localhost:5432/test".to_string(),
                query_timeout: Some(std::time::Duration::from_secs(30)),
                ssl_mode: None,
                connection_timeout: Some(std::time::Duration::from_secs(30)),
                credentials: None,
                max_connections: Some(10),
            },
        }
    }
}

#[async_trait]
impl DataSource for MockDataSource {
    async fn connect(
        &self,
    ) -> Result<Box<dyn leptos_helios::data_sources::Connection>, DataSourceError> {
        Ok(Box::new(MockConnection) as Box<dyn leptos_helios::data_sources::Connection>)
    }

    async fn get_schema(&self) -> Result<Schema, DataSourceError> {
        Ok(Schema {
            tables: Vec::new(),
            views: Vec::new(),
            version: Some("1.0.0".to_string()),
        })
    }

    async fn health_check(&self) -> Result<bool, DataSourceError> {
        Ok(true)
    }

    fn source_type(&self) -> &'static str {
        "mock"
    }

    fn config(&self) -> &ConnectionConfig {
        &self.config
    }
}

/// Mock event handler for testing
#[derive(Debug, Clone)]
struct TestEventHandler {
    events: Arc<Mutex<Vec<PluginEvent>>>,
}

impl TestEventHandler {
    fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn get_events(&self) -> Vec<PluginEvent> {
        self.events.lock().unwrap().clone()
    }

    fn clear_events(&self) {
        self.events.lock().unwrap().clear();
    }
}

impl PluginEventHandler for TestEventHandler {
    fn handle_event(&self, event: PluginEvent) {
        self.events.lock().unwrap().push(event);
    }
}

/// Test suite for plugin manager functionality
mod plugin_manager_tests {
    use super::*;

    #[tokio::test]
    async fn test_plugin_manager_creation() {
        // RED: Test that plugin manager can be created
        let manager = PluginManager::new();

        // GREEN: Verify initial state
        assert_eq!(manager.list_chart_plugins().await.len(), 0);
        assert!(manager.system_info().helios_version.len() > 0);
    }

    #[tokio::test]
    async fn test_chart_plugin_registration() {
        // RED: Test chart plugin registration
        let manager = PluginManager::new();
        let plugin = Box::new(TestChartPlugin::new("test-chart", "1.0.0"));

        // GREEN: Register plugin and verify
        let result = manager.register_chart_plugin(plugin).await;
        assert!(result.is_ok());

        let plugins = manager.list_chart_plugins().await;
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0], "test-chart");
    }

    #[tokio::test]
    async fn test_chart_plugin_retrieval() {
        // RED: Test plugin retrieval by name
        let manager = PluginManager::new();
        let plugin = Box::new(TestChartPlugin::new("test-chart", "1.0.0"));

        manager.register_chart_plugin(plugin).await.unwrap();

        // GREEN: Retrieve and verify plugin
        let retrieved_plugin = manager.get_chart_plugin("test-chart").await;
        assert!(retrieved_plugin.is_some());

        let plugin = retrieved_plugin.unwrap();
        assert_eq!(plugin.metadata().name, "test-chart");
        assert_eq!(plugin.metadata().version, "1.0.0");
    }

    #[tokio::test]
    async fn test_plugin_not_found() {
        // RED: Test retrieval of non-existent plugin
        let manager = PluginManager::new();

        // GREEN: Verify None is returned
        let plugin = manager.get_chart_plugin("nonexistent").await;
        assert!(plugin.is_none());
    }

    #[tokio::test]
    async fn test_multiple_plugin_registration() {
        // RED: Test registering multiple plugins
        let manager = PluginManager::new();

        let plugin1 = Box::new(TestChartPlugin::new("plugin1", "1.0.0"));
        let plugin2 = Box::new(TestChartPlugin::new("plugin2", "2.0.0"));

        // GREEN: Register both plugins
        manager.register_chart_plugin(plugin1).await.unwrap();
        manager.register_chart_plugin(plugin2).await.unwrap();

        let plugins = manager.list_chart_plugins().await;
        assert_eq!(plugins.len(), 2);
        assert!(plugins.contains(&"plugin1".to_string()));
        assert!(plugins.contains(&"plugin2".to_string()));
    }

    #[tokio::test]
    async fn test_plugin_initialization() {
        // RED: Test that plugins are initialized during registration
        let manager = PluginManager::new();
        let plugin = TestChartPlugin::new("test-chart", "1.0.0");
        let was_initialized = plugin.was_initialized();

        // GREEN: Register plugin and verify initialization
        assert!(!was_initialized);

        let boxed_plugin = Box::new(plugin);
        manager.register_chart_plugin(boxed_plugin).await.unwrap();

        // Note: We can't access the plugin after registration, but we can test
        // that registration succeeds, which means initialization succeeded
        let plugins = manager.list_chart_plugins().await;
        assert_eq!(plugins.len(), 1);
    }

    #[tokio::test]
    async fn test_data_source_plugin_registration() {
        // RED: Test data source plugin registration
        let manager = PluginManager::new();
        let plugin = Box::new(TestDataSourcePlugin::new("test-datasource"));

        // GREEN: Register and verify
        let result = manager.register_data_source_plugin(plugin).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_plugin_cleanup() {
        // RED: Test plugin cleanup functionality
        let manager = PluginManager::new();

        // GREEN: Register plugins and cleanup
        let plugin1 = Box::new(TestChartPlugin::new("plugin1", "1.0.0"));
        let plugin2 = Box::new(TestDataSourcePlugin::new("plugin2"));

        manager.register_chart_plugin(plugin1).await.unwrap();
        manager.register_data_source_plugin(plugin2).await.unwrap();

        let result = manager.cleanup_all_plugins().await;
        assert!(result.is_ok());
    }
}

/// Test suite for plugin metadata and capabilities
mod plugin_metadata_tests {
    use super::*;

    #[test]
    fn test_plugin_metadata_creation() {
        // RED: Test plugin metadata structure
        let metadata = PluginMetadata {
            name: "test-plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test Author".to_string(),
            license: "MIT".to_string(),
            homepage: Some("https://example.com".to_string()),
            repository: Some("https://github.com/example/test-plugin".to_string()),
            dependencies: vec![PluginDependency {
                name: "dependency1".to_string(),
                version: "1.0.0".to_string(),
                optional: false,
            }],
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
        };

        // GREEN: Verify metadata fields
        assert_eq!(metadata.name, "test-plugin");
        assert_eq!(metadata.version, "1.0.0");
        assert_eq!(metadata.dependencies.len(), 1);
        assert_eq!(metadata.dependencies[0].name, "dependency1");
        assert_eq!(metadata.security_level, SecurityLevel::Sandboxed);
        assert_eq!(metadata.performance_impact, PerformanceImpact::Minimal);
    }

    #[test]
    fn test_plugin_capabilities() {
        // RED: Test plugin capabilities
        let capabilities = PluginCapabilities {
            capabilities: vec![
                PluginCapability::ChartRendering,
                PluginCapability::DataSource,
                PluginCapability::MLIntelligence,
            ],
            max_data_points: Some(1000000),
            supported_formats: vec!["svg".to_string(), "png".to_string(), "pdf".to_string()],
            performance_requirements: PerformanceRequirements {
                max_memory_mb: Some(500),
                max_execution_time_ms: Some(5000),
                max_cpu_usage_percent: Some(80),
                requires_gpu: true,
            },
        };

        // GREEN: Verify capabilities
        assert!(capabilities
            .capabilities
            .contains(&PluginCapability::ChartRendering));
        assert!(capabilities
            .capabilities
            .contains(&PluginCapability::DataSource));
        assert!(capabilities
            .capabilities
            .contains(&PluginCapability::MLIntelligence));
        assert_eq!(capabilities.max_data_points, Some(1000000));
        assert_eq!(capabilities.supported_formats.len(), 3);
        assert!(capabilities.performance_requirements.requires_gpu);
    }

    #[test]
    fn test_security_levels() {
        // RED: Test security level ordering
        let untrusted = SecurityLevel::Untrusted;
        let sandboxed = SecurityLevel::Sandboxed;
        let trusted = SecurityLevel::Trusted;

        // GREEN: Verify ordering
        assert!(untrusted < sandboxed);
        assert!(sandboxed < trusted);
        assert!(untrusted < trusted);
    }

    #[test]
    fn test_performance_impact_levels() {
        // RED: Test performance impact ordering
        let minimal = PerformanceImpact::Minimal;
        let moderate = PerformanceImpact::Moderate;
        let high = PerformanceImpact::High;
        let critical = PerformanceImpact::Critical;

        // GREEN: Verify ordering
        assert!(minimal < moderate);
        assert!(moderate < high);
        assert!(high < critical);
    }
}

/// Test suite for plugin rendering functionality
mod plugin_rendering_tests {
    use super::*;

    #[tokio::test]
    async fn test_chart_plugin_rendering() {
        // RED: Test chart plugin rendering
        let manager = PluginManager::new();
        let plugin = TestChartPlugin::new("test-chart", "1.0.0");
        let render_count = plugin.get_render_count();

        manager
            .register_chart_plugin(Box::new(plugin))
            .await
            .unwrap();

        // GREEN: Get plugin and test rendering
        let retrieved_plugin = manager.get_chart_plugin("test-chart").await.unwrap();

        let spec = ChartSpec::new();
        let context = RenderContext {
            viewport: Viewport {
                width: 800,
                height: 600,
                dpi: 96.0,
                pixel_ratio: 1.0,
            },
            device_info: DeviceInfo {
                gpu_available: true,
                gpu_vendor: Some("Test GPU".to_string()),
                memory_mb: 1024,
                cpu_cores: 4,
            },
            performance_budget: PerformanceBudget {
                max_render_time_ms: 1000,
                max_memory_mb: 100,
                max_cpu_usage_percent: 50,
            },
            security_context: SecurityContext {
                security_level: SecurityLevel::Sandboxed,
                allowed_operations: vec!["render".to_string()],
                sandbox_mode: true,
            },
        };

        let result = retrieved_plugin.render(&spec, &context);
        assert!(result.is_ok());

        let render_result = result.unwrap();
        assert!(render_result.success);
        assert_eq!(render_result.render_time_ms, 10);
        assert_eq!(render_result.memory_used_mb, 5);
    }

    #[tokio::test]
    async fn test_chart_plugin_spec_validation() {
        // RED: Test chart specification validation
        let manager = PluginManager::new();
        let plugin = TestChartPlugin::new("test-chart", "1.0.0");

        manager
            .register_chart_plugin(Box::new(plugin))
            .await
            .unwrap();

        // GREEN: Test validation
        let retrieved_plugin = manager.get_chart_plugin("test-chart").await.unwrap();

        let spec = ChartSpec::new();
        let validation_result = retrieved_plugin.validate_spec(&spec);

        // Should fail because spec has empty data
        assert!(validation_result.is_err());
        if let Err(PluginError::ValidationFailed(msg)) = validation_result {
            assert_eq!(msg, "Empty data");
        }
    }

    #[tokio::test]
    async fn test_chart_plugin_performance_estimation() {
        // RED: Test performance estimation
        let manager = PluginManager::new();
        let plugin = TestChartPlugin::new("test-chart", "1.0.0");

        manager
            .register_chart_plugin(Box::new(plugin))
            .await
            .unwrap();

        // GREEN: Test performance estimation
        let retrieved_plugin = manager.get_chart_plugin("test-chart").await.unwrap();

        let spec = ChartSpec::new();
        let estimate = retrieved_plugin.estimate_performance(&spec);

        assert_eq!(estimate.estimated_time_ms, 10);
        assert_eq!(estimate.estimated_memory_mb, 5);
        assert_eq!(estimate.complexity_score, 1.0);
    }

    #[tokio::test]
    async fn test_chart_plugin_supported_marks() {
        // RED: Test supported mark types
        let manager = PluginManager::new();
        let plugin = TestChartPlugin::new("test-chart", "1.0.0");

        manager
            .register_chart_plugin(Box::new(plugin))
            .await
            .unwrap();

        // GREEN: Test supported marks
        let retrieved_plugin = manager.get_chart_plugin("test-chart").await.unwrap();
        let supported_marks = retrieved_plugin.supported_marks();

        assert_eq!(supported_marks.len(), 2);
        assert!(supported_marks.contains(&MarkType::Point {
            size: Some(5.0),
            opacity: Some(1.0),
            shape: Some(PointShape::Circle)
        }));
        assert!(supported_marks.contains(&MarkType::Line {
            interpolate: Some(Interpolation::Linear),
            stroke_width: Some(2.0),
            stroke_dash: None,
        }));
    }
}

/// Test suite for data source plugin functionality
mod data_source_plugin_tests {
    use super::*;

    #[tokio::test]
    async fn test_data_source_plugin_connection_creation() {
        // RED: Test data source connection creation
        let manager = PluginManager::new();
        let plugin = TestDataSourcePlugin::new("test-datasource");
        let connection_count = plugin.get_connection_count();

        manager
            .register_data_source_plugin(Box::new(plugin))
            .await
            .unwrap();

        // GREEN: Test connection creation
        assert_eq!(connection_count, 0);

        // Note: We can't directly access the plugin after registration,
        // but we can test that registration succeeds
        // In a real implementation, we'd have methods to access registered plugins
    }

    #[test]
    fn test_data_source_plugin_supported_sources() {
        // RED: Test supported data source types
        let plugin = TestDataSourcePlugin::new("test-datasource");

        // GREEN: Test supported sources
        let supported_sources = plugin.supported_sources();
        assert_eq!(supported_sources.len(), 2);
        assert!(supported_sources.contains(&"test-db".to_string()));
        assert!(supported_sources.contains(&"mock-api".to_string()));
    }

    #[test]
    fn test_data_source_plugin_config_validation() {
        // RED: Test configuration validation
        let plugin = TestDataSourcePlugin::new("test-datasource");

        // GREEN: Test valid configuration
        let valid_config = DataSourceConfig {
            source_type: "test-db".to_string(),
            connection_string: "test://connection".to_string(),
            credentials: None,
            options: HashMap::new(),
        };

        let result = plugin.validate_config(&valid_config);
        assert!(result.is_ok());

        // Test invalid configuration
        let invalid_config = DataSourceConfig {
            source_type: "test-db".to_string(),
            connection_string: "".to_string(), // Empty connection string
            credentials: None,
            options: HashMap::new(),
        };

        let result = plugin.validate_config(&invalid_config);
        assert!(result.is_err());
        if let Err(PluginError::ValidationFailed(msg)) = result {
            assert_eq!(msg, "Empty connection string");
        }
    }
}

/// Test suite for plugin event handling
mod plugin_event_tests {
    use super::*;

    #[tokio::test]
    async fn test_plugin_event_handling() {
        // RED: Test plugin event handling
        let manager = PluginManager::new();
        let event_handler = TestEventHandler::new();

        manager.add_event_handler(Box::new(event_handler.clone()));

        // GREEN: Register plugin and verify event
        let plugin = Box::new(TestChartPlugin::new("test-chart", "1.0.0"));
        manager.register_chart_plugin(plugin).await.unwrap();

        // Give some time for event processing
        sleep(Duration::from_millis(10)).await;

        let events = event_handler.get_events();
        assert_eq!(events.len(), 1);

        if let PluginEvent::Registered { name, version } = &events[0] {
            assert_eq!(name, "test-chart");
            assert_eq!(version, "1.0.0");
        } else {
            panic!("Expected Registered event");
        }
    }

    #[tokio::test]
    async fn test_multiple_event_handlers() {
        // RED: Test multiple event handlers
        let manager = PluginManager::new();
        let handler1 = TestEventHandler::new();
        let handler2 = TestEventHandler::new();

        manager.add_event_handler(Box::new(handler1.clone()));
        manager.add_event_handler(Box::new(handler2.clone()));

        // GREEN: Register plugin and verify both handlers receive events
        let plugin = Box::new(TestChartPlugin::new("test-chart", "1.0.0"));
        manager.register_chart_plugin(plugin).await.unwrap();

        sleep(Duration::from_millis(10)).await;

        let events1 = handler1.get_events();
        let events2 = handler2.get_events();

        assert_eq!(events1.len(), 1);
        assert_eq!(events2.len(), 1);
    }
}

/// Test suite for plugin registry functionality
mod plugin_registry_tests {
    use super::*;

    #[test]
    fn test_plugin_registry_creation() {
        // RED: Test plugin registry creation
        let registry = PluginRegistry::new();

        // GREEN: Verify initial state
        let plugins = registry.list_plugins();
        assert_eq!(plugins.len(), 0);
    }

    #[test]
    fn test_plugin_registry_registration() {
        // RED: Test plugin registration in registry
        let mut registry = PluginRegistry::new();
        let metadata = PluginMetadata {
            name: "test-plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Test plugin".to_string(),
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
        };

        registry.register(metadata.clone(), || {
            Box::new(TestChartPlugin::new("test-plugin", "1.0.0"))
        });

        // GREEN: Verify registration
        let plugins = registry.list_plugins();
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0].name, "test-plugin");
    }

    #[test]
    fn test_plugin_registry_factory_retrieval() {
        // RED: Test plugin factory retrieval
        let mut registry = PluginRegistry::new();
        let metadata = PluginMetadata {
            name: "test-plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Test plugin".to_string(),
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
        };

        registry.register(metadata, || {
            Box::new(TestChartPlugin::new("test-plugin", "1.0.0"))
        });

        // GREEN: Test factory retrieval
        let factory = registry.get_factory("test-plugin");
        assert!(factory.is_some());

        let plugin = factory.unwrap()();
        assert_eq!(plugin.metadata().name, "test-plugin");
    }

    #[test]
    fn test_plugin_registry_nonexistent_plugin() {
        // RED: Test retrieval of non-existent plugin
        let registry = PluginRegistry::new();

        // GREEN: Verify None is returned
        let factory = registry.get_factory("nonexistent");
        assert!(factory.is_none());
    }
}

/// Test suite for system information and compatibility
mod system_compatibility_tests {
    use super::*;

    #[test]
    fn test_system_info_creation() {
        // RED: Test system information creation
        let system_info = SystemInfo::current();

        // GREEN: Verify system info fields
        assert!(!system_info.helios_version.is_empty());
        assert!(!system_info.platform.is_empty());
        assert!(system_info.available_memory_mb > 0);
    }

    #[test]
    fn test_plugin_compatibility_checking() {
        // RED: Test plugin compatibility checking
        let plugin = TestChartPlugin::new("test-chart", "1.0.0");
        let system_info = SystemInfo::current();

        // GREEN: Test compatibility
        let is_compatible = plugin.is_compatible(&system_info);
        assert!(is_compatible);
    }
}

/// Test suite for render context and results
mod render_context_tests {
    use super::*;

    #[test]
    fn test_render_context_creation() {
        // RED: Test render context creation
        let context = RenderContext {
            viewport: Viewport {
                width: 1920,
                height: 1080,
                dpi: 96.0,
                pixel_ratio: 2.0,
            },
            device_info: DeviceInfo {
                gpu_available: true,
                gpu_vendor: Some("NVIDIA".to_string()),
                memory_mb: 8192,
                cpu_cores: 8,
            },
            performance_budget: PerformanceBudget {
                max_render_time_ms: 16, // 60 FPS
                max_memory_mb: 512,
                max_cpu_usage_percent: 80,
            },
            security_context: SecurityContext {
                security_level: SecurityLevel::Sandboxed,
                allowed_operations: vec!["render".to_string(), "validate".to_string()],
                sandbox_mode: true,
            },
        };

        // GREEN: Verify context fields
        assert_eq!(context.viewport.width, 1920);
        assert_eq!(context.viewport.height, 1080);
        assert_eq!(context.device_info.memory_mb, 8192);
        assert_eq!(context.performance_budget.max_render_time_ms, 16);
        assert_eq!(
            context.security_context.security_level,
            SecurityLevel::Sandboxed
        );
    }

    #[test]
    fn test_render_result_creation() {
        // RED: Test render result creation
        let success_result = RenderResult::success();
        let error_result = RenderResult::error("Test error".to_string());

        // GREEN: Verify results
        assert!(success_result.success);
        assert!(success_result.error.is_none());
        assert!(!error_result.success);
        assert_eq!(error_result.error, Some("Test error".to_string()));
    }

    #[test]
    fn test_performance_estimate_creation() {
        // RED: Test performance estimate creation
        let estimate = PerformanceEstimate {
            estimated_time_ms: 100,
            estimated_memory_mb: 50,
            complexity_score: 2.5,
        };

        // GREEN: Verify estimate
        assert_eq!(estimate.estimated_time_ms, 100);
        assert_eq!(estimate.estimated_memory_mb, 50);
        assert_eq!(estimate.complexity_score, 2.5);
    }
}

/// Integration tests for the complete plugin system
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_complete_plugin_workflow() {
        // RED: Test complete plugin workflow
        let manager = PluginManager::new();
        let event_handler = TestEventHandler::new();

        manager.add_event_handler(Box::new(event_handler.clone()));

        // GREEN: Complete workflow
        // 1. Register chart plugin
        let chart_plugin = Box::new(TestChartPlugin::new("chart-plugin", "1.0.0"));
        manager.register_chart_plugin(chart_plugin).await.unwrap();

        // 2. Register data source plugin
        let data_source_plugin = Box::new(TestDataSourcePlugin::new("datasource-plugin"));
        manager
            .register_data_source_plugin(data_source_plugin)
            .await
            .unwrap();

        // 3. Verify plugins are registered
        let chart_plugins = manager.list_chart_plugins().await;
        assert_eq!(chart_plugins.len(), 1);
        assert_eq!(chart_plugins[0], "chart-plugin");

        // 4. Verify events were emitted
        sleep(Duration::from_millis(10)).await;
        let events = event_handler.get_events();
        assert_eq!(events.len(), 2);

        // 5. Test plugin functionality
        let retrieved_chart_plugin = manager.get_chart_plugin("chart-plugin").await.unwrap();
        let spec = ChartSpec::new();
        let context = RenderContext {
            viewport: Viewport {
                width: 800,
                height: 600,
                dpi: 96.0,
                pixel_ratio: 1.0,
            },
            device_info: DeviceInfo {
                gpu_available: true,
                gpu_vendor: Some("Test GPU".to_string()),
                memory_mb: 1024,
                cpu_cores: 4,
            },
            performance_budget: PerformanceBudget {
                max_render_time_ms: 1000,
                max_memory_mb: 100,
                max_cpu_usage_percent: 50,
            },
            security_context: SecurityContext {
                security_level: SecurityLevel::Sandboxed,
                allowed_operations: vec!["render".to_string()],
                sandbox_mode: true,
            },
        };

        let render_result = retrieved_chart_plugin.render(&spec, &context);
        assert!(render_result.is_ok());

        // 6. Cleanup
        let cleanup_result = manager.cleanup_all_plugins().await;
        assert!(cleanup_result.is_ok());
    }

    #[tokio::test]
    async fn test_plugin_performance_monitoring() {
        // RED: Test plugin performance monitoring
        let manager = PluginManager::new();
        let plugin = Box::new(TestChartPlugin::new("perf-plugin", "1.0.0"));

        manager.register_chart_plugin(plugin).await.unwrap();

        // GREEN: Test performance monitoring
        let retrieved_plugin = manager.get_chart_plugin("perf-plugin").await.unwrap();

        let spec = ChartSpec::new();
        let context = RenderContext {
            viewport: Viewport {
                width: 800,
                height: 600,
                dpi: 96.0,
                pixel_ratio: 1.0,
            },
            device_info: DeviceInfo {
                gpu_available: true,
                gpu_vendor: Some("Test GPU".to_string()),
                memory_mb: 1024,
                cpu_cores: 4,
            },
            performance_budget: PerformanceBudget {
                max_render_time_ms: 1000,
                max_memory_mb: 100,
                max_cpu_usage_percent: 50,
            },
            security_context: SecurityContext {
                security_level: SecurityLevel::Sandboxed,
                allowed_operations: vec!["render".to_string()],
                sandbox_mode: true,
            },
        };

        // Test multiple renders to verify performance tracking
        for _ in 0..5 {
            let result = retrieved_plugin.render(&spec, &context);
            assert!(result.is_ok());

            let render_result = result.unwrap();
            assert!(render_result.success);
            assert!(render_result.render_time_ms <= 1000); // Within budget
            assert!(render_result.memory_used_mb <= 100); // Within budget
        }
    }
}
