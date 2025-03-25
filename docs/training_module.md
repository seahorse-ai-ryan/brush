# Training Module 🧠

This document provides a comprehensive overview of Brush's training module, which optimizes 3D Gaussian splats to match input images.

## Overview 🔍

The training module (`brush-train`) is responsible for:

1. Optimizing Gaussian parameters to match input images
2. Managing training progress and statistics
3. Evaluating reconstruction quality
4. Providing visualizations during training

The module leverages the Burn framework for automatic differentiation and GPU acceleration, enabling efficient optimization of Gaussian parameters.

## Architecture 🏗️

```
┌─────────────────────────────────────────────────────────────────┐
│                      Training Module                             │
│                                                                  │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌────────────┐  │
│  │            │  │            │  │            │  │            │  │
│  │ Parameter  │  │ Optimizer  │  │   Loss     │  │ Evaluation │  │
│  │ Management │  │   Engine   │  │ Computation│  │   Metrics  │  │
│  │            │  │            │  │            │  │            │  │
│  └────────────┘  └────────────┘  └────────────┘  └────────────┘  │
│         │              │               │                │        │
│         ▼              ▼               ▼                ▼        │
│  ┌─────────────────────────────────────────────────────────────┐ │
│  │                         Burn Glue                           │ │
│  └─────────────────────────────────────────────────────────────┘ │
│                              │                                    │
└──────────────────────────────┼────────────────────────────────────┘
                               │
        ┌─────────────────────┼──────────────────────┐
        │                      │                      │
        ▼                      ▼                      ▼
┌────────────────┐   ┌───────────────────┐   ┌───────────────────┐
│  brush-dataset │   │  Burn Framework   │   │ brush-render-bwd  │
│  (Scene Data)  │   │   (Autodiff +     │   │(Backward Rendering│
│                │   │   WebGPU Backend) │   │ & Gradients)      │
└────────────────┘   └───────────────────┘   └───────────────────┘
```

### Key Components

1. **Parameter Management**: Handles Gaussian creation, removal, and updating
2. **Optimizer Engine**: Implements optimization algorithms like Adam
3. **Loss Computation**: Calculates differences between rendered and ground truth images
4. **Evaluation Metrics**: Computes metrics like PSNR and SSIM
5. **Burn Glue**: Connects with the Burn framework for autodifferentiation
6. **External Dependencies**:
   - **brush-dataset**: Provides scene representation
   - **brush-render-bwd**: Implements backward rendering for gradients

## Core Files 📁

- **train.rs**: Main training loop and orchestration
- **adam_scaled.rs**: Custom Adam optimizer implementation
- **eval.rs**: Evaluation metrics implementation
- **ssim.rs**: Structural Similarity Index (SSIM) calculation
- **stats.rs**: Training statistics tracking
- **quat_vec.rs**: Quaternion vector operations for rotations

## Training Process 🔄

The training process follows these steps:

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│  Initialize  │     │  Render      │     │  Compute     │
│  Gaussians   │────►│  Current     │────►│  Loss        │
└──────────────┘     │  View        │     │              │
                     └──────────────┘     └──────────────┘
                                                 │
                                                 ▼
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│  Update      │     │  Compute     │     │  Backward    │
│  Parameters  │◄────│  Gradients   │◄────│  Pass        │
└──────────────┘     └──────────────┘     └──────────────┘
        │
        ▼
┌──────────────┐     ┌──────────────┐
│  Density     │     │  Track       │
│  Control     │────►│  Statistics  │
└──────────────┘     └──────────────┘
```

### 1. Initialization

The training starts by initializing Gaussians:

```rust
/// Create initial Gaussians from input point cloud or random positions
pub fn initialize_scene(dataset: &Dataset) -> Scene {
    // Use scene creation functionality from brush-dataset
    brush_dataset::scene::create_initial_scene(dataset)
}
```

### 2. Forward Pass

The forward pass renders the scene from training viewpoints:

```rust
/// Render the scene from a camera viewpoint
pub fn render_view(
    scene: &Scene, 
    camera: &Camera,
    device: &Device,
) -> RenderOutput {
    // Use forward rendering from brush-render
    brush_render::render::render_scene(scene, camera, device)
}
```

### 3. Loss Computation

The loss measures the difference between rendered and ground truth images:

```rust
/// Compute the loss between rendered and ground truth images
pub fn compute_loss(
    rendered: &Image,
    ground_truth: &Image,
) -> (Tensor, f32) {
    // Compute L2 loss between images
    let l2_loss = mse_loss(rendered.tensor(), ground_truth.tensor());
    
    // Add regularization if needed
    let total_loss = l2_loss + REGULARIZATION_WEIGHT * regularization_term();
    
    (total_loss, l2_loss.item())
}
```

### 4. Backward Pass and Optimization

The backward pass computes gradients and updates parameters:

```rust
/// Update Gaussian parameters based on gradients
pub fn update_gaussians(
    optimizer: &mut AdamScaled,
    params: &mut GaussianParameters,
    loss: Tensor,
) {
    // Compute gradients using brush-render-bwd
    let grads = brush_render_bwd::backward::compute_gradients(loss);
    
    // Apply gradients to update parameters
    optimizer.step(params, &grads);
    
    // Apply constraints (e.g., positive scale)
    params.apply_constraints();
}
```

### 5. Adaptive Density Control

Density control adaptively adds or removes Gaussians:

```rust
/// Adaptively control the number of Gaussians
pub fn density_control(
    scene: &mut Scene,
    metrics: &TrainingMetrics,
    iteration: usize,
) {
    // Every N iterations, perform density control
    if iteration % DENSITY_CONTROL_INTERVAL == 0 {
        // Remove low opacity Gaussians
        scene.prune_transparent_gaussians(OPACITY_THRESHOLD);
        
        // Split high-importance Gaussians
        let candidates = find_split_candidates(scene, metrics);
        for candidate in candidates {
            scene.split_gaussian(candidate);
        }
    }
}
```

### 6. Statistics Tracking

Training metrics are tracked throughout the process:

```rust
/// Update and log training statistics
pub fn update_statistics(
    stats: &mut TrainingStats,
    loss: f32,
    psnr: f32,
    iteration: usize,
) {
    stats.update(loss, psnr, iteration);
    
    // Log to console periodically
    if iteration % LOG_INTERVAL == 0 {
        println!("Iteration {}: Loss={:.6}, PSNR={:.2}dB", 
                 iteration, loss, psnr);
    }
    
    // Log to Rerun for visualization
    if let Some(rec) = stats.recording() {
        rec.log("training/loss", iteration, loss);
        rec.log("training/psnr", iteration, psnr);
    }
}
```

## Gaussian Parameters 📊

Gaussian splats are parameterized by:

| Parameter | Description | Size | Optimization Strategy |
|-----------|-------------|------|----------------------|
| Position | 3D center point | 3 | Direct update |
| Scale | Size along each axis | 3 | Log-space update |
| Rotation | Quaternion orientation | 4 | Special quaternion update |
| Opacity | Transparency | 1 | Sigmoid-space update |
| Color | RGB or SH coefficients | 3 or 3×(SH_DEGREE+1)² | Direct update |

### Parameter Representation

Parameters are represented as tensors:

```rust
/// Gaussian parameters for optimization
pub struct GaussianParameters {
    /// Positions (N, 3)
    pub positions: Tensor,
    
    /// Log-scales (N, 3) - stored in log space for unconstrained optimization
    pub log_scales: Tensor,
    
    /// Rotations as quaternions (N, 4)
    pub rotations: Tensor,
    
    /// Opacity logits (N, 1) - stored as logits for unconstrained optimization
    pub opacity_logits: Tensor,
    
    /// Colors or SH coefficients (N, C) where C depends on SH degree
    pub colors: Tensor,
}
```

## Optimization Algorithms 🔧

### Adam Optimizer

The primary optimizer is a scaled version of Adam:

```rust
/// Scaled Adam optimizer for training
pub struct AdamScaled {
    /// Learning rates for each parameter type
    pub learning_rates: LearningRates,
    
    /// Beta1 parameter for momentum
    pub beta1: f32,
    
    /// Beta2 parameter for variance
    pub beta2: f32,
    
    /// Epsilon for numerical stability
    pub epsilon: f32,
    
    /// First moment estimates
    m: HashMap<String, Tensor>,
    
    /// Second moment estimates
    v: HashMap<String, Tensor>,
    
    /// Current iteration
    t: usize,
}
```

The optimizer uses different learning rates for different parameter types:

```rust
/// Learning rates for different parameter types
pub struct LearningRates {
    /// Position learning rate
    pub position: f32,
    
    /// Scale learning rate
    pub scale: f32,
    
    /// Rotation learning rate
    pub rotation: f32,
    
    /// Opacity learning rate
    pub opacity: f32,
    
    /// Color learning rate
    pub color: f32,
}
```

### Learning Rate Scheduling

Brush implements learning rate scheduling to improve convergence:

```rust
/// Update learning rates based on iteration
pub fn update_learning_rates(
    optimizer: &mut AdamScaled,
    iteration: usize,
) {
    // Compute decay factor
    let decay = f32::powf(
        FINAL_DECAY_FACTOR, 
        (iteration as f32) / (MAX_ITERATIONS as f32)
    );
    
    // Apply decay to all learning rates
    optimizer.learning_rates.position *= decay;
    optimizer.learning_rates.scale *= decay;
    optimizer.learning_rates.rotation *= decay;
    optimizer.learning_rates.opacity *= decay;
    optimizer.learning_rates.color *= decay;
}
```

## Evaluation Metrics 📏

### PSNR (Peak Signal-to-Noise Ratio)

PSNR measures the quality of reconstruction:

```rust
/// Compute PSNR between rendered and ground truth images
pub fn compute_psnr(rendered: &Image, ground_truth: &Image) -> f32 {
    let mse = mean_squared_error(rendered.data(), ground_truth.data());
    
    if mse == 0.0 {
        return f32::INFINITY;
    }
    
    let max_value = 1.0; // For normalized images
    10.0 * f32::log10(max_value * max_value / mse)
}
```

### SSIM (Structural Similarity Index)

SSIM captures structural similarity between images:

```rust
/// Compute SSIM between rendered and ground truth images
pub fn compute_ssim(rendered: &Image, ground_truth: &Image) -> f32 {
    let c1 = 0.01 * 0.01; // Constants for stability
    let c2 = 0.03 * 0.03;
    
    // Compute mean, variance, and covariance
    let (mean_x, mean_y, var_x, var_y, cov_xy) = compute_statistics(
        rendered.data(), 
        ground_truth.data()
    );
    
    // Compute SSIM
    let numerator = (2.0 * mean_x * mean_y + c1) * (2.0 * cov_xy + c2);
    let denominator = (mean_x * mean_x + mean_y * mean_y + c1) * 
                     (var_x + var_y + c2);
    
    numerator / denominator
}
```

## Configuration Parameters ⚙️

Training can be configured with various parameters:

```rust
/// Training configuration
pub struct TrainingConfig {
    /// Maximum number of iterations
    pub max_iterations: usize,
    
    /// Initial learning rates
    pub learning_rates: LearningRates,
    
    /// Warmup iterations
    pub warmup_iterations: usize,
    
    /// Density control interval
    pub density_control_interval: usize,
    
    /// Position regularization weight
    pub position_regularization: f32,
    
    /// Evaluation interval
    pub eval_interval: usize,
    
    /// Random seed
    pub seed: u64,
    
    /// Spherical harmonics degree
    pub sh_degree: usize,
}
```

## Visualizations 📊

Brush provides visualization during training:

1. **Progress View**: Shows current training iteration and metrics
2. **Comparison View**: Side-by-side comparison of rendering and ground truth
3. **Rerun Integration**: Detailed visualizations in Rerun viewer

```rust
/// Log training progress to Rerun
pub fn log_to_rerun(
    rec: &RecordingStream,
    scene: &Scene,
    rendered: &Image,
    ground_truth: &Image,
    metrics: &TrainingMetrics,
    iteration: usize,
) {
    // Log Gaussian positions and attributes
    rec.log_points("gaussians/positions", 
                  scene.positions(), 
                  scene.colors(),
                  scene.scales());
    
    // Log rendered and ground truth images
    rec.log_image("views/rendered", rendered.data());
    rec.log_image("views/ground_truth", ground_truth.data());
    
    // Log metrics over time
    rec.log_scalar("metrics/psnr", iteration, metrics.psnr);
    rec.log_scalar("metrics/loss", iteration, metrics.loss);
    
    // Log additional data for analysis
    rec.log_scalar("stats/gaussian_count", iteration, scene.gaussians.len());
}
```

## Customization Points 🔌

The training module provides several customization points:

1. **Custom Loss Functions**: Implement alternative losses
2. **Optimization Strategies**: Create custom optimization algorithms
3. **Regularization Terms**: Add custom regularization
4. **Evaluation Metrics**: Implement additional quality metrics
5. **Sampling Strategies**: Customize view/ray sampling

Example of custom loss function:

```rust
/// Custom loss function combining MSE and perceptual loss
pub fn custom_loss(
    rendered: &Image,
    ground_truth: &Image,
    weights: &LossWeights,
) -> Tensor {
    // Basic MSE loss
    let mse_loss = mean_squared_error(rendered, ground_truth);
    
    // Optional perceptual loss
    let perceptual_loss = if weights.perceptual > 0.0 {
        compute_perceptual_loss(rendered, ground_truth)
    } else {
        Tensor::zeros([], device())
    };
    
    // Combine losses
    mse_loss * weights.mse + perceptual_loss * weights.perceptual
}
```

## Performance Considerations ⚡

### GPU Memory Management

Efficient GPU memory usage is critical:

```rust
/// Manage GPU memory during training
pub fn manage_memory(
    scene: &Scene,
    device: &Device,
) -> Result<(), Error> {
    // Check current memory usage
    let memory_info = device.memory_info()?;
    
    // If memory usage exceeds threshold, reduce batch size
    if memory_info.used > MEMORY_THRESHOLD {
        return Ok(());
    }
    
    // Estimate memory required for next iteration
    let estimated_memory = estimate_memory_usage(scene);
    
    // Ensure enough memory is available
    if memory_info.free < estimated_memory {
        return Err(Error::OutOfMemory);
    }
    
    Ok(())
}
```

### Batch Processing

Processing multiple views in batches improves efficiency:

```rust
/// Process multiple views in a batch
pub fn process_batch(
    scene: &Scene,
    batch: &ViewBatch,
    device: &Device,
) -> BatchResults {
    // Prepare tensors for batch processing
    let positions = Tensor::from_vec(
        scene.positions_flat(), 
        (scene.gaussians.len(), 3),
        device
    );
    
    // Similar for other parameters...
    
    // Process batch in a single GPU operation
    let results = render_batch(&positions, &scales, &rotations, &colors, &batch.cameras);
    
    // Compute losses for all views in batch
    let losses = compute_batch_losses(results, &batch.ground_truth);
    
    BatchResults { rendered: results, losses }
}
```

## Error Handling 🚫

The training module handles various error conditions:

```rust
/// Training module errors
pub enum TrainingError {
    /// GPU memory overflow
    OutOfMemory,
    
    /// Numerical instability detected
    NumericalInstability { parameter: String, value: f32 },
    
    /// Invalid configuration
    InvalidConfiguration(String),
    
    /// Dataset error
    DatasetError(String),
    
    /// Renderer error
    RenderError(String),
}
```

Error handling example:

```rust
/// Handle potential errors during training
pub fn train_with_error_handling(
    config: TrainingConfig,
    dataset: Dataset,
) -> Result<Scene, TrainingError> {
    // Validate configuration
    if config.max_iterations == 0 {
        return Err(TrainingError::InvalidConfiguration(
            "max_iterations must be greater than 0".to_string()
        ));
    }
    
    // Try training with memory management
    match train_internal(config, dataset) {
        Ok(scene) => Ok(scene),
        Err(TrainingError::OutOfMemory) => {
            // Try with reduced resolution
            let reduced_config = config.with_reduced_resolution();
            train_internal(reduced_config, dataset)
        }
        Err(e) => Err(e),
    }
}
```

## Next Steps 🔍

- Learn about the [Rendering Module](rendering_module.md)
- Explore the [CLI](cli.md) for training configuration
- Understand [Performance Optimization](performance_optimization.md) techniques 