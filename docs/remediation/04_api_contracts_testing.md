# API Contracts & Contract Testing Implementation

**Priority:** HIGH  
**Timeline:** Weeks 3-6  
**Current Status:** No contract testing, no API stability guarantees  
**Target:** Full contract coverage for public APIs with semver compliance

## Current API Surface Analysis

### Public API Inventory
```rust
// helios-core/src/lib.rs - Current public exports
pub use chart::{ChartSpec, MarkType, Encoding, DataReference};
pub use rendering::{Renderer, WebGpuRenderer, Canvas2DRenderer};
pub use export_system::{ExportSystem, ExportFormat, ExportConfig};
pub use data_sources::{DataSource, DataSourceManager};
pub use accessibility::{AccessibilityConfig, ScreenReaderSupport};
```

### API Stability Concerns
- ❌ No semantic versioning policy documented
- ❌ No deprecation strategy for breaking changes  
- ❌ Multiple crates claiming same version without coordination
- ❌ Public APIs return `unimplemented!()` or panic
- ❌ No TypeScript bindings for WASM exports

## Phase 1: API Contract Definition (Week 3)

### 1.1 Semantic Versioning Policy
```toml
# Cargo.toml versioning strategy
[workspace.package]
version = "0.7.1"  # Current: pre-1.0, breaking changes allowed

# Crate stability tiers
[package.metadata.helios]
stability = "experimental"  # experimental | beta | stable
breaking-changes-allowed = true
```

**Versioning Rules:**
- `0.x.y` - Breaking changes allowed in minor versions
- `1.x.y` - SemVer compliance required, only additive changes
- Pre-1.0: Only `helios-core` guaranteed stable within patch versions

### 1.2 API Contract Traits
```rust
// src/contracts/mod.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Contract definition for all renderer implementations
pub trait RendererContract: Send + Sync {
    /// Contract: Must return Ok() for valid ChartSpec + DataFrame
    /// Contract: Must return specific error types for invalid inputs
    /// Contract: Must complete within 5 seconds for <10K points
    fn render_chart(
        &self,
        spec: &ChartSpec,
        data: &DataFrame,
    ) -> Result<RenderedChart, RenderError>;
    
    /// Contract: Must support PNG, SVG formats at minimum
    fn supported_formats(&self) -> &[ExportFormat];
    
    /// Contract: Must be idempotent - same inputs = same outputs
    fn export_chart(
        &self,
        chart: &RenderedChart,
        format: ExportFormat,
        config: &ExportConfig,
    ) -> Result<Vec<u8>, ExportError>;
}

/// Contract for data source implementations  
pub trait DataSourceContract: Send + Sync {
    /// Contract: Must validate connection before returning Ok()
    async fn connect(&mut self) -> Result<(), DataSourceError>;
    
    /// Contract: Must return consistent schema for same query
    async fn execute_query(&self, query: &str) -> Result<DataFrame, DataSourceError>;
    
    /// Contract: Must handle connection drops gracefully
    async fn health_check(&self) -> Result<ConnectionStatus, DataSourceError>;
}
```

### 1.3 Contract Test Framework
```rust
// tests/contracts/renderer_contract.rs
use helios_core::contracts::*;
use rstest::*;

pub struct RendererContractTester<T: RendererContract> {
    renderer: T,
}

impl<T: RendererContract> RendererContractTester<T> {
    pub fn new(renderer: T) -> Self {
        Self { renderer }
    }
    
    /// Test all renderer contract requirements
    pub fn test_full_contract(&self) -> Result<(), ContractViolation> {
        self.test_basic_render()?;
        self.test_error_handling()?;
        self.test_performance_contract()?;
        self.test_export_formats()?;
        Ok(())
    }
    
    fn test_basic_render(&self) -> Result<(), ContractViolation> {
        let spec = sample_chart_spec();
        let data = sample_dataframe();
        
        let result = self.renderer.render_chart(&spec, &data);
        
        match result {
            Ok(chart) => {
                // Contract: rendered chart must have dimensions
                if chart.width() == 0 || chart.height() == 0 {
                    return Err(ContractViolation::InvalidDimensions);
                }
                Ok(())
            }
            Err(e) => Err(ContractViolation::UnexpectedError(e.to_string()))
        }
    }
    
    fn test_performance_contract(&self) -> Result<(), ContractViolation> {
        let large_data = generate_10k_dataframe();
        let start = std::time::Instant::now();
        
        let _result = self.renderer.render_chart(&sample_chart_spec(), &large_data);
        
        if start.elapsed() > std::time::Duration::from_secs(5) {
            return Err(ContractViolation::PerformanceViolation {
                expected_max_ms: 5000,
                actual_ms: start.elapsed().as_millis() as u64,
            });
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ContractViolation {
    InvalidDimensions,
    PerformanceViolation { expected_max_ms: u64, actual_ms: u64 },
    UnexpectedError(String),
    FormatNotSupported(ExportFormat),
    InconsistentBehavior(String),
}
```

## Phase 2: Implementation Testing (Week 4-5)

### 2.1 Renderer Contract Tests
```rust
// Test all renderer implementations against same contract
#[rstest]
#[case::webgpu(WebGpuRenderer::new())]
#[case::canvas2d(Canvas2DRenderer::new())]  
#[case::headless(HeadlessRenderer::new())]
fn test_renderer_contracts<R: RendererContract>(#[case] renderer: R) {
    let tester = RendererContractTester::new(renderer);
    tester.test_full_contract().expect("Renderer contract violation");
}
```

### 2.2 Data Source Contract Tests
```rust
#[rstest]
#[case::postgres(PostgresDataSource::new("test_url"))]
#[case::clickhouse(ClickHouseDataSource::new("test_url"))]
#[case::csv(CsvDataSource::new("test.csv"))]
fn test_data_source_contracts<D: DataSourceContract>(#[case] mut source: D) {
    let tester = DataSourceContractTester::new(source);
    tester.test_full_contract().expect("Data source contract violation");
}
```

### 2.3 WASM Contract Testing
```rust
// tests/contracts/wasm_contract.rs
#[wasm_bindgen_test]
async fn test_wasm_api_contract() {
    let chart = JsChart::new();
    
    // Contract: JS API must match Rust API semantics
    let result = chart.render_with_data(sample_js_data()).await;
    assert!(result.is_ok(), "WASM contract: render should succeed with valid data");
    
    // Contract: Error handling must be consistent
    let invalid_result = chart.render_with_data(invalid_js_data()).await;
    assert!(invalid_result.is_err(), "WASM contract: should error on invalid data");
}
```

## Phase 3: Breaking Change Detection (Week 5)

### 3.1 Cargo Semver Checks Integration
```toml
[dev-dependencies]
cargo-semver-checks = "0.35"
```

```yaml
# .github/workflows/semver-check.yml
name: Semver Check
on: [pull_request]

jobs:
  semver-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - name: Check semver compatibility
        run: |
          cargo semver-checks check-release
```

### 3.2 API Surface Documentation
```rust
// Generate API documentation with contract info
/// # Contract Guarantees  
/// - Always returns `Ok(())` for valid inputs
/// - Completes within 100ms for datasets <1K rows
/// - Thread-safe and can be called concurrently
/// - Idempotent: same inputs produce identical outputs
///
/// # Breaking Change Policy
/// This API follows semver: breaking changes only in major versions
///
/// # Example
/// ```rust
/// let renderer = WebGpuRenderer::new();
/// let result = renderer.render_chart(&spec, &data)?;
/// ```
impl RendererContract for WebGpuRenderer {
    fn render_chart(&self, spec: &ChartSpec, data: &DataFrame) -> Result<RenderedChart, RenderError> {
        // Implementation...
    }
}
```

### 3.3 TypeScript Contract Generation
```rust
// Generate TypeScript definitions from Rust contracts
#[wasm_bindgen(typescript_custom_section)]
const CONTRACT_TYPES: &'static str = r#"
export interface RendererContract {
  renderChart(spec: ChartSpec, data: DataFrame): Promise<RenderedChart>;
  supportedFormats(): ExportFormat[];
  exportChart(chart: RenderedChart, format: ExportFormat): Promise<Uint8Array>;
}
"#;
```

## Phase 4: Contract Evolution Strategy (Week 6)

### 4.1 Deprecation Workflow
```rust
// Deprecation attributes with timeline
#[deprecated(
    since = "0.8.0",
    note = "Use `render_chart_async` instead. Will be removed in 1.0.0"
)]
pub fn render_chart_sync(&self, spec: &ChartSpec) -> Result<RenderedChart, RenderError> {
    // Legacy implementation
}

/// Replacement API with improved async support
#[since("0.8.0")]
pub async fn render_chart_async(&self, spec: &ChartSpec) -> Result<RenderedChart, RenderError> {
    // New implementation
}
```

### 4.2 Feature Flag Strategy
```rust
// Use feature flags for experimental APIs
#[cfg(feature = "experimental")]
pub mod experimental {
    /// Experimental 3D rendering API
    /// 
    /// # Stability Warning
    /// This API is experimental and may change without notice.
    /// Not covered by semver guarantees until stabilized.
    pub trait Renderer3D {
        fn render_3d(&self, spec: &Chart3DSpec) -> Result<Rendered3DChart, RenderError>;
    }
}
```

### 4.3 Contract Documentation Automation
```rust
// Generate contract documentation automatically  
#[derive(ContractDoc)]
pub struct ChartSpec {
    /// Contract: Must be non-empty
    #[contract(non_empty)]
    pub data: Vec<DataPoint>,
    
    /// Contract: Must be a valid mark type
    #[contract(valid_enum)]
    pub mark: MarkType,
}
```

## Testing Infrastructure

### Contract Test Suite Structure
```
tests/contracts/
├── mod.rs                 # Contract test framework
├── renderer_contract.rs   # All renderer implementations
├── data_source_contract.rs # All data source implementations  
├── export_contract.rs     # Export format contracts
├── wasm_contract.rs       # WASM binding contracts
└── performance_contract.rs # Performance requirement tests
```

### Automated Contract Validation
```yaml
# Run contract tests on every PR
- name: Contract Tests
  run: |
    cargo test contracts:: --release
    cargo test --target wasm32-unknown-unknown contracts::wasm
```

## Risk Management

### Contract Evolution Risks
**Risk**: Breaking changes slip through semver checks  
**Mitigation**: Require manual approval for any API changes, comprehensive contract tests

**Risk**: Performance regressions break contracts  
**Mitigation**: Automated performance testing in CI, strict SLA enforcement

**Risk**: WASM/Rust API drift  
**Mitigation**: Auto-generated TypeScript bindings, cross-language contract tests

## Success Metrics

### Week 3 Targets
- [ ] All public APIs have documented contracts
- [ ] Contract test framework implemented
- [ ] Semver checking configured in CI

### Week 4 Targets  
- [ ] All renderer implementations pass contract tests
- [ ] Data source contract tests implemented
- [ ] Performance contracts validated

### Week 5 Targets
- [ ] Breaking change detection functional
- [ ] TypeScript contract generation working
- [ ] API documentation includes contract guarantees

### Week 6 Targets
- [ ] Contract evolution strategy documented
- [ ] Deprecation workflow established  
- [ ] Feature flag system for experimental APIs

**Next Action**: Begin with API contract trait definitions and basic contract test framework setup.
