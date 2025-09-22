# 🦀 Rust/WASM Demos Only - leptos-helios

## ✅ **Clean Repository Status**

This repository now contains **ONLY Rust/WASM demos** - all simple CSS/JS demos have been removed.

### **Removed Non-Rust Demos:**
- ❌ `demo-server.py` - Python demo server
- ❌ `example.html` - Simple HTML demo
- ❌ `index.html` - Basic HTML demo
- ❌ `realtime-demo.html` - HTML realtime demo
- ❌ `webgpu-test.html` - HTML WebGPU test
- ❌ `working_example.html` - HTML working example
- ❌ `package.json` - Node.js dependencies
- ❌ `playwright.config.js` - JavaScript config
- ❌ `helios-demo-v0.2.0/` - Old demo directory with HTML/Python
- ❌ All shell scripts launching non-Rust demos
- ❌ All Python demo servers

### **Kept Rust/WASM Demos:**
- ✅ `helios-examples/` - Rust example applications
- ✅ `helios-core/examples/` - Core Rust examples
- ✅ `helios-wasm/` - WebAssembly bindings
- ✅ `helios-wasm-core/` - Core WASM functionality
- ✅ `helios-leptos/` - Leptos components
- ✅ `helios-app/` - Rust application (Trunk-based)
- ✅ `examples/` - Rust example files

---

## 🚀 **Available Rust/WASM Demos**

### **1. Core Examples (`helios-core/examples/`)**
```rust
// WebGPU demo with Rust
use leptos_helios::chart::*;

pub fn webgpu_demo() {
    let chart_spec = ChartSpec::new();
    // WebGPU rendering with Rust
}
```

### **2. Leptos Examples (`helios-examples/`)**
```rust
// Leptos components with WASM
use leptos::*;
use leptos_helios::*;

#[component]
pub fn StylishChart() -> impl IntoView {
    view! {
        <div class="chart-container">
            <HeliosChart spec=chart_spec />
        </div>
    }
}
```

### **3. WASM Examples (`helios-wasm/`)**
```rust
// WebAssembly bindings
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn create_chart(data: &[f64]) -> Result<JsValue, JsValue> {
    // WASM chart creation
}
```

### **4. Application Demo (`helios-app/`)**
```rust
// Full Rust application with Trunk
use leptos::*;
use leptos_helios::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <h1>"leptos-helios Demo"</h1>
            <ChartContainer />
        </main>
    }
}
```

---

## 🎯 **How to Run Rust/WASM Demos**

### **1. Core Examples**
```bash
cd helios-core
cargo run --example webgpu_demo
```

### **2. Leptos Examples**
```bash
cd helios-examples
cargo run --example stylish_demo
```

### **3. WASM Examples**
```bash
cd helios-wasm
wasm-pack build --target web
```

### **4. Full Application**
```bash
cd helios-app
trunk serve
```

---

## 🏗️ **Architecture**

All demos are built with:

- **🦀 Rust** - Core language
- **⚡ Leptos v0.8** - Reactive framework
- **🌐 WebAssembly** - Browser execution
- **🎨 WebGPU** - GPU acceleration
- **📦 Trunk** - Build system
- **🎯 Type Safety** - Compile-time validation

---

## 🎨 **Stylish Charts Demo**

The new `stylish_demo.rs` shows how to create beautiful charts by combining:

- **leptos-helios** - Chart rendering engine
- **leptos-shadcn-ui** - Modern UI components
- **Tailwind CSS** - Styling system
- **WebGPU** - GPU acceleration

### **Features:**
- 🎨 Modern, beautiful chart designs
- 📱 Responsive layouts
- 🌙 Dark/light themes
- ⚡ Interactive controls
- 🚀 GPU-accelerated rendering
- ♿ Accessibility features

---

## ✅ **Repository Status**

- **✅ Rust/WASM Only** - No simple CSS/JS demos
- **✅ Type Safety** - All demos are type-checked
- **✅ Performance** - WebGPU acceleration
- **✅ Modern** - Latest Rust/Leptos features
- **✅ Production Ready** - Enterprise-grade demos

This repository now focuses exclusively on **high-performance, type-safe Rust/WASM visualizations** with modern styling and GPU acceleration.
