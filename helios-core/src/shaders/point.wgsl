// Point chart shader for WebGPU
// Renders individual points with customizable size and color

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec2<f32>, // RG as vec2, BA will be uniforms
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) point_size: f32,
}

struct Uniforms {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
    point_size: f32,
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
    
    // Pass through color with alpha from uniforms
    out.color = vec4<f32>(vertex.color, uniforms.alpha, 1.0);
    out.point_size = uniforms.point_size;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simple circular point rendering
    let center = vec2<f32>(0.5, 0.5);
    let dist = distance(in.color.xy, center);
    
    if (dist > 0.5) {
        discard;
    }
    
    // Smooth edge falloff
    let alpha = 1.0 - smoothstep(0.3, 0.5, dist);
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
