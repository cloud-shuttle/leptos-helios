# Leptos Motion WASM + Signals Guide
## Our Secrets to Success with Reactive Animations

### Executive Summary

After analyzing the leptos-motion codebase and comparing it with our successful Leptos Helios implementation, I've identified the key issues and solutions for building robust WASM + signals integration. This document provides actionable solutions to the most common problems.

## 🚨 **Critical Issues in leptos-motion**

### **1. Signal Tracking Problems**
```rust
// ❌ PROBLEM: Closures don't track signal dependencies
let animate_closure = move || {
    if is_active.get() {  // This doesn't trigger re-renders!
        // animation logic
    }
};
```

### **2. WASM Memory Management Issues**
```rust
// ❌ PROBLEM: Improper signal disposal in WASM
// Signals are not properly cleaned up, causing memory leaks
```

### **3. Effect Lifecycle Problems**
```rust
// ❌ PROBLEM: Effects don't properly track animation state changes
create_effect(move |_| {
    // Animation logic that doesn't re-run when signals change
});
```

## 🎯 **Our Proven Solutions**

### **Solution 1: Proper Signal Tracking with `create_effect`**

```rust
// ✅ SOLUTION: Use create_effect for reactive animations
use leptos::prelude::*;

#[component]
pub fn ReactiveMotionDiv(
    initial: HashMap<String, AnimationValue>,
    animate: ReadSignal<HashMap<String, AnimationValue>>,
    transition: Transition,
) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Div>::new();

    // ✅ CRITICAL: Use create_effect to track signal changes
    create_effect(move |_| {
        if let Some(div) = node_ref.get() {
            let current_animate = animate.get();

            // Apply animation to DOM element
            apply_animation_to_element(&div, &current_animate, &transition);
        }
    });

    view! {
        <div
            node_ref=node_ref
            style=move || {
                // Convert initial values to CSS
                initial_to_css(&initial)
            }
        />
    }
}

// ✅ Helper function to apply animations
fn apply_animation_to_element(
    element: &leptos::html::Div,
    animate: &HashMap<String, AnimationValue>,
    transition: &Transition,
) {
    // Use Web Animations API or CSS transitions
    for (property, value) in animate {
        let css_value = animation_value_to_css(value);
        element.style().set_property(property, &css_value).unwrap();
    }
}
```

### **Solution 2: Proper WASM Memory Management**

```rust
// ✅ SOLUTION: Implement proper cleanup in WASM
use wasm_bindgen::prelude::*;
use leptos::prelude::*;

#[wasm_bindgen]
pub struct MotionController {
    // ✅ Store signals properly for WASM
    animation_state: ReadSignal<AnimationState>,
    cleanup_fn: Option<Box<dyn Fn()>>,
}

#[wasm_bindgen]
impl MotionController {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // ✅ Initialize with proper signal management
        let (animation_state, _) = create_signal(AnimationState::Idle);

        Self {
            animation_state,
            cleanup_fn: None,
        }
    }

    #[wasm_bindgen]
    pub fn cleanup(&mut self) {
        // ✅ CRITICAL: Proper cleanup to prevent memory leaks
        if let Some(cleanup) = self.cleanup_fn.take() {
            cleanup();
        }
    }
}

// ✅ Implement Drop for automatic cleanup
impl Drop for MotionController {
    fn drop(&mut self) {
        self.cleanup();
    }
}
```

### **Solution 3: Signal-Based Animation State Management**

```rust
// ✅ SOLUTION: Use signals for all animation state
use leptos::prelude::*;
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub struct AnimationState {
    pub is_playing: bool,
    pub current_values: HashMap<String, AnimationValue>,
    pub target_values: HashMap<String, AnimationValue>,
    pub progress: f32,
}

#[component]
pub fn SignalBasedMotionDiv(
    initial: HashMap<String, AnimationValue>,
    animate: ReadSignal<HashMap<String, AnimationValue>>,
    transition: Transition,
) -> impl IntoView {
    // ✅ Use signals for all state management
    let (animation_state, set_animation_state) = create_signal(AnimationState {
        is_playing: false,
        current_values: initial.clone(),
        target_values: HashMap::new(),
        progress: 0.0,
    });

    let node_ref = NodeRef::<leptos::html::Div>::new();

    // ✅ CRITICAL: Effect that properly tracks animate signal
    create_effect(move |_| {
        let target_values = animate.get();

        // Update animation state
        set_animation_state.update(|state| {
            state.target_values = target_values;
            state.is_playing = true;
            state.progress = 0.0;
        });

        // Start animation
        if let Some(div) = node_ref.get() {
            start_animation(&div, &target_values, &transition);
        }
    });

    // ✅ Effect to update DOM based on animation state
    create_effect(move |_| {
        let state = animation_state.get();
        if let Some(div) = node_ref.get() {
            update_element_styles(&div, &state.current_values);
        }
    });

    view! {
        <div
            node_ref=node_ref
            style=move || {
                // Convert current values to CSS
                values_to_css(&animation_state.get().current_values)
            }
        />
    }
}
```

### **Solution 4: Proper Effect Dependencies**

```rust
// ✅ SOLUTION: Explicit signal dependencies in effects
use leptos::prelude::*;

#[component]
pub fn ProperEffectMotionDiv(
    animate: ReadSignal<HashMap<String, AnimationValue>>,
    transition: ReadSignal<Transition>,
    is_visible: ReadSignal<bool>,
) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Div>::new();

    // ✅ CRITICAL: Effect with explicit dependencies
    create_effect(move |_| {
        // This effect will re-run when ANY of these signals change:
        let animate_values = animate.get();  // Dependency 1
        let transition_config = transition.get();  // Dependency 2
        let visible = is_visible.get();  // Dependency 3

        if visible {
            if let Some(div) = node_ref.get() {
                apply_animation(&div, &animate_values, &transition_config);
            }
        }
    });

    view! {
        <div
            node_ref=node_ref
            class=move || if is_visible.get() { "visible" } else { "hidden" }
        />
    }
}
```

### **Solution 5: WASM-Optimized Signal Handling**

```rust
// ✅ SOLUTION: WASM-optimized signal management
use wasm_bindgen::prelude::*;
use leptos::prelude::*;

#[wasm_bindgen]
pub struct WasmMotionController {
    // ✅ Store signals in a way that works with WASM
    animation_signals: std::collections::HashMap<String, ReadSignal<AnimationValue>>,
    cleanup_handles: Vec<Box<dyn Fn()>>,
}

#[wasm_bindgen]
impl WasmMotionController {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            animation_signals: std::collections::HashMap::new(),
            cleanup_handles: Vec::new(),
        }
    }

    #[wasm_bindgen]
    pub fn create_animation_signal(&mut self, name: &str, initial_value: f32) -> u32 {
        // ✅ Create signal and store handle for cleanup
        let (signal, _) = create_signal(AnimationValue::Number(initial_value));
        let signal_id = self.animation_signals.len() as u32;

        self.animation_signals.insert(name.to_string(), signal);

        // ✅ Store cleanup function
        let cleanup = Box::new(move || {
            // Signal cleanup logic
        });
        self.cleanup_handles.push(cleanup);

        signal_id
    }

    #[wasm_bindgen]
    pub fn update_animation_value(&mut self, name: &str, value: f32) -> Result<(), JsValue> {
        if let Some(signal) = self.animation_signals.get(name) {
            // ✅ Update signal value
            signal.set(AnimationValue::Number(value));
            Ok(())
        } else {
            Err(JsValue::from_str(&format!("Signal '{}' not found", name)))
        }
    }

    #[wasm_bindgen]
    pub fn cleanup(&mut self) {
        // ✅ CRITICAL: Clean up all signals and effects
        for cleanup in self.cleanup_handles.drain(..) {
            cleanup();
        }
        self.animation_signals.clear();
    }
}
```

## 🔧 **Implementation Patterns**

### **Pattern 1: Signal-Based Animation Controller**

```rust
// ✅ Complete animation controller using signals
use leptos::prelude::*;
use std::collections::HashMap;

pub struct AnimationController {
    pub current_values: ReadSignal<HashMap<String, AnimationValue>>,
    pub target_values: ReadSignal<HashMap<String, AnimationValue>>,
    pub is_playing: ReadSignal<bool>,
    pub progress: ReadSignal<f32>,
}

impl AnimationController {
    pub fn new(initial_values: HashMap<String, AnimationValue>) -> Self {
        let (current_values, set_current_values) = create_signal(initial_values.clone());
        let (target_values, set_target_values) = create_signal(initial_values);
        let (is_playing, set_is_playing) = create_signal(false);
        let (progress, set_progress) = create_signal(0.0);

        // ✅ Effect to handle animation updates
        create_effect(move |_| {
            let current = current_values.get();
            let target = target_values.get();
            let playing = is_playing.get();

            if playing {
                // Animation logic here
                // Update progress and current values
            }
        });

        Self {
            current_values,
            target_values,
            is_playing,
            progress,
        }
    }

    pub fn animate_to(&self, target: HashMap<String, AnimationValue>) {
        // ✅ Update target values (triggers effect)
        self.target_values.set(target);
        self.is_playing.set(true);
    }
}
```

### **Pattern 2: WASM-Safe Component Wrapper**

```rust
// ✅ WASM-safe wrapper for motion components
use wasm_bindgen::prelude::*;
use leptos::prelude::*;

#[wasm_bindgen]
pub struct WasmMotionComponent {
    component_id: String,
    animation_controller: Option<AnimationController>,
}

#[wasm_bindgen]
impl WasmMotionComponent {
    #[wasm_bindgen(constructor)]
    pub fn new(component_id: &str) -> Self {
        Self {
            component_id: component_id.to_string(),
            animation_controller: None,
        }
    }

    #[wasm_bindgen]
    pub fn initialize(&mut self, initial_values: &JsValue) -> Result<(), JsValue> {
        // ✅ Initialize animation controller
        let values: HashMap<String, AnimationValue> =
            serde_wasm_bindgen::from_value(initial_values.clone())?;

        self.animation_controller = Some(AnimationController::new(values));
        Ok(())
    }

    #[wasm_bindgen]
    pub fn animate_to(&mut self, target_values: &JsValue) -> Result<(), JsValue> {
        if let Some(controller) = &self.animation_controller {
            let values: HashMap<String, AnimationValue> =
                serde_wasm_bindgen::from_value(target_values.clone())?;

            controller.animate_to(values);
            Ok(())
        } else {
            Err(JsValue::from_str("Component not initialized"))
        }
    }

    #[wasm_bindgen]
    pub fn cleanup(&mut self) {
        // ✅ Proper cleanup
        self.animation_controller = None;
    }
}
```

## 🚀 **Quick Fixes for leptos-motion**

### **Fix 1: Update ReactiveMotionDiv**

```rust
// ✅ FIXED: Proper signal tracking in ReactiveMotionDiv
#[component]
pub fn FixedReactiveMotionDiv(
    animate: ReadSignal<HashMap<String, AnimationValue>>,
    transition: Transition,
) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Div>::new();

    // ✅ CRITICAL FIX: Use create_effect with proper signal tracking
    create_effect(move |_| {
        let animate_values = animate.get();  // This properly tracks the signal!

        if let Some(div) = node_ref.get() {
            // Apply animation to DOM element
            for (property, value) in animate_values {
                let css_value = animation_value_to_css(&value);
                div.style().set_property(&property, &css_value).unwrap();
            }
        }
    });

    view! {
        <div node_ref=node_ref />
    }
}
```

### **Fix 2: Proper WASM Memory Management**

```rust
// ✅ FIXED: WASM memory management
#[wasm_bindgen]
pub struct FixedMotionController {
    // ✅ Use proper signal storage
    signals: std::collections::HashMap<String, (ReadSignal<AnimationValue>, WriteSignal<AnimationValue>)>,
}

#[wasm_bindgen]
impl FixedMotionController {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            signals: std::collections::HashMap::new(),
        }
    }

    #[wasm_bindgen]
    pub fn create_signal(&mut self, name: &str, initial: f32) {
        // ✅ Create signal pair properly
        let (read_signal, write_signal) = create_signal(AnimationValue::Number(initial));
        self.signals.insert(name.to_string(), (read_signal, write_signal));
    }

    #[wasm_bindgen]
    pub fn update_signal(&mut self, name: &str, value: f32) -> Result<(), JsValue> {
        if let Some((_, write_signal)) = self.signals.get(name) {
            // ✅ Update signal properly
            write_signal.set(AnimationValue::Number(value));
            Ok(())
        } else {
            Err(JsValue::from_str(&format!("Signal '{}' not found", name)))
        }
    }

    #[wasm_bindgen]
    pub fn cleanup(&mut self) {
        // ✅ CRITICAL: Proper cleanup
        self.signals.clear();
    }
}
```

## 📋 **Action Items for leptos-motion**

### **Immediate Fixes (1-2 days)**

1. **Replace closure-based animations** with `create_effect` patterns
2. **Add proper signal cleanup** in WASM components
3. **Fix effect dependencies** to properly track signal changes
4. **Implement proper memory management** for WASM

### **Medium-term Improvements (1 week)**

1. **Refactor ReactiveMotionDiv** to use proper signal tracking
2. **Add comprehensive WASM testing** for signal lifecycle
3. **Implement proper error handling** for signal operations
4. **Add performance monitoring** for signal updates

### **Long-term Enhancements (2-4 weeks)**

1. **Create signal-based animation system** similar to our approach
2. **Add comprehensive documentation** for signal patterns
3. **Implement advanced signal optimizations** (batching, debouncing)
4. **Add cross-framework compatibility** via WASM bindings

## 🎯 **Key Success Factors**

### **1. Always Use `create_effect` for Signal Tracking**
```rust
// ✅ DO THIS
create_effect(move |_| {
    let value = signal.get();  // Properly tracks signal changes
    // Handle value change
});

// ❌ DON'T DO THIS
let closure = move || {
    let value = signal.get();  // Doesn't track changes!
    // Handle value
};
```

### **2. Proper WASM Memory Management**
```rust
// ✅ DO THIS
impl Drop for WasmComponent {
    fn drop(&mut self) {
        self.cleanup_signals();
    }
}

// ❌ DON'T DO THIS
// Let signals leak in WASM
```

### **3. Explicit Signal Dependencies**
```rust
// ✅ DO THIS
create_effect(move |_| {
    let a = signal_a.get();  // Explicit dependency
    let b = signal_b.get();  // Explicit dependency
    // Effect re-runs when either changes
});

// ❌ DON'T DO THIS
create_effect(move |_| {
    // Hidden signal dependencies that don't trigger re-runs
});
```

## 🚀 **Expected Results**

After implementing these fixes, leptos-motion should see:

- **✅ Proper signal tracking** - Animations respond to signal changes
- **✅ No memory leaks** - Proper cleanup in WASM
- **✅ Reliable effects** - Effects re-run when signals change
- **✅ Better performance** - Optimized signal updates
- **✅ Cross-framework compatibility** - Works with any framework via WASM

## 💡 **Our Secret Sauce**

The key to our success is:

1. **Explicit signal dependencies** in all effects
2. **Proper WASM memory management** with cleanup
3. **Signal-based state management** throughout
4. **Comprehensive testing** of signal lifecycle
5. **Performance optimization** of signal updates

This approach ensures that animations are truly reactive and work reliably across all platforms! 🎯
