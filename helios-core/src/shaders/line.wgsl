//! Line Chart Shader
//!
//! Real WGSL shader for rendering line charts with WebGPU

// Vertex shader
@vertex
fn vs_main(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
    // Convert from normalized coordinates (-1 to 1) to clip space
    return vec4<f32>(position, 0.0, 1.0);
}

// Fragment shader
@fragment
fn fs_main() -> @location(0) vec4<f32> {
    // Return a solid color for the line
    return vec4<f32>(0.0, 0.5, 1.0, 1.0); // Blue color
}

// Alternative fragment shader with configurable color
@fragment
fn fs_main_colored(@location(0) color: vec4<f32>) -> @location(0) vec4<f32> {
    return color;
}
