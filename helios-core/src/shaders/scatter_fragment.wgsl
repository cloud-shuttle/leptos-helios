// Scatter plot fragment shader
struct FragmentInput {
    @location(0) color: vec4<f32>,
}

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
    return in.color;
}
