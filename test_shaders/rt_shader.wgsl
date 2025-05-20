// Voxel ray tracing shader

#import rt/utils.wgsl as Utils
#import rt/constants.wgsl as Constants
#import rt/aabb.wgsl as Aabb
#import rt/ray.wgsl as Ray
#import rt/voxel.wgsl as Voxel
#import rt/chunk.wgsl as Chunk
#import rt/enums/voxel_mat.wgsl as VoxelMaterial

// ========= Uniforms =========

@group(0) @binding(0)
var<uniform> camera_transform: Utils::Transform;

@group(0) @binding(1)
var<storage, read> transforms_buffer: array<Utils::RtTransform>;

@group(1) @binding(0)
var<storage, read_write> color_data_buffer: array<Utils::ColorData>;

@group(2) @binding(0)
var<uniform> camera: Utils::Camera;

@group(2) @binding(1)
var<storage, read> palettes_buffer: array<vec4<f32>>;

fn chunks_hit(
    ray: Ray::Ray,
    t_min: f32,
    t_max: f32,
    record: ptr<function, Ray::HitRecord>,
    color_data: ptr<function, Utils::ColorData>,
    coords: vec2<f32>,
    first_iteration: bool,
) -> bool {
    var temp = Ray::HitRecord();
    var hit_anything = false;
    var closest_so_far = t_max;

    for (var i = 0; i < i32(textureDimensions(Chunk::chunk_storage).z / Constants::CHUNK_SIZE); i++) {
        var current_ray = ray;
        current_ray.origin = (transforms_buffer[i].current_transform.inverse_matrix * vec4<f32>(ray.origin, 1.0)).xyz;
        current_ray.direction = (transforms_buffer[i].current_transform.inverse_matrix * vec4<f32>(ray.direction, 0.0)).xyz;

        if Chunk::hit(i, current_ray, t_min, closest_so_far, &temp) {
            hit_anything = true;
            closest_so_far = temp.t;
            *record = temp;

            (*record).p = (transforms_buffer[i].current_transform.transform_matrix * vec4<f32>((*record).p, 1.0)).xyz;
            (*record).normal = (transforms_buffer[i].current_transform.transform_matrix * vec4<f32>((*record).normal, 0.0)).xyz;

            if first_iteration {
                (*color_data).velocity = calc_velocity(coords, temp.t, i, closest_so_far);
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

fn render(ray: Ray::Ray, co: vec2<u32>, scan_depth: u32) {
    let j = (Utils::taa_config.jitter.x + Utils::taa_config.jitter.y) * 0.5;
    let index = co.x + co.y * Utils::taa_config.canvas_width;
    let coords = vec2<f32>(f32(co.x)/f32(Utils::taa_config.canvas_width), f32(co.y)/f32(Utils::taa_config.canvas_height));

    let unit_direction = normalize(ray.direction);
    let a = 0.5 * (unit_direction.y + 1.0);
    let background_color = (1.0 - a) * vec3<f32>(1.0) + a * vec3<f32>(0.5, 0.7, 1.0);

    var current_ray = ray;
    var main_attenuation = vec3<f32>(1.0);
    
    var color_data = Utils::ColorData();

    var voxel_color = vec3<f32>(0.0);

    for (var i = 0u; i < scan_depth; i++) {
        var hit_record = Ray::HitRecord();

        if !chunks_hit(current_ray, 0.001, 3.40282347e+38, &hit_record, &color_data, coords, i == 0u) {
            color_data.color += main_attenuation * background_color;
            break;
        }

        if i == 0u {
            voxel_color = hit_record.voxel_color;
            if hit_record.voxel_mat == VoxelMaterial::EMISSIVE {
                color_data.emission = hit_record.voxel_color;
            }
            color_data.depth = hit_record.t;
        }

        var scattered = Ray::Ray();
        var emitted = emit(hit_record);
        var attenuation = vec3<f32>(0.0);

        if (!scatter(ray, &hit_record, &attenuation, &scattered, vec2<f32>(co), j)) {
            color_data.color += main_attenuation * emitted;
            break;
        }

        color_data.color += main_attenuation * emitted;

        main_attenuation *= attenuation;
        current_ray = scattered;
    }

    // let blue_noise = voxel_color * vec3<f32>(Utils::rand_range(vec2<f32>(co), 0.0, 1.0, Utils::taa_config.jitter));
    color_data_buffer[index] = color_data;
}

fn calc_velocity(
    coord: vec2<f32>, 
    depth_sample: f32, 
    transform_idx: i32,
    max_depth: f32,
) -> vec2<f32> {
    if (depth_sample >= 3.40282347e+38) {
        return vec2<f32>(0.0, 0.0);
    }

    let pos_clip = vec4(coord.x * 2.0 - 1.0, (1.0 - coord.y) * 2.0 - 1.0, depth_sample, 1.0);
    let pos_world_w = transforms_buffer[transform_idx].current_transform.inverse_matrix * pos_clip;
    let pos_world = pos_world_w / pos_world_w.w;

    let current_pos = pos_clip;
    let previous_pos = transforms_buffer[transform_idx].previous_transform.transform_matrix * pos_world;
    let velocity = (current_pos - previous_pos) / 2.0;
    return vec2<f32>(velocity.x, velocity.y);
}

@compute @workgroup_size(16, 16, 1)
fn cs_main(
    @builtin(global_invocation_id) id: vec3<u32>
) {
    let current_ray = Ray::on_coords(id.xy, camera);

    render(current_ray, id.xy, camera.scan_depth);
}