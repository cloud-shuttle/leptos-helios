//! Radar Chart Shader
//! 
//! WebGPU shader for rendering radar charts with polar coordinates

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) category_index: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) category_index: f32,
}

struct RadarUniforms {
    center: vec2<f32>,
    radius: f32,
    max_value: f32,
    stroke_width: f32,
    fill_opacity: f32,
    show_grid: f32,
    num_categories: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: RadarUniforms;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Convert polar coordinates to cartesian
    let angle = 2.0 * 3.14159265359 * model.category_index / uniforms.num_categories;
    let normalized_value = model.position.x / uniforms.max_value;
    
    let x = uniforms.center.x + uniforms.radius * normalized_value * cos(angle);
    let y = uniforms.center.y + uniforms.radius * normalized_value * sin(angle);
    
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.color = model.color;
    out.category_index = model.category_index;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Apply fill opacity for area rendering
    var color = in.color;
    color.a *= uniforms.fill_opacity;
    
    return color;
}

// Grid rendering shader
@vertex
fn vs_grid(vertex: vec2<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(vertex, 0.0, 1.0);
}

@fragment
fn fs_grid() -> @location(0) vec4<f32> {
    return vec4<f32>(0.8, 0.8, 0.8, 0.3); // Light gray grid lines
}