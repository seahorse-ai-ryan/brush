# Documentation Structure Outline

This document records the rationale behind the chosen `/docs` directory structure and outlines the planned content for each file.

*(This structure was finalized after evaluating alternatives and considering target audience needs - see `authoring-guide.md`.)*

## Rationale for Final Structure (User Guides vs. Development)

Initial structural ideas considered organizing by goal or concept directly. However, iterative drafting exercises revealed potential overlaps and highlighted the need to clearly separate content based on the primary user intent:

1.  **User Intent:** Is the reader trying to *use* Brush as-is, or *understand/modify* its internals?
2.  **Audience Alignment:** This maps well to our target audiences (Users/Enthusiasts primarily need Guides; Researchers/Developers primarily need Development docs).
3.  **Co-location for Developers:** This structure allows developer-focused conceptual explanations (e.g., UI architecture) and practical modification guidance (e.g., how to add a UI panel) to live within the same `/development` section, reducing context switching.
4.  **Clear Entry Points:** Provides distinct starting points (`/docs/guides` vs. `/docs/development`) based on user goals.

Therefore, the final proposed structure separates **User Guides** (task-based, how-to-use) from **Development Concepts & Guides** (conceptual explanations and how-to-modify).

## Proposed File Structure Outline

*   **`./README.md` (Project Root)**
    *   Succinct project overview, value prop, tech highlights (Rust, Burn, WGPU), platforms (Desktop, Web, Android), status (Proof of Concept fork). Basic `cargo run`. Links (Demo, Releases, Docs). License/Disclaimer.
    *   *Inspiration:* [Google Original](https://github.com/google-research/google-research/blob/master/brush_splat/README.md), [Arthur's Fork](https://github.com/ArthurBrussee/brush/blob/main/README.md), [Previous AI Attempt](https://github.com/seahorse-ai-ryan/brush/blob/gemini-docs/README.md)
*   **`/docs/README.md` (Documentation Root)**
    *   Welcome, explain structure (Guides, Development, Reference). Concise Quick Start (run demo/binary, view PLY). Links to main sections.
*   **`/docs/guides/`** (User Guides - How to *Use* Brush)
    *   `installing-brush.md`: Getting Brush running (pre-built binaries, web demo access, build from source *for use*).
    *   `training-a-scene.md`: Step-by-step: Load dataset (COLMAP/Nerfstudio format via Zip/Dir/URL), configure basic settings (UI/CLI), monitor training (UI stats, Rerun), export final PLY.
    *   `viewing-scenes.md`: How to load and view pre-trained `.ply` files using the UI or web demo (including direct URL loading).
    *   `cli-usage.md`: Using the `brush` CLI (`brush_app` binary) for headless training, export, and potentially viewing/other tasks. Key flags and usage examples.
*   **`/docs/development/`** (Developer Concepts & Guides - How Brush *Works* & How to *Modify* It)
    *   `setup.md`: Setting up the full dev environment (Rust, `cargo`, Trunk, WASM target, system deps) for building, testing, and contributing.
    *   `architecture.md`: Crate responsibilities (`brush-app`, `brush-process`, `brush-train`, `brush-render`, `brush-dataset`, helpers like `brush-sort`), data flow overview, UI (`app.rs`) vs. background (`process_loop.rs`) split, threading model (native threads vs. WASM tasks), platform support strategy (`#[cfg]`, `wgpu` backends), notes relevant for porting.
    *   `core-technologies.md`: Deeper look at key dependencies: How Brush uses `burn` (WGPU backend, Autodiff, custom ops/kernels via `burn-cube`), `wgpu`/`wgsl` (kernel structure via `naga_oil`, specific GPU feature usage/limitations like atomics/subgroups), `egui`/`eframe` (UI structure, `AppPanel` trait, `egui_tiles`).
    *   `training-and-rendering.md`: The core reconstruction loop: Differentiable rendering process (forward pass details from `brush-render`), loss calculation (L1+SSIM), backward pass (`brush-render-bwd`), optimizer step (`AdamScaled`), density control logic (`refine_if_needed` in `brush-train` - pruning/densification), rendering pipeline stages (projection, sorting, rasterization).
    *   `ui.md`: UI internals (`crates/brush-app`): `AppContext` shared state (`Arc<RwLock<>>` usage), `AppPanel` trait implementation, `egui_tiles` layout, message passing patterns (`ControlMessage`/`ProcessMessage`), examples of modifying/adding panels.
    *   `data-handling.md`: Input dataset parsing (`brush-dataset`, COLMAP/Nerfstudio formats), internal scene representation (`Scene`, `SceneView`), PLY export (`splat_export`), filesystem interactions (native vs. web considerations, `rrfd` usage).
    *   `performance.md`: How to profile using Tracy (`--features=tracy`), identifying common bottleneck areas (GPU sorting, rendering stages, data loading, CPU/GPU synchronization), general optimization considerations.
*   **`/docs/reference/`**
    *   `glossary.md`: Definitions (Gaussian Splatting, SH, NeRF, COLMAP, WGPU, Burn, Egui, etc.).
    *   `config-options.md`: Detailed list and explanation of parameters found in `TrainConfig`, `ModelConfig`, `LoadDataseConfig`, `ProcessConfig`, `RerunConfig` (mapping to CLI flags).
    *   `api-notes.md`: Guidance on using core crates (`brush-render`, `brush-train`, `brush-dataset`) as libraries in other Rust projects, linking prominently to generated `rustdoc`. 