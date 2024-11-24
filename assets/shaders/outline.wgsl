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

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let pixel_position = vec2<i32>(in.position.xy);
    let normals_color = textureLoad(normals_texture, pixel_position, 0);
    let depth_color = textureLoad(depth_texture, pixel_position, 0);
    return normals_color * (1.0 - depth_color);
}
