# **Revitalizing the Brush Application with AI-Assisted Development: An Expert Analysis of the Project Plan**

# Brush Development Plan: AI-Assisted Revitalization Strategy

This document outlines a comprehensive development plan for revitalizing the Brush application - a cross-platform 3D reconstruction framework. The plan employs AI-assisted development techniques while incorporating lessons learned from previous attempts.

> **Note:** For details on GitHub project management, including issues, milestones, and pull requests, see [ai_github_workflow_guide.md](/project/ai_github_workflow_guide.md).

## Executive Summary

The development plan for the Brush application provides a well-structured approach to enhancing this cross-platform 3D reconstruction framework. The emphasis on learning from challenges encountered in prior attempts demonstrates a commitment to iterative development and a pragmatic understanding of potential pitfalls.

### Key Strengths

- Clear articulation of goals and project scope
- Prioritization of foundational improvements in documentation and development environment
- Strategic integration of AI within the Cursor IDE to improve development processes
- Comprehensive modularization strategy for improved maintainability and extensibility
- Modern UI approach with both dockable and movable windows
- Flexible data storage options for different platforms
- Creation of a modular reconstruction pipeline to enable algorithm integration and comparison
- Strong cross-platform support across desktop, web, and mobile environments
- Thorough incorporation of lessons learned from previous attempts

## Development Phases

### Phase 1: Documentation and Development Environment Setup

The initial phase prioritizes establishing a solid foundation through improved documentation and a streamlined development environment, both critical for effective AI agent integration and project success.

#### Documentation Tasks

- Review and refine human-readable documentation in the docs directory
- Ensure documentation covers current architecture, functionality, and planned changes
- Maintain consistency across all documents including existing ai_assisted_workflow.md
- Create comprehensive documentation for AI agents to understand code structure and patterns

#### AI-readable Cursor Rules

- Develop clear rules within the .cursor/rules directory specifically for AI agents
- Ensure rules are unambiguous, actionable, and aligned with human documentation
- Define coding standards, architectural patterns, and implementation preferences
- Focus on rules that minimize extensive code generation in favor of targeted assistance
- Research and apply best practices from resources like awesome-cursorrules

#### Development Environment Streamlining

- Create clear processes for launching MCP servers, Trunk web server, and desktop compilation
- Investigate methods to consolidate process management within Cursor IDE
- Explore development containers for consistent contributor environments
- Establish plan for Windows executable with RTX GPU support for testing

### Phase 2: Modularity from UI

This phase begins the modularization of the Brush application, starting with separating UI components from core logic.

#### Tasks

1. Analyze codebase to identify components with tightly coupled UI and core functionality
2. Define abstraction layers and interfaces for UI-backend communication
3. Implement initial refactoring of key components to use the new abstractions
4. Test thoroughly to ensure application functionality remains intact

### Phase 3: Panels and Windows UI Enhancements

This phase focuses on modernizing the user interface with the EGUI library while maintaining existing functionality.

#### Tasks

1. Implement movable UI panels using EGUI's Window struct
2. Maintain and integrate with the existing dockable panel system
3. Prioritize the main Scene render pane in the UI layout
4. Gather and incorporate user feedback on UI preferences

### Phase 4: Add Local Datasets

This phase enhances data management with local storage options for both desktop and web platforms.

#### Tasks

1. Implement file system storage for desktop application
2. Develop IndexedDB storage for web application
3. Create consistent APIs across both storage mechanisms
4. Test data operations for reliability and performance

### Phase 5: Add Cloud Datasets

This phase integrates cloud storage capabilities into the application.

#### Tasks

1. Set up Google Cloud backend with user authentication
2. Integrate Firebase for analytics and synchronization foundation
3. Implement UI for cloud dataset management
4. Develop plans for optional integration with other cloud providers

### Phase 6: Demonstrate Another UI App Using the Same Backends

This phase showcases the modularity of the architecture by creating a separate application using shared backend logic.

#### Tasks

1. Ensure backend interfaces are well-defined and stable
2. Develop a minimal UI application using the common backend
3. Test thoroughly to validate modularity goals

### Phase 7: Enhance Testing and Debugging

This phase establishes robust automated testing across all platforms with AI assistance.

#### Tasks

1. Develop comprehensive unit tests for core components
2. Implement UI testing strategies for all target platforms
3. Create clear testing workflows for AI agents
4. Provide efficient debugging processes across platforms
5. Explore AI-assisted test generation

### Phase 8: Modular Reconstruction Pipeline

This phase creates a flexible pipeline for integrating different reconstruction algorithms.

#### Tasks

1. Define interfaces for reconstruction pipeline stages
2. Implement abstraction layers for algorithm interchangeability
3. Integrate existing algorithm into the new pipeline architecture
4. Document process for researchers to integrate custom algorithms

### Phase 9: Further UI Enhancements and Refinements

This phase refines UI components based on feedback and implements additional features.

#### Tasks

1. Enhance UI for local directory selection
2. Implement cloud storage management interfaces
3. Add transcoding and remastering UI elements
4. Develop local/cloud processing selection interface
5. Continuously incorporate user feedback

## Key Clarifications

### Clarification on Development Approaches

#### Understanding "Vibe Coding"
The term "vibe coding" refers to a development approach coined by Andrej Karpathy, which involves using AI agents such as Claude 3.7 Sonnet in IDEs like Cursor to assist with coding tasks. It's important to note that:

1. **Not a Core Focus**: Vibe coding is not a central focus of the Brush project, but rather an optional approach that some contributors may choose to employ.
   
2. **Guidelines for Vibe Coding Contributions**: Any contributions resulting from vibe coding must be:
   - Small in scope
   - Well-documented
   - Carefully reviewed before integration into the main codebase
   
3. **Selective Application**: Documentation about vibe coding will be kept in separate, optional guides rather than being a central part of the project documentation, providing pointers for those interested in this approach.

### Cross-Platform Strategy Refinement

The project's cross-platform support is a key strength, but requires further investigation and planning in the following areas:

1. **Current Platform Support Assessment**: An initial task will be to investigate the current state of mobile platform support in the Brush application. Specifically, we need to determine whether Brush currently builds native iOS and Android applications, or if it simply provides web applications that function in Chrome on mobile devices.

2. **Testing Resources**: 
   - Available hardware for testing includes:
     - MacBook Pro for macOS/desktop development
     - Android Pixel phone
     - iOS iPad Pro
     - Potential access to Windows machine with RTX NVIDIA GPU
   
   - A thorough investigation of the main branch's current testing methodology will be conducted to understand existing test coverage and approaches.
   
   - Cloud service resources for automated testing will be evaluated, particularly for platforms that may not be available for local testing.

3. **Windows with RTX Support**: The plan will include specific steps for building a Windows executable with RTX GPU support for testing, potentially leveraging GitHub workflows for automation.

### Burn Framework Integration Clarification

The Brush application relies on the Burn machine learning framework, which is developed by a separate team. Key aspects of this relationship include:

1. **External Dependency**: Burn is an external library developed independently, requiring careful coordination when integrating updates.

2. **Bug Coordination**: Previous development efforts have encountered situations where Brush bugs were blocked by underlying issues in Burn. The Burn development team has been responsive to these reports.

3. **Documentation Task**: During Phase 1 (Documentation), a specific task will be added to thoroughly document the current integration between Brush and Burn, including:
   - How the reconstruction pipeline currently interfaces with Burn
   - Known limitations or dependencies
   - Communication channels with the Burn development team

4. **Modular Design Consideration**: The modular reconstruction pipeline design will need to account for Burn's architecture and interfaces, ensuring that different algorithms can be plugged in while maintaining compatibility with Burn's capabilities.

### Local vs. Cloud Processing Implementation

The plan for allowing users to choose between local and cloud processing needs further clarification:

1. **Default Approach**: The main Brush application will continue to assume local processing as the default, even for the web application. This includes:
   - Local processing for developers testing with the web app using Trunk locally and localhost
   - Local processing for users of the pre-compiled Brush app hosted on the web

2. **Enterprise Cloud Processing**: The enterprise version of the application will offer optional cloud processing capabilities:
   - The reconstruction pipeline could run on cloud servers instead of using the user's local machine via WASM
   - 3D rendering of resulting Gaussian splats and PLY files will always be performed locally on the client machine
   - A key technical challenge will be implementing live updates when cloud GPUs handle reconstruction while clients handle rendering

3. **User Interface**: When processing a dataset, users will be presented with an option to choose between local or cloud processing, which may also be available as a global setting. Enterprise settings for this feature will be developed later in the project.

4. **Technical Architecture**: The application architecture will need to be designed to support both processing modes while maintaining a consistent user experience, with clear separation between processing and rendering components.

### Cursor Rules Implementation Strategy

The development of effective Cursor rules for AI agents requires a fresh approach based on lessons from previous attempts:

1. **Learning from Prior Challenges**: Previous Cursor rules (available at https://github.com/seahorse-ai-ryan/brush/tree/new-ui/.cursor/rules) had mixed results due to:
   - Conflicts between rules and other documentation
   - Ambiguity about when different aspects of rules should be followed
   - AI agents appearing to "skim" rather than strictly follow rules
   - Non-deterministic results when multiple approaches were provided

2. **Targeted Rule Design**: New rules will be:
   - Highly specific to individual tasks
   - Clear and unambiguous in their directives
   - Free from internal contradictions
   - Structured to minimize interpretation differences

3. **Context-Specific Application**: Each rule will clearly specify:
   - WHEN it should be applied (which specific tasks or contexts)
   - HOW the AI agent should indicate it's following a particular rule
   - What the expected outcomes are

4. **Validation Process**: AI agents will be instructed to explain their actions before performing them, explicitly stating whether they are following specific Cursor rules, base training, chat context, or project documentation.

5. **Continuous Improvement**: Rules will be iteratively refined based on observed AI agent behavior, creating a feedback loop to improve rule effectiveness.

### Technical Implementation of Modularity

The modular architecture will be implemented with specific attention to separating UI components from processing logic:

1. **Concrete Example**: A key modularity issue in the current architecture involves the Scene pane, which contains both UI elements and logic for exporting PLY files after training. This tight coupling means that UI adjustments require moving processing logic, which has caused issues in previous development attempts.

2. **Decoupling Approach**: 
   - UI buttons and controls will be designed to be placement-flexible, able to trigger background processes or affect UI layout without being tied to specific workflows
   - Processing logic will be moved to dedicated service classes or modules that can be invoked from any UI component
   - The architecture will leverage the existing CLI tool as proof that much of the functionality can run headlessly without UI

3. **Investigation Phase**: As a first step, the team will conduct a thorough code review to identify areas where UI and process dependencies exist, creating an inventory of components needing separation.

4. **Interface Design**: 
   - Clear interfaces will be defined for communication between UI components and processing logic
   - Events and messaging patterns will be employed to reduce direct dependencies
   - State management will be centralized to avoid scattered state across UI components

5. **Reconstruction Pipeline Modularity**: 
   - The pipeline will be broken down into distinct stages (data loading, preprocessing, feature extraction, reconstruction algorithm, post-processing)
   - Each stage will have well-defined inputs and outputs
   - Different implementations of each stage can be plugged in as long as they conform to the stage interface

### Another UI App Scope

Phase 6 involves creating a separate UI application that demonstrates the modularity of the Brush architecture. This aspect of the project requires further elaboration:

1. **Purpose**: This secondary application will serve as both a proof-of-concept for the modular architecture and potentially address specific use cases that may be defined in forthcoming Product Requirements Documents (PRDs).

2. **Implementation Approach**:
   - The application will be deliberately minimal, focusing only on core functionality needed to demonstrate backend reuse
   - It will utilize the same backend interfaces and services as the main Brush application
   - Development will emphasize speed and simplicity rather than comprehensive feature parity

3. **Further Definition**: The precise nature and requirements for this application will be detailed in additional PRD documents, which will be incorporated into the planning once available.

4. **Potential Examples**:
   - A simplified web-based viewer that loads and displays 3D Gaussian splats
   - A specialized tool focusing on a single aspect of the reconstruction pipeline
   - A dashboard-style application for monitoring multiple reconstruction processes

5. **Success Criteria**: The secondary application should demonstrate that the backend can function independently of the main Brush UI, potentially enabling embedding within other platforms or websites.

## Development Environment Specifics

The streamlining of the development environment is a critical foundation for efficient development, with particular attention to the following challenges:

1. **MCP Server Integration**: 
   - Past experience with Cursor and Claude using MCP servers has revealed inconsistent results
   - AI agents sometimes struggled to access console logs from MCP servers, leading to workaround attempts
   - A key priority will be establishing a reliable setup for the MCP server that AI agents can easily understand and use
   - Resources like https://browsertools.agentdesk.ai/installation will be incorporated into the documentation

2. **Consolidated Environment Management**:
   - Development will investigate tools and scripts to manage multiple services (MCP servers, Trunk web server, desktop compilation) from a single interface
   - Potential approaches include creating a unified control panel or dashboard within Cursor
   - Shell scripts or a small control application might be developed to streamline starting and stopping services

3. **Standardized Development Workflows**:
   - Clear, step-by-step procedures will be documented for common tasks like:
     - Setting up the development environment from scratch
     - Running the desktop application in development mode
     - Launching the web version via Trunk
     - Accessing console logs and debugging information
     - Testing across different platforms

4. **Background Service Management**:
   - Solutions will be explored for reliably running background services without terminal window proliferation
   - Potential use of development containers will be evaluated to ensure consistent environments

5. **IDE Integration**:
   - The setup will be optimized for the Cursor IDE, with specific configurations to improve AI agent effectiveness
   - Tasks and launch configurations will be pre-defined to simplify common operations

## Data Storage Architecture

The data storage architecture for the Brush application remains an area for further exploration, with several potential approaches to be evaluated:

1. **Current Status**: Initial phases will involve a detailed assessment of the current storage mechanisms in the Brush application, documenting strengths, limitations, and areas for improvement.

2. **Potential Technologies**:
   - Firebase for cloud storage and real-time database capabilities
   - PostgreSQL with GIS extension for spatial data management
   - Other cloud-native storage solutions that may be identified during the investigation phase

3. **Abstraction Strategy**: 
   - A storage interface layer will be designed to abstract the underlying storage mechanisms
   - This abstraction will allow seamless switching between local and cloud storage options
   - The architecture will accommodate adding new storage providers in the future

4. **Storage Requirements Analysis**:
   - Dataset formats and sizes will be analyzed to determine optimal storage approaches
   - Performance characteristics for different operations (read, write, query) will be documented
   - Security and access control requirements will be defined

5. **Implementation Approach**:
   - The storage architecture will be developed incrementally, starting with local storage options
   - Cloud integration will build upon the established abstractions
   - Testing will verify consistent behavior across different storage mechanisms

## Testing Framework Recommendations

Based on industry best practices and the specific needs of the Brush application, the following testing frameworks are recommended for evaluation during Phase 7:

1. **Desktop Testing**:
   - Unit Tests: Rust's built-in testing framework
   - UI Testing: egui_kittest for EGUI-based UI components
   - Integration Testing: Custom test harnesses built on Rust's test framework

2. **Web Testing**:
   - Unit Tests: wasm-bindgen-test for WebAssembly components
   - UI Testing: Selenium or Playwright for browser automation
   - Console Logging: Enhanced MCP integration for better debugging visibility

3. **Android Testing**:
   - UI Testing: Espresso (Google's native UI testing framework) or UI Automator for native components
   - Cross-platform Testing: Appium if a unified testing approach is preferred
   - Integration with CI: GitHub Actions for automated testing

4. **iOS Testing**:
   - UI Testing: XCTest/XCUITest (Apple's native testing frameworks) or EarlGrey (Google's iOS testing framework)
   - Cross-platform Testing: Appium as an alternative for unified testing
   - Integration with CI: GitHub Actions for automated testing

## UI Elements for Local/Cloud Processing

The user interface for selecting between local and cloud processing will include:

1. **Dataset Processing Options**:
   - When initiating dataset processing, users will be presented with a modal dialog or settings panel
   - The option will clearly explain the tradeoffs (performance, privacy, cost) between local and cloud processing
   - Visual indicators will show the current selection and processing status

2. **Global Preferences**:
   - A global setting in the application preferences will allow users to set their default processing location
   - Enterprise users will have additional configuration options for cloud resource allocation and billing

3. **Status Indicators**:
   - The UI will provide clear visual feedback about where processing is occurring
   - Progress indicators will be tailored to the selected processing mode
   - Estimated completion times will be displayed based on the processing location

4. **Hybrid Processing Management**:
   - For scenarios where reconstruction occurs in the cloud but rendering is local,
     the UI will provide transparency about data transfer and synchronization status

## Phase Completion Metrics

To provide clear guidance on when each phase is considered complete, the following success metrics will be established:

### Phase 1: Documentation and Development Environment Setup
- **Exit Criteria:**
  - All human-readable documentation has been reviewed and updated for accuracy
  - A complete set of AI-readable Cursor rules is established and tested
  - Development environment setup is documented and verified on all target platforms
  - Setup time for a new developer is reduced to under 30 minutes

### Phase 2: Modularity from UI
- **Exit Criteria:**
  - Complete inventory of UI-logic coupling points is documented
  - Initial abstraction layers are defined and implemented for at least 3 key components
  - First round of refactoring is complete with no regression in functionality
  - UI components can be modified without requiring changes to business logic

### Phase 3: Panels and Windows UI Enhancements
- **Exit Criteria:**
  - Movable UI panels are fully implemented with EGUI
  - Existing dockable panel system is maintained and integrated with movable windows
  - UI layout properly prioritizes the main Scene render pane
  - User testing confirms improved usability with the new UI

### Phase 4: Add Local Datasets
- **Exit Criteria:**
  - Desktop version can save/load datasets to/from specified local directory
  - Web version successfully utilizes IndexedDB for dataset storage
  - All dataset operations (save, load, modify) work consistently across platforms
  - Performance testing shows acceptable read/write speeds for typical dataset sizes

### Phase 5: Add Cloud Datasets
- **Exit Criteria:**
  - Cloud backend on Google Cloud is fully operational
  - User authentication system is implemented and tested
  - Firebase analytics integration is complete
  - Users can successfully connect to and manage datasets in Google Cloud

### Phase 6: Demonstrate Another UI App Using the Same Backends
- **Exit Criteria:**
  - Backend interfaces are fully documented and stable
  - New minimal UI application is developed and functional
  - The new UI successfully utilizes the Brush backend components
  - Both applications can operate on the same datasets without conflicts

### Phase 7: Enhance Testing and Debugging
- **Exit Criteria:**
  - Unit test coverage for core components reaches at least 80%
  - UI testing is implemented and automated for all platforms
  - AI-assisted testing workflow is documented and demonstrated
  - Developers can execute all tests and debug efficiently across platforms

### Phase 8: Modular Reconstruction Pipeline
- **Exit Criteria:**
  - Clear interfaces are defined for all reconstruction pipeline stages
  - Abstraction layers successfully allow different algorithms to be used
  - Current reconstruction algorithm is successfully integrated into new pipeline
  - Documentation for integrating custom algorithms is complete and verified

### Phase 9: Further UI Enhancements and Refinements
- **Exit Criteria:**
  - UI for selecting local directories is implemented and tested
  - Cloud storage management UI is complete
  - Transcoding and remastering UI components are functional
  - Local/cloud processing selection UI is implemented
  - User feedback is incorporated into final designs

## Conclusion

This development plan provides a comprehensive roadmap for revitalizing the Brush application while addressing the challenges encountered in previous attempts. By taking an incremental, modular approach and leveraging AI-assisted development techniques, the project has a strong foundation for success.

The plan prioritizes documentation and development environment improvements before tackling more complex architectural changes, ensuring that both human developers and AI agents have the context and tools needed to contribute effectively.

With a focus on cross-platform compatibility, modular architecture, and a modern user interface, the revitalized Brush application will serve as an extensible framework for 3D reconstruction that is accessible across various platforms and processing options.