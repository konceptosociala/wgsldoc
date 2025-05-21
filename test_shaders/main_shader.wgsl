// Vertex data
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) frag_pos: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) camera_pos: vec3<f32>,
};

// Camera
struct CameraUniform {
    position: vec3<f32>,
    view_projection: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

// Transform
struct TransformUniform {
    transform_matrix: mat4x4<f32>,
    inverse_matrix: mat4x4<f32>,
};

var<push_constant> transform: TransformUniform;

// Entries
@vertex
fn vs_main(
    input: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    let transform = TransformUniform();

    out.color = input.color;
    out.frag_pos = vec3<f32>((transform.transform_matrix * vec4<f32>(input.position, 1.0)).xyz);

    var transp = transpose(transform.inverse_matrix);
    out.normal = mat3x3<f32>(transp[0].xyz, transp[1].xyz, transp[2].xyz) * input.normal;
    out.camera_pos = camera.position;
    out.clip_position = camera.view_projection * vec4<f32>(out.frag_pos, 1.0);
    return out;
}

@fragment
fn fs_main(output: VertexOutput) -> @location(0) vec4<f32> {
    let light_pos = vec3(-64.0, 64.0, 64.0);
    let light_color = vec3<f32>(1.0, 1.0, 1.0);

    // ambient
    let ambient_strength = 0.1;
    let ambient = ambient_strength * output.color;
  	
    // diffuse 
    let norm = normalize(output.normal);
    let light_dir = normalize(light_pos - output.frag_pos);
    let diff = max(dot(norm, light_dir), 0.0);
    let diffuse = diff * light_color;
    
    // specular
    let specular_strength = 0.5;
    let view_dir = normalize(output.camera_pos - output.frag_pos);
    let reflect_dir = reflect(-light_dir, norm);  
    let spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32.0);
    let specular = specular_strength * spec * light_color;  
        
    let result = (ambient + diffuse + specular) * output.color;

    return vec4<f32>(result, 1.0);
}