#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct ChromaticAbberationSettings {
  intensity: f32,
  distance_exponent: f32,
}
@group(0) @binding(2) var<uniform> settings: ChromaticAbberationSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let offset = settings.intensity;
    let exponent = settings.distance_exponent;
    let distance_from_center = length(in.uv - vec2<f32>(0.5, 0.5));
    let r_offset = in.uv + vec2<f32>(offset, -offset) * pow(distance_from_center, exponent);
    let g_offset = in.uv + vec2<f32>(-offset, 0.0) * pow(distance_from_center, exponent);
    let b_offset = in.uv + vec2<f32>(-offset, offset) * pow(distance_from_center, exponent);

    let r = textureSample(screen_texture, texture_sampler, r_offset).r;
    let g = textureSample(screen_texture, texture_sampler, g_offset).g;
    let b = textureSample(screen_texture, texture_sampler, b_offset).b;
    return vec4<f32>(r, g, b, 1.0);
}
