# Core Concepts 🧠

This document explains the fundamental concepts that underpin the Brush project, focusing on 3D Gaussian splatting and related principles.

## Gaussian Splatting Overview 🔍

**3D Gaussian Splatting** is a powerful technique for reconstructing 3D scenes from 2D images. It represents the scene as a collection of 3D Gaussian primitives (also called "splats") that efficiently model color, density, and spatial distribution.

Key advantages of this approach include:

- **Real-time rendering** of complex 3D scenes
- **Compact representation** compared to traditional point clouds or meshes
- **High visual quality** with smooth surfaces and accurate details
- **Cross-platform compatibility** due to efficient rendering requirements

```
     ┌───────────────┐           ┌────────────────┐           ┌────────────────┐
     │ Input Images  │           │ 3D Gaussians   │           │ Novel Views    │
     │ with Poses    │ ──────▶   │ (Splats)       │ ──────▶   │ Rendering      │
     └───────────────┘           └────────────────┘           └────────────────┘
              │                          ▲                            │
              │                          │                            │
              └──────────────────────────┴────────────────────────────┘
                            Optimization Loop
```

## Core Components 🧩

### 1. Gaussian Primitives

Each Gaussian primitive ("splat") is defined by several parameters:

- **Position** (x, y, z): The 3D center of the Gaussian
- **Scale** (sx, sy, sz): The size of the Gaussian along each axis
- **Rotation** (quaternion): The orientation of the Gaussian
- **Opacity** (α): The transparency/visibility of the Gaussian
- **Color** (RGB or Spherical Harmonics): The color information

### 2. Training Process

Training in Brush involves optimizing these Gaussian parameters to match the input images:

1. **Initialization**: Create initial Gaussians in the scene
2. **Forward Rendering**: Project Gaussians to create synthetic views
3. **Loss Calculation**: Compare with ground truth images
4. **Backpropagation**: Update Gaussian parameters using gradients
5. **Regularization**: Apply constraints to ensure realistic results
6. **Density Control**: Adaptively add or remove Gaussians as needed

### 3. Rendering Pipeline

The rendering pipeline in Brush consists of these key steps:

1. **Culling**: Remove Gaussians outside the view frustum
2. **Sorting**: Order Gaussians by depth from the camera
3. **Rasterization**: Project 3D Gaussians to 2D screen space
4. **Blending**: Combine overlapping Gaussians using alpha blending
5. **Shading**: Apply lighting and color information

```
                  ┌────────┐    ┌────────┐    ┌─────────────┐    ┌──────────┐    ┌─────────┐
                  │ Culling │ ─▶ │ Sorting │ ─▶ │Rasterization│ ─▶ │ Blending │ ─▶ │ Shading │
                  └────────┘    └────────┘    └─────────────┘    └──────────┘    └─────────┘
```

## Mathematical Foundation 📐

### Gaussian Function

A 3D Gaussian is defined by the following function:

```
G(x) = exp(-0.5 * (x - μ)ᵀ Σ⁻¹ (x - μ))
```

Where:
- **x** is a 3D point
- **μ** is the mean (position)
- **Σ** is the covariance matrix (determined by scale and rotation)

### Spherical Harmonics

Brush uses spherical harmonics (SH) to represent view-dependent appearance:

- **SH coefficients** store how a Gaussian's color changes with viewing direction
- **Higher-order SH** enables more complex view-dependent effects
- **Basic RGB color** is the 0th order spherical harmonic

## Burn Framework Integration 🔥

Brush uses the [Burn machine learning framework](https://github.com/tracel-ai/burn) for:

- **Autodifferentiation**: Enables efficient gradient computation for optimization
- **GPU Acceleration**: Leverages WebGPU for cross-platform graphics processing
- **Tensor Operations**: Provides efficient numerical computations

## Cross-Platform Architecture 🌐

Brush achieves cross-platform capabilities through:

- **WebGPU Backend**: A modern, portable graphics API supported across devices
- **WASM Compilation**: For web browser support
- **Native Builds**: For desktop platforms
- **Mobile Support**: For Android and iOS

## Data Formats 📊

Brush works with these primary data formats:

1. **Input Data**:
   - COLMAP sparse reconstructions
   - Nerfstudio transforms.json format
   - Image datasets with camera poses

2. **Output Data**:
   - PLY files containing 3D Gaussians
   - Animation data for dynamic scenes
   - Visualization data for Rerun

## Next Steps 🔍

- Dive deeper into the [Architecture](architecture.md)
- Learn about the [Code Structure](code_structure.md)
- Explore the [Training Module](training_module.md) and [Rendering Module](rendering_module.md) 