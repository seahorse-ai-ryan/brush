use burn::{
    backend::{
        autodiff::{
            checkpoint::{base::Checkpointer, strategy::CheckpointStrategy},
            grads::Gradients,
            ops::{Backward, Ops, OpsKind},
        },
        Autodiff,
    },
    tensor::{
        backend::AutodiffBackend,
        ops::FloatTensor,
        repr::{CustomOpDescription, HandleContainer, OperationDescription},
        DType, Tensor, TensorPrimitive,
    },
};
use burn_fusion::{client::FusionClient, stream::Operation, Fusion};
use burn_jit::fusion::{FusionJitRuntime, JitFusionHandle};
use burn_wgpu::WgpuRuntime;

use crate::{
    camera::Camera,
    render::{
        calc_tile_bounds, max_intersections, render_backward, render_forward, sh_coeffs_for_degree,
        sh_degree_from_coeffs,
    },
    shaders, BBase, Backend, GaussianBackwardState, RenderAuxPrimitive, SplatGrads,
};

// Implement forward functions for the inner wgpu backend.
impl Backend for BBase {
    fn render_splats(
        camera: &Camera,
        img_size: glam::UVec2,
        means: FloatTensor<Self>,
        _xy_dummy: FloatTensor<Self>,
        log_scales: FloatTensor<Self>,
        quats: FloatTensor<Self>,
        sh_coeffs: FloatTensor<Self>,
        raw_opacity: FloatTensor<Self>,
        render_u32_buffer: bool,
    ) -> (FloatTensor<Self>, RenderAuxPrimitive<Self>) {
        render_forward(
            camera,
            img_size,
            means,
            log_scales,
            quats,
            sh_coeffs,
            raw_opacity,
            render_u32_buffer,
        )
    }

    fn render_splats_bwd(
        state: GaussianBackwardState<Self>,
        v_output: FloatTensor<Self>,
    ) -> SplatGrads<Self> {
        let bwd_state = state.rx.borrow().data().clone();
        let max_intersects = state.compact_gid_from_isect.shape.dims[0] as u32;

        if bwd_state.num_intersects > max_intersects {
            panic!("Too many tile intersections. This can happen for scenes with a lot of large gaussians ({} > {})", bwd_state.num_intersects, max_intersects);
        }

        render_backward(
            v_output,
            state.means,
            state.quats,
            state.log_scales,
            state.raw_opac,
            state.out_img,
            state.projected_splats,
            state.uniforms_buffer,
            state.compact_gid_from_isect,
            state.global_from_compact_gid,
            state.tile_offsets,
            state.final_index,
            bwd_state.num_visible,
            state.sh_degree,
        )
    }
}

#[derive(Debug)]
struct RenderBackwards;

const NUM_ARGS: usize = 6;

// Implement gradient registration when rendering backwards.
impl<B: Backend> Backward<B, NUM_ARGS> for RenderBackwards {
    type State = GaussianBackwardState<B>;

    fn backward(
        self,
        ops: Ops<Self::State, NUM_ARGS>,
        grads: &mut Gradients,
        _checkpointer: &mut Checkpointer,
    ) {
        let _span = tracing::trace_span!("render_gaussians backwards").entered();

        let state = ops.state;

        let v_output = grads.consume::<B>(&ops.node);

        // Register gradients for parent nodes (This code is already skipped entirely
        // if no parent nodes require gradients).
        let [mean_parent, xys_parent, log_scales_parent, quats_parent, coeffs_parent, raw_opacity_parent] =
            ops.parents;

        let v_tens = B::render_splats_bwd(state, v_output);

        if let Some(node) = mean_parent {
            grads.register::<B>(node.id, v_tens.v_means);
        }

        // Register the gradients for the dummy xy input.
        if let Some(node) = xys_parent {
            grads.register::<B>(node.id, v_tens.v_xy);
        }

        if let Some(node) = log_scales_parent {
            grads.register::<B>(node.id, v_tens.v_scales);
        }

        if let Some(node) = quats_parent {
            grads.register::<B>(node.id, v_tens.v_quats);
        }

        if let Some(node) = coeffs_parent {
            grads.register::<B>(node.id, v_tens.v_coeffs);
        }

        if let Some(node) = raw_opacity_parent {
            grads.register::<B>(node.id, v_tens.v_raw_opac);
        }
    }
}

// Implement
impl<B: Backend, C: CheckpointStrategy> Backend for Autodiff<B, C> {
    fn render_splats(
        camera: &Camera,
        img_size: glam::UVec2,
        means: FloatTensor<Self>,
        xy_dummy: FloatTensor<Self>,
        log_scales: FloatTensor<Self>,
        quats: FloatTensor<Self>,
        sh_coeffs: FloatTensor<Self>,
        raw_opacity: FloatTensor<Self>,
        render_u32_buffer: bool,
    ) -> (FloatTensor<Self>, RenderAuxPrimitive<Self>) {
        // Get backend tensors & dequantize if needed. Could try and support quantized inputs
        // in the future.

        // Prepare backward pass, and check if we even need to do it. Store nodes that need gradients.
        let prep_nodes = RenderBackwards
            .prepare::<C>([
                means.node.clone(),
                xy_dummy.node.clone(),
                log_scales.node.clone(),
                quats.node.clone(),
                sh_coeffs.node.clone(),
                raw_opacity.node.clone(),
            ])
            .compute_bound()
            .stateful();

        // Render complete forward pass.
        let (out_img, aux) = B::render_splats(
            camera,
            img_size,
            means.clone().into_primitive(),
            xy_dummy.into_primitive(),
            log_scales.clone().into_primitive(),
            quats.clone().into_primitive(),
            sh_coeffs.clone().into_primitive(),
            raw_opacity.clone().into_primitive(),
            render_u32_buffer,
        );

        let (send, rx) = tokio::sync::watch::channel(crate::BwdAux::default());

        let wrapped_aux = RenderAuxPrimitive::<Self> {
            projected_splats: <Self as AutodiffBackend>::from_inner(aux.projected_splats.clone()),
            radii: <Self as AutodiffBackend>::from_inner(aux.radii),
            num_intersections: aux.num_intersections.clone(),
            num_visible: aux.num_visible.clone(),
            final_index: aux.final_index.clone(),
            tile_offsets: aux.tile_offsets.clone(),
            compact_gid_from_isect: aux.compact_gid_from_isect.clone(),
            global_from_compact_gid: aux.global_from_compact_gid.clone(),
            uniforms_buffer: aux.uniforms_buffer.clone(),
            sender: Some(send),
        };

        match prep_nodes {
            OpsKind::Tracked(prep) => {
                // Save state needed for backward pass.
                let state = GaussianBackwardState {
                    means: means.into_primitive(),
                    log_scales: log_scales.into_primitive(),
                    quats: quats.into_primitive(),
                    raw_opac: raw_opacity.into_primitive(),
                    sh_degree: sh_degree_from_coeffs(
                        Tensor::<Self, 3>::from_primitive(TensorPrimitive::Float(sh_coeffs)).dims()
                            [1] as u32,
                    ),
                    out_img: out_img.clone(),
                    rx,
                    projected_splats: aux.projected_splats,
                    uniforms_buffer: aux.uniforms_buffer,
                    final_index: aux.final_index,
                    tile_offsets: aux.tile_offsets,
                    compact_gid_from_isect: aux.compact_gid_from_isect,
                    global_from_compact_gid: aux.global_from_compact_gid,
                };

                let finish = prep.finish(state, out_img);

                (finish, wrapped_aux)
            }
            OpsKind::UnTracked(prep) => {
                // When no node is tracked, we can just use the original operation without
                // keeping any state.
                (prep.finish(out_img), wrapped_aux)
            }
        }
    }
}

impl Backend for Fusion<BBase> {
    fn render_splats(
        cam: &Camera,
        img_size: glam::UVec2,
        means: FloatTensor<Self>,
        xy_grad_dummy: FloatTensor<Self>,
        log_scales: FloatTensor<Self>,
        quats: FloatTensor<Self>,
        sh_coeffs: FloatTensor<Self>,
        raw_opacity: FloatTensor<Self>,
        render_u32_buffer: bool,
    ) -> (FloatTensor<Self>, RenderAuxPrimitive<Self>) {
        struct CustomOp {
            cam: Camera,
            img_size: glam::UVec2,
            render_u32_buffer: bool,
            desc: CustomOpDescription,
        }

        impl Operation<FusionJitRuntime<WgpuRuntime, u32>> for CustomOp {
            fn execute(self: Box<Self>, h: &mut HandleContainer<JitFusionHandle<WgpuRuntime>>) {
                let (
                    [means, xy_dummy, log_scales, quats, sh_coeffs, raw_opacity],
                    [projected_splats, uniforms_buffer, num_intersections, num_visible, final_index, tile_offsets, compact_gid_from_isect, global_from_compact_gid, radii, out_img],
                ) = self.desc.consume();

                let (img, aux) = BBase::render_splats(
                    &self.cam,
                    self.img_size,
                    h.get_float_tensor::<BBase>(&means),
                    h.get_float_tensor::<BBase>(&xy_dummy),
                    h.get_float_tensor::<BBase>(&log_scales),
                    h.get_float_tensor::<BBase>(&quats),
                    h.get_float_tensor::<BBase>(&sh_coeffs),
                    h.get_float_tensor::<BBase>(&raw_opacity),
                    self.render_u32_buffer,
                );

                // Register output.
                h.register_float_tensor::<BBase>(&out_img.id, img);
                h.register_float_tensor::<BBase>(&projected_splats.id, aux.projected_splats);
                h.register_int_tensor::<BBase>(&uniforms_buffer.id, aux.uniforms_buffer);
                h.register_int_tensor::<BBase>(&num_intersections.id, aux.num_intersections);
                h.register_int_tensor::<BBase>(&num_visible.id, aux.num_visible);
                h.register_int_tensor::<BBase>(&final_index.id, aux.final_index);
                h.register_int_tensor::<BBase>(&tile_offsets.id, aux.tile_offsets);
                h.register_int_tensor::<BBase>(
                    &compact_gid_from_isect.id,
                    aux.compact_gid_from_isect,
                );
                h.register_int_tensor::<BBase>(
                    &global_from_compact_gid.id,
                    aux.global_from_compact_gid,
                );
                h.register_float_tensor::<BBase>(&radii.id, aux.radii);
            }
        }

        let stream = means.stream;
        let client = means.client.clone();

        let num_points = means.shape[0];

        let proj_size = size_of::<shaders::helpers::ProjectedSplat>() / 4;
        let uniforms_size = size_of::<shaders::helpers::RenderUniforms>() / 4;
        let tile_bounds = calc_tile_bounds(img_size);
        let max_intersects = max_intersections(img_size, num_points as u32);

        // If render_u32_buffer is true, we render a packed buffer of u32 values, otherwise
        // render RGBA f32 values.
        let channels = if render_u32_buffer { 1 } else { 4 };

        let out_img = client.tensor_uninitialized(
            vec![img_size.y as usize, img_size.x as usize, channels],
            DType::F32,
        );

        let aux = RenderAuxPrimitive::<Self> {
            projected_splats: client.tensor_uninitialized(vec![num_points, proj_size], DType::F32),
            uniforms_buffer: client.tensor_uninitialized(vec![uniforms_size], DType::I32),
            num_intersections: client.tensor_uninitialized(vec![1], DType::I32),
            num_visible: client.tensor_uninitialized(vec![1], DType::I32),
            final_index: client
                .tensor_uninitialized(vec![img_size.y as usize, img_size.x as usize], DType::I32),
            tile_offsets: client.tensor_uninitialized(
                vec![(tile_bounds.y * tile_bounds.x) as usize + 1],
                DType::I32,
            ),
            compact_gid_from_isect: client
                .tensor_uninitialized(vec![max_intersects as usize], DType::I32),
            global_from_compact_gid: client.tensor_uninitialized(vec![num_points], DType::I32),
            radii: client.tensor_uninitialized(vec![num_points], DType::F32),
            sender: None,
        };

        let desc = CustomOpDescription::new(
            "render_splats",
            &[
                means.into_description(),
                xy_grad_dummy.into_description(),
                log_scales.into_description(),
                quats.into_description(),
                sh_coeffs.into_description(),
                raw_opacity.into_description(),
            ],
            &[
                aux.projected_splats.to_description_out(),
                aux.uniforms_buffer.to_description_out(),
                aux.num_intersections.to_description_out(),
                aux.num_visible.to_description_out(),
                aux.final_index.to_description_out(),
                aux.tile_offsets.to_description_out(),
                aux.compact_gid_from_isect.to_description_out(),
                aux.global_from_compact_gid.to_description_out(),
                aux.radii.to_description_out(),
                out_img.to_description_out(),
            ],
        );

        let op = CustomOp {
            cam: cam.clone(),
            img_size,
            render_u32_buffer,
            desc: desc.clone(),
        };

        client.register(vec![stream], OperationDescription::Custom(desc), op);

        (out_img, aux)
    }

    fn render_splats_bwd(
        state: GaussianBackwardState<Self>,
        v_output: FloatTensor<Self>,
    ) -> SplatGrads<Self> {
        struct CustomOp {
            desc: CustomOpDescription,
            state: GaussianBackwardState<Fusion<BBase>>,
        }

        impl Operation<FusionJitRuntime<WgpuRuntime, u32>> for CustomOp {
            fn execute(self: Box<Self>, h: &mut HandleContainer<JitFusionHandle<WgpuRuntime>>) {
                let ([v_output], [v_means, v_quats, v_scales, v_coeffs, v_raw_opac, v_xy]) =
                    self.desc.consume();

                let state = self.state;

                let inner_state = GaussianBackwardState {
                    means: h.get_float_tensor::<BBase>(&state.means.into_description()),
                    log_scales: h.get_float_tensor::<BBase>(&state.log_scales.into_description()),
                    quats: h.get_float_tensor::<BBase>(&state.quats.into_description()),
                    raw_opac: h.get_float_tensor::<BBase>(&state.raw_opac.into_description()),
                    out_img: h.get_float_tensor::<BBase>(&state.out_img.into_description()),
                    projected_splats: h
                        .get_float_tensor::<BBase>(&state.projected_splats.into_description()),
                    uniforms_buffer: h
                        .get_int_tensor::<BBase>(&state.uniforms_buffer.into_description()),
                    final_index: h.get_int_tensor::<BBase>(&state.final_index.into_description()),
                    tile_offsets: h.get_int_tensor::<BBase>(&state.tile_offsets.into_description()),
                    compact_gid_from_isect: h
                        .get_int_tensor::<BBase>(&state.compact_gid_from_isect.into_description()),
                    global_from_compact_gid: h
                        .get_int_tensor::<BBase>(&state.global_from_compact_gid.into_description()),
                    sh_degree: state.sh_degree,
                    rx: state.rx,
                };

                let grads =
                    BBase::render_splats_bwd(inner_state, h.get_float_tensor::<BBase>(&v_output));

                // // Register output.
                h.register_float_tensor::<BBase>(&v_means.id, grads.v_means);
                h.register_float_tensor::<BBase>(&v_quats.id, grads.v_quats);
                h.register_float_tensor::<BBase>(&v_scales.id, grads.v_scales);
                h.register_float_tensor::<BBase>(&v_coeffs.id, grads.v_coeffs);
                h.register_float_tensor::<BBase>(&v_raw_opac.id, grads.v_raw_opac);
                h.register_float_tensor::<BBase>(&v_xy.id, grads.v_xy);
            }
        }

        let stream = v_output.stream;
        let client = v_output.client.clone();

        let num_points = state.means.shape[0];
        let num_visible = state.rx.borrow().data().num_visible;

        let coeffs = sh_coeffs_for_degree(state.sh_degree) as usize;

        let grads = SplatGrads::<Self> {
            v_means: client.tensor_uninitialized(vec![num_points, 3], DType::F32),
            v_quats: client.tensor_uninitialized(vec![num_points, 4], DType::F32),
            v_scales: client.tensor_uninitialized(vec![num_points, 3], DType::F32),
            v_coeffs: client.tensor_uninitialized(vec![num_points, coeffs, 3], DType::F32),
            v_raw_opac: client.tensor_uninitialized(vec![num_points], DType::F32),
            v_xy: client.tensor_uninitialized(vec![num_visible as usize, 2], DType::F32),
        };

        let desc = CustomOpDescription::new(
            "render_splat_bwd",
            &[v_output.into_description()],
            &[
                grads.v_means.to_description_out(),
                grads.v_quats.to_description_out(),
                grads.v_scales.to_description_out(),
                grads.v_coeffs.to_description_out(),
                grads.v_raw_opac.to_description_out(),
                grads.v_xy.to_description_out(),
            ],
        );

        let op = CustomOp {
            state,
            desc: desc.clone(),
        };

        client.register(vec![stream], OperationDescription::Custom(desc), op);
        grads
    }
}

impl<B: Backend, C: CheckpointStrategy> crate::AutodiffBackend for Autodiff<B, C> {}
