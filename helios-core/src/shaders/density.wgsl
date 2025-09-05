// Density plot shader for WebGPU
// Renders smooth density curves with kernel estimation

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
    bandwidth: f32,
    fill_enabled: f32, // 0=line only, 1=filled
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
    if (uniforms.fill_enabled > 0.5) {
        // Filled density plot
        return in.color;
    } else {
        // Line-only density plot
        let line_width = 0.02;
        let distance_from_line = abs(in.uv.y - 0.5);

        if (distance_from_line > line_width) {
            discard;
        }

        let alpha = 1.0 - smoothstep(0.0, line_width, distance_from_line);
        return vec4<f32>(in.color.rgb, in.color.a * alpha);
    }
}
