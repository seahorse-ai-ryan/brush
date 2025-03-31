# 3.y Extending Brush

Brush is designed with modularity in mind, allowing developers to extend its core functionality or leverage its components to build new applications and workflows.

## 3.y.1 Contributing to Core Brush

Contributing improvements or new features directly to the Brush open-source project is highly encouraged. Potential areas for contribution include:

*   Implementing new reconstruction algorithms or variations.
*   Adding support for different dataset formats or camera models.
*   Optimizing GPU kernels (`brush-sort`, `brush-render`, etc.).
*   Improving the UI/UX (`brush-ui`).
*   Enhancing the training process (`brush-train`).
*   Adding new export formats.

**Key Considerations:**

*   **Architecture:** Familiarize yourself with the **[Architecture Overview](architecture.md)**, particularly the roles of different crates.
*   **Core Technologies:** Understand how **[Burn](core-technologies.md#343-burn)**, **[wgpu](core-technologies.md#345-wgpu--wgsl)**, and **[egui](core-technologies.md#344-egui--eframe)** are used.
*   **Contribution Guidelines:** Strictly follow the process outlined in [`CONTRIBUTING.md`](../../CONTRIBUTING.md) regarding code style, testing, branching, and pull requests.
*   **API Stability:** Consider the impact of changes on existing APIs, especially those likely used by downstream crates or potential external users.

## 3.y.2 Building Custom Applications

The modular nature of Brush allows its crates to be used as libraries in other Rust projects.

**Potential Use Cases:**

*   **Specialized Viewers:** Create a custom viewer application with specific features not present in the default `brush_app` UI, potentially using a different GUI framework or integrating Brush rendering into a larger application.
*   **Domain-Specific Tools:** Build tools tailored for specific industries (e.g., cultural heritage, robotics, VFX) that use Brush's reconstruction or rendering capabilities.
*   **Alternative Front-Ends:** Develop different user interfaces (e.g., a simplified mobile app, a web front-end with different features) that utilize the core Brush libraries (`brush-train`, `brush-render`, `brush-dataset`) for the heavy lifting.

**Using Brush Crates:**

1.  Add the desired Brush crates (e.g., `brush-render`, `brush-dataset`) as dependencies in your `Cargo.toml`. You might point to the Git repository directly or a published version if available.
2.  Interact with the public APIs exposed by these crates.
3.  Note that you will need to manage the setup of `wgpu` and potentially `egui` or `Burn` in your own application if using those components directly.

## 3.y.3 Automation, Scripting, and Services

The command-line interface (`brush_cli` or `brush_app` used via CLI arguments) provides a powerful way to integrate Brush into automated workflows.

**Example Scenarios:**

*   **Batch Processing:** Write scripts (e.g., shell scripts, Python) to automate the training of multiple datasets sequentially using `brush_app` with appropriate command-line arguments.
    ```bash
    # Example pseudo-script
    for dataset in /path/to/datasets/*; do
      brush_app --dataset "$dataset" --output "$dataset/output.ply" --total-steps 30000 --save-final
    done
    ```
*   **Cloud Services:** Build a cloud-based 3D reconstruction service where users upload datasets, and backend workers use `brush_app` or its core libraries to process them on GPU instances.
*   **Integration with Pipelines:** Incorporate Brush reconstruction as a step within larger data processing or content creation pipelines.

**Using the CLI:**

*   Run `brush_app --help` (or `brush --help` if using the dedicated CLI build) to see all available commands and options.
*   Key commands likely involve specifying input dataset paths (`--dataset`), output file paths (`--output`), training parameters (`--total-steps`, learning rates, etc.), and actions like training (`--train`) or exporting (`--save-final`).
*   Ensure the environment where the script runs has access to the necessary hardware (GPU) and dependencies. 