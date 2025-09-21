# Leptos Helios API Reference

## ⚠️ Implementation Status
- **Status**: 📋 Planned - Most features are type definitions only
- **ETA**: Q1-Q2 2026
- **Current State**: API surface defined, implementation pending

## Overview

Leptos Helios is a high-performance, enterprise-grade charting library built with Rust and WebAssembly. It provides comprehensive data visualization capabilities with advanced features including WebGPU rendering, accessibility compliance, enterprise security, and extensive customization options.

## Table of Contents

- [Core Charting API](#core-charting-api)
- [Rendering Backends](#rendering-backends)
- [Data Sources](#data-sources)
- [Export System](#export-system)
- [Accessibility](#accessibility)
- [Security & Governance](#security--governance)
- [Performance](#performance)
- [Plugin System](#plugin-system)
- [Headless Rendering](#headless-rendering)

## Core Charting API

### ChartSpec

The fundamental structure for defining charts in Leptos Helios.

```rust
use leptos_helios::chart::{ChartSpec, MarkType, Encoding, DataReference};

let chart_spec = ChartSpec {
    transform: vec![],
    selection: vec![],
    intelligence: None,
    config: ChartConfig::default(),
    data: DataReference::Static(dataframe),
    encoding: Encoding::default(),
};
```

### Mark Types

Supported chart types with comprehensive customization options:

```rust
use leptos_helios::chart::MarkType;

// Bar charts
let bar_chart = MarkType::Bar {
    width: Some(20.0),
    corner_radius: Some(4.0),
};

// Line charts
let line_chart = MarkType::Line {
    interpolate: Some(Interpolation::Smooth),
};

// Scatter plots
let scatter_plot = MarkType::Scatter {
    size: Some(5.0),
    opacity: Some(0.8),
};

// Area charts
let area_chart = MarkType::Area {
    interpolate: Some(Interpolation::Step),
    fill_opacity: Some(0.6),
};
```

### Data Encoding

Define how data maps to visual properties:

```rust
use leptos_helios::chart::{Encoding, Channel, Field};

let encoding = Encoding {
    x: Some(Channel::Quantitative(Field::new("sales"))),
    y: Some(Channel::Quantitative(Field::new("profit"))),
    color: Some(Channel::Nominal(Field::new("category"))),
    size: Some(Channel::Quantitative(Field::new("volume"))),
};
```

## Rendering Backends

### WebGPU Renderer

High-performance GPU-accelerated rendering:

```rust
use leptos_helios::webgpu_renderer::WebGpuRenderer;

let renderer = WebGpuRenderer::new()?;
let result = renderer.render_chart(&chart_spec, &data).await?;
```

### Canvas2D Renderer

Fallback rendering for broader compatibility:

```rust
use leptos_helios::canvas2d_renderer::Canvas2DRenderer;

let renderer = Canvas2DRenderer::new();
let result = renderer.render_chart(&chart_spec, &data)?;
```

### WebGL2 Renderer

WebGL-based rendering for modern browsers:

```rust
use leptos_helios::renderer::WebGl2Renderer;

let renderer = WebGl2Renderer::new()?;
let result = renderer.render_chart(&chart_spec, &data)?;
```

## Data Sources

### Database Connections

Connect to various database systems:

```rust
use leptos_helios::data_sources::{PostgresAdapter, ClickHouseAdapter};

// PostgreSQL connection
let postgres = PostgresAdapter::new(ConnectionConfig {
    connection_string: "postgresql://user:pass@localhost/db".to_string(), // pragma: allowlist secret
    query_timeout: Some(30),
    ssl_mode: SslMode::Require,
    connection_timeout: Some(10),
    credentials: Some(Credentials::new("user", "pass")),
    max_connections: Some(10),
}).await?;

// ClickHouse connection
let clickhouse = ClickHouseAdapter::new(ConnectionConfig {
    connection_string: "clickhouse://localhost:9000/db".to_string(),
    query_timeout: Some(60),
    ssl_mode: SslMode::Disable,
    connection_timeout: Some(15),
    credentials: Some(Credentials::new("user", "pass")),
    max_connections: Some(20),
}).await?;
```

### Data Processing

Process and transform data efficiently:

```rust
use leptos_helios::data_sources::DataSource;

let connection = postgres.connect().await?;
let result = connection.query("SELECT * FROM sales_data").await?;
let dataframe = result.to_dataframe()?;
```

## Export System

### Supported Formats

Export charts to various formats:

```rust
use leptos_helios::export_system::{ExportSystem, ExportFormat, ExportConfig};

let export_system = ExportSystem::new()?;

// PNG export
let png_data = export_system.export_to_png(
    &chart_spec,
    &data,
    ExportConfig {
        width: 800,
        height: 600,
        dpi: 300,
        background: Some(Color::White),
    }
).await?;

// PDF export
let pdf_data = export_system.export_to_pdf(
    &chart_spec,
    &data,
    ExportConfig {
        width: 800,
        height: 600,
        dpi: 300,
        background: Some(Color::White),
    }
).await?;

// SVG export
let svg_data = export_system.export_to_svg(
    &chart_spec,
    &data,
    ExportConfig {
        width: 800,
        height: 600,
        dpi: 300,
        background: Some(Color::White),
    }
).await?;
```

## Accessibility

### WCAG 2.1 AA Compliance

Ensure accessibility compliance:

```rust
use leptos_helios::accessibility::{AccessibilitySystem, AccessibilityConfig, PerformanceConfig};

let config = AccessibilityConfig {
    wcag_level: WCAGLevel::AA,
    screen_reader: ScreenReaderSupport {
        enabled: true,
        generate_alt_text: true,
        create_data_tables: true,
        provide_summaries: true,
        announce_updates: true,
        aria_labels: true,
        structured_navigation: true,
    },
    keyboard_nav: KeyboardNavigation::default(),
    color_vision: ColorVisionSupport::default(),
    motion: MotionPreferences::default(),
    focus_management: FocusManagement::default(),
    alternative_formats: AlternativeFormats::default(),
};

let performance_config = PerformanceConfig::default();
let accessibility_system = AccessibilitySystem::new(config, performance_config);

// Validate compliance
let compliance_result = accessibility_system
    .validate_wcag_compliance(&chart_spec, &data)?;

// Generate alt text
let alt_text = accessibility_system
    .generate_alt_text(&chart_spec, &data)?;

// Create data table
let data_table = accessibility_system
    .create_data_table(&chart_spec, &data)?;
```

### Keyboard Navigation

Configure keyboard navigation:

```rust
use leptos_helios::accessibility::KeyboardNavigation;

let keyboard_nav = KeyboardNavigation {
    enabled: true,
    tab_order: true,
    arrow_key_navigation: true,
    skip_links: true,
    focus_indicators: true,
    escape_handling: true,
    custom_shortcuts: {
        let mut shortcuts = HashMap::new();
        shortcuts.insert("zoom_in".to_string(), "ctrl+plus".to_string());
        shortcuts.insert("zoom_out".to_string(), "ctrl+minus".to_string());
        shortcuts.insert("reset_view".to_string(), "ctrl+0".to_string());
        shortcuts.insert("toggle_data_table".to_string(), "ctrl+t".to_string());
        shortcuts
    },
};
```

## Security & Governance

### Authentication & Authorization

Enterprise security features:

```rust
use leptos_helios::security::{SecurityConfig, OAuth2Provider, SAMLProvider, RBACProvider};

// OAuth2 authentication
let oauth2_provider = OAuth2Provider::new(
    "client_id".to_string(),
    "client_secret".to_string(),
    "https://auth.example.com/authorize".to_string(),
    "https://auth.example.com/token".to_string(),
    "https://auth.example.com/userinfo".to_string(),
    vec!["read", "write"].iter().map(|s| s.to_string()).collect(),
);

// SAML authentication
let saml_provider = SAMLProvider::new(
    "https://saml.example.com/metadata".to_string(),
    "entity_id".to_string(),
    "x509_cert".to_string(),
    "private_key".to_string(),
);

// Role-based access control
let rbac_provider = RBACProvider::new();
rbac_provider.create_role("admin", vec!["read", "write", "delete"]).await?;
rbac_provider.create_role("user", vec!["read"]).await?;
rbac_provider.assign_role_to_user("user123", "admin").await?;

let security_config = SecurityConfig::new(
    Box::new(oauth2_provider),
    Box::new(saml_provider),
    Box::new(rbac_provider),
);
```

### Data Governance

Comprehensive data governance and compliance:

```rust
use leptos_helios::security::{DataGovernance, DataClassification, DataPolicy};

let governance = DataGovernance::new();

// Classify data
governance.classify_data("customer_data", DataClassification::Confidential).await?;

// Add data policy
let policy = DataPolicy {
    id: "policy_1".to_string(),
    name: "Confidential Data Policy".to_string(),
    classification: DataClassification::Confidential,
    retention_days: Some(2555), // 7 years
    geographic_restrictions: vec!["EU".to_string(), "US".to_string()],
    allowed_exports: vec!["encrypted_pdf".to_string()],
    required_approvals: vec!["data_officer".to_string()],
    encryption_required: true,
    anonymization_required: false,
    access_controls: vec!["rbac".to_string(), "mfa".to_string()],
    audit_required: true,
};

governance.add_data_policy(policy).await?;

// Check export compliance
let is_compliant = governance
    .check_export_compliance("customer_data", "encrypted_pdf", &user)
    .await?;
```

### Audit Logging

Comprehensive audit trail:

```rust
use leptos_helios::security::{AuditLogger, AuditEvent, AuditEventType, AuditResult};

let audit_logger = AuditLogger::new()
    .with_retention_days(90)
    .with_max_log_size(100_000)
    .with_real_time_alerts(true)
    .with_alert_threshold(AuditEventType::SecurityViolation, 5);

// Log authentication event
audit_logger.log_authentication(
    "user123",
    "oauth2",
    AuditResult::Success,
    Some("Login successful".to_string()),
).await?;

// Log data access
audit_logger.log_data_access(
    "user123",
    "customer_data",
    "read",
    AuditResult::Success,
    Some("Data accessed for report generation".to_string()),
    Some(DataClassification::Confidential),
).await?;

// Get audit statistics
let stats = audit_logger.get_audit_statistics().await?;
println!("Total events: {}", stats.total_events);
println!("Security violations: {}", stats.security_violations);
```

## Performance

### Performance Monitoring

Monitor and optimize performance:

```rust
use leptos_helios::performance::{PerformanceProfiler, PerformanceMetrics};

let profiler = PerformanceProfiler::new();

// Start profiling
profiler.start_profiling("chart_rendering");

// Render chart
let result = renderer.render_chart(&chart_spec, &data).await?;

// Stop profiling and get metrics
let metrics = profiler.stop_profiling("chart_rendering")?;

println!("Render time: {}ms", metrics.render_time_ms);
println!("FPS: {}", metrics.fps);
println!("Memory usage: {}MB", metrics.memory_usage_mb);
```

### SIMD Optimization

High-performance data processing:

```rust
use leptos_helios::performance::SimdDataProcessor;

let processor = SimdDataProcessor::new(1024, true);
let processed_data = processor.process_data_points(&raw_data)?;
```

### Memory Management

Advanced memory pooling:

```rust
use leptos_helios::performance::AdvancedMemoryPool;

let memory_pool = AdvancedMemoryPool::new(100_000_000); // 100MB
let buffer = memory_pool.allocate_buffer(1024)?;
// Use buffer...
memory_pool.deallocate_buffer(buffer);
```

## Plugin System

### Creating Plugins

Extend functionality with plugins:

```rust
use leptos_helios::plugin_system::{Plugin, PluginMetadata, PluginCapabilities, ChartPlugin};

struct CustomChartPlugin {
    metadata: PluginMetadata,
}

impl Plugin for CustomChartPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities {
            chart_types: vec!["custom_chart".to_string()],
            data_sources: vec![],
            transforms: vec![],
            exports: vec![],
            ml_features: vec![],
            themes: vec![],
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ChartPlugin for CustomChartPlugin {
    fn create_chart(&self, spec: &ChartSpec, data: &DataFrame) -> Result<Vec<u8>, PluginError> {
        // Custom chart implementation
        Ok(vec![])
    }
}
```

### Plugin Management

Manage and load plugins:

```rust
use leptos_helios::plugin_system::{PluginManager, PluginRegistry};

let mut plugin_manager = PluginManager::new();
let plugin_registry = PluginRegistry::new();

// Register plugin
let custom_plugin = CustomChartPlugin::new();
plugin_manager.register_chart_plugin(Box::new(custom_plugin)).await?;

// Load plugin from file
plugin_manager.load_plugin_from_file("path/to/plugin.wasm").await?;

// Get available plugins
let available_plugins = plugin_manager.get_available_plugins().await?;
```

## Headless Rendering

### Server-Side Rendering

Generate charts on the server:

```rust
use leptos_helios::headless_renderer::{HeadlessRenderer, HeadlessConfig, HeadlessService};

let config = HeadlessConfig {
    max_concurrent_renders: 10,
    render_timeout: Duration::from_secs(30),
    memory_limit_mb: 512,
    enable_caching: true,
    cache_size_mb: 100,
};

let renderer = HeadlessRenderer::new(config);
let service = HeadlessService::new(renderer, 10, 0);

// Render chart to PNG
let png_data = service.render_to_png(&chart_spec, &data).await?;

// Render chart to SVG
let svg_data = service.render_to_svg(&chart_spec, &data).await?;

// Render chart to PDF
let pdf_data = service.render_to_pdf(&chart_spec, &data).await?;
```

### Batch Processing

Process multiple charts efficiently:

```rust
use leptos_helios::headless_renderer::HeadlessService;

let service = HeadlessService::new(renderer, 10, 0);

let chart_requests = vec![
    (chart_spec_1, data_1),
    (chart_spec_2, data_2),
    (chart_spec_3, data_3),
];

let results = service.render_batch(chart_requests).await?;
for (i, result) in results.iter().enumerate() {
    match result {
        Ok(png_data) => {
            std::fs::write(format!("chart_{}.png", i), png_data)?;
        }
        Err(e) => {
            eprintln!("Failed to render chart {}: {}", i, e);
        }
    }
}
```

## Error Handling

### Common Error Types

```rust
use leptos_helios::errors::*;

match result {
    Ok(data) => {
        // Handle success
    }
    Err(HeliosError::RenderingError(e)) => {
        eprintln!("Rendering failed: {}", e);
    }
    Err(HeliosError::DataError(e)) => {
        eprintln!("Data processing failed: {}", e);
    }
    Err(HeliosError::SecurityError(e)) => {
        eprintln!("Security violation: {}", e);
    }
    Err(HeliosError::AccessibilityError(e)) => {
        eprintln!("Accessibility issue: {}", e);
    }
    Err(e) => {
        eprintln!("Unexpected error: {}", e);
    }
}
```

## Best Practices

### Performance Optimization

1. **Use WebGPU when available**: Always check for WebGPU support and fall back gracefully
2. **Implement data virtualization**: For large datasets, use level-of-detail rendering
3. **Enable caching**: Use the built-in caching mechanisms for repeated operations
4. **Monitor performance**: Use the performance monitoring tools to identify bottlenecks

### Security

1. **Classify data appropriately**: Use the data governance system to classify sensitive data
2. **Implement proper authentication**: Use OAuth2 or SAML for enterprise authentication
3. **Enable audit logging**: Track all data access and modifications
4. **Validate exports**: Check compliance before allowing data exports

### Accessibility

1. **Validate WCAG compliance**: Use the built-in compliance checker
2. **Provide alternative formats**: Generate data tables and text descriptions
3. **Support keyboard navigation**: Ensure all interactions are keyboard accessible
4. **Test with screen readers**: Validate alt text and ARIA labels

### Data Management

1. **Use appropriate data sources**: Choose the right database adapter for your needs
2. **Implement connection pooling**: Use the built-in connection pooling for better performance
3. **Handle errors gracefully**: Implement proper error handling for data operations
4. **Monitor data lineage**: Track data transformations and dependencies

## Examples

### Complete Example: Interactive Dashboard

```rust
use leptos_helios::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize security
    let security_config = SecurityConfig::new(
        Box::new(OAuth2Provider::new(/* ... */)),
        Box::new(SAMLProvider::new(/* ... */)),
        Box::new(RBACProvider::new()),
    );

    // Initialize data governance
    let governance = DataGovernance::new();
    governance.classify_data("sales_data", DataClassification::Internal).await?;

    // Connect to database
    let postgres = PostgresAdapter::new(ConnectionConfig {
        connection_string: "postgresql://user:pass@localhost/sales".to_string(), // pragma: allowlist secret
        query_timeout: Some(30),
        ssl_mode: SslMode::Require,
        connection_timeout: Some(10),
        credentials: Some(Credentials::new("user", "pass")),
        max_connections: Some(10),
    }).await?;

    // Fetch data
    let connection = postgres.connect().await?;
    let result = connection.query("SELECT * FROM sales_data WHERE date >= NOW() - INTERVAL '30 days'").await?;
    let data = result.to_dataframe()?;

    // Create chart specification
    let chart_spec = ChartSpec {
        transform: vec![],
        selection: vec![],
        intelligence: None,
        config: ChartConfig::default(),
        data: DataReference::Static(data.clone()),
        encoding: Encoding {
            x: Some(Channel::Temporal(Field::new("date"))),
            y: Some(Channel::Quantitative(Field::new("sales"))),
            color: Some(Channel::Nominal(Field::new("region"))),
        },
    };

    // Initialize renderer
    let renderer = WebGpuRenderer::new()?;

    // Initialize accessibility
    let accessibility_config = AccessibilityConfig::default();
    let performance_config = PerformanceConfig::default();
    let accessibility_system = AccessibilitySystem::new(accessibility_config, performance_config);

    // Validate compliance
    let compliance_result = accessibility_system
        .validate_wcag_compliance(&chart_spec, &data)?;

    if !compliance_result.is_compliant {
        eprintln!("Accessibility compliance issues found");
        for violation in &compliance_result.violations {
            eprintln!("- {}", violation);
        }
    }

    // Render chart
    let chart_result = renderer.render_chart(&chart_spec, &data).await?;

    // Export chart
    let export_system = ExportSystem::new()?;
    let png_data = export_system.export_to_png(
        &chart_spec,
        &data,
        ExportConfig {
            width: 800,
            height: 600,
            dpi: 300,
            background: Some(Color::White),
        }
    ).await?;

    // Save to file
    std::fs::write("sales_dashboard.png", png_data)?;

    println!("Dashboard generated successfully!");
    Ok(())
}
```

This comprehensive API reference provides everything needed to build enterprise-grade data visualizations with Leptos Helios. For more specific examples and advanced usage patterns, see the [Tutorials](TUTORIALS.md) and [Examples](examples/) directories.
