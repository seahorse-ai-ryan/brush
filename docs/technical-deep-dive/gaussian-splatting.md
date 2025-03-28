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

For training, the rendering pipeline needs to be differentiable. The backward pass (`brush-render-bwd`) calculates the gradients of the loss (difference between rendered and ground truth image) with respect to the Gaussian parameters.

*   This involves propagating gradients back through the blending, rasterization, and projection steps.
*   Custom backward kernels are required for the GPU operations.
*   The [Burn](core-technologies.md#343-burn) framework manages the automatic differentiation process, coordinating the forward and backward passes and parameter updates.

*(TODO: Add more specific implementation details based on code analysis of `brush-render-bwd` and its interaction with Burn and `brush-kernel`.)* 