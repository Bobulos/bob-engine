@group(0) @binding(0) var t_diffuse: texture_2d<f32>;
@group(0) @binding(1) var s_diffuse: sampler;
@group(0) @binding(2) var<uniform> camera: mat4x4<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    @location(0) v_pos: vec2<f32>,
    @location(1) v_uv: vec2<f32>,

    @location(2) i_pos: vec2<f32>,
    @location(3) i_size: vec2<f32>,
    @location(4) i_uv_offset: vec2<f32>,
    @location(5) i_uv_scale: vec2<f32>,
    @location(6) i_rotation: f32, // rotation in radians
) -> VertexOutput {
    var out: VertexOutput;

    // Scale vertex
    let scaled = v_pos * i_size;

    // Rotation matrix
    let c = cos(i_rotation);
    let s = sin(i_rotation);

    let rotated = vec2<f32>(
        scaled.x * c - scaled.y * s,
        scaled.x * s + scaled.y * c
    );

    // Translate to world position
    let world_pos = rotated + i_pos;

    // Camera transform
    out.clip_position = camera * vec4<f32>(world_pos, 0.0, 1.0);

    // Texture coordinates
    out.tex_coords = (v_uv * i_uv_scale) + i_uv_offset;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
