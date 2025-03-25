# Key Technologies 🔧

This document provides in-depth information about the core technologies used in Brush, explaining their role in the system and how they work together.

## Rust Programming Language 🦀

[Rust](https://www.rust-lang.org/) is the foundation of Brush, providing safety, performance, and cross-platform capabilities:

- **Memory Safety**: Prevents common bugs like null pointer dereferences and data races
- **Zero-Cost Abstractions**: High-level programming with minimal runtime overhead
- **Cross-Platform Support**: Allows Brush to run on multiple platforms with a single codebase
- **Cargo Ecosystem**: Simplifies dependency management and builds

Brush leverages Rust's strengths with:
- Trait-based abstractions for platform-specific implementations
- Strong typing for GPU data structures
- Ownership model for resource management

## Burn Machine Learning Framework 🔥

[Burn](https://github.com/tracel-ai/burn) is a machine learning framework written in Rust that powers Brush's optimization capabilities:

- **Auto-Differentiation**: Automatically computes gradients for optimization
- **Tensor Operations**: Provides efficient mathematical operations
- **Multiple Backends**: Supports various computation backends
- **WebGPU Integration**: Enables cross-platform GPU acceleration

Burn's key components used in Brush:
- The autodiff backend for gradient computation
- Tensor operations for numerical processing
- Device management for GPU operations

```
  ┌─────────────────────────────────┐
  │         Brush Application       │
  └───────────────┬─────────────────┘
                  │
                  ▼
  ┌─────────────────────────────────┐
  │    Autodiff<Wgpu> Backend       │──┐
  └───────────────┬─────────────────┘  │
                  │                    │
                  ▼                    │
  ┌─────────────────────────────────┐  │ Burn
  │      Tensor Operations          │  │ Framework
  └───────────────┬─────────────────┘  │
                  │                    │
                  ▼                    │
  ┌─────────────────────────────────┐  │
  │       Device Management         │──┘
  └───────────────┬─────────────────┘
                  │
                  ▼
  ┌─────────────────────────────────┐
  │           WebGPU API            │
  └─────────────────────────────────┘
```

## WebGPU 🌐

[WebGPU](https://www.w3.org/TR/webgpu/) is a modern graphics and compute API that enables Brush to run on multiple platforms:

- **Cross-Platform**: Works on browsers, desktop, and mobile
- **Modern Graphics Pipeline**: Provides modern rendering capabilities
- **Compute Shaders**: Enables general-purpose GPU computation
- **Efficient Memory Model**: Offers explicit memory management

Brush uses WebGPU through the [`wgpu`](https://github.com/gfx-rs/wgpu) Rust implementation for:
- Rendering Gaussian splats
- GPU acceleration of optimization algorithms
- Unified graphics API across platforms

## WGSL (WebGPU Shading Language) ⚡

[WGSL](https://www.w3.org/TR/WGSL/) is the shading language used with WebGPU:

- **Platform Independent**: Works across all WebGPU implementations
- **Strongly Typed**: Reduces shader bugs through type checking
- **Modern Features**: Supports modern GPU programming techniques
- **Similar to GLSL**: Familiar syntax for graphics programmers

Brush uses WGSL for:
- Rendering shaders for Gaussian splatting
- Compute shaders for optimization algorithms
- Cross-platform shader code

Example WGSL shader snippet from Brush:
```wgsl
@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let idx = global_id.x;
    if (idx >= u_globals.num_points) {
        return;
    }
    
    // Gaussian processing code
    // ...
}
```

## EGUI/EFRAME 🖼️

[egui](https://github.com/emilk/egui) is an immediate-mode GUI library used for Brush's user interface:

- **Immediate Mode**: Simplifies UI state management
- **Cross-Platform**: Works on web, desktop, and mobile
- **WebGPU Integration**: Integrates with Brush's rendering
- **Lightweight**: Minimal overhead and dependencies

[eframe](https://github.com/emilk/eframe) provides the frame for egui applications:

- **Window Management**: Handles window creation and events
- **Platform Integration**: Adapts to different platforms
- **Rendering Integration**: Connects egui with WebGPU

Brush uses egui/eframe for:
- Application UI panels
- Controls and parameter adjustment
- Information display
- Cross-platform UI consistency

## Rerun Visualization 📊

[Rerun](https://github.com/rerun-io/rerun) is a visualization tool integrated with Brush:

- **Real-time Visualization**: Displays training progress and results
- **Time-Series Data**: Shows how parameters evolve over time
- **3D Visualization**: Renders 3D data alongside 2D visualizations
- **Extensible API**: Supports custom data types and visualizations

Brush uses Rerun for:
- Visualizing training progress
- Debugging reconstruction issues
- Analyzing optimization behavior
- Comparing different model parameters

## Gaussian Splatting Algorithms 🧮

Gaussian splatting is the core reconstruction algorithm in Brush:

- **Differentiable Rendering**: Enables gradient-based optimization
- **Efficient Representation**: Balances quality and computational requirements
- **Real-time Performance**: Supports interactive visualization
- **Adaptive Density**: Adjusts detail based on scene complexity

Key algorithmic innovations include:
- Fast GPU-accelerated sorting for splatting
- Efficient covariance matrix operations
- Adaptive density control during optimization
- Tiled rasterization for performance

## Additional Technologies 🛠️

### Tokio

[Tokio](https://tokio.rs/) provides asynchronous runtime capabilities:

- **Async/Await**: Enables non-blocking operations
- **Task Scheduling**: Manages concurrent tasks
- **IO Operations**: Handles non-blocking IO
- **Cross-Platform**: Works on all supported platforms

### glam

[glam](https://github.com/bitshifter/glam-rs) is a math library for graphics:

- **SIMD Acceleration**: Fast mathematical operations
- **Graphics-Focused**: Designed for graphics applications
- **Ergonomic API**: Easy-to-use vector and matrix operations
- **No Dependencies**: Minimizes dependency footprint

### Trunk

[Trunk](https://github.com/trunk-rs/trunk) builds Brush for the web:

- **WASM Bundling**: Packages Rust code for WebAssembly
- **Asset Handling**: Manages static assets
- **Development Server**: Provides hot-reloading
- **Build Pipeline**: Streamlines web deployment

## Technology Integration 🔄

The technologies in Brush work together in a cohesive system:

```
┌──────────────────────────────────────────────────────────────────────┐
│                            User Interface (egui/eframe)              │
└───────────────────────────────┬──────────────────────────────────────┘
                                │
                                ▼
┌────────────────┬──────────────┴───────────────┬─────────────────────┐
│                │                              │                     │
│   Training     │      Rendering              │      Data           │
│   (Burn)       │      (WebGPU/WGSL)          │     Loading         │
│                │                              │                     │
└────────┬───────┴──────────────┬──────────────┴─────────┬───────────┘
         │                      │                        │
         │                      ▼                        │
         │        ┌─────────────────────────────┐        │
         └───────►│      Rerun Visualization    │◄───────┘
                  └─────────────────────────────┘
```

## Next Steps 🔍

- Learn about the [Build Process](build_process.md)
- Explore the [Development Workflow](development_workflow.md)
- Understand the [Training Module](training_module.md) and [Rendering Module](rendering_module.md) 