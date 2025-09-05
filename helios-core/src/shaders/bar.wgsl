// Bar chart shader for WebGPU
// Renders rectangular bars with customizable colors and gradients

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec2<f32>, // RG as vec2, BA will be uniforms
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
}

struct Uniforms {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
    alpha: f32,
    gradient_enabled: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Transform position
    let world_pos = vec4<f32>(vertex.position, 0.0, 1.0);
    out.clip_position = uniforms.projection_matrix * uniforms.view_matrix * world_pos;

    // Pass through color with alpha from uniforms
    out.color = vec4<f32>(vertex.color, uniforms.alpha, 1.0);

    // UV coordinates for gradient effect
    out.uv = vertex.position;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var color = in.color;

    // Apply gradient if enabled
    if (uniforms.gradient_enabled > 0.5) {
        let gradient_factor = in.uv.y * 0.3 + 0.7; // Subtle vertical gradient
        color.rgb *= gradient_factor;
    }

    return color;
}
