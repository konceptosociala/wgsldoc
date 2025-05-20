// Voxel ray tracing shader

#import rt/utils.wgsl as Utils
#import rt/constants.wgsl as Constants
#import rt/aabb.wgsl as Aabb
#import rt/ray.wgsl as Ray
#import rt/voxel.wgsl as Voxel
#import rt/chunk.wgsl as Chunk
#import rt/enums/voxel_mat.wgsl as VoxelMaterial
#import rt/bvh.wgsl as Bvh

// ========= Uniforms =========

@group(0) @binding(0)
var<uniform> camera_transform: Utils::RtTransform;

@group(0) @binding(1)
var<storage, read> transforms_buffer: array<Utils::RtTransform>;

@group(0) @binding(3)
var<uniform> sun_buffer: vec4<f32>;

@group(1) @binding(0)
var<uniform> camera: Utils::Camera;

@group(1) @binding(1)
var<storage, read> palettes_buffer: array<vec4<f32>>;

@group(1) @binding(2)
var chunk_storage: texture_3d<u32>;

struct FragmentOut {
    @location(0) output_color: vec4<f32>,
    @location(1) velocity: vec4<f32>,
    @location(2) emission: vec4<f32>,
}

fn bvh_hit(
    ray: Ray::Ray,
    t_min: f32,
    t_max: f32,
    record: ptr<function, Ray::HitRecord>,
    coords: vec2<f32>,
    shadowed: ptr<function, bool>,
    iteration: u32,
) -> bool {
    var temp = Ray::HitRecord();
    var hit_anything = false;
    var closest_so_far = t_max;

    var stack = array<i32, Constants::MAX_STACK>();
    var stack_ptr = 0;
    stack[stack_ptr] = 0; // Start from root node
    stack_ptr++;

    while (stack_ptr > 0) {
        stack_ptr--;
        let node_index = stack[stack_ptr];
        let node = Bvh::bvh_buffer[node_index];

        if Bvh::is_invalid(node) {
            return false;
        }

        if (Aabb::hit(node.aabb, ray, t_min, closest_so_far, &temp)) {
            if (node.left == -1 && node.right == -1) {
                // Leaf node
                for (var i = 0; i < node.count; i++) {
                    let idx = node.start + i;

                    var current_ray = ray;

                    current_ray.origin = (
                        transforms_buffer[idx].current_transform.inverse_matrix 
                            * vec4<f32>(ray.origin, 1.0)
                    ).xyz;

                    current_ray.direction = (
                        transforms_buffer[idx].current_transform.inverse_matrix
                            * vec4<f32>(ray.direction, 0.0)
                    ).xyz;

                    if Chunk::hit(idx, current_ray, t_min, closest_so_far, &temp) {
                        hit_anything = true;
                        closest_so_far = temp.t;
                        *record = temp;

                        if iteration == 0u {
                            var sun_ray: Ray::Ray;
                            sun_ray.origin = (*record).p + (*record).normal * 0.00001;
                            sun_ray.direction = sun_buffer.xyz;

                            var sun_record = Ray::HitRecord();

                            if Chunk::hit(idx, sun_ray, t_min, closest_so_far, &temp) {
                                *shadowed = true;
                            } else {
                                *shadowed = false;
                            }
                        }

                        (*record).p = (
                            transforms_buffer[idx].current_transform.transform_matrix 
                                * vec4<f32>((*record).p, 1.0)
                        ).xyz;

                        (*record).normal = ( 
                            transforms_buffer[idx].current_transform.transform_matrix
                                * vec4<f32>((*record).normal, 0.0)
                        ).xyz;

                        if iteration == 0 {
                            (*record).velocity = calc_velocity(coords, (*record).t);
                        }
                    }
                }
            } else {
                // Internal node: push children onto the stack
                stack[stack_ptr] = node.left;
                stack_ptr++;

                stack[stack_ptr] = node.right;
                stack_ptr++;
            }
        }
    }

    return hit_anything;
}

fn chunks_hit(
    ray: Ray::Ray,
    t_min: f32,
    t_max: f32,
    record: ptr<function, Ray::HitRecord>,
    coords: vec2<f32>,
    shadowed: ptr<function, bool>,
    iteration: u32,
) -> bool {
    var temp = Ray::HitRecord();
    var hit_anything = false;
    var closest_so_far = t_max;

    for (var i = 0; i < i32(textureDimensions(Chunk::chunk_storage).z / Constants::CHUNK_SIZE); i++) {
        var current_ray = ray;

        current_ray.origin = (
            transforms_buffer[i].current_transform.inverse_matrix 
                * vec4<f32>(ray.origin, 1.0)
        ).xyz;

        current_ray.direction = (
            transforms_buffer[i].current_transform.inverse_matrix
                * vec4<f32>(ray.direction, 0.0)
        ).xyz;

        if Chunk::hit(i, current_ray, t_min, closest_so_far, &temp) {
            hit_anything = true;
            closest_so_far = temp.t;
            *record = temp;

            if iteration == 0u {
                var sun_ray: Ray::Ray;
                sun_ray.origin = (*record).p + (*record).normal * 0.00001;
                sun_ray.direction = sun_buffer.xyz;

                var sun_record = Ray::HitRecord();

                if Chunk::hit(i, sun_ray, t_min, closest_so_far, &temp) {
                    *shadowed = true;
                } else {
                    *shadowed = false;
                }
            }

            (*record).p = (
                transforms_buffer[i].current_transform.transform_matrix 
                    * vec4<f32>((*record).p, 1.0)
            ).xyz;

            (*record).normal = ( 
                transforms_buffer[i].current_transform.transform_matrix
                    * vec4<f32>((*record).normal, 0.0)
            ).xyz;

            if iteration == 0 {
                (*record).velocity = calc_velocity(coords, (*record).t);
            }
        }
    }

    return hit_anything;
}

fn scatter(
    ray: Ray::Ray,
    record: ptr<function, Ray::HitRecord>,
    attenuation: ptr<function, vec3<f32>>,
    scattered: ptr<function, Ray::Ray>,
    co: vec2<f32>, 
    jitter: f32,
) -> bool {
    var material = (*record).voxel_mat;
    switch material {
        case VoxelMaterial::EMISSIVE: {
            return false;
        }
        default: {
            var scatter_direction = (*record).normal + Utils::random_vec_in_unit_sphere(co, jitter);
            *scattered = Ray::Ray((*record).p, scatter_direction);
            *attenuation = (*record).voxel_color;
            return true;
        }
    }
}

fn emit(hit_record: Ray::HitRecord) -> vec3<f32> {
    switch hit_record.voxel_mat {
        case VoxelMaterial::EMISSIVE: {
            return hit_record.voxel_color;
        }
        default: {
            return vec3<f32>(0.0);
        }
    }
}

fn render(ray: Ray::Ray, coords: vec2<f32>, scan_depth: u32) -> FragmentOut {
    let buffer_pos = vec2<u32>(
        u32(coords.x + 1.0),
        u32(coords.y + 1.0),
    );

    let j = (Utils::taa_config.jitter.x + Utils::taa_config.jitter.y) * 0.5;

    var current_ray = ray;
    current_ray.origin = (
        camera_transform.current_transform.inverse_matrix 
            * vec4<f32>(ray.origin, 1.0)
    ).xyz;
    current_ray.direction = (
        camera_transform.current_transform.inverse_matrix
            * vec4<f32>(ray.direction, 0.0)
    ).xyz;

    let unit_direction = normalize(current_ray.direction);
    let a = 0.5 * (unit_direction.y + 1.0);
    let background_color = (1.0 - a) * vec3<f32>(1.0) + a * vec3<f32>(0.5, 0.7, 1.0);

    var main_attenuation = vec3<f32>(1.0);
    
    var color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    var voxel_color = vec3<f32>(0.0);
    var velocity = vec2<f32>(-999.0, -999.0);
    var emission = vec4<f32>(0.0);

    var hit_record = Ray::HitRecord();
    var sky = false;
    var shadowed = false;

    for (var i = 0u; i < scan_depth; i++) {
        if !bvh_hit(current_ray, 0.001, 3.40282347e+38, &hit_record, coords, &shadowed, i) {
            color += vec4<f32>(main_attenuation * background_color, 0.0);
            if i == 0 {
                sky = true;
            }
            break;
        }
        
        if i == 0u {
            voxel_color = hit_record.voxel_color;
            velocity = hit_record.velocity;
            if hit_record.voxel_mat == VoxelMaterial::EMISSIVE {
                emission = vec4<f32>(hit_record.voxel_color, 1.0);
            }
        }

        var scattered = Ray::Ray();
        var emitted = emit(hit_record);
        var attenuation = vec3<f32>(0.0);

        if (!scatter(ray, &hit_record, &attenuation, &scattered, coords, j)) {
            color += vec4<f32>(main_attenuation * emitted, 0.0);
            break;
        }

        color += vec4<f32>(main_attenuation * emitted, 0.0);

        main_attenuation *= attenuation;
        current_ray = scattered;
    }

    if shadowed {
        color *= 0.5;
    } else {
        if !sky {
            color *= sun_buffer.w;
        }
    }

    return FragmentOut(color, vec4<f32>(velocity, 0.0, 0.0), emission);
}

fn calc_velocity(coord: vec2<f32>, depth_sample: f32) -> vec2<f32> {
    let current_pos = vec4<f32>(
        /* x */ (coord.x / f32(Utils::taa_config.canvas_width)) * 2.0 - 1.0,
        /* y */ (1.0 - (coord.y / f32(Utils::taa_config.canvas_height))) * 2.0 - 1.0,
        /* z */ depth_sample,
        1.0,
    );

    var pos_world = camera_transform.current_transform.inverse_matrix * current_pos;
    var previous_pos = camera_transform.previous_transform.transform_matrix * pos_world;

    let velocity = (current_pos - previous_pos) / 10.0;

    return vec2<f32>(velocity.x, -velocity.y);
}

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32
) -> @builtin(position) vec4<f32> {
    var x = 0.0;
    var y = 0.0;

    switch vertex_index {
        case 0u: {
            x = -1.0;
            y = -1.0;
        } 
        case 1u: {
            x = 1.0;
            y = -1.0;
        } 
        case 2u: {
            x = -1.0;
            y = 1.0;
        } 
        case 3u: {
            x = 1.0;
            y = -1.0;
        }
        case 4u: {
            x = 1.0;
            y = 1.0;
        } 
        case 5u: {
            x = -1.0;
            y = 1.0;
        }
        default: {}
    };

    return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_main(
    @builtin(position) frag_pos: vec4<f32>,
) -> FragmentOut {
    let current_ray = Ray::on_coordsf(frag_pos.xy, camera);

    return render(current_ray, frag_pos.xy, camera.scan_depth);
}