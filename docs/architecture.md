# Brush Architecture 🏗️

This document provides a detailed overview of Brush's system architecture, components, and data flow.

## System Overview 🔍

Brush is organized into a modular architecture that separates concerns and promotes code reusability. The system comprises several key components that work together to provide 3D reconstruction and visualization capabilities.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                             Brush System                                 │
│                                                                         │
│  ┌───────────┐    ┌────────────┐    ┌───────────┐    ┌───────────────┐  │
│  │   Data    │    │  Training  │    │ Rendering │    │      UI       │  │
│  │  Module   │◄──►│   Module   │◄──►│  Module   │◄──►│    Module     │  │
│  └───────────┘    └────────────┘    └───────────┘    └───────────────┘  │
│        ▲                 ▲               ▲                 ▲            │
│        │                 │               │                 │            │
│        ▼                 ▼               ▼                 ▼            │
│  ┌────────────────────────────────────────────────────────────────────┐ │
│  │                    Cross-Platform Foundation                        │ │
│  │  ┌──────────┐  ┌────────────┐  ┌───────────┐  ┌───────────────────┐ │ │
│  │  │   Burn   │  │   WebGPU    │  │  WASM     │  │  Platform-Specific │ │ │
│  │  │Framework │  │ Integration │  │ Support   │  │    Adaptations     │ │ │
│  │  └──────────┘  └────────────┘  └───────────┘  └───────────────────┘ │ │
│  └────────────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────────┘
```

## Core Components 🧩

### 1. Data Module (`brush-dataset`)

The Data Module handles loading, parsing, and managing different types of input datasets:

- **COLMAP Data**: Parses camera poses and sparse point clouds
- **Nerfstudio Format**: Reads transforms.json files
- **Image Loading**: Efficiently loads and processes image data
- **Masking Support**: Handles transparency and mask information
- **Scene Management**: Stores and manages scene state and representation

### 2. Training Module (`brush-train`)

The Training Module optimizes 3D Gaussian parameters to match input images:

- **Optimizer**: Implements adaptive optimization algorithms
- **Loss Functions**: Computes image-space differences
- **Parameter Updates**: Updates Gaussian properties based on gradients
- **Gaussian Management**: Adds/removes Gaussians as needed
- **Evaluation**: Measures reconstruction quality with metrics like PSNR and SSIM

### 3. Rendering Module (`brush-render` & `brush-render-bwd`)

The Rendering Module is split into two components:

#### Forward Rendering (`brush-render`)

- **Gaussian Representation**: Manages core data structures for Gaussians
- **Forward Rendering Pipeline**: Implements culling, sorting, and rasterization
- **Camera Management**: Handles camera parameters and movement
- **View Synthesis**: Generates novel views for visualization

#### Backward Rendering (`brush-render-bwd`)

- **Gradient Computation**: Calculates gradients for parameter optimization
- **Backward Rasterization**: Implements gradient-based operations
- **Differential Rendering**: Enables training through backpropagation
- **Integration with Burn**: Connects with Burn's autodifferentiation system

### 4. UI Module (`brush-ui`)

The UI Module provides user interfaces for interaction:

- **Egui Integration**: Builds on the [egui](https://github.com/emilk/egui) immediate mode GUI library
- **Panels**: Organizes controls and information
- **Camera Controls**: Handles user input for navigation
- **Visualization**: Displays images, metrics, and 3D content

### 5. Application Module (`brush-app`)

The Application Module ties everything together:

- **Application State**: Manages application lifecycle
- **Command Processing**: Handles user commands
- **Integration**: Connects all modules
- **Platform Specifics**: Adapts to different platforms

### 6. CLI Module (`brush-cli`)

The CLI Module provides command-line interfaces:

- **Command Parsing**: Processes command-line arguments
- **Batch Processing**: Enables non-interactive operation
- **Parameter Configuration**: Sets training/rendering parameters

## Cross-Platform Foundation 🌐

The cross-platform foundation enables Brush to run on diverse platforms:

- **Burn Framework**: Provides machine learning operations and auto-differentiation
- **WebGPU Integration**: Offers a modern graphics API that works across platforms
- **WASM Support**: Enables web browser execution
- **Platform-Specific Adaptations**: Handles differences between platforms

## Data Flow 📊

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│ Input Data  │────►│   Training  │────►│  Gaussian   │────►│  Rendering  │
│ (Images)    │     │   Process   │     │ Parameters  │     │   Engine    │
└─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘
                          │                                        │
                          │                                        │
                          ▼                                        ▼
                    ┌─────────────┐                         ┌─────────────┐
                    │ Optimization│                         │    View     │
                    │  Feedback   │◄────────────────────────│  Synthesis  │
                    └─────────────┘                         └─────────────┘
```

1. **Input Data** flows into the system via the Data Module
2. **Training Process** optimizes Gaussian parameters
3. **Gaussian Parameters** are passed to the Rendering Module
4. **Rendering Engine** creates views for evaluation and display
5. **View Synthesis** is compared with ground truth for **Optimization Feedback**

## Memory Management 💾

Brush uses a combination of memory management approaches:

- **GPU Buffers**: Store Gaussian parameters and intermediate results
- **CPU-GPU Synchronization**: Transfers data between CPU and GPU memory
- **Tensor Operations**: Leverage Burn for efficient tensor handling
- **Custom Allocators**: Optimize memory use for specific operations

## Module Dependencies 🔄

```
┌───────────────┐       ┌───────────────┐
│  brush-app    │──────►│   brush-ui    │
└───────────────┘       └───────────────┘
       │                        │
       │                        │
       ▼                        ▼
┌───────────────┐       ┌───────────────┐
│  brush-cli    │──────►│  brush-render │◄────┐
└───────────────┘       └───────────────┘     │
       │                        │             │
       │                        │             │
       ▼                        ▼             │
┌───────────────┐       ┌───────────────┐    │
│ brush-process │──────►│  brush-train  │────┼────────┐
└───────────────┘       └───────────────┘    │        │
       │                        │             │        │
       │                        │             │        │
       ▼                        ▼             │        │
┌───────────────┐       ┌───────────────┐    │        │
│ brush-dataset │◄─────►│ brush-kernel  │    │        │
└───────────────┘       └───────────────┘    │        │
                                             │        │
                                             ▼        ▼
                                      ┌───────────────┐
                                      │brush-render-bwd│
                                      └───────────────┘
```

## Extensibility Points 🔌

Brush was designed with extensibility in mind:

1. **Custom Reconstruction Algorithms**: Integration points for new reconstruction techniques
2. **Alternative Renderers**: Support for different rendering backends
3. **Data Format Adapters**: Extension points for new input/output formats
4. **UI Customization**: Modular UI components for custom interfaces
5. **Platform Support**: Abstraction layers for additional platforms

## Performance Considerations ⚡

Key performance optimizations in Brush include:

- **GPU Acceleration**: Heavy use of GPU for computation and rendering
- **Parallel Processing**: Multi-threaded operations where appropriate
- **Memory Efficiency**: Careful management of GPU memory
- **Adaptive Computation**: Dynamic adjustment of workload based on scene complexity

## Next Steps 🔍

- Explore the [Code Structure](code_structure.md) for implementation details
- Learn about the [Key Technologies](key_technologies.md) used
- Dive into the [Training Module](training_module.md) and [Rendering Module](rendering_module.md) for specific components 