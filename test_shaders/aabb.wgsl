//! # 📦 Axis-Aligned Bounding Box (AABB) Shader
//! Module for defining AABBs and performing ray-box intersection tests.
//! <br>
//! Depends on the `Ray` module (`ray.wgsl`) which provides ray structures and helpers.

#import ray.wgsl as Ray

/// 📐 Defines an axis-aligned bounding box using minimum and maximum 3D coordinates.
struct Aabb {
    /// Minimum corner of the box (lowest x/y/z).
    start: vec3<f32>,
    /// Maximum corner of the box (highest x/y/z).
    end: vec3<f32>,
}

/// 🔍 Tests whether a ray intersects a bounding box.
/// <br>
/// Updates the hit record with intersection details if a hit occurs.
fn hit(
    /// The axis-aligned bounding box to test against.
    aabb: Aabb, 

    /// The ray to test intersection with.
    ray: Ray::Ray, 

    /// The minimum t-value for valid intersections.
    t_min: f32, 

    /// The maximum t-value for valid intersections.
    t_max: f32,

    /// Pointer to a hit record that will be populated if the ray hits the box.
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
    (*record).p = Ray::at(ray, tmin);

    let center = (aabb.end + aabb.start) * 0.5;
    let direction = normalize((*record).p - center);

    // Estimate the normal based on the dominant direction from center
    (*record).normal = vec3<f32>(1.0, 1.0, 1.0);

    if abs(direction.x) >= abs(direction.y) && abs(direction.x) >= abs(direction.z) {
        (*record).normal = vec3<f32>(1.0, 0.0, 0.0);
    }
    if abs(direction.y) >= abs(direction.x) && abs(direction.y) >= abs(direction.z) {
        (*record).normal = vec3<f32>(0.0, 1.0, 0.0);
    }
    if abs(direction.z) >= abs(direction.x) && abs(direction.z) >= abs(direction.y) {
        (*record).normal = vec3<f32>(0.0, 0.0, 1.0);
    }

    // Flip normal to face against ray direction
    if dot(ray.direction, (*record).normal) >= 0.0 {
        (*record).normal = -(*record).normal;
    }

    return true;
}
