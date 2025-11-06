#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::view::View;
#import aabb.wgsl as Aabb;

/// Screen color texture containing the rendered scene
@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var screen_sampler: sampler;
/// Normal vectors texture in view space for lighting calculations
@group(0) @binding(2) var normals_texture: texture_2d<f32>;
@group(0) @binding(3) var normals_sampler: sampler;
/// Depth buffer texture storing scene depth information
@group(0) @binding(4) var depth_texture: texture_2d<f32>;
@group(0) @binding(5) var depth_sampler: sampler;
@group(0) @binding(6) var world_volume: texture_3d<u32>;
/// Jitter values for temporal anti-aliasing and sampling
@group(0) @binding(7) var<uniform> jitter: Jitter;
@group(0) @binding(8) var<uniform> view: View;
/// Blue noise texture for high-quality stochastic sampling
@group(0) @binding(9) var blue_noise: texture_2d<f32>;
@group(0) @binding(10) var blue_noise_sampler: sampler;
@group(0) @binding(11) var<uniform> aabb: Aabb::Aabb;

struct DebugOptionsRaw {
    mode: u32,
    frame_number: u32,
    screen_size: vec2<f32>,
    bias: f32,
}

var<push_constant> debug_options: DebugOptionsRaw;

fn map_blue_noise(uv: vec2<f32>) -> f32 {
    let screen_px = uv * debug_options.screen_size;
    let tex_uv = fract(screen_px / 512.0);

    let frame_offset = f32(debug_options.frame_number % 100u) * 0.61803398875;
    let blue_noise = textureSample(blue_noise, blue_noise_sampler, tex_uv).r;
    let noise = fract(blue_noise + frame_offset);

    return noise;
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // let noise = map_blue_noise(in.uv);
    let color = textureSample(screen_texture, screen_sampler, in.uv).xyz;
    let depth = textureSample(depth_texture, depth_sampler, in.uv).x;
    if depth >= 3.40282347E+38 {
        return vec4<f32>(color, 1.0);
    }

    // Generate better noise seed using frame number and screen coordinates
    let screen_coord = floor(in.uv * debug_options.screen_size);
    let frame_seed = f32(debug_options.frame_number % 1024u) * 0.61803398875; // Golden ratio for better distribution
    let seed_ = vec2<f32>(
        screen_coord.x * 12.9898 + screen_coord.y * 78.233 + frame_seed,
        screen_coord.x * 93.9898 + screen_coord.y * 47.233 + frame_seed * 2.5
    );
    let seed = ((in.uv * 2.0 - 1.0) + jitter.value) * 0.5;
    var normal = textureSample(normals_texture, normals_sampler, in.uv).xyz;

    switch debug_options.mode {
        case 0u: {
            let trace1 = render(
                in.uv,
                color, 
                normal, 
                seed,
                depth,
            );

            // Second sample with different seed for better quality
            let seed2 = vec2<f32>(seed.y + 127.1, seed.x + 311.7);
            let trace2 = render(
                in.uv,
                color, 
                normal, 
                seed2,
                depth,
            );

            // Average multiple samples for smoother shadows
            return (trace1 + trace2) * 0.5;
            // return trace1;
        }

        case 1u {
            return vec4<f32>(color, 1.0);
        }

        case 2u: {
            return vec4<f32>(normal, 1.0);
        }

        case 3u: {
            return vec4<f32>(depth*0.1, 0.0, 0.0, 1.0);
        }

        case 4u: {
            return vec4<f32>(get_ray_origin(in.uv, depth), 1.0);
        }

        case 5u: {
            return vec4<f32>(get_ray_direction2(seed, normal), 1.0);
        }

        default: {
            return vec4<f32>(0.0);
        }
    }
}

const LIGHT_RADIUS: f32 = 0.08; // Slightly smaller for tighter shadows
const LIGHT_DIR: vec3<f32> = vec3<f32>(0.30, -0.95, 0.12);
const MAX_SHADOW_DISTANCE: f32 = 10.0; // Maximum distance to trace shadows

/// Ambient Occlusion radius (How far to trace for occluders) 
const AO_RADIUS: f32 = 2.0; 
/// Ambient Occlusion samples (Number of AO samples per pixel)
const AO_SAMPLES: u32 = 4u; 
/// Ambient Occlusion strength (How strong the AO effect is)
const AO_STRENGTH: f32 = 0.6; 

fn calculate_ambient_occlusion(
    origin: vec3<f32>,
    normal: vec3<f32>,
    seed: vec2<f32>,
    dimensions: vec3<u32>,
    dimensionsf: vec3<f32>,
    aabb_min: vec3<f32>,
    aabb_max: vec3<f32>,
    bias: f32
) -> f32 {
    var occlusion = 0.0;
    var valid_samples = 0.0;
    
    for (var i = 0u; i < AO_SAMPLES; i++) {
        // Generate different seeds for each sample
        let sample_seed = seed + vec2<f32>(f32(i) * 23.14, f32(i) * 17.93);
        
        // Generate random direction in hemisphere around normal
        let ao_direction = get_ray_direction2(sample_seed, normal);
        
        // Apply proper bias to prevent self-occlusion artifacts
        // Use the same bias calculation as shadow rays for consistency
        let surface_offset = normal * bias * 2.0; // Move away from surface
        var ao_origin = origin + surface_offset;
        
        // Check if ray intersects with world volume
        let rayDistances = intersectAABB(ao_origin, ao_direction, aabb_min, aabb_max);
        if (rayDistances.x >= rayDistances.y) {
            // Ray doesn't hit volume bounds - no occlusion
            continue;
        }
        
        ao_origin += ao_direction * rayDistances.x;
        let p = (ao_origin - aabb_min) / VOXEL_SIZE;
        let d = normalize(ao_direction);
        
        let step = sign(ao_direction);
        var v = floor(p);
        
        let tDelta = abs(1.0 / d);
        var tMax = vec3<f32>(0.0);
        
        // Initialize tMax for each axis
        if (d.x < 0) {
            tMax.x = (p.x - v.x) * tDelta.x;
        } else if (d.x > 0) {
            tMax.x = (v.x + 1.0 - p.x) * tDelta.x;
        }
        if (d.y < 0) {
            tMax.y = (p.y - v.y) * tDelta.y;
        } else if (d.y > 0) {
            tMax.y = (v.y + 1.0 - p.y) * tDelta.y;
        }
        if (d.z < 0) {
            tMax.z = (p.z - v.z) * tDelta.z;
        } else if (d.z > 0) {
            tMax.z = (v.z + 1.0 - p.z) * tDelta.z;
        }
        
        var t = 0.0;
        let maxIterations = u32(AO_RADIUS / VOXEL_SIZE);
        var hit_found = false;
        
        // Trace ray for ambient occlusion
        for (var j = 0u; j < maxIterations; j++) {
            // Check bounds
            if (v.x < 0 || v.x >= dimensionsf.x || 
                v.y < 0 || v.y >= dimensionsf.y || 
                v.z < 0 || v.z >= dimensionsf.z) {
                break;
            }
            
            let hit = textureLoad(world_volume, vec3<u32>(u32(v.x), u32(v.y), u32(v.z)), 0).x;
            if hit != 0 {
                // Found an occluder - calculate occlusion based on distance
                let distance_factor = 1.0 - (t * VOXEL_SIZE / AO_RADIUS);
                occlusion += max(distance_factor, 0.0);
                hit_found = true;
                break;
            }
            
            // Step to next voxel
            if (tMax.x < tMax.y) {
                if(tMax.x < tMax.z) {
                    v.x = v.x + step.x;
                    t = tMax.x;
                    tMax.x += tDelta.x;
                } else {
                    v.z = v.z + step.z;
                    t = tMax.z;
                    tMax.z += tDelta.z;
                }
            } else {
                if (tMax.y < tMax.z) {
                    v.y = v.y + step.y;
                    t = tMax.y;
                    tMax.y += tDelta.y;
                } else {
                    v.z = v.z + step.z;
                    t = tMax.z;
                    tMax.z += tDelta.z;
                }
            }
        }
        
        valid_samples += 1.0;
    }
    
    if (valid_samples > 0.0) {
        occlusion = occlusion / valid_samples;
    }
    
    // Convert occlusion to ambient factor (1.0 = no occlusion, 0.0 = full occlusion)
    return 1.0 - (occlusion * AO_STRENGTH);
}

fn render(
    uv: vec2<f32>,
    color: vec3<f32>,
    normal: vec3<f32>,
    seed: vec2<f32>,
    depth: f32,
) -> vec4<f32> {
    var origin = get_ray_origin(uv, depth);
    // let direction = get_ray_direction2(seed, normal);

    let dimensions = textureDimensions(world_volume);
    let dimensionsf = vec3<f32>(f32(dimensions.x), f32(dimensions.y), f32(dimensions.z));

    let aabb_min = -0.5 * dimensionsf * VOXEL_SIZE;
    let aabb_max =  0.5 * dimensionsf * VOXEL_SIZE;

    // Calculate ambient occlusion using a different seed portion
    let ao_seed = vec2<f32>(seed.x * 0.7 + 42.0, seed.y * 0.7 + 73.0);
    let bias = debug_options.bias * VOXEL_SIZE;
    let ao_factor_ = calculate_ambient_occlusion(origin, normal, ao_seed, dimensions, dimensionsf, aabb_min, aabb_max, bias);
    let ao_factor = ao_factor_ * ao_factor_;
    // Better random number generation for light sampling  
    let rng1 = rand(seed);
    let rng2 = rand(seed * 1.618 + vec2<f32>(127.1, 311.7)); // Better decorrelation
    
    // Uniform disk sampling - proper implementation
    let point_radius = LIGHT_RADIUS * sqrt(rng1);
    let point_angle = 2.0 * PI * rng2;
    let disk_point = vec2<f32>(
        point_radius * cos(point_angle),
        point_radius * sin(point_angle),
    );

    // Create orthonormal basis for the light disk
    let light_dir_norm = normalize(LIGHT_DIR);
    let light_tangent = normalize(cross(light_dir_norm, vec3<f32>(0.0, 1.0, 0.0)));
    let light_bitangent = cross(light_dir_norm, light_tangent); // Already normalized

    // Sample point on the light disk and create shadow ray direction
    let light_sample_point = disk_point.x * light_tangent + disk_point.y * light_bitangent;
    let shadow_ray_dir = normalize(light_dir_norm + light_sample_point);

    // Better bias calculation to prevent shadow acne  
    let surface_offset = normal * bias * 2.0; // Move away from surface
    origin += surface_offset;

    let rayDistances = intersectAABB(origin, shadow_ray_dir, aabb_min, aabb_max);
    origin += shadow_ray_dir * rayDistances.x;

    let p = (origin - aabb_min) / VOXEL_SIZE;
    let d = normalize(shadow_ray_dir);

    let step = sign(shadow_ray_dir);

    var v = floor(p);

    let tDelta = abs(1.0 / d);

    var tMax = vec3<f32>(0.0);
    if (d.x < 0) {
        tMax.x = (p.x - v.x) * tDelta.x;
    }
    else if (d.x > 0) {
        tMax.x = (v.x + 1.0 - p.x) * tDelta.x;
    }
    if (d.y < 0) {
        tMax.y = (p.y - v.y) * tDelta.y;
    }
    else if (d.y > 0) {
        tMax.y = (v.y + 1.0 - p.y) * tDelta.y;
    }
    if (d.z < 0) {
        tMax.z = (p.z - v.z) * tDelta.z;
    }
    else if (d.z > 0) {
        tMax.z = (v.z + 1.0 - p.z) * tDelta.z;
    }

    var t = 0.0;

    let maxIterations = dimensions.x+dimensions.y+dimensions.z;

    for (var i = 0u; i < maxIterations; i++) {
        let hit = textureLoad(world_volume, vec3<u32>(u32(v.x), u32(v.y), u32(v.z)), 0).x;
        if hit != 0 {
            // Shadow hit - calculate shadow strength based on distance
            let normalized_distance = min(t / MAX_SHADOW_DISTANCE, 1.0);
            let shadow_strength = 1.0 - normalized_distance * 0.3; // Distance-based attenuation
            let shadow_factor = shadow_strength * 0.2 + 0.1; // Shadow factor between 0.1 and 0.3
            
            // Combine shadow and ambient occlusion
            let final_lighting = shadow_factor * ao_factor;
            return vec4<f32>(color * final_lighting, 1.0);
        }

        if (tMax.x < tMax.y) {
            if(tMax.x < tMax.z) {
                v.x = v.x + step.x;
                if (v.x < 0 || v.x >= dimensionsf.x) { break; }
                t = tMax.x;
                tMax.x += tDelta.x;
            } else {
                v.z = v.z + step.z;
                if (v.z < 0 || v.z >= dimensionsf.z) { break; }
                t = tMax.z;
                tMax.z += tDelta.z;
            }
        } else {
            if (tMax.y < tMax.z) {
                v.y = v.y + step.y;
                if (v.y < 0 || v.y >= dimensionsf.y) { break; }
                t = tMax.y;
                tMax.y += tDelta.y;
            } else {
                v.z = v.z + step.z;
                if (v.z < 0 || v.z >= dimensionsf.z) { break; }
                t = tMax.z;
                tMax.z += tDelta.z;
            }
        }
    }

    // No shadow hit - apply ambient occlusion only
    // Add subtle distance-based darkening for depth
    let distance_factor = 1.0 - min(t * 0.05, 0.2); // Very subtle darkening for depth
    let final_lighting = ao_factor * distance_factor;
    return vec4<f32>(color * max(final_lighting, 0.2), 1.0); // Minimum lighting of 20%
}

// === UTILS ===

struct Jitter {
    value: vec2<f32>,
}

/// Super cool constant, what does that mean?
const PI: f32 = 3.1415926535897932384626433;
const VOXEL_SIZE = 1. / 8.;

fn intersectAABB(
    ray_origin: vec3<f32>,
    ray_dir: vec3<f32>,
    box_min: vec3<f32>,
    box_max: vec3<f32>,
) -> vec2<f32> {
    let t_min = (box_min - ray_origin) / ray_dir;
    let t_max = (box_max - ray_origin) / ray_dir;
    let t1 = min(t_min, t_max);
    let t2 = max(t_min, t_max);
    var t_near = max(max(t1.x, t1.y), t1.z);
    let t_far = min(min(t2.x, t2.y), t2.z);

    if (t_near < 0.0) {
        t_near = 0.0;
    }

    return vec2<f32>(t_near, t_far);
};

fn cosine_sample_hemisphere(seed: vec2<f32>) -> vec3<f32> {
    let r1 = rand(seed);
    let r2 = rand(seed.yx); // decorrelate
    let phi = 2.0 * PI * r1;
    let x = cos(phi) * sqrt(r2);
    let y = sin(phi) * sqrt(r2);
    let z = sqrt(1.0 - r2);
    return vec3<f32>(x, y, z); // In tangent space
}

fn get_ray_direction2(seed: vec2<f32>, normal_in: vec3<f32>) -> vec3<f32> {
    // 1) Safe normal
    let n = normalize(normal_in);

    // 2) Two decorrelated uniform randoms in [0,1)
    let r1 = rand(seed);
    let r2 = rand(seed + vec2<f32>(37.0, 17.0)); // cheap decorrelation

    // 3) Cosine-weighted sample in local (+Z) hemisphere
    let phi = 2.0 * PI * r1;
    let r   = sqrt(max(r2, 0.0));
    let x   = cos(phi) * r;
    let y   = sin(phi) * r;
    let z   = sqrt(max(1.0 - r*r, 0.0));
    let local_dir = vec3<f32>(x, y, z);

    // 4) Orthonormal basis (TBN) with a robust 'up' selection
    //    Choose an 'up' that isn't parallel to n.
    let up = select(vec3<f32>(0.0, 0.0, 1.0), vec3<f32>(1.0, 0.0, 0.0), abs(n.z) > 0.999);
    let tangent   = normalize(cross(up, n));
    let bitangent = normalize(cross(n, tangent));

    // 5) To world
    let dir = tangent * local_dir.x + bitangent * local_dir.y + n * local_dir.z;
    return normalize(dir);
}

fn get_ray_direction(
    seed: vec2<f32>,
    normal: vec3<f32>,
) -> vec3<f32> {
    let r1 = rand(seed);
    let r2 = rand(seed.yx);
    let phi = 2.0 * PI * r1;
    let x = cos(phi) * sqrt(r2);
    let y = sin(phi) * sqrt(r2);
    let z = sqrt(1.0 - r2);
    let local_dir = vec3<f32>(x, y, z);

    // Build orthonormal basis (TBN)
    let up = select(vec3<f32>(0.0, 1.0, 0.0), vec3<f32>(1.0, 0.0, 0.0), abs(normal.y) > 0.999);
    let tangent = normalize(cross(up, normal));
    let bitangent = cross(normal, tangent);

    // Transform to world space
    return normalize(
        tangent * local_dir.x +
        bitangent * local_dir.y +
        normal * local_dir.z
    );
}

fn get_ray_origin(
    coords: vec2<f32>,
    depth_lin: f32,
) -> vec3<f32> {
    // UV [0,1] -> NDC [-1,1], flip Y (screen origin at top-left)
    let ndc_xy = vec2<f32>(coords.x * 2.0 - 1.0, (1.0 - coords.y) * 2.0 - 1.0);

    // Unproject a point on the far plane to get the per-pixel *view-space direction*
    var v4 = view.view_from_clip * vec4<f32>(ndc_xy, 1.0, 1.0);
    v4 /= v4.w;
    let dir_v = normalize(v4.xyz); // z should be negative for a typical right-handed view

    // Scale along the view ray so that -z == depth_lin:
    // P = t * dir_v, want -P.z = depth_lin => t = depth_lin / (-dir_v.z)
    let t = depth_lin / (-dir_v.z + 1e-8);
    let pos_v = dir_v * t;

    // View -> World
    return (view.world_from_view * vec4<f32>(pos_v, 1.0)).xyz;
}

fn rand(seed: vec2<f32>) -> f32 {
    // Better white noise function with multiple octaves
    let dot_val = dot(seed, vec2<f32>(12.9898, 78.233));
    let noise1 = fract(sin(dot_val) * 43758.5453123);
    
    let dot_val2 = dot(seed, vec2<f32>(269.5, 183.3));
    let noise2 = fract(sin(dot_val2) * 59374.2861);
    
    return fract(noise1 + noise2 * 0.5);
}