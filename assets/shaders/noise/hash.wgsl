const SINE_FACTOR: f32 = 43758.5453123;

fn hash_21(p: vec2<f32>) -> f32 {
    let d = dot(p, vec2<f32>(12.9898, 78.233));
    let s = sin(d);
    let f = fract(s * SINE_FACTOR);
    return f;
}

fn hash_31(p: vec3<f32>) -> f32 {
    let d = dot(p, vec3<f32>(127.1, 311.7, 74.7));
    let s = sin(d);
    let f = fract(s * SINE_FACTOR);
    return f;
}

fn hash_33(p: vec3<f32>) -> vec3<f32> {
    let d = vec3<f32>(
        dot(p, vec3<f32>(127.1, 311.7, 74.7)),
        dot(p, vec3<f32>(269.5, 183.3, 246.1)),
        dot(p, vec3<f32>(113.5, 271.9, 124.6))
    );
    let s = sin(d);
    let f = fract(s * SINE_FACTOR);
    return f;
}
