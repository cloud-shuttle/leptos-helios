// Box plot shader for WebGPU
// Renders statistical box plots with whiskers and outliers

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec2<f32>,
    @location(2) statistical_value: f32, // 0=min, 1=q1, 2=median, 3=q3, 4=max, 5=outlier
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) stat_type: f32,
}

struct Uniforms {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
    box_width: f32,
    whisker_length: f32,
    outlier_size: f32,
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
    
    // Color based on statistical element type
    var color = vertex.color;
    if (vertex.statistical_value >= 5.0) {
        // Outlier - different color
        color = vec2<f32>(1.0, 0.0); // Red for outliers
    }
    
    out.color = vec4<f32>(color, uniforms.alpha, 1.0);
    out.stat_type = vertex.statistical_value;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Different rendering for different statistical elements
    if (in.stat_type >= 5.0) {
        // Render outlier as circle
        let center = vec2<f32>(0.5, 0.5);
        let dist = distance(in.color.xy, center);
        
        if (dist > 0.5) {
            discard;
        }
        
        let alpha = 1.0 - smoothstep(0.3, 0.5, dist);
        return vec4<f32>(in.color.rgb, in.color.a * alpha);
    } else {
        // Render box/whisker elements
        return in.color;
    }
}
