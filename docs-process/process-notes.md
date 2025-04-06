# Documentation Process Notes: Learnings & AI Collaboration Strategy

This document captures historical context, including learnings from previous documentation efforts (Phase 0 audit) and the intended AI-human collaboration strategy for documentation tasks.

*(See `authoring-guide.md` for the current documentation principles and style guide, and `structure-outline.md` for the planned content.)*

## Summary of Past Documentation Generation Issues (Phase 0 Learnings)

*   **Lack of Grounding:** Documentation often described features, parameters, or performance inaccurately, not reflecting the actual codebase.
*   **Error Compounding:** Subsequent AI edits treated incorrect existing documentation as truth, propagating errors.
*   **Context Limitations:** AI often lacked sufficient context (code, prior steps) leading to hallucinations or incorrect assumptions (e.g., assuming standard directory structures like `tests/`).
*   **Reading Future Plans:** Documentation sometimes described planned features (from TODOs, comments) as if they already existed.
*   **Overly Verbose/Generic:** Content sometimes lacked specific, actionable details relevant to Brush.

These learnings informed the rules (`cursor-rules.mdc`, `documentation.mdc`, etc.) and the structured, grounded approach used in the subsequent documentation rebuild phases.

## Learnings from High-Quality Documentation

Research into characteristics of highly-praised technical documentation (e.g., Stripe, Twilio, Docker) highlighted these common traits, which also inform our principles:

*   **User-Centricity:** Focus on user tasks and goals, providing practical examples and clear navigation.
*   **Clarity & Structure:** Logical organization, clear explanations, effective use of visuals (where appropriate), and easy navigation (ToCs, linking) are crucial.
*   **Layered Information:** Provide high-level overviews alongside accessible deep-dives caters to different audience needs.
*   **Accuracy & Trust:** Documentation must be accurate, up-to-date, and grounded in the actual product behavior.

## Project Context & Constraints

These factors influenced the documentation strategy:

*   **Limited Resources:** Brush is a relatively small project, necessitating a focus on essential documentation rather than exhaustive coverage.
*   **GitHub Markdown:** Documentation resides in standard Markdown files, limiting complex layouts or interactive features found on dedicated documentation websites. Focus is on clarity through structure, text, and code examples.
*   **Documentation Goal:** Create *concise*, *accurate*, and *essential* documentation focused on the most important user journeys and technical concepts.

## AI Collaboration Strategy (Co-Creation within Cursor)

This outlines the general approach for how the AI agent (Gemini 2.5 Pro/Max) and the User will collaborate during the documentation rebuild process. This strategy leverages **Cursor Project Rules** and potentially **Custom Modes** (beta) to prioritize accuracy, clarity, persona alignment, and efficient use of context, aiming to mitigate issues identified in Phase 0.

**Core Principles:**

*   **User-Guided:** The user directs the overall process, selects tasks, provides goals, and makes final decisions.
*   **AI as Co-Creator & Verifier:** The AI assists in generating drafts, applying specific corrections, finding information, and verifying claims against provided context (code, findings), guided by defined rules.
*   **Incremental & Verifiable:** We will favor smaller, manageable steps with explicit verification over large, monolithic changes.
*   **Rule-Driven Consistency:** Utilize Cursor's features to enforce best practices and reduce repetitive prompting or user error.

**Leveraging Cursor Features (Setup during Phase 0/1):**

1.  **Project Rules (`.cursor/rules`):** Define persistent instructions triggered by file paths or semantics. Examples:
    *   `cursor-rules.mdc`: Core guidelines.
    *   `documentation.mdc`: Documentation authoring rules.
    *   `git-workflow.mdc`: Git procedures.
    *   `ui-dev.mdc` (applied to `crates/brush-app/**`): Guide UI development.
    *   `reconstruction-dev.mdc` (applied to core algorithm crates): Guide algorithm development.
    *   *...(other goal-specific rules)*
2.  **Custom Modes (Beta - If Enabled/Suitable):** Compose modes that bundle specific rules, prompts, or even model settings for different phases or tasks. Examples:
    *   **`Plan`:** For outlining structure or features.
    *   **`Documentation`:** For general doc writing/editing.
    *   **`Git`:** For Git operations.
    *   **`Brush Client` / `Brush Web`:** For specific development tasks.
*(Using modes helps make the current context and constraints explicit)*

**Workflow & Interaction:**

*   **Mode/Rule Activation:** Before starting a task, ensure the appropriate Cursor Mode (if used) or relevant Project Rules are active/applicable.
*   **Task Definition:** User clearly defines the goal, target file(s), and relevant **audience considerations**.
*   **Context Gathering & Confirmation:** AI identifies context needed (code, findings, `/old-docs` snippets, plan). AI explicitly states context used. **AI Asks** for confirmation if context seems insufficient or potentially ambiguous *despite* active rules.
*   **Planning & Clarification:** For new sections, AI proposes an outline. **AI Asks** clarifying questions if the request conflicts with active rules or lacks detail.
*   **Execution (Small Steps):** AI performs actions incrementally, respecting active rules (e.g., grounding claims, using correct commit format). Use targeted `edit_file`.
*   **Verification & User Feedback:** AI presents results. **AI Waits** for user review and confirmation ("Does this draft align with the 'Researcher' persona rule?", "Does this commit command use the correct format?", "Ready to proceed?") before committing or moving on.
*   **Progress Tracking:** Primary tracking via frequent **local Git commits** initiated by user or AI (with user confirmation). Push to remote only when requested by the user. Commit messages describe the task and phase.
*   **Context Management:** Focus strictly on task-relevant context. Rely on Rules/Modes for persistent instructions rather than chat history recall.

**Adaptability:** This strategy, including the specific Rules/Modes, will be reviewed and refined based on Phase 0 findings and practical experience during the rebuild.

*   **Tool Usage Considerations:**
    *   **Terminal Timeouts:** Commands involving potentially long compilations (`cargo run --release`) may time out before output can be captured. Prefer building first (potentially manually or backgrounded) then running the compiled binary directly for output capture. If direct verification fails repeatedly, state assumptions clearly and mark for manual verification.
    *   **File Edit Review:** While `.mdc` rules must be applied manually, even `edit_file` operations on standard `.md` files should be carefully reviewed by the user, as the application of the edit can sometimes be inaccurate or incomplete.
    *   **Context Preservation:** Before deleting planning or audit documents, ensure all relevant context (including rationale, constraints, inspirations) has been explicitly migrated to persistent documentation (`docs-process/` files) or is deemed no longer necessary. Consider delaying deletion until the relevant project phase is complete. 