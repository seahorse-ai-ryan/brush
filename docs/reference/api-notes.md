# API Notes & Library Usage

While Brush is primarily used via the `brush-app` executable (either GUI or CLI), its core logic is split into several library crates that could potentially be used independently in other Rust projects.

## Key Library Crates

*   **`brush-dataset`:** Provides functionality for loading datasets (COLMAP, Nerfstudio format) via its VFS, parsing PLY files (`splat_import`), and exporting splats to PLY (`splat_export`). It defines core data structures like `Scene`, `SceneView`, and configuration objects.
*   **`brush-render`:** Contains the forward rendering logic for `Splats` (projection, sorting, rasterization) and the `Splats` data structure itself.
*   **`brush-render-bwd`:** Provides the backward pass for differentiable rendering.
*   **`brush-train`:** Implements the training loop logic, including the optimizer (`AdamScaled`), loss calculations (using `brush-render`), and density control.
*   **Helper Crates:** `brush-sort`, `brush-prefix-sum`, `brush-kernel`, `colmap-reader`, etc., provide specific functionalities.

## Using Crates as Libraries

Using these crates in your own Rust project involves adding them as dependencies in your `Cargo.toml`. Since Brush often uses Git dependencies (especially for `burn` and potentially `wgpu`), you might need to replicate those Git dependencies or use compatible versions from crates.io if available.

```toml
# Example Cargo.toml dependency
[dependencies]
brush-render = { path = "../path/to/brush/crates/brush-render" } # Or specify git/version
# ... other necessary brush crates ...
# ... plus core dependencies like burn, wgpu, glam ...
```

**Important Considerations:**

*   **GPU Context:** Crates like `brush-render` and `brush-train` require a `wgpu` device/queue and likely a `burn` backend instance (`Autodiff<Wgpu>`) to be initialized and passed in.
*   **API Stability:** The internal APIs of these crates are not guaranteed to be stable between Brush versions. They are primarily designed for internal use by the Brush application.
*   **Complexity:** Replicating the setup and orchestration logic performed by `brush-process` and `brush-app` (e.g., device initialization, data loading pipeline, training loop management) can be complex.

## Key Public Entry Points (Illustrative)

While `rustdoc` provides the definitive API reference, these are some key public structs/functions you might interact with:

*   `brush_dataset::load_dataset(...)`: Parses and loads a dataset from a VFS source.
*   `brush_dataset::splat_import::splat_from_ply(...)`: Loads splats from PLY data.
*   `brush_dataset::splat_export::splat_to_ply(...)`: Exports splats to PLY format.
*   `brush_render::gaussian_splats::Splats`: The core data structure holding Gaussian parameters.
*   `brush_train::train::SplatTrainer`: Manages the training state and optimization loop.
*   `brush_process::process_loop::process_stream(...)`: Creates the main processing stream (viewing or training).

> **Warning:** Rely on `rustdoc` for accurate function signatures, struct fields, and module paths. This list is illustrative and may become outdated.

## Feature Flags

Brush uses Cargo feature flags to enable optional functionality, primarily controlled when building/running `brush-app`:

*   **`tracy`:** Enables integration with the [Tracy](https://github.com/wolfpld/tracy) profiler for detailed performance analysis on native targets. Requires `tracing` feature.
    *   Usage: `cargo run --features tracy ...`
*   **`rerun`:** Enables logging visualization data to the [Rerun](https://www.rerun.io/) viewer during training.
    *   Usage: `cargo run --features rerun ...`
*   **`tracing`:** Base feature enabling the `tracing` infrastructure, potentially used by `tracy`. (Enabled automatically by `tracy`).

(Other features might exist within specific dependencies but are not typically toggled directly by end-users of `brush-app`).

## Generated Documentation (`rustdoc`)

For detailed information on public functions, structs, and traits within each crate, refer to the generated `rustdoc` documentation.

You can generate this locally by running:

```bash
cargo doc --no-deps --open
```

This command builds the documentation for all workspace crates (excluding dependencies) and opens it in your web browser.

> **Note:** `--no-deps` speeds up the build by not documenting dependencies. Remove it if you need to see documentation for dependency types used in Brush's public APIs.

---
_End of API Notes_ 