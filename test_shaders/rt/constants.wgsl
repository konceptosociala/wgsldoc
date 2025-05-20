// ========= Constants =========

const MAX_CHUNKS: u32 = 64;

// FIXME: use `log2(MAX_CHUNKS) + 5` instead of hardcoding
const MAX_STACK: u32 = 11;

const MAX_NODES: u32 = 2 * MAX_CHUNKS - 1;

const MAX_TRAVERSAL_STEPS: u32 = 128;

const VOXEL_SIZE: f32 = 1.0 / 8.0;

const HALF_VOXEL_SIZE: f32 = VOXEL_SIZE / 2.0;

const CHUNK_SIZE: u32 = 32;

const CHUNK_ARRAY_SIZE: u32 = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

const CHUNK_MIN: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

const CHUNK_MAX: vec3<f32> = vec3<f32>(
    f32(CHUNK_SIZE) * VOXEL_SIZE,
    f32(CHUNK_SIZE) * VOXEL_SIZE,
    f32(CHUNK_SIZE) * VOXEL_SIZE,
);

const BLOCK_1_SIZE: u32 = 4;

const BLOCK_1_MIN: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

const BLOCK_1_MAX: vec3<f32> = vec3<f32>(
    f32(BLOCK_1_SIZE) * VOXEL_SIZE,
    f32(BLOCK_1_SIZE) * VOXEL_SIZE,
    f32(BLOCK_1_SIZE) * VOXEL_SIZE,
);