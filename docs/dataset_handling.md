# Brush Datasets Window Documentation

## Overview

The Brush application provides a dedicated datasets window that allows users to browse, select, and process datasets for 3D visualization. This document summarizes how the datasets window works, from accessing it via the disk icon to managing files and saving user preferences.

## Accessing the Datasets Window

The datasets window is accessed through a panel on the left side of the main interface:

1. **Disk Icon (📁)**: Located in the left panel of the main interface
2. **Click Action**: Opens the datasets panel when clicked
3. **Panel State**: The panel can be toggled open/closed, and this state is preserved between sessions

## Window Layout and Components

The datasets window consists of several key components:

1. **Header Section**
   - Title: "Datasets" displayed at the top
   - Refresh Button (🔄): Updates the list of datasets from the configured folder
   - Dataset Count: Shows the total number of datasets found (e.g., "5 datasets")

2. **Dataset Folder Configuration**
   - Path Display: Shows the currently selected dataset folder path
   - "Select Dataset Folder" Button: Opens a file dialog to choose a new folder
   - "Set" Button: Confirms the selected folder as the new dataset location

3. **Dataset List (Table View)**
   - Scrollable area containing all discovered datasets
   - Each dataset entry includes:
     - Status Icon: ✓ (processed) or 📁 (unprocessed)
     - Dataset Name: Derived from the filename
     - Dataset Details: Size and last modified date
     - "Process" Button: Initiates processing for the selected dataset

## Cross-Platform File Handling

### Architecture Overview

Brush implements a platform-agnostic file handling system with two distinct layers:

1. **UI Layer**: Platform-specific file dialogs and user interaction
   - Native OS dialogs on desktop platforms
   - Web-based file pickers in browser environment
   - Consistent UI feedback across platforms

2. **Export Service Layer**: Background processing and file operations
   - Separated from UI to prevent blocking
   - Handles large file operations asynchronously
   - Platform-independent core logic

### Platform-Specific Implementations

1. **Desktop Application**
   ```rust
   #[cfg(not(target_arch = "wasm32"))]
   fn handle_file_selection() {
       // Uses native file dialog via rfd
       // Provides full filesystem access
       // Supports folder selection
   }
   ```

2. **Web Application**
   ```rust
   #[cfg(target_arch = "wasm32")]
   fn handle_file_selection() {
       // Uses browser's file picker API
       // Limited to user-selected files only
       // No direct folder access
   }
   ```

### Key Design Decisions

1. **File Handle Management**
   - File handles are consumed upon read
   - Important metadata extracted before consumption
   - Prevents common ownership issues in async code

2. **Export Service Separation**
   - UI remains responsive during file operations
   - Progress reporting via message passing
   - Cancellable operations where possible

3. **Platform Adaptations**
   - Graceful degradation for web limitations
   - Consistent user experience across platforms
   - Clear feedback for platform-specific features

## Dataset Processing Flow

1. **File Selection**
   - User selects file(s) via platform-appropriate dialog
   - Files are validated for supported formats
   - Metadata is extracted and displayed

2. **Processing**
   - Dataset is queued for processing
   - Progress is reported through UI
   - Background processing prevents UI blocking

3. **Export**
   - Processed results can be exported
   - Export format selection via UI
   - Background processing with progress indication

## Implementation Notes

1. **Browser Limitations**
   - No direct folder access
   - File system API restrictions
   - Memory constraints for large files

2. **Desktop Capabilities**
   - Full filesystem access
   - Native performance
   - System-native file dialogs

3. **Error Handling**
   - Clear user feedback
   - Graceful fallbacks
   - Platform-specific error messages

## Best Practices

1. **File Operations**
   - Extract metadata before consuming handles
   - Use async operations for large files
   - Implement proper cleanup

2. **UI Feedback**
   - Show operation progress
   - Provide clear error messages
   - Indicate platform limitations

3. **Cross-Platform Development**
   - Test on all target platforms
   - Handle platform differences early
   - Document platform-specific behaviors

## File System Interaction

### Local File System Access

The datasets window interacts with the local file system in several ways:

1. **Folder Selection**: Uses the native file dialog to browse and select a folder containing datasets
   - Implemented using the `rfd` (Rust File Dialog) library
   - Provides a familiar file browsing experience for the user's operating system

2. **Dataset Discovery**: Scans the selected folder for compatible dataset files
   - Searches for ZIP files that may contain 3D data
   - Reads file metadata (size, modification date) for display in the table

3. **Dataset Processing**: Loads and processes selected datasets for visualization
   - Reads the dataset file from disk
   - Processes the content for 3D visualization in the main view

### File Listing in Table Format

The datasets window displays discovered files in a table-like format:

1. **Automatic Scanning**: Files are automatically scanned when:
   - The application starts (if a dataset folder is already configured)
   - The user clicks the "Refresh" button
   - A new dataset folder is selected and confirmed

2. **Table Columns**:
   - Status: Visual indicator of whether the dataset has been processed
   - Name: Filename of the dataset
   - Size: Human-readable file size (KB, MB, GB)
   - Modified: Last modification date in a readable format
   - Actions: Buttons for processing or other operations

3. **Sorting**: The dataset list is sorted alphabetically by default

## User Preferences

The application saves user preferences to ensure a consistent experience across sessions:

### Saved Preferences

1. **Dataset Folder Path**: The last selected dataset folder
2. **Panel State**: Whether the datasets panel was open or closed
3. **Dataset Index**: Information about previously processed datasets, including:
   - Paths to known datasets
   - Processing status of each dataset
   - Last scan timestamp

### Storage Mechanism

Preferences are stored in JSON format in the user's configuration directory:

1. **Location**: User's configuration directory + "brush-clean-ui"
   - Windows: `%APPDATA%\brush-clean-ui\`
   - macOS: `~/Library/Application Support/brush-clean-ui/`
   - Linux: `~/.config/brush-clean-ui/`

2. **Files**:
   - `config.json`: General application configuration
   - `dataset_index.json`: Dataset-specific information

3. **Automatic Saving**: Preferences are automatically saved when:
   - A new dataset folder is selected
   - A dataset is processed
   - The application is closed

## Workflow Example

A typical workflow for using the datasets window:

1. User clicks the disk icon (📁) in the left panel to open the datasets window
2. If no dataset folder is configured, user clicks "Select Dataset Folder"
3. User navigates to a folder containing dataset ZIP files and selects it
4. The application scans the folder and displays available datasets in the table
5. User clicks "Process" on a dataset of interest
6. The application loads and processes the dataset
7. The view switches to the 3D visualization of the processed dataset
8. User preferences are saved automatically for future sessions

## Technical Implementation

The datasets window is implemented using:

1. **UI Framework**: egui for cross-platform user interface
2. **File System Access**: Standard Rust file system libraries and rfd for dialogs
3. **State Management**: Rust structs with serialization/deserialization for preferences
4. **Asynchronous Processing**: Background processing to keep the UI responsive

This architecture ensures a responsive, intuitive interface for managing datasets while maintaining a consistent user experience across sessions. 