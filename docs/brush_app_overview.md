# Brush App Overview

## Introduction

Brush is a Gaussian Splat visualization and training application built with Rust. It provides a modern, cross-platform interface for working with 3D Gaussian Splats, allowing users to load, visualize, and train models from various data sources.

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
2. **Process Layer**: Handles training and rendering processes
3. **Data Layer**: Manages datasets and model data
4. **Rendering Layer**: Handles GPU-accelerated visualization

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

This architecture allows for a responsive, high-performance application that can handle complex 3D visualization and machine learning tasks while maintaining a clean, user-friendly interface. 