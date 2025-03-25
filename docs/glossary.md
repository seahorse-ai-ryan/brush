# Glossary 📖

This glossary provides definitions for technical terms used throughout the Brush documentation and codebase.

## 3D Reconstruction
The process of creating three-dimensional models of objects or scenes from a collection of two-dimensional images.

## Burn
A deep learning framework written in Rust that enables portable machine learning across various hardware platforms. Brush uses Burn for its machine learning operations.

## COLMAP
An open-source Structure-from-Motion (SfM) and Multi-View Stereo (MVS) pipeline used to generate camera poses and sparse 3D reconstructions from images.

## Digital Twin
A virtual representation of a physical object, process, or system. In Brush, digital twins can be created from multiple scans of the same physical object or space over time.

## Gaussian Splatting
A 3D representation technique that models a scene using a set of 3D Gaussian primitives (or "splats"). This method allows for high-quality, efficient rendering compared to traditional techniques.

## GPU (Graphics Processing Unit)
A specialized electronic circuit designed to rapidly manipulate and alter memory to accelerate the creation of images. Brush leverages GPUs for accelerated training and rendering.

## WASM (WebAssembly)
A binary instruction format designed as a portable target for compiling high-level languages like Rust. WASM enables Brush to run in web browsers.

## WebGPU
A modern web API for accessing GPU capabilities that provides better performance than WebGL. Brush uses WebGPU for cross-platform GPU acceleration.

## Posed Images
Images with known camera parameters including position, orientation, and intrinsic properties (focal length, etc.). Brush requires posed images for training.

## Rendering
The process of generating an image from a 3D model. In Brush, this involves projecting 3D Gaussians onto a 2D image plane.

## EGUI
A lightweight, immediate-mode GUI library for Rust used by Brush for its user interface.

## SfM (Structure from Motion)
A photogrammetric technique for estimating three-dimensional structures from two-dimensional image sequences, potentially coupled with local motion signals.

## Splat
A 3D Gaussian primitive used to represent points in a 3D scene. In Brush, a scene is represented as a collection of these splats.

## Trunk
A WASM web application bundler for Rust, used by Brush to build for the web.

## wgpu
A cross-platform, safe GPU abstraction in Rust, implementing WebGPU API. Brush uses wgpu for portable graphics operations.

## Nerfstudio Format
A data format standard for neural radiance fields (NeRF) datasets, which Brush can import for training.

## PLY (Polygon File Format)
A file format designed to store three-dimensional data from 3D scanners. Brush can load standard PLY files for viewing.

## PSNR (Peak Signal-to-Noise Ratio)
A metric used to measure the quality of reconstructed images compared to original images, expressed in decibels (dB). Higher values indicate better quality.

## SSIM (Structural Similarity Index)
A perceptual metric that quantifies image quality degradation caused by processing. Unlike PSNR, SSIM considers structural information.

## Regularization
Techniques applied during training to prevent overfitting. In Brush, opacity regularization is applied to improve the quality of the reconstruction.

## Training View
Images used during the training process to optimize the 3D Gaussian representation.

## Evaluation View
Images not used in training but used to assess the quality of the reconstruction.

## Radix Sort
An efficient non-comparative sorting algorithm used by Brush for sorting 3D Gaussians by depth.

## Semantic Segmentation
The process of partitioning an image into segments where each segment corresponds to a different object class. Brush incorporates solutions for semantic labeling.

## Rerun
A visualization library used by Brush to visualize additional data during training. 