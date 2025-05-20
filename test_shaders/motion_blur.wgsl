// Motion Blur Shader

#import rt/utils.wgsl as Utils
#import rt/enums/debug_mode.wgsl as DebugMode

// ========= Uniforms =========

@group(0) @binding(0)
var velocity_texture: texture_2d<f32>;

@group(0) @binding(1)
var velocity_sampler: sampler;

@group(1) @binding(0)
var current_texture: texture_2d<f32>;

@group(1) @binding(1)
var current_sampler: sampler;

@group(2) @binding(0)
var<uniform> camera_transform: Utils::RtTransform;

@group(2) @binding(1)
var<storage, read> transforms_buffer: array<Utils::RtTransform>;

@group(2) @binding(2)
var<uniform> taa_config2: Utils::TaaConfig;

var<push_constant> debug_info: Utils::DebugInfo;

// ========= Render =========

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
    let color_pos = vec2<u32>(u32(frag_pos.x + 1.0), u32(frag_pos.y + 1.0));
    let velocity = textureSample(
        velocity_texture, 
        velocity_sampler, 
        frag_pos.xy / vec2<f32>(
            f32(taa_config2.canvas_width), 
            f32(taa_config2.canvas_height),
        ),
    ).xy;

    let coords = frag_pos.xy / vec2<f32>(f32(taa_config2.canvas_width), f32(taa_config2.canvas_height));

    var color = textureSample(current_texture, current_sampler, coords).xyz;

    if velocity.x != -999.0 {
        var tex_coord = coords + velocity * 0.05;

        for (var i = 1; i < 20; i++) {
            let current_color = textureSample(current_texture, current_sampler, tex_coord).xyz;
            color += current_color;

            tex_coord += velocity * 0.05;
        }

        color = color / 20;
    }

    return FragmentOut(vec4<f32>(color, 1.0));
}
