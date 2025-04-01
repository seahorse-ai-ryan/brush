# 3.6 Extending Brush

Brush is designed with modularity in mind, allowing developers to extend its core functionality or leverage its components to build new applications and workflows.

## 3.6.1 Contributing to Core Brush

Contributing improvements or new features directly to the Brush open-source project is highly encouraged.

**Potential Areas for Contribution:**

*   **Reconstruction Algorithms:**
    - Implement new optimization strategies in `brush-train`
    - Add support for different camera models in `brush-dataset`
    - Enhance Gaussian refinement in `brush-process`

*   **Rendering Optimizations:**
    - Improve GPU kernels in `brush-sort` and `brush-render`
    - Optimize memory usage in `brush-render-bwd`
    - Add new rendering features to `brush-kernel`

*   **UI/UX Improvements:**
    - Enhance training controls in `brush-ui`
    - Add new visualization tools
    - Improve platform-specific UI adaptations

**Key Considerations:**

*   **Architecture:** Familiarize yourself with the **[Architecture Overview](architecture.md)**
*   **Core Technologies:** Understand how **[Burn](core-technologies.md#343-burn)**, **[wgpu](core-technologies.md#345-wgpu--wgsl)**, and **[egui](core-technologies.md#344-egui--eframe)** are used
*   **Contribution Guidelines:** Follow the process in **[Contributing Guide](../../CONTRIBUTING.md)**
*   **API Stability:** Consider impact on existing APIs

## 3.6.2 Building Custom Applications

The modular nature of Brush allows its crates to be used as libraries in other Rust projects.

**Example Usage:**

```rust
use brush_render::render;
use brush_dataset::{LoadDataseConfig, ModelConfig};
use burn::backend::WgpuDevice;

// Configure dataset loading
let load_config = LoadDataseConfig {
    max_frames: None,
    max_resolution: 1920,
    eval_split_every: None,
    subsample_frames: None,
    subsample_points: None,
};

// Configure model parameters
let model_config = ModelConfig {
    sh_degree: 3,
};

// Set up GPU device
let device = WgpuDevice::default();
let backend = WgpuRuntime::new(device);

// Render configuration
let render_config = RenderConfig {
    tile_size: 16,
    max_splats: 10_000_000,
    ..Default::default()
};

// Use Brush components
let (out_img, aux) = render(
    means,
    log_scales,
    quats,
    sh_coeffs,
    opacities,
    camera,
    img_size,
    bwd_info,
);
```

**Common Use Cases:**

*   **Custom Viewers:**
    - Create specialized visualization tools
    - Integrate with other 3D applications
    - Build platform-specific viewers

*   **Domain-Specific Tools:**
    - Cultural heritage digitization
    - Robotics and SLAM
    - VFX and animation

## 3.6.3 Automation and Services

Brush provides CLI tools for automation and service integration.

**Example Scripts:**

```bash
# Batch processing example
for dataset in /path/to/datasets/*; do
  brush_app --dataset "$dataset" \
           --output "$dataset/output.ply" \
           --total-steps 30000 \
           --save-final
done

# Service integration example
brush_app --dataset /path/to/dataset \
         --output /path/to/output \
         --train \
         --no-ui
```

**Key CLI Options:**
*   `--dataset`: Input dataset path
*   `--output`: Output file path
*   `--total-steps`: Training iterations (default: 30000)
*   `--train`: Run training
*   `--no-ui`: Run without GUI
*   `--max-resolution`: Maximum image resolution (default: 1920)
*   `--sh-degree`: Spherical harmonics degree (default: 3)

**Performance Considerations:**
*   GPU memory requirements: ~8GB VRAM
*   Training speed: 10-20 iterations/second
*   Rendering target: 60+ FPS
*   Memory bandwidth: 20GB/s+ for GPU operations

---

## Where to Go Next?

*   Dive into the code structure: **[Architecture Overview](architecture.md)**.
*   See the detailed API structure: **[API Reference](../api-reference.md)**.
*   Find contribution rules: **[CONTRIBUTING](../../CONTRIBUTING.md)**.
*   Ready to set up your dev environment? **[Developer Guide](../getting-started/developer-guide.md)**. 