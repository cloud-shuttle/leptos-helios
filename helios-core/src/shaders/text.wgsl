// Text rendering shader for WebGPU
// Renders text using signed distance fields

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

struct Uniforms {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
    text_color: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(0) @binding(1)
var text_texture: texture_2d<f32>;
@group(0) @binding(2)
var text_sampler: sampler;

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    let world_pos = vec4<f32>(vertex.position, 0.0, 1.0);
    out.clip_position = uniforms.projection_matrix * uniforms.view_matrix * world_pos;
    out.uv = vertex.uv;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let distance = textureSample(text_texture, text_sampler, in.uv).r;
    let alpha = smoothstep(0.5 - 0.1, 0.5 + 0.1, distance);
    
    return vec4<f32>(uniforms.text_color.rgb, uniforms.text_color.a * alpha);
}
