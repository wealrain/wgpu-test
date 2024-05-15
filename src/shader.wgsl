// 顶点着色器

struct VertexInput {
    @location(0) position: vec3f,
    // @location(1) color: vec3f
    @location(1) tex_coords: vec2f,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4f,
    // @location(0) color: vec3f
    @location(0) tex_coords: vec2f
};

// @vertex 
// fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
//     var out: VertexOutput;
//     let x = f32(1 - i32(in_vertex_index)) * 0.5;
//     let y = f32(i32(in_vertex_index &1u)*2 - 1) * 0.5;
//     out.clip_position = vec4f(x, y, 0.0, 1.0);
//     return out;
// }

@vertex
fn vs_main(model:VertexInput) -> VertexOutput {
    var out: VertexOutput;
    // out.color = model.color;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4f(model.position,1.0);
    return out;
}

// 片元着色器
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    // return vec4f(0.3, 0.2, 0.1, 1.0);
    // return vec4f(in.color,1.0);
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}