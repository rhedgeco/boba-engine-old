@group(0) @binding(0)
var<uniform> camera_matrix: mat4x4<f32>;
@group(1) @binding(0)
var<uniform> model_matrix: mat4x4<f32>;

@group(2)@binding(0)
var s_diffuse: sampler;
@group(2) @binding(1)
var t_diffuse: texture_2d<f32>;
@group(2) @binding(2)
var<uniform> color: vec4<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) world_position: vec3<f32>,
    @location(3) world_normal: vec3<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    let world_position = model_matrix * vec4<f32>(model.position, 1.0);

    out.normal = model.normal;
    out.tex_coords = model.tex_coords;
    out.world_position = world_position.xyz;
    out.clip_position = camera_matrix * world_position;
    out.world_normal = (model_matrix * vec4<f32>(model.normal, 1.0)).xyz;
    return out;
}

@fragment
fn position_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.world_position, 1.0);
}

@fragment
fn normal_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.world_normal, 1.0);
}

@fragment
fn albedo_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords) * color;
}

@fragment
fn specular_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}
