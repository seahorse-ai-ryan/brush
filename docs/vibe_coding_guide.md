# Vibe Coder's Guide to Contributing to Brush

*For AI Agents & Human Developers*

```json
{
  "document_type": "contribution_guide",
  "project_name": "Brush",
  "version": "1.0",
  "target_audience": ["agentic_ai_assistants", "human_developers"],
  "min_capability_level": "advanced_reasoning",
  "key_technologies": ["rust", "wgpu", "egui", "burn", "wasm", "trunk", "tokio"],
  "priority_tasks": ["ui_improvements", "cross_platform_compatibility", "user_experience"],
  "last_updated": "2024-03-07"
}
```

> **IMPORTANT NOTE**: This guide is specifically designed for agentic AI assistants with advanced reasoning capabilities and may not be appropriate for simpler LLMs. The instructions assume the ability to maintain context, execute multi-step reasoning, and perform complex code analysis.

## Welcome

Welcome to contributing to Brush, an open-source project based on [ArthurBrussee/brush](https://github.com/ArthurBrussee/brush)!

This document is designed to guide both human "vibe coders" and AI coding agents in contributing to the Brush application. Brush is focused on making 3D neural reconstruction accessible to everyone, building upon the foundation laid by the original brush project. This guide will provide context, point you to helpful resources, and outline best practices for development within this project, even if you are working in a forked repository.

## Project Context

Brush is a small project with a limited number of developers, as described in the Product Requirements Document (PRD). This means:

- Changes can have significant impact across the codebase
- There may be less extensive documentation than in larger projects
- You have the opportunity to make meaningful contributions that directly shape the project's direction
- Communication and coordination with the small team is essential

## Understanding Brush and its Goals

Brush is a unique application leveraging cutting-edge technologies to perform 3D Gaussian Splat reconstruction. Key features and goals to keep in mind:

### Accessibility
- Brush aims to democratize 3D reconstruction, making it usable by individuals beyond developers and researchers
- A primary focus of current development (Brush 1.0) is on improving user experience and simplifying the workflow for casual users

### Cross-Platform Compatibility
Brush is designed to run on a variety of platforms:
- **Desktop**: Windows, macOS, Linux
- **Web**: In modern web browsers (thanks to WASM)
- **Mobile** (In Progress): Android and iOS

### Cutting-Edge Technology
Brush utilizes a modern and somewhat experimental technology stack, including:

- **Rust**: For performance and reliability - [rust-lang.org](https://www.rust-lang.org/)
  - Core programming language providing performance, safety, and cross-platform support
- **WGPU**: Low-level graphics API abstraction - [github.com/gfx-rs/wgpu](https://github.com/gfx-rs/wgpu)
  - Enables hardware-accelerated graphics rendering, crucial for 3D
- **egui**: Immediate mode GUI library - [github.com/emilk/egui](https://github.com/emilk/egui)
  - Used for creating the application's user interface in Rust
- **Burn**: Machine learning framework - [github.com/burn-rs/burn](https://github.com/burn-rs/burn)
  - A flexible and performant deep learning framework written in Rust
- **WebAssembly (WASM)**: Compilation target for web deployment - [webassembly.org](https://webassembly.org/)
  - Allows running the Rust-based application in web browsers
- **Trunk**: Build tool for bundling Rust/WASM web applications - [trunkrs.dev](https://trunkrs.dev/)
  - Simplifies building and deploying Rust applications to the web
- **tokio**: Asynchronous runtime - [tokio.rs](https://tokio.rs/)
  - Enables efficient handling of asynchronous tasks and I/O in Rust

## Essential Project Resources

### Documentation
Crucially, please begin by thoroughly reviewing the markdown documentation located in the `/docs/` folder of the Brush repository. These documents provide essential insights into:

- **Brush Architecture**: Understand the application's structure and component interactions
- **3D Reconstruction Pipeline**: Learn how Brush processes images to generate 3D models
- **Rendering Process**: Study the logic for rendering 3D Gaussian Splats in real-time, including live training updates
- **Technical Guides**: Find links to documentation for Rust, Burn, WASM, and other technologies used in the project

### Product Requirements Document (PRD)
Refer to the Product Requirements Document for Brush 1.0. Note that the PRD outlines both current goals for Brush 1.0 and aspirational future functionality. Focus on the requirements specifically marked for the 1.0 release when contributing.

## Guidance for AI Agents & Vibe Coders

To provide the most helpful assistance, please begin by asking the user about their development experience. For example:

> "To best assist you, could you please tell me about your development experience level? Are you a:
> 
> - Research PhD with ML/3D graphics expertise?
> - Experienced software engineer with Rust/WASM familiarity?
> - Intern or student learning these technologies?
> - 'Vibe coder' contributing to open source and exploring new technologies?
> 
> Knowing your background will help me tailor my responses to be the most relevant and helpful for you."

## AI Agent-Specific Guidance

### Repository Synchronization Check

```yaml
# AI AGENT INSTRUCTION: REPOSITORY SYNC CHECK
trigger_condition: "developer hasn't mentioned syncing to public Brush repository recently"
priority: high
action_type: proactive_suggestion
steps:
  - ask_if_user_wants_to_check_for_updates: true
  - if_user_agrees:
      - check_remote_configuration
      - add_upstream_remote_if_needed
      - fetch_latest_changes
      - merge_or_rebase_as_appropriate
  - after_sync:
      - compile_application
      - test_application
      - verify_functionality
```

### Checkpoint Recommendations

```yaml
# AI AGENT INSTRUCTION: CHECKPOINT RECOMMENDATION
trigger_condition: "after several successful edits OR significant wall clock time elapsed"
priority: medium
action_type: proactive_suggestion
steps:
  - suggest_saving_checkpoint: true
  - if_user_agrees:
      - commit_changes_with_descriptive_message
      - push_to_remote_if_appropriate
      - verify_application_builds_and_runs
  - remind_about_benefits:
      - prevent_loss_of_work
      - easier_to_identify_when_issues_were_introduced
```

### Learning from Mistakes

```yaml
# AI AGENT INSTRUCTION: DOCUMENT LESSONS LEARNED
trigger_condition: "after resolving non-obvious bugs or errors"
qualifying_error_types:
  - required_multiple_iterations_to_fix
  - produced_significant_compiler_errors
  - involved_rust_type_system_ownership_borrowing_issues
  - related_to_platform_specific_behavior
priority: medium
action_type: proactive_suggestion
steps:
  - ask_user_about_documenting_lesson: true
  - if_user_agrees:
      - create_or_update_file: "/docs/ai_agent_lessons_learned.md"
      - include_metadata:
          - timestamp
          - agent_name_and_version
          - intended_change_description
          - error_summary
          - better_approach
      - insert_at_top_of_file: true
      - keep_entry_concise: true
```

### Error Pattern Recognition

```yaml
# AI AGENT INSTRUCTION: ERROR PATTERN RECOGNITION
trigger_condition: "encountering compiler or runtime errors"
priority: high
action_type: analysis
steps:
  - categorize_error_type:
      - ownership_borrowing
      - type_mismatch
      - lifetime_issues
      - platform_specific
      - dependency_related
  - check_against_known_patterns_in_lessons_learned
  - suggest_solution_based_on_pattern_matching
  - explain_underlying_concept_to_user
```

## Technical Focus Areas

When contributing to Brush, please pay special attention to these core areas:

### Machine Learning & 3D Reconstruction Logic
This is the heart of Brush's unique capability. Understand how Burn is used to train models from image datasets and generate 3D Gaussian Splats.

**Key Point**: Brush's ability to perform ML efficiently even without an NVIDIA GPU (using WASM and supporting different hardware backends) is a core strength.

Investigate the code related to:
- Data loading and preprocessing
- Gaussian Splat training algorithms (likely within the burn framework integration)
- Parameter configuration for training

### 3D Gaussian Splat Rendering
The real-time rendering of Gaussian Splats, especially during live training updates, is a potentially complex and performance-sensitive area. Exercise caution and thorough testing when modifying rendering code.

Focus on code related to:
- Loading and processing .ply splat files
- Real-time rendering pipelines (likely using a graphics library compatible with Rust/WASM)
- Optimization for performance across different platforms

## Development Best Practices

### Iterative Development & Frequent Testing
Break down large changes into smaller, manageable chunks. After each code modification, always compile and run the application to ensure your changes are working as expected and haven't introduced regressions.

### Cross-Platform Testing
Brush is designed to be cross-platform. Periodically test your changes on different target platforms (Desktop, Web at a minimum) to confirm functionality and avoid platform-specific issues. While Android and iOS are in progress, consider their constraints (especially resource limitations) as well.

### Refer to Technical Guides
Brush uses relatively new technologies. Don't hesitate to consult the linked documentation for Rust, Burn, WASM, and other components in the `/docs/` folder to deepen your understanding.

## Modular UI & Pipeline

Keep in mind Brush's intended modular architecture:

### UI/UX Focus for Brush 1.0
The current focus is on enhancing the User Interface and User Experience. When contributing UI code, strive for clarity, intuitiveness, and ease of use, especially for casual users.

### Pipeline Modularity
The underlying processing pipeline is designed to be modular. Aim to maintain this modularity when making changes, which will facilitate future extensibility and allow for swapping out components without breaking the overall application. Ensure any changes to the pipeline still support compatible input formats and produce valid .ply Gaussian Splat outputs to maintain rendering compatibility.

## Common Pitfalls and Solutions

| Pitfall | Symptoms | Solution |
|---------|----------|----------|
| Ownership issues with GPU resources | Compiler errors about moved values | Use appropriate lifetime parameters or Arc/Rc for shared ownership |
| Cross-platform UI inconsistencies | Different appearance or behavior across platforms | Test on multiple platforms early and use platform-agnostic egui features |
| Performance degradation | Slow rendering or training | Profile with appropriate tools and optimize hotspots |
| Memory leaks with WASM | Growing memory usage in browser | Ensure proper cleanup of resources, especially with WebGL contexts |
| Dependency version conflicts | Build failures with cryptic errors | Use cargo-tree to identify conflicts and specify compatible versions |

---

By following this guide, consulting the documentation, and adopting an iterative development approach with thorough testing, you can effectively contribute to the Brush project and help make 3D neural reconstruction accessible to everyone!

Thank you for your contributions!