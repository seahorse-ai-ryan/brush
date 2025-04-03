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

This outlines the general approach for how the AI agent (Gemini 2.5 Pro/Max) and the User will collaborate during the documentation rebuild process. This strategy prioritizes accuracy, clarity, and efficient use of context, and may be refined after Phase 0.

**Core Principles:**

*   **User-Guided:** The user directs the overall process, selects tasks, provides goals, and makes final decisions.
*   **AI as Co-Creator & Verifier:** The AI assists in generating drafts, applying specific corrections, finding information, and verifying claims against provided context (code, findings).
*   **Incremental & Verifiable:** We will favor smaller, manageable steps with explicit verification over large, monolithic changes.

**Workflow & Interaction:**

1.  **Task Definition:** The user clearly defines the goal for the current task (e.g., "Sanitize file X according to findings Y," "Draft the 'Installation' section for the Enthusiast persona," "Find code relevant to claim Z").
2.  **Context Gathering & Confirmation:**
    *   AI identifies the necessary context (specific parts of audit findings, sections of `/old-docs`, code snippets via file reads or search, this plan).
    *   AI explicitly states the context it intends to use.
    *   **AI Asks:** "Is this the correct/sufficient context, or should I include anything else?" before proceeding with generation or complex analysis.
3.  **Planning & Clarification:**
    *   For non-trivial generation tasks (e.g., a new page/section), AI proposes a brief outline or plan first.
    *   **AI Asks:** If a request is ambiguous or requires technical assumptions beyond the provided context, AI asks clarifying questions *before* generating content or code edits.
4.  **Execution (Small Steps):**
    *   AI breaks down larger generation tasks into logical sub-sections.
    *   AI performs actions (reading, searching, drafting, editing) one manageable step at a time.
    *   Use targeted `edit_file` for applying changes rather than outputting large blocks in chat. State clearly *what* change is being proposed.
5.  **Verification & User Feedback:**
    *   After each significant step (generating a draft section, applying a set of corrections), AI presents the result (e.g., via `edit_file` diff).
    *   **AI Waits:** AI explicitly waits for user review and confirmation ("Does this look correct?", "Ready for the next step?") before proceeding with further edits, committing, or moving to the next task.
6.  **Progress Tracking:**
    *   Primary tracking mechanism is **Git commits**. User or AI initiates commits after logical units of work are completed and verified by the user. Commit messages should clearly describe the completed task/change.
    *   Avoid adding progress-tracking comments directly into the documentation files.
    *   Use temporary scratchpad files sparingly, only for complex intermediate notes explicitly agreed upon, not as a primary progress log.
7.  **Context Management:**
    *   Focus strictly on the context needed for the immediate task. Avoid loading entire directories (`/old-docs`, codebase) unless necessary and explicitly requested.
    *   After completing a task on one file/section and before starting another, mentally (or explicitly state) "dropping" the previous specific file context to keep the window clean.

**Adaptability:** This is a starting point. We will reflect during/after Phase 0 on what worked well or poorly in the initial documentation attempt and refine this collaboration strategy accordingly. 