# Brush Documentation Index

## Introduction

This document serves as the central index for Brush documentation. Brush is a Gaussian Splat visualization and training application built with Rust. It provides a modern, cross-platform interface for working with 3D Gaussian Splats, allowing users to load, visualize, and train models from various data sources.

## Documentation Index

- **[Dataset Handling](dataset_handling.md)**: How datasets are managed, loaded, and processed
- **[Reconstruction Process](reconstruction_process.md)**: The end-to-end 3D reconstruction pipeline
- **[Scene Rendering Pipeline](scene_rendering_pipeline.md)**: How 3D Gaussian Splats are rendered
- **[Export Service](export_service.md)**: The centralized export functionality
- **[Development Environment](development_environment.md)**: Setting up and working with the codebase
- **[Vibe Coding Guide](vibe_coding_guide.md)**: Coding standards and best practices
- **[Browser Tools Setup](browser_tools_setup.md)**: How to set up Browser Tools MCP for enhanced development and debugging
- **[Web Debugging Guide](debugging-web.md)**: Comprehensive guide for debugging web applications with Browser Tools MCP

## Core Functionality

- **Dataset Loading**: Import data from various sources (PLY files, ZIP archives, directories, URLs)
- **3D Visualization**: Real-time rendering of Gaussian Splats with camera controls
- **Training**: Train Gaussian Splat models with configurable parameters
- **Export**: Save trained models and rendered views

## Technology Stack

- **Rust**: Core programming language providing performance, safety, and cross-platform support
- **WGPU**: Low-level graphics API abstraction for GPU acceleration
- **egui**: Immediate mode GUI library for the user interface
- **Burn**: Machine learning framework for the training pipeline
- **WebAssembly (WASM)**: Compilation target for web deployment
- **Trunk**: Build tool for bundling Rust/WASM web applications
- **tokio**: Asynchronous runtime for handling concurrent operations

## Code Structure

The application is organized as a Rust workspace with multiple crates:

- **brush-app**: Main application with UI panels and application logic
  - `app.rs`: Core application structure and initialization
  - `panels/`: UI components (settings, scene, datasets)
  - `overlays/`: UI overlays (dataset details, controls, stats)
  - `export_service.rs`: Centralized export functionality
  - `orbit_controls.rs`: Camera control implementation
  
- **brush-ui**: UI utilities and components
- **brush-render**: Rendering pipeline for Gaussian Splats
- **brush-train**: Training implementation for Gaussian Splat models
- **brush-dataset**: Dataset loading and management
- **brush-process**: Process management for training and rendering
- **brush-kernel**: Core algorithms and data structures

## Architecture

The application follows a modular architecture:

1. **UI Layer**: Built with egui, providing panels for different functionality
   - Main panels: Scene, Dataset, Controls
   - Overlay windows: Dataset Details, Settings, Stats
   
2. **Process Layer**: Handles training and rendering processes
   - Asynchronous processing with message passing
   - State management for training and visualization
   
3. **Data Layer**: Manages datasets and model data
   - Dataset loading and preprocessing
   - Camera pose estimation
   - Gaussian Splat representation
   
4. **Rendering Layer**: Handles GPU-accelerated visualization
   - WGPU-based rendering pipeline
   - Real-time Gaussian Splat visualization
   - Camera controls and interaction

## Cross-Platform Support

The application supports multiple platforms:

- **Desktop**: Native applications for Windows, macOS, and Linux
- **Web**: Compiled to WebAssembly for browser-based usage
- **Mobile**: Experimental support for Android

## Data Flow

1. User loads a dataset (images, point clouds, etc.)
2. Data is processed and prepared for training
3. Training process optimizes Gaussian Splat parameters
4. Rendering pipeline visualizes the results in real-time
5. User can adjust parameters and export results

## Getting Started

To get started with Brush:

1. See the [Development Environment](development_environment.md) guide for setup instructions
2. Explore the [Dataset Handling](dataset_handling.md) documentation to understand how to load data
3. Learn about the [Reconstruction Process](reconstruction_process.md) to understand the pipeline
4. Check the [Scene Rendering Pipeline](scene_rendering_pipeline.md) for visualization details
5. Review the [Export Service](export_service.md) documentation for exporting models

## Contributing

For developers interested in contributing to Brush:

1. Follow the [Vibe Coding Guide](vibe_coding_guide.md) for coding standards
2. Review the [AI Agent Lessons Learned](ai_agent_lessons_learned.md) for common pitfalls and solutions
3. Set up your development environment following the [Development Environment](development_environment.md) guide

This architecture allows for a responsive, high-performance application that can handle complex 3D visualization and machine learning tasks while maintaining a clean, user-friendly interface. 