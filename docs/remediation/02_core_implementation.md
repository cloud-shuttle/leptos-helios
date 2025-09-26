# Core Implementation Plan (Days 8-30)
**Priority**: 🔴 HIGH
**Owner**: Senior Rust Developer
**Timeline**: 3 weeks
**Goal**: Single working rendering path

## Phase 1: MVP Scope Definition

### Minimum Viable Product Scope
**Focus**: Prove the concept works end-to-end

**Included**:
- Basic bar chart rendering only
- WebGPU backend only
- Static data input (DataFrame)
- PNG export only
- No interactivity
- No enterprise features

**Excluded** (for now):
- Line charts, scatter plots, complex charts
- Canvas2D/WebGL2 fallbacks
- Real-time data, streaming
- ML/AI features
- Security, accessibility
- Plugin system

## Implementation Plan

### Week 1 (Days 8-14): WebGPU Foundation

#### Day 8-9: WebGPU Device Setup
**File**: `helios-core/src/webgpu/device.rs` (NEW, <150 lines)

```rust
//! WebGPU device management and initialization
//! Status: 🚧 IN PROGRESS

use wgpu::*;

pub struct WebGpuDevice {
    device: Device,
    queue: Queue,
    surface: Option<Surface>,
}

impl WebGpuDevice {
    pub async fn new() -> Result<Self, WebGpuError> {
        // IMPLEMENT: Real device creation, not stub
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
        });

        let adapter = instance
            .request_adapter(&RequestAdapterOptions::default())
            .await
            .ok_or(WebGpuError::NoAdapter)?;

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default(), None)
            .await
            .map_err(WebGpuError::DeviceRequest)?;

        Ok(Self {
            device,
            queue,
            surface: None,
        })
    }

    pub fn create_shader(&self, source: &str) -> ShaderModule {
        self.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("chart_shader"),
            source: ShaderSource::Wgsl(source.into()),
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WebGpuError {
    #[error("No WebGPU adapter found")]
    NoAdapter,
    #[error("Device request failed: {0}")]
    DeviceRequest(#[from] RequestDeviceError),
}
```

**Test**: `tests/webgpu_device_test.rs`
```rust
#[tokio::test]
async fn test_webgpu_device_creation() {
    let device = WebGpuDevice::new().await;
    assert!(device.is_ok(), "WebGPU device should be created");
}
```

#### Day 10-11: Basic Vertex Pipeline
**File**: `helios-core/src/webgpu/pipeline.rs` (NEW, <200 lines)

```rust
//! Basic rendering pipeline for bar charts
//! Status: 🚧 IN PROGRESS

pub struct BarChartPipeline {
    pipeline: RenderPipeline,
    vertex_buffer: Buffer,
}

impl BarChartPipeline {
    pub fn new(device: &WebGpuDevice) -> Result<Self, PipelineError> {
        // IMPLEMENT: Real pipeline creation
        let shader = device.create_shader(include_str!("shaders/bar_chart.wgsl"));

        let pipeline = device.device().create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("bar_chart_pipeline"),
            layout: None,
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[VertexBufferLayout {
                    array_stride: std::mem::size_of::<BarVertex>() as BufferAddress,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &[
                        VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: VertexFormat::Float32x2,
                        },
                        VertexAttribute {
                            offset: 8,
                            shader_location: 1,
                            format: VertexFormat::Float32x3,
                        },
                    ],
                }],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: TextureFormat::Bgra8UnormSrgb,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            // ... rest of pipeline config
        });

        Ok(Self {
            pipeline,
            vertex_buffer: device.create_vertex_buffer()?,
        })
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BarVertex {
    position: [f32; 2],
    color: [f32; 3],
}
```

#### Day 12-14: Basic Shader
**File**: `helios-core/src/webgpu/shaders/bar_chart.wgsl` (NEW, <100 lines)

```wgsl
// Basic bar chart vertex shader
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.color = input.color;
    out.clip_position = vec4<f32>(input.position, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
```

### Week 2 (Days 15-21): Data to Geometry

#### Day 15-16: Data Processing Pipeline
**File**: `helios-core/src/data/processor.rs` (REFACTOR from existing, <150 lines)

```rust
//! Simple data processing for bar charts
//! Status: 🚧 IN PROGRESS

use polars::prelude::*;

pub struct BarChartData {
    pub labels: Vec<String>,
    pub values: Vec<f32>,
    pub max_value: f32,
}

impl BarChartData {
    pub fn from_dataframe(df: &DataFrame, label_col: &str, value_col: &str) -> Result<Self, DataError> {
        // IMPLEMENT: Real data extraction, not stub
        let labels = df
            .column(label_col)?
            .utf8()?
            .into_iter()
            .map(|opt| opt.unwrap_or("").to_string())
            .collect();

        let values: Vec<f32> = df
            .column(value_col)?
            .f32()?
            .into_iter()
            .map(|opt| opt.unwrap_or(0.0))
            .collect();

        let max_value = values.iter().fold(0.0, |a, &b| a.max(b));

        Ok(Self {
            labels,
            values,
            max_value,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("Column not found: {0}")]
    ColumnNotFound(String),
    #[error("Polars error: {0}")]
    Polars(#[from] polars::error::PolarsError),
}
```

#### Day 17-19: Geometry Generation
**File**: `helios-core/src/chart/bar_geometry.rs` (NEW, <200 lines)

```rust
//! Convert bar chart data to WebGPU vertices
//! Status: 🚧 IN PROGRESS

use crate::data::BarChartData;
use crate::webgpu::pipeline::BarVertex;

pub struct BarGeometryBuilder {
    width: f32,
    height: f32,
    margin: f32,
}

impl BarGeometryBuilder {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            margin: 40.0,
        }
    }

    pub fn build_vertices(&self, data: &BarChartData) -> Vec<BarVertex> {
        // IMPLEMENT: Real geometry generation
        let chart_width = self.width - 2.0 * self.margin;
        let chart_height = self.height - 2.0 * self.margin;
        let bar_width = chart_width / data.values.len() as f32;

        let mut vertices = Vec::new();

        for (i, &value) in data.values.iter().enumerate() {
            let x = self.margin + (i as f32 + 0.1) * bar_width;
            let bar_height = (value / data.max_value) * chart_height;
            let y = self.height - self.margin - bar_height;

            // Create rectangle (2 triangles)
            let color = [0.2, 0.6, 0.8]; // Blue bars

            // Convert to NDC coordinates (-1 to 1)
            let x_ndc = (x / self.width) * 2.0 - 1.0;
            let y_ndc = 1.0 - (y / self.height) * 2.0;
            let w_ndc = (bar_width * 0.8) / self.width * 2.0;
            let h_ndc = bar_height / self.height * 2.0;

            // Triangle 1
            vertices.push(BarVertex { position: [x_ndc, y_ndc], color });
            vertices.push(BarVertex { position: [x_ndc + w_ndc, y_ndc], color });
            vertices.push(BarVertex { position: [x_ndc, y_ndc - h_ndc], color });

            // Triangle 2
            vertices.push(BarVertex { position: [x_ndc + w_ndc, y_ndc], color });
            vertices.push(BarVertex { position: [x_ndc + w_ndc, y_ndc - h_ndc], color });
            vertices.push(BarVertex { position: [x_ndc, y_ndc - h_ndc], color });
        }

        vertices
    }
}
```

#### Day 20-21: Basic Rendering
**File**: `helios-core/src/webgpu/renderer.rs` (REFACTOR existing, <250 lines)

```rust
//! WebGPU chart renderer - MVP implementation
//! Status: 🚧 IN PROGRESS

use crate::data::BarChartData;
use crate::webgpu::{WebGpuDevice, BarChartPipeline};
use crate::chart::BarGeometryBuilder;

pub struct WebGpuRenderer {
    device: WebGpuDevice,
    pipeline: BarChartPipeline,
    geometry_builder: BarGeometryBuilder,
}

impl WebGpuRenderer {
    pub async fn new(width: u32, height: u32) -> Result<Self, RendererError> {
        // IMPLEMENT: Real renderer creation, not stub
        let device = WebGpuDevice::new().await?;
        let pipeline = BarChartPipeline::new(&device)?;
        let geometry_builder = BarGeometryBuilder::new(width as f32, height as f32);

        Ok(Self {
            device,
            pipeline,
            geometry_builder,
        })
    }

    pub async fn render_bar_chart(&mut self, data: &BarChartData) -> Result<Vec<u8>, RendererError> {
        // IMPLEMENT: Real rendering, not stub
        let vertices = self.geometry_builder.build_vertices(data);

        // Upload vertices to GPU
        self.pipeline.update_vertex_buffer(&self.device, &vertices)?;

        // Create render texture
        let texture = self.device.create_render_texture(800, 600)?;

        // Render
        let mut encoder = self.device.create_command_encoder()?;
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("bar_chart_render"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &texture.view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::WHITE),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.pipeline.pipeline);
            render_pass.set_vertex_buffer(0, self.pipeline.vertex_buffer.slice(..));
            render_pass.draw(0..vertices.len() as u32, 0..1);
        }

        self.device.queue.submit([encoder.finish()]);

        // Read back pixels
        self.device.read_texture_pixels(&texture).await
    }
}
```

### Week 3 (Days 22-30): End-to-End Integration

#### Day 22-24: WASM Integration
**File**: `helios-wasm/src/lib.rs` (REFACTOR, <150 lines)

```rust
//! WASM bindings for basic bar chart rendering
//! Status: 🚧 IN PROGRESS

use wasm_bindgen::prelude::*;
use helios_core::*;

#[wasm_bindgen]
pub struct WasmBarChart {
    renderer: Option<WebGpuRenderer>,
}

#[wasm_bindgen]
impl WasmBarChart {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        Self { renderer: None }
    }

    #[wasm_bindgen]
    pub async fn init(&mut self, width: u32, height: u32) -> Result<(), JsValue> {
        // IMPLEMENT: Real WASM renderer initialization
        let renderer = WebGpuRenderer::new(width, height)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        self.renderer = Some(renderer);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn render_from_json(&mut self, json_data: &str) -> Result<Vec<u8>, JsValue> {
        // IMPLEMENT: Real JSON parsing and rendering
        let renderer = self.renderer.as_mut()
            .ok_or_else(|| JsValue::from_str("Renderer not initialized"))?;

        let data: serde_json::Value = serde_json::from_str(json_data)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let bar_data = BarChartData::from_json(&data)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        renderer.render_bar_chart(&bar_data)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
```

#### Day 25-27: Demo Application
**File**: `helios-app/src/main.rs` (NEW, <100 lines)

```rust
//! Simple demo application showing working bar chart
//! Status: 🚧 IN PROGRESS

use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (chart_data, set_chart_data) = create_signal(None::<Vec<u8>>);

    let render_chart = create_action(|_: &()| async move {
        // IMPLEMENT: Real chart rendering demo
        let data = serde_json::json!({
            "labels": ["Jan", "Feb", "Mar", "Apr", "May"],
            "values": [100, 120, 110, 130, 125]
        });

        let mut chart = WasmBarChart::new();
        chart.init(800, 600).await?;
        let png_data = chart.render_from_json(&data.to_string()).await?;

        Ok::<Vec<u8>, JsValue>(png_data)
    });

    view! {
        <div>
            <h1>"Leptos Helios MVP Demo"</h1>
            <button on:click=move |_| render_chart.dispatch(())>
                "Render Bar Chart"
            </button>

            {move || match chart_data.get() {
                Some(data) => {
                    // Convert PNG bytes to data URL
                    let b64 = base64::encode(&data);
                    let data_url = format!("data:image/png;base64,{}", b64);
                    view! {
                        <img src={data_url} alt="Generated chart" />
                    }.into_view()
                },
                None => view! { <p>"Click to render chart"</p> }.into_view()
            }}
        </div>
    }
}
```

#### Day 28-30: Testing and Validation
**File**: `tests/integration/mvp_test.rs` (NEW, <100 lines)

```rust
//! Integration test for MVP functionality
//! Status: 🚧 IN PROGRESS

#[tokio::test]
async fn test_end_to_end_bar_chart() {
    // Test the full pipeline: data → geometry → rendering → output
    let data = BarChartData {
        labels: vec!["A".to_string(), "B".to_string(), "C".to_string()],
        values: vec![10.0, 20.0, 15.0],
        max_value: 20.0,
    };

    let mut renderer = WebGpuRenderer::new(800, 600)
        .await
        .expect("Renderer should initialize");

    let png_data = renderer.render_bar_chart(&data)
        .await
        .expect("Rendering should succeed");

    // Verify PNG format
    assert_eq!(&png_data[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]); // PNG magic
    assert!(png_data.len() > 1000); // Reasonable size

    // Save for manual inspection
    std::fs::write("test_output.png", &png_data).unwrap();
}
```

## Success Criteria

At the end of 30 days, we must have:

1. **Working WebGPU renderer** that can display basic bar charts
2. **Data pipeline** that converts DataFrame → chart geometry
3. **WASM integration** that works in browsers
4. **Demo application** showing end-to-end functionality
5. **Integration tests** proving the pipeline works
6. **PNG output** that can be verified manually

## Risk Mitigation

**High Risk**: WebGPU compatibility issues
- **Mitigation**: Test on Chrome/Firefox early, have Canvas2D backup plan

**Medium Risk**: Complex coordinate transformations
- **Mitigation**: Start with simple normalized coordinates, validate with known data

**Low Risk**: WASM compilation issues
- **Mitigation**: Use established wasm-bindgen patterns

## Files Modified/Created

**New Files** (~8 files, <1500 total lines):
- `helios-core/src/webgpu/device.rs`
- `helios-core/src/webgpu/pipeline.rs`
- `helios-core/src/webgpu/shaders/bar_chart.wgsl`
- `helios-core/src/chart/bar_geometry.rs`
- `helios-app/src/main.rs`
- `tests/integration/mvp_test.rs`

**Refactored Files** (~3 files, remove stubs, add real implementation):
- `helios-core/src/webgpu/renderer.rs`
- `helios-core/src/data/processor.rs`
- `helios-wasm/src/lib.rs`

**Total Effort**: ~15-20 person-days focused implementation work
**Outcome**: Proof that the architecture can work, foundation for further development
