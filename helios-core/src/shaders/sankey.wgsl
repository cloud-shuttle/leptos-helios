// Sankey diagram shader for WebGPU
// Renders flow diagrams with nodes and links

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
    node_width: f32,
    link_opacity: f32,
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
    // Create sankey flow effect
    let flow_width = uniforms.node_width;

    // Determine if this is a node or link based on position
    let is_node = (in.uv.x < 0.1) || (in.uv.x > 0.9);

    if (is_node) {
        // Render as node (rectangle)
        let node_distance = min(abs(in.uv.x - 0.05), abs(in.uv.x - 0.95));
        if (node_distance > flow_width / 2.0) {
            discard;
        }
    } else {
        // Render as link (flowing curve)
        let link_center = 0.5;
        let link_distance = abs(in.uv.y - link_center);
        if (link_distance > flow_width / 2.0) {
            discard;
        }

        // Add flow effect
        let flow_factor = 1.0 - (link_distance / (flow_width / 2.0)) * 0.3;
        return vec4<f32>(in.color.rgb * flow_factor, in.color.a * uniforms.link_opacity);
    }

    return in.color;
}
