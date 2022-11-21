@group(0) @binding(0)
var<uniform> view_matrix: mat4x4<f32>;
@group(1) @binding(0)
var<uniform> model_matrix: mat4x4<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.tex_coords = model.tex_coords;
    out.clip_position = view_matrix * model_matrix * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(1.0, 0.0, 0.0, 1.0);
}
