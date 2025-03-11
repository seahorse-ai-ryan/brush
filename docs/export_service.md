# Export Service

## Overview

This document describes the centralized Export Service in Brush. The service decouples export functionality from specific UI components, enabling any UI element to trigger exports while maintaining a consistent export workflow.

## Implementation Features

The `ExportService` provides:

1. A centralized API for exporting splats in various formats
2. Decoupled export logic from UI components
3. The ability for any UI element to trigger exports
4. Support for batch processing of datasets
5. Easy extensibility for new export formats
6. Auto-saving of models at specified training intervals
7. Consistent, meaningful filenames based on the current dataset

## Architecture

### 1. Export Service Module

The `export_service.rs` module:

- Defines the `ExportService` struct
- Implements methods for different export formats
- Handles file system operations
- Manages export configurations
- Provides auto-save functionality

```rust
// Simplified structure
pub struct ExportService {
    // Configuration and state
    export_dir: Option<PathBuf>,
    default_format: ExportFormat,
    auto_save_config: Option<AutoSaveConfig>,
    last_auto_save_step: u32,
}

pub enum ExportFormat {
    PLY,
    // Other formats can be added here
}

pub struct AutoSaveConfig {
    enabled: bool,
    interval_steps: u32,
    max_saves: Option<u32>,
    format: ExportFormat,
    prefix: String,
}

impl ExportService {
    pub fn new(export_dir: Option<PathBuf>) -> Self { ... }
    
    pub fn export_splats(&self, splats: &Splats, format: ExportFormat, filename: &str) -> Result<PathBuf, ExportError> { ... }
    
    pub fn export_ply(&self, splats: &Splats, filename: &str) -> Result<PathBuf, ExportError> { ... }
    
    // Auto-save functionality
    pub fn configure_auto_save(&mut self, config: AutoSaveConfig) { ... }
    
    pub fn check_auto_save(&mut self, splats: &Splats, current_step: u32) -> Option<Result<PathBuf, ExportError>> { ... }
    
    // Batch processing methods
    pub fn export_all_datasets(&self, datasets: &[Dataset], format: ExportFormat) -> Vec<Result<PathBuf, ExportError>> { ... }
}
```

### 2. Integration with AppContext

The `ExportService` is integrated into the `AppContext` to make it accessible throughout the application:

```rust
pub struct AppContext {
    // Existing fields
    pub dataset: Dataset,
    pub camera: Camera,
    // ...
    
    // Track the current dataset name for export filenames
    current_dataset_name: Option<String>,
    
    // Export service
    pub export_service: ExportService,
}

impl AppContext {
    // Helper method to check for auto-save during training
    pub fn on_training_step(&mut self, step: u32) {
        if let Some(splats) = self.get_current_splats() {
            if let Some(result) = self.export_service.check_auto_save(splats, step) {
                // Handle auto-save result (log success/failure)
            }
        }
    }
    
    // Set the current dataset name
    pub fn set_current_dataset_name(&mut self, name: String) {
        self.current_dataset_name = Some(name);
    }
    
    // Get the current dataset name
    pub fn current_dataset_name(&self) -> Option<&String> {
        self.current_dataset_name.as_ref()
    }
    
    // Generate a filename for export based on dataset name and current timestamp
    fn generate_export_filename(&self) -> String {
        // Use the current dataset name if available
        let dataset_name = if let Some(name) = self.current_dataset_name() {
            name.clone()
        } else {
            // Fall back to extracting from path if no current dataset name is set
            // (path extraction logic)
            "dataset".to_string()
        };
        
        // Get current timestamp
        let now = chrono::Local::now();
        let timestamp = now.format("%Y%m%d_%H%M%S");
        
        // Combine dataset name and timestamp
        format!("{}_{}.ply", dataset_name, timestamp)
    }
}
```

### 3. Dataset Name Tracking

To ensure consistent and meaningful export filenames, dataset name tracking is implemented:

1. **Capture Dataset Name on Processing**:
   ```rust
   // In dataset_detail.rs
   pub(crate) fn process_dataset(&self, dataset_path: &PathBuf, context: &mut AppContext) {
       // Get the dataset name from the path
       let dataset_name = dataset_path.file_name()
           .unwrap_or_default()
           .to_string_lossy()
           .to_string();
       
       // Set the current dataset name in the context
       context.set_current_dataset_name(dataset_name);
       
       // Process the dataset...
   }
   ```

2. **Preserve Dataset Name Across State Changes**:
   ```rust
   // In app.rs
   pub fn connect_to(&mut self, process: RunningProcess<TrainBack>) {
       // Save the current dataset name before resetting
       let current_dataset_name = self.current_dataset_name.clone();
       
       // Reset context & view
       *self = Self::new(self.device.clone(), self.ctx.clone(), &self.cam_settings);
       
       // Restore the current dataset name
       self.current_dataset_name = current_dataset_name;
       
       // Continue with process connection...
   }
   ```

### 4. UI Integration

#### Controls Panel

The Controls panel uses the Export Service:

```rust
// In controls_detail.rs
if export_button.clicked() {
    if let Some(splats) = context.get_current_splats() {
        // Use the generated filename based on current dataset name
        let filename = context.generate_export_filename();
        match context.export_service.export_ply(splats, &filename) {
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

The Scene panel uses the Export Service instead of implementing export logic directly.

#### Settings Panel

The Settings panel includes UI controls for configuring auto-save:

```rust
// In settings_detail.rs
ui.checkbox(&mut auto_save_enabled, "Enable auto-save");
if auto_save_enabled {
    ui.add(egui::Slider::new(&mut auto_save_interval, 100..=10000).text("Save interval (steps)"));
    ui.add(egui::Slider::new(&mut auto_save_max, 1..=100).text("Maximum saves"));
    // ...
    
    // Update export service config
    let config = AutoSaveConfig {
        enabled: auto_save_enabled,
        interval_steps: auto_save_interval,
        max_saves: Some(auto_save_max),
        format: ExportFormat::PLY,
        prefix: "autosave_".to_string(),
    };
    context.export_service.configure_auto_save(config);
}
```

### 5. CLI Integration

The CLI uses the Export Service for consistency:

```rust
// In CLI command handling
let export_service = ExportService::new(output_dir);
export_service.export_ply(&splats, output_filename)?;
```

## Benefits

1. **Decoupling**: UI components don't need to know export implementation details
2. **Consistency**: All export operations use the same code path
3. **Extensibility**: Easy to add new export formats
4. **Batch Processing**: Enable processing multiple datasets at once
5. **Testability**: Export logic can be tested independently of UI
6. **Auto-save**: Centralized auto-save functionality with consistent configuration
7. **Meaningful Filenames**: Export filenames consistently reflect the current dataset being processed

## Future Enhancements

1. Support for additional export formats
2. Export progress indicators
3. Configurable export settings (compression, precision, etc.)
4. Export presets for common configurations
5. Background processing for large exports
6. Advanced auto-save options (different formats at different intervals)
7. Custom naming templates for exports

## Comparison with CLI Implementation

The CLI currently implements export functionality directly, which creates duplication with the UI implementation. By centralizing export logic in the Export Service, we ensure that:

1. CLI and UI use the same export code path
2. Changes to export formats are automatically available in both interfaces
3. Export configurations can be shared between CLI and UI
4. Auto-save functionality is consistently implemented

This approach aligns with the principle of having a single source of truth for export functionality, making the codebase more maintainable and consistent. 