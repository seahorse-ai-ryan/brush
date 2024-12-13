// #[test]
// fn test_mean_grads() {
//     let cam = Camera::new(glam::vec3(0.0, 0.0, -5.0), glam::Quat::IDENTITY, 0.5, 0.5);
//     let num_points = 1;

//     let img_size = glam::uvec2(16, 16);

//     let means = Tensor::<DiffBack, 2, _>::zeros([num_points, 3], &device).require_grad();
//     let log_scales = Tensor::ones([num_points, 3], &device).require_grad();
//     let quats = Tensor::from_data(glam::Quat::IDENTITY.to_array(), &device)
//         .unsqueeze_dim(0)
//         .repeat(0, num_points)
//         .require_grad();
//     let sh_coeffs = Tensor::zeros([num_points, 4], &device).require_grad();
//     let raw_opacity = Tensor::zeros([num_points], &device).require_grad();

//     let mut dloss_dmeans_stat = Tensor::zeros([num_points], &device);

//     // Calculate a stochasic gradient estimation by perturbing random dimensions.
//     let num_iters = 50;

//     for _ in 0..num_iters {
//         let eps = 1e-4;

//         let flip_vec = Tensor::<DiffBack, 1>::random(
//             [num_points],
//             burn::tensor::Distribution::Uniform(-1.0, 1.0),
//             &device,
//         );
//         let seps = flip_vec * eps;

//         let l1 = render(
//             &cam,
//             img_size,
//             means.clone(),
//             log_scales.clone(),
//             quats.clone(),
//             sh_coeffs.clone(),
//             raw_opacity.clone() - seps.clone(),
//             glam::Vec3::ZERO,
//         )
//         .0
//         .mean();

//         let l2 = render(
//             &cam,
//             img_size,
//             means.clone(),
//             log_scales.clone(),
//             quats.clone(),
//             sh_coeffs.clone(),
//             raw_opacity.clone() + seps.clone(),
//             glam::Vec3::ZERO,
//         )
//         .0
//         .mean();

//         let df = l2 - l1;
//         dloss_dmeans_stat = dloss_dmeans_stat + df * (seps * 2.0).recip();
//     }

//     dloss_dmeans_stat = dloss_dmeans_stat / (num_iters as f32);
//     let dloss_dmeans_stat = dloss_dmeans_stat.into_data().value;

//     let loss = render(
//         &cam,
//         img_size,
//         means.clone(),
//         log_scales.clone(),
//         quats.clone(),
//         sh_coeffs.clone(),
//         raw_opacity.clone(),
//         glam::Vec3::ZERO,
//     )
//     .0
//     .mean();
//     // calculate numerical gradients.
//     // Compare to reference value.

//     // Check if rendering doesn't hard crash or anything.
//     // These are some zero-sized gaussians, so we know
//     // what the result should look like.
//     let grads = loss.backward();

//     // Get the gradient of the rendered image.
//     let dloss_dmeans = (Tensor::<BurnBack, 1>::from_primitive(
//         grads.get(&raw_opacity.clone().into_primitive()).unwrap(),
//     ))
//     .into_data()
//     .value;

//     println!("Stat grads {dloss_dmeans_stat:.5?}");
//     println!("Calc grads {dloss_dmeans:.5?}");

//     // TODO: These results don't make sense at all currently, which is either
//     // mildly bad news or very bad news :)
// }
