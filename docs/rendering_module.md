# Rendering Modules 🖼️

This document provides a detailed overview of Brush's rendering modules, which visualize 3D Gaussian splats in real-time and compute gradients for optimization.

## Overview 🔍

Brush separates rendering into two distinct modules:

1. **Forward Rendering Module (`brush-render`)** - Responsible for:
   - Representing Gaussian splats in memory
   - Projecting 3D Gaussians to 2D
   - Rendering the splats to images
   - Managing camera controls and view transformations

2. **Backward Rendering Module (`brush-render-bwd`)** - Responsible for:
   - Computing gradients for parameter optimization
   - Implementing backward (adjoint) operations
   - Supporting automatic differentiation
   - Enabling end-to-end training

Both modules leverage WebGPU through wgpu for efficient GPU-accelerated rendering across platforms.

## Architecture 🏗️

```
┌─────────────────────────────────────────────────────────────────┐
│                    Forward Rendering Module                      │
│                                                                  │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌────────────┐  │
│  │            │  │            │  │            │  │            │  │
│  │  Gaussian  │  │ Projection │  │  Sorting   │  │Rasterization│  │
│  │Representation│ │  Pipeline  │  │  System   │  │   Engine    │  │
│  │            │  │            │  │            │  │            │  │
│  └────────────┘  └────────────┘  └────────────┘  └────────────┘  │
│         │              │               │                │        │
│         ▼              ▼               ▼                ▼        │
│  ┌─────────────────────────────────────────────────────────────┐ │
│  │                        wgpu Glue                            │ │
│  └─────────────────────────────────────────────────────────────┘ │
└──────────────────────────────┬────────────────────────────────────┘
                               │
                               ▼
                      ┌───────────────┐
                      │   WebGPU API  │
                      │   (wgpu)      │
                      └───────┬───────┘
                              │
┌─────────────────────────────┴────────────────────────────────────┐
│                    Backward Rendering Module                      │
│                                                                   │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌────────────┐   │
│  │            │  │            │  │            │  │            │   │
│  │  Gradient  │  │  Backward  │  │  Gradient  │  │Differentiable│  │
│  │ Computation│  │ Projection │  │  Gathering │  │Rasterization │  │
│  │            │  │            │  │            │  │            │   │
│  └────────────┘  └────────────┘  └────────────┘  └────────────┘   │
│         │              │               │                │         │
│         ▼              ▼               ▼                ▼         │
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │                        Burn Glue                            │  │
│  └─────────────────────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────────────────┘
```

## Core Components 🧩

### Forward Rendering Components

1. **Gaussian Representation**
   - Positions (xyz)
   - Scales (xyz)
   - Rotations (quaternions)
   - Colors (RGB or spherical harmonics)
   - Opacity values

2. **Projection Pipeline**
   - View transformation
   - Covariance matrix calculation
   - Perspective projection

3. **Sorting System**
   - CPU sorting for small datasets
   - GPU radix sort for large datasets

4. **Rasterization Engine**
   - Tile-based rasterization
   - EWA filtering
   - Alpha blending

### Backward Rendering Components

1. **Gradient Computation**
   - Parameter gradients calculation
   - Chain rule application

2. **Backward Projection**
   - Adjoint operations for projection
   - Covariance matrix differentiation

3. **Gradient Gathering**
   - Accumulating gradients across pixels
   - Reducing to parameter updates

4. **Differentiable Rasterization**
   - Backpropagation through the rendering process
   - Handling of partial derivatives

## Core Files 📁

### Forward Rendering (`brush-render`)

- **lib.rs**: Public API and main exports
- **render.rs**: Main rendering pipeline implementation
- **gaussian_splats.rs**: Gaussian representation and operations
- **camera.rs**: Camera handling and view transformations
- **sh.rs**: Spherical harmonics implementation
- **kernels.rs**: GPU kernel management
- **shaders/**: WGSL shader files for forward rendering

### Backward Rendering (`brush-render-bwd`)

- **lib.rs**: Public API for backward rendering
- **burn_glue.rs**: Integration with the Burn framework
- **kernels.rs**: GPU kernel management for gradients
- **shaders/**: WGSL shader files for backward operations
  - **gather_grads.wgsl**: Gradient gathering
  - **project_backwards.wgsl**: Backward projection
  - **rasterize_backwards.wgsl**: Backward rasterization

## Rendering Pipelines 🔄

### Forward Rendering Process

1. **Frustum Culling**: Eliminate Gaussians outside the view
2. **Gaussian Projection**: Transform 3D Gaussians to 2D
3. **Depth Sorting**: Sort by depth for correct blending
4. **Tile-based Rasterization**: Efficiently render splats
5. **Alpha Blending**: Combine overlapping Gaussians

### Backward Rendering Process

1. **Loss Computation**: Calculate difference from target image
2. **Gradient Initialization**: Set up gradient buffers
3. **Backward Rasterization**: Compute pixel-wise gradients
4. **Gradient Gathering**: Accumulate per-Gaussian gradients
5. **Backward Projection**: Propagate to 3D parameters

## Integration with Training 🧠

The rendering modules connect with the training process:

- **Forward Path**: 
  - `brush-render` produces images for comparison with ground truth
  - Used for both training and visualization

- **Backward Path**:
  - `brush-render-bwd` computes gradients for optimization
  - Enables end-to-end differentiable rendering
  - Interfaces with Burn's autodifferentiation system

## Next Steps 🔍

- Learn about the [Training Module](training_module.md) for optimization details
- Explore [Cross-Platform Framework](cross_platform_framework.md) details
- Understand [Performance Optimization](performance_optimization.md) techniques 