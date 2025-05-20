// ========= Voxel =========

struct Voxel {
    is_active: bool,    
    color_id: u32,
    material: u32,
}

fn parse(b_u32: vec4<u32>) -> Voxel {
    let is_active = (b_u32.x & 0xFFu) != 0u;    
    let color_id = b_u32.y & 0xFFu;
    let material = b_u32.z & 0xFFu;

    return Voxel(is_active, color_id, material);
}