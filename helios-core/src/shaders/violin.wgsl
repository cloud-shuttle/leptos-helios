// Violin plot shader for WebGPU
// Renders smooth distribution curves with kernel density estimation

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec2<f32>,
    @location(2) density_value: f32, // Kernel density value
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) density: f32,
    @location(2) uv: vec2<f32>,
}

struct Uniforms {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
    bandwidth: f32,
    kernel_type: f32, // 0=Gaussian, 1=Epanechnikov, 2=Uniform, 3=Triangular, 4=Cosine
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
    
    // Pass through color and density
    out.color = vec4<f32>(vertex.color, uniforms.alpha, 1.0);
    out.density = vertex.density_value;
    out.uv = vertex.position;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Apply kernel density visualization
    let density_factor = in.density;
    
    // Create smooth violin shape based on density
    let center_x = 0.5;
    let distance_from_center = abs(in.uv.x - center_x);
    let max_width = density_factor * 0.4; // Scale density to width
    
    if (distance_from_center > max_width) {
        discard;
    }
    
    // Smooth falloff at edges
    let alpha = 1.0 - smoothstep(max_width * 0.7, max_width, distance_from_center);
    
    // Add subtle gradient for depth
    let gradient = 1.0 - (distance_from_center / max_width) * 0.3;
    
    return vec4<f32>(in.color.rgb * gradient, in.color.a * alpha);
}
