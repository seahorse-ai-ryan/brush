# Docs Audit Findings: Gemini Max

## Audit Summary

This audit systematically compared all Brush project documentation (README, CONTRIBUTING, files in `/docs/`) against the codebase and maintainer feedback. 

*   **Scope:** All `.md` files within the project (excluding crate READMEs).
*   **Verification:** Numerous technical claims, setup instructions, code examples, UI descriptions, and configurations were checked.
*   **Accuracy:** While core architectural concepts, setup guides, and high-level features were often accurately described, significant inaccuracies were found, particularly within the `technical-deep-dive` section. Specific parameter values, algorithm details, feature flags, and code examples were frequently incorrect, aligning with maintainer feedback about "hallucinated" details.
*   **Completeness:** Key areas requiring deeper technical explanation were identified, aligning with maintainer feedback about missing important information.
*   **Actionability:** This document details specific findings (verified claims, errors, missing content suggestions, revision recommendations) for each audited file to guide documentation correction.

---

## `architecture.md`

### Section 3.2: Core Architecture Patterns

*   **Claim:** GPU-Accelerated Rendering using WGPU/WGSL and tile-based rendering with 16x16 tile size.
    *   **Status:** ✅ Verified
    *   **Evidence:**
        *   Code in `crates/brush-render/src/` extensively uses `wgpu`, `burn_wgpu`, and related types (`WgpuRuntime`, `CubeTensor`).
        *   Code in `crates/brush-render/src/render.rs` implements tile-based logic (`calc_tile_bounds`, `tile_offsets`, etc.).
        *   `crates/brush-render/src/shaders/mod.rs` defines `pub const TILE_WIDTH: u32 = 16;`, confirming the 16x16 tile size.
    *   **Notes:** The implementation appears consistent with the documentation's high-level description.

*   **Claim:** Differentiable Training built on Burn for GPU-accelerated autodiff, using a custom AdamScaled optimizer with per-parameter learning rates.
    *   **Status:** ✅ Verified
    *   **Evidence:**
        *   Code in `crates/brush-train/src/` and related crates heavily utilizes the `burn` framework, including `burn::backend::Autodiff` and `burn::backend::Wgpu`.
        *   `crates/brush-train/src/train.rs` uses `loss.backward()` for gradient computation.
        *   `crates/brush-train/src/adam_scaled.rs` defines the `AdamScaled` optimizer.
        *   `crates/brush-train/src/train.rs` instantiates `AdamScaled` and calls `optimizer.step` with distinct learning rates (`lr_coeffs`, `lr_rotation`, etc.) for different parameter groups (gradients for coefficients, rotation, etc.).
    *   **Notes:** The implementation confirms the use of Burn, autodiff, and the custom optimizer applying varied learning rates.

*   **Claim:** Cross-Platform Support (WebGPU for web, native GPU for desktop, unified API, platform-specific optimizations).
    *   **Status:** ✅ Verified
    *   **Evidence:**
        *   Extensive use of `#[cfg(target_os = "android")]`, `#[cfg(target_family = "wasm")]` and their negations indicates platform-specific code paths.
        *   Existence of `crates/brush-android` crate confirms a platform adapter layer.
        *   Comments and code confirm targeting `WebGPU` for web via the `wgpu` abstraction.
        *   Conditional compilation disables certain features (e.g., filesystem access, full training) on specific platforms (`wasm`, `android`).
    *   **Notes:** Implementation matches the documented cross-platform strategy.

*   **Claim:** Memory Management (Efficient buffer reuse, compact data structures, smart memory allocation, automatic cleanup).
    *   **Status:** ⚠️ Partially Verified / Needs Deeper Dive
    *   **Evidence (Verified):**
        *   Extensive use of GPU buffers (`buffer`, `uniforms_buffer`) throughout rendering and training code.
        *   Explicit `client.memory_cleanup()` calls found in `crates/brush-train/src/train.rs` suggest managed cleanup.
        *   Rust's ownership model inherently provides automatic cleanup via `Drop`.
    *   **Evidence (Needs Deeper Dive):**
        *   No clear evidence of custom allocators, memory pools, or explicit buffer reuse patterns found via keyword search.
        *   Verification of "compact data structures" requires inspecting struct definitions (`Splats`, etc.) and tensor data types.
        *   A comment in `crates/brush-render/src/render.rs` mentions potential memory challenges.
    *   **Notes:** While basic buffer usage and cleanup are evident, claims of *efficiency*, *compactness*, and *smart* allocation require more in-depth code analysis.

### Section 3.4: Crate Breakdown

*   **Crate:** `brush-app`
    *   **Claim (Purpose):** Main graphical application
    *   **Claim (Components):** Initializes `eframe` window, sets up UI panels, manages application state, orchestrates UI, processing, and rendering.
    *   **Status:** ✅ Verified
    *   **Evidence:**
        *   Implements `eframe::App` trait (`app.rs:507`).
        *   Imports and manages UI panels (`SettingsPanel`, `ScenePanel`, etc.) using `egui_tiles` (`app.rs:3`, `app.rs:112`, `app.rs:370-400`).
        *   Defines and manages shared state via `AppContext` struct (`app.rs:118`, `app.rs:403`).
        *   Handles background process communication (`RunningProcess`, `ProcessMessage`, `ControlMessage`) (`app.rs:5`, `app.rs:133`, `app.rs:442`).
        *   Drives UI updates via `eframe::App::update` (`app.rs:508`).
        *   Passes rendering context to `ScenePanel` (`app.rs:371`).
    *   **Notes:** Code structure and implementation align well with the documented purpose and components.

*   **Crate:** `brush-train`
    *   **Claim (Purpose):** 3D reconstruction training
    *   **Claim (Components):** Implements `SplatTrainer`, manages optimization loop, handles Gaussian refinement.
    *   **Status:** ✅ Verified
    *   **Evidence:**
        *   `SplatTrainer` struct defined (`train.rs:160`) and implemented (`train.rs:177`).
        *   `SplatTrainer::step` function (`train.rs:197`) performs rendering, loss calculation, backpropagation, and optimizer steps.
        *   `TrainConfig` contains refinement parameters (`train.rs:106-132`).
        *   `SplatTrainer::refine_if_needed` function (`train.rs:415`) implements periodic pruning (using `prune_points`) and densification/growth based on gradients and configuration.
    *   **Notes:** Code aligns well with the documented purpose and components.

*   **Crate:** `brush-render`
    *   **Claim (Purpose):** Forward rendering
    *   **Claim (Components):** Defines `Splats` structure, implements rendering pipeline, manages projection and rasterization.
    *   **Status:** ✅ Verified
    *   **Evidence:**
        *   `Splats` struct defined in `gaussian_splats.rs`.
        *   `render_forward` function in `render.rs` implements the main rendering pipeline.
        *   Pipeline includes steps using camera uniforms (`viewmat`, `focal`) and calls GPU kernels (`ProjectSplats`, `ProjectVisible`) related to projection.
        *   Pipeline prepares tile-based data and calls a `Rasterize` GPU kernel for rasterization.
    *   **Notes:** Code aligns well with the documented purpose and components.

*   **Crate:** `brush-process`
    *   **Claim (Purpose):** Processing orchestration
    *   **Claim (Components):** Manages viewing/training streams, controls main processing loop, coordinates data and training.
    *   **Status:** ✅ Verified
    *   **Evidence:**
        *   `process_stream` function in `process_loop/process.rs` routes to `view_stream` or `train_stream` based on input.
        *   `train_stream.rs` contains the main training loop (`for iter in ...`), calling `trainer.step` and `trainer.refine_if_needed`.
        *   `train_stream.rs` uses `brush_dataset::load_dataset` and `SceneLoader` to load data and provides batches to the `SplatTrainer`.
    *   **Notes:** Code structure confirms the crate's role in managing the backend processing workflows.

*   **Crate:** `brush-dataset`
    *   **Claim (Purpose):** Dataset management
    *   **Claim (Components):** Defines core data structures, handles format parsing, manages data I/O operations.
    *   **Status:** ✅ Verified
    *   **Evidence:**
        *   Core structures like `Dataset` (`lib.rs`), `Scene` (`scene.rs:160`), `SceneView` (`scene.rs:153`) are defined.
        *   Format parsing handled by `formats/`, `splat_import.rs`, `colmap-reader` crate, and image parsing in `scene.rs`.
        *   I/O managed via `BrushVfs` (`brush_vfs.rs`), `SceneLoader` (`scene_loader.rs`), `splat_import.rs`, `splat_export.rs`.
    *   **Notes:** Code structure and components directly map to the documented responsibilities.

---

## `core-technologies.md`

### Section 3.4.1: Rust
*   **Claim:** Rust 1.85.0 used (verified via `rust-toolchain.toml`).
*   **Claim:** Uses ownership/borrowing, async/await, custom derive macros, workspace organization (verified via `Cargo.toml` and code audit).
*   **Claim:** Dependencies: `glam` 0.28, `serde` 1.0.215, `tokio` 1.42.0, `wgpu` (verified versions in `Cargo.toml`).
*   **Status:** ✅ Verified

### Section 3.4.2: WebAssembly (WASM)
*   **Claim:** Uses `wasm-bindgen`, `Trunk`, WebGPU backend (verified via `Cargo.toml`, `Trunk.toml`, and code audit).
*   **Claim:** Build config in `Trunk.toml` (verified).
*   **Claim:** Dependencies: `tokio_with_wasm` 0.8.2, `reqwest` with `rustls-tls` (verified versions and features in `Cargo.toml`).
*   **Status:** ✅ Verified

### Section 3.4.3: Burn
*   **Claim:** Uses `AdamScaled` optimizer, WGSL backend, memory-efficient operations (verified optimizer and backend; efficiency requires profiling).
*   **Claim:** Key Components: `brush-burn` crate, custom WGSL kernels, fusion optimization (verified kernels and fusion; `brush-burn` crate does not exist).
*   **Claim:** Features Used: Autodiff, GPU tensor ops, custom backend implementation (verified autodiff and GPU ops; "custom backend implementation" seems inaccurate - uses standard backend with custom *kernels*).
*   **Status:** ⚠️ Partially Verified / Documentation Errors
*   **Errors:**
    *   Crate named `brush-burn` does not exist; integration is spread across several crates.
    *   Claim of "custom backend implementation" is misleading; it uses the standard Burn WGPU backend with custom operations/kernels.

### Section 3.4.4: EGUI / Eframe
*   **Claim:** Custom panels, `wgpu` rendering, real-time performance monitoring (verified).
*   **Claim:** Immediate mode UI, custom widgets for splat visualization, platform-specific optimizations (verified).
*   **Claim:** Dependencies: `egui`, `eframe`, `wgpu` (verified in `Cargo.toml`).
*   **Status:** ✅ Verified

### Section 3.4.5: WGPU / WGSL
*   **Claim:** Custom WGSL in `brush-wgsl`, GPU sort/prefix sum, efficient memory management (verified WGSL generation, sort/prefix sum crates; efficiency requires profiling).
*   **Claim:** Key Components: `brush-kernel`, `brush-render`, `brush-render-bwd` (verified).
*   **Claim:** Specs: 16x16 tile size, `ProjectedSplat` structure (10 floats, 40 bytes), custom memory layout (verified tile size; `ProjectedSplat` has 9 floats/36 bytes; "custom layout" vague).
*   **Status:** ⚠️ Partially Verified / Documentation Error
*   **Errors:**
    *   `ProjectedSplat` description is incorrect (9 floats, 36 bytes, not 10 floats, 40 bytes).
    *   Claim of "custom memory layout" is vague and hard to verify definitively beyond standard `repr(C)` usage.

### Section 3.4.6: Rerun
*   **Claim:** Optional dependency via `brush-rerun`, training/metrics logging, splat visualization (verified crate and logging usage).
*   **Claim:** Enable via `rerun` feature flag, configure logging in `brush-process` (verified runtime config in `brush-process`; no evidence of a `rerun` feature flag).
*   **Claim:** Real-time visualization, performance/memory tracking (verified via Rerun capabilities and logging calls).
*   **Status:** ⚠️ Partially Verified / Documentation Error
*   **Errors:**
    *   Rerun integration is controlled by runtime configuration (`rerun_config.rerun_enabled`), not a Cargo feature flag named `rerun`.

---

## `extending-brush.md`

*   **Claim:** Provides guidance on contributing to core Brush and building custom applications using Brush crates.
*   **Claim:** Suggests contribution areas consistent with previous crate audits.
*   **Claim:** Provides Rust and CLI usage examples.
*   **Status:** ⚠️ Partially Verified / Documentation Errors
*   **Notes:** High-level guidance seems reasonable and consistent with architecture.
*   **Errors:**
    *   Rust code example (Section 3.6.2) uses a non-existent function name (`brush_render::render` instead of `brush_render::render_forward`). The general concept is plausible, but the specific API call is wrong.
    *   CLI example (Section 3.6.3) uses the wrong command name (`brush_app` instead of the likely CLI binary name).
    *   CLI example incorrectly lists `--output` (should be `--export-path`/`--export-name`) and `--no-ui` (running the CLI *is* no-ui) as arguments.

---

## `performance.md`

*   **Claim:** Describes Tracy profiling setup, common bottlenecks, hardware recommendations, and memory considerations.
*   **Claim:** Tracy integration uses `tracy` feature flag and `tracing-tracy` dependency (verified).
*   **Claim:** Specifies performance targets (<1ms sort, 60+ FPS render, 10-20 iter/sec train) (accepted as documented goals).
*   **Claim:** Specifies hardware recommendations (8GB+ VRAM, etc.) (accepted as recommendations).
*   **Claim:** Repeats memory details: `ProjectedSplat` size, tile size, max splats config, efficiency claims.
*   **Status:** ⚠️ Partially Verified / Documentation Errors
*   **Notes:** Provides useful context on performance analysis and expectations.
*   **Errors:**
    *   Memory Considerations (Section 3.5.4) repeats the **incorrect `ProjectedSplat` size** (40 bytes instead of 36).

*   **Finding:** Specific performance targets (<1ms sort, 60+ FPS render, 10-20 iter/sec train) mentioned are not asserted or verified in the codebase's benchmarks or tests. They appear to be aspirational goals or point-in-time observations, not maintained requirements. Recommendation: Remove or rephrase these specific numerical targets in the documentation.

---

## `reconstruction-pipeline.md`

*   **Claim:** Details the 3D reconstruction pipeline (training process).
*   **Claim:** Correctly identifies crate roles (`brush-train`, `brush-process`, `brush-dataset`, `brush-render*`).
*   **Claim:** Correctly identifies input format support (COLMAP, Nerfstudio).
*   **Claim:** Correctly identifies loss function (L1 + SSIM).
*   **Claim:** Correctly identifies random initialization strategy.
*   **Claim:** Lists most configuration parameter defaults correctly.
*   **Claim:** Describes optimizer details (Adam, β values).
*   **Claim:** Describes density control concepts (pruning low opacity, densifying high gradient).
*   **Status:** ⚠️ Partially Verified / Documentation Errors
*   **Errors:**
    *   Claims grid-based initialization strategy, which was not found in the code.
    *   Claims gradient clipping (±1.0) is used in the optimizer, but it is not enabled by default in the code.
    *   Incorrectly describes the learning rate decay mechanism (exponential over total steps vs. fixed decay per N steps).
    *   Incorrectly specifies the default opacity threshold for pruning (~0.0035 vs. 0.01 claimed).
    *   Incorrectly specifies the default gradient threshold for densification (0.00085 vs. 0.0002 claimed).
    *   Provides an inaccurate/oversimplified description of the densification "growth rate".

*   **Finding:** Specific performance targets (10-20 iter/sec train) mentioned are not asserted or verified in the codebase's benchmarks or tests. They appear to be aspirational goals or point-in-time observations, not maintained requirements. Recommendation: Remove or rephrase these specific numerical targets in the documentation.

---

## `rendering-pipeline.md`

*   **Claim:** Details the real-time rendering pipeline for viewing and training.
*   **Claim:** Correctly identifies pipeline stages (Projection, Tile Mapping, Sorting, Rasterization) and supporting crates (`brush-render`, `brush-sort`, `brush-prefix-sum`, `brush-render-bwd`).
*   **Claim:** Correctly describes integration with the training workflow (forward/backward passes).
*   **Claim:** Correctly defines `INTERSECTS_UPPER_BOUND`.
*   **Status:** ⚠️ Partially Verified / Documentation Error
*   **Errors:**
    *   Memory Requirements section repeats the incorrect `ProjectedSplat` size (40 bytes instead of 36), although it lists the 9 fields correctly.

*   **Finding:** Specific performance targets (60+ FPS render) mentioned are not asserted or verified in the codebase's benchmarks or tests. They appear to be aspirational goals or point-in-time observations, not maintained requirements. Recommendation: Remove or rephrase these specific numerical targets in the documentation.

---

## Suggestions for Further Documentation (Content Gaps)

Based on the audit (primarily of the technical sections), the following technical areas lack sufficient explanation and represent opportunities for new or expanded content. This content could be integrated into existing technical deep-dive documents or form new sections/pages as appropriate:

1.  **GPU Memory Management Strategy:** Specifics on buffer allocation, pooling, reuse (e.g., via Burn's `WgpuRuntime` or custom logic), and the details of `client.memory_cleanup()`.
2.  **Burn Fusion Implementation Details:** Explanation of which operations are fused, how custom kernels interact with the fusion engine, and if custom fusion operations/rules are defined.
3.  **WGSL Kernel Optimizations & Logic:** Deeper dive into the algorithms within key shaders (`rasterize`, `project_visible`, `*_backwards`), including any performance-specific techniques (subgroups, shared memory) or data layout assumptions.
4.  **Adaptive Density Control Algorithm:** Precise description of the pruning and densification logic (selection criteria beyond thresholds, how splats are split/cloned, how new parameters are determined).
5.  **Rationale/Details for `AdamScaled` Optimizer:** Explanation for needing a custom optimizer and details on its specific scaling mechanism (how the `scaling` field in `AdamState` is used).
6.  **Data Loading Normalization:** Details on how different input formats (COLMAP, Nerfstudio) are processed and potentially normalized into the internal `Dataset` structure, including coordinate system assumptions.
7.  **Tile-Based Rendering Internals:** Explanation of how splat-to-tile mapping (`map_gaussian_to_intersects`) works and how `tile_offsets` are used in subsequent sorting and rasterization stages.
8.  **Concrete Platform Differences:** Elaboration on functional or performance differences between Web, Desktop, and Android beyond simple feature gating.

---

## `/docs/introduction.md`

*   **Claim:** Provides high-level project overview, motivation, target audience, key features.
*   **Claim:** Claims are consistent with previous audits (portability, tech stack, CLI, Rerun, masking).
*   **Claim:** Mentions performance targets (60+ FPS, 10-20 iter/sec).
*   **Status:** ✅ Verified (with note on performance targets)
*   **Notes:** Accurately summarizes project aspects. Performance targets lack code verification (see findings for `performance.md` etc.).

---

## `/docs/api-reference.md`

*   **Claim:** Acts as a guide to the API documentation, outlining structure, layers, feature flags, examples, and generation instructions.
*   **Claim:** Provides pseudo-code struct definitions for key crates.
*   **Claim:** Lists feature flags: `tracing`, `debug`, `webgpu`, `native`.
*   **Claim:** Provides examples for config, loading, rendering.
*   **Claim:** Provides `cargo doc` instructions.
*   **Status:** ⚠️ Partially Verified / Documentation Errors
*   **Notes:** File acts more as a *guide* to the `rustdoc` than a reference itself. The structural overview is consistent with the architecture. `cargo doc` instructions are correct.
*   **Errors:**
    *   Pseudo-code struct definitions in Section 4.2 are inaccurate/incomplete representations of the actual APIs.
    *   Feature flag descriptions (Section 4.3) are inaccurate:
        *   `tracing` flag exists but structure shown differs slightly.
        *   `debug` flag likely does not exist (standard logging controls used instead).
        *   `webgpu` / `native` flags do not exist (platform support uses target configuration, not features).
    *   Rendering code example repeats the incorrect `brush_render::render` function name.

---

## `/docs/benchmarks.md`

*   **Claim:** Presents benchmark methodology and results comparing Brush to other implementations (Inria, gsplat) on standard datasets.
*   **Claim:** Repeats performance targets and memory requirements from other documents.
*   **Claim:** Repeats Tracy profiling instructions.
*   **Status:** ✅ Verified (with known issues repeated)
*   **Notes:** Methodology seems reasonable. Results table cites an external source. Most content repeats information from other docs or presents external data.
*   **Known Issues Repeated:**
    *   Repeats performance targets (60+ FPS, 10-20 iter/sec) that lack code verification.
    *   Repeats the incorrect `ProjectedSplat` size (40 bytes vs 36).

---

## `/docs/maintenance.md`

*   **Claim:** Outlines the process for maintaining documentation (contributions, review, reporting issues, AI verification).
*   **Status:** ✅ Verified (as process documentation)
*   **Notes:** This file describes the documentation process itself and does not make technical claims about the Brush codebase that require code verification.

---

## `/docs/supporting-materials/faq.md`

*   **Claim:** Answers frequently asked questions about licensing, hardware, data, features, and troubleshooting.
*   **Status:** ✅ Verified
*   **Notes:** Answers are consistent with audit findings regarding license, hardware (no CUDA), data formats (COLMAP, Nerfstudio, PLY export), camera model handling (parsed but only pinhole used, distortion ignored), feature scope (no 4D training, editing, VR, etc.), and VFS handling of ZIP files. Known issues like WebGPU limitations are mentioned.

---

## `/docs/supporting-materials/glossary.md`

*   **Claim:** Defines key terms related to Brush, Gaussian Splatting, Rust, Burn, WGPU, etc.
*   **Status:** ✅ Verified
*   **Notes:** Definitions appear accurate and consistent with standard terminology and prior audit findings. Correctly distinguishes between `brush` (CLI binary) and `brush_app` (GUI binary).

---

## `/docs/getting-started/user-guide.md`

*   **Claim:** Guides users on installation, prerequisites, core UI workflows (train, export, view), CLI usage, and hardware requirements.
*   **Status:** ⚠️ Partially Verified / Documentation Error
*   **Notes:** Installation, prerequisites, UI workflow descriptions, and hardware requirements appear consistent and plausible.
*   **Errors:**
    *   CLI usage examples (Section 2.1.4) use incorrect/non-existent flags (`--output`, `--save-final`). The actual arguments for controlling output seem to be `--export-path` and `--export-name`.

---

## `/docs/getting-started/ui-overview.md`

*   **Claim:** Describes the UI panels (Scene, Dataset, Settings, Stats, Presets) and their controls/displayed information.
*   **Status:** ✅ Verified (based on consistency with code config)
*   **Notes:** Descriptions of panels, controls, settings, and stats appear plausible and consistent with configuration options found in the codebase (e.g., `TrainConfig`, `ModelConfig`, `RerunConfig`). Final verification would require running the UI, but no direct contradictions found.

---

## `/docs/getting-started/developer-guide.md`

*   **Claim:** Guides developers on environment setup, building (Desktop, Web, CLI), running examples, and testing.
*   **Status:** ✅ Verified
*   **Notes:** Provides accurate and standard instructions. Correctly identifies required tools (Rust, Trunk, WASM target, Rerun), system dependencies, build commands (`cargo build/run`, `trunk serve/build`, `-p brush-cli`), test commands (`cargo test --workspace`), and feature flags (`tracy`, `tracing`). Accurately describes the included `train-2d` example.

---

## `/README.md`

*   **Claim:** Provides project overview, features (training, viewing, CLI, Rerun), build instructions, benchmark results summary, acknowledgements.
*   **Status:** ✅ Verified
*   **Notes:** Accurately summarizes project goals, features (COLMAP/Nerfstudio support, masking, animation viewing, CLI, Rerun), build process, and benchmark results, consistent with detailed audit findings. Correctly identifies relevant commands and dependencies.

---

## `/CONTRIBUTING.md`

*   **Claim:** Outlines contribution process: prerequisites, setup, reporting bugs, suggesting enhancements, pull requests, code guidelines (performance, style, test, docs, commits).
*   **Status:** ✅ Verified (with known issues repeated)
*   **Notes:** Describes a standard contribution workflow for a Rust project. Correctly specifies commands for building, testing, formatting, linting, and doc generation.
*   **Known Issues Repeated:**
    *   Repeats performance targets (60+ FPS, 10-20 iter/sec) that lack code verification.
    *   Repeats the incorrect `ProjectedSplat` size (40 bytes vs 36).