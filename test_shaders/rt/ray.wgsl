// ========= Ray =========

#import utils.wgsl as Utils

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
}

fn on_coords(
    pos: vec2<u32>, 
    camera: Utils::Camera,
) -> Ray {
    let pixel_sample = camera.first_pixel
        + (f32(pos.x) * camera.pixel_delta_u)
        + (f32(pos.y) * camera.pixel_delta_v);

    let ray_direction = pixel_sample - camera.center;

    return Ray(camera.center, ray_direction);
}

fn on_coordsf(
    pos: vec2<f32>, 
    camera: Utils::Camera,
) -> Ray {
    let pixel_sample = camera.first_pixel
        + (f32(pos.x) * camera.pixel_delta_u)
        + (f32(pos.y) * camera.pixel_delta_v);

    let ray_direction = pixel_sample - camera.center;

    return Ray(camera.center, ray_direction);
}

fn at(ray: Ray, t: f32) -> vec3<f32> {
    return ray.origin + t * ray.direction;
}

struct HitRecord {
    p: vec3<f32>,
    t: f32,
    normal: vec3<f32>,
    front_face: bool,
    voxel_color: vec3<f32>,
    voxel_mat: u32,
    velocity: vec2<f32>,
}

fn hit_record_set_face_normal(record: ptr<function, HitRecord>, ray: Ray, outward_normal: vec3<f32>) {
    (*record).front_face = dot(ray.direction, outward_normal) < 0.0;
    if (*record).front_face {
        (*record).normal = outward_normal;
    } else {
        (*record).normal = -outward_normal;
    }
}