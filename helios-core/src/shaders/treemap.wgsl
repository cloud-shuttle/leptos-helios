//! Treemap Shader
//! 
//! WebGPU shader for rendering treemap visualizations with hierarchical rectangles

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) depth: f32,
    @location(3) value: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) depth: f32,
    @location(2) value: f32,
}

struct TreemapUniforms {
    padding: f32,
    border_width: f32,
    border_color: vec4<f32>,
    viewport_size: vec2<f32>,
    max_depth: f32,
    color_scheme: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: TreemapUniforms;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Scale position to viewport
    let x = (model.position.x / uniforms.viewport_size.x) * 2.0 - 1.0;
    let y = (model.position.y / uniforms.viewport_size.y) * 2.0 - 1.0;
    
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.color = model.color;
    out.depth = model.depth;
    out.value = model.value;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Apply depth-based color variation
    var color = in.color;
    
    // Darken color based on depth
    let depth_factor = 1.0 - (in.depth / uniforms.max_depth) * 0.3;
    color.rgb *= depth_factor;
    
    // Apply value-based brightness
    let value_factor = 0.7 + (in.value / 100.0) * 0.3;
    color.rgb *= value_factor;
    
    return color;
}

// Border rendering shader
@vertex
fn vs_border(vertex: vec2<f32>) -> @builtin(position) vec4<f32> {
    let x = (vertex.x / uniforms.viewport_size.x) * 2.0 - 1.0;
    let y = (vertex.y / uniforms.viewport_size.y) * 2.0 - 1.0;
    return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_border() -> @location(0) vec4<f32> {
    return uniforms.border_color;
}