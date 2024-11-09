#import "shaders/noise/hash.wgsl"::hash_31;
#import "shaders/noise/value.wgsl"::value_3d;
#import "shaders/noise/voronoi.wgsl"::voronoi_3d;

@group(2) @binding(0) var<uniform> camera_position: vec3<f32>;

@fragment
fn fragment(@location(0) world_position: vec3<f32>) -> @location(0) vec4<f32> {
    let scaled_position = world_position / 255.0;
    var color = mix(vec3<f32>(0.04, 0.05, 0.05), vec3<f32>(0.04, 0.03, 0.07), value_3d(scaled_position));
    let dither_strength = 0.002;
    color = color + (dither_strength * hash_31(scaled_position)) - (dither_strength * 0.5);

    let relative_position = world_position - camera_position;
    let star_noise = voronoi_3d(relative_position / 10.0);

    let star_color = select(
        vec3<f32>(0.0),
        vec3<f32>(0.9, 0.9, 0.7),
        star_noise < 0.003
    );

    color = color + star_color;

    return vec4<f32>(color, 1.0);
}
