# Brush Development Worklog

This file tracks key decisions, findings, and progress during the Brush project development, including the documentation rebuild effort.

## 2025-04-03

*   Initial documentation audit review completed ([Postmortem](./docs-audit-postmortem-gemini.md), [Findings](./docs-audit-findings-gemini.md), [Revision Plan](./docs-post-audit-revisions-plan-gemini.md)).
*   Decision made to pivot from persona-based guidance to goal-based guidance for AI collaboration rules.
*   Defined initial development goals: UI, Reconstruction, Infrastructure, Developer Environment, Documentation.
*   Created initial Cursor rules based on audit findings and revision plan.
*   Initiated refinement of Cursor rules to incorporate goal-based approach and triggers.
*   Refined and finalized Cursor rules (`.cursor/rules/*.mdc`) based on audit findings, incorporating goal-based triggers and explicit anti-patterns.
*   Defined and configured Custom Modes in Cursor settings (`Plan`, `Documentation`, `Git`, `Brush Client`, `Brush Web`) to streamline workflows and enforce rules.
*   Created `scratchpad/ai-lessons-learned.md` to track collaboration insights.
*   Completed Phase 0 (Root Cause Analysis & Process Improvement).

## 2025-04-04

*   Entering Phase 1: Sanitize Existing Docs & Setup New Structure.
*   Activating `Documentation` mode.
*   Sanitized `docs/technical-deep-dive/core-technologies.md` by correcting errors related to Burn, WGPU/WGSL (ProjectedSplat size), and Rerun activation.
*   Sanitized `docs/technical-deep-dive/extending-brush.md` by correcting Rust function name and CLI examples/options.
*   Sanitized `docs/technical-deep-dive/performance.md` by correcting ProjectedSplat size and removing unverified performance targets.
*   Sanitized `docs/technical-deep-dive/reconstruction-pipeline.md` by correcting initialization, gradient clipping, LR decay, density control parameters/descriptions, and removing unverified performance targets.
*   Sanitized `docs/technical-deep-dive/rendering-pipeline.md` by correcting ProjectedSplat size and removing unverified performance targets.
*   Sanitized `docs/api-reference.md` by correcting feature flag descriptions, fixing the rendering example function name, and removing the inaccurate pseudo-code section (4.2).
*   Sanitized `docs/benchmarks.md` by correcting ProjectedSplat size and removing/rephrasing unverified performance targets.
*   Sanitized `docs/getting-started/user-guide.md` by correcting CLI examples (command name, flags).
*   Sanitized `CONTRIBUTING.md` by correcting ProjectedSplat size and removing unverified performance targets. 