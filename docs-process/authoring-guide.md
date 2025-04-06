# Brush Documentation Authoring Guide

This guide outlines the principles, target audience considerations, and stylistic best practices to follow when writing or editing documentation for the Brush project in the `/docs` directory. It complements the technical rules defined in `documentation.mdc` and `cursor-rules.mdc`.

*(See `structure-outline.md` for the overall documentation plan and `process-notes.md` for the history and collaboration strategy.)*

## Guiding Principles

These principles are derived from analysis of past documentation efforts and research into effective technical communication (see `process-notes.md` for background).

1.  **Accuracy & Grounding:** All technical claims MUST be verifiable from the codebase. Avoid assumptions, unverified performance numbers, or documenting non-existent features. (Ref: `documentation.mdc`).
2.  **Goal/Task-Oriented:** Structure content around *achieving tasks* (e.g., training a model, modifying UI) and understanding concepts relevant to those tasks.
3.  **User Journey Focused:** Design clear paths for different user intents (e.g., quick start vs. deep dive), ensuring logical progression and strong internal linking.
4.  **Layered Information & Audience Awareness:** Provide concise overviews first, linking to more detailed guides or conceptual explanations for those who need them. Tailor language appropriately for the target audience of each section (User vs. Developer).
5.  **Practicality & Conciseness:** Include clear, runnable examples where applicable. Focus on essential information unique to Brush, avoiding unnecessary verbosity.
6.  **Maintainability (GitHub Markdown):** Structure and write content suitable for standard GitHub Markdown rendering, relying on headings, ToCs, and linking for navigation.
7.  **Grounded in Code (Reiteration):** Emphasize linking concepts and guides back to the relevant source code where feasible.

## Target Audience Needs

We must address the needs of key audiences:

*   **New Users / Enthusiasts:** Individuals interested in 3D reconstruction for VFX, games, immersive experiences, or AI development. Some are exploring Gaussian Splatting and want to try Brush, comparing it to alternatives. Needs: Quick start (demo/binary), simple usage examples, comparison points (if possible), hardware info.
*   **Researchers:** Academics or industry researchers focused on 3D reconstruction, inverse graphics, or ML-driven rendering. Aiming to improve algorithms, architectures, quality, speed, or capabilities. Needs: Technical deep dives (algorithms, math), parameter details, code pointers for core logic, contribution paths for algorithms/performance.
    *   *Examples:* Experimenting with alternative splat representations, adapting Brush for dynamic scenes, exploring performance bottlenecks, or integrating novel techniques like LLM processing.
*   **Developers:** Software engineers building applications or solutions on top of Brush, or integrating its capabilities. Needs: Dev setup guide, architecture overview, API guides/examples (including library usage), contribution guides for features/integrations.
    *   *Examples:* Creating native iOS/Android builds (potentially with ARCore), hosting Brush in the cloud, extending for enterprise use, integrating COLMAP/pose estimation into the input pipeline, significantly overhauling the UI/UX (potentially contributing upstream to `egui`), adapting for XR/wearables, integrating Brush into other engines (Unreal, Unity, Three.js), or performing deeper Rerun integrations.

*(Note: Any of these audiences might also become contributors).*

## Technical Writing Style & Formatting

Adhere to these best practices for clear, consistent, and visually appealing documentation within GitHub Markdown:

*   **Consistency:**
    *   Use consistent terminology (e.g., "Gaussian Splatting model", "dataset", "training step"). Define terms in the `reference/glossary.md` or on first use.
    *   Use backticks (`) consistently for code symbols, file paths (`/docs/guides/`), directory names (`crates/brush-app/`), commands (`cargo run`), UI element labels (`Load file` button), and command-line flags (`--total-steps`).
*   **Clarity & Conciseness:**
    *   Write short sentences and paragraphs.
    *   Use active voice where possible.
    *   Explain the "Why" behind concepts, not just the "What".
*   **Structure & Readability:**
    *   Use Markdown headings (`#`, `##`, `###`) logically to structure content.
    *   Use ordered (`1.`, `2.`) and unordered (`*`, `-`) lists effectively.
    *   Ensure adequate whitespace around code blocks, lists, and paragraphs.
    *   Use horizontal rules (`---`) sparingly to separate major sections if needed.
*   **Code Examples:**
    *   Keep examples focused and runnable.
    *   Use appropriate language tags for syntax highlighting (e.g., ` ```rust`, ` ```bash`, ` ```toml`).
    *   Verify examples against the current codebase.
*   **Visual Cues:**
    *   **Alerts/Callouts:** Use blockquotes (`>`) prefixed with bold text to highlight important information. Common prefixes include:
        *   `> **Note:**` (General information)
        *   `> **Tip:**` (Helpful suggestions)
        *   `> **Warning:**` (Potential issues or caveats)
        *   `> **Danger:**` (Critical warnings, use sparingly)
        *   `> **Prerequisites:**` (Required setup before following steps)
    *   **Icons/Emojis:** Use standard emojis sparingly for visual emphasis where appropriate (e.g., ⚙️ for prerequisites/settings, 💡 for tips, ⚠️ for warnings). Avoid clutter.
    *   **Screenshots/Diagrams:** When documenting UI elements or complex workflows, consider referencing screenshots or diagrams (even if added manually later via TODOs) to improve clarity and accuracy.
*   **Linking:**
    *   Link liberally *within* the `/docs` directory using relative paths (e.g., `[Training Guide](./training-a-scene.md)`, `[Architecture](../development/architecture.md)`).
    *   Use external links for dependencies (Rust, Rerun, etc.) or relevant external resources.
    *   Consider adding placeholder links for planned sections (`<!-- TODO: Link to Performance Guide -->`).
*   **TODOs:**
    *   Use HTML comments `<!-- TODO: Verify this flag -->` to mark areas needing verification or future content.

## Review Checklist

Before considering a documentation section (or the entire set) complete, perform these checks:

*   **Accuracy:** Are all technical claims, parameters, commands, and code examples verified against the current codebase?
*   **Grounding:** Are claims based *only* on verifiable information within the project context?
*   **Clarity:** Is the language clear, concise, and appropriate for the target audience?
*   **Completeness:** Does the section cover the intended scope adequately? Are there obvious gaps?
*   **Consistency:** Does the terminology, formatting, and style align with this guide and other documents?
*   **Linking:** Are relevant internal links present? Do external links work?
*   **TODOs:** Have all `<!-- TODO: ... -->` comments been addressed or acknowledged?
*   **Visuals:** Are callouts used effectively? Are screenshots/diagrams (if used) clear and relevant?