// Bar chart vertex shader
struct Uniforms {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec2<f32>,
}

struct InstanceInput {
    @location(1) instance_position: vec2<f32>,
    @location(2) instance_size: vec2<f32>,
    @location(3) instance_color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;
    let scaled_position = vertex.position * instance.instance_size;
    let world_position = vec4<f32>(scaled_position + instance.instance_position, 0.0, 1.0);
    out.clip_position = uniforms.projection_matrix * uniforms.view_matrix * world_position;
    out.color = instance.instance_color;
    return out;
}
