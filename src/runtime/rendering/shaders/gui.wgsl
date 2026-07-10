@group(0) @binding(0) var t_diffuse: texture_2d<f32>;
@group(0) @binding(1) var s_diffuse: sampler;
@group(0) @binding(2) var<uniform> camera: mat4x4<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords:    vec2<f32>,
    @location(1) color:         vec4<f32>,
    @location(2) local_pos:     vec2<f32>,
    @location(3) half_size:     vec2<f32>,
    @location(4) corner_radius: f32,
    @location(5) border_width:  f32,
    @location(6) border_color:  vec4<f32>,
};

@vertex
fn vs_main(
    @location(0) v_pos: vec2<f32>,
    @location(1) v_uv:  vec2<f32>,
    @location(2) i_pos:           vec2<f32>,
    @location(3) i_size:          vec2<f32>,
    @location(4) i_uv_offset:     vec2<f32>,
    @location(5) i_uv_scale:      vec2<f32>,
    @location(6) i_corner_radius: f32,
    @location(7) i_color:         vec4<f32>,
    @location(8) i_border_width:  f32,
    @location(9) i_border_color:  vec4<f32>,
) -> VertexOutput {
    var out: VertexOutput;

    let local     = v_pos * i_size;
    let center    = i_pos + i_size * 0.5;
    let world_pos = center + local;

    out.clip_position = camera * vec4<f32>(world_pos, 0.0, 1.0);
    out.tex_coords    = i_uv_offset + v_uv * i_uv_scale;
    out.color         = i_color;
    out.local_pos     = local;
    out.half_size     = i_size * 0.5;
    out.corner_radius = i_corner_radius;
    out.border_width  = i_border_width;
    out.border_color  = i_border_color;
    return out;
}

fn sd_rounded_box(p: vec2<f32>, half: vec2<f32>, radius: f32) -> f32 {
    let r = min(radius, min(half.x, half.y));
    let q = abs(p) - half + vec2<f32>(r);
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - r;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let dist        = sd_rounded_box(in.local_pos, in.half_size, in.corner_radius);
    let aa          = fwidth(dist);
    let shape_alpha = 1.0 - smoothstep(-aa, aa, dist);

    if shape_alpha <= 0.0 {
        discard;
    }

    let tex_color = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    var out_color = tex_color * in.color;

    if in.border_width > 0.0 {
        let inner_dist  = dist + in.border_width;
        let border_mask = smoothstep(-aa, aa, inner_dist) * shape_alpha;
        out_color       = mix(out_color, in.border_color, border_mask);
    }

    out_color.a *= shape_alpha;
    return out_color;
}