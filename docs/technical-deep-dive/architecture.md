# 3. Architecture Overview

This section provides a high-level overview of Brush's architecture, followed by detailed explanations of key components and patterns.

## 3.1 System Overview

Brush is designed as a modular system with three primary workflows:
1. **Training:** Converting input images into optimized 3D Gaussian Splats
2. **Viewing:** Real-time rendering of Gaussian Splat scenes
3. **Extension:** Building custom applications using Brush components

These workflows are implemented through a layered architecture:

```mermaid
flowchart TD
    subgraph APP["Application Layer"]
        UI["UI Components"]
        CLI["Command Line"]
    end

    subgraph DOMAIN["Domain Layer"]
        Process["Process<br>Orchestration"]
        Training["Training<br>Engine"]
        Rendering["Rendering<br>Engine"]
    end

    subgraph INFRA["Infrastructure Layer"]
        GPU["GPU<br>Abstractions"]
        Data["Data<br>Management"]
        Platform["Platform<br>Adapters"]
    end

    UI --> Process
    CLI --> Process
    Process --> Training
    Process --> Rendering
    Training --> GPU
    Rendering --> GPU
    Training --> Data
    Process --> Data
    UI --> Platform

    style APP fill:#f5f5f5,stroke:#333
    style DOMAIN fill:#e6f3ff,stroke:#333
    style INFRA fill:#fff0f0,stroke:#333
```

## 3.2 Core Architecture Patterns

Brush implements a layered architecture with several key patterns:

1. **GPU-Accelerated Rendering**
   - Uses WGPU/WGSL for cross-platform GPU support
   - Implements tile-based rendering with 16x16 tile size
   - Custom memory management for GPU buffers
   - Efficient data structures for splat representation

2. **Differentiable Training**
   - Built on Burn for GPU-accelerated autodiff
   - Custom AdamScaled optimizer with per-parameter learning rates
   - Efficient gradient computation through backward pass
   - Memory-efficient training with compact data structures

3. **Cross-Platform Support**
   - WebGPU for web deployment
   - Native GPU support for desktop
   - Unified API across platforms
   - Platform-specific optimizations

4. **Memory Management**
   - Efficient buffer reuse
   - Compact data structures
   - Smart memory allocation
   - Automatic cleanup

## 3.3 Cross-Platform Strategy

Brush achieves platform independence through a dedicated adaptation layer:

```mermaid
flowchart TD
    subgraph CORE["Core Components"]
        Train["Training Engine"]
        Render["Render Engine"]
        Data["Data Management"]
    end

    subgraph PLATFORM["Platform Layer"]
        Desktop["Desktop Adapter"]
        Web["Web Adapter"]
        Mobile["Mobile Adapter"]
    end

    subgraph FEATURES["Feature Management"]
        Required["Required Features"]
        Optional["Optional Features"]
        Platform-Specific["Platform-Specific"]
    end

    CORE --> PLATFORM
    PLATFORM --> FEATURES

    style CORE fill:#f0f7ff,stroke:#333
    style PLATFORM fill:#fff7f0,stroke:#333
    style FEATURES fill:#f5f5f5,stroke:#333
```

### Feature Management Matrix

| Category | Desktop | Web | Mobile |
|----------|---------|-----|---------|
| **Core Rendering** | Full GPU Pipeline | WebGPU Pipeline | Optimized Pipeline |
| **Training** | Full Capability | Memory-Constrained | Limited/Viewing Only |
| **Data Handling** | Local File System | Browser Storage + Remote | App Storage + Remote |
| **UI/UX** | Full Window System | Browser-Adapted | Touch-Optimized |

### Implementation Strategy

1. **Abstraction Layers**
   - Hardware abstraction through `wgpu`
   - Storage abstraction via Virtual File System
   - UI framework abstraction using `egui`

2. **Runtime Adaptation**
   - Dynamic feature detection
   - Resource-based capability scaling
   - Platform-optimized code paths
   - Performance monitoring:
     - GPU utilization tracking
     - Memory usage profiling
     - Frame time analysis

3. **Performance Considerations**
   - Platform-specific memory management
   - Render quality scaling
   - Computation workload adaptation
   - UI responsiveness optimization
   - Target metrics:
     - Desktop: Full quality, max performance
     - Web: Adaptive quality, 30+ FPS
     - Mobile: Power-efficient, 60+ FPS

## 3.4 Crate Breakdown

Brush is composed of several specialized crates, organized into the following categories:

### Application Layer

| Crate | Purpose | Key Components |
|-------|---------|----------------|
| `brush-app` | Main graphical application | - Initializes `eframe` window<br>- Sets up UI panels<br>- Manages application state<br>- Orchestrates UI, processing, and rendering |
| `brush-ui` | UI utilities and integration | - Integrates `egui`, `wgpu`, and `burn`<br>- Manages UI rendering and display<br>- Provides UI element helpers |
| `brush-cli` | Command-line interface | - Provides CLI binary<br>- Parses arguments via `clap`<br>- Runs operations without GUI |

### Core Processing

| Crate | Purpose | Key Components |
|-------|---------|----------------|
| `brush-process` | Processing orchestration | - Manages viewing/training streams<br>- Controls main processing loop<br>- Coordinates data and training |
| `brush-train` | 3D reconstruction training | - Implements `SplatTrainer`<br>- Manages optimization loop<br>- Handles Gaussian refinement |
| `brush-dataset` | Dataset management | - Defines core data structures<br>- Handles format parsing<br>- Manages data I/O operations |

### Rendering Pipeline

| Crate | Purpose | Key Components |
|-------|---------|----------------|
| `brush-render` | Forward rendering | - Defines `Splats` structure<br>- Implements rendering pipeline<br>- Manages projection and rasterization |
| `brush-render-bwd` | Backward pass | - Implements differentiable rendering<br>- Computes training gradients |
| `brush-kernel` | WGSL kernel utilities | - Manages GPU compute kernels<br>- Integrates with Burn backend |
| `brush-wgsl` | Shader processing | - Handles WGSL shader composition<br>- Generates Rust bindings |

### GPU Utilities

| Crate | Purpose | Key Components |
|-------|---------|----------------|
| `brush-sort` | GPU sorting | - Implements radix sort<br>- Provides sorting kernels |
| `brush-prefix-sum` | GPU prefix sum | - Implements parallel scan<br>- Supports GPU algorithms |

### Platform & Utilities

| Crate | Purpose | Key Components |
|-------|---------|----------------|
| `brush-android` | Android support | - Platform-specific bindings<br>- JNI integration |
| `brush-rerun` | Visualization tools | - Training progress logging<br>- Metric visualization |
| `colmap-reader` | Format parsing | - COLMAP format support<br>- Camera/image parsing |
| `sync-span` | Profiling utilities | - GPU operation sync<br>- Performance tracing |
| `rrfd` | File dialogs | - Cross-platform file operations |

## Where to Go Next?

For detailed information about specific components:

- **Training Process:** See the [Reconstruction Pipeline](reconstruction-pipeline.md#training-workflow)
- **Rendering Details:** Explore the [Rendering Pipeline](rendering-pipeline.md#viewing-workflow)
- **API Documentation:** Browse the [API Reference](../api-reference.md)
- **Extension Points:** Learn about [Extending Brush](extending-brush.md)