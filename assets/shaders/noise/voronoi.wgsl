#import "shaders/noise/hash.wgsl"::hash_33;

fn voronoi_3d(p: vec3<f32>) -> f32 {
    let p_floor = floor(p);
    let p_fract = fract(p);

    var res = 100.0;
    for (var x: f32 = -1.0; x <= 1.0; x = x + 1.0) {
        for (var y: f32 = -1.0; y <= 1.0; y = y + 1.0) {
            for (var z: f32 = -1.0; z <= 1.0; z = z + 1.0) {
                let b = vec3<f32>(x, y, z);
                let r = vec3<f32>(b) - p_fract + hash_33(p_floor + b);
                let d = dot(r, r);

                if d < res {
                    res = d;
                }
            }
        }
    }
    return res;
}
