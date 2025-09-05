// Histogram shader for WebGPU
// Renders bar charts for distribution visualization

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
    bin_width: f32,
    density_mode: f32, // 0=count, 1=density
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
    // Create histogram bar effect
    let bar_center = 0.5;
    let bar_width = uniforms.bin_width;
    let distance_from_center = abs(in.uv.x - bar_center);

    if (distance_from_center > bar_width / 2.0) {
        discard;
    }

    // Add subtle gradient for depth
    let gradient = 1.0 - (distance_from_center / (bar_width / 2.0)) * 0.2;

    // Add border effect
    let border_distance = min(min(in.uv.x, 1.0 - in.uv.x), min(in.uv.y, 1.0 - in.uv.y));
    let border_factor = smoothstep(0.0, 0.02, border_distance);

    return vec4<f32>(in.color.rgb * gradient * border_factor, in.color.a);
}
