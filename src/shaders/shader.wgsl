// Vertex shader

struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) light: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    //out.light = dot(model.normal, vec3<f32>(0.56, 0.56, 0.56));
    out.light = model.normal;
    out.tex_coords = model.tex_coords;
    return out;
}

// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    //return textureSample(t_diffuse, s_diffuse, in.tex_coords) * in.light;
    //return  vec4<f32>(0.8, 0.8, 0.8, 1.0) * (in.light + 0.1);
    var l = dot(in.light, vec3<f32>(0.56, 0.56, 0.56));
    var c = vec3<f32>(0.8, 0.6, 0.8) * l;
    return  vec4<f32>(c, 1.0);
}
