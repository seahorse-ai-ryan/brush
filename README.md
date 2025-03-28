# Brush - AI-Enhanced Revitalization Project

This is a fork of the [Brush](https://github.com/ArthurBrussee/brush) project, an amazing cross-platform 3D reconstruction framework. This fork is an experimental effort by Ryan Hickman to contribute new ideas and explore enhancements.

## Vision

Brush demonstrates how advanced 3D reconstruction can work directly in a standard web browser on machines without high-end NVIDIA GPUs, while also taking full advantage of hardware acceleration when available. This cross-platform flexibility makes it uniquely positioned for widespread adoption as a digital twin platform.

This project aims to demonstrate Brush's potential as a foundation for cloud-hosted digital twin solutions. Realizing this vision involves refactoring the open-source project to empower third-party developers to build new applications leveraging the core technology.

To enable Brush as a foundation for digital twin applications, this project focuses on enhancing modularity throughout the codebase. This allows for third-party applications to build on top of Brush's core capabilities or remain compatible with it as the project evolves.

## Key Core Enhancement Features

The core enhancements focus on improving the foundation of Brush:

1. **Modular Architecture** - Separating UI from core logic to enable multiple applications to use the same backend
2. **Enhanced UI Experience** - Modernizing the interface with movable panels while maintaining the focus on 3D content
3. **Flexible Storage Options** - Implementing robust local storage for desktop and web platforms with a consistent API
4. **Extensible Reconstruction Pipeline** - Creating a modular pipeline that can easily integrate new reconstruction algorithms
5. **Cross-Platform Testing Framework** - Establishing comprehensive testing across all supported platforms

## Key Enterprise Digital Twin Features

The enterprise digital twin features build on the enhanced core:

1. **Multi-scan Management** - Aligning and organizing multiple scans of the same physical object or space
2. **Time-Series Database Integration** - Storing reconstruction data with temporal metadata for change tracking
3. **Spatial-Temporal Queries** - Enabling searches based on location, time, and semantic properties
4. **Cloud Processing Capabilities** - Implementing hybrid processing between cloud GPUs/TPUs and client rendering
5. **Seamless Authentication** - Supporting SSO authentication (initially with Google Accounts)

## Documentation

The project documentation is organized into two main sections:

- [**Existing Implementation**](/docs/) - Documentation of the current implementation details
- [**Project Planning**](/project/) - Roadmaps and future-oriented documentation

Key project documents:
- [Brush Development Roadmap](/project/brush_development_roadmap.md) - High-level roadmap of planned development phases
- [AI-Assisted Development Approach](/project/ai_assisted_development.md) - Information about the experimental AI-assisted development approach
- [Developer Context](/project/developer_context.md) - Background on developer and AI tools used

This project is an experimental exploration not only of enhancing Brush but also of using AI-assisted development techniques to contribute to complex codebases.

## Contribution & Community

This is currently a solo project by Ryan Hickman, but feedback is welcome from the Brush developer community on Discord. Small, focused changes will be proposed as upstream pull requests to the original Brush project.

## Acknowledgements

This fork would not be possible without:

* [Brush](https://github.com/ArthurBrussee/brush) - The original project by Arthur Brussee and contributors that provides the foundation for this work
* [Rerun](https://github.com/rerun-io/rerun) - The incredible visualization library used by Brush
* Open source libraries used within Brush including (but not limited to):
  * [egui](https://github.com/emilk/egui) - The immediate mode GUI library
  * [eframe](https://github.com/emilk/eframe) - The egui framework
  * [wgpu](https://github.com/gfx-rs/wgpu) - The WebGPU implementation
  * [Rust](https://www.rust-lang.org/) - The programming language enabling safe, concurrent, practical systems programming

Many thanks to Arthur Brussee and all the contributors to the original Brush project for making their work open source and enabling this exploration.

## Original Brush Features

For information about the original Brush project features, including training capabilities, viewing functionality, CLI usage, and build instructions, please see the [original README](https://github.com/ArthurBrussee/brush#readme).

## License

This project maintains the license of the original Brush project. See the [LICENSE](LICENSE) file for details.
