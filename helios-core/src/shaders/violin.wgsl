//! Violin Plot Shader
//! 
//! WebGPU shader for rendering violin plots with kernel density estimation

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) density: f32,
    @location(3) category_index: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) density: f32,
    @location(2) category_index: f32,
}

struct ViolinUniforms {
    bandwidth: f32,
    fill_opacity: f32,
    stroke_width: f32,
    show_box_plot: f32,
    show_points: f32,
    point_size: f32,
    viewport_size: vec2<f32>,
    num_categories: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: ViolinUniforms;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Scale position to viewport
    let x = (model.position.x / uniforms.viewport_size.x) * 2.0 - 1.0;
    let y = (model.position.y / uniforms.viewport_size.y) * 2.0 - 1.0;
    
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.color = model.color;
    out.density = model.density;
    out.category_index = model.category_index;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Apply density-based opacity
    var color = in.color;
    color.a *= uniforms.fill_opacity;
    
    // Apply density variation for violin shape
    let density_factor = 0.5 + in.density * 0.5;
    color.a *= density_factor;
    
    return color;
}

// Box plot rendering shader
@vertex
fn vs_box_plot(vertex: vec2<f32>) -> @builtin(position) vec4<f32> {
    let x = (vertex.x / uniforms.viewport_size.x) * 2.0 - 1.0;
    let y = (vertex.y / uniforms.viewport_size.y) * 2.0 - 1.0;
    return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_box_plot() -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 1.0); // Black box plot lines
}

// Point rendering shader
@vertex
fn vs_points(vertex: vec2<f32>) -> @builtin(position) vec4<f32> {
    let x = (vertex.x / uniforms.viewport_size.x) * 2.0 - 1.0;
    let y = (vertex.y / uniforms.viewport_size.y) * 2.0 - 1.0;
    return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_points() -> @location(0) vec4<f32> {
    return vec4<f32>(0.2, 0.2, 0.2, 0.8); // Dark gray points
}