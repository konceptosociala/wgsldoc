//! **Axis-Aligned Bounding Box shader**
//! Module that describes AABB data structures
//! and functions to work with it

/// Ray import docs
#import ray.wgsl as Ray

/// Aabb struct docs
struct Aabb {
    /// Field1 docs
    start: vec2<f32>,
    end: vec3<f32>,

    test_field: bool,
    test_field2: u32,
    test_field3: Path,
    test_field4: Mod::Path,
}

/// Function docs
fn hit(
    /// Function arg1 docs
    aabb: Aabb, 
    ray: Ray::Ray, 
    t_min: f32, 
    t_max: f32,
    /// Function arg5 docs
    record: ptr<function, Ray::HitRecord>,
) -> bool {
    var tmin = t_min;
    var tmax = t_max;

    for (var axis = 0; axis < 3; axis++) {
        let t1 = (aabb.start[axis] - ray.origin[axis]) / ray.direction[axis];
        let t2 = (aabb.end[axis] - ray.origin[axis]) / ray.direction[axis];

        let dmin = min(t1, t2);
        let dmax = max(t1, t2);

        tmin = max(dmin, tmin);
        tmax = min(dmax, tmax);
    }

    if (tmax < tmin) {
        return false;
    }

    (*record).t = tmin;
    (*record).p = Ray::at(ray, (*record).t);
    
    let center = (aabb.end + aabb.start) * 0.5;    
    let norm_dir = normalize(vec3<f32>((*record).p.x - center.x, (*record).p.y - center.y, (*record).p.z - center.z));

    (*record).normal = vec3<f32>(1.0, 1.0, 1.0);

    if abs(norm_dir.x) >= abs(norm_dir.y) && abs(norm_dir.x) >= abs(norm_dir.z) {
        (*record).normal = vec3<f32>(1.0, 0.0, 0.0);
    }

    if abs(norm_dir.y) >= abs(norm_dir.x) && abs(norm_dir.y) >= abs(norm_dir.z) {
        (*record).normal = vec3<f32>(0.0, 1.0, 0.0);
    }

    if abs(norm_dir.z) >= abs(norm_dir.y) && abs(norm_dir.z) >= abs(norm_dir.x) {
        (*record).normal = vec3<f32>(0.0, 0.0, 1.0);
    }

    if dot(ray.direction, (*record).normal) >= 0.0 {
        (*record).normal = -(*record).normal;
    }

    return true;
}

