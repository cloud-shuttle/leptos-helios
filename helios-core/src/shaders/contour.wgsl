// Contour plot shader for WebGPU
// Renders 2D contour lines for density visualization

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
}

struct Uniforms {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
    contour_levels: f32,
    stroke_width: f32,
    alpha: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Transform position
    let world_pos = vec4<f32>(vertex.position, 0.0, 1.0);
    out.clip_position = uniforms.projection_matrix * uniforms.view_matrix * world_pos;

    // Pass through color and UV coordinates
    out.color = vec4<f32>(vertex.color, uniforms.alpha, 1.0);
    out.uv = vertex.position;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Create contour line effect
    let line_width = uniforms.stroke_width;

    // Simple contour pattern (would be more complex in real implementation)
    let contour_pattern = sin(in.uv.x * uniforms.contour_levels * 10.0) * sin(in.uv.y * uniforms.contour_levels * 10.0);
    let contour_factor = smoothstep(-line_width, line_width, contour_pattern);

    if (contour_factor < 0.1) {
        discard;
    }

    return vec4<f32>(in.color.rgb, in.color.a * contour_factor);
}
