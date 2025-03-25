# Brush - AI-Enhanced Revitalization Project

This is a fork of the [Brush](https://github.com/ArthurBrussee/brush) project, an amazing cross-platform 3D reconstruction framework. This fork is an experimental effort by Ryan Hickman to contribute new ideas and explore enhancements. The primary goals are to:

1. Enhance Brush with AI-assisted development
2. Create a more modular, maintainable, and user-friendly application
3. Demonstrate Brush's potential as a foundation for cloud-hosted digital twin solutions

## Vision

This project aims to demonstrate Brush's potential as a foundation for cloud-hosted digital twin solutions. Realizing this vision involves refactoring the open-source project to empower third-party developers to build new applications leveraging the core technology. 

A key assumption is that reconstruction algorithms are constantly evolving, so this project focuses on creating a modular pipeline that can readily integrate the latest solutions. This may involve reprocessing older datasets to ensure compatibility with the newest client rendering techniques.

## Key Features

A core requirement for cloud-based digital twins is the ability to handle multiple scans of the same physical object or space. This necessitates:

* Aligning multiple datasets
* Storing data in a time-series database
* Enabling queries based on location and time
* Implementing a hybrid processing approach between cloud-based GPUs/TPUs and client-side rendering

## Project Roadmap

The project will be approached in the following phases, initially focusing on the core of Brush and then expanding to a new cloud-first application:

1. **Documentation of Existing System**: Document the existing Brush system for human and AI agents to understand.
2. **Robust Dev Environment**: Create a more robust development environment for automated testing.
3. **Modularity from UI**: Decouple the UI from the core logic.
4. **Panels and Windows UI Enhancements**: Modernize the UI to focus on the 3D content and support movable windows.
5. **Add Local Datasets**: Implement small-scale local dataset management.
6. **Modular Reconstruction Pipeline**: Implement and demonstrate a modular reconstruction pipeline backend.
7. **Utilize semantic parsing**: Incorporate solutions from public papers on semantic segmentation & labeling

The project will then create a new cloud-first app with enterprise features:

8. **Demonstrate Another UI App Using the Same Backends**: Create a separate UI application to demonstrate backend modularity.
9. **Add user authentication**: Implement modular SSO authentication (starting with Google Accounts)
10. **Add Cloud Datasets**: Implement modular scalable cloud dataset management (GCP initially).
11. **Ad-hoc time and space queries**: Utilize a timeseries database with geo features to query by time and space
12. **Configurable Workflow UI**: Enhanced user experience tool for crafting customizable workflows.

Each phase will have its own detailed documentation, outlining specific tasks, timelines, and deliverables.

## Product Requirements

For detailed product requirements, see:
- [Brush Core Enhancements PRD](./docs/brush_core_enhancements_prd.md)
- [Brush Enterprise Digital Twin PRD](./docs/brush_enterprise_digital_twin_prd.md)

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
