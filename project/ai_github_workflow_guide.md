# GitHub Project Management Plan for Brush Core Development

**For AI Agent Use with Cursor**

This document provides a structured plan for an AI agent using Cursor to manage and execute the core development tasks for the Brush project using the GitHub API. The AI agent should use this plan to create and manage GitHub Project Issues and Milestones. This plan is tailored for a solo developer working in conjunction with an AI agent.

> **Note:** This document specifically focuses on GitHub project management. For the overall development strategy and technical details, refer to the main [brush_development_plan.md](/project/brush_development_plan.md).

## I. Project Tab and Configuration

* **GitHub Projects Version:** Use GitHub Projects (newer version) for this project.
* **Create a Project:** Create a GitHub Project named "Brush Core Development."
* **Views Configuration:**
  * **Board View:** Set up with the following columns:
    * To Do: Issues that are ready to be worked on.
    * In Progress: Issues currently being implemented.
    * Review/Testing: Issues that have PRs created and need review.
    * Done: Issues that are completed and merged.
  * **Table View:** Configure with custom fields:
    * Stack Rank: Number field for prioritization (lower = higher priority)
    * Risk Level: Single select (Low, Medium, High)
    * Complexity: Single select (Simple, Moderate, Complex) - to be estimated based on Claude 3.7 Sonnet MAX capabilities
* **Automation Rules:**
  * Move Issues to "In Progress" when work begins
  * Move Issues to "Review/Testing" when a Pull Request is created
  * Move Issues to "Done" when the linked Pull Request is merged

> **Important Note on GitHub Projects:** All GitHub Projects operations must be performed via the GitHub API through Cursor agent mode. If any operation cannot be completed through the API, document the limitation and suggest an alternative approach. Do not suggest manual UI interactions.

## II. Issue Management

* **Issue Creation Responsibility:**
  * The AI agent will create and populate issues based on the development plan.
  * The AI should plan several steps ahead but assume details will need to be filled in along the way.
  * Create issues in batches related to specific milestones or development phases.

* **Issue Structure:**
  * Title: Concise task description (e.g., "Implement movable UI panels")
  * Description: Extremely detailed task description, including:
    * Phase and related PRD: (e.g., "Phase 3 of Core Enhancements PRD")
    * Context: Provide background and rationale.
    * Objective: What should the code achieve?
    * Technical Details:
      * Specific files to modify.
      * APIs or libraries to use (e.g., EGUI Window struct).
      * Code snippets or examples.
      * Error handling requirements.
      * Performance considerations.
    * Dependencies: List any dependent Issues or tasks.
    * Assumptions: Explicitly state any assumptions.
    * Out-of-Scope: Clearly define what is NOT part of this task.
    * Risk Assessment: Identify potential risks and their mitigations, using the following format:
      ```
      ### Risk Assessment
      - **Risk Level**: [Low/Medium/High]
      - **Affected Components**: [List of components]
      - **Potential Regressions**: [Description]
      - **Mitigation Strategy**: [Steps to mitigate risks]
      ```
    * Testing:
      * Manual Testing Steps: Detailed steps for manual testing.
      * Automated Testing: Specify what automated tests are needed (unit, UI, integration).
      * Regression Testing: Describe which existing tests should be run to ensure no functionality is broken.
    * Acceptance Criteria:
      * Very specific, measurable criteria for when the Issue is complete.
      * Example: "Movable panels can be dragged and dropped. Panels maintain their position after application restart. No performance degradation."
    * AI Agent Instructions:
      * Direct instructions for the AI agent.
      * Example: "Use the EGUI Window struct to create movable panels. Ensure existing dockable panel functionality is preserved. Write a unit test to verify panel movement and position persistence."
  * Labels: (See Labels section)
  * Priority: high, medium, low
  * Stack Rank: Integer (1 being highest priority)
  * Complexity: Simple, Moderate, Complex

* **Prioritization:**
  * Work on Issues in order of their `Stack Rank`, starting with the lowest numerical value.
  * If multiple Issues have the same rank, address them in order of creation date.

## III. Labels and Organization

* **Type Labels:**
  * `enhancement`: New features or improvements
  * `bug`: Bug fixes
  * `documentation`: Documentation changes
  * `refactor`: Code restructuring without functional changes

* **Area Labels:**
  * `UI`: User interface components
  * `data`: Data management and storage
  * `pipeline`: Reconstruction pipeline
  * `testing`: Test infrastructure and test cases
  * `cross-platform`: Platform-specific considerations

* **Risk Labels:**
  * `risk:low`: Minimal risk of regressions
  * `risk:medium`: Moderate risk, contained to specific areas
  * `risk:high`: High risk, potentially affecting multiple components

## IV. Milestones and Planning

* **Milestone Strategy:**
  * Use Milestones to group related changes that will be pushed upstream together in a single Pull Request.
  * Each Milestone should result in one coherent PR.
  * Assign each Issue to a Milestone.

* **Milestone Structure:**
  * Name: Descriptive name for the group of changes
  * Description: Details the overall goal and scope
  * Due Date: Target completion date (optional)

* **Planning Ahead:**
  * The AI should create milestones for upcoming work based on the development plan
  * Create detailed issues for the current milestone, with more general issues for future milestones
  * Refine future milestone issues as they come closer to implementation

## V. Pull Requests and Reviews

* **PR Creation:**
  * Create PRs for each completed Milestone
  * Link all relevant issues to the PR
  * Ensure all code changes related to the issues are included

* **PR Content:**
  * Description summarizing all changes
  * Links to relevant issues and milestone
  * Testing details (what was tested and how)
  * Screenshots or demonstrations if UI-related
  * Known limitations or future work

* **Self-Review Process:**
  * AI agent must self-review all code before submitting
  * Use Google and Rust project standards for review criteria
  * Run all tests, including regression tests, before submission
  * Document risks and their mitigations
  * Document any performance considerations

* **Upstream PR Preparation:**
  * PRs intended for upstream contribution must be:
    * Small in scope
    * Well-documented
    * Thoroughly tested
    * Follow upstream project conventions

* **IMPORTANT: Never Submit PRs Automatically**
  * The AI agent must NEVER submit PRs automatically.
  * Always propose PR content to the developer for review.
  * Wait for explicit approval before proceeding with any PR creation.

## VI. Definition of Done

Before considering an issue complete, ensure all of the following criteria are met:

1. **Code Implementation:**
   * All code is written according to project standards
   * The implementation meets all requirements in the issue description
   * Code has been self-reviewed by the AI agent

2. **Testing:**
   * All existing tests still pass (regression testing)
   * New tests have been added for new functionality
   * Manual testing steps have been executed and documented

3. **Documentation:**
   * Code comments are clear and follow project standards
   * The `/docs/` directory has been updated to reflect changes
   * User-facing changes are documented appropriately

4. **Review:**
   * The AI agent has conducted a thorough self-review
   * All feedback has been addressed

5. **Pull Request:**
   * PR is created and linked to appropriate milestone
   * PR description includes all relevant information

## VII. AI Agent Workflow

1. **Issue Selection and Planning:**
   * AI agent retrieves the highest priority Issue (lowest `Stack Rank`) from the "To Do" column.
   * AI agent reads the Issue description, labels, and assigned Milestone thoroughly.
   * AI agent plans all code changes before making them.
   * AI agent critiques the plan for ambiguities or unknowns.
   * AI agent asks the developer for clarification on ambiguous points before proceeding.

2. **Implementation:**
   * AI agent moves the Issue to "In Progress" when work begins.
   * AI agent implements the required changes according to the plan.
   * AI agent writes appropriate tests for the changes.
   * AI agent proposes documentation updates for review.

3. **Testing and Validation:**
   * AI agent runs tests locally to verify the implementation.
   * AI agent performs regression testing to ensure no functionality is broken.
   * AI agent documents test results and any issues encountered.

4. **Pull Request Management:**
   * Once all Issues for a Milestone are completed, the AI agent creates a Pull Request.
   * AI agent links the Pull Request to the Milestone and relevant Issues.
   * AI agent moves the Issues to the "Review/Testing" column.
   * After the PR is approved and merged, the AI agent moves the Issues to the "Done" column.

5. **Documentation Updates:**
   * The AI agent should propose a summary of documentation changes first for developer review.
   * Only after explicit approval should the AI update documentation files or create documentation PRs.
   * The `/docs/` directory should reflect the current code as of that checkpoint.
   * The `/project/` directory should be updated as the project moves along to track what's current vs. upcoming.

## VIII. GitHub API Access and Automation

* **API Access:**
  * The AI agent has direct API access to GitHub via Cursor agent mode.
  * The AI agent should use this access to create and update issues, manage projects, and create pull requests.

* **Limitations and Workarounds:**
  * The AI should be aware of the 25 tool call chat limit in Cursor.
  * For longer tasks, the AI should break work into logical sessions.
  * The AI should document its progress at the end of each session to enable easy resumption.

## IX. Communication and Feedback

* **Clarification Process:**
  * The AI should plan for all code changes before making them.
  * The AI should critique its own plan and identify potential ambiguities.
  * The AI should ask the developer for clarification on unclear points before proceeding.

* **Progress Updates:**
  * Provide status updates at logical points during implementation.
  * Document what has been completed and what remains to be done.
  * Highlight any challenges or unexpected issues encountered.

* **Availability Considerations:**
  * The developer works on this project ad-hoc whenever time permits.
  * The AI should document its work thoroughly to enable easy resumption.
  * Issues should be self-contained enough to allow for intermittent work patterns.

## X. Commit and Code Standards

* **Commit Message Format:**
  ```
  [Type] [Scope]: [Description]

  [Optional body with more details]

  [Optional footer with issue references, etc.]
  ```

* **Commit Types:**
  * `feat`: New feature
  * `fix`: Bug fix
  * `docs`: Documentation change
  * `style`: Code style change
  * `refactor`: Code refactoring
  * `perf`: Performance improvement
  * `test`: Test-related change
  * `build`: Build system change
  * `ci`: CI configuration change
  * `chore`: Maintenance tasks

* **Code Standards:**
  * Follow Google and Rust project best practices.
  * Use standard Rust formatting (rustfmt).
  * Document public APIs and functions thoroughly.
  * Include appropriate error handling.
  * Ensure cross-platform compatibility.

## XI. Chat Session Planning

Due to the 25 tool call limit in Cursor, the AI agent should structure work into planned chat sessions with specific goals:

1. **Planning Session:**
   * Creating issues and milestones
   * Initial project setup
   * Roadmap planning
   * Breaking down large tasks into smaller issues

2. **Implementation Session:**
   * Coding specific features
   * Running tests
   * Fixing bugs
   * Local debugging

3. **Documentation Session:**
   * Updating technical documentation
   * Creating usage guides
   * Documenting API changes
   * Reviewing and updating project documentation

At the end of each session, the AI should:
* Summarize what was accomplished
* Document the current state for easy resumption
* List what's planned for the next session
* Save any important context or decisions

This approach ensures that progress can be made consistently despite the tool call limitation.

---

**TODO:** Follow up on learning transfer documentation strategy in a separate chat session.

