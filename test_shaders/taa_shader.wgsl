// TAA Shader

#import rt/utils.wgsl as Utils
#import rt/enums/debug_mode.wgsl as DebugMode

// ========= Uniforms =========

@group(0) @binding(0)
var current_velocity_texture: texture_2d<f32>;

@group(0) @binding(1)
var current_velocity_sampler: sampler;

@group(0) @binding(2)
var previous_velocity_texture: texture_2d<f32>;

@group(0) @binding(3)
var previous_velocity_sampler: sampler;

@group(1) @binding(0)
var history_texture: texture_2d<f32>;

@group(1) @binding(1)
var history_sampler: sampler;

@group(2) @binding(0)
var current_texture: texture_2d<f32>;

@group(2) @binding(1)
var current_sampler: sampler;

@group(3) @binding(0)
var emission_texture: texture_2d<f32>;

@group(3) @binding(1)
var emission_sampler: sampler;

var<push_constant> debug_info: Utils::DebugInfo;

// ========= Render =========

fn box_blur(
    texture: texture_2d<f32>, 
    texture_sampler: sampler, 
    uv: vec2<f32>,
) -> vec4<f32> {
    let resolution = textureDimensions(current_texture);
    
    let kernel = 20.0;
    let weight = 1.0;

	var sum = vec3<f32>(0.);
    let pixel_size = 1.0 / f32(resolution.x);
    
    var accumulation = vec3<f32>(0.);
    var weightsum = vec3<f32>(0.);
    for (var i = -kernel; i <= kernel; i += 1.0) {
        accumulation += textureSample(texture, texture_sampler, uv + vec2<f32>(i * pixel_size, 0.0)).xyz * weight;
        weightsum += weight;
    }
    
    for (var i = -kernel; i <= kernel; i += 1.0) {
        accumulation += textureSample(texture, texture_sampler, uv + vec2<f32>(0.0, i * pixel_size)).xyz * weight;
        weightsum += weight;
    }
    
    sum = accumulation / weightsum;
    
    return vec4<f32>(sum, 1.0);
}

fn gaussian_blur(
    texture: texture_2d<f32>, 
    texture_sampler: sampler, 
    uv: vec2<f32>,
) -> vec4<f32> {  
    let resolution = textureDimensions(current_texture);
    let color = textureSample(texture, texture_sampler, uv);

    let radius = 10;
    let pi = 3.1415926;
    let sigma = 5.;
    
    var gauss_sum = vec4<f32>(0.);
    
    for (var x = -radius; x <= radius; x++){
        for (var y = -radius; y <= radius; y++){
            let offset = vec2<f32>(f32(x), f32(y)) / vec2<f32>(f32(resolution.x), f32(resolution.y));

            let new_uv = uv + offset;
            gauss_sum += textureSample(texture, texture_sampler, new_uv) * (exp(-(pow(f32(x), 2.) + pow(f32(y), 2.)) / (2. * pow(sigma, 2.))) / (2. * pi * pow(sigma, 2.)));
        }   
    }

    return gauss_sum;
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

struct FragmentOut {
    @location(0) output_color: vec4<f32>,
}

@fragment
fn fs_main(
    @builtin(position) frag_pos: vec4<f32>,
) -> FragmentOut {
    var color = vec4<f32>(1.0);
    
    let resolution = textureDimensions(current_texture);
    let coords = frag_pos.xy / vec2<f32>(f32(resolution.x), f32(resolution.y));
    let emission = textureSample(emission_texture, emission_sampler, coords);
    // let emission = gaussian_blur(emission_texture, emission_sampler, coords) * 2;
    let current_velocity = textureSample(current_velocity_texture, current_velocity_sampler, coords).xy;
    let previous_velocity = textureSample(previous_velocity_texture, previous_velocity_sampler, coords + current_velocity).xy;

    let velocity_length = length(previous_velocity - current_velocity);
    let velocity_disocclusion = saturate((velocity_length - 0.001) * 10.0);

    let reproj_coords = coords + current_velocity;

    let current_color = textureSample(current_texture, current_sampler, coords).xyz;
    let blurred = box_blur(current_texture, current_sampler, coords).xyz;
    var history_color = textureSample(history_texture, history_sampler, reproj_coords).xyz;

    if (reproj_coords.x < 0.0
        || reproj_coords.x > 1.0
        || reproj_coords.y < 0.0
        || reproj_coords.y > 1.0) && current_velocity.x != -999.0
    {
        return FragmentOut(vec4<f32>(blurred, 1.0));
    } 

    var min_color = vec3<f32>(9999.0);
    var max_color = vec3<f32>(-9999.0);
 
    for (var x = -1; x <= 1; x++)
    {
        for (var y = -1; y <= 1; y++)
        {
            let color = Utils::adjust_hdr_color(textureSample(
                current_texture,
                current_sampler, 
                coords + vec2<f32>(f32(x), f32(y)) / vec2<f32>(f32(resolution.x), f32(resolution.y)),
            ).rgb).rgb;

            min_color = min(min_color, color);
            max_color = max(max_color, color);
        }
    }
    
    history_color = clamp(history_color, min_color, max_color);

    switch debug_info.debug_mode {
        case DebugMode::COLOR_TAA: {
            let accumulation = current_color * 0.1 + history_color * 0.9;
            
            if current_velocity.x == -999.0 {
                color = vec4<f32>(current_color, 1.0);
            } else if current_velocity.x != 0 && current_velocity.y != 0 {
                color = vec4<f32>(Utils::lerp(accumulation, blurred, velocity_disocclusion), 1.0);
            } else {
                color = vec4<f32>(accumulation, 1.0);
            }

            color = mix(color, emission, emission.a);
        }
        case DebugMode::VELOCITY: {
            color = vec4<f32>(previous_velocity.x, previous_velocity.y, 0.0, 1.0);
        }
        case DebugMode::EMISSION {
            color = vec4<f32>(current_velocity.x, current_velocity.y, 0.0, 1.0);
        }
        case DebugMode::COLOR_RAW {
            color = vec4<f32>(current_color, 1.0);
        }
        case DebugMode::DEPTH {
            color = emission;
        }
        default {
            color = vec4<f32>(0.0);
        }
    }
    return FragmentOut(color);
}
