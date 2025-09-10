# Rust xyflow Design Document
## Leveraging Leptos Helios Architecture for Node-Based UI

### Executive Summary

This document outlines the design for `rust-xyflow`, a high-performance node-based UI library built in Rust that leverages the proven architecture, performance optimizations, and multi-framework support patterns from the Leptos Helios ecosystem.

## 🎯 **Strategic Advantages from Leptos Helios**

### **1. Proven Architecture Patterns**
- **Modular workspace structure** with clear separation of concerns
- **Framework-agnostic core** with framework-specific integrations
- **WASM bindings** for universal web compatibility
- **Compile-time validation** and optimization macros

### **2. Performance Infrastructure**
- **WebGPU + Canvas2D** rendering pipeline
- **Memory pool management** and efficient allocation
- **SIMD optimizations** for geometric calculations
- **Multi-threaded data processing** capabilities

### **3. Developer Experience**
- **Type-safe specifications** with compile-time validation
- **Comprehensive testing framework** (1000+ tests)
- **Performance benchmarking** and profiling tools
- **Cross-browser compatibility** testing

### **4. Production-Ready Features**
- **Export capabilities** (PNG, SVG, PDF)
- **Accessibility support** (WCAG 2.1 AA)
- **Responsive design** system
- **Animation and interaction** framework

## 🏗️ **Architecture Design**

### **Repository Structure**
```
rust-xyflow/
├── xyflow-core/           # Core node-based UI engine
├── xyflow-leptos/         # Leptos integration
├── xyflow-wasm/           # WASM bindings (universal)
├── xyflow-macros/         # Compile-time utilities
├── xyflow-examples/       # Examples and demos
├── xyflow-benchmarks/     # Performance testing
└── shared/                # Shared utilities with helios
```

### **Core Dependencies (Leveraging Helios)**
```toml
# xyflow-core/Cargo.toml
[dependencies]
# Rendering (from helios-core)
wgpu = { workspace = true }
bytemuck = "1.14"

# Performance (from helios-core)
rayon = "1.8"
bincode = "1.3"

# Serialization (from helios-core)
serde = { workspace = true }
serde_json = { workspace = true }

# Error handling (from helios-core)
thiserror = { workspace = true }
anyhow = { workspace = true }

# Utilities (from helios-core)
uuid = { workspace = true }
chrono = { workspace = true }
rand = { workspace = true }

# Web/WASM (from helios-core)
wasm-bindgen = { workspace = true }
web-sys = { workspace = true }
js-sys = { workspace = true }
```

## 🎨 **Core Data Structures**

### **Node System**
```rust
// xyflow-core/src/node.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Core node representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub position: Position,
    pub data: NodeData,
    pub node_type: NodeType,
    pub selected: bool,
    pub dragging: bool,
    pub style: NodeStyle,
    pub class_name: Option<String>,
    pub hidden: bool,
    pub parent_node: Option<String>,
    pub extent: Option<Extent>,
    pub expand_parent: Option<bool>,
    pub position_absolute: Option<Position>,
    pub dragging: bool,
    pub target_position: Option<Position>,
    pub source_position: Option<Position>,
}

/// Node position with precision
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

/// Node data container
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeData {
    pub label: Option<String>,
    pub custom_data: serde_json::Value,
}

/// Node types (extensible)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeType {
    Input,
    Output,
    Default,
    Group,
    Custom(String),
}

/// Node styling
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeStyle {
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub background_color: Option<String>,
    pub border_color: Option<String>,
    pub border_width: Option<f32>,
    pub border_radius: Option<f32>,
    pub opacity: Option<f32>,
    pub z_index: Option<i32>,
}
```

### **Edge System**
```rust
// xyflow-core/src/edge.rs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub source_handle: Option<String>,
    pub target_handle: Option<String>,
    pub label: Option<String>,
    pub label_style: Option<LabelStyle>,
    pub label_show_bg: Option<bool>,
    pub label_bg_style: Option<LabelBgStyle>,
    pub style: Option<EdgeStyle>,
    pub animated: Option<bool>,
    pub hidden: bool,
    pub deletable: Option<bool>,
    pub focusable: Option<bool>,
    pub data: Option<serde_json::Value>,
    pub class_name: Option<String>,
    pub source_x: Option<f32>,
    pub source_y: Option<f32>,
    pub target_x: Option<f32>,
    pub target_y: Option<f32>,
    pub selected: bool,
    pub marker_start: Option<MarkerType>,
    pub marker_end: Option<MarkerType>,
    pub path_options: Option<PathOptions>,
    pub interaction_width: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MarkerType {
    Arrow,
    ArrowClosed,
    Circle,
    CircleClosed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PathOptions {
    pub offset: Option<f32>,
    pub radius: Option<f32>,
    pub curvature: Option<f32>,
}
```

### **Flow System**
```rust
// xyflow-core/src/flow.rs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Flow {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub viewport: Viewport,
    pub default_edge_options: Option<EdgeOptions>,
    pub default_node_options: Option<NodeOptions>,
    pub fit_view_on_init: Option<bool>,
    pub fit_view_on_init_options: Option<FitViewOptions>,
    pub only_render_visible_elements: Option<bool>,
    pub nodes_draggable: Option<bool>,
    pub nodes_connectable: Option<bool>,
    pub elements_selectable: Option<bool>,
    pub select_nodes_on_drag: Option<bool>,
    pub pan_on_drag: Option<bool>,
    pub pan_on_scroll: Option<bool>,
    pub pan_on_scroll_mode: Option<PanOnScrollMode>,
    pub zoom_on_scroll: Option<bool>,
    pub zoom_on_pinch: Option<bool>,
    pub zoom_on_double_click: Option<bool>,
    pub prevent_scrolling: Option<bool>,
    pub node_extent: Option<Extent>,
    pub translate_extent: Option<Extent>,
    pub node_origin: Option<Origin>,
    pub default_marker_color: Option<String>,
    pub connection_mode: Option<ConnectionMode>,
    pub snap_to_grid: Option<bool>,
    pub snap_grid: Option<SnapGrid>,
    pub connect_on_click: Option<bool>,
    pub auto_pan_on_connect: Option<bool>,
    pub auto_pan_on_node_drag: Option<bool>,
    pub connection_radius: Option<f32>,
    pub on_init: Option<Callback>,
    pub on_click: Option<Callback>,
    pub on_node_click: Option<Callback>,
    pub on_edge_click: Option<Callback>,
    pub on_node_double_click: Option<Callback>,
    pub on_edge_double_click: Option<Callback>,
    pub on_node_mouse_enter: Option<Callback>,
    pub on_node_mouse_move: Option<Callback>,
    pub on_node_mouse_leave: Option<Callback>,
    pub on_node_context_menu: Option<Callback>,
    pub on_edge_context_menu: Option<Callback>,
    pub on_selection_change: Option<Callback>,
    pub on_nodes_change: Option<Callback>,
    pub on_edges_change: Option<Callback>,
    pub on_connect: Option<Callback>,
    pub on_connect_start: Option<Callback>,
    pub on_connect_end: Option<Callback>,
    pub on_pane_click: Option<Callback>,
    pub on_pane_scroll: Option<Callback>,
    pub on_pane_context_menu: Option<Callback>,
    pub on_move: Option<Callback>,
    pub on_move_start: Option<Callback>,
    pub on_move_end: Option<Callback>,
    pub on_selection_drag: Option<Callback>,
    pub on_selection_drag_start: Option<Callback>,
    pub on_selection_drag_stop: Option<Callback>,
    pub on_selection_context_menu: Option<Callback>,
    pub on_viewport_change: Option<Callback>,
    pub on_viewport_change_start: Option<Callback>,
    pub on_viewport_change_end: Option<Callback>,
    pub on_nodes_delete: Option<Callback>,
    pub on_edges_delete: Option<Callback>,
    pub on_delete: Option<Callback>,
    pub on_intersection_change: Option<Callback>,
    pub on_error: Option<Callback>,
    pub on_cleanup: Option<Callback>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Extent {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}
```

## 🎨 **Rendering System (Leveraging Helios)**

### **WebGPU Renderer**
```rust
// xyflow-core/src/renderer/webgpu.rs
use wgpu::*;
use bytemuck::{Pod, Zeroable};

/// WebGPU-based node renderer
pub struct WebGpuNodeRenderer {
    device: Device,
    queue: Queue,
    surface: Surface,
    render_pipeline: RenderPipeline,
    node_buffer: Buffer,
    edge_buffer: Buffer,
    uniform_buffer: Buffer,
    bind_group: BindGroup,
}

impl WebGpuNodeRenderer {
    pub fn new() -> Result<Self, RenderError> {
        // Leverage helios-core WebGPU setup
        let (device, queue, surface) = Self::create_device().await?;

        let render_pipeline = Self::create_render_pipeline(&device)?;
        let (node_buffer, edge_buffer, uniform_buffer, bind_group) =
            Self::create_buffers(&device)?;

        Ok(Self {
            device,
            queue,
            surface,
            render_pipeline,
            node_buffer,
            edge_buffer,
            uniform_buffer,
            bind_group,
        })
    }

    pub fn render_nodes(&mut self, nodes: &[Node], viewport: &Viewport) -> Result<(), RenderError> {
        // Leverage helios-core rendering patterns
        self.update_node_buffer(nodes)?;
        self.update_uniform_buffer(viewport)?;
        self.render()?;
        Ok(())
    }
}
```

### **Canvas2D Fallback**
```rust
// xyflow-core/src/renderer/canvas2d.rs
use web_sys::CanvasRenderingContext2d;

/// Canvas2D-based node renderer (fallback)
pub struct Canvas2DNodeRenderer {
    context: CanvasRenderingContext2d,
    canvas_width: f32,
    canvas_height: f32,
}

impl Canvas2DNodeRenderer {
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Self, RenderError> {
        let context = canvas.get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        Ok(Self {
            context,
            canvas_width: canvas.width() as f32,
            canvas_height: canvas.height() as f32,
        })
    }

    pub fn render_nodes(&mut self, nodes: &[Node], viewport: &Viewport) -> Result<(), RenderError> {
        // Leverage helios-core Canvas2D patterns
        self.clear_canvas()?;
        self.apply_viewport_transform(viewport)?;

        for node in nodes {
            self.render_node(node)?;
        }

        Ok(())
    }
}
```

## 🧠 **Performance Optimizations (From Helios)**

### **Memory Pool Management**
```rust
// xyflow-core/src/memory.rs
use std::collections::VecDeque;

/// Memory pool for efficient node/edge allocation
pub struct NodeMemoryPool {
    available_nodes: VecDeque<Node>,
    available_edges: VecDeque<Edge>,
    pool_size: usize,
}

impl NodeMemoryPool {
    pub fn new(pool_size: usize) -> Self {
        Self {
            available_nodes: VecDeque::with_capacity(pool_size),
            available_edges: VecDeque::with_capacity(pool_size),
            pool_size,
        }
    }

    pub fn get_node(&mut self) -> Node {
        self.available_nodes.pop_front()
            .unwrap_or_else(|| Node::default())
    }

    pub fn return_node(&mut self, mut node: Node) {
        if self.available_nodes.len() < self.pool_size {
            node.reset(); // Reset to default state
            self.available_nodes.push_back(node);
        }
    }
}
```

### **Spatial Indexing**
```rust
// xyflow-core/src/spatial.rs
use std::collections::HashMap;

/// Spatial index for efficient node queries
pub struct SpatialIndex {
    grid: HashMap<(i32, i32), Vec<String>>, // (grid_x, grid_y) -> node_ids
    cell_size: f32,
    bounds: Extent,
}

impl SpatialIndex {
    pub fn new(cell_size: f32, bounds: Extent) -> Self {
        Self {
            grid: HashMap::new(),
            cell_size,
            bounds,
        }
    }

    pub fn insert_node(&mut self, node: &Node) {
        let grid_pos = self.world_to_grid(node.position);
        self.grid.entry(grid_pos).or_insert_with(Vec::new).push(node.id.clone());
    }

    pub fn query_nodes_in_rect(&self, rect: Extent) -> Vec<String> {
        let mut result = Vec::new();
        let min_grid = self.world_to_grid((rect.min_x, rect.min_y));
        let max_grid = self.world_to_grid((rect.max_x, rect.max_y));

        for x in min_grid.0..=max_grid.0 {
            for y in min_grid.1..=max_grid.1 {
                if let Some(node_ids) = self.grid.get(&(x, y)) {
                    result.extend(node_ids.iter().cloned());
                }
            }
        }

        result
    }
}
```

## 🔧 **Framework Integrations**

### **Leptos Integration**
```rust
// xyflow-leptos/src/lib.rs
use leptos::*;
use xyflow_core::*;

#[component]
pub fn FlowEditor(
    nodes: ReadSignal<Vec<Node>>,
    edges: ReadSignal<Vec<Edge>>,
    on_nodes_change: Option<Callback<Vec<NodeChange>>>,
    on_edges_change: Option<Callback<Vec<EdgeChange>>>,
    on_connect: Option<Callback<ConnectionParams>>,
) -> impl IntoView {
    let flow_ref = NodeRef::<leptos::html::Div>::new();

    // Initialize flow editor
    create_effect(move |_| {
        if let Some(div) = flow_ref.get() {
            let flow = FlowEditor::new(&div);
            flow.set_nodes(nodes.get());
            flow.set_edges(edges.get());

            if let Some(on_nodes_change) = on_nodes_change {
                flow.on_nodes_change(move |changes| {
                    on_nodes_change.call(changes);
                });
            }
        }
    });

    view! {
        <div node_ref=flow_ref class="xyflow-editor" />
    }
}

#[component]
pub fn Node(
    id: String,
    position: Position,
    data: NodeData,
    #[prop(optional)] node_type: NodeType,
    #[prop(optional)] style: Option<NodeStyle>,
) -> impl IntoView {
    view! {
        <div
            class="xyflow-node"
            data-id=id
            style=format!("left: {}px; top: {}px;", position.x, position.y)
        >
            {data.label.unwrap_or_default()}
        </div>
    }
}
```

### **WASM Bindings (Universal)**
```rust
// xyflow-wasm/src/lib.rs
use wasm_bindgen::prelude::*;
use xyflow_core::*;

#[wasm_bindgen]
pub struct FlowEditor {
    flow: Flow,
    renderer: Box<dyn NodeRenderer>,
    spatial_index: SpatialIndex,
    memory_pool: NodeMemoryPool,
}

#[wasm_bindgen]
impl FlowEditor {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: &HtmlCanvasElement) -> Result<FlowEditor, JsValue> {
        // Try WebGPU first, fallback to Canvas2D
        let renderer: Box<dyn NodeRenderer> = match WebGpuNodeRenderer::new().await {
            Ok(renderer) => Box::new(renderer),
            Err(_) => Box::new(Canvas2DNodeRenderer::new(canvas)?),
        };

        Ok(FlowEditor {
            flow: Flow::default(),
            renderer,
            spatial_index: SpatialIndex::new(100.0, Extent::infinite()),
            memory_pool: NodeMemoryPool::new(1000),
        })
    }

    #[wasm_bindgen]
    pub fn add_node(&mut self, node: JsValue) -> Result<(), JsValue> {
        let node: Node = serde_wasm_bindgen::from_value(node)?;
        self.flow.nodes.push(node.clone());
        self.spatial_index.insert_node(&node);
        self.render()?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn add_edge(&mut self, edge: JsValue) -> Result<(), JsValue> {
        let edge: Edge = serde_wasm_bindgen::from_value(edge)?;
        self.flow.edges.push(edge);
        self.render()?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn render(&mut self) -> Result<(), JsValue> {
        self.renderer.render_nodes(&self.flow.nodes, &self.flow.viewport)?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn export_svg(&self) -> Result<String, JsValue> {
        // Leverage helios-core export system
        let svg = self.renderer.export_svg(&self.flow.nodes, &self.flow.edges)?;
        Ok(svg)
    }
}
```

## 🧪 **Testing Strategy (From Helios)**

### **Comprehensive Test Suite**
```rust
// xyflow-core/tests/integration_tests.rs
use proptest::prelude::*;
use xyflow_core::*;

proptest! {
    #[test]
    fn test_node_creation_properties(
        x in -1000.0f32..1000.0f32,
        y in -1000.0f32..1000.0f32,
        label in ".*"
    ) {
        let position = Position { x, y };
        let data = NodeData {
            label: Some(label.clone()),
            custom_data: serde_json::Value::Null
        };

        let node = Node {
            id: Uuid::new_v4().to_string(),
            position,
            data,
            node_type: NodeType::Default,
            selected: false,
            dragging: false,
            style: NodeStyle::default(),
            class_name: None,
            hidden: false,
            parent_node: None,
            extent: None,
            expand_parent: None,
            position_absolute: None,
            target_position: None,
            source_position: None,
        };

        assert_eq!(node.position.x, x);
        assert_eq!(node.position.y, y);
        assert_eq!(node.data.label, Some(label));
    }
}

#[test]
fn test_spatial_index_performance() {
    let mut index = SpatialIndex::new(100.0, Extent::infinite());
    let mut nodes = Vec::new();

    // Create 1000 nodes
    for i in 0..1000 {
        let node = Node {
            id: i.to_string(),
            position: Position { x: i as f32, y: i as f32 },
            data: NodeData { label: None, custom_data: serde_json::Value::Null },
            node_type: NodeType::Default,
            selected: false,
            dragging: false,
            style: NodeStyle::default(),
            class_name: None,
            hidden: false,
            parent_node: None,
            extent: None,
            expand_parent: None,
            position_absolute: None,
            target_position: None,
            source_position: None,
        };
        nodes.push(node);
    }

    // Insert all nodes
    for node in &nodes {
        index.insert_node(node);
    }

    // Query performance test
    let start = std::time::Instant::now();
    let query_rect = Extent {
        min_x: 0.0,
        min_y: 0.0,
        max_x: 100.0,
        max_y: 100.0,
    };
    let results = index.query_nodes_in_rect(query_rect);
    let duration = start.elapsed();

    assert!(duration.as_millis() < 10); // Should be very fast
    assert!(!results.is_empty());
}
```

## 🚀 **Fast Development Strategy**

### **Phase 1: Core Foundation (Week 1)**
1. **Setup workspace** with proven helios-core patterns
2. **Implement basic data structures** (Node, Edge, Flow)
3. **Create Canvas2D renderer** (fastest to implement)
4. **Basic interaction handling** (click, drag, zoom)

### **Phase 2: Rendering Pipeline (Week 2)**
1. **WebGPU renderer** (leverage helios-core WebGPU setup)
2. **Performance optimizations** (spatial indexing, memory pools)
3. **Animation system** (leverage helios-core animation patterns)
4. **Export capabilities** (SVG, PNG)

### **Phase 3: Framework Integration (Week 3)**
1. **Leptos integration** (reactive components)
2. **WASM bindings** (universal web support)
3. **React/Vue/Angular** compatibility via WASM
4. **Desktop integration** (Tauri, Bevy)

### **Phase 4: Advanced Features (Week 4)**
1. **Custom node types** and edge types
2. **Advanced interactions** (multi-select, keyboard shortcuts)
3. **Performance benchmarking** and optimization
4. **Documentation** and examples

## 📊 **Performance Targets**

### **Benchmarks (Leveraging Helios Infrastructure)**
```rust
// xyflow-benchmarks/benches/performance.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use xyflow_core::*;

fn benchmark_node_rendering(c: &mut Criterion) {
    let mut renderer = Canvas2DNodeRenderer::new(&canvas).unwrap();
    let nodes = create_test_nodes(1000);

    c.bench_function("render_1000_nodes", |b| {
        b.iter(|| {
            renderer.render_nodes(black_box(&nodes), black_box(&Viewport::default()))
        })
    });
}

fn benchmark_spatial_queries(c: &mut Criterion) {
    let mut index = SpatialIndex::new(100.0, Extent::infinite());
    let nodes = create_test_nodes(10000);

    for node in &nodes {
        index.insert_node(node);
    }

    c.bench_function("spatial_query_10000_nodes", |b| {
        b.iter(|| {
            index.query_nodes_in_rect(black_box(Extent {
                min_x: 0.0,
                min_y: 0.0,
                max_x: 100.0,
                max_y: 100.0,
            }))
        })
    });
}

criterion_group!(benches, benchmark_node_rendering, benchmark_spatial_queries);
criterion_main!(benches);
```

## 🎯 **Success Metrics**

### **Performance Goals**
- **Render 1000 nodes** at 60 FPS
- **Spatial queries** in <1ms for 10,000 nodes
- **Memory usage** <50MB for 1000 nodes
- **Bundle size** <500KB (WASM + JS)

### **Feature Parity**
- **100% API compatibility** with xyflow
- **All node types** (Input, Output, Default, Group, Custom)
- **All edge types** (Default, Step, SmoothStep, Straight, Bezier)
- **All interactions** (drag, zoom, pan, select, connect)

### **Framework Support**
- **Leptos** (native integration)
- **React/Vue/Angular** (via WASM)
- **Desktop** (Tauri, Bevy)
- **Universal** (vanilla JS)

## 🚀 **Implementation Timeline**

### **Week 1: Foundation**
- [ ] Workspace setup
- [ ] Core data structures
- [ ] Basic Canvas2D renderer
- [ ] Simple interactions

### **Week 2: Rendering**
- [ ] WebGPU renderer
- [ ] Performance optimizations
- [ ] Animation system
- [ ] Export capabilities

### **Week 3: Integration**
- [ ] Leptos components
- [ ] WASM bindings
- [ ] Framework compatibility
- [ ] Desktop support

### **Week 4: Polish**
- [ ] Advanced features
- [ ] Performance tuning
- [ ] Documentation
- [ ] Examples and demos

## 💡 **Key Advantages**

1. **🚀 Fast Development**: Leverage proven helios-core patterns
2. **⚡ High Performance**: Rust + WebGPU + optimized algorithms
3. **🌐 Universal**: Works with any framework via WASM
4. **🛡️ Type Safe**: Compile-time validation and optimization
5. **🧪 Well Tested**: Comprehensive test suite from day one
6. **📚 Documented**: Clear API and examples
7. **🔧 Extensible**: Plugin system for custom node types

This design leverages all the wins from Leptos Helios while creating a focused, high-performance node-based UI library that can be built quickly and efficiently! 🎯
