# Documentation Process Notes: Learnings & AI Collaboration Strategy

## Summary of Past Documentation Generation Issues (Phase 0 Learnings)

This section summarizes key hypothesized root causes identified during the postmortem of the *initial* AI-assisted documentation attempt. Understanding these helps inform better processes moving forward.

**Key Themes:**

1.  **Error Compounding:** Initial factual errors, if missed, were propagated and amplified in subsequent generation steps when the incorrect information became part of the AI's context.
    *   *Mitigation:* Rigorous verification against code, incremental steps, treating prior generated text with skepticism.
2.  **Lack of Grounding:** AI-generated claims (especially technical specifics like performance numbers or struct sizes) were sometimes not sufficiently based *exclusively* on the provided codebase context, potentially drawing from general knowledge or external (unverified) sources.
    *   *Mitigation:* Explicitly instruct AI to ground claims in provided code, verify existence of symbols/paths, state when information cannot be verified from context (`documentation.mdc`).
3.  **Interpretation Gaps:** Complex or niche code (Rust, WGSL, specific frameworks like Burn) might be misinterpreted, or claimed verification steps might be insufficient.
    *   *Mitigation:* Focus on describing verifiable behavior, use code snippets, ask clarifying questions for complex areas.
4.  **Context Blurring:** Difficulty distinguishing the hierarchy of truthfulness between sources (code vs. old docs vs. comments vs. external info).
    *   *Mitigation:* Prioritize code context, use audit findings cautiously, clearly define source priority.
5.  **Reading Future Plans:** Presenting planned features (from TODOs, comments, etc.) as existing functionality.
    *   *Mitigation:* Document only existing, verifiable functionality (`documentation.mdc`).
6.  **Context Limitations:** Finite context windows and imperfect recall mean constraints, workarounds, or prior instructions can be forgotten if not present in the current context or reinforced by rules.
    *   *Mitigation:* Use persistent rules (`.cursor/rules`), keep prompts focused, don't assume recall of distant conversation history, re-state critical constraints when necessary.

These learnings informed the rules (`cursor-rules.mdc`, `documentation.mdc`, etc.) and the structured, grounded approach used in the subsequent documentation rebuild phases.

---

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