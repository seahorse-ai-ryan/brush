# Docs Audit Plan: Gemini Max - Brush Project

**Goal:** This audit was initiated in response to valuable feedback from Arthur, the primary maintainer of the upstream Brush project (and echoed by others), regarding the quality and accuracy of the existing documentation. Key concerns raised included:

*   _"Most of the information in technical-deep-dive is wrong."_
*   _"there are lots of hallucinated details"_
*   _"the performance numbers are meaningless"_
*   _"most important things are missing."_
*   _"The api-reference is similairly almost entirely hallucinated."_
*   A general lack of clear _"intent"_ or explanation of _"why"_ certain design choices were made, making the documentation less useful even when technically accurate in parts.

The primary goal of this audit, therefore, is to rigorously verify the accuracy and completeness of all project documentation against the current codebase and application behavior, directly addressing the maintainer's feedback and ensuring the resulting documentation is trustworthy and useful for developers and researchers.

**Source of Truth:** The codebase (including code comments) is considered the primary source of truth for technical claims. Screenshots and observed application behavior are sources of truth for UI/UX and workflow descriptions. Maintainer feedback guides the assessment of relevance, clarity, and completeness.

**Methodology:**

1.  **Inventory:** List all documentation files to be audited:
    *   `/README.md`
    *   `/CONTRIBUTING.md`
    *   `/docs/introduction.md`
    *   `/docs/api-reference.md`
    *   `/docs/benchmarks.md`
    *   `/docs/maintenance.md`
    *   `/docs/getting-started/user-guide.md`
    *   `/docs/getting-started/ui-overview.md`
    *   `/docs/getting-started/developer-guide.md`
    *   `/docs/technical-deep-dive/architecture.md` (Audited)
    *   `/docs/technical-deep-dive/core-technologies.md` (Audited)
    *   `/docs/technical-deep-dive/extending-brush.md` (Audited)
    *   `/docs/technical-deep-dive/performance.md` (Audited)
    *   `/docs/technical-deep-dive/reconstruction-pipeline.md` (Audited)
    *   `/docs/technical-deep-dive/rendering-pipeline.md` (Audited)
    *   `/docs/supporting-materials/faq.md`
    *   `/docs/supporting-materials/glossary.md`
2.  **Code/Behavior Mapping:** For each documentation file (or section), identify the relevant corresponding source code, application feature, or workflow.
3.  **Comparison:** Systematically compare the claims, explanations, examples, and diagrams in the documentation against the source of truth (code or observed behavior).
    *   Verify technical claims against code.
    *   Verify UI/workflow descriptions against application behavior (potentially using screenshots if needed).
    *   Check code examples for correctness and syntax.
    *   Validate diagrams against architecture and implementation.
4.  **Findings Tracking:** Document inaccuracies, outdated information, or missing details in a separate markdown file: `docs-audit/docs-audit-findings-gemini.md`.
5.  **Suggest Missing Content:** Based on the audit, identify and document potentially valuable technical details or explanations missing from the documentation in `docs-audit/docs-audit-findings-gemini.md`.
6.  **Revision:** Generate corrected documentation content based on the findings documented in `docs-audit/docs-audit-findings-gemini.md` and outlined in `docs-audit/docs-post-audit-revisions-plan-gemini.md`. 