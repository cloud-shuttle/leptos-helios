# Contributing to Helios

> Building the future of web visualization together

## Welcome Contributors!

Helios aims to be the definitive visualization library for Rust and Leptos, and we need your help to make that vision a reality. Whether you're fixing bugs, adding features, improving documentation, or optimizing performance, your contributions are valuable.

## Quick Start for Contributors

### Development Environment Setup

```bash
# Clone the repository
git clone https://github.com/cloudshuttle/helios.git
cd helios

# Install Rust with WebAssembly support
rustup target add wasm32-unknown-unknown

# Install development tools
cargo install trunk wasm-pack wasm-opt
cargo install cargo-nextest  # For faster testing
cargo install cargo-audit    # Security auditing

# Install Node.js tools for examples
npm install -g concurrently serve

# Run the development setup
make setup  # or cargo make setup if you have cargo-make
```

### Building and Testing

```bash
# Build the project
cargo build

# Run all tests
cargo nextest run

# Build WASM examples
trunk build examples/basic-charts/index.html
trunk serve examples/basic-charts/index.html --open

# Run benchmarks
cargo bench

# Check code quality
cargo clippy -- -D warnings
cargo fmt --check
```

### Project Structure

```
helios/
├── Cargo.toml              # Workspace configuration
├── Makefile                # Development commands
├── README.md
├── LICENSE
│
├── crates/
│   ├── helios-core/        # Core visualization engine
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── chart/      # Chart specifications
│   │   │   ├── data/       # Data processing pipeline
│   │   │   ├── render/     # Rendering backends
│   │   │   ├── gpu/        # WebGPU/compute shaders
│   │   │   ├── intelligence/# ML and NLP features
│   │   │   └── utils/      # Utilities
│   │   ├── tests/
│   │   └── benches/
│   │
│   ├── helios-leptos/      # Leptos integration
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── components/ # React-like components
│   │   │   ├── hooks/      # Custom hooks
│   │   │   └── server/     # Server functions
│   │   └── tests/
│   │
│   ├── helios-macros/      # Procedural macros
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── chart.rs    # chart! macro
│   │   │   └── dashboard.rs # dashboard! macro
│   │   └── tests/
│   │
│   └── helios-wasm/        # WASM bindings
│       ├── src/
│       │   ├── lib.rs
│       │   └── bindings/
│       └── tests/
│
├── examples/               # Example applications
│   ├── basic-charts/       # Simple chart examples
│   ├── dashboard/          # Dashboard example
│   ├── streaming/          # Real-time data
│   ├── ml-features/        # AI/ML examples
│   └── performance/        # Performance demos
│
├── docs/                   # Documentation
│   ├── architecture.md
│   ├── api.md
│   ├── getting-started.md
│   ├── performance.md
│   ├── contributing.md (this file)
│   └── ecosystem.md
│
├── benchmarks/             # Performance benchmarks
│   ├── rendering/
│   ├── data-processing/
│   └── memory/
│
├── tests/                  # Integration tests
│   ├── browser/           # Browser-specific tests
│   ├── wasm/              # WASM tests
│   └── e2e/               # End-to-end tests
│
└── tools/                  # Development tools
    ├── build-scripts/
    ├── ci/
    └── profiling/
```

## Contribution Areas

### 1. Core Engine Development 🏗️

**Skills Needed**: Rust, Graphics Programming, Performance Optimization

**Areas**:
- WebGPU renderer improvements
- Data processing pipeline optimization
- Memory management and buffer pooling
- Compute shader development
- Cross-platform compatibility

**Getting Started**:
- Look for issues labeled `core-engine` and `good-first-issue`
- Check the performance benchmarks in `/benchmarks`
- Read the architecture documentation

### 2. Leptos Integration 🎯

**Skills Needed**: Rust, Leptos, Web Development

**Areas**:
- Component API improvements
- Server function optimization
- Reactive system integration
- Developer experience enhancements

**Getting Started**:
- Explore `/crates/helios-leptos`
- Try building examples in `/examples`
- Check issues labeled `leptos-integration`

### 3. Visualization Features 📊

**Skills Needed**: Data Visualization, UX/UI Design

**Areas**:
- New chart types (3D, geographic, network graphs)
- Interaction systems (brushing, linking, animations)
- Accessibility improvements
- Theme system development

**Getting Started**:
- Look at existing chart implementations
- Check issues labeled `visualization-features`
- Contribute to the example gallery

### 4. Performance Optimization ⚡

**Skills Needed**: Performance Analysis, SIMD, GPU Programming

**Areas**:
- SIMD optimization
- GPU shader optimization
- Memory layout improvements
- Streaming performance

**Getting Started**:
- Run the benchmark suite
- Profile existing implementations
- Check issues labeled `performance`

### 5. Machine Learning Integration 🤖

**Skills Needed**: Machine Learning, Statistics

**Areas**:
- Forecasting algorithms
- Anomaly detection
- Natural language processing
- Model optimization

**Getting Started**:
- Explore `/crates/helios-core/src/intelligence`
- Check issues labeled `ml-features`
- Contribute ML examples

### 6. Documentation and Examples 📚

**Skills Needed**: Technical Writing, Web Development

**Areas**:
- API documentation
- Tutorial creation
- Example applications
- Performance guides

**Getting Started**:
- Check issues labeled `documentation`
- Improve existing docs
- Create new examples

## Development Workflow

### 1. Issue Creation and Discussion

Before starting work:
1. **Check existing issues** to avoid duplicates
2. **Create an issue** describing the problem/feature
3. **Discuss approach** with maintainers
4. **Get assignment** to avoid conflicts

### 2. Branching Strategy

We use a **feature branch workflow**:

```bash
# Create feature branch from main
git checkout main
git pull origin main
git checkout -b feature/your-feature-name

# Make your changes
git add .
git commit -m "feat: add new chart type"

# Keep branch updated
git rebase main

# Push and create PR
git push origin feature/your-feature-name
```

### 3. Commit Message Convention

We follow **Conventional Commits**:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `perf`: Performance improvement
- `refactor`: Code refactoring
- `test`: Test additions/changes
- `chore`: Build/tooling changes

**Examples**:
```
feat(core): add 3D scatter plot support

- Implement 3D point cloud rendering
- Add camera controls for 3D navigation
- Include depth testing and perspective projection

Closes #123
```

```
perf(render): optimize GPU buffer management

- Implement buffer pooling to reduce allocations
- Use persistent buffer mapping where available
- 40% reduction in frame time for large datasets
```

### 4. Pull Request Process

1. **Create PR** with descriptive title and description
2. **Link related issues** using "Closes #123"
3. **Request review** from code owners
4. **Respond to feedback** promptly
5. **Ensure CI passes** before requesting final review

### 5. Code Review Guidelines

**For Authors**:
- Keep PRs focused and reasonably sized
- Write clear commit messages
- Include tests for new functionality
- Update documentation as needed

**For Reviewers**:
- Be constructive and respectful
- Focus on code quality, performance, and maintainability
- Test the changes locally when possible
- Approve when satisfied with quality

## Code Standards

### Rust Style Guide

We follow **Rust standard style** with some additions:

```rust
// Good: Clear, descriptive names
pub struct ChartRenderer {
    device: wgpu::Device,
    pipeline_cache: HashMap<ChartType, RenderPipeline>,
}

impl ChartRenderer {
    /// Renders a chart specification to the given render pass.
    ///
    /// # Arguments
    /// * `spec` - The chart specification to render
    /// * `render_pass` - The WebGPU render pass
    ///
    /// # Returns
    /// Rendering statistics including frame time and memory usage
    pub fn render_chart(
        &mut self,
        spec: &ChartSpec,
        render_pass: &mut RenderPass
    ) -> RenderStats {
        // Implementation
    }
}
```

### Performance Guidelines

1. **Measure first**: Profile before optimizing
2. **Prefer zero-cost abstractions**: Use compile-time techniques
3. **Minimize allocations**: Reuse objects when possible
4. **Use SIMD**: Optimize hot paths with vectorization
5. **GPU-first**: Leverage parallel processing

### Error Handling

Use `Result<T, E>` for fallible operations:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChartError {
    #[error("Invalid data format: {0}")]
    InvalidData(String),

    #[error("Rendering failed: {0}")]
    RenderError(#[from] wgpu::Error),

    #[error("Configuration error: {field} must be > 0")]
    ConfigError { field: String },
}

pub fn create_chart(spec: &ChartSpec) -> Result<Chart, ChartError> {
    spec.validate().map_err(ChartError::InvalidData)?;
    Ok(Chart::new(spec))
}
```

### Testing Requirements

Every contribution should include appropriate tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chart_creation() {
        let spec = ChartSpec {
            mark: MarkType::Point,
            // ... other fields
        };

        let chart = create_chart(&spec).unwrap();
        assert_eq!(chart.mark_type(), MarkType::Point);
    }

    #[wasm_bindgen_test]
    async fn test_wasm_rendering() {
        let chart = create_test_chart();
        let stats = render_chart_to_canvas(&chart).await;

        assert!(stats.frame_time < Duration::from_millis(16));
        assert!(stats.memory_used < 100 * 1024 * 1024); // 100MB limit
    }
}
```

## Performance Benchmarking

### Writing Benchmarks

Use `criterion.rs` for benchmarks:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_point_rendering(c: &mut Criterion) {
    let points = generate_test_points(100_000);

    c.bench_function("render 100k points", |b| {
        b.iter(|| {
            render_points(black_box(&points))
        })
    });
}

criterion_group!(benches, benchmark_point_rendering);
criterion_main!(benches);
```

### Performance Regression Testing

All performance-critical PRs must include benchmark results:

```bash
# Run benchmarks before changes
cargo bench > before.txt

# Make your changes
# ...

# Run benchmarks after changes
cargo bench > after.txt

# Compare results
cargo install critcmp
critcmp before.txt after.txt
```

## Documentation Standards

### API Documentation

All public APIs must have comprehensive documentation:

```rust
/// A high-performance chart renderer using WebGPU.
///
/// The `ChartRenderer` provides efficient rendering of data visualizations
/// by leveraging GPU acceleration and intelligent optimization strategies.
///
/// # Examples
///
/// ```rust
/// use helios::ChartRenderer;
///
/// let renderer = ChartRenderer::new().await?;
/// let stats = renderer.render(&chart_spec)?;
/// println!("Rendered in {}ms", stats.frame_time.as_millis());
/// ```
///
/// # Performance
///
/// - Supports up to 10M points at 60fps with WebGPU
/// - Memory usage scales linearly with data size
/// - Automatic quality adjustment based on performance
pub struct ChartRenderer {
    // fields...
}
```

### Changelog Updates

For significant changes, update `CHANGELOG.md`:

```markdown
## [Unreleased]

### Added
- 3D scatter plot support with camera controls
- Geographic visualization with map tile integration
- Natural language query processing for chart generation

### Changed
- Improved WebGPU buffer management for 40% better performance
- Enhanced accessibility with WCAG 2.1 AA compliance

### Fixed
- Memory leak in streaming data pipeline
- Incorrect color interpolation in gradients
```

## Release Process

### Version Numbering

We follow **Semantic Versioning** (semver):
- **Major** (x.0.0): Breaking changes
- **Minor** (0.x.0): New features, backward compatible
- **Patch** (0.0.x): Bug fixes, backward compatible

### Release Checklist

For maintainers preparing releases:

1. **Update version** in `Cargo.toml`
2. **Update CHANGELOG.md** with release notes
3. **Run full test suite** including benchmarks
4. **Test examples** in multiple browsers
5. **Create release tag** and GitHub release
6. **Publish to crates.io**

## Community Guidelines

### Code of Conduct

We are committed to providing a welcoming and inclusive environment. All contributors are expected to adhere to our Code of Conduct:

- **Be respectful** and inclusive
- **Be collaborative** and help others learn
- **Be patient** with questions and mistakes
- **Be constructive** in feedback and criticism

### Communication Channels

- **GitHub Issues**: Bug reports, feature requests, technical discussion
- **GitHub Discussions**: General questions, ideas, showcases
- **Discord**: Real-time chat and community support (link in README)

### Recognition

Contributors are recognized in:
- `CONTRIBUTORS.md` file
- GitHub contributor graphs
- Release notes for significant contributions
- Special recognition for sustained contributions

## Getting Help

### Development Questions

1. **Check existing documentation** first
2. **Search GitHub issues** for similar questions
3. **Ask in GitHub Discussions** for general questions
4. **Join Discord** for real-time help
5. **Create issue** if you find a bug or have a specific technical question

### Mentorship Program

We offer mentorship for new contributors:
- **Good first issues** labeled for newcomers
- **Mentorship available** on request
- **Pair programming sessions** for complex features
- **Code review feedback** focused on learning

## Areas Needing Help

### High Priority 🔥
- WebGPU compute shader optimizations
- Mobile browser performance testing
- Accessibility improvements
- Documentation and examples

### Medium Priority 📋
- New chart types (network graphs, 3D visualizations)
- Server-side rendering optimizations
- Developer tooling improvements
- Internationalization

### Future Features 🚀
- Natural language query interface
- Advanced ML integration
- Real-time collaboration features
- Plugin system for custom visualizations

## Success Metrics

We measure project success through:
- **Performance benchmarks**: Continuous improvement in speed and memory usage
- **Community growth**: Active contributors and users
- **Production adoption**: Real-world usage and feedback
- **Developer experience**: How easy it is to create visualizations

## Thank You!

Every contribution, no matter how small, helps make Helios better. Whether you're:
- Fixing typos in documentation
- Reporting bugs you've encountered
- Contributing new features
- Helping other users
- Spreading the word about Helios

You're helping build the future of web visualization. Thank you for being part of this journey! 🚀

---

*Ready to contribute? Start by exploring [good first issues](https://github.com/cloudshuttle/helios/labels/good%20first%20issue) or join our [Discord community](https://discord.gg/helios) to connect with other contributors.*
