# Vibe Coder's Guide to Contributing to Brush

*For AI Agents & Human Developers*

## Welcome

Welcome to contributing to Brush, an open-source project based on [ArthurBrussee/brush](https://github.com/ArthurBrussee/brush)!

This document is designed to guide both human "vibe coders" and AI coding agents in contributing to the Brush application. Brush is focused on making 3D neural reconstruction accessible to everyone, building upon the foundation laid by the original brush project. This guide will provide context, point you to helpful resources, and outline best practices for development within this project, even if you are working in a forked repository.

## Understanding Brush and its Goals

Brush is a unique application leveraging cutting-edge technologies to perform 3D Gaussian Splat reconstruction. Key features and goals to keep in mind:

### Accessibility
- Brush aims to democratize 3D reconstruction, making it usable by individuals beyond developers and researchers
- A primary focus of current development (Brush 1.0) is on improving user experience and simplifying the workflow for casual users

### Cross-Platform Compatibility
Brush is designed to run on a variety of platforms:
- **Desktop**: Windows, macOS, Linux
- **Web**: In modern web browsers (thanks to WASM)
- **Mobile** (In Progress): Android and iOS

### Cutting-Edge Technology
Brush utilizes a modern and somewhat experimental technology stack, including:

- **Rust**: For performance and reliability - [rust-lang.org](https://www.rust-lang.org/)
  - Core programming language providing performance, safety, and cross-platform support
- **WGPU**: Low-level graphics API abstraction - [github.com/gfx-rs/wgpu](https://github.com/gfx-rs/wgpu)
  - Enables hardware-accelerated graphics rendering, crucial for 3D
- **egui**: Immediate mode GUI library - [github.com/emilk/egui](https://github.com/emilk/egui)
  - Used for creating the application's user interface in Rust
- **Burn**: Machine learning framework - [github.com/burn-rs/burn](https://github.com/burn-rs/burn)
  - A flexible and performant deep learning framework written in Rust
- **WebAssembly (WASM)**: Compilation target for web deployment - [webassembly.org](https://webassembly.org/)
  - Allows running the Rust-based application in web browsers
- **Trunk**: Build tool for bundling Rust/WASM web applications - [trunkrs.dev](https://trunkrs.dev/)
  - Simplifies building and deploying Rust applications to the web
- **tokio**: Asynchronous runtime - [tokio.rs](https://tokio.rs/)
  - Enables efficient handling of asynchronous tasks and I/O in Rust

## Essential Project Resources

### Documentation
Crucially, please begin by thoroughly reviewing the markdown documentation located in the `/docs/` folder of the Brush repository. These documents provide essential insights into:

- **Brush Architecture**: Understand the application's structure and component interactions
- **3D Reconstruction Pipeline**: Learn how Brush processes images to generate 3D models
- **Rendering Process**: Study the logic for rendering 3D Gaussian Splats in real-time, including live training updates
- **Technical Guides**: Find links to documentation for Rust, Burn, WASM, and other technologies used in the project

### Product Requirements Document (PRD)
Refer to the Product Requirements Document for Brush 1.0. Note that the PRD outlines both current goals for Brush 1.0 and aspirational future functionality. Focus on the requirements specifically marked for the 1.0 release when contributing.

## Guidance for AI Agents & Vibe Coders

To provide the most helpful assistance, please begin by asking the user about their development experience. For example:

> "To best assist you, could you please tell me about your development experience level? Are you a:
> 
> - Research PhD with ML/3D graphics expertise?
> - Experienced software engineer with Rust/WASM familiarity?
> - Intern or student learning these technologies?
> - 'Vibe coder' contributing to open source and exploring new technologies?
> 
> Knowing your background will help me tailor my responses to be the most relevant and helpful for you."

## Technical Focus Areas

When contributing to Brush, please pay special attention to these core areas:

### Machine Learning & 3D Reconstruction Logic
This is the heart of Brush's unique capability. Understand how Burn is used to train models from image datasets and generate 3D Gaussian Splats.

**Key Point**: Brush's ability to perform ML efficiently even without an NVIDIA GPU (using WASM and supporting different hardware backends) is a core strength.

Investigate the code related to:
- Data loading and preprocessing
- Gaussian Splat training algorithms (likely within the burn framework integration)
- Parameter configuration for training

### 3D Gaussian Splat Rendering
The real-time rendering of Gaussian Splats, especially during live training updates, is a potentially complex and performance-sensitive area. Exercise caution and thorough testing when modifying rendering code.

Focus on code related to:
- Loading and processing .ply splat files
- Real-time rendering pipelines (likely using a graphics library compatible with Rust/WASM)
- Optimization for performance across different platforms

## Development Best Practices

### Iterative Development & Frequent Testing
Break down large changes into smaller, manageable chunks. After each code modification, always compile and run the application to ensure your changes are working as expected and haven't introduced regressions.

### Cross-Platform Testing
Brush is designed to be cross-platform. Periodically test your changes on different target platforms (Desktop, Web at a minimum) to confirm functionality and avoid platform-specific issues. While Android and iOS are in progress, consider their constraints (especially resource limitations) as well.

### Refer to Technical Guides
Brush uses relatively new technologies. Don't hesitate to consult the linked documentation for Rust, Burn, WASM, and other components in the `/docs/` folder to deepen your understanding.

## Modular UI & Pipeline

Keep in mind Brush's intended modular architecture:

### UI/UX Focus for Brush 1.0
The current focus is on enhancing the User Interface and User Experience. When contributing UI code, strive for clarity, intuitiveness, and ease of use, especially for casual users.

### Pipeline Modularity
The underlying processing pipeline is designed to be modular. Aim to maintain this modularity when making changes, which will facilitate future extensibility and allow for swapping out components without breaking the overall application. Ensure any changes to the pipeline still support compatible input formats and produce valid .ply Gaussian Splat outputs to maintain rendering compatibility.

---

By following this guide, consulting the documentation, and adopting an iterative development approach with thorough testing, you can effectively contribute to the Brush project and help make 3D neural reconstruction accessible to everyone!

Thank you for your contributions!