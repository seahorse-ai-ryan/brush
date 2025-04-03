# Brush Documentation Revision Plan

## Goal

To holistically revise the Brush project documentation, moving beyond simple accuracy checks towards creating a resource that is truly useful and beneficial for the target audience (software developers, 3D researchers). This plan is guided by the findings of the technical audit and direct feedback from the project maintainer.

The core aims are to:

*   Ensure technical accuracy.
*   Improve organization and information architecture based on user tasks and workflows.
*   Clearly explain the "Why" behind design decisions and architecture.
*   Provide practical, concrete examples and usage guidance.
*   Address specific content gaps and inaccuracies identified in the audit.

## Guiding Principles for Revision

To achieve the goal, the revision process will focus on the following principles:

1.  **Problem/Solution & Task Orientation:** Structure documentation around what users want to *do* (e.g., "Getting Started," "Integrating Brush," "Understanding the Pipeline," "Customizing Behavior," "Performance Tuning") rather than just listing features.
2.  **Explain the "Why":** Explicitly include the rationale behind design choices, algorithms, and architecture. Help users build a correct mental model.
3.  **Layered Information:** Provide high-level overviews first, then link clearly to deeper dives, tutorials, or API references. Avoid overwhelming users with detail upfront.
4.  **Concrete Examples & Tutorials:** Include practical, runnable code snippets and step-by-step guides for common tasks. Link these back to conceptual explanations.
5.  **Audience-Centric Language:** Tailor explanations to the expected background of software developers and 3D researchers, defining necessary jargon but avoiding overly simplistic or overly academic language.
6.  **Accurate and Purposeful Visuals:** If diagrams or flowcharts are used, ensure they are accurate, simple, clearly explained, and directly aid understanding of a specific concept or workflow.
7.  **Refined API Documentation:** Keep API references (`rustdoc`) concise and focused on usage (parameters, return types, errors, basic examples), linking *out* to conceptual docs for broader explanations.
8.  **Clear Information Architecture:** Ensure information is easy to find, logically organized, and minimally redundant. Use cross-linking effectively.
9.  **Trustworthiness:** Prioritize correcting all identified factual errors to rebuild confidence in the documentation's reliability.

## Specific Revision Actions (Based on Audit & Maintainer Feedback)

Based on the detailed audit findings and the maintainer's feedback, the following specific actions are planned:

**1. Content to Remove / Re-evaluate:**

*   **Most Diagrams/Flowcharts (Applicable: Primarily `architecture.md`, `reconstruction-pipeline.md`, `rendering-pipeline.md`):** Remove potentially misleading or hard-to-verify diagrams (flowcharts, complex architectural diagrams) as per maintainer feedback ("don't have much to do with the actual code"). Simple, verifiable diagrams (e.g., crate dependencies) may be kept if deemed essential and accurate.
*   **Specific Numerical Performance Targets (Applicable: `performance.md`, `reconstruction-pipeline.md`, `rendering-pipeline.md`, `introduction.md`, `CONTRIBUTING.md`):** Remove specific, unverified performance numbers (e.g., <1ms sort, 60+ FPS, 10-20 iter/sec) flagged as "meaningless" by the maintainer and unverified by the audit. These can be replaced with qualitative statements or links to live benchmark data if available.
*   **Inaccurate `api-reference.md` Content (Applicable: `api-reference.md`):** Remove pseudo-code struct definitions and incorrect feature flag descriptions, aligning with maintainer feedback that it was "almost entirely hallucinated." Refocus the file as a guide to generating/using `rustdoc`.
*   **All Specifically Identified Errors (Applicable: All files with documented errors):** Remove or correct all factual errors identified in the audit findings (incorrect parameters, struct details, function/command names, non-existent features/flags, wrong crate names, inaccurate algorithm descriptions) to address maintainer feedback about "wrong" and "hallucinated details."
*   **Potentially "Irrelevant" Sections (Applicable: Primarily `technical-deep-dive` section):** Critically evaluate sections, especially in technical deep dives, that lack clear explanations of "why" or "how" (addressing maintainer feedback about lack of "intent" and "relevance"). Consider removing/condensing superficial descriptions that don't add significant value beyond the code itself.

**2. Content to Add / Clarify:**

*   **Corrections for All Errors (Applicable: All files with documented errors):** Implement fixes for every inaccuracy documented in the audit findings.
*   **Missing Technical Explanations (Applicable: See 'Suggestions' section in `docs-audit-findings.md` for placement):** Add detailed explanations for the topics identified in the audit's "Suggestions for Further Documentation" section (GPU Memory, Burn Fusion, WGSL Kernels, Density Control Algorithm, AdamScaled Rationale, Data Loading, Tile Rendering, Platform Differences) to address maintainer feedback on "most important things are missing."
*   **Clearer Narrative and Intent ("Why") (Applicable: Primarily `technical-deep-dive` section):** Restructure technical sections to explain motivation and design choices *before* implementation details, addressing feedback about content lacking clear "why". Frame technical details within the context of solving specific problems or achieving specific goals.
*   **Accurate, Minimal Code Examples (Applicable: `extending-brush.md`, potentially `api-reference.md`):** Replace incorrect code examples with verified, simple examples for key library usage and correct CLI commands. Ensure examples are runnable and demonstrate common use cases.
*   **(Optional) Verified Simple Diagrams (Applicable: Where needed for clarity):** If diagrams are reintroduced, ensure they are simple, accurate, clearly labeled, explained in accompanying text, and add significant value to understanding. Consider standard notations (e.g., simple box-and-arrow for architecture, basic flowcharts for processes).
*   **Task-Oriented Guides:** Consider adding or restructuring content into specific guides (e.g., "Integration Guide," "Performance Tuning Guide," "Custom Kernel Guide") based on the principles outlined above. 