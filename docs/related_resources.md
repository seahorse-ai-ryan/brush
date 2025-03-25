# Related Resources 🔗

This page provides links to external resources, research papers, and other relevant information related to Brush and 3D Gaussian Splatting technology.

## Research Papers

### Core Gaussian Splatting Papers

- [**3D Gaussian Splatting for Real-Time Radiance Field Rendering**](https://repo-sam.inria.fr/fungraph/3d-gaussian-splatting/) - The original paper introducing 3D Gaussian Splatting by Kerbl et al. (2023)
- [**4D Gaussian Splatting for Real-Time Dynamic Scene Rendering**](https://guanjunwu.github.io/4dgs/) - Extension to dynamic scenes
- [**Cat-4D: Learning to Concatenate 4D Gaussian Splatting for Multi-Scene 4D Reconstruction**](https://cat-4d.github.io/) - Method for handling multiple 4D scenes
- [**Cap4D: Physically Based Capture of 4D Explicit Neural Representation**](https://felixtaubner.github.io/cap4d/) - Focus on physically-based capture for 4D representations

### Semantic Parsing / Segmentation

- [**3D-GS-Seg: 3D Gaussian Splatting for Semantic Segmentation**](https://arxiv.org/abs/2403.09929)
- [**Semantic-GS: Enhancing 3D Gaussian Splatting with Semantic Scene Representations**](https://arxiv.org/abs/2403.15383)

## Software and Libraries

### Core Technologies

- [**Burn ML Framework**](https://github.com/tracel-ai/burn) - The machine learning framework used by Brush
- [**wgpu**](https://github.com/gfx-rs/wgpu) - The WebGPU implementation Brush relies on
- [**Rerun**](https://rerun.io/) - Visualization library used by Brush for additional data visualization
- [**EGUI**](https://github.com/emilk/egui) - The immediate mode GUI library used for Brush's interface
- [**Trunk**](https://github.com/trunk-rs/trunk) - WASM bundler for building Brush for the web

### Related Projects

- [**nerfstudio**](https://github.com/nerfstudio-project/nerfstudio) - A framework for neural radiance fields that defines a dataset format Brush can import
- [**gsplat**](https://github.com/nerfstudio-project/gsplat) - Reference implementation for 3D Gaussian Splatting
- [**COLMAP**](https://colmap.github.io/) - Structure-from-Motion and Multi-View Stereo software used to generate camera poses

## Learning Resources

### Tutorials and Guides

- [**Rust Programming Language Book**](https://doc.rust-lang.org/book/) - Comprehensive guide to learning Rust
- [**WebGPU Fundamentals**](https://webgpufundamentals.org/) - Tutorials on WebGPU concepts
- [**Burn Documentation**](https://burn.dev/book/) - Guide to using the Burn ML framework

### Blog Posts and Explanations

- [**Understanding 3D Gaussian Splatting**](https://medium.com/@ArthurBrussee/understanding-3d-gaussian-splatting-e7dae93d20d9) - Detailed explanation of the technology
- [**Web-Based 3D Gaussian Splatting**](https://github.com/antimatter15/splat) - Explanation of browser-based implementation challenges

## Datasets

- [**Mip-NeRF 360 Dataset**](https://jonbarron.info/mipnerf360/) - Commonly used dataset for evaluating novel view synthesis
- [**Tanks and Temples**](https://www.tanksandtemples.org/) - Benchmark dataset for image-based 3D reconstruction
- [**LLFF (Local Light Field Fusion)**](https://github.com/google-research/llff) - Dataset for novel view synthesis

## Community Resources

- [**Brush Discord Channel**](https://discord.gg/TbxJST2BbC) - Official Discord for Brush discussion
- [**Brush Web Demo**](https://arthurbrussee.github.io/brush-demo) - Online demonstration of Brush capabilities
- [**GitHub Discussions**](https://github.com/ArthurBrussee/brush/discussions) - Community discussions about Brush

## Digital Twin Resources

- [**Digital Twin Consortium**](https://www.digitaltwinconsortium.org/) - Industry group focused on digital twin technologies
- [**Digital Twin on Google Cloud**](https://cloud.google.com/solutions/digital-twin) - Cloud-based digital twin solutions
- [**Azure Digital Twins**](https://azure.microsoft.com/en-us/services/digital-twins/) - Microsoft's digital twin platform

## Best Practices and Standards

- [**WebGPU Specification**](https://www.w3.org/TR/webgpu/) - Official WebGPU standard
- [**PLY Format Specification**](http://paulbourke.net/dataformats/ply/) - Details on the PLY file format
- [**Time Series Database Best Practices**](https://docs.timescale.com/timescaledb/latest/how-to-guides/best-practices/) - For implementing time-series data capabilities 