#import ray.wgsl as Ray
#import aabb.wgsl as Aabb
#import constants.wgsl as Constants
#import chunk.wgsl as Chunk

@group(1) @binding(3)
var<storage, read> bvh_buffer: array<BvhNode>;

struct BvhNode {
    aabb: Aabb::Aabb,
    left: i32,   // Index of the left child (-1 if leaf)
    right: i32,  // Index of the right child (-1 if leaf)
    start: i32,  // Start index of primitives (-1 if not leaf)
    count: i32,  // Number of primitives (-1 if not leaf)
}

fn is_invalid(node: BvhNode) -> bool {
    return node.left == -1 && node.right == -1 && node.start == -1 && node.count == -1;
}