// Heatmap shader for WebGPU
// Renders 2D heatmaps with color mapping and interpolation

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec2<f32>,
    @location(2) value: f32, // Heatmap value for color mapping
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) heat_value: f32,
    @location(2) uv: vec2<f32>,
}

struct Uniforms {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
    color_scheme: f32, // 0=viridis, 1=plasma, 2=inferno, 3=magma, 4=jet
    min_value: f32,
    max_value: f32,
    alpha: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

// Color scheme functions
fn viridis_color(t: f32) -> vec3<f32> {
    let c0 = vec3<f32>(0.267004, 0.004874, 0.329415);
    let c1 = vec3<f32>(0.127568, 0.566949, 0.550556);
    let c2 = vec3<f32>(0.993248, 0.906157, 0.143936);
    
    let t2 = t * t;
    let t3 = t2 * t;
    
    return c0 * (1.0 - t) * (1.0 - t) + c1 * 2.0 * (1.0 - t) * t + c2 * t2;
}

fn plasma_color(t: f32) -> vec3<f32> {
    let c0 = vec3<f32>(0.050383, 0.029803, 0.527975);
    let c1 = vec3<f32>(0.132500, 0.677316, 0.491653);
    let c2 = vec3<f32>(0.975158, 0.975158, 0.975158);
    
    let t2 = t * t;
    let t3 = t2 * t;
    
    return c0 * (1.0 - t) * (1.0 - t) + c1 * 2.0 * (1.0 - t) * t + c2 * t2;
}

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Transform position
    let world_pos = vec4<f32>(vertex.position, 0.0, 1.0);
    out.clip_position = uniforms.projection_matrix * uniforms.view_matrix * world_pos;
    
    // Pass through heat value and UV coordinates
    out.heat_value = vertex.value;
    out.uv = vertex.position;
    
    // Base color (will be modified in fragment shader)
    out.color = vec4<f32>(vertex.color, uniforms.alpha, 1.0);
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Normalize heat value to [0, 1] range
    let normalized_value = (in.heat_value - uniforms.min_value) / (uniforms.max_value - uniforms.min_value);
    let clamped_value = clamp(normalized_value, 0.0, 1.0);
    
    // Apply color scheme
    var heat_color: vec3<f32>;
    if (uniforms.color_scheme < 0.5) {
        heat_color = viridis_color(clamped_value);
    } else {
        heat_color = plasma_color(clamped_value);
    }
    
    // Add subtle border effect
    let border_distance = min(min(in.uv.x, 1.0 - in.uv.x), min(in.uv.y, 1.0 - in.uv.y));
    let border_factor = smoothstep(0.0, 0.05, border_distance);
    
    return vec4<f32>(heat_color * border_factor, uniforms.alpha);
}
