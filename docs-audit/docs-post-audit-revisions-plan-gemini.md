# Brush Documentation Revision Plan (Strategy: Start Fresh)

## Introduction

This plan outlines the steps to rebuild the Brush project documentation from a clean slate. The previous documentation, while containing some useful elements, suffered from significant inaccuracies and structural issues, as detailed in the maintainer feedback ([Audit Plan](./docs-ai-agent-audit-plan.md)) and the technical audit ([Audit Findings](./docs-audit-findings-gemini.md)).

This "Start Fresh" approach aims to create documentation that is fundamentally useful, trustworthy, and beneficial for the target audience (software developers, 3D researchers, enthusiasts).

The core principles guiding this rebuild are: ensuring **technical accuracy** based on the audit findings, designing a **user-persona-centric structure**, clearly explaining the **"Why"** behind design decisions, providing **concrete examples**, and establishing overall **clarity and trustworthiness**.

## Revision Phases

The revision process will proceed in the following phases:

### Phase 0: Root Cause Analysis & Process Improvement

*   **Action:** Hypothesize *why* the initial documentation generation resulted in numerous factual errors and structural issues. Consider factors like:
    *   Over-reliance on LLM generative capabilities without sufficient grounding/verification.
    *   Potential issues in initial prompting or goal-setting (e.g., requesting verbosity over clarity, not emphasizing the "Why").
    *   Assumptions made about the LLM's understanding of the niche domain (Rust + ML frameworks) and specific codebase details.
    *   Lack of iterative verification during the initial generation.
*   **Goal:** Identify lessons learned and define improved AI collaboration patterns (prompting techniques, verification steps, context management) to avoid repeating these mistakes during the rebuild process.
*   **Action:** Define and implement relevant Cursor Rules and/or Modes based on Phase 0 findings to enforce grounding, code priority, workarounds, and persona focus, reducing manual oversight and potential user error.

### Phase 1: Sanitize Existing Docs & Setup New Structure

*   **Action:** **Fix Factual Errors in Current `/docs`:** Go through the *current* `/docs` directory and fix *only* the specific factual errors identified in the [Audit Findings](./docs-audit-findings-gemini.md). This ensures the archived version isn't actively misleading if referenced. Do not spend time restructuring or rewriting `/docs` content beyond these specific factual fixes.
*   **Action:** Rename the existing, sanitized `/docs` directory to `/old-docs`.
*   **Action:** Create a new, empty `/docs` directory.
*   **Action (Future Consideration):** Once the new documentation is sufficiently mature, consider moving the `docs-audit` directory outside the main project workspace to prevent the detailed findings from confusing future context windows during new content generation.

### Phase 2: Structure Planning (Persona-Driven)

*   **Action:** Define the new information architecture for the `/docs` directory based on key user personas:
    *   **3D Enthusiast:** Needs clear getting started guides (binaries, demos, simple builds), basic usage, cool examples.
    *   **3D Researcher:** Needs deep dives on algorithms (reconstruction, rendering), framework understanding for modification/benchmarking, contribution guides for core tech.
    *   **Software Engineer:** Needs contribution guides (features, UX, design), examples of using Brush crates as a library, architectural overview for integration/extension (new apps, backends, AR/VR, etc.).
*   **Action:** Create placeholder files and the directory structure within the new `/docs` reflecting this persona-based organization.

### Phase 3: Content Generation (Iterative & Persona-Focused)

*   **Action:** Select a section/persona to focus on (e.g., Getting Started for Enthusiasts).
*   **Action:** Generate new content specifically for that section and persona:
    *   **Reference [Audit Findings](./docs-audit-findings-gemini.md):** Use the findings as a guide for required accuracy points and as an "anti-pattern" list (i.e., explicitly avoid the types of errors found previously).
    *   **Reference Sanitized `/old-docs` Sparingly:** Only consult `/old-docs` for specific, *verified* factual snippets (e.g., a correctly stated default parameter value) if needed. Do *not* copy structure or unclear prose.
    *   **Focus:** Write new content tailored to the target persona, emphasizing their goals, the "Why" behind concepts, clear explanations, and practical, runnable examples.
    *   **Apply Lessons:** Actively apply insights and improved collaboration patterns identified in Phase 0.

### Phase 4: Review & Refinement

*   **Action:** Continuously review generated content for technical accuracy (cross-referencing code/audit findings), clarity, conciseness, and alignment with the persona's needs.
*   **Action:** Refine iteratively based on review feedback.

## AI Collaboration Strategy (Co-Creation within Cursor)

This outlines the general approach for how the AI agent (Gemini 2.5 Pro/Max) and the User will collaborate during the documentation rebuild process. This strategy leverages **Cursor Project Rules** and potentially **Custom Modes** (beta) to prioritize accuracy, clarity, persona alignment, and efficient use of context, aiming to mitigate issues identified in Phase 0.

**Core Principles:**

*   **User-Guided:** The user directs the overall process, selects tasks, provides goals, and makes final decisions.
*   **AI as Co-Creator & Verifier:** The AI assists in generating drafts, applying specific corrections, finding information, and verifying claims against provided context (code, findings), guided by defined rules.
*   **Incremental & Verifiable:** We will favor smaller, manageable steps with explicit verification over large, monolithic changes.
*   **Rule-Driven Consistency:** Utilize Cursor's features to enforce best practices and reduce repetitive prompting or user error.

**Leveraging Cursor Features (Setup during Phase 0/1):**

1.  **Project Rules (`.cursor/rules`):** Define persistent instructions triggered by file paths or semantics. Examples:
    *   `accuracy.rule`: "Base all factual claims *only* on provided code snippets or explicitly cited `/old-docs` snippets. State inability to verify if context is missing."
    *   `code_priority.rule`: "When context includes both code and documentation, prioritize code as the source of truth for technical details."
    *   `git_commit.rule`: "When using the terminal tool for multi-line git commit messages, *always* use the `printf '.' | git commit -F -` format."
    *   `persona_researcher.rule` (applied to `/docs/for-researchers/*`): "Tailor language and depth for 3D graphics researchers. Explain algorithms and link to relevant code sections."
    *   `persona_engineer.rule` (applied to `/docs/for-engineers/*`): "Focus on API usage, contribution guidelines, and system architecture for software engineers."
    *   `persona_enthusiast.rule` (applied to `/docs/for-enthusiasts/*`): "Prioritize clear setup, basic usage, and engaging examples for users exploring Brush."
    *   `sanitization.rule` (applied to `/old-docs/*` during Phase 1): "Focus *only* on correcting specific factual errors listed in `docs-audit-findings-gemini.md`. Do not make stylistic changes or add new content."
2.  **Custom Modes (Beta - If Enabled/Suitable):** Compose modes that bundle specific rules, prompts, or even model settings for different phases or tasks. Examples:
    *   **"Sanitize Mode":** Activates `sanitization.rule` and `accuracy.rule`.
    *   **"DocGen - Researcher Mode":** Activates `accuracy.rule`, `code_priority.rule`, `persona_researcher.rule`, maybe a default prompt like "Drafting content for 3D researchers...".
    *   **"DocGen - Engineer Mode":** Similar, but activates `persona_engineer.rule`.
    *   **"Git Helper Mode":** Activates `git_commit.rule`.
    *(Using modes could streamline switching contexts and associated instructions.)*

**Workflow & Interaction:**

*   **Mode/Rule Activation:** Before starting a task, ensure the appropriate Cursor Mode (if used) or relevant Project Rules are active/applicable.
*   **Task Definition:** User clearly defines the goal, target file(s), and relevant persona.
*   **Context Gathering & Confirmation:** AI identifies context needed (code, findings, `/old-docs` snippets, plan). AI explicitly states context used. **AI Asks** for confirmation if context seems insufficient or potentially ambiguous *despite* active rules.
*   **Planning & Clarification:** For new sections, AI proposes an outline. **AI Asks** clarifying questions if the request conflicts with active rules or lacks detail.
*   **Execution (Small Steps):** AI performs actions incrementally, respecting active rules (e.g., grounding claims, using correct commit format). Use targeted `edit_file`.
*   **Verification & User Feedback:** AI presents results. **AI Waits** for user review and confirmation ("Does this draft align with the 'Researcher' persona rule?", "Does this commit command use the correct format?", "Ready to proceed?") before committing or moving on.
*   **Progress Tracking:** Primary tracking via frequent **local Git commits** initiated by user or AI (with user confirmation). Push to remote only when requested by the user. Commit messages describe the task and phase.
*   **Context Management:** Focus strictly on task-relevant context. Rely on Rules/Modes for persistent instructions rather than chat history recall.

**Adaptability:** This strategy, including the specific Rules/Modes, will be reviewed and refined based on Phase 0 findings and practical experience during the rebuild. 