# Code Structure 📂

This document describes the project's file organization, coding conventions, and key modules. Understanding the code structure will help you navigate and contribute to the Brush project effectively.

## Project Organization 🧱

Brush follows a Rust workspace pattern with multiple crates organized by functionality:

```
brush/
├── crates/                    # Main code directory with individual crates
│   ├── brush-app/             # Main application
│   ├── brush-cli/             # Command line interface
│   ├── brush-dataset/         # Dataset loading and scene management
│   ├── brush-kernel/          # GPU kernel implementations
│   ├── brush-prefix-sum/      # Prefix sum implementations
│   ├── brush-process/         # Process management
│   ├── brush-render/          # Forward rendering engine
│   ├── brush-render-bwd/      # Backward rendering for gradients
│   ├── brush-rerun/           # Rerun visualization integration
│   ├── brush-sort/            # Sorting algorithms
│   ├── brush-train/           # Training module
│   ├── brush-ui/              # User interface components
│   ├── brush-wgsl/            # WGSL shader utilities
│   ├── brush-android/         # Android-specific code
│   ├── colmap-reader/         # COLMAP format parser
│   ├── rrfd/                  # Resource handling
│   └── sync-span/             # Synchronization utilities
├── examples/                  # Example code and demonstrations
│   └── train-2d/              # 2D training example
├── docs/                      # Documentation files
├── Cargo.toml                 # Workspace definition
├── Cargo.lock                 # Dependency lock file
├── deny.toml                  # Dependency audit configuration
├── rust-toolchain.toml        # Rust toolchain specification
├── Trunk.toml                 # Trunk configuration for web builds
└── README.md                  # Project overview
```

## Key Crates 📦

### brush-app

The main application crate that integrates all components:

```
brush-app/
├── src/
│   ├── bin/                   # Binary entry points
│   │   └── bin.rs             # Main executable
│   ├── panels/                # UI panel implementations
│   ├── app.rs                 # Application state and logic
│   ├── camera_controls.rs     # Camera control handling
│   ├── lib.rs                 # Library exports
│   └── running_process.rs     # Process management
├── assets/                    # Application assets
├── Cargo.toml                 # Crate manifest
└── index.html                 # Web entry point
```

### brush-render

The forward rendering engine for Gaussian splatting:

```
brush-render/
├── src/
│   ├── shaders/               # WGSL shader files
│   ├── tests/                 # Unit tests
│   ├── bounding_box.rs        # Bounding box implementation
│   ├── burn_glue.rs           # Integration with Burn framework
│   ├── camera.rs              # Camera implementation
│   ├── dim_check.rs           # Dimension validation
│   ├── gaussian_splats.rs     # Gaussian splat representation
│   ├── kernels.rs             # Shader kernel management
│   ├── lib.rs                 # Library exports
│   ├── render.rs              # Main rendering logic
│   └── sh.rs                  # Spherical harmonics
├── Cargo.toml                 # Crate manifest
└── build.rs                   # Build script
```

### brush-render-bwd

The backward rendering engine for gradient computation:

```
brush-render-bwd/
├── src/
│   ├── shaders/               # WGSL shader files for backward operations
│   │   ├── gather_grads.wgsl  # Gradient gathering shader
│   │   ├── project_backwards.wgsl # Backward projection shader
│   │   └── rasterize_backwards.wgsl # Backward rasterization shader
│   ├── burn_glue.rs           # Integration with Burn framework
│   ├── kernels.rs             # Shader kernel management for gradients
│   └── lib.rs                 # Library exports
├── Cargo.toml                 # Crate manifest
└── build.rs                   # Build script
```

### brush-train

The training module for optimizing Gaussian parameters:

```
brush-train/
├── src/
│   ├── tests/                 # Unit tests
│   ├── adam_scaled.rs         # Adam optimizer implementation
│   ├── eval.rs                # Evaluation metrics
│   ├── lib.rs                 # Library exports
│   ├── multinomial.rs         # Multinomial distribution
│   ├── quat_vec.rs            # Quaternion vector operations
│   ├── ssim.rs                # SSIM metric implementation
│   ├── stats.rs               # Statistics tracking
│   ├── stats_kernel.rs        # Statistics computation kernels
│   └── train.rs               # Main training logic
├── test_cases/                # Test datasets
└── Cargo.toml                 # Crate manifest
```

### brush-dataset

Dataset handling and scene representation:

```
brush-dataset/
├── src/
│   ├── formats/               # Dataset format parsers
│   │   ├── colmap.rs          # COLMAP format support
│   │   ├── nerfstudio.rs      # Nerfstudio format support
│   │   └── mod.rs             # Format module exports
│   ├── brush_vfs.rs           # Virtual filesystem for datasets
│   ├── lib.rs                 # Library exports
│   ├── scene.rs               # Scene representation and management
│   └── scene_loader.rs        # Scene loading utilities
└── Cargo.toml                 # Crate manifest
```

## Coding Conventions 📝

### File Organization

- **lib.rs**: Exports the public API of each crate
- **mod.rs**: Organizes submodules within directories (when used)
- **[feature].rs**: Implements specific features
- **tests/**: Contains test files

### Code Style

Brush follows standard Rust coding conventions:

- **Naming**: 
  - `snake_case` for variables, functions, and modules
  - `CamelCase` for types and traits
  - `SCREAMING_SNAKE_CASE` for constants

- **Documentation**: 
  - Public APIs have rustdoc comments
  - Complex algorithms include explanatory comments

- **Error Handling**:
  - Uses `Result<T, E>` and `Option<T>` for error propagation
  - Custom error types defined where appropriate

- **Module Structure**:
  - Modules are organized by functionality
  - Public interfaces are clearly defined in lib.rs

## Dependency Management 🔗

Brush manages dependencies through:

- **Workspace Dependencies**: Common dependencies defined in the workspace root
- **Crate-Specific Dependencies**: Additional dependencies defined in each crate
- **Version Pinning**: Specific versions are pinned in Cargo.lock

Key dependencies include:

- **Burn**: Machine learning framework
- **wgpu**: WebGPU implementation
- **egui/eframe**: UI framework
- **tokio**: Async runtime
- **glam**: Math library

## Build System 🛠️

Brush uses Cargo for build management with:

- **build.rs Scripts**: Perform pre-build steps like shader compilation
- **Feature Flags**: Enable optional functionality
- **Conditional Compilation**: Platform-specific adaptations
- **Trunk**: Web-specific build tooling

## Testing Strategy 🧪

- **Unit Tests**: Located within modules or in dedicated test modules
- **Integration Tests**: Located in tests/ directories
- **Test Cases**: Real-world data for comprehensive testing

## Cross-Platform Adaptations 🌐

Platform-specific code is isolated:

- **cfg Attributes**: Conditional compilation for different platforms
- **Feature Flags**: Enable platform-specific features
- **Abstraction Layers**: Hide platform differences

## Navigation Tips 🧭

To find your way around the codebase:

1. Start with **brush-app/src/bin/bin.rs** for the application entry point
2. Look at **brush-app/src/app.rs** for the application structure
3. Explore the modular crates for specific functionality
4. Check tests for examples of how components are used

## Common Patterns 🔄

Brush employs several recurring patterns:

- **Backend Abstraction**: Interfaces for backend-agnostic code
- **Resource Management**: RAII pattern for resources
- **State Management**: Centralized application state
- **Event Handling**: Observer pattern for UI events
- **Builder Pattern**: For creating complex objects

## Next Steps 🔍

- Learn about the [Key Technologies](key_technologies.md) used in Brush
- Explore the [Development Workflow](development_workflow.md)
- Dive into specific modules like [Rendering](rendering_module.md) or [Training](training_module.md) 