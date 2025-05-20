// ========= Oriented Bounding Box =========

#import ray.wgsl as Ray
#import utils.wgsl as Utils

struct OBB {
    position: vec3<f32>,
    size: vec3<f32>,
}

fn hit(
    obb: OBB,   
    ray: Ray::Ray, 
    t_min: f32, 
    t_max: f32,
    transform: Utils::Transform,
    record: ptr<function, Ray::HitRecord>,
) -> bool {
    let delta = transform.transform_matrix[3].xyz - ray.origin;
    var tmin = t_min;
    var tmax = t_max;

    for (var d = 0u; d < 3u; d++) {
        let axis = transform.transform_matrix[d].xyz;
        let e = dot(axis, delta);
        let f_inv = 1.0 / dot(ray.direction, axis);

        var t1 = (e + 0.0) * f_inv;
        var t2 = (e + obb.size[d]) * f_inv;

        if t1 > t2 {
            let temp = t1;
            t1 = t2;
            t2 = temp;
        }

        tmin = max(t1, tmin);
        tmax = min(t2, tmax);

        if (tmax - 0.0001 < tmin) {
            return false;
        }
    }

    (*record).t = tmin;
    (*record).p = (transform.inverse_matrix * vec4<f32>(Ray::at(ray, (*record).t - 0.0001), 1.0)).xyz;
    (*record).normal = calculate_normal(obb, ray, (*record).p, transform);

    return true;
}

fn calculate_normal(
    obb: OBB,
    ray: Ray::Ray, 
    p: vec3<f32>,
    transform: Utils::Transform,
) -> vec3<f32> {
    let min = vec3<f32>(0.0);
    let max = obb.size;
    var normal = vec3<f32>(0.0);

    if abs(p.x - min.x) < 0.001 {
        normal = vec3<f32>(-1.0, 0.0, 0.0);
    } else if abs(p.x - max.x) < 0.001 {
        normal = vec3<f32>(1.0, 0.0, 0.0);
    } else if abs(p.y - min.y) < 0.001 {
        normal = vec3<f32>(0.0, -1.0, 0.0);
    } else if abs(p.y - max.y) < 0.001 {
        normal = vec3<f32>(0.0, 1.0, 0.0);
    } else if abs(p.z - min.z) < 0.001 {
        normal = vec3<f32>(0.0, 0.0, -1.0);
    } else if abs(p.z - max.z) < 0.001 {
        normal = vec3<f32>(0.0, 0.0, 1.0);
    }

    return normalize((transform.transform_matrix * vec4<f32>(normal, 0.0)).xyz);
}