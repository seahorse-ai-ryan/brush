---

# Product Requirements Document: Brush 1.0 - Usability Enhancements for 3D Neural Reconstruction

**1. Introduction**

*   1.1. Purpose:  To define requirements for Brush 1.0 usability update.
*   1.2. Background: Briefly introduce the Burn project and the Brush application. Mention its unique capabilities (WASM, browser compatibility, etc.) and target audience (expanding to casual users).
*   1.3. Goals for Brush 1.0: State the primary goals of Brush 1.0 - improved usability, accessibility for casual users, focusing on UI/UX enhancements, not core technology changes. Mention the objective of being accepted into the main Brush repository.
*   1.4. Target Audience for this Document: Specify that this PRD is primarily for AI agents and human developers contributing to Brush 1.0.

**2. Goals and Objectives**

*   2.1. Primary Goal: To make 3D neural reconstruction with Brush accessible and user-friendly for a wider audience, including casual users, hobbyists, and educators.
*   2.2. Measurable Objectives:
    *   Acceptance of Brush 1.0 update into the main ArthurBrussee/brush repository.
    *   Increase in GitHub forks and stars.
    *   Increase in application downloads (if tracked).
    *   Positive community feedback on usability improvements.

**3. Target Users**

*   3.1. Primary Target User: Casual Users & Newcomers to 3D Reconstruction (Describe the primary target audience - those new to 3D reconstruction, hobbyists, educators, users without deep technical expertise. Emphasize the need for an intuitive and approachable UI.)
*   3.2. Secondary Target User: Advanced Users & Researchers (Acknowledge the existing user base of researchers and advanced users.  State that Brush 1.0 should still cater to their needs by providing access to advanced features and debug information, while not overwhelming casual users.)

**4. Features**

*   4.1. Getting Started & First Impressions
    *   4.1.1. Demo 3D Model on Launch: Display a default 3D model on first and subsequent launches. Example: 3D paintbrush Gaussian Splat.  Benefit: Immediate visual appeal and demonstration of capabilities.
    *   4.1.2. Sample Dataset on First Launch: Automatically load and highlight a sample dataset on first launch. Benefit:  Easy first-time experience, reduces initial friction.
    *   4.1.3. Improved Preset Usability: Make Presets directly runnable, visually improved, and more informative. Benefit: Quick access to example reconstructions.

*   4.2. Data Input & Dataset Management
    *   4.2.1. Zip File Upload: Allow users to upload zip files of images. Benefit: Simplified data input.
    *   4.2.2. Directory Dataset Referencing: Enable users to point to dataset directories. Benefit: Organized dataset management.
    *   4.2.3. Local Dataset List (JSON-based): Maintain a local JSON list of datasets with status. Benefit: Basic project/session persistence.
    *   4.2.4. Dataset Creation Guidance: In-app guidance on creating datasets. Benefit: Helps users create viable datasets for successful reconstruction.

*   4.3. Model Viewing & Input
    *   4.3.1. Open .ply Files (URL & Local Upload): Allow users to open and view existing 3D Gaussian Splat models in .ply format from URLs or local files. Benefit: View existing splats and share results easily.
    *   4.3.2. Clear Preview of Results (Live & Post-Training): Provide intuitive live and post-training 3D preview. Benefit: Visual feedback and result inspection.
    *   4.3.3. Easy Export Functionality (.ply): One-click export to .ply format. Benefit: Simple output generation.

*   4.4. Reconstruction Setup & Configuration
    *   4.4.1. Simplified User Interface: Describe the overall goal of simplifying the UI for setup.
    *   4.4.2. Collapsible Settings Pane: Collapsible settings. Benefit: Reduces initial overwhelm.
    *   4.4.3. Tooltips for Technical Terms: Tooltips explaining jargon. Benefit: Improved understanding for novice users.
    *   4.4.4. Simplified Terminology: User-friendly labels. Benefit: Easier comprehension.

*   4.5. Training Progress & Monitoring
    *   4.5.1. Live Training Rendering: Real-time 3D visualization during training. Benefit: Visual feedback on progress.
    *   4.5.2. Clear Status & Progress Indicators: User-friendly status and progress updates. Benefit: Clear understanding of training status.
    *   4.5.3. Optional Debug View: Collapsible/separate debug view for advanced users. Benefit:  Provides detailed info without cluttering UI for casual users.

*   4.6. Help & Guidance
    *   4.6.1. Help Menu: In-app Help Menu. Benefit: Centralized access to help resources.
    *   4.6.2. Version Information: Display version info in Help Menu. Benefit: Transparency and debugging info.
    *   4.6.3. Dataset Creation Guidance (in Help): Reiterate dataset guidance availability in Help Menu.
    *   4.6.4. Clear Error Handling and User Feedback: User-friendly error messages. Benefit: Improved troubleshooting and user experience.

*   4.7. User Interface & General UX Principles
    *   4.7.1. Prioritize Workflow & Layout First, Aesthetics Later: Describe this development principle.
    *   4.7.2. Modular UI Design: Describe this design principle.
    *   4.7.3. Overall UI Cleanliness and Intuitiveness: Describe this UX goal.
    *   4.7.4. Consistent Visual Language and Navigation: Describe this UX goal.
    *   4.7.5. Improved Information Hierarchy: Describe this UX goal.

*   4.8. Modularity & Extensibility (Pipeline)
    *   4.8.1. Modular Pipeline Design: Swappable pipeline components. Benefit: Extensibility and future-proofing.
    *   4.8.2. CLI Compatibility & Coexistence: UI complements CLI. Benefit: Consistent experience for different user types.

*   4.9. Session & Project Management (Basic)
    *   4.9.1. Local Dataset List (JSON): Reiterate as session management element.
    *   4.9.2. Basic Settings Persistence: Local saving of basic settings. Benefit: User convenience.

**5. Non-Functional Requirements**

*   5.1. Performance: Application should be responsive and performant on target hardware. Reconstruction times should be reasonable for typical datasets.
*   5.2. Usability: Application must be intuitive and easy to use for casual users, while remaining functional for advanced users.
*   5.3. Reliability: Application should be stable and reliable, with minimal crashes or errors.
*   5.4. Platform Compatibility: Desktop (Windows, macOS, Linux), Web Browser (WASM-compatible browsers). Android and iOS (in progress).
*   5.5. Technology Stack: Rust, Burn, WASM, Trunk, Tokio, .ply format.

**6. Future Considerations (Post-1.0)**

*   6.1. Reconstruction Presets (Quality/Speed)
*   6.2. In-app Tutorials and Walkthroughs
*   6.3. Broader Export Formats (.obj, .glb, etc.)
*   6.4. Asset Optimization Features (Compression, Decimation)
*   6.5. Cloud-Based Processing (Long-term vision)
*   6.6. Community Features (Sharing Models, etc.)

**7. Open Issues & Questions**

*   7.1.  Specific .ply Export Options?
*   7.2.  Detailed UI Mockups/Wireframes?
*   7.3.  User Feedback Integration Plan?
*   7.4.  Testing Strategy for 1.0?

---

**How to Save to a File:**

1.  **Select All and Copy:** Carefully select all the text within the code block above (including the `---` lines at the beginning and end) and copy it (usually Ctrl+C or Cmd+C).
2.  **Open a Text Editor:** Open a plain text editor on your computer (like Notepad on Windows, TextEdit on Mac - be sure to set it to "Plain Text" mode, or a code editor like VS Code, Sublime Text, Atom, etc.).
3.  **Paste:** Paste the copied text into the text editor (usually Ctrl+V or Cmd+V).
4.  **Save As:**  Save the file. Choose "Save As" and give it a filename like `Brush_1.0_PRD_Outline.md`.  The `.md` extension is for Markdown. If you just want plain text, you can save it with a `.txt` extension like `Brush_1.0_PRD_Outline.txt`.  **Important:** Make sure you save it as "Plain Text" or "All Files" type if you are using a basic text editor to ensure it doesn't add hidden formatting.

Now you have the PRD outline saved in a file that you can open, edit, and share! Let me know if you have any trouble saving it or if you'd like the outline in a different format.  We can then start filling in the details for each section of this outline.