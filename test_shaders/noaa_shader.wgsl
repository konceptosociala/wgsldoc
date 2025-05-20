// TAA Shader

#import rt/utils.wgsl as Utils
#import rt/enums/debug_mode.wgsl as DebugMode

// ========= Uniforms =========

@group(0) @binding(0)
var<uniform> camera_transform: Utils::Transform;

@group(0) @binding(1)
var<storage, read> transforms_buffer: array<Utils::RtTransform>;

@group(0) @binding(2)
var<uniform> taa_config2: Utils::TaaConfig;

@group(1) @binding(0)
var<uniform> camera: Utils::Camera;

@group(1) @binding(1)
var<storage, read> palettes_buffer: array<vec4<f32>>;

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

    let color_pos = vec2<u32>(
        u32(frag_pos.x + 1.0),
        u32(frag_pos.y + 1.0),
    );

    var color = vec4<f32>(1.0);

    return FragmentOut(color);
}
