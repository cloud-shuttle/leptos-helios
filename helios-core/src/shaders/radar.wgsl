// Radar chart shader for WebGPU
// Renders multivariate data in polar coordinates

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
    radius: f32,
    fill_opacity: f32,
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
    // Convert to polar coordinates
    let center = vec2<f32>(0.5, 0.5);
    let offset = in.uv - center;
    let distance = length(offset);
    let angle = atan2(offset.y, offset.x);

    // Create radar chart pattern
    let max_radius = uniforms.radius;
    let normalized_distance = distance / max_radius;

    if (normalized_distance > 1.0) {
        discard;
    }

    // Add grid lines
    let grid_factor = 1.0 - smoothstep(0.0, 0.02, abs(fract(normalized_distance * 5.0) - 0.5));

    // Apply fill opacity
    let fill_alpha = uniforms.fill_opacity * (1.0 - normalized_distance * 0.3);

    return vec4<f32>(in.color.rgb, in.color.a * fill_alpha * (1.0 + grid_factor * 0.5));
}
