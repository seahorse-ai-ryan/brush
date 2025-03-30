# 3.3. 3D Gaussian Splat Rendering

This section explains the Gaussian Splatting rendering technique used in Brush, which is essential for both viewing and training.

## 3.3.1. Conceptual Overview

3D Gaussian Splatting is a rasterization-based rendering technique. Instead of triangles, it renders a scene composed of potentially millions of 3D Gaussians. Each Gaussian has properties like:

*   **Position (xyz):** Center of the Gaussian.
*   **Covariance (3x3 matrix):** Defines the shape and orientation (ellipsoid). Often represented by scale and rotation (quaternion).
*   **Color (RGB):** Stored typically as Spherical Harmonics (SH) coefficients to represent view-dependent color.
*   **Opacity (alpha):** Controls transparency.

The rendering process involves:

1.  **Projection:** Projecting the 3D Gaussians onto the 2D image plane.
2.  **Sorting:** Sorting the projected Gaussians based on depth (typically front-to-back) to handle occlusion correctly during blending.
3.  **Rasterization/Splatting:** For each pixel, accumulate the color and opacity contribution from overlapping sorted 2D Gaussians ("splats").

This method allows for high-quality, real-time rendering of complex scenes learned from images.

Reference: [3D Gaussian Splatting for Real-Time Radiance Field Rendering](https://repo-sam.inria.fr/fungraph/3d-gaussian-splatting/)

## 3.3.2. Rendering Pipeline (Forward Pass)

The forward rendering pass (`brush-render`) generates an image from a set of Gaussians and a camera view. It leverages GPU acceleration heavily.

*   **Input:** Camera parameters, Set of 3D Gaussians.
*   **GPU Kernels (`brush-kernel`, `brush-wgsl`):** Custom compute shaders executed on the GPU likely handle:
    *   Projecting 3D Gaussians to 2D.
    *   Calculating view-dependent color from SH coefficients.
    *   Rasterizing the 2D splats onto a pixel grid.
*   **Sorting (`brush-sort`):** A GPU-based radix sort is used to efficiently sort the projected Gaussians by depth.
*   **Prefix Sums (`brush-prefix-sum`):** May be used as part of the sorting algorithm or other parallel primitives within the rendering pipeline.
*   **Blending:** Alpha compositing is performed (likely on the GPU) by iterating through the sorted Gaussians front-to-back for each pixel.
*   **UI Interaction (`brush-ui`/EGUI):** The final rendered image is displayed within the EGUI interface. User interactions (camera movement) update the camera parameters, triggering re-rendering.

## 3.3.3. Training/Optimization Pass (Backward Pass)

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