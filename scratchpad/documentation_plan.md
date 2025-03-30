# Brush Project Documentation Plan

**Version:** 1.1 (Updated)
**Date:** March 28, 2024

**Guiding Principle for AI Generation:** The AI assistant generating this documentation **must** ensure all technical details (commands, file paths, parameters, API descriptions, supported formats, etc.) are strictly accurate and reflect the **current state** of the codebase and existing documentation (`README.md`, `Cargo.toml`, source code). Do not include planned features, speculative capabilities, or hallucinated information. Accuracy and adherence to the existing implementation are paramount.

**Navigation Suggestion:** For each major content page (Introduction, Guides, Deep Dives, Examples, API Reference), add a 'Next Steps' or 'Where to Go Next?' section at the end. This section should provide 2-4 contextually relevant links (using relative paths) to other documentation pages to facilitate non-linear exploration. *(Status: Implemented for major pages)*

## 1. Introduction

*   **1.1. Project Overview:** *(Status: Complete)*
    *   Describe Brush: An open-source 3D reconstruction application built with Rust.
    *   Explain its core purpose (e.g., creating 3D models/scenes from input data like images/video).
    *   Mention its target platforms (Desktop and Web via WASM).
    *   Highlight key differentiators (e.g., performance, specific algorithms like Gaussian Splatting, modern tech stack).
    *   Mention supported input formats (e.g., images, COLMAP datasets, Nerfstudio format).
*   **1.2. Target Audience:** *(Status: Complete)*
    *   Developers (contributing to the codebase, integrating Brush).
    *   Researchers (using Brush for experiments, understanding/extending algorithms).
*   **1.3. Key Features:** *(Status: Complete)*
    *   List main capabilities (e.g., data loading, reconstruction process, rendering, export).
*   **1.4. High-Level Architecture Diagram:** *(Status: Updated based on analysis, requires visual verification/refinement)*
    *   Include a visual representation of the main components (`crates`) and their interactions.

## 2. Getting Started

*   **2.1. User Guide:**
    *   **2.1.1. Installation:** *(Status: Complete, except checking for hosted WASM)*
        *   Instructions for Desktop (pre-built binaries added, build from source mentioned).
        *   Instructions for accessing/running the Web version (confirmed no hosted version currently, local build steps added).
    *   **2.1.2. Basic Workflows (Step-by-Step):** *(Status: Structure exists, needs detailed steps/visuals)*
        *   Workflow 1: Loading a Dataset. *(TODO: Add detailed steps/visuals)*
        *   Workflow 2: Running the 3D Reconstruction. *(TODO: Add detailed steps/visuals)*
        *   Workflow 3: Viewing and Interacting with the Rendered Scene. *(TODO: Add detailed steps/visuals)*
        *   Workflow 4: Exporting Results. *(TODO: Add detailed steps/visuals)*
        *   Workflow 5: Using the Command Line Interface (`brush_app`). *(TODO: Add detailed steps/visuals/examples)*
    *   **2.1.3. Hardware & Software Requirements:** *(Status: Complete)*
        *   OS (Linux, macOS, Windows, Web Browsers).
        *   CPU/RAM recommendations.
        *   GPU requirements (mention WGPU compatibility, CUDA not mandatory).
        *   Browser compatibility for WASM version.
*   **2.2. Developer Guide:**
    *   **2.2.1. Development Environment Setup:** *(Status: Complete)*
        *   Installing Rust (rustup, toolchains specified in `rust-toolchain.toml`).
        *   Installing WASM target (`rustup target add wasm32-unknown-unknown`).
        *   Installing Trunk (`cargo install trunk`).
        *   System dependencies added (Linux Wayland/X11, macOS, Windows).
        *   IDE setup recommendations (VS Code with rust-analyzer, etc.).
    *   **2.2.2. Building the Project:** *(Status: Complete)*
        *   Command to build the native Desktop application (`cargo build --release`).
        *   Command to build the WebAssembly version (`trunk build --release`).
    *   **2.2.3. Running Examples:** *(Status: Updated - clarified no standard examples, pointed to UI/CLI)*
        *   How to find and run examples from the `examples/` directory.
    *   **2.2.4. Running Tests & Coverage:** *(Status: Updated - added command, noted coverage scope needs work)*
        *   Command to run the test suite (`cargo test --workspace`).
        *   Description of what aspects of the codebase are covered by tests (e.g., specific crate functionality, integration tests) and areas that might have less coverage.
    *   **2.2.5. Contribution Guidelines:** *(Status: Complete - `CONTRIBUTING.md` created and linked)*
        *   Created `CONTRIBUTING.md` covering style, process, issues. *(TODO: Add `CODE_OF_CONDUCT.md`)*

## 3. Technical Deep Dive

*   **3.1. Architecture Overview:**
    *   **3.1.1. Monorepo Structure:** *(Status: Complete)*
    *   **3.1.2. Crate Breakdown:** *(Status: Complete - added `sync-span`, `rrfd`)*
    *   **3.1.3. Data Flow:** *(Status: Complete - text description added based on analysis)* *(TODO: Consider adding diagram)*
    *   **3.1.4. Cross-Platform Strategy:** *(Status: Complete)*
*   **3.2. 3D Reconstruction Pipeline:**
    *   **3.2.1. Conceptual Overview:** *(Status: Complete)*
    *   **3.2.2. Algorithm(s):** *(Status: Updated - confirmed direct GS optimization)*
    *   **3.2.3. Implementation Details:** *(Status: Updated - added key data structures)*
    *   **3.2.4. Configuration:** *(Status: Complete - mentions ProcessArgs)*
*   **3.3. 3D Gaussian Splat Rendering:**
    *   **3.3.1. Conceptual Overview:** *(Status: Complete)*
    *   **3.3.2. Rendering Pipeline:** *(Status: Complete)*
    *   **3.3.3. Training/Optimization Pass:** *(Status: Complete - added backward pass details)*
*   **3.4. Core Technologies Guide:** *(Status: Complete)*

## 4. API Reference

*   **4.1. Generating Documentation:** *(Status: Complete)*
*   **4.2. Key Public APIs:** *(Status: Complete - points to rustdoc)*

## 5. Supporting Materials

*   **5.1. FAQ:** *(Status: Updated - common issues expanded, new questions added)*
*   **5.2. Glossary:** *(Status: Updated - added new terms)*

## 6. Examples *(Section Removed - Content moved to /examples/README.md)*

## 7. Documentation Maintenance

*   Outline the process for keeping documentation up-to-date. *(Status: Complete)*

## 8. Code of Conduct

*   Create `CODE_OF_CONDUCT.md`. *(Status: Complete - template created, contact points to Discord)*

## 9. Visual Enhancement Suggestions

*(To be added based on AI review on March 30, 2024)*

Based on a review of the current documentation (`README.md`, User Guide, Developer Guide, Benchmarks, etc.), the following visual enhancements could improve readability, highlight key information, and provide a more engaging experience, while adhering to standard GitHub practices:

**General:**

*   **Consistency:** Ensure consistent use of formatting (e.g., bolding for UI elements, backticks for code/commands) across all documents.
*   **Icons (Emojis):** Use sparingly for emphasis on notes, tips, or warnings (e.g., ℹ️ for info, ✨ for tips, ⚠️ for warnings/prerequisites).

**README.md (`README.md`):**

1.  **Badges:** Consider adding more relevant badges from [Shields.io](https://shields.io/) at the top, if applicable (e.g., build status from CI, latest release version, Discord link - already present in upstream).
2.  **Callouts:**
    *   Use a `> [!NOTE]` or `> [!IMPORTANT]` blockquote for the section describing the project's origin and fork status.
    *   Use `> [!WARNING]` for the note about the experimental nature of the web demo and browser requirements.

**User Guide (`docs/getting-started/user-guide.md`):**

1.  **Callouts:**
    *   In **Installation (2.1.1)**:
        *   Use `> [!NOTE]` for hints about finding pre-built binaries on the Releases page.
        *   Use `> [!IMPORTANT]` or `> [!WARNING]` for the "Important Note (Training on Web)" regarding the Burn bug.
    *   In **Hardware & Software Requirements (2.1.3)**:
        *   Use `> [!IMPORTANT]` for the WebGPU browser support requirement.
    *   In **Understanding the UI Panels (2.1.4)**:
        *   Use `> [!TIP]` 💡 or an info icon ℹ️ before the "Navigation Controls" list for the Scene Panel to draw attention to it.
2.  **Diagrams/Flow:**
    *   In **Basic Workflows (2.1.2)**:
        *   This section is currently a `TODO`. Consider creating simple diagrams (using Mermaid syntax within Markdown, or static images) for each workflow (Load -> Train -> View -> Export).
        *   Supplement diagrams with numbered steps and screenshots for each step within the UI workflow.
3.  **Collapsible Sections (`<details>`/`<summary>`):**
    *   In **Basic Workflows (2.1.2)**: If the CLI command examples become very long with many options explained, consider wrapping them in `<details>` blocks so users can expand them if needed.

**Developer Guide (`docs/getting-started/developer-guide.md`):**

1.  **Callouts:**
    *   In **Development Environment Setup (2.2.1)**: Use `> [!NOTE]` for the lists of system dependencies for Linux/macOS/Windows.
2.  **Structure:** Ensure clear visual separation between build instructions for Desktop, Web, Android, and CLI using headings and potentially horizontal rules (`---`).

**Benchmarks (`docs/benchmarks.md`):**

1.  **Callouts:**
    *   Use `> [!NOTE]` to highlight the footnote about default opacity regularization.
    *   Use `> [!TIP]` 💡 for the "Profiling" section.

**Introduction (`docs/introduction.md`):**

1.  **Diagrams:**
    *   Review/Refine the "High-Level Architecture Diagram" (1.4). Consider using Mermaid syntax for maintainability or ensuring a clean static image (SVG preferred).

**Technical Deep Dive (`docs/technical_deep_dive/...`):**

1.  **Diagrams:**
    *   **Architecture Overview:** Refine the crate diagram as mentioned for the Introduction.
    *   **Data Flow:** Create a diagram (Mermaid sequence or flowchart?) illustrating how data moves between components during loading, training, and rendering.
    *   **Reconstruction/Rendering Pipelines:** Consider simple Mermaid flowcharts outlining the major steps in these processes.

* **AI Generation Note:** This documentation was initially drafted by an AI assistant (Gemini 2.5 Pro) on March 28, 2024, based on project analysis and user guidance. It has been iteratively updated but requires human review and verification, especially for workflow details and diagrams. 