# Data Handling

This document covers how Brush loads, represents, and saves scene data, focusing on the `crates/brush-dataset` crate and related interactions.

## Input Data Sources & VFS

Brush can load data from multiple sources, managed via the `DataSource` enum (`crates/brush-process/src/data_source.rs`) and a Virtual File System (VFS) abstraction implemented in `crates/brush-dataset/src/brush_vfs.rs`.

*   **Data Sources:**
    *   `PickFile`: User selects a local file (usually `.zip` or `.ply`) via a native dialog.
    *   `PickDirectory`: User selects a local directory (COLMAP or Nerfstudio format) via a native dialog (Not available on Web/Android).
    *   `Url(String)`: A URL pointing to a `.zip` or `.ply` file.
*   **VFS (`BrushVfs`):** Provides a consistent async trait `Vfs` for accessing file contents (`read_file`, `file_names`) regardless of the source (local, zipped, remote URL).
    *   Handles downloading from URLs.
    *   Handles reading files from within Zip archives.

## Dataset Parsing (`crates/brush-dataset`)

The `load_dataset` function (`src/formats/mod.rs`) and helpers within `src/formats/` handle parsing standard dataset formats using the VFS:

*   **COLMAP:** Parses `images.bin`/`.txt`, `cameras.bin`/`.txt`, and `points3D.bin`/`.txt` from the `sparse/0` directory. Uses the `colmap-reader` crate.
*   **Nerfstudio (Synthetic NeRF):** Parses `transforms.json` to extract camera poses and image paths.
*   **Image Loading:** Uses the `image` crate to load and decode images specified in the dataset files (via VFS).
*   **Configuration (`LoadDataseConfig`):** Allows controlling aspects like:
    *   `--max-resolution`: Limits loaded image dimensions.
    *   `--max-frames` / `--subsample-frames`: Limits the number of views loaded.
    *   `--eval-split-every`: Creates a separate evaluation dataset.
    *   `--subsample-points`: Limits initial points from COLMAP.

## Internal Scene Representation

Once loaded, the data is typically stored in these key structures:

*   **`SceneView` (`brush-dataset/src/scene.rs`):** Represents a single camera viewpoint.
    *   Contains `Camera` parameters (`brush-render`).
    *   Ground truth `image` (`image::DynamicImage`).
    *   Optionally converted to a `burn` tensor (`img_tensor`) for training.
*   **`Scene` (`brush-dataset/src/scene.rs`):** Represents the collection of views.
    *   Contains `Vec<SceneView>`.
    *   May store initial `points3d` if loaded from COLMAP.
*   **`Dataset` (`brush-dataset/src/lib.rs`):** Holds the `train: Scene` and `eval: Option<Scene>`.
*   **`Splats` (`brush-render/src/gaussian_splats.rs`):** The core data structure representing the set of 3D Gaussians. Contains `burn` tensors marked as trainable parameters:
    *   `means: Param<Tensor<B, 2>>`
    *   `log_scales: Param<Tensor<B, 2>>`
    *   `rotation: Param<Tensor<B, 2>>` (quaternions)
    *   `raw_opacity: Param<Tensor<B, 1>>`
    *   `sh_coeffs: Param<Tensor<B, 3>>`

## PLY Import/Export

*   **Import (`splat_import.rs`):** Loads `.ply` files containing Gaussian Splatting data (matching the standard format) directly into a `Splats` structure. Used primarily for viewing pre-trained models.
*   **Export (`splat_export.rs`):** The `splat_to_ply` async function saves the current state of a `Splats` structure to the standard PLY format.
    *   Asynchronously reads tensor data back from the GPU.
    *   **Rotation Normalization:** Ensures quaternions are normalized before saving.
    *   **SH Coefficient Ordering:** Permutes SH coefficients to the standard [xyz, d_scale, d_rotation, opacity, f_dc, f_rest] order.
    *   **Formatting:** Creates a PLY header defining the vertex properties (x, y, z, opacity, scale_*, rot_*, f_dc_*, f_rest_*) matching the [Inria standard format](https://github.com/graphdeco-inria/gaussian-splatting/blob/main/utils/sh_utils.py).
    *   **Writing:** Uses the `ply-rs` crate (forked version) to write the header and binary payload into a `Vec<u8>` buffer.

## Filesystem Interactions & Platform Differences

*   **VFS Abstraction:** The `BrushVfs` aims to hide many platform differences for *reading* data.
*   **Saving/Exporting:** Platform differences are more apparent when saving:
    *   **Native:** Can directly write to the filesystem.
    *   **Web (WASM):** Cannot directly write. Relies on browser mechanisms to trigger downloads. `rrfd::save_file` attempts to provide a consistent API but implementation varies.
*   **`rrfd` crate:** Used in `brush-app` to provide native file open/save dialogs. On the web, it might simulate dialogs or interact with browser APIs, with limitations.

## Next Steps

*   See how datasets are used in the [Training and Rendering Pipeline](./training-and-rendering.md).
*   Understand how data flows through the overall [Project Architecture](./architecture.md).
*   Review the specific [Configuration Options](../reference/config-options.md) related to data loading. 