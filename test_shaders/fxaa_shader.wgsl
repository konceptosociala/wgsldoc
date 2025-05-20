// FXAA Shader

#import rt/utils.wgsl as Utils
#import rt/enums/debug_mode.wgsl as DebugMode

// ========= Uniforms =========

@group(0) @binding(0)
var source_texture: texture_2d<f32>;

@group(0) @binding(1)
var source_sampler: sampler;

@group(1) @binding(0)
var<uniform> camera_transform: Utils::Transform;

@group(1) @binding(1)
var<storage, read> transforms_buffer: array<Utils::RtTransform>;

@group(1) @binding(2)
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

const FXAA_REDUCE_MIN: f32 = 1. / 128.;
const FXAA_REDUCE_MUL: f32 = 1. / 8.;
const FXAA_SPAN_MAX: f32 = 8.;

fn texcoords(
    frag_coord: vec2<f32>, 
    resolution: vec2<f32>,
    v_rgbNW: ptr<function, vec2<f32>>, 
    v_rgbNE: ptr<function, vec2<f32>>,
    v_rgbSW: ptr<function, vec2<f32>>, 
    v_rgbSE: ptr<function, vec2<f32>>,
    v_rgbM: ptr<function, vec2<f32>>,
) {
	let inverse_vp = 1.0 / resolution.xy;
	*v_rgbNW = (frag_coord + vec2<f32>(-1.0, -1.0)) * inverse_vp;
	*v_rgbNE = (frag_coord + vec2<f32>(1.0, -1.0)) * inverse_vp;
	*v_rgbSW = (frag_coord + vec2<f32>(-1.0, 1.0)) * inverse_vp;
	*v_rgbSE = (frag_coord + vec2<f32>(1.0, 1.0)) * inverse_vp;
	*v_rgbM = vec2<f32>(frag_coord * inverse_vp);
}

fn apply(
    tex: texture_2d<f32>, 
    sam: sampler,
    frag_coord: vec2<f32>, 
    resolution: vec2<f32>,
) -> vec4<f32> {
    var v_rgbNW: vec2<f32>;
	var v_rgbNE: vec2<f32>;
	var v_rgbSW: vec2<f32>;
	var v_rgbSE: vec2<f32>;
	var v_rgbM: vec2<f32>;

    texcoords(frag_coord, resolution, &v_rgbNW, &v_rgbNE, &v_rgbSW, &v_rgbSE, &v_rgbM);

    return fxaa(tex, sam, frag_coord, resolution, v_rgbNW, v_rgbNE, v_rgbSW, v_rgbSE, v_rgbM);
}

fn fxaa(
    tex: texture_2d<f32>, 
    sam: sampler,
    frag_coord: vec2<f32>, 
    resolution: vec2<f32>,
    v_rgbNW: vec2<f32>, 
    v_rgbNE: vec2<f32>,
    v_rgbSW: vec2<f32>, 
    v_rgbSE: vec2<f32>,
    v_rgbM: vec2<f32>,
) -> vec4<f32> {
    var color: vec4<f32>;
    var inverse_vp = vec2<f32>(1.0 / resolution.x, 1.0 / resolution.y);
    let rgbNW = textureSample(tex, sam, v_rgbNW).xyz;
    let rgbNE = textureSample(tex, sam, v_rgbNE).xyz;
    let rgbSW = textureSample(tex, sam, v_rgbSW).xyz;
    let rgbSE = textureSample(tex, sam, v_rgbSE).xyz;
    let texColor = textureSample(tex, sam, v_rgbM);
    let rgbM  = texColor.xyz;
    let luma = vec3<f32>(0.299, 0.587, 0.114);
    let lumaNW = dot(rgbNW, luma);
    let lumaNE = dot(rgbNE, luma);
    let lumaSW = dot(rgbSW, luma);
    let lumaSE = dot(rgbSE, luma);
    let lumaM  = dot(rgbM,  luma);
    let lumaMin = min(lumaM, min(min(lumaNW, lumaNE), min(lumaSW, lumaSE)));
    let lumaMax = max(lumaM, max(max(lumaNW, lumaNE), max(lumaSW, lumaSE)));
    
    var dir = vec2<f32>(0.0);
    dir.x = -((lumaNW + lumaNE) - (lumaSW + lumaSE));
    dir.y =  ((lumaNW + lumaSW) - (lumaNE + lumaSE));
    
    let dirReduce = max((lumaNW + lumaNE + lumaSW + lumaSE) *
                          (0.25 * FXAA_REDUCE_MUL), FXAA_REDUCE_MIN);
    
    let rcpDirMin = 1.0 / (min(abs(dir.x), abs(dir.y)) + dirReduce);
    dir = min(vec2(FXAA_SPAN_MAX, FXAA_SPAN_MAX),
              max(vec2(-FXAA_SPAN_MAX, -FXAA_SPAN_MAX),
              dir * rcpDirMin)) * inverse_vp;
    
    let rgbA = 0.5 * (
        textureSample(tex, sam, frag_coord * inverse_vp + dir * (1.0 / 3.0 - 0.5)).xyz +
        textureSample(tex, sam, frag_coord * inverse_vp + dir * (2.0 / 3.0 - 0.5)).xyz);
    let rgbB = rgbA * 0.5 + 0.25 * (
        textureSample(tex, sam, frag_coord * inverse_vp + dir * -0.5).xyz +
        textureSample(tex, sam, frag_coord * inverse_vp + dir * 0.5).xyz);

    let lumaB = dot(rgbB, luma);
    if (lumaB < lumaMin) || (lumaB > lumaMax) {
        color = vec4(rgbA, texColor.a);
    } else {
        color = vec4(rgbB, texColor.a);
    }
    return color;
}

@fragment
fn fs_main(
    @builtin(position) frag_pos: vec4<f32>,
) -> FragmentOut {
    return FragmentOut(apply(
        source_texture,
        source_sampler,
        frag_pos.xy,
        vec2<f32>(
            f32(taa_config2.canvas_width), 
            f32(taa_config2.canvas_height),
        ),
    ));

    // return FragmentOut(textureSample(
    //     source_texture, 
    //     source_sampler, 
    //     frag_pos.xy / vec2<f32>(
    //         f32(taa_config2.canvas_width), 
    //         f32(taa_config2.canvas_height),
    //     ),
    // ));
}
