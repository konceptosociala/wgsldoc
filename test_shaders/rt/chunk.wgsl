// ========= Chunk =========

#import ray.wgsl as Ray
#import aabb.wgsl as Aabb
#import voxel.wgsl as Voxel
#import constants.wgsl as Constants
#import utils.wgsl as Utils

@group(1) @binding(1)
var<storage, read> palettes_buffer: array<vec4<f32>>;

@group(1) @binding(2)
var chunk_storage: texture_3d<u32>;

var<push_constant> debug_info: Utils::DebugInfo;

const MAX_LOD: u32 = 2u;
const COORDS_ARRAY: u32 = MAX_LOD + 1u;

fn hit(
    chunk_id: i32,
    ray: Ray::Ray, 
    box_t_min: f32, 
    box_t_max: f32,
    record: ptr<function, Ray::HitRecord>,
) -> bool {
    if length(ray.direction) == 0 {
        return false;
    }

    var grid_record = Ray::HitRecord();

    let aabb = Aabb::Aabb(vec3<f32>(Constants::CHUNK_MIN), vec3<f32>(Constants::CHUNK_MAX));
    let chunk_size = Constants::CHUNK_SIZE / 4;
    
    if !Aabb::hit(aabb, ray, box_t_min, box_t_max, &grid_record) {
        return false;
    }

    let voxels_per_unit = f32(chunk_size) / (Constants::CHUNK_MAX - Constants::CHUNK_MIN);

    var entry_pos = ((ray.origin + ray.direction * (grid_record.t + 0.0001)) - Constants::CHUNK_MIN) * voxels_per_unit;

    let step = Utils::vec_sign(ray.direction);
    let t_delta = (1.0 / ray.direction) * step;

    var pos = clamp(floor(entry_pos), vec3<f32>(0.0), vec3<f32>(f32(chunk_size)));
    var t_max = (pos - entry_pos + max(step, vec3<f32>(0.0))) / ray.direction;

    var voxel = Voxel::Voxel();

    var axis = 0;

    for (var steps = 0u; steps < 32; steps++) {
        // Get voxel from voxel storage
        voxel = Voxel::parse(textureLoad(
            chunk_storage, 
            vec3<i32>(
                i32(pos.x), 
                i32(pos.y), 
                i32(pos.z) + chunk_id * i32(chunk_size),
            ), 
            2,
        ));

        if voxel.is_active {
            let big_pos = pos;
            let coef = Constants::VOXEL_SIZE * 4.0;

            let block_size = 4;
            let block_min = big_pos * coef;
            let block_max = (big_pos + vec3<f32>(1.0)) * coef;

            var grid_record = Ray::HitRecord();

            let aabb = Aabb::Aabb(vec3<f32>(block_min), vec3<f32>(block_max));

            if !Aabb::hit(aabb, ray, box_t_min, box_t_max, &grid_record) {
                return false;
            }

            let voxels_per_unit = f32(block_size) / (block_max - block_min);

            var entry_pos = ((ray.origin + ray.direction * (grid_record.t + 0.0001)) - block_min) * voxels_per_unit;

            let step = Utils::vec_sign(ray.direction);
            let t_delta = (1.0 / ray.direction) * step;

            var pos = clamp(floor(entry_pos), vec3<f32>(0.0), vec3<f32>(f32(block_size)));
            var t_max = (pos - entry_pos + max(step, vec3<f32>(0.0))) / ray.direction;

            var voxel = Voxel::Voxel();

            var axis = 0;

            for (var steps = 0u; steps < 16; steps++) {
                voxel = Voxel::parse(textureLoad(
                    chunk_storage,
                    vec3<i32>(
                        i32(big_pos.x) * 4 + i32(pos.x), 
                        i32(big_pos.y) * 4 + i32(pos.y), 
                        i32(Constants::CHUNK_SIZE) * chunk_id + i32(big_pos.z) * 4 + i32(pos.z),
                    ), 
                    0,
                ));

                if voxel.is_active {
                    (*record).normal = vec3<f32>(0.0);

                    var offset = 0.0;
                    // Check, if the voxel is first on the grid
                    if steps > 0 {
                        offset = (t_max[axis] - t_delta[axis]) / voxels_per_unit[axis];

                        (*record).normal[axis] = -step[axis];
                    } else {
                        (*record).normal = grid_record.normal;
                    }

                    (*record).t = grid_record.t + offset;
                    (*record).p = Ray::at(ray, (*record).t);
                    (*record).voxel_color = palettes_buffer[voxel.color_id].xyz;
                    (*record).voxel_mat = voxel.material;

                    return true;
                }

                if t_max.x < t_max.y { 
                    if t_max.x < t_max.z {
                        pos.x += step.x;
                        if pos.x < 0.0 || pos.x >= f32(block_size) { break; }

                        axis = 0;
                        t_max.x += t_delta.x;
                    } else {
                        pos.z += step.z; 
                        if pos.z < 0.0 || pos.z >= f32(block_size) { break; }

                        axis = 2;
                        t_max.z += t_delta.z;
                    } 
                } else { 
                    if t_max.y < t_max.z { 
                        pos.y += step.y; 
                        if pos.y < 0.0 || pos.y >= f32(block_size) { break; } 

                        axis = 1;
                        t_max.y += t_delta.y; 
                    } else { 
                        pos.z += step.z; 
                        if pos.z < 0.0 || pos.z >= f32(block_size) { break; } 

                        axis = 2;
                        t_max.z += t_delta.z; 
                    }
                }
            }
        } 

        if t_max.x < t_max.y { 
            if t_max.x < t_max.z {
                pos.x += step.x;
                if pos.x < 0.0 || pos.x >= f32(chunk_size) { return false; }

                axis = 0;
                t_max.x += t_delta.x;
            } else {
                pos.z += step.z; 
                if pos.z < 0.0 || pos.z >= f32(chunk_size) { return false; }

                axis = 2;
                t_max.z += t_delta.z;
            } 
        } else { 
            if t_max.y < t_max.z { 
                pos.y += step.y; 
                if pos.y < 0.0 || pos.y >= f32(chunk_size) { return false; } 

                axis = 1;
                t_max.y += t_delta.y; 
            } else { 
                pos.z += step.z; 
                if pos.z < 0.0 || pos.z >= f32(chunk_size) { return false; } 

                axis = 2;
                t_max.z += t_delta.z; 
            }
        } 
    }

    return false;
}

// fn hit(
//     chunk_id: i32,
//     ray: Ray::Ray, 
//     box_t_min: f32, 
//     box_t_max: f32,
//     record: ptr<function, Ray::HitRecord>,
// ) -> bool {
//     if length(ray.direction) == 0 {
//         return false;
//     }

//     var grid_record = Ray::HitRecord();

//     let aabb = Aabb::Aabb(vec3<f32>(Constants::CHUNK_MIN), vec3<f32>(Constants::CHUNK_MAX));
//     let chunk_size = Constants::CHUNK_SIZE / 4;
    
//     if !Aabb::hit(aabb, ray, box_t_min, box_t_max, &grid_record) {
//         return false;
//     }

//     let voxels_per_unit = f32(chunk_size) / (Constants::CHUNK_MAX - Constants::CHUNK_MIN);

//     var entry_pos = ((ray.origin + ray.direction * (grid_record.t + 0.0001)) - Constants::CHUNK_MIN) * voxels_per_unit;

//     let step = Utils::vec_sign(ray.direction);
//     let t_delta = (1.0 / ray.direction) * step;

//     var pos = clamp(floor(entry_pos), vec3<f32>(0.0), vec3<f32>(f32(chunk_size)));
//     var t_max = (pos - entry_pos + max(step, vec3<f32>(0.0))) / ray.direction;

//     var voxel = Voxel::Voxel();

//     var axis = 0;

//     for (var steps = 0u; steps < 16; steps++) {
//         // Get voxel from voxel storage
//         voxel = Voxel::parse(textureLoad(
//             chunk_storage,
//             vec3<i32>(
//                 i32(pos.x),
//                 i32(pos.y),
//                 // i32(pos.z) + chunk_id * i32(Constants::CHUNK_SIZE),
//                 i32(pos.z),
//             ), 
//             2,
//         ));

//         if voxel.is_active {
//             let prev_pos = pos;

//             let block_size = 2;
//             let block_min = prev_pos * Constants::VOXEL_SIZE * 4.0;
//             let block_max = block_min + vec3<f32>(Constants::VOXEL_SIZE * 4.0);

//             var grid_record = Ray::HitRecord();

//             let aabb = Aabb::Aabb(vec3<f32>(block_min), vec3<f32>(block_max));

//             if !Aabb::hit(aabb, ray, box_t_min, box_t_max, &grid_record) {
//                 return false;
//             }

//             let voxels_per_unit = f32(block_size) / (block_max - block_min);

//             var entry_pos = ((ray.origin + ray.direction * (grid_record.t + 0.0001)) - block_min) * voxels_per_unit;

//             let step = Utils::vec_sign(ray.direction);
//             let t_delta = (1.0 / ray.direction) * step;

//             var pos = clamp(floor(entry_pos), vec3<f32>(0.0), vec3<f32>(f32(block_size)));
//             var t_max = (pos - entry_pos + max(step, vec3<f32>(0.0))) / ray.direction;

//             var voxel = Voxel::Voxel();

//             var axis = 0;

//             for (var steps = 0u; steps < 4; steps++) {
//                 voxel = Voxel::parse(textureLoad(
//                     chunk_storage,
//                     vec3<i32>(
//                         i32(prev_pos.x) * 2 + i32(pos.x), 
//                         i32(prev_pos.y) * 2 + i32(pos.y), 
//                         // i32(Constants::CHUNK_SIZE) * chunk_id + i32(prev_pos.z) + i32(pos.z),
//                         i32(prev_pos.z) * 2 + i32(pos.z),
//                     ), 
//                     1,
//                 ));

//                 if voxel.is_active {
//                     let prev_prev_pos = prev_pos;
//                     let prev_pos = pos;

//                     let block_size = 2;
//                     let block_min = block_min + prev_pos * Constants::VOXEL_SIZE * 2.0;
//                     let block_max = block_min + vec3<f32>(Constants::VOXEL_SIZE * 2.0);

//                     var grid_record = Ray::HitRecord();

//                     let aabb = Aabb::Aabb(vec3<f32>(block_min), vec3<f32>(block_max));

//                     if !Aabb::hit(aabb, ray, box_t_min, box_t_max, &grid_record) {
//                         return false;
//                     }

//                     let voxels_per_unit = f32(block_size) / (block_max - block_min);

//                     var entry_pos = ((ray.origin + ray.direction * (grid_record.t + 0.0001)) - block_min) * voxels_per_unit;

//                     let step = Utils::vec_sign(ray.direction);
//                     let t_delta = (1.0 / ray.direction) * step;

//                     var pos = clamp(floor(entry_pos), vec3<f32>(0.0), vec3<f32>(f32(block_size)));
//                     var t_max = (pos - entry_pos + max(step, vec3<f32>(0.0))) / ray.direction;

//                     var voxel = Voxel::Voxel();

//                     var axis = 0;

//                     for (var steps = 0u; steps < 4; steps++) {
//                         voxel = Voxel::parse(textureLoad(
//                             chunk_storage,
//                             vec3<i32>(
//                                 i32(prev_prev_pos.x) * 4 + i32(prev_pos.x) * 2 + i32(pos.x), 
//                                 i32(prev_prev_pos.y) * 4 + i32(prev_pos.y) * 2 + i32(pos.y), 
//                                 // i32(Constants::CHUNK_SIZE) * chunk_id + i32(prev_pos.z) + i32(pos.z),
//                                 i32(prev_prev_pos.z) * 4 + i32(prev_pos.z) * 2 + i32(pos.z),
//                             ),
//                             0,
//                         ));

//                         if voxel.is_active {
//                             (*record).normal = vec3<f32>(0.0);

//                             var offset = 0.0;
//                             // Check, if the voxel is first on the grid
//                             if steps > 0 {
//                                 offset = (t_max[axis] - t_delta[axis]) / voxels_per_unit[axis];

//                                 (*record).normal[axis] = -step[axis];
//                             } else {
//                                 (*record).normal = grid_record.normal;
//                             }

//                             (*record).t = grid_record.t + offset;
//                             (*record).p = Ray::at(ray, (*record).t);
//                             (*record).voxel_color = palettes_buffer[voxel.color_id].xyz;
//                             (*record).voxel_mat = voxel.material;

//                             return true;
//                         }

//                         if t_max.x < t_max.y { 
//                             if t_max.x < t_max.z {
//                                 pos.x += step.x;
//                                 if pos.x < 0.0 || pos.x >= f32(block_size) { break; }

//                                 axis = 0;
//                                 t_max.x += t_delta.x;
//                             } else {
//                                 pos.z += step.z; 
//                                 if pos.z < 0.0 || pos.z >= f32(block_size) { break; }

//                                 axis = 2;
//                                 t_max.z += t_delta.z;
//                             } 
//                         } else { 
//                             if t_max.y < t_max.z { 
//                                 pos.y += step.y; 
//                                 if pos.y < 0.0 || pos.y >= f32(block_size) { break; } 

//                                 axis = 1;
//                                 t_max.y += t_delta.y; 
//                             } else { 
//                                 pos.z += step.z; 
//                                 if pos.z < 0.0 || pos.z >= f32(block_size) { break; } 

//                                 axis = 2;
//                                 t_max.z += t_delta.z; 
//                             }
//                         }
//                     }
//                 }

//                 if t_max.x < t_max.y { 
//                     if t_max.x < t_max.z {
//                         pos.x += step.x;
//                         if pos.x < 0.0 || pos.x >= f32(block_size) { break; }

//                         axis = 0;
//                         t_max.x += t_delta.x;
//                     } else {
//                         pos.z += step.z; 
//                         if pos.z < 0.0 || pos.z >= f32(block_size) { break; }

//                         axis = 2;
//                         t_max.z += t_delta.z;
//                     } 
//                 } else { 
//                     if t_max.y < t_max.z { 
//                         pos.y += step.y; 
//                         if pos.y < 0.0 || pos.y >= f32(block_size) { break; } 

//                         axis = 1;
//                         t_max.y += t_delta.y; 
//                     } else { 
//                         pos.z += step.z; 
//                         if pos.z < 0.0 || pos.z >= f32(block_size) { break; } 

//                         axis = 2;
//                         t_max.z += t_delta.z; 
//                     }
//                 }
//             }
//         } 

//         if t_max.x < t_max.y { 
//             if t_max.x < t_max.z {
//                 pos.x += step.x;
//                 if pos.x < 0.0 || pos.x >= f32(chunk_size) { return false; }

//                 axis = 0;
//                 t_max.x += t_delta.x;
//             } else {
//                 pos.z += step.z; 
//                 if pos.z < 0.0 || pos.z >= f32(chunk_size) { return false; }

//                 axis = 2;
//                 t_max.z += t_delta.z;
//             } 
//         } else { 
//             if t_max.y < t_max.z { 
//                 pos.y += step.y; 
//                 if pos.y < 0.0 || pos.y >= f32(chunk_size) { return false; } 

//                 axis = 1;
//                 t_max.y += t_delta.y; 
//             } else { 
//                 pos.z += step.z; 
//                 if pos.z < 0.0 || pos.z >= f32(chunk_size) { return false; } 

//                 axis = 2;
//                 t_max.z += t_delta.z; 
//             }
//         } 
//     }

//     return false;
// }

// {
//     var prev_pos = vec3<f32>(0.0);
//     var block_min = vec3<f32>(0.0);
//     var block_max = vec3<f32>(0.0);
//     var block_size = 0u;
//     var max_steps = 0u;
//     var accumulated_coords = array<vec3<i32>, COORDS_ARRAY>();

//     for (var lod = MAX_LOD; lod >= 0u; lod--) {
//         var grid_record = Ray::HitRecord();

//         if lod == MAX_LOD {
//             block_size = Constants::CHUNK_SIZE >> MAX_LOD;
//             block_min = Constants::CHUNK_MIN;
//             block_max = Constants::CHUNK_MAX;
//         } else {
//             block_size = 2u;
//             block_min = block_min + prev_pos * Constants::VOXEL_SIZE * f32(2u << lod);
//             block_max = block_min + vec3<f32>(Constants::VOXEL_SIZE * f32(2u << lod));
//         }

//         max_steps = block_size << 1u;

//         let aabb = Aabb::Aabb(block_min, block_max);

//         if !Aabb::hit(aabb, ray, box_t_min, box_t_max, &grid_record) {
//             return false;
//         }

//         let voxels_per_unit = f32(block_size) / (block_max - block_min);

//         var entry_pos = ((ray.origin + ray.direction * (grid_record.t + 0.0001)) - block_min) * voxels_per_unit;

//         let step = Utils::vec_sign(ray.direction);
//         let t_delta = (1.0 / ray.direction) * step;

//         var pos = clamp(floor(entry_pos), vec3<f32>(0.0), vec3<f32>(f32(block_size)));
//         var t_max = (pos - entry_pos + max(step, vec3<f32>(0.0))) / ray.direction;

//         var voxel = Voxel::Voxel();

//         var axis = 0;

//         for (var steps = 0u; steps < max_steps; steps++) {
//             var coords = vec3<i32>(
//                 i32(pos.x), 
//                 i32(pos.y), 
//                 i32(pos.z),
//             );

//             for (var i = 0u; i < MAX_LOD - lod; i++) {
//                 coords += accumulated_coords[lod + 1u + i] * i32(2u << i);
//             }
//             voxel = Voxel::parse(textureLoad(chunk_storage, coords, i32(lod)));

//             if voxel.is_active {
//                 if lod == 0u {
//                     (*record).normal = vec3<f32>(0.0);

//                     var offset = 0.0;
//                     // Check, if the voxel is first on the grid
//                     if steps > 0 {
//                         offset = (t_max[axis] - t_delta[axis]) / voxels_per_unit[axis];

//                         (*record).normal[axis] = -step[axis];
//                     } else {
//                         (*record).normal = grid_record.normal;
//                     }

//                     (*record).t = grid_record.t + offset;
//                     (*record).p = Ray::at(ray, (*record).t);
//                     (*record).voxel_color = palettes_buffer[voxel.color_id].xyz;
//                     (*record).voxel_mat = voxel.material;

//                     return true;
//                 } else {
//                     prev_pos = pos;
//                     accumulated_coords[lod] = vec3<i32>(
//                         i32(pos.x), 
//                         i32(pos.y), 
//                         i32(pos.z),
//                     );

//                     break;
//                 }
//             }

//             if t_max.x < t_max.y { 
//                 if t_max.x < t_max.z {
//                     pos.x += step.x;
//                     if pos.x < 0.0 || pos.x >= f32(block_size) { break; }

//                     axis = 0;
//                     t_max.x += t_delta.x;
//                 } else {
//                     pos.z += step.z; 
//                     if pos.z < 0.0 || pos.z >= f32(block_size) { break; }

//                     axis = 2;
//                     t_max.z += t_delta.z;
//                 } 
//             } else { 
//                 if t_max.y < t_max.z { 
//                     pos.y += step.y; 
//                     if pos.y < 0.0 || pos.y >= f32(block_size) { break; } 

//                     axis = 1;
//                     t_max.y += t_delta.y; 
//                 } else { 
//                     pos.z += step.z; 
//                     if pos.z < 0.0 || pos.z >= f32(block_size) { break; } 

//                     axis = 2;
//                     t_max.z += t_delta.z; 
//                 }
//             }
//         }
//     }

//     return false;
// }