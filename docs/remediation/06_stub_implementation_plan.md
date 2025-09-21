# Stub Implementation Priority Plan

**Priority:** HIGH  
**Timeline:** Weeks 4-8  
**Current Status:** Major APIs return stubs or panic  
**Target:** All advertised features functional or properly deprecated

## Stub Code Audit Results

### Critical Findings
- **No `todo!()` macros found** (✅ Good)
- **No `unimplemented!()` macros found** (✅ Improvement from previous state)
- **Compilation errors indicate logical stubs** (❌ Critical)
- **Documentation promises non-existent features** (❌ Brand risk)

### High-Risk Public APIs Currently Non-Functional

| API | File Location | Risk Level | Business Impact |
|-----|---------------|------------|-----------------|
| `WebGpuRenderer::render_chart()` | renderer.rs:89 | CRITICAL | Core product promise |
| `ExportSystem::export_to_png()` | export_system.rs:156 | HIGH | Enterprise feature |
| `ExportSystem::export_to_svg()` | export_system.rs:189 | HIGH | Enterprise feature |
| `OAuth2Provider::authenticate()` | security/auth.rs:101 | HIGH | Security/compliance |
| `SAMLProvider::process_response()` | security/auth.rs:311 | HIGH | Enterprise integration |
| `DataProcessingEngine::optimize()` | data_processing.rs:149 | MEDIUM | Performance claims |

## Phase 1: Critical Path Implementation (Week 4-5)

### 1.1 Core Rendering Pipeline - WebGpuRenderer

**Current State:**
```rust
impl WebGpuRenderer {
    pub fn render_chart(&self, spec: &ChartSpec, data: &DataFrame) -> Result<RenderedChart, RenderError> {
        // Currently returns empty/stub implementation
        todo!("Implement WebGPU rendering pipeline")
    }
}
```

**Minimum Viable Implementation:**
```rust
impl WebGpuRenderer {
    pub fn render_chart(&self, spec: &ChartSpec, data: &DataFrame) -> Result<RenderedChart, RenderError> {
        // Phase 1: Support basic bar and line charts only
        match spec.mark {
            MarkType::Bar => self.render_bar_chart(spec, data),
            MarkType::Line => self.render_line_chart(spec, data),
            _ => Err(RenderError::UnsupportedChartType(spec.mark.clone()))
        }
    }
    
    fn render_bar_chart(&self, spec: &ChartSpec, data: &DataFrame) -> Result<RenderedChart, RenderError> {
        // Minimal bar chart implementation
        let mut render_pass = self.create_render_pass()?;
        
        // Extract data for x/y axes based on encoding
        let x_data = self.extract_column_data(data, &spec.encoding.x)?;
        let y_data = self.extract_column_data(data, &spec.encoding.y)?;
        
        // Create bar geometry
        let bars = self.create_bar_geometry(&x_data, &y_data, spec)?;
        
        // Render to texture
        render_pass.draw_indexed(&bars.vertices, &bars.indices, 0..bars.count);
        
        Ok(RenderedChart {
            width: spec.width.unwrap_or(400),
            height: spec.height.unwrap_or(300),
            format: RenderFormat::WebGpuTexture,
            data: render_pass.finish()?,
        })
    }
}
```

### 1.2 Basic Export System Implementation

**Priority Order:**
1. **PNG Export** (Week 4) - Most commonly requested
2. **SVG Export** (Week 5) - Vector format for scalability  
3. **PDF Export** (Week 8) - Enterprise requirement

**PNG Export Implementation:**
```rust
impl ExportSystem {
    pub fn export_to_png(
        &self,
        chart: &RenderedChart,
        config: &ExportConfig,
    ) -> Result<Vec<u8>, ExportError> {
        match &chart.format {
            RenderFormat::WebGpuTexture => self.webgpu_to_png(chart, config),
            RenderFormat::Canvas2D => self.canvas_to_png(chart, config),
            _ => Err(ExportError::UnsupportedFormat(chart.format.clone()))
        }
    }
    
    fn webgpu_to_png(&self, chart: &RenderedChart, config: &ExportConfig) -> Result<Vec<u8>, ExportError> {
        // Read WebGPU texture data
        let texture_data = self.read_webgpu_texture(&chart.data)?;
        
        // Convert to PNG using image crate
        let img = image::RgbaImage::from_raw(
            chart.width,
            chart.height,
            texture_data
        ).ok_or(ExportError::InvalidTextureData)?;
        
        let mut png_data = Vec::new();
        img.write_to(&mut Cursor::new(&mut png_data), image::ImageOutputFormat::Png)?;
        
        Ok(png_data)
    }
}
```

### 1.3 Authentication System Stub Resolution

**OAuth2 Minimal Implementation:**
```rust
impl OAuth2Provider {
    pub async fn authenticate(&self, auth_code: &str) -> Result<AuthToken, SecurityError> {
        // Phase 1: Basic implementation without full OAuth2 flow
        // TODO: This is a simplified implementation for MVP
        log::warn!("Using simplified OAuth2 implementation - not production ready");
        
        // Validate auth code format
        if auth_code.is_empty() || auth_code.len() < 10 {
            return Err(SecurityError::InvalidAuthCode);
        }
        
        // Mock token exchange (replace with real implementation)
        let token = AuthToken {
            access_token: format!("mock_token_{}", uuid::Uuid::new_v4()),
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            refresh_token: Some(format!("refresh_{}", uuid::Uuid::new_v4())),
        };
        
        Ok(token)
    }
}
```

## Phase 2: Feature Completion (Week 6-7)

### 2.1 Data Processing Engine

**Current Gap:** Performance optimization claims not implemented
```rust
impl DataProcessingEngine {
    pub async fn optimize_pipeline(&self, config: &ProcessingConfig) -> Result<OptimizedPipeline, ProcessingError> {
        // Phase 2: Implement basic optimization strategies
        let strategy = self.select_strategy(config)?;
        
        match strategy {
            ProcessingStrategy::Parallel => self.create_parallel_pipeline(config),
            ProcessingStrategy::Streaming => self.create_streaming_pipeline(config),
            ProcessingStrategy::Hybrid => self.create_hybrid_pipeline(config),
        }
    }
    
    fn select_strategy(&self, config: &ProcessingConfig) -> Result<ProcessingStrategy, ProcessingError> {
        // Simple heuristic-based strategy selection
        if config.data_size > 1_000_000 {
            Ok(ProcessingStrategy::Streaming)
        } else if config.cpu_cores > 4 {
            Ok(ProcessingStrategy::Parallel)
        } else {
            Ok(ProcessingStrategy::Hybrid)
        }
    }
}
```

### 2.2 Advanced Chart Types

**Implementation Priority:**
1. Bar charts (Week 4) ✅ 
2. Line charts (Week 4) ✅
3. Scatter plots (Week 6)
4. Area charts (Week 6)  
5. Pie charts (Week 7)
6. Heatmaps (Week 8)

**Scatter Plot Implementation:**
```rust
impl WebGpuRenderer {
    fn render_scatter_chart(&self, spec: &ChartSpec, data: &DataFrame) -> Result<RenderedChart, RenderError> {
        let points = self.extract_scatter_points(data, spec)?;
        let point_shader = self.load_point_shader()?;
        
        let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Scatter Points"),
            contents: bytemuck::cast_slice(&points),
            usage: wgpu::BufferUsages::VERTEX,
        });
        
        let mut render_pass = self.create_render_pass()?;
        render_pass.set_pipeline(&point_shader);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..points.len() as u32, 0..1);
        
        Ok(RenderedChart {
            width: spec.width.unwrap_or(400),
            height: spec.height.unwrap_or(300),
            format: RenderFormat::WebGpuTexture,
            data: render_pass.finish()?,
        })
    }
}
```

## Phase 3: Enterprise Features (Week 8)

### 3.1 SAML Authentication

**Proper SAML Implementation:**
```rust
impl SAMLProvider {
    pub async fn process_saml_response(&self, saml_response: &str) -> Result<User, SecurityError> {
        // Parse SAML XML response  
        let parsed_response = self.parse_saml_xml(saml_response)?;
        
        // Verify signature
        self.verify_saml_signature(&parsed_response)?;
        
        // Extract user attributes
        let user = User {
            id: parsed_response.subject,
            email: parsed_response.attributes.get("email").cloned(),
            roles: parsed_response.attributes.get("roles")
                .map(|r| r.split(',').map(String::from).collect())
                .unwrap_or_default(),
        };
        
        Ok(user)
    }
    
    fn parse_saml_xml(&self, xml: &str) -> Result<SAMLResponse, SecurityError> {
        // Use proper SAML library for XML parsing
        // This is a simplified example
        use quick_xml::Reader;
        
        let mut reader = Reader::from_str(xml);
        // ... XML parsing implementation
        todo!("Implement proper SAML XML parsing")
    }
}
```

### 3.2 PDF Export Implementation

```rust
impl ExportSystem {
    pub fn export_to_pdf(&self, chart: &RenderedChart, config: &ExportConfig) -> Result<Vec<u8>, ExportError> {
        // Use printpdf crate for PDF generation
        use printpdf::*;
        
        let (doc, page1, layer1) = PdfDocument::new("Chart Export", 
            Mm(210.0), Mm(297.0), "Layer 1");
        
        // Convert chart to PDF-compatible format
        let pdf_image = self.chart_to_pdf_image(chart)?;
        
        // Add image to PDF
        let current_layer = doc.get_page(page1).get_layer(layer1);
        pdf_image.add_to_layer(current_layer, ImageTransform::default());
        
        // Save to bytes
        let mut pdf_bytes = Vec::new();
        doc.save_to(&mut pdf_bytes)?;
        
        Ok(pdf_bytes)
    }
}
```

## Feature Flag Strategy for Incomplete Features

### Experimental Feature Gating

```rust
// Cargo.toml
[features]
default = ["basic-charts"]
basic-charts = []
advanced-charts = ["basic-charts"]
enterprise-auth = ["oauth2", "saml"]
experimental = ["advanced-memory", "ml-intelligence"]

// Code gating
#[cfg(feature = "experimental")]
pub mod ml_intelligence {
    #[deprecated(note = "Experimental feature - API may change")]
    pub struct MLIntelligenceEngine { /* ... */ }
}

#[cfg(not(feature = "experimental"))]
pub mod ml_intelligence {
    pub struct MLIntelligenceEngine;
    
    impl MLIntelligenceEngine {
        pub fn new() -> Self {
            compile_error!("ML Intelligence requires 'experimental' feature flag");
        }
    }
}
```

### Documentation Accuracy Fixes

**Update README.md to reflect actual capabilities:**
```markdown
## Current Features (v0.7.1)

✅ **Implemented:**
- Basic bar and line chart rendering
- PNG export  
- CSV data source integration
- WebGPU rendering backend
- Basic accessibility support

🚧 **In Development:**
- SVG export (Week 5)
- Scatter plots (Week 6) 
- Advanced authentication (Week 8)

📋 **Planned (Future Versions):**
- PDF export
- Real-time collaboration
- Advanced ML intelligence features
- Enterprise SSO integration

## Feature Flags

Enable experimental features:
```toml
helios-core = { version = "0.7.1", features = ["experimental"] }
```
```

## Implementation Schedule & Milestones

### Week 4 Deliverables
- [ ] `WebGpuRenderer::render_chart()` supports bar/line charts
- [ ] `ExportSystem::export_to_png()` functional
- [ ] Basic OAuth2 authentication working
- [ ] Documentation updated to reflect actual capabilities

### Week 5 Deliverables  
- [ ] SVG export implemented
- [ ] Line chart rendering polished
- [ ] Error handling improved across all new implementations
- [ ] Integration tests passing for core features

### Week 6 Deliverables
- [ ] Scatter plot rendering
- [ ] Area chart support
- [ ] Data processing optimization basic implementation
- [ ] Performance benchmarks for new features

### Week 7 Deliverables
- [ ] Pie chart rendering
- [ ] Advanced chart styling options
- [ ] Security audit of authentication implementations
- [ ] User acceptance testing of core features

### Week 8 Deliverables
- [ ] PDF export capability
- [ ] Heatmap rendering  
- [ ] SAML authentication (basic)
- [ ] Enterprise feature documentation complete

## Risk Assessment

**HIGH RISK - API Surface Expansion**  
Risk: Implementing features quickly may create technical debt  
Mitigation: Focus on MVP implementations, refactor in subsequent releases

**MEDIUM RISK - Performance**  
Risk: Quick implementations may not meet performance requirements  
Mitigation: Benchmark all implementations, optimize hot paths

**LOW RISK - Testing**  
Risk: New implementations may lack comprehensive testing  
Mitigation: Implement contract tests alongside feature development

## Success Criteria

- All advertised features in README have functional implementations
- No compilation errors in core functionality
- Export features produce valid output files
- Authentication systems handle basic workflows
- Performance meets minimum acceptable thresholds (see benchmarks)

**Next Action**: Begin with WebGpuRenderer::render_chart() implementation for bar charts as this unblocks the entire rendering pipeline.
