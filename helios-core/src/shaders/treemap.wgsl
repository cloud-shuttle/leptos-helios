// Treemap shader for WebGPU
// Renders hierarchical data as nested rectangles

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
    padding: f32,
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
    // Create treemap rectangle effect
    let padding = uniforms.padding;

    // Check if we're in the padding area
    let border_distance = min(min(in.uv.x, 1.0 - in.uv.x), min(in.uv.y, 1.0 - in.uv.y));

    if (border_distance < padding) {
        // Render as border
        let border_factor = smoothstep(0.0, uniforms.stroke_width, border_distance);
        return vec4<f32>(in.color.rgb * 0.8, in.color.a * border_factor);
    }

    // Add subtle gradient for depth
    let gradient = 1.0 - (border_distance - padding) * 0.2;

    return vec4<f32>(in.color.rgb * gradient, in.color.a);
}
