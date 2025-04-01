# 3.2 3D Gaussian Splat Rendering

This section explains the Gaussian Splatting rendering technique used in Brush, which is essential for both viewing and training.

## 3.2.1 Conceptual Overview

3D Gaussian Splatting, introduced in the original [INRIA paper](https://repo-sam.inria.fr/fungraph/3d-gaussian-splatting/), is a rasterization-based technique that renders scenes composed of potentially millions of 3D Gaussians. Each Gaussian has properties like position, shape (covariance represented by scale and rotation), color (using Spherical Harmonics), and opacity.

The general rendering pipeline involves:

1.  **Projection:** Transforming the 3D Gaussians into 2D representations ("splats") on the image plane based on the camera view.
2.  **Sorting:** Ordering the projected 2D splats by depth (typically front-to-back) to ensure correct alpha blending for occlusion.
3.  **Rasterization:** Iterating through screen pixels (often organized into tiles) and accumulating color and opacity contributions from all overlapping sorted splats.

Brush implements this pipeline leveraging the GPU via custom compute shaders written in **WGSL** (managed by `brush-wgsl` and `brush-kernel`). It utilizes the **Burn** framework to orchestrate GPU operations and relies on dedicated crates like **`brush-sort`** for efficient GPU-based radix sorting of the projected splats. This contrasts with implementations like [gsplat](https://github.com/nerfstudio-project/gsplat) which typically use custom CUDA kernels. Brush's use of WGSL aims for broader cross-platform compatibility (including WebGPU).

## 3.2.2 Rendering Pipeline (Forward Pass)

The forward rendering pass (`brush-render`) generates an image from a set of Gaussians and a camera view. It leverages GPU acceleration heavily.

*   **Input:** Camera parameters, Set of 3D Gaussians (`Splats` struct).
*   **GPU Kernels (`brush-kernel`, `brush-wgsl`):** Custom WGSL compute shaders execute the core rendering steps:
    *   `project_forward`/`project_visible`: Project 3D Gaussians to 2D, calculate view-dependent color, and determine potentially visible splats.
    *   `map_gaussian_to_intersects`: Identify which screen tiles each projected splat overlaps.
    *   `rasterize`: Accumulate color and opacity from sorted splats for each pixel within its tile.
*   **Sorting (`brush-sort`):** A GPU-based radix sort efficiently orders the projected Gaussians by depth before rasterization.
*   **Prefix Sums (`brush-prefix-sum`):** Provides GPU-accelerated scan operations used as a key component within the `brush-sort` radix sort algorithm.
*   **Blending:** Alpha compositing occurs within the `rasterize` kernel as it iterates through the sorted Gaussians front-to-back for each pixel.
*   **UI Display (`brush-ui`, `brush-app`):** The final rendered image is typically displayed within an EGUI panel (`ScenePanel`). User camera controls update the view parameters, triggering re-rendering.

In the Brush application UI, the `ScenePanel` includes a toggle ("Live update splats"). When enabled (default), the panel visually updates with the latest splat data received during training. Disabling this toggle prevents these visual updates in the `ScenePanel` (useful for performance on lower-end systems or complex scenes) but does not pause the underlying training computations. Key metrics derived from the rendering process, such as the current number of splats and the active Spherical Harmonic degree, can be monitored in the `Stats` panel.

## 3.2.3 Training/Optimization Pass (Backward Pass)

The forward rendering pass calculates the final image color based on the Gaussian parameters. To *train* these parameters, we need to compute how changes in each parameter affect the final rendered image and, consequently, the loss function (typically comparing the rendered image to a ground truth image). This is achieved through a **backward pass**, also known as backpropagation.

Brush implements this using Burn's automatic differentiation framework, integrated via the `brush-render-bwd` crate:

1.  **Custom Backward Step:** `brush-render-bwd` defines a custom backward operation (`RenderBackwards`) for the forward `render_splats` function. This tells Burn how to calculate gradients for the Gaussian parameters when gradients flow back from the loss function.
2.  **Forward Pass State Saving:** When the differentiable version of `render_splats` (from `SplatForwardDiff`) is called during training (likely by `SplatTrainer`), it performs the normal forward rendering. Crucially, if any input Gaussian parameters require gradients, it saves necessary intermediate results (like projected splats, tile information, visibility data, and the input parameters themselves) into a `GaussianBackwardState` struct. This state is registered with Burn's computation graph.
3.  **Gradient Backpropagation:** When the overall loss is calculated (e.g., L1 + SSIM loss between rendered and ground truth image) and `loss.backward()` is called in `SplatTrainer`:
    *   Burn propagates gradients back through the computation graph.
    *   When the gradient reaches the output of the `render_splats` operation, Burn invokes the custom `RenderBackwards::backward` method.
4.  **Backward Kernel Execution:** The `RenderBackwards::backward` method takes the saved `GaussianBackwardState` and the incoming gradient (representing how the loss changes with respect to each pixel's color) and calls the core backward logic (`render_backward` function, likely executing custom WGSL kernels defined in `brush-render-bwd/src/kernels.rs` and `shaders.rs`).
5.  **Parameter Gradient Calculation:** These backward kernels compute the gradients of the loss function with respect to each of the input Gaussian parameters (position, scale, rotation, opacity, SH coefficients).
6.  **Gradient Registration:** The computed parameter gradients (`SplatGrads`) are registered back into Burn's gradient tracking system.
7.  **Optimizer Step:** Finally, the optimizer (e.g., Adam, managed by `SplatTrainer`) uses these registered gradients to update the Gaussian parameter tensors, completing one training step.

In essence, `brush-render-bwd` provides the specific mathematical operations (implemented as GPU kernels) needed to reverse the rendering process for gradient calculation, plugging seamlessly into Burn's automatic differentiation machinery.

---

## Where to Go Next?

*   See how these rendering passes are used in training: **[Reconstruction Pipeline](reconstruction-pipeline.md)**.
*   Explore the GPU programming model: **[WGPU/WGSL in Core Technologies](core-technologies.md#345-wgpu--wgsl)**.
*   Look at the rendering code: **[API Reference](../api-reference.md)** (focus on `brush-render`, `brush-render-bwd`, `brush-kernel`).
*   See the overall structure: **[Architecture Overview](architecture.md)**. 