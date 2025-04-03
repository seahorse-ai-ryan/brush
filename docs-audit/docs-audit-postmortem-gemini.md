# Brush Documentation Audit: Postmortem Analysis (Phase 0)

## Purpose

This document captures the analysis performed as part of Phase 0 of the documentation rebuild plan. The goal is to hypothesize *why* the initial documentation generation process (using AI assistance) resulted in significant factual errors, structural issues, and maintainer concerns, as detailed in the [Audit Findings](./docs-audit-findings-gemini.md). Understanding these potential root causes helps inform a better process moving forward.

## Hypothesized Root Causes & Supporting Evidence

Based on a review of the generated documentation, the audit findings, maintainer feedback, and past interactions (including screenshots from the initial generation process), several factors likely contributed to the errors:

1.  **Error Compounding (Snowball Effect):**
    *   **Hypothesis:** An initial error, once generated and accepted (or missed during review), becomes part of the context for future generation tasks. The AI then treats this incorrect information as factual, leading to the error spreading across multiple documents.
    *   **Evidence:** The incorrect `ProjectedSplat` size (stated as 10 floats/40 bytes instead of the correct 9 floats/36 bytes) was identified early in the audit process (Screenshot 2) with the claim it was "shown in the code." This specific error was then repeated in multiple files (`performance.md`, `rendering-pipeline.md`, `benchmarks.md`, `CONTRIBUTING.md`).

2.  **Lack of Grounding / Prompting Issues:**
    *   **Hypothesis:** Prompts might not have sufficiently instructed the AI to base claims *exclusively* on verified information from the current codebase. The desire to fulfill a general request (e.g., "document performance") might have overridden the need for strict verification if readily available, verified data wasn't present in the immediate context.
    *   **Evidence:** Specific performance numbers (60+ FPS, 10-20 iter/sec) were added to `introduction.md` (Screenshot 3) with the claim they were sourced from the "codebase" and "upstream benchmarks." However, the audit found these specific numbers weren't validated locally in *this* project's code/tests. This suggests the AI fulfilled the goal of adding performance metrics without sufficiently grounding *those specific numbers* in the primary source of truth.

3.  **LLM Interpretation / Verification Gaps:**
    *   **Hypothesis:** The AI may have misinterpreted complex or niche code (Rust, WGSL, Burn framework specifics) or the verification step claimed during generation was not sufficiently rigorous or accurate.
    *   **Evidence:** The initial misstatement of the `ProjectedSplat` size claimed verification against the code (Screenshot 2), indicating either misinterpretation or a flawed check. Similarly, explanations for concepts like Spherical Harmonics (Screenshot 1) seemed plausible based on general knowledge but required deeper verification against Brush's specific implementation, which may not have occurred thoroughly.

4.  **Context Blurring (Docs/Code/External):**
    *   **Hypothesis:** The AI might have difficulty distinguishing the hierarchy of truthfulness between different context sources (existing incorrect docs, comments, actual code, external websites/benchmarks) unless explicitly guided. It might blend information from these sources.
    *   **Evidence:** The performance update (Screenshot 3) explicitly mentions sourcing information from both the "codebase" and "upstream benchmarks," potentially blurring the lines between the project's current, verifiable state and external or general performance expectations.

5.  **Reading Future Plans / Non-Code Sources:**
    *   **Hypothesis:** If design documents, PRDs, TODO comments, or other non-code text describing *planned* features were included in the context, the AI might have presented these as existing functionality.
    *   **Evidence:** While no direct screenshot confirms this, the maintainer feedback about "hallucinated details" and features that "don't have much to do with the actual code" strongly suggests this possibility, especially for complex features.

6.  **LLM Limitations:**
    *   **Hypothesis:** The underlying LLM may have inherent limitations in understanding the complex, niche domain (Rust + WGPU + Burn) or accurately interpreting intricate code logic, leading it to confabulate or rely more heavily on potentially flawed prose context.
    *   **Evidence:** The difficulty in accurately representing complex algorithm details (e.g., density control, `AdamScaled` specifics) identified in the audit aligns with the challenge LLMs face with highly specialized codebases compared to more common stacks.

7.  **Process Issues:**
    *   **Hypothesis:** The initial workflow may have lacked sufficient iterative verification. Errors introduced in early drafts might have been missed due to insufficient cross-referencing with the code during review stages.
    *   **Evidence:** The persistence of errors like the `ProjectedSplat` size across multiple files suggests they weren't caught and corrected early in the process.

8.  **Context Window / RAG Failures:**
    *   **Hypothesis:** The specific lines of code needed to verify a claim might not have been retrieved by the RAG system and included in the context window provided to the LLM for a given turn.
    *   **Evidence:** This is difficult to prove directly from the outputs but is a known limitation of RAG systems. An LLM cannot verify against information it wasn't given.

## Conclusion & Mitigation

It's likely that a combination of these factors contributed to the documentation issues. The revised "Start Fresh" strategy outlined in the [Revision Plan](./docs-post-audit-revisions-plan-gemini.md) aims to mitigate these risks through:

*   Explicit root cause analysis (this document).
*   Sanitizing reference material (`/old-docs`).
*   A clear, persona-driven structure planned upfront.
*   An AI collaboration strategy emphasizing:
    *   Clear, grounded prompts.
    *   Explicit context confirmation.
    *   Incremental generation and verification.
    *   Prioritizing code as the source of truth for technical claims.
    *   Careful context management. 