use crate::{camera::Camera, Backend};
use assert_approx_eq::assert_approx_eq;
use burn::{
    backend::Autodiff,
    tensor::{Tensor, TensorPrimitive},
};
use burn_wgpu::{Wgpu, WgpuDevice};

type DiffBack = Autodiff<Wgpu>;

#[tokio::test]
async fn renders_at_all() {
    // Check if rendering doesn't hard crash or anything.
    // These are some zero-sized gaussians, so we know
    // what the result should look like.
    let cam = Camera::new(
        glam::vec3(0.0, 0.0, 0.0),
        glam::Quat::IDENTITY,
        0.5,
        0.5,
        glam::vec2(0.5, 0.5),
    );
    let img_size = glam::uvec2(32, 32);
    let device = WgpuDevice::DefaultDevice;
    let num_points = 8;
    let means = Tensor::<DiffBack, 2>::zeros([num_points, 3], &device);
    let xy_dummy = Tensor::<DiffBack, 2>::zeros([num_points, 2], &device);
    let log_scales = Tensor::<DiffBack, 2>::ones([num_points, 3], &device) * 2.0;
    let quats: Tensor<DiffBack, 2> =
        Tensor::<DiffBack, 1>::from_floats(glam::Quat::IDENTITY.to_array(), &device)
            .unsqueeze_dim(0)
            .repeat_dim(0, num_points);
    let sh_coeffs = Tensor::<DiffBack, 3>::ones([num_points, 1, 3], &device);
    let raw_opacity = Tensor::<DiffBack, 1>::zeros([num_points], &device);
    let (output, aux) = DiffBack::render_splats(
        &cam,
        img_size,
        means.into_primitive().tensor(),
        xy_dummy.into_primitive().tensor(),
        log_scales.into_primitive().tensor(),
        quats.into_primitive().tensor(),
        sh_coeffs.into_primitive().tensor(),
        raw_opacity.into_primitive().tensor(),
        false,
    );
    aux.into_wrapped().debug_assert_valid();

    let output: Tensor<DiffBack, 3> = Tensor::from_primitive(TensorPrimitive::Float(output));
    let rgb = output.clone().slice([0..32, 0..32, 0..3]);
    let alpha = output.clone().slice([0..32, 0..32, 3..4]);
    let rgb_mean = rgb.clone().mean().to_data().as_slice::<f32>().unwrap()[0];
    let alpha_mean = alpha.clone().mean().to_data().as_slice::<f32>().unwrap()[0];
    assert_approx_eq!(rgb_mean, 0.0, 1e-5);
    assert_approx_eq!(alpha_mean, 0.0);
}
