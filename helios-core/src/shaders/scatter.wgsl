// Enhanced scatter plot shader for WebGPU
// Supports jitter, trend lines, and advanced point rendering

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec2<f32>, // RG as vec2, BA will be uniforms
    @location(2) size: f32,        // Point size
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) point_size: f32,
    @location(2) world_pos: vec2<f32>,
}

struct Uniforms {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
    base_size: f32,
    jitter_amount: f32,
    alpha: f32,
    trend_line_enabled: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Apply jitter if enabled
    var jittered_pos = vertex.position;
    if (uniforms.jitter_amount > 0.0) {
        let jitter = (fract(sin(dot(vertex.position, vec2<f32>(12.9898, 78.233))) * 43758.5453) - 0.5) * uniforms.jitter_amount;
        jittered_pos += vec2<f32>(jitter, jitter);
    }

    // Transform position
    let world_pos = vec4<f32>(jittered_pos, 0.0, 1.0);
    out.clip_position = uniforms.projection_matrix * uniforms.view_matrix * world_pos;
    out.world_pos = jittered_pos;

    // Pass through color with alpha from uniforms
    out.color = vec4<f32>(vertex.color, uniforms.alpha, 1.0);
    out.point_size = uniforms.base_size * vertex.size;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Enhanced point rendering with smooth edges
    let center = vec2<f32>(0.5, 0.5);
    let dist = distance(in.color.xy, center);

    if (dist > 0.5) {
        discard;
    }

    // Smooth edge falloff with enhanced anti-aliasing
    let alpha = 1.0 - smoothstep(0.2, 0.5, dist);

    // Add subtle glow effect
    let glow = 1.0 - smoothstep(0.0, 0.3, dist);

    return vec4<f32>(in.color.rgb + glow * 0.1, in.color.a * alpha);
}
