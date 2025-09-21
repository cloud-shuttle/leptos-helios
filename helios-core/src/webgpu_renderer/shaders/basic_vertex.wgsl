// Basic vertex shader for WebGPU rendering

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

struct Uniforms {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
    model_matrix: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Transform vertex position
    let world_position = uniforms.model_matrix * vec4<f32>(model.position, 1.0);
    let view_position = uniforms.view_matrix * world_position;
    out.clip_position = uniforms.projection_matrix * view_position;
    
    // Pass through color
    out.color = model.color;
    
    return out;
}
