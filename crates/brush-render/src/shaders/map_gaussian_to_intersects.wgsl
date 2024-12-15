#import helpers;

struct IsectInfo {
    compact_gid: i32,
    tile_id: i32,
}

@group(0) @binding(0) var<storage, read> num_intersections: i32;
@group(0) @binding(1) var<storage, read> isect_info: array<IsectInfo>;

@group(0) @binding(2) var<storage, read_write> cum_hits: array<atomic<i32>>;
@group(0) @binding(3) var<storage, read_write> tile_counts: array<atomic<i32>>;

@group(0) @binding(4) var<storage, read_write> tile_id_from_isect: array<i32>;
@group(0) @binding(5) var<storage, read_write> compact_gid_from_isect: array<i32>;

@compute
@workgroup_size(512, 1, 1)
fn main(@builtin(global_invocation_id) gid: vec3u) {
    let total_id = i32(gid.x);

    if total_id >= num_intersections {
        return;
    }

    let isect_info = isect_info[total_id];
    let tile_id = isect_info.tile_id;
    let compact_gid = isect_info.compact_gid;

    // Keep track of how many hits each tile has.
    atomicAdd(&tile_counts[tile_id + 1], 1);

    // Find base offset in the cumulative ghits.
    // var isect_id = cum_hits[compact_gid];
    var isect_id = atomicAdd(&cum_hits[compact_gid], 1);

    // Write to the intersection buffers which are now sorted by depth.
    tile_id_from_isect[isect_id] = tile_id;
    compact_gid_from_isect[isect_id] = compact_gid;
}
