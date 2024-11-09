#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
#import "shaders/noise/value.wgsl"::value_3d;
#import "shaders/noise/hash.wgsl"::hash_31;

@group(0) @binding(0) var main_texture: texture_2d<f32>;
@group(0) @binding(1) var main_texture_sampler: sampler;

@group(0) @binding(2) var depth_texture: texture_depth_multisampled_2d;

struct VolumetricNebulaSettings {
  time: f32,
  camera_position: vec3<f32>,
  camera_right: vec3<f32>,
  camera_up: vec3<f32>,
  camera_forward: vec3<f32>,
  light_direction: vec3<f32>,
  speed: f32,
  scale: f32,
  iso_value: f32,
  step_count: i32,
  step_distance: f32,
}
@group(0) @binding(3) var<uniform> settings: VolumetricNebulaSettings;

fn noise(p: vec3<f32>, uv: vec2<f32>) -> vec4<f32> {
    let time = settings.time;
    let offset = vec3<f32>(0.0);
    let point = p + offset;
    let n1 = value_3d(point * settings.scale * 0.2 * vec3<f32>(0.75, 1.25, 0.75)) * vec4<f32>(0.8, 0.2, 1.0, 1.0);
    let n2 = value_3d(point * settings.scale * 0.7 * vec3<f32>(1.25, 0.75, 1.25)) * vec4<f32>(0.1, 1.0, 0.2, 1.0);
    let n4 = hash_31(point * settings.scale) * vec4<f32>(1.0, 1.0, 0.8, 1.0);
    let noise_value = (n1 * 28 + n2 * 18 + n4 * 4) / 50.0;
    return noise_value;
}

fn intensity_at_depth(x: f32) -> f32 {
    return -pow(2.0 * x - 1.0, 2.0) + 1.0;
}

fn ray_march(origin: vec3<f32>, ray_direction: vec3<f32>, ray_depth: f32, uv: vec2<f32>) -> vec4<f32> {
    var depth = 0.0;
    var full_depth = f32(settings.step_count) * settings.step_distance;
    var p = origin;
    var result = vec4<f32>(0.0);
    for (var i: i32 = 0; i < settings.step_count; i = i + 1) {
        var density = noise(p, uv);
        if length(density) - settings.iso_value > 0.0 {
            result = result + density * intensity_at_depth(depth / full_depth) * (1.0 / f32(settings.step_count));
        }
        depth = depth + settings.step_distance;
        if depth <= ray_depth * full_depth {
          break;
        }
        p = origin + depth * ray_direction;
    }
    return result;
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let ndc = in.uv * 2.0 - 1.0;
    let ray_origin = settings.camera_position * settings.speed;
    let ray_direction = normalize(
        settings.camera_forward + settings.camera_right * ndc.x + settings.camera_up * -ndc.y
    );
    let ray_depth = textureLoad(depth_texture, vec2<i32>(in.position.xy), 0);
    let ray_color = ray_march(ray_origin, ray_direction, ray_depth, in.uv) * 0.15;
    let scene_color = textureSample(main_texture, main_texture_sampler, in.uv);
    return ray_color + scene_color;
}
