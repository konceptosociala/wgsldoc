// Gaussian blur shader

#import rt/utils.wgsl as Utils

// var<private> GAUSSIAN_KERNEL: array<array<f32, 5>, 5> = array(
//     array(0.003, 0.013, 0.022, 0.013, 0.003),
//     array(0.013, 0.059, 0.097, 0.059, 0.013),
//     array(0.022, 0.097, 0.159, 0.097, 0.022),
//     array(0.013, 0.059, 0.097, 0.059, 0.013),
//     array(0.003, 0.013, 0.022, 0.013, 0.003)
// );

// fn apply_gaussian_blur(image_size: vec2<u32>, coord: vec2<u32>) -> vec4<f32> {
//     var color: vec4<f32> = vec4<f32>(0.0);
//     var kernel_sum: f32 = 0.0;

//     // Loop through the 5x5 kernel
//     for (var i: i32 = -3; i <= 3; i = i + 1) {
//         for (var j: i32 = -3; j <= 3; j = j + 1) {
//             let x: i32 = i32(coord.x) + i;
//             let y: i32 = i32(coord.y) + j;

//             // Check bounds to avoid reading outside the image
//             if (x >= 0 && y >= 0 && x < i32(image_size.x) && y < i32(image_size.y)) {
//                 let pixel_coord: u32 = u32(y * i32(image_size.x) + x);
//                 let pixel: vec4<f32> = vec4<f32>(color_data_buffer[pixel_coord].emission, 1.0);

//                 let weight: f32 = GAUSSIAN_KERNEL[i + 3][j + 3];
//                 color = color + pixel * weight;
//                 kernel_sum = kernel_sum + weight;
//             }
//         }
//     }

//     // Normalize the color
//     if (kernel_sum > 0.0) {
//         color = color / kernel_sum;
//     }

//     return color;
// }

@compute @workgroup_size(16, 16, 1)
fn cs_main(@builtin(global_invocation_id) id: vec3<u32>) {
    // let index = id.x + id.y * Utils::taa_config.canvas_width;

    // color_data_buffer[index].emission = apply_gaussian_blur(
    //     vec2<u32>(Utils::taa_config.canvas_width, Utils::taa_config.canvas_height), 
    //     id.xy,
    // ).xyz;
}