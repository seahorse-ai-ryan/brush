# Brush Documentation Revision Plan

## Introduction

This plan outlines the steps to holistically revise the Brush project documentation. It aims to move beyond simple accuracy checks towards creating a resource that is truly useful, trustworthy, and beneficial for the target audience (software developers, 3D researchers).

This effort is guided by:
1.  The specific maintainer feedback received regarding inaccuracies and lack of clarity (detailed in the [Audit Plan](./docs-ai-agent-audit-plan.md)).
2.  The detailed findings of the technical audit (documented in [Audit Findings](./docs-audit-findings-gemini.md)).

The core principles guiding these revisions are: ensuring **technical accuracy**, adopting a **user-centric and task-oriented structure**, clearly explaining the **"Why"** behind design decisions, providing **concrete examples**, and improving overall **clarity and trustworthiness**.

## Revision Phases

The revision process will proceed in the following phases:

### Phase 1: Correct Factual Errors

*   **Action:** Systematically review the [Audit Findings](./docs-audit-findings-gemini.md) and correct *every* specifically identified factual error marked with `⚠️ Partially Verified / Documentation Errors`. This includes incorrect parameter values, struct details, algorithm descriptions, function/command names, non-existent features/flags, wrong crate names, etc., across all affected files.

### Phase 2: Remove Problematic Content

*   **Action:** Remove potentially misleading or hard-to-verify diagrams (flowcharts, complex architectural diagrams) as per maintainer feedback.
    *   *Applicable Files:* Primarily `architecture.md`, `reconstruction-pipeline.md`, `rendering-pipeline.md`.
*   **Action:** Remove specific, unverified numerical performance targets (e.g., `<1ms sort`, `60+ FPS`, `10-20 iter/sec`) flagged as "meaningless" and unverified by the audit. These can be replaced with qualitative statements or links to live benchmark data if available later.
    *   *Applicable Files:* `performance.md`, `reconstruction-pipeline.md`, `rendering-pipeline.md`, `introduction.md`, `CONTRIBUTING.md`.
*   **Action:** Remove inaccurate content from `api-reference.md`, specifically the pseudo-code struct definitions and incorrect feature flag descriptions. Refocus this file as a guide to generating/using `rustdoc`.
    *   *Applicable Files:* `api-reference.md`.

### Phase 3: Revise, Clarify, and Enhance

*   **Action:** Add missing technical explanations and address identified content gaps. Integrate detailed explanations for the following topics into appropriate documents:
    1.  **GPU Memory Management Strategy:** Specifics on buffer allocation, pooling, reuse (e.g., via Burn's `WgpuRuntime` or custom logic), and the details of `client.memory_cleanup()`.
    2.  **Burn Fusion Implementation Details:** Explanation of which operations are fused, how custom kernels interact with the fusion engine, and if custom fusion operations/rules are defined.
    3.  **WGSL Kernel Optimizations & Logic:** Deeper dive into the algorithms within key shaders (`rasterize`, `project_visible`, `*_backwards`), including any performance-specific techniques (subgroups, shared memory) or data layout assumptions.
    4.  **Adaptive Density Control Algorithm:** Precise description of the pruning and densification logic (selection criteria beyond thresholds, how splats are split/cloned, how new parameters are determined).
    5.  **Rationale/Details for `AdamScaled` Optimizer:** Explanation for needing a custom optimizer and details on its specific scaling mechanism (how the `scaling` field in `AdamState` is used).
    6.  **Data Loading Normalization:** Details on how different input formats (COLMAP, Nerfstudio) are processed and potentially normalized into the internal `Dataset` structure, including coordinate system assumptions.
    7.  **Tile-Based Rendering Internals:** Explanation of how splat-to-tile mapping (`map_gaussian_to_intersects`) works and how `tile_offsets` are used in subsequent sorting and rasterization stages.
    8.  **Concrete Platform Differences:** Elaboration on functional or performance differences between Web, Desktop, and Android beyond simple feature gating.
*   **Action:** Restructure technical sections to clarify narrative and intent ("Why"). Explain motivation and design choices *before* implementation details, framing technical details within the context of solving specific problems or achieving specific goals.
    *   *Applicable Files:* Primarily `technical-deep-dive` section documents.
*   **Action:** Replace incorrect/missing code examples with verified, simple, runnable examples for key library usage and correct CLI commands.
    *   *Applicable Files:* `extending-brush.md`, potentially `api-reference.md`.
*   **Action:** Critically re-evaluate sections flagged as potentially "irrelevant" during the audit, particularly those lacking clear explanations of "why" or "how". Condense or remove superficial descriptions that don't add significant value beyond the code itself.
    *   *Applicable Files:* Primarily `technical-deep-dive` section documents.
*   **Action (Optional):** If diagrams are reintroduced, ensure they are simple, accurate, clearly labeled, explained in accompanying text, and add significant value. Consider standard notations (e.g., simple box-and-arrow for architecture, basic flowcharts for processes).
*   **Action (Strategic):** Consider adding or restructuring content into specific task-oriented guides (e.g., "Integration Guide," "Performance Tuning Guide," "Custom Kernel Guide") based on the guiding principles.

## Execution Strategy for AI Agent

To execute this plan systematically, especially within context window limits:

1.  **Process File-by-File:** Iterate through the documentation files listed in the [Audit Plan](./docs-ai-agent-audit-plan.md).
2.  **Load Context:** For each target file:
    *   Read the relevant sections of the target documentation file.
    *   Read the corresponding findings section for that file from [Audit Findings](./docs-audit-findings-gemini.md).
    *   Keep this Revision Plan accessible for reference.
3.  **Execute Phases Sequentially (within file context):**
    *   **Phase 1:** Apply all specific error corrections identified in the findings for the current file.
    *   **Phase 2:** Apply all content removals applicable to the current file.
    *   **Phase 3:** Apply relevant revisions, clarifications, and additions (like adding missing explanations if the current file is the designated place, improving examples, clarifying intent).
4.  **Commit Incrementally:** After processing each file (or a logical group of related changes), commit the modifications with a clear message referencing the phase and file(s) changed.
5.  **Track Progress:** Mark completed actions or files within a local copy or by referencing commit history against this plan. 