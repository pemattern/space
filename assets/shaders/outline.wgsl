#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
#import "shaders/noise/value.wgsl"::value_3d;
#import "shaders/noise/hash.wgsl"::hash_31;

@group(0) @binding(0) var main_texture: texture_2d<f32>;
@group(0) @binding(1) var main_texture_sampler: sampler;

@group(0) @binding(2) var depth_texture: texture_depth_multisampled_2d; // 4 samples
@group(0) @binding(3) var normals_texture: texture_multisampled_2d<f32>; // 4 samples

struct OutlineSettings {
  cutoff: f32,
}
@group(0) @binding(4) var<uniform> settings: OutlineSettings;

fn roberts_cross_depth(pixel_position: vec2<i32>) -> f32 {
    let tl = textureLoad(depth_texture, pixel_position + vec2<i32>(-1, 1), 0);
    let tr = textureLoad(depth_texture, pixel_position + vec2<i32>(1, 1), 0);
    let bl = textureLoad(depth_texture, pixel_position + vec2<i32>(-1, -1), 0);
    let br = textureLoad(depth_texture, pixel_position + vec2<i32>(1, -1), 0);

    let gx = tl - br;
    let gy = tr - bl;

    let value = sqrt(pow(gx, 2.0) + pow(gy, 2.0));
    return select(0.0, 1.0, value > 0.0001);
}

fn roberts_cross_normals(pixel_position: vec2<i32>) -> f32 {
    let tl = textureLoad(normals_texture, pixel_position + vec2<i32>(-1, 1), 0).rgb;
    let tr = textureLoad(normals_texture, pixel_position + vec2<i32>(1, 1), 0).rgb;
    let bl = textureLoad(normals_texture, pixel_position + vec2<i32>(-1, -1), 0).rgb;
    let br = textureLoad(normals_texture, pixel_position + vec2<i32>(1, -1), 0).rgb;

    let gx = tl - br;
    let gy = tr - bl;

    return sqrt(dot(gx, gx) + dot(gy, gy));
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let pixel_position = vec2<i32>(in.position.xy);
    let main_color = textureSample(main_texture, main_texture_sampler, in.uv);
    let normals_color = roberts_cross_normals(pixel_position);
    let depth_color = roberts_cross_depth(pixel_position);
    let edge_color = max(0.0, depth_color);
    return main_color + vec4<f32>(edge_color);
}
