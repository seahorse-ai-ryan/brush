use brush_render::gaussian_splats::Splats;
use brush_render::RenderAux;
use burn::backend::{Autodiff, Wgpu};
use burn::prelude::*;
use burn::tensor::{Tensor, TensorMetadata, TensorPrimitive};
use tracing::trace_span;

type B = Autodiff<Wgpu>;

pub(crate) struct RefineRecord<B: Backend> {
    // Helper tensors for accumulating the viewspace_xy gradients and the number
    // of observations per gaussian. Used in pruning and densification.
    grad_2d_accum: Tensor<B, 1>,
    xy_grad_counts: Tensor<B, 1, Int>,
    max_radii: Tensor<B, 1>,
}

impl RefineRecord<B> {
    pub(crate) fn new(num_points: usize, device: &<B as Backend>::Device) -> RefineRecord<B> {
        Self {
            grad_2d_accum: Tensor::zeros([num_points], device),
            xy_grad_counts: Tensor::zeros([num_points], device),
            max_radii: Tensor::zeros([num_points], device),
        }
    }

    pub(crate) fn gather_stats(
        &mut self,
        splats: &Splats<B>,
        xys_grad: Tensor<B, 2>,
        aux: RenderAux<B>,
        device: &<B as Backend>::Device,
    ) {
        let _span = trace_span!("Gather stats", sync_burn = true);

        let gs_ids = Tensor::from_primitive(aux.global_from_compact_gid);
        let radii = Tensor::from_primitive(TensorPrimitive::Float(aux.radii));

        let [h, w] = aux.final_index.shape().dims();

        let scale =
            Tensor::<B, 1>::from_floats([w as f32 / 2.0, h as f32 / 2.0], device).reshape([1, 2]);
        let xys_grad = xys_grad * scale;
        let xys_grad_norm = xys_grad.powi_scalar(2).sum_dim(1).squeeze(1).sqrt();

        let num_vis = Tensor::from_primitive(aux.num_visible.clone());
        let valid = Tensor::arange(0..splats.num_splats() as i64, device).lower(num_vis);

        self.grad_2d_accum =
            self.grad_2d_accum
                .clone()
                .select_assign(0, gs_ids.clone(), xys_grad_norm);

        self.xy_grad_counts =
            self.xy_grad_counts
                .clone()
                .select_assign(0, gs_ids.clone(), valid.int());

        let radii_norm = radii / (w.max(h) as f32);
        self.max_radii = self.max_radii.clone().max_pair(radii_norm);
    }

    pub(crate) fn average_grad_2d(&self) -> Tensor<B, 1> {
        self.grad_2d_accum.clone() / self.xy_grad_counts.clone().clamp_min(1).float()
    }

    pub(crate) fn max_radii(&self) -> Tensor<B, 1> {
        self.max_radii.clone()
    }
}
