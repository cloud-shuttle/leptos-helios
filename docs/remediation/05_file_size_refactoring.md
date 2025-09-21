# File Size Refactoring Plan - Breaking Down Large Files

**Priority:** MEDIUM  
**Timeline:** Weeks 6-8  
**Current Status:** 58 files >300 lines, largest at 1,177 lines  
**Target:** All files <300 lines, enforced by CI

## Large File Analysis

### Critical Files Requiring Immediate Refactoring

| File | Lines | Category | Refactor Priority |
|------|-------|----------|------------------|
| `helios-core/tests/unit/production_readiness.rs` | 868 | Test | HIGH |
| `helios-core/tests/unit/theme_engine_tests.rs` | 624 | Test | HIGH |
| `helios-core/tests/charts/advanced_charts.rs` | 675 | Test | HIGH |
| `helios-core/tests/webgpu/webgpu_integration.rs` | 593 | Test | HIGH |
| `helios-core/src/nl_processor.rs` | 1,177 | Core | CRITICAL |
| `helios-core/src/rendering/webgpu_renderer.rs` | 1,045 | Core | CRITICAL |
| `helios-core/src/security/mod.rs` | 892 | Core | HIGH |
| `helios-core/src/data_processing/mod.rs` | 756 | Core | HIGH |

### Refactoring Strategy by File Type

#### Core Logic Files (>500 lines)
- **Split by domain responsibility** (Single Responsibility Principle)
- **Extract trait implementations** into separate files  
- **Create module hierarchies** with clear public interfaces

#### Test Files (>400 lines)  
- **Group by functionality** being tested
- **Extract test utilities** to shared modules
- **Split integration vs unit tests**

## Phase 1: Critical Core Files (Week 6)

### 1.1 nl_processor.rs Refactoring (1,177 lines → ~200 each)

**Current Structure Analysis:**
```rust
// src/nl_processor.rs - BEFORE (1,177 lines)
pub struct NLProcessor { /* ... */ }
impl NLProcessor {
    // Query parsing: ~300 lines
    // Chart type inference: ~250 lines  
    // Data type detection: ~200 lines
    // Intelligence extraction: ~300 lines
    // Output generation: ~127 lines
}
```

**Refactored Structure:**
```
src/nl_processor/
├── mod.rs                    # Public API re-exports (~50 lines)
├── query_parser.rs           # Query parsing logic (~280 lines) 
├── chart_inference.rs        # Chart type detection (~240 lines)
├── data_analyzer.rs          # Data type analysis (~190 lines)
├── intelligence_extractor.rs # ML/AI feature extraction (~290 lines)
├── output_generator.rs       # Result formatting (~120 lines)
└── types.rs                  # Shared types and enums (~80 lines)
```

**Implementation:**
```rust
// src/nl_processor/mod.rs
mod query_parser;
mod chart_inference;
mod data_analyzer;
mod intelligence_extractor;
mod output_generator;
mod types;

pub use types::*;
use query_parser::QueryParser;
use chart_inference::ChartInferenceEngine;
use data_analyzer::DataTypeAnalyzer;
use intelligence_extractor::IntelligenceExtractor;
use output_generator::OutputGenerator;

pub struct NLProcessor {
    query_parser: QueryParser,
    chart_engine: ChartInferenceEngine,
    data_analyzer: DataTypeAnalyzer,
    intelligence: IntelligenceExtractor,
    output_gen: OutputGenerator,
}

impl NLProcessor {
    pub fn new() -> Self {
        Self {
            query_parser: QueryParser::new(),
            chart_engine: ChartInferenceEngine::new(),
            data_analyzer: DataTypeAnalyzer::new(),
            intelligence: IntelligenceExtractor::new(),
            output_gen: OutputGenerator::new(),
        }
    }
    
    pub async fn process_query(&self, query: &str, data: &DataFrame) -> Result<ChartSpec, ProcessingError> {
        let parsed = self.query_parser.parse(query)?;
        let chart_type = self.chart_engine.infer_chart_type(&parsed, data)?;
        let data_types = self.data_analyzer.analyze_schema(data)?;
        let intelligence = self.intelligence.extract_features(&parsed, data)?;
        
        self.output_gen.generate_spec(chart_type, data_types, intelligence)
    }
}
```

### 1.2 WebGPU Renderer Refactoring (1,045 lines → ~250 each)

**Refactored Structure:**
```
src/rendering/webgpu/
├── mod.rs                    # Public API (~80 lines)
├── device_manager.rs         # WebGPU device/queue management (~240 lines)
├── pipeline_cache.rs         # Render pipeline caching (~220 lines)
├── buffer_manager.rs         # Buffer allocation/reuse (~180 lines)
├── shader_compiler.rs        # WGSL shader compilation (~210 lines)
├── render_commands.rs        # Command buffer generation (~200 lines)
└── surface_manager.rs        # Surface/swapchain management (~160 lines)
```

### 1.3 Security Module Refactoring (892 lines → ~200 each)

**Refactored Structure:**
```
src/security/
├── mod.rs                    # Public API (~60 lines)
├── authentication/
│   ├── mod.rs               # Auth traits (~80 lines)
│   ├── oauth2.rs            # OAuth2 implementation (~240 lines)
│   ├── saml.rs              # SAML implementation (~190 lines)
│   └── jwt.rs               # JWT handling (~160 lines)  
├── authorization/
│   ├── mod.rs               # RBAC traits (~70 lines)
│   ├── rbac.rs              # Role-based access (~220 lines)
│   └── permissions.rs       # Permission management (~150 lines)
└── audit/
    ├── mod.rs               # Audit interface (~40 lines)  
    └── logger.rs            # Audit logging (~180 lines)
```

## Phase 2: Test File Refactoring (Week 7)

### 2.1 Test Organization Strategy

**Before: Monolithic Test Files**
```rust
// tests/unit/production_readiness.rs (868 lines)
mod security_tests { /* 200 lines */ }
mod performance_tests { /* 250 lines */ }
mod accessibility_tests { /* 180 lines */ }
mod export_tests { /* 238 lines */ }
```

**After: Focused Test Modules**
```
tests/unit/
├── security/
│   ├── mod.rs
│   ├── authentication_tests.rs      (~180 lines)
│   ├── authorization_tests.rs       (~190 lines)
│   └── audit_tests.rs              (~150 lines)
├── performance/
│   ├── mod.rs  
│   ├── rendering_benchmarks.rs     (~200 lines)
│   ├── data_processing_perf.rs     (~180 lines)
│   └── memory_usage_tests.rs       (~160 lines)
├── accessibility/
│   ├── mod.rs
│   ├── wcag_compliance_tests.rs    (~170 lines)
│   └── screen_reader_tests.rs      (~140 lines)
└── exports/
    ├── mod.rs
    ├── png_export_tests.rs         (~180 lines)
    ├── svg_export_tests.rs         (~170 lines)
    └── pdf_export_tests.rs         (~160 lines)
```

### 2.2 Test Utilities Extraction

**Shared Test Infrastructure:**
```rust
// tests/common/mod.rs
pub mod fixtures;      // Sample data generation
pub mod assertions;    // Custom assertion macros
pub mod mocks;        // Mock implementations  
pub mod helpers;      // Test helper functions

// tests/common/fixtures.rs (~150 lines)
pub fn sample_dataframe() -> DataFrame { /* ... */ }
pub fn complex_chart_spec() -> ChartSpec { /* ... */ }
pub fn mock_webgpu_device() -> MockDevice { /* ... */ }
```

### 2.3 Property Test Extraction

**Before: Mixed in large files**
```rust
// Large test file containing property tests mixed with unit tests
```

**After: Dedicated property test modules**
```rust
// tests/property/
├── chart_generation.rs      # Property tests for chart specs
├── data_processing.rs       # Property tests for data pipelines  
└── rendering_consistency.rs # Property tests for render determinism
```

## Phase 3: Module Hierarchy Enforcement (Week 8)

### 3.1 CI Enforcement Tool

**Custom XTask for Line Count Validation:**
```rust
// xtask/src/line_limits.rs
use std::path::Path;
use walkdir::WalkDir;

const MAX_LINES: usize = 300;
const MAX_TEST_LINES: usize = 250; // Stricter for tests

pub fn check_file_sizes() -> Result<(), Box<dyn std::error::Error>> {
    let mut violations = Vec::new();
    
    for entry in WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        let path = entry.path();
        let line_count = count_lines(path)?;
        
        let limit = if path.to_string_lossy().contains("tests/") {
            MAX_TEST_LINES
        } else {
            MAX_LINES
        };
        
        if line_count > limit {
            violations.push((path.to_path_buf(), line_count, limit));
        }
    }
    
    if !violations.is_empty() {
        eprintln!("File size violations found:");
        for (path, actual, limit) in violations {
            eprintln!("  {} has {} lines (limit: {})", 
                     path.display(), actual, limit);
        }
        return Err("File size limits exceeded".into());
    }
    
    println!("✅ All files within size limits");
    Ok(())
}
```

### 3.2 Pre-commit Hook Integration

```bash
#!/bin/sh
# .pre-commit-hook.sh
echo "Checking file size limits..."
cargo xtask line-limits

if [ $? -ne 0 ]; then
    echo "❌ File size violations found. Please refactor large files."
    exit 1
fi

echo "✅ All files within size limits"
```

### 3.3 GitHub Actions Integration

```yaml
# .github/workflows/code-quality.yml  
name: Code Quality
on: [push, pull_request]

jobs:
  file-size-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Check file size limits
        run: cargo xtask line-limits
```

## Refactoring Best Practices

### 3.1 Module Extraction Process

1. **Identify Cohesive Units**: Group related functions/types
2. **Define Clear Interfaces**: Minimize public surface area
3. **Extract Incrementally**: One logical unit at a time
4. **Maintain Tests**: Ensure tests pass after each extraction
5. **Update Documentation**: Keep module docs current

### 3.2 API Preservation Strategy

```rust
// Preserve public API during refactoring using re-exports
// src/nl_processor.rs → src/nl_processor/mod.rs

// Public API stays the same
pub use query_parser::QueryParser;
pub use chart_inference::{ChartInferenceEngine, ChartType};
pub use data_analyzer::{DataTypeAnalyzer, InferredDataType};

// Internal modules remain private
mod query_parser;
mod chart_inference; 
mod data_analyzer;
```

### 3.3 Test Migration Strategy

```rust
// Move tests to stay with their implementation
// src/nl_processor.rs (with #[cfg(test)]) 
// → src/nl_processor/query_parser.rs (with #[cfg(test)])

#[cfg(test)]  
mod tests {
    use super::*;
    
    #[test]
    fn test_query_parsing() {
        let parser = QueryParser::new();
        // Test stays with implementation
    }
}
```

## Risk Management

### Potential Issues During Refactoring

**Risk**: Breaking internal APIs during extraction  
**Mitigation**: Maintain comprehensive test coverage, refactor incrementally

**Risk**: Import/visibility complications  
**Mitigation**: Use `pub(crate)` for internal APIs, careful re-export planning

**Risk**: Performance impact from module boundaries  
**Mitigation**: Benchmark before/after, inline critical paths if needed

## Success Criteria

### Week 6 Targets
- [ ] All critical core files (>1000 lines) refactored
- [ ] nl_processor.rs broken into 6 focused modules
- [ ] webgpu_renderer.rs restructured with clear separation

### Week 7 Targets  
- [ ] All test files <250 lines
- [ ] Test utilities extracted to common modules
- [ ] Property tests separated into dedicated modules

### Week 8 Targets
- [ ] CI enforcement of file size limits active
- [ ] Pre-commit hooks preventing large file commits
- [ ] Documentation updated to reflect new module structure
- [ ] Zero files >300 lines in entire codebase

### Long-term Maintenance
- [ ] Monthly file size audit in CI reports
- [ ] New contributor guidelines include file size limits
- [ ] Architecture documentation explains module organization rationale

**Next Action**: Begin with nl_processor.rs refactoring as it's the largest and most critical file.
