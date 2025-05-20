// ========= Utils =========

struct TaaConfig {
    canvas_width: u32,
    canvas_height: u32,
    jitter: vec2<f32>,
};

struct ColorData {
    color: vec3<f32>,
    depth: f32,
    emission: vec3<f32>,
    velocity: vec2<f32>,
}

struct DebugInfo {
    debug_mode: u32,
    mip_map: u32,
}

struct Transform {
    transform_matrix: mat4x4<f32>,
    inverse_matrix: mat4x4<f32>,
}

struct RtTransform {
    current_transform: Transform,
    previous_transform: Transform,
}

struct Camera {
    image_width: u32,
    image_height: u32,
    center: vec3<f32>,
    first_pixel: vec3<f32>,
    pixel_delta_u: vec3<f32>,
    pixel_delta_v: vec3<f32>,
    scan_depth: u32,
}

@group(0) @binding(2)
var<uniform> taa_config: TaaConfig;

fn lerp(a: vec3<f32>, b: vec3<f32>, t: f32) -> vec3<f32> {
    return a + (b - a) * t;
}

fn adjust_hdr_color(color: vec3<f32>) -> vec4<f32> {
    let luminance = dot(color, vec3<f32>(0.299, 0.587, 0.114));
    let luminance_weight = 1.0 / (1.0 + luminance);
    return vec4<f32>(color, 1.0) * luminance_weight;
}

fn random_vec_in_unit_sphere(co: vec2<f32>, jitter: f32) -> vec3<f32> {
    var vector = vec3<f32>(0.0);
    for (var j = 0.0; j < 1.0; j += 0.1) {
        vector = random_vec_range(co, -1.0, 1.0, jitter + j);
        if vec_len_squared(vector) < 1.0 {
            break;
        }
    }

    return vector;
}

fn random_vec_range(co: vec2<f32>, min: f32, max: f32, jitter: f32) -> vec3<f32> {
    return vec3<f32>(
        rand_range(co - 1.0, min, max, jitter),
        rand_range(co + 2.0, min, max, jitter),
        rand_range(co + 5.0, min, max, jitter)
    );
}

fn rand_range(co: vec2<f32>, min: f32, max: f32, jitter: f32) -> f32 {
    return min + (max - min) * rand(co, jitter);
}

fn hash(co: vec2<f32>, jitter: f32) -> f32 {
    return fract(sin(dot(co + jitter, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

fn rand(co: vec2<f32>, jitter: f32) -> f32 {
    var v = hash( co + vec2<f32>(-1., 0.), jitter )
             + hash( co + vec2<f32>( 1., 0.), jitter )
             + hash( co + vec2<f32>( 0., 1.), jitter )
             + hash( co + vec2<f32>( 0.,-1.), jitter ); 
    return  hash(co, jitter) - v/4.  + .5;
}

fn vec_len_squared(vector: vec3<f32>) -> f32 {
    return vector.x*vector.x + vector.y*vector.y + vector.z*vector.z;
}

fn vec_sign(vector: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(
        sign(vector.x), 
        sign(vector.y), 
        sign(vector.z),
    );
}

fn sign(value: f32) -> f32 {
    if value > 0.0 {
        return 1.0;
    } else if value < 0.0 {
        return -1.0;
    } else {
        return 0.0;
    }
}