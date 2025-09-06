//! Sankey Diagram Shader
//! 
//! WebGPU shader for rendering Sankey diagrams with flow visualization

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) node_id: f32,
    @location(3) flow_value: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) node_id: f32,
    @location(2) flow_value: f32,
}

struct SankeyUniforms {
    node_width: f32,
    node_padding: f32,
    link_opacity: f32,
    viewport_size: vec2<f32>,
    num_nodes: f32,
    num_links: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: SankeyUniforms;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Scale position to viewport
    let x = (model.position.x / uniforms.viewport_size.x) * 2.0 - 1.0;
    let y = (model.position.y / uniforms.viewport_size.y) * 2.0 - 1.0;
    
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.color = model.color;
    out.node_id = model.node_id;
    out.flow_value = model.flow_value;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Apply link opacity for flow visualization
    var color = in.color;
    
    // Different opacity for nodes vs links
    if (in.node_id >= 0.0) {
        // Node rendering - full opacity
        color.a = 1.0;
    } else {
        // Link rendering - reduced opacity
        color.a *= uniforms.link_opacity;
    }
    
    return color;
}

// Link curve rendering shader
@vertex
fn vs_link_curve(vertex: vec2<f32>) -> @builtin(position) vec4<f32> {
    let x = (vertex.x / uniforms.viewport_size.x) * 2.0 - 1.0;
    let y = (vertex.y / uniforms.viewport_size.y) * 2.0 - 1.0;
    return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_link_curve() -> @location(0) vec4<f32> {
    return vec4<f32>(0.5, 0.5, 0.5, uniforms.link_opacity);
}