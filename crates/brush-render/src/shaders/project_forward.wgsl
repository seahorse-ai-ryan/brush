#define UNIFORM_WRITE

#import helpers;

// Unfiroms contains the splat count which we're writing to.
@group(0) @binding(0) var<storage, read_write> uniforms: helpers::RenderUniforms;

@group(0) @binding(1) var<storage, read> means: array<helpers::PackedVec3>;
@group(0) @binding(2) var<storage, read> log_scales: array<helpers::PackedVec3>;
@group(0) @binding(3) var<storage, read> quats: array<vec4f>;

@group(0) @binding(4) var<storage, read_write> global_from_compact_gid: array<u32>;
@group(0) @binding(5) var<storage, read_write> depths: array<f32>;

@compute
@workgroup_size(256, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3u) {
    let global_gid = global_id.x;

    if global_gid >= uniforms.total_splats {
        return;
    }

    // Project world space to camera space.
    let mean = helpers::as_vec(means[global_gid]);

    let img_size = uniforms.img_size;
    let viewmat = uniforms.viewmat;
    let R = mat3x3f(viewmat[0].xyz, viewmat[1].xyz, viewmat[2].xyz);
    let mean_c = R * mean + viewmat[3].xyz;

    if mean_c.z < 0.01 || mean_c.z > 1e12 {
        return;
    }

    // compute the projected covariance
    let scale = exp(helpers::as_vec(log_scales[global_gid]));
    let quat = quats[global_gid];

    let cov3d = helpers::calc_cov3d(scale, quat);
    let cov2d = helpers::calc_cov2d(cov3d, mean_c, uniforms.focal, uniforms.img_size, uniforms.pixel_center, viewmat);
    let det = cov2d.x * cov2d.z - cov2d.y * cov2d.y;

    if det <= 0.0 {
        return;
    }

    // Calculate ellipse conic.
    let conic = helpers::inverse_symmetric(cov2d);

    // compute the projected mean
    let mean2d = uniforms.focal * mean_c.xy * (1.0 / mean_c.z) + uniforms.pixel_center;

    // TODO: Include opacity here or is this ok?
    let radius = helpers::radius_from_cov(helpers::inverse_symmetric(conic), 1.0);

    if (radius <= 0) {
        return;
    }

    let radius_f = f32(radius);

    // mask out gaussians outside the image region
    if (mean2d.x + radius_f <= 0 || mean2d.x - radius_f >= f32(uniforms.img_size.x) ||
        mean2d.y + radius_f <= 0 || mean2d.y - radius_f >= f32(uniforms.img_size.y)) {
        return;
    }

    let tile_minmax = helpers::get_tile_bbox(mean2d, radius, uniforms.tile_bounds);
    let tile_min = tile_minmax.xy;
    let tile_max = tile_minmax.zw;

    // Now write all the data to the buffers.
    let write_id = atomicAdd(&uniforms.num_visible, 1u);
    global_from_compact_gid[write_id] = global_gid;
    depths[write_id] = mean_c.z;
}
