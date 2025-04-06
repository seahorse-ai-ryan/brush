# Training and Rendering Pipeline

This document describes the core loop for training Gaussian Splatting models in Brush (`crates/brush-train`) and the associated differentiable rendering process (`crates/brush-render`, `crates/brush-render-bwd`).

## Overview

Training aims to optimize the parameters of a set of 3D Gaussians (`Splats`) – specifically their position (`means`), rotation (`rotation`), scale (`log_scales`), opacity (`raw_opacity`), and color (Spherical Harmonics coefficients, `sh_coeffs`) – to best reconstruct a scene captured by input camera views.

This involves repeatedly rendering the current Gaussians from different camera viewpoints and adjusting their parameters using gradient descent to minimize the difference (loss) between the rendered image and the ground truth input image.

## Core Training Loop (`brush-train::train::SplatTrainer`)

The main training logic resides in `SplatTrainer::step`. This function, typically called repeatedly by `brush-process`, performs the following steps for each training iteration (`--total-steps`):

1.  **Batch Selection:** Receives a batch (`SceneBatch`) containing a camera view (`Camera`) and its corresponding ground truth image (`img_tensor`) from `brush-process`.
2.  **Forward Pass (Rendering):** Renders the current `Splats` from the batch's camera viewpoint. This crucial step, detailed further below, uses `brush-render` and returns the predicted image as well as auxiliary information needed for the backward pass and density control.
3.  **Loss Calculation:** Compares the `pred_image` to the `gt_rgb` from the batch:
    *   **L1 Loss:** Calculates `(pred_rgb - gt_rgb).abs()`.
    *   **SSIM Loss:** Calculates the Structural Similarity Index using the `ssim` module, weighted by `--ssim-weight`. SSIM captures perceptual similarity better than pure L1.
    *   **Combined RGB Loss:** The L1 and SSIM losses are combined: `L1 * (1 - ssim_weight) + SSIM * ssim_weight`.
    *   **Alpha Loss:** If the input image has an alpha channel (`batch.has_alpha()`), an L1 loss on alpha `(pred_alpha - gt_alpha).abs()` is added, weighted by `--match-alpha-weight`. (Note: Masked alpha is handled differently).
    *   **Opacity Regularization:** A small loss term `opacity * visibility * opac_loss_weight * (1 - train_progress)` is added to encourage invisible or unnecessary splats to fade away over time.
    *   **Total Loss:** The sum of the combined RGB loss, alpha loss (if applicable), and opacity regularization.
4.  **Backward Pass (Gradients):** Calculates `loss.backward()`. This triggers:
    *   **Custom Gradients (`brush-render-bwd`):** Specialized WGSL kernels calculate the gradients of the loss with respect to the *outputs* of the forward rendering pass (e.g., pixel colors).
    *   **Burn Autodiff:** Burn propagates these initial gradients backward through the rendering operations (tracked via `SplatForwardDiff`) and the loss calculations to compute the final gradients for each trainable parameter in the `Splats` struct (`means.grad()`, `log_scales.grad()`, etc.).
5.  **Optimizer Step (`AdamScaled`):** Updates the trainable parameters using the computed gradients and scheduled learning rates.
    *   Applies different base learning rates (`--lr-mean`, `--lr-coeffs-dc`, etc.) for different parameter groups.
    *   Applies exponential decay to mean and scale learning rates (`--lr-mean-end`, `--lr-scale-end`).
    *   Scales SH coefficient learning rates based on order (`--lr-coeffs-sh-scale`).
    *   Uses the custom `AdamScaled` optimizer, potentially applying per-parameter scaling (e.g., for SH coefficients).
6.  **Density Control (`refine_if_needed`):** Periodically (every `--refine-every` steps, until `--growth-stop-iter`), adjusts the number of Gaussians based on statistics gathered during rendering (primarily positional gradients captured in `refine_weight_holder`):
    *   **Pruning:** Removes Gaussians where opacity (`sigmoid(raw_opacity)`) drops below `MIN_OPACITY` (approx 0.0035).
    *   **Densification (Cloning & Offsetting):** Identifies Gaussians with large view-space positional gradients (norm > `--growth-grad-threshold`). A fraction (`--growth-select-fraction`) of these are selected. Each selected Gaussian is effectively **cloned**: the original is shrunk (scale divided by ~1.4, opacity adjusted) and slightly offset, while a new Gaussian is created with the shrunk scale and offset in the opposite direction. New Gaussians receive **zeroed** optimizer states. <!-- Resolved: Cloning with offset; new splats get zeroed optimizer state -->
    *   **Resampling:** If pruning removed Gaussians, new ones are created by sampling existing high-opacity Gaussians to replace the pruned ones, helping fill gaps.
7.  **Logging/Updates:** Returns `TrainStepStats` and the updated `Splats` to `brush-process`, which then sends a `ProcessMessage` to the UI.

## Rendering Pipeline Details (`brush-render`)

The forward rendering pass (`SplatForwardDiff::render_splats` / `render_forward` in `render.rs`) implemented in `brush-render` involves several GPU-accelerated stages:

1.  **Input:** `Splats` data (means, scales, rotations, colors, opacities) and a `Camera`.
2.  **Projection & Culling (`ProjectSplats` Kernel):** The initial compute shader transforms 3D means to view space, calculates depths, and determines which splats are potentially visible (e.g., not behind the camera, project within reasonable bounds). Only indices and depths of these potentially visible splats are passed on. This acts as the primary culling step.
3.  **Depth Sorting (`brush-sort`):** Sorts the potentially visible splats based on depth using a GPU radix sort. This returns `global_from_compact_gid`, an index mapping sorted compact IDs back to original global splat IDs.
4.  **Visibility/Intersection Calculation (`ProjectVisible` Kernel):** Using the sorted visible splats (`global_from_compact_gid`), this kernel calculates the 2D projected covariance, color, and determines which screen tiles each splat intersects, writing out intersection information (`isect_info`, `tiles_hit_per_splat`).
5.  **Intersection Sorting & Indexing (`MapGaussiansToIntersect`, `radix_argsort`, `prefix_sum`):** Further GPU passes sort the intersection information by tile ID and calculate offsets (`tile_offsets`) to know which range of intersections belongs to each tile.
6.  **Rasterization (`Rasterize` Kernel):** Blends the splats onto the image grid.
    *   Uses the `tile_offsets` to process intersections tile by tile.
    *   For each pixel within a tile, iterates through relevant Gaussians (using `compact_gid_from_isect`).
    *   Calculates the Gaussian's contribution (`alpha`) at the pixel center.
    *   Blends colors back-to-front: `C_out = C_in * (1 - alpha) + Color_splat * alpha`.
    *   Performed in WGSL compute shaders via `brush-kernel`.

## Backward Pass Details (`brush-render-bwd`)

`brush-render-bwd` provides the crucial custom gradients for the rendering steps, allowing Burn's autodiff to function:

*   It takes the gradient signal (dL/dColor) from the loss function.
*   Custom WGSL kernels compute how changes in the rendering inputs (splat parameters projected to 2D) affect the output pixel colors.
*   These kernels effectively reverse the rasterization and projection steps to calculate `dL/dMeans2D`, `dL/dCovariance2D`, `dL/dColor`, `dL/dOpacity`.
*   Burn then propagates these gradients further back to the original 3D splat parameters.

**See Also:** [Core Technologies](./core-technologies.md), [Burn Autodiff](https://burn-rs.github.io/book/autodiff/introduction.html), `crates/brush-train/src/train.rs`, `crates/brush-render/src/gaussian_splats.rs`, `crates/brush-render-bwd/`

## Next Steps

*   Understand the [Project Architecture](./architecture.md) for crate relationships.
*   Review the specific [Configuration Options](../reference/config-options.md) controlling this pipeline.
*   Explore [Data Handling](./data-handling.md) to see where input data comes from. 