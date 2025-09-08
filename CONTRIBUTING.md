# Contributing to Leptos Helios

Thank you for your interest in contributing to Leptos Helios! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)
- [Pull Request Process](#pull-request-process)
- [Release Process](#release-process)

## Code of Conduct

This project adheres to a code of conduct that we expect all contributors to follow. Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md).

### Our Pledge

We pledge to make participation in our project a harassment-free experience for everyone, regardless of age, body size, disability, ethnicity, gender identity and expression, level of experience, nationality, personal appearance, race, religion, or sexual identity and orientation.

### Our Standards

Examples of behavior that contributes to creating a positive environment include:

- Using welcoming and inclusive language
- Being respectful of differing viewpoints and experiences
- Gracefully accepting constructive criticism
- Focusing on what is best for the community
- Showing empathy towards other community members

Examples of unacceptable behavior include:

- The use of sexualized language or imagery
- Trolling, insulting/derogatory comments, and personal or political attacks
- Public or private harassment
- Publishing others' private information without explicit permission
- Other conduct which could reasonably be considered inappropriate in a professional setting

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Node.js 18 or later (for frontend development)
- pnpm (for package management)
- Git

### Development Setup

1. **Fork and clone the repository**
   ```bash
   git clone https://github.com/your-username/leptos-helios.git
   cd leptos-helios
   ```

2. **Install dependencies**
   ```bash
   # Install Rust dependencies
   cargo build

   # Install Node.js dependencies
   pnpm install
   ```

3. **Run tests**
   ```bash
   # Run Rust tests
   cargo test

   # Run Playwright E2E tests
   pnpm test:e2e
   ```

4. **Start development server**
   ```bash
   pnpm dev
   ```

## Development Workflow

### Branch Strategy

We use a Git Flow-inspired branching strategy:

- `main` - Production-ready code
- `develop` - Integration branch for features
- `feature/*` - Feature branches
- `bugfix/*` - Bug fix branches
- `hotfix/*` - Critical production fixes
- `release/*` - Release preparation branches

### Creating a Feature Branch

```bash
# Start from develop
git checkout develop
git pull origin develop

# Create feature branch
git checkout -b feature/your-feature-name

# Make your changes
# ... commit your work ...

# Push to your fork
git push origin feature/your-feature-name
```

### Commit Message Format

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**Types:**
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Changes that do not affect the meaning of the code
- `refactor`: A code change that neither fixes a bug nor adds a feature
- `perf`: A code change that improves performance
- `test`: Adding missing tests or correcting existing tests
- `chore`: Changes to the build process or auxiliary tools

**Examples:**
```
feat(webgpu): add support for compute shaders
fix(accessibility): resolve screen reader compatibility issues
docs(api): update chart specification documentation
test(e2e): add Playwright tests for WebGPU rendering
```

## Coding Standards

### Rust Code Style

We follow the standard Rust formatting and linting rules:

```bash
# Format code
cargo fmt

# Check linting
cargo clippy -- -D warnings
```

### Key Principles

1. **Performance First**: Optimize for performance, especially for large datasets
2. **Accessibility**: Ensure all features are accessible by default
3. **Security**: Follow security best practices, especially for enterprise features
4. **Documentation**: Document all public APIs and complex logic
5. **Testing**: Write comprehensive tests for all new features

### Code Organization

```
helios-core/src/
├── chart/              # Core charting functionality
├── renderer/           # Rendering backends (WebGPU, Canvas2D, WebGL2)
├── data_sources/       # Data source adapters
├── export_system/      # Export functionality
├── accessibility/      # Accessibility features
├── security/           # Security and governance
├── performance/        # Performance optimization
├── plugin_system/      # Plugin architecture
├── headless_renderer/  # Server-side rendering
└── tests/              # Test suites
```

### Error Handling

Use proper error handling with custom error types:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChartError {
    #[error("Invalid chart specification: {0}")]
    InvalidSpec(String),

    #[error("Rendering failed: {0}")]
    RenderingFailed(String),

    #[error("Data processing error: {0}")]
    DataError(#[from] polars::PolarsError),
}

// Use in functions
pub fn create_chart(spec: &ChartSpec) -> Result<Chart, ChartError> {
    if spec.is_empty() {
        return Err(ChartError::InvalidSpec("Empty specification".to_string()));
    }

    // ... implementation
    Ok(chart)
}
```

### Async/Await Patterns

Use async/await consistently for I/O operations:

```rust
pub async fn render_chart(
    &self,
    spec: &ChartSpec,
    data: &DataFrame,
) -> Result<Vec<u8>, RenderError> {
    // Async operations
    let processed_data = self.process_data(data).await?;
    let rendered = self.render(processed_data).await?;

    Ok(rendered)
}
```

## Testing Guidelines

### Test Structure

We use a comprehensive testing strategy:

1. **Unit Tests** - Test individual functions and methods
2. **Integration Tests** - Test component interactions
3. **E2E Tests** - Test complete user workflows with Playwright
4. **Performance Tests** - Test performance characteristics
5. **Accessibility Tests** - Test accessibility compliance

### Writing Tests

#### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chart_creation() {
        let spec = ChartSpec::default();
        let chart = Chart::new(spec);
        assert!(chart.is_valid());
    }

    #[tokio::test]
    async fn test_async_rendering() {
        let renderer = WebGpuRenderer::new().await.unwrap();
        let result = renderer.render_chart(&spec, &data).await;
        assert!(result.is_ok());
    }
}
```

#### Integration Tests

```rust
// tests/integration_tests.rs
use leptos_helios::*;

#[tokio::test]
async fn test_full_rendering_pipeline() {
    // Test complete rendering pipeline
    let data = create_test_data();
    let spec = create_test_spec();
    let renderer = WebGpuRenderer::new().await.unwrap();

    let result = renderer.render_chart(&spec, &data).await;
    assert!(result.is_ok());

    let chart_data = result.unwrap();
    assert!(!chart_data.is_empty());
}
```

#### E2E Tests

```javascript
// tests/e2e/chart-rendering.spec.js
import { test, expect } from '@playwright/test';

test('chart renders correctly', async ({ page }) => {
  await page.goto('/chart-demo');

  // Wait for chart to load
  await page.waitForSelector('#chart-container');

  // Check if chart is visible
  const chart = page.locator('#chart-container canvas');
  await expect(chart).toBeVisible();

  // Check chart dimensions
  const box = await chart.boundingBox();
  expect(box.width).toBeGreaterThan(0);
  expect(box.height).toBeGreaterThan(0);
});
```

### Test Coverage

Maintain high test coverage:

```bash
# Run tests with coverage
cargo tarpaulin --out Html

# Check coverage threshold
cargo tarpaulin --fail-under 80
```

### Performance Testing

```rust
#[tokio::test]
async fn test_rendering_performance() {
    let renderer = WebGpuRenderer::new().await.unwrap();
    let large_data = create_large_dataset(100_000);

    let start = std::time::Instant::now();
    let result = renderer.render_chart(&spec, &large_data).await;
    let duration = start.elapsed();

    assert!(result.is_ok());
    assert!(duration.as_millis() < 1000); // Should render in under 1 second
}
```

## Documentation

### API Documentation

Document all public APIs:

```rust
/// Renders a chart using the specified specification and data.
///
/// # Arguments
/// * `spec` - The chart specification defining the chart type and configuration
/// * `data` - The data to be visualized
///
/// # Returns
/// * `Ok(Vec<u8>)` - The rendered chart as PNG data
/// * `Err(RenderError)` - If rendering fails
///
/// # Examples
/// ```rust
/// let renderer = WebGpuRenderer::new().await?;
/// let chart_data = renderer.render_chart(&spec, &data).await?;
/// ```
pub async fn render_chart(
    &self,
    spec: &ChartSpec,
    data: &DataFrame,
) -> Result<Vec<u8>, RenderError> {
    // Implementation
}
```

### README Updates

Update relevant README files when adding new features:

- `README.md` - Main project documentation
- `docs/API_REFERENCE.md` - API documentation
- `docs/TUTORIALS.md` - Tutorial guides
- `examples/` - Code examples

### Code Comments

Add inline comments for complex logic:

```rust
// Calculate the optimal batch size for SIMD processing
// This ensures we process data in chunks that align with SIMD vector sizes
let batch_size = if data.len() > 1024 {
    // For large datasets, use larger batches for better SIMD utilization
    256
} else {
    // For smaller datasets, use smaller batches to avoid overhead
    64
};
```

## Pull Request Process

### Before Submitting

1. **Run all tests**
   ```bash
   cargo test
   pnpm test:e2e
   ```

2. **Check formatting and linting**
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   ```

3. **Update documentation**
   - Update API documentation if needed
   - Add examples for new features
   - Update README if necessary

4. **Write a comprehensive description**
   - Explain what the PR does
   - Reference any related issues
   - Include screenshots for UI changes
   - List any breaking changes

### PR Template

```markdown
## Description
Brief description of the changes.

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] E2E tests pass
- [ ] Manual testing completed

## Checklist
- [ ] Code follows the project's coding standards
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No breaking changes (or breaking changes documented)

## Related Issues
Fixes #(issue number)
```

### Review Process

1. **Automated Checks**
   - All tests must pass
   - Code coverage must not decrease
   - No linting errors

2. **Code Review**
   - At least one maintainer review required
   - Address all review comments
   - Ensure code quality and performance

3. **Final Approval**
   - Maintainer approval required
   - All CI checks must pass

## Release Process

### Version Numbering

We follow [Semantic Versioning](https://semver.org/):

- `MAJOR` - Breaking changes
- `MINOR` - New features (backward compatible)
- `PATCH` - Bug fixes (backward compatible)

### Release Steps

1. **Create release branch**
   ```bash
   git checkout develop
   git checkout -b release/v0.4.0
   ```

2. **Update version numbers**
   - Update `Cargo.toml` versions
   - Update `package.json` versions
   - Update documentation

3. **Run full test suite**
   ```bash
   cargo test --all
   pnpm test:e2e
   ```

4. **Create release notes**
   - Document new features
   - List bug fixes
   - Note breaking changes

5. **Create GitHub release**
   - Tag the release
   - Upload release notes
   - Publish to crates.io

### Hotfix Process

For critical production issues:

1. **Create hotfix branch from main**
   ```bash
   git checkout main
   git checkout -b hotfix/critical-fix
   ```

2. **Make minimal fix**
   - Focus only on the critical issue
   - Avoid unnecessary changes

3. **Test thoroughly**
   - Run all tests
   - Test the specific fix

4. **Merge to main and develop**
   ```bash
   git checkout main
   git merge hotfix/critical-fix
   git tag v0.3.1

   git checkout develop
   git merge hotfix/critical-fix
   ```

## Community Guidelines

### Getting Help

- **GitHub Issues** - For bug reports and feature requests
- **GitHub Discussions** - For questions and general discussion
- **Discord** - For real-time community chat

### Reporting Issues

When reporting issues, please include:

1. **Environment details**
   - Rust version
   - Operating system
   - Browser version (for frontend issues)

2. **Reproduction steps**
   - Clear steps to reproduce the issue
   - Expected vs actual behavior

3. **Code examples**
   - Minimal code that reproduces the issue
   - Error messages and stack traces

4. **Additional context**
   - Screenshots if applicable
   - Related issues or discussions

### Feature Requests

When requesting features:

1. **Check existing issues** - Avoid duplicates
2. **Provide use case** - Explain why the feature is needed
3. **Propose implementation** - Suggest how it might work
4. **Consider alternatives** - Are there existing solutions?

### Contributing Examples

We welcome example contributions:

1. **Create example directory**
   ```bash
   mkdir examples/your-example
   cd examples/your-example
   ```

2. **Include documentation**
   - README explaining the example
   - Code comments
   - Screenshots or demos

3. **Follow example guidelines**
   - Keep examples focused and simple
   - Use realistic data
   - Include error handling

## Recognition

Contributors are recognized in:

- **CONTRIBUTORS.md** - List of all contributors
- **Release notes** - Contributors for each release
- **GitHub contributors** - Automatic recognition

Thank you for contributing to Leptos Helios! Your contributions help make this project better for everyone.
