// Composite chart shader for WebGPU
// Renders multiple chart types in a single pass

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec2<f32>,
    @location(2) chart_type: f32, // 0=point, 1=line, 2=bar, etc.
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) chart_type: f32,
}

struct Uniforms {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
    alpha: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let world_pos = vec4<f32>(vertex.position, 0.0, 1.0);
    out.clip_position = uniforms.projection_matrix * uniforms.view_matrix * world_pos;
    out.color = vec4<f32>(vertex.color, uniforms.alpha, 1.0);
    out.chart_type = vertex.chart_type;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Apply different rendering logic based on chart type
    var color = in.color;

    if (in.chart_type < 0.5) {
        // Point rendering
        color.rgb *= 1.2; // Slightly brighter for points
    } else if (in.chart_type < 1.5) {
        // Line rendering
        color.rgb *= 0.9; // Slightly dimmer for lines
    }

    return color;
}
