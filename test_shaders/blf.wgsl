// Bilateral Filter Shader

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

const SIGMA: f32 = 10.0;
const BSIGMA: f32 = 0.1;
const MSIZE: i32 = 10;
const KSIZE = (MSIZE-1)/2;

struct FragmentOut {
    @location(0) output_color: vec4<f32>,
}

fn normpdf(x: f32, sigma: f32) -> f32 {
	return 0.39894*exp(-0.5*x*x/(sigma*sigma))/sigma;
}

fn normpdf3(v: vec3<f32>, sigma: f32) -> f32 {
	return 0.39894*exp(-0.5*dot(v,v)/(sigma*sigma))/sigma;
}

@fragment
fn fs_main(
    @builtin(position) frag_pos: vec4<f32>,
) -> FragmentOut {
    let resolution = vec2<f32>(f32(taa_config2.canvas_width), f32(taa_config2.canvas_height));
    let color = textureSample(
        source_texture,
        source_sampler,
        frag_pos.xy / resolution.xy,
    ).rgb;

    var kernel = array<f32, MSIZE>();
    var final_colour = vec3<f32>(0.0);
    
    var z = 0.0;
    for (var j = 0; j <= KSIZE; j++) {
        let value = normpdf(f32(j), SIGMA);
        kernel[KSIZE + j] = value;
        kernel[KSIZE - j] = value;
    }

    var cc = vec3<f32>(0.0);
    var factor = 0.0;
    let b_z = 1.0 / normpdf(0.0, BSIGMA);

    for (var i = -KSIZE; i <= KSIZE; i++) {
        for (var j = -KSIZE; j <= KSIZE; j++) {
            cc = textureSample(
                source_texture,
                source_sampler,
                (frag_pos.xy + vec2<f32>(f32(i), f32(j))) / resolution.xy,
            ).rgb;
            
            factor = normpdf3(cc - color, BSIGMA) * b_z * kernel[KSIZE + j] * kernel[KSIZE + i];
            z += factor;
            final_colour += factor * cc;

        }
    }

    return FragmentOut(vec4<f32>(final_colour / z, 1.0));
}
