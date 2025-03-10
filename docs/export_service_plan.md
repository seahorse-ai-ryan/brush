# Export Service Plan

## Overview

This document outlines the plan for implementing a centralized Export Service in Brush. The service will decouple export functionality from specific UI components, enabling any UI element to trigger exports while maintaining a consistent export workflow.

## Current Implementation Issues

1. Export functionality is currently tightly coupled to the Scene panel
2. The Export button in the Controls window needs to access Scene panel functionality
3. No centralized way to handle different export formats (PLY, etc.)
4. Difficult to implement batch processing of datasets

## Proposed Solution: Centralized Export Service

We will implement a dedicated `ExportService` that will:

1. Provide a centralized API for exporting splats in various formats
2. Decouple export logic from UI components
3. Enable any UI element to trigger exports
4. Support batch processing of datasets
5. Make it easier to add new export formats in the future

## Architecture

### 1. Export Service Module

Create a new module `export_service.rs` that will:

- Define the `ExportService` struct
- Implement methods for different export formats
- Handle file system operations
- Manage export configurations

```rust
// Simplified example structure
pub struct ExportService {
    // Configuration and state
    export_dir: PathBuf,
    default_format: ExportFormat,
}

pub enum ExportFormat {
    PLY,
    // Other formats can be added here
}

impl ExportService {
    pub fn new(export_dir: PathBuf) -> Self { ... }
    
    pub fn export_splats(&self, splats: &Splats, format: ExportFormat, filename: &str) -> Result<PathBuf, ExportError> { ... }
    
    pub fn export_ply(&self, splats: &Splats, filename: &str) -> Result<PathBuf, ExportError> { ... }
    
    // Batch processing methods
    pub fn export_all_datasets(&self, datasets: &[Dataset], format: ExportFormat) -> Vec<Result<PathBuf, ExportError>> { ... }
}
```

### 2. Integration with AppContext

Add the `ExportService` to the `AppContext` to make it accessible throughout the application:

```rust
pub struct AppContext {
    // Existing fields
    pub dataset: Dataset,
    pub camera: Camera,
    // ...
    
    // New field
    pub export_service: ExportService,
}
```

### 3. UI Integration

#### Controls Panel

Update the Controls panel to use the Export Service:

```rust
// In controls_detail.rs
if export_button.clicked() {
    if let Some(splats) = context.get_current_splats() {
        match context.export_service.export_ply(splats, "export.ply") {
            Ok(path) => {
                // Show success message
            },
            Err(err) => {
                // Show error message
            }
        }
    } else {
        // Show error: no splats available
    }
}
```

#### Scene Panel

Refactor the Scene panel to use the Export Service instead of implementing export logic directly:

```rust
// Remove direct export implementation from Scene panel
// Replace with calls to the Export Service
```

### 4. CLI Integration

Update the CLI to use the Export Service for consistency:

```rust
// In CLI command handling
let export_service = ExportService::new(output_dir);
export_service.export_ply(&splats, output_filename)?;
```

## Implementation Plan

### Phase 1: Create the Export Service

1. Create the `export_service.rs` module with basic structure
2. Implement PLY export functionality (migrated from Scene panel)
3. Add unit tests for the service

### Phase 2: Integrate with AppContext

1. Add the Export Service to AppContext
2. Create helper methods in AppContext to access the service
3. Update initialization code to create the service

### Phase 3: Update UI Components

1. Refactor Scene panel to use the Export Service
2. Update Controls panel to use the Export Service for the Export button
3. Add appropriate error handling and user feedback

### Phase 4: CLI Integration and Batch Processing

1. Update CLI to use the Export Service
2. Implement batch processing functionality
3. Add UI for batch export configuration

## Benefits

1. **Decoupling**: UI components don't need to know export implementation details
2. **Consistency**: All export operations use the same code path
3. **Extensibility**: Easy to add new export formats
4. **Batch Processing**: Enable processing multiple datasets at once
5. **Testability**: Export logic can be tested independently of UI

## Future Enhancements

1. Support for additional export formats
2. Export progress indicators
3. Configurable export settings (compression, precision, etc.)
4. Export presets for common configurations
5. Background processing for large exports

## Comparison with CLI Implementation

The CLI currently implements export functionality directly, which creates duplication with the UI implementation. By centralizing export logic in the Export Service, we ensure that:

1. CLI and UI use the same export code path
2. Changes to export formats are automatically available in both interfaces
3. Export configurations can be shared between CLI and UI

This approach aligns with the principle of having a single source of truth for export functionality, making the codebase more maintainable and consistent. 