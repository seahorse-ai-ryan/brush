# Phase 2: Documentation Structure Plan

This document outlines the planning for the new `/docs` directory structure for the Brush project, following the "Start Fresh" strategy outlined in the [Revision Plan](./docs-post-audit-revisions-plan-gemini.md).

## Learning from Excellent Documentation & Audit Findings

Before defining a new structure, we reviewed the [Docs Audit Postmortem](./docs-audit-postmortem-gemini.md) to understand root causes of previous issues (e.g., lack of grounding, error compounding, context limitations) and researched characteristics of highly-praised technical documentation.

**Key Takeaways from Research:**

*   **User-Centricity:** Successful documentation (often citing examples like **Stripe**, **Twilio**, **Docker**) focuses on user tasks and goals, providing practical examples and clear navigation.
*   **Clarity & Structure:** Logical organization, clear explanations, well-structured guides, effective use of visuals (where appropriate), and easy navigation (ToCs, linking) are crucial.
*   **Layered Information:** Providing high-level overviews alongside accessible deep-dives caters to different audience needs (Beginner to Researcher).
*   **Accuracy & Trust:** Documentation must be accurate, up-to-date, and grounded in the actual product behavior.

**Project Context & Constraints:**
*   Brush is a small project with limited resources.
*   Documentation resides in GitHub Markdown files, limiting complex layouts or features found on dedicated doc websites.
*   **Goal:** Create *concise*, *accurate*, and *essential* documentation focused on the most important user journeys and technical concepts, rather than attempting exhaustive coverage.

## Guiding Principles for New Documentation

Based on the Postmortem analysis and research findings, the new documentation will adhere to the following principles:

1.  **Accuracy & Grounding:** All technical claims MUST be verifiable from the codebase. Avoid assumptions, unverified performance numbers, or documenting non-existent features. (Addresses Postmortem: Lack of Grounding, Hallucination, Reading Future Plans; Ref: `documentation.mdc`).
2.  **Goal/Task-Oriented:** Structure content around *achieving tasks* (e.g., training a model, modifying UI) and understanding concepts relevant to those tasks. (Addresses Research: User-Centricity, Focus on User Tasks).
3.  **User Journey Focused:** Design clear paths for different user intents (e.g., quick start vs. deep dive), ensuring logical progression and strong internal linking. (Addresses Research: Clear Navigation & Structure).
4.  **Layered Information & Audience Awareness:** Provide concise overviews first, linking to more detailed guides or conceptual explanations for those who need them. Tailor language appropriately for the target audience of each section (User vs. Developer). (Addresses Research: Audience Awareness, Layered Information).
5.  **Practicality & Conciseness:** Include clear, runnable examples where applicable. Focus on essential information unique to Brush, avoiding unnecessary verbosity. (Addresses Research: Practicality; Mitigates Postmortem: Potential LLM verbosity).
6.  **Maintainability (GitHub Markdown):** Structure and write content suitable for standard GitHub Markdown rendering, relying on headings, ToCs, and linking for navigation. (Addresses Project Context).
7.  **Grounded in Code (Reiteration):** Emphasize linking concepts and guides back to the relevant source code where feasible. (Addresses Postmortem: Lack of Grounding).

## Rationale for Final Structure (User Guides vs. Development)

Initial structural ideas considered organizing by goal or concept directly. However, iterative drafting exercises (see `/scratchpad/`) revealed potential overlaps and highlighted the need to clearly separate content based on the primary user intent:

1.  **User Intent:** Is the reader trying to *use* Brush as-is, or *understand/modify* its internals?
2.  **Audience Alignment:** This maps well to our target audiences (Users/Enthusiasts primarily need Guides; Researchers/Developers primarily need Development docs).
3.  **Co-location for Developers:** This structure allows developer-focused conceptual explanations (e.g., UI architecture) and practical modification guidance (e.g., how to add a UI panel) to live within the same `/development` section, reducing context switching.
4.  **Clear Entry Points:** Provides distinct starting points (`/docs/guides` vs. `/docs/development`) based on user goals.

Therefore, the final proposed structure separates **User Guides** (task-based, how-to-use) from **Development Concepts & Guides** (conceptual explanations and how-to-modify).

## Target Audience Needs (Recap)

We must address the needs of key audiences:

*   **New Users / Enthusiasts:** Individuals interested in 3D reconstruction for VFX, games, immersive experiences, or AI development. Some are exploring Gaussian Splatting and want to try Brush, comparing it to alternatives. Needs: Quick start (demo/binary), simple usage examples, comparison points (if possible), hardware info.
*   **Researchers:** Academics or industry researchers focused on 3D reconstruction, inverse graphics, or ML-driven rendering. Aiming to improve algorithms, architectures, quality, speed, or capabilities. Needs: Technical deep dives (algorithms, math), parameter details, code pointers for core logic, contribution paths for algorithms/performance.
*   **Developers:** Software engineers building applications or solutions on top of Brush, or integrating its capabilities. Needs: Dev setup guide, architecture overview, API guides/examples (including library usage), contribution guides for features/integrations.

*(Note: Any of these audiences might also become contributors).* 

## Proposed File Structure Outline (Final Draft)

*   **`./README.md` (Project Root)**
    *   Succinct project overview, value prop, tech highlights (Rust, Burn, WGPU), platforms (Desktop, Web, Android), status (Proof of Concept fork). Basic `cargo run`. Links (Demo, Releases, Docs). License/Disclaimer. *Inspiration:* [Google Original](https://github.com/google-research/google-research/blob/master/brush_splat/README.md), [Arthur's Fork](https://github.com/ArthurBrussee/brush/blob/main/README.md), [Previous AI Attempt](https://github.com/seahorse-ai-ryan/brush/blob/gemini-docs/README.md)
*   **`/docs/README.md` (Documentation Root)**
    *   Welcome, explain structure (Guides, Development, Reference). Concise Quick Start (run demo/binary, view PLY). Links to main sections.
*   **`/docs/guides/`** (User Guides - How to *Use* Brush)
    *   `installing-brush.md`: Getting Brush running (pre-built binaries, web demo access, build from source *for use*).
    *   `training-a-scene.md`: Step-by-step: Load dataset (COLMAP/Nerfstudio format via Zip/Dir/URL), configure basic settings (UI/CLI), monitor training (UI stats, Rerun), export final PLY.
    *   `viewing-scenes.md`: How to load and view pre-trained `.ply` files using the UI or web demo (including direct URL loading).
    *   `cli-usage.md`: Using the `brush` CLI for headless training, export, and potentially viewing/other tasks. Key flags (`--dataset`, `--export-path`, `--export-name`, `--total-steps`, etc.).
*   **`/docs/development/`** (Developer Concepts & Guides - How Brush *Works* & How to *Modify* It)
    *   `setup.md`: Setting up the full dev environment (Rust, `cargo`, Trunk, WASM target, system deps) for building, testing, and contributing.
    *   `architecture.md`: Crate responsibilities (`brush-app`, `brush-process`, `brush-train`, `brush-render`, `brush-dataset`, helpers like `brush-sort`), data flow overview, UI (`app.rs`) vs. background (`process_loop.rs`) split, threading model (native threads vs. WASM tasks), platform support strategy (`#[cfg]`, `wgpu` backends), notes relevant for porting.
    *   `core-technologies.md`: Deeper look at key dependencies: How Brush uses `burn` (WGPU backend, Autodiff, custom ops/kernels via `burn-cube`), `wgpu`/`wgsl` (kernel structure via `naga_oil`, specific GPU feature usage/limitations like atomics/subgroups), `egui`/`eframe` (UI structure, `AppPanel` trait, `egui_tiles`).
    *   `training-and-rendering.md`: The core reconstruction loop: Differentiable rendering process (forward pass details from `brush-render`), loss calculation (L1+SSIM), backward pass (`brush-render-bwd`), optimizer step (`AdamScaled`), density control logic (`refine_if_needed` in `brush-train` - pruning/densification), rendering pipeline stages (projection, sorting, rasterization).
    *   `ui.md`: UI internals (`crates/brush-app`): `AppContext` shared state (`Arc<RwLock<>>` usage), `AppPanel` trait implementation, `egui_tiles` layout, message passing patterns (`ControlMessage`/`ProcessMessage`), examples of modifying/adding panels (incorporating scratchpad draft).
    *   `data-handling.md`: Input dataset parsing (`brush-dataset`, COLMAP/Nerfstudio formats), internal scene representation (`Scene`, `SceneView`), PLY export (`splat_export`), filesystem interactions (native vs. web considerations, `rrfd` usage).
    *   `performance.md`: How to profile using Tracy (`--features=tracy`), identifying common bottleneck areas (GPU sorting, rendering stages, data loading, CPU/GPU synchronization), general optimization considerations.
*   **`/docs/reference/`**
    *   `glossary.md`: Definitions (Gaussian Splatting, SH, NeRF, COLMAP, WGPU, Burn, Egui, etc.).
    *   `config-options.md`: Detailed list and explanation of parameters found in `TrainConfig`, `ModelConfig`, `LoadDataseConfig`, `ProcessConfig`, `RerunConfig`.
    *   `api-notes.md`: Guidance on using core crates (`brush-render`, `brush-train`, `brush-dataset`) as libraries in other Rust projects, linking prominently to generated `rustdoc`.

## Next Steps

*   Create the directory structure (`guides/`, `development/`, `reference/`) and placeholder `/docs/README.md` in `/docs`. *(Completed)*
*   **Begin Phase 3: Content Generation, following an inside-out approach:**
    1.  Draft initial content for key **User Guides** (`/docs/guides/`).
    2.  Draft content for **Developer Concepts & Guides** (`/docs/development/`).
    3.  Populate **Reference** material (`/docs/reference/`).
    4.  Draft introductory content for `/docs/README.md` and the project root `./README.md` last. 