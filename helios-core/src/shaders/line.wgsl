// Line chart vertex shader
struct Uniforms {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world_position = vec4<f32>(vertex.position, 0.0, 1.0);
    out.clip_position = uniforms.projection_matrix * uniforms.view_matrix * world_position;
    out.color = vec4<f32>(1.0, 0.0, 0.0, 1.0); // Red color for line
    return out;
}
