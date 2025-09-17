# Leptos Helios Tutorials

## Table of Contents

- [Getting Started](#getting-started)
- [Basic Chart Creation](#basic-chart-creation)
- [Advanced Charting](#advanced-charting)
- [Data Integration](#data-integration)
- [Enterprise Features](#enterprise-features)
- [Performance Optimization](#performance-optimization)
- [Accessibility Implementation](#accessibility-implementation)
- [Security & Compliance](#security--compliance)
- [Deployment Guide](#deployment-guide)

## Getting Started

### Installation

Add Leptos Helios to your `Cargo.toml`:

```toml
[dependencies]
leptos-helios = "0.3.0"
polars = "0.40"
tokio = { version = "1.0", features = ["full"] }
```

### Hello World

Create your first chart:

```rust
use leptos_helios::*;
use polars::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create sample data
    let data = df! [
        "x" => [1, 2, 3, 4, 5],
        "y" => [2, 4, 6, 8, 10]
    ].unwrap();

    // Create chart specification
    let chart_spec = ChartSpec {
        transform: vec![],
        selection: vec![],
        intelligence: None,
        config: ChartConfig::default(),
        data: DataReference::Static(data),
        encoding: Encoding {
            x: Some(Channel::Quantitative(Field::new("x"))),
            y: Some(Channel::Quantitative(Field::new("y"))),
        },
    };

    // Render chart
    let renderer = Canvas2DRenderer::new();
    let result = renderer.render_chart(&chart_spec, &data)?;

    println!("Chart rendered successfully!");
    Ok(())
}
```

## Basic Chart Creation

### Bar Charts

```rust
use leptos_helios::chart::{ChartSpec, MarkType, Encoding, Channel, Field};

let chart_spec = ChartSpec {
    transform: vec![],
    selection: vec![],
    intelligence: None,
    config: ChartConfig::default(),
    data: DataReference::Static(data),
    encoding: Encoding {
        x: Some(Channel::Nominal(Field::new("category"))),
        y: Some(Channel::Quantitative(Field::new("value"))),
    },
};

// Customize bar appearance
let bar_mark = MarkType::Bar {
    width: Some(30.0),
    corner_radius: Some(5.0),
};
```

### Line Charts

```rust
let line_chart_spec = ChartSpec {
    transform: vec![],
    selection: vec![],
    intelligence: None,
    config: ChartConfig::default(),
    data: DataReference::Static(data),
    encoding: Encoding {
        x: Some(Channel::Temporal(Field::new("date"))),
        y: Some(Channel::Quantitative(Field::new("sales"))),
        color: Some(Channel::Nominal(Field::new("product"))),
    },
};

let line_mark = MarkType::Line {
    interpolate: Some(Interpolation::Smooth),
};
```

### Scatter Plots

```rust
let scatter_spec = ChartSpec {
    transform: vec![],
    selection: vec![],
    intelligence: None,
    config: ChartConfig::default(),
    data: DataReference::Static(data),
    encoding: Encoding {
        x: Some(Channel::Quantitative(Field::new("price"))),
        y: Some(Channel::Quantitative(Field::new("sales"))),
        size: Some(Channel::Quantitative(Field::new("volume"))),
        color: Some(Channel::Nominal(Field::new("category"))),
    },
};

let scatter_mark = MarkType::Scatter {
    size: Some(8.0),
    opacity: Some(0.7),
};
```

## Advanced Charting

### Multi-Series Charts

```rust
// Create data with multiple series
let data = df! [
    "date" => ["2024-01-01", "2024-01-02", "2024-01-03", "2024-01-04", "2024-01-05"],
    "sales" => [100, 120, 110, 130, 125],
    "profit" => [20, 25, 22, 28, 26],
    "region" => ["North", "North", "South", "South", "North"]
].unwrap();

let multi_series_spec = ChartSpec {
    transform: vec![],
    selection: vec![],
    intelligence: None,
    config: ChartConfig::default(),
    data: DataReference::Static(data),
    encoding: Encoding {
        x: Some(Channel::Temporal(Field::new("date"))),
        y: Some(Channel::Quantitative(Field::new("sales"))),
        color: Some(Channel::Nominal(Field::new("region"))),
    },
};
```

### Interactive Charts

```rust
use leptos_helios::interactions::{InteractionSystem, InteractionConfig};

let interaction_config = InteractionConfig {
    enable_zoom: true,
    enable_pan: true,
    enable_tooltip: true,
    enable_brush: true,
    enable_crosshair: true,
    zoom_sensitivity: 1.2,
    pan_sensitivity: 1.0,
    tooltip_delay: 300,
};

let interaction_system = InteractionSystem::new(interaction_config);

// Add interactions to chart
let interactive_spec = ChartSpec {
    transform: vec![],
    selection: vec![
        Selection::Interval {
            name: "brush".to_string(),
            encodings: vec!["x".to_string()],
            on: "mousedown".to_string(),
            translate: "mousedown".to_string(),
            zoom: "wheel".to_string(),
        }
    ],
    intelligence: None,
    config: ChartConfig::default(),
    data: DataReference::Static(data),
    encoding: Encoding {
        x: Some(Channel::Temporal(Field::new("date"))),
        y: Some(Channel::Quantitative(Field::new("value"))),
    },
};
```

### Custom Styling

```rust
use leptos_helios::styling::{StyleConfig, ColorScheme, Typography};

let style_config = StyleConfig {
    color_scheme: ColorScheme::Dark,
    typography: Typography {
        font_family: "Inter, sans-serif".to_string(),
        font_size: 14,
        font_weight: "normal".to_string(),
    },
    spacing: SpacingConfig {
        padding: 20,
        margin: 10,
        gap: 5,
    },
    colors: ColorConfig {
        primary: "#1f77b4".to_string(),
        secondary: "#ff7f0e".to_string(),
        background: "#ffffff".to_string(),
        text: "#333333".to_string(),
    },
};

let styled_spec = ChartSpec {
    transform: vec![],
    selection: vec![],
    intelligence: None,
    config: ChartConfig {
        style: Some(style_config),
        ..Default::default()
    },
    data: DataReference::Static(data),
    encoding: Encoding::default(),
};
```

## Data Integration

### Database Connections

#### PostgreSQL

```rust
use leptos_helios::data_sources::{PostgresAdapter, ConnectionConfig, SslMode, Credentials};

let config = ConnectionConfig {
    connection_string: "postgresql://user:password@localhost:5432/database".to_string(), // pragma: allowlist secret
    query_timeout: Some(30),
    ssl_mode: SslMode::Require,
    connection_timeout: Some(10),
    credentials: Some(Credentials::new("user", "password")),
    max_connections: Some(10),
};

let postgres = PostgresAdapter::new(config).await?;
let connection = postgres.connect().await?;

// Execute query
let result = connection.query("SELECT * FROM sales_data WHERE date >= $1")
    .bind(chrono::Utc::now() - chrono::Duration::days(30))
    .await?;

let dataframe = result.to_dataframe()?;
```

#### ClickHouse

```rust
use leptos_helios::data_sources::ClickHouseAdapter;

let config = ConnectionConfig {
    connection_string: "clickhouse://localhost:9000/database".to_string(),
    query_timeout: Some(60),
    ssl_mode: SslMode::Disable,
    connection_timeout: Some(15),
    credentials: Some(Credentials::new("user", "password")),
    max_connections: Some(20),
};

let clickhouse = ClickHouseAdapter::new(config).await?;
let connection = clickhouse.connect().await?;

// Execute query
let result = connection.query("SELECT * FROM events WHERE timestamp >= now() - INTERVAL 1 DAY").await?;
let dataframe = result.to_dataframe()?;
```

### Data Streaming

```rust
use leptos_helios::streaming::{StreamingDataSource, WebSocketConfig};

let ws_config = WebSocketConfig {
    url: "ws://localhost:8080/stream".to_string(),
    reconnect_interval: Duration::from_secs(5),
    max_reconnect_attempts: 10,
    heartbeat_interval: Duration::from_secs(30),
};

let streaming_source = StreamingDataSource::new(ws_config);
let mut stream = streaming_source.connect().await?;

// Process streaming data
while let Some(data_chunk) = stream.next().await {
    match data_chunk {
        Ok(data) => {
            // Update chart with new data
            update_chart(&data).await?;
        }
        Err(e) => {
            eprintln!("Streaming error: {}", e);
        }
    }
}
```

### Data Transformation

```rust
use leptos_helios::chart::{Transform, TransformType, Field};

let transform_spec = ChartSpec {
    transform: vec![
        Transform {
            transform_type: TransformType::Filter,
            field: Some(Field::new("value")),
            predicate: Some("value > 100".to_string()),
            parameters: HashMap::new(),
        },
        Transform {
            transform_type: TransformType::Aggregate,
            field: Some(Field::new("sales")),
            predicate: None,
            parameters: {
                let mut params = HashMap::new();
                params.insert("groupby".to_string(), "region".to_string());
                params.insert("aggregate".to_string(), "sum".to_string());
                params
            },
        },
    ],
    selection: vec![],
    intelligence: None,
    config: ChartConfig::default(),
    data: DataReference::Static(data),
    encoding: Encoding::default(),
};
```

## Enterprise Features

### Authentication & Authorization

#### OAuth2 Integration

```rust
use leptos_helios::security::{SecurityConfig, OAuth2Provider, RBACProvider};

// Configure OAuth2
let oauth2_provider = OAuth2Provider::new(
    "your_client_id".to_string(),
    "your_client_secret".to_string(),
    "https://auth.example.com/authorize".to_string(),
    "https://auth.example.com/token".to_string(),
    "https://auth.example.com/userinfo".to_string(),
    vec!["read", "write"].iter().map(|s| s.to_string()).collect(),
);

// Configure RBAC
let rbac_provider = RBACProvider::new();
rbac_provider.create_role("admin", vec!["read", "write", "delete"]).await?;
rbac_provider.create_role("analyst", vec!["read"]).await?;
rbac_provider.create_role("viewer", vec!["read"]).await?;

// Assign roles to users
rbac_provider.assign_role_to_user("user123", "admin").await?;
rbac_provider.assign_role_to_user("user456", "analyst").await?;

let security_config = SecurityConfig::new(
    Box::new(oauth2_provider),
    Box::new(SAMLProvider::new(/* ... */)),
    Box::new(rbac_provider),
);
```

#### SAML Integration

```rust
use leptos_helios::security::SAMLProvider;

let saml_provider = SAMLProvider::new(
    "https://saml.example.com/metadata".to_string(),
    "your_entity_id".to_string(),
    "your_x509_cert".to_string(),
    "your_private_key".to_string(),
);

// Generate SAML request
let saml_request = saml_provider.generate_saml_request("relay_state")?;

// Parse SAML response
let auth_result = saml_provider.parse_saml_response(&saml_response).await?;
```

### Data Governance

```rust
use leptos_helios::security::{DataGovernance, DataClassification, DataPolicy, PrivacyRule, PrivacyRuleType};

let governance = DataGovernance::new();

// Classify data
governance.classify_data("customer_pii", DataClassification::Confidential).await?;
governance.classify_data("public_analytics", DataClassification::Public).await?;

// Add data policies
let confidential_policy = DataPolicy {
    id: "confidential_policy".to_string(),
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

governance.add_data_policy(confidential_policy).await?;

// Add privacy rules
let gdpr_rule = PrivacyRule {
    id: "gdpr_rule".to_string(),
    name: "GDPR Data Minimization".to_string(),
    rule_type: PrivacyRuleType::DataMinimization,
    conditions: vec![],
    actions: vec![PrivacyAction::Anonymize, PrivacyAction::LogAccess],
    jurisdiction: "EU".to_string(),
    framework: "GDPR".to_string(),
};

governance.add_privacy_rule(gdpr_rule).await?;
```

### Audit Logging

```rust
use leptos_helios::security::{AuditLogger, AuditEventType, AuditResult};

let audit_logger = AuditLogger::new()
    .with_retention_days(90)
    .with_max_log_size(100_000)
    .with_real_time_alerts(true)
    .with_alert_threshold(AuditEventType::SecurityViolation, 5);

// Log authentication events
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

// Log security violations
audit_logger.log_security_violation(
    "user456",
    "unauthorized_access_attempt",
    "Attempted to access restricted data without proper authorization",
    AuditResult::Failure,
).await?;

// Get audit statistics
let stats = audit_logger.get_audit_statistics().await?;
println!("Total events: {}", stats.total_events);
println!("Security violations: {}", stats.security_violations);
println!("Failed authentications: {}", stats.failed_authentications);
```

## Performance Optimization

### WebGPU Rendering

```rust
use leptos_helios::webgpu_renderer::WebGpuRenderer;

// Check WebGPU support
if WebGpuRenderer::is_supported() {
    let renderer = WebGpuRenderer::new()?;

    // Configure for high performance
    let config = WebGpuConfig {
        power_preference: PowerPreference::HighPerformance,
        antialias: true,
        alpha_mode: AlphaMode::Premultiplied,
        format: TextureFormat::Bgra8Unorm,
    };

    let result = renderer.render_chart_with_config(&chart_spec, &data, config).await?;
} else {
    // Fallback to Canvas2D
    let renderer = Canvas2DRenderer::new();
    let result = renderer.render_chart(&chart_spec, &data)?;
}
```

### Data Virtualization

```rust
use leptos_helios::performance::{DataVirtualizer, VirtualizationConfig};

let config = VirtualizationConfig {
    viewport_size: 1000,
    buffer_size: 2000,
    update_threshold: 0.1,
};

let virtualizer = DataVirtualizer::new(config);

// Process large dataset
let large_data = load_large_dataset().await?;
let virtualized_data = virtualizer.virtualize(&large_data)?;

// Render only visible data
let visible_data = virtualizer.get_visible_data(&viewport)?;
let result = renderer.render_chart(&chart_spec, &visible_data).await?;
```

### Memory Management

```rust
use leptos_helios::performance::{AdvancedMemoryPool, MemoryConfig};

let memory_config = MemoryConfig {
    max_memory_mb: 512,
    gc_threshold: 0.8,
    gc_frequency: Duration::from_secs(30),
};

let memory_pool = AdvancedMemoryPool::new(memory_config);

// Allocate buffers efficiently
let buffer = memory_pool.allocate_buffer(1024)?;
// Use buffer...
memory_pool.deallocate_buffer(buffer);

// Monitor memory usage
let usage = memory_pool.get_memory_usage()?;
println!("Memory usage: {}MB / {}MB", usage.used_mb, usage.total_mb);
```

### Performance Monitoring

```rust
use leptos_helios::performance::{PerformanceProfiler, PerformanceMetrics};

let profiler = PerformanceProfiler::new();

// Profile chart rendering
profiler.start_profiling("chart_rendering");
let result = renderer.render_chart(&chart_spec, &data).await?;
let metrics = profiler.stop_profiling("chart_rendering")?;

println!("Render time: {}ms", metrics.render_time_ms);
println!("FPS: {}", metrics.fps);
println!("Memory usage: {}MB", metrics.memory_usage_mb);

// Check performance budget
if metrics.render_time_ms > 100.0 {
    eprintln!("Performance budget exceeded!");
}
```

## Accessibility Implementation

### WCAG 2.1 AA Compliance

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
    keyboard_nav: KeyboardNavigation {
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
    },
    color_vision: ColorVisionSupport {
        enabled: true,
        high_contrast_mode: true,
        colorblind_friendly_palette: true,
        pattern_support: true,
        texture_support: false,
        minimum_contrast_ratio: 4.5,
    },
    motion: MotionPreferences {
        respect_prefers_reduced_motion: true,
        reduce_animations: false,
        static_alternative: false,
        disable_auto_play: true,
        animation_duration_multiplier: 1.0,
    },
    focus_management: FocusManagement {
        enabled: true,
        visible_focus_indicator: true,
        focus_trap: true,
        restore_focus: true,
        focus_order: vec!["chart".to_string(), "controls".to_string(), "data_table".to_string()],
    },
    alternative_formats: AlternativeFormats {
        data_tables: true,
        text_descriptions: true,
        sonification: true,
        tactile_graphics: false,
        high_contrast_version: true,
    },
};

let performance_config = PerformanceConfig::default();
let accessibility_system = AccessibilitySystem::new(config, performance_config);

// Validate compliance
let compliance_result = accessibility_system
    .validate_wcag_compliance(&chart_spec, &data)?;

if !compliance_result.is_compliant {
    eprintln!("Accessibility compliance issues found:");
    for violation in &compliance_result.violations {
        eprintln!("- {}", violation);
    }
}

// Generate accessible content
let alt_text = accessibility_system
    .generate_alt_text(&chart_spec, &data)?;

let data_table = accessibility_system
    .create_data_table(&chart_spec, &data)?;

println!("Alt text: {}", alt_text);
println!("Data table: {} rows, {} columns", data_table.rows.len(), data_table.headers.len());
```

### Screen Reader Support

```rust
// Generate comprehensive alt text
let alt_text = accessibility_system.generate_alt_text(&chart_spec, &data)?;

// Create accessible data table
let data_table = accessibility_system.create_data_table(&chart_spec, &data)?;

// Generate ARIA labels
let aria_labels = accessibility_system.generate_aria_labels(&chart_spec)?;

// Create keyboard navigation map
let keyboard_map = accessibility_system.generate_keyboard_map(&chart_spec);
```

### Color Vision Support

```rust
// Validate color palette for accessibility
let color_palette = vec![
    "#1f77b4".to_string(), // Blue
    "#ff7f0e".to_string(), // Orange
    "#2ca02c".to_string(), // Green
    "#d62728".to_string(), // Red
];

let validation_result = accessibility_system
    .validate_color_palette(&color_palette)?;

if !validation_result.is_colorblind_friendly {
    eprintln!("Color palette is not colorblind-friendly");
    for recommendation in &validation_result.recommendations {
        eprintln!("- {}", recommendation);
    }
}

// Generate high contrast palette
let high_contrast_palette = accessibility_system
    .generate_high_contrast_palette()?;
```

## Security & Compliance

### Data Classification

```rust
use leptos_helios::security::{DataGovernance, DataClassification};

let governance = DataGovernance::new();

// Classify different types of data
governance.classify_data("customer_pii", DataClassification::Confidential).await?;
governance.classify_data("financial_records", DataClassification::Restricted).await?;
governance.classify_data("public_analytics", DataClassification::Public).await?;
governance.classify_data("internal_reports", DataClassification::Internal).await?;
governance.classify_data("classified_intel", DataClassification::TopSecret).await?;
```

### Export Compliance

```rust
// Check if export is compliant
let is_compliant = governance
    .check_export_compliance("customer_pii", "encrypted_pdf", &user)
    .await?;

if !is_compliant {
    return Err("Export not compliant with data policy".into());
}

// Proceed with export
let export_system = ExportSystem::new()?;
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
```

### Risk Assessment

```rust
// Conduct risk assessment
let risk_assessment = governance
    .conduct_risk_assessment("customer_pii".to_string(), "risk_assessor".to_string())
    .await?;

println!("Risk level: {:?}", risk_assessment.risk_level);
println!("Identified risks: {}", risk_assessment.identified_risks.len());
println!("Mitigation measures: {}", risk_assessment.mitigation_measures.len());

// Check if additional controls are needed
if matches!(risk_assessment.risk_level, RiskLevel::High | RiskLevel::Critical) {
    println!("High risk data - additional controls required");
}
```

## Deployment Guide

### Docker Deployment

Create a `Dockerfile`:

```dockerfile
FROM rust:1.70-slim as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/leptos-helios /usr/local/bin/
EXPOSE 8080

CMD ["leptos-helios"]
```

### Kubernetes Deployment

Create `k8s-deployment.yaml`:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: leptos-helios
spec:
  replicas: 3
  selector:
    matchLabels:
      app: leptos-helios
  template:
    metadata:
      labels:
        app: leptos-helios
    spec:
      containers:
      - name: leptos-helios
        image: leptos-helios:latest
        ports:
        - containerPort: 8080
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: db-secret
              key: url
        - name: REDIS_URL
          valueFrom:
            secretKeyRef:
              name: redis-secret
              key: url
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
---
apiVersion: v1
kind: Service
metadata:
  name: leptos-helios-service
spec:
  selector:
    app: leptos-helios
  ports:
  - port: 80
    targetPort: 8080
  type: LoadBalancer
```

### Environment Configuration

Create `.env` file:

```env
# Database Configuration
DATABASE_URL=postgresql://user:password@localhost:5432/database # pragma: allowlist secret
REDIS_URL=redis://localhost:6379

# Security Configuration
JWT_SECRET=your-jwt-secret
OAUTH2_CLIENT_ID=your-client-id
OAUTH2_CLIENT_SECRET=your-client-secret
SAML_ENTITY_ID=your-entity-id

# Performance Configuration
MAX_CONCURRENT_RENDERS=10
MEMORY_LIMIT_MB=512
RENDER_TIMEOUT_SECONDS=30

# Monitoring Configuration
ENABLE_METRICS=true
METRICS_PORT=9090
LOG_LEVEL=info
```

### Production Checklist

- [ ] **Security**
  - [ ] Enable HTTPS/TLS
  - [ ] Configure authentication (OAuth2/SAML)
  - [ ] Set up RBAC policies
  - [ ] Enable audit logging
  - [ ] Configure data classification

- [ ] **Performance**
  - [ ] Enable WebGPU rendering
  - [ ] Configure memory pooling
  - [ ] Set up performance monitoring
  - [ ] Enable caching
  - [ ] Configure load balancing

- [ ] **Accessibility**
  - [ ] Validate WCAG 2.1 AA compliance
  - [ ] Enable screen reader support
  - [ ] Configure keyboard navigation
  - [ ] Test with assistive technologies

- [ ] **Monitoring**
  - [ ] Set up health checks
  - [ ] Configure metrics collection
  - [ ] Enable log aggregation
  - [ ] Set up alerting

- [ ] **Backup & Recovery**
  - [ ] Configure database backups
  - [ ] Set up disaster recovery
  - [ ] Test backup restoration

This comprehensive tutorial guide provides everything needed to build, deploy, and maintain enterprise-grade data visualizations with Leptos Helios. For more specific use cases and advanced patterns, see the [Examples](examples/) directory and [API Reference](API_REFERENCE.md).
