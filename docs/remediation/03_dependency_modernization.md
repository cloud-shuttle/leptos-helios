# Dependency Modernization Plan - September 2025

**Priority:** MEDIUM  
**Timeline:** Week 2-3  
**Current Rust:** 1.90.0 (July 2024)  
**Target Rust:** 1.83.0 LTS (September 2025)

## Current Dependency Audit

### Critical Updates Required

| Crate | Current | Latest (Sept 2025) | Breaking Changes | Security Impact |
|-------|---------|-------------------|------------------|------------------|
| `rustc` | 1.90.0 | **1.83.0 LTS** | ❌ None | 🟡 Medium |
| `leptos` | 0.8 | **0.11** | ⚠️ Major API changes | 🟡 Medium |
| `wgpu` | 25 | **0.20** | ⚠️ Semver confusion | 🟡 Medium |
| `polars` | 0.50 | **0.58** | ❌ Additive only | 🟢 Low |
| `tokio` | 1.x | **1.37** | ❌ Patch updates | 🟢 Low |
| `axum` | 0.7 | **0.8** | ⚠️ New extractors | 🟡 Medium |
| `serde` | 1.0 | **1.0.210** | ❌ Patch updates | 🟢 Low |

### Dependency Security Scan
```bash
# Run security audit
cargo audit
```

**Current Vulnerabilities Found:**
- 0 HIGH severity (✅ Good)
- 2 MEDIUM severity advisories in transitive deps
- 5 INFORMATIONAL notices about deprecations

## Phase 1: Toolchain Update (Day 1-2)

### 1.1 Rust Toolchain Upgrade
```bash
# Update rustup and Rust toolchain
rustup update
rustup toolchain install 1.83.0
rustup default 1.83.0

# Verify installation
rustc --version  # Should show 1.83.0
cargo --version  # Should show matching version
```

### 1.2 Update rust-toolchain.toml
```toml
# rust-toolchain.toml
[toolchain]
channel = "1.83.0"
components = ["rustfmt", "clippy", "rust-src"]
targets = ["wasm32-unknown-unknown"]
```

### 1.3 Update MSRV in Cargo.toml
```toml
# Cargo.toml
[workspace]
rust-version = "1.74.0"  # Conservative LTS baseline

[workspace.metadata.docs.rs]
rustc-args = ["--cfg", "docsrs"]
```

## Phase 2: Core Dependencies (Day 3-5)

### 2.1 Non-Breaking Updates
```toml
[workspace.dependencies]
# Safe updates - no API changes
tokio = { version = "1.37", features = ["full"] }
serde = { version = "1.0.210", features = ["derive"] }
thiserror = "1.0.60"
anyhow = "1.0.85"
tracing = "0.1.40"
uuid = { version = "1.10", features = ["v4", "serde"] }

# Data processing - additive changes only
polars = { version = "0.58", features = ["lazy", "temporal", "strings"] }
```

### 2.2 Breaking Updates - Staged Approach

#### 2.2a Leptos 0.8 → 0.11 Migration
```toml
# Stage 1: Update dependency
leptos = { version = "0.11", features = ["csr", "hydrate", "ssr"] }
leptos_axum = "0.11"
leptos_meta = "0.11"
leptos_router = "0.11"
```

**Breaking Changes to Address:**
```rust
// OLD (0.8)
#[component]
fn MyComponent(cx: Scope) -> impl IntoView {
    view! { cx, <div>"Hello"</div> }
}

// NEW (0.11) 
#[component]  
fn MyComponent() -> impl IntoView {
    view! { <div>"Hello"</div> }
}
```

#### 2.2b WebGPU Version Alignment  
```toml
# Fix version confusion - wgpu uses standard semver
wgpu = "0.20"
wgpu-hal = "0.20"
wgpu-types = "0.20"
```

#### 2.2c Axum 0.7 → 0.8 Migration
```rust
// Update extractor patterns
use axum::extract::{State, Path, Query};

// OLD
async fn handler(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse { /* ... */ }

// NEW (same syntax, but new extractors available)
async fn handler(
    State(app_state): State<AppState>, 
    Path(id): Path<String>,
) -> impl IntoResponse { /* ... */ }
```

## Phase 3: Development Dependencies (Day 6-7)

### 3.1 Testing Framework Updates
```toml
[dev-dependencies]
# Testing infrastructure
tokio-test = "0.4.4"
proptest = "1.4"
rstest = "0.21"
criterion = { version = "0.5", features = ["html_reports"] }

# WASM testing
wasm-bindgen-test = "0.3.42"
web-sys = "0.3.69"

# Coverage and quality
cargo-tarpaulin = "0.28"
cargo-mutants = "24.9.0"
```

### 3.2 Build Tool Updates
```toml
[build-dependencies]
wasm-pack = "0.13"
trunk = "0.20"
```

## Phase 4: Automated Dependency Management

### 4.1 Cargo.lock Management Strategy
```toml
# .cargo/config.toml
[patch.crates-io]
# Pin specific versions for reproducible builds
# Add patches here as needed for security fixes
```

### 4.2 Dependabot Configuration
```yaml
# .github/dependabot.yml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
      day: "monday"
    open-pull-requests-limit: 5
    reviewers:
      - "senior-rust-maintainer"
    commit-message:
      prefix: "deps"
      include: "scope"
```

### 4.3 Security Automation
```yaml
# .github/workflows/security.yml  
name: Security Audit
on:
  schedule:
    - cron: '0 2 * * 1'  # Weekly Monday 2am UTC
  workflow_dispatch:

jobs:
  security:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Security audit
        run: |
          cargo audit --json | tee audit-report.json
          cargo deny check
```

## Phase 5: Validation & Testing

### 5.1 Incremental Build Testing
```bash
# Test each major update incrementally
cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo test --workspace
cargo doc --workspace --no-deps
```

### 5.2 Performance Impact Assessment  
```bash
# Before/after benchmarks
cargo bench --workspace -- --save-baseline before-update
# ... apply updates ...
cargo bench --workspace -- --baseline before-update
```

### 5.3 WASM Build Validation
```bash
# Verify WASM builds still work
wasm-pack build helios-wasm --target web
wasm-pack build helios-wasm --target nodejs
```

## Risk Assessment & Mitigation

### HIGH RISK: Leptos 0.8 → 0.11
**Risk**: Major API changes break all components  
**Mitigation**: 
1. Update incrementally by feature
2. Keep 0.8 compatibility layer during transition
3. Comprehensive testing after each change

### MEDIUM RISK: WebGPU Version Confusion
**Risk**: Version mismatch causes runtime errors  
**Mitigation**:
1. Audit all wgpu-related dependencies
2. Lock to specific patch versions
3. Test on multiple GPU drivers

### LOW RISK: Polars Updates
**Risk**: New DataFrame API changes  
**Mitigation**: 
1. Additive changes only - should be safe
2. Test data processing pipelines thoroughly

## Success Criteria

### Phase 1 Complete (Day 2)
- [ ] Rust 1.83.0 LTS installed and default
- [ ] All workspace crates compile with new toolchain
- [ ] No new clippy warnings introduced

### Phase 2 Complete (Day 5)  
- [ ] All non-breaking dependencies updated
- [ ] Leptos 0.11 migration functional
- [ ] WebGPU version alignment resolved

### Phase 3 Complete (Day 7)
- [ ] All dev dependencies updated
- [ ] Test suite runs with new dependencies
- [ ] Benchmarks show no performance regression

### Phase 4 Complete (Day 10)
- [ ] Automated dependency management configured
- [ ] Security monitoring in place
- [ ] Documentation updated with new versions

## Rollback Plan

If critical issues arise during updates:

1. **Immediate Rollback**: `git revert` the dependency changes
2. **Toolchain Rollback**: `rustup default 1.90.0` 
3. **Incremental Recovery**: Update one dependency at a time
4. **Feature Flags**: Use Cargo features to gate new dependency usage

## Post-Update Monitoring

### Week 1 After Updates
- Monitor CI stability (should be >95% pass rate)
- Check for new security advisories
- Validate performance benchmarks weekly

### Monthly Maintenance
- Review new security advisories
- Update patch versions automatically via Dependabot
- Assess new major versions for roadmap inclusion

**Next Action**: Begin with Rust toolchain update, then proceed incrementally through dependency tiers.
