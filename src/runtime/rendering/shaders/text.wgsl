@group(0) @binding(0) var t_diffuse: texture_2d<f32>;
@group(0) @binding(1) var s_diffuse: sampler;
@group(0) @binding(2) var<uniform> camera: mat4x4<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};


// v for vertex i for instance
@vertex
fn vs_main(
    @location(0) v_pos: vec2<f32>,
    @location(1) v_uv: vec2<f32>,

    @location(2) i_pos: vec2<f32>,
    @location(3) i_size: vec2<f32>,
    @location(4) i_uv_offset: vec2<f32>,
    @location(5) i_uv_scale: vec2<f32>,
) -> VertexOutput {
    var out: VertexOutput;

    let scaled = v_pos * i_size;
    let world_pos = i_pos + scaled;

    out.clip_position = camera * vec4<f32>(world_pos, 0.0, 1.0);

    // Flip the texture horizontally (across the Y axis).
    out.tex_coords = vec2<f32>(
        i_uv_offset.x + (v_uv.x) * i_uv_scale.x,
        i_uv_offset.y + (1.0 - v_uv.y) * i_uv_scale.y
    );

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
