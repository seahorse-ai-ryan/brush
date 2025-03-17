# Brush Web Storage Implementation Plan

## Overview

This plan outlines the implementation of browser-based storage for the Brush web application using IndexedDB, with considerations for future cloud storage capabilities. The key feature is that IndexedDB will only be used when users explicitly choose to save datasets, allowing for an in-memory-only workflow when preferred.

## Phase 1: IndexedDB Implementation

### 1. Create Platform-Specific Storage Module

```rust
// src/storage/mod.rs
pub mod indexed_db;
pub mod filesystem;

#[cfg(target_arch = "wasm32")]
pub use indexed_db as storage_impl;

#[cfg(not(target_arch = "wasm32"))]
pub use filesystem as storage_impl;

pub use storage_impl::DatasetStorage;
```

### 2. Implement IndexedDB Storage

```rust
// src/storage/indexed_db.rs
use wasm_bindgen::prelude::*;
use web_sys::{IdbDatabase, IdbObjectStore, IdbTransaction};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

const DB_NAME: &str = "brush-datasets";
const DATASET_STORE: &str = "datasets";
const METADATA_STORE: &str = "metadata";
const DB_VERSION: u32 = 1;

pub struct DatasetStorage {
    db: IdbDatabase,
    cached_metadata: Option<DatasetMetadata>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DatasetMetadata {
    datasets: HashMap<String, DatasetInfo>,
    preferences: UserPreferences,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DatasetInfo {
    name: String,
    size_bytes: u64,
    created_at: u64,
    modified_at: u64,
    storage_type: StorageType,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum StorageType {
    Local,
    Cloud { url: String, provider: CloudProvider },
}

#[derive(Serialize, Deserialize, Clone)]
pub enum CloudProvider {
    None,
    S3,
    GCS,
    Custom,
}

impl DatasetStorage {
    pub async fn new() -> Result<Self, JsValue> {
        let window = web_sys::window().unwrap();
        let idb_factory = window.indexed_db()?;
        
        // Open database with version
        let db_promise = idb_factory.open_with_u32(DB_NAME, DB_VERSION)?;
        
        // Setup database if needed
        let db = JsFuture::from(db_promise).await?;
        let db = db.dyn_into::<IdbDatabase>()?;
        
        // Return initialized storage
        let storage = Self {
            db,
            cached_metadata: None,
        };
        
        Ok(storage)
    }
    
    pub async fn load_metadata(&mut self) -> Result<DatasetMetadata, JsValue> {
        // Implementation to load metadata from IndexedDB
        // If not found, initialize with empty metadata
    }
    
    pub async fn save_dataset(&mut self, name: &str, data: &[u8]) -> Result<(), JsValue> {
        // Store dataset binary data
        // Update metadata with size and timestamps
        // Save updated metadata
    }
    
    pub async fn load_dataset(&self, name: &str) -> Result<Vec<u8>, JsValue> {
        // Retrieve dataset binary data
    }
    
    pub async fn delete_dataset(&mut self, name: &str) -> Result<(), JsValue> {
        // Delete dataset and update metadata
    }
    
    pub async fn get_total_storage_used(&self) -> Result<u64, JsValue> {
        // Calculate total storage used from metadata
    }
}
```

### 3. Update App to Use Platform-Specific Storage

```rust
// In app.rs
#[cfg(target_arch = "wasm32")]
async fn initialize_storage(&mut self) -> Result<(), Error> {
    let storage = DatasetStorage::new().await?;
    self.storage = Some(storage);
    self.load_datasets_from_storage().await?;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn initialize_storage(&mut self) -> Result<(), Error> {
    // Use filesystem approach
}
```

## Phase 2: UI Modifications

### 1. Support In-Memory Processing Option

```rust
// In dataset_import.rs or similar
fn import_options(&mut self, ui: &mut Ui) {
    // Add checkbox to control whether to save to dataset storage
    ui.checkbox(&mut self.add_to_datasets, "Add to datasets list");
    
    if self.add_to_datasets {
        #[cfg(target_arch = "wasm32")]
        {
            ui.label("Dataset will be stored in browser storage");
            // Show estimated size if available
            if let Some(size) = self.estimate_dataset_size() {
                ui.label(format!("Estimated size: {}", format_bytes(size)));
            }
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Show dataset folder options
        }
    } else {
        ui.label("Dataset will be processed in memory only");
        ui.label("Note: Dataset will be lost when you close the application");
    }
}

// In processing logic
fn process_dataset(&mut self, data: Vec<u8>, filename: &str) -> Result<(), Error> {
    // Process the dataset (always done in memory)
    let processed_dataset = self.processor.process(&data, filename)?;
    
    // If add_to_datasets is checked, save to storage
    if self.add_to_datasets {
        #[cfg(target_arch = "wasm32")]
        {
            // Save to IndexedDB
            self.storage.save_dataset(filename, &data).await?;
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Save to filesystem
        }
    }
    
    // Continue with the processing workflow
    self.current_dataset = Some(processed_dataset);
    Ok(())
}
```

### 2. Update Dataset Management UI

```rust
// In dataset_detail.rs
fn display_storage_info(&self, ui: &mut Ui) {
    #[cfg(target_arch = "wasm32")]
    {
        // Hide folder selection
        if let Some(total_size) = self.get_total_storage_used() {
            ui.label(format!("Total storage used: {}", format_bytes(total_size)));
        }
        
        // Add cloud storage placeholder
        ui.horizontal(|ui| {
            ui.label("Storage location:");
            ui.selectable_value(&mut self.storage_type, StorageType::Local, "Browser storage");
            if ui.selectable_value(&mut self.storage_type, StorageType::Cloud, "Cloud storage (coming soon)").clicked() {
                ui.label("Cloud storage options will be available in a future update");
            }
        });
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Show folder selection UI
        if let Some(path) = &self.datasets_folder {
            ui.horizontal(|ui| {
                ui.label("Datasets folder:");
                ui.label(path.to_string_lossy().to_string());
                if ui.button("Change").clicked() {
                    self.select_datasets_folder = true;
                }
            });
        }
    }
}
```

### 3. Add Dataset Deletion with Confirmation

```rust
// In dataset_detail.rs
fn dataset_actions(&mut self, ui: &mut Ui, dataset_name: &str) {
    ui.horizontal(|ui| {
        if ui.button("Open").clicked() {
            self.open_dataset(dataset_name);
        }
        
        if ui.button("Delete").clicked() {
            self.confirm_delete_dataset = Some(dataset_name.to_string());
        }
    });
    
    // Show confirmation dialog if needed
    if let Some(name) = &self.confirm_delete_dataset {
        Window::new("Confirm deletion")
            .collapsible(false)
            .resizable(false)
            .show(ui.ctx(), |ui| {
                ui.label(format!("Are you sure you want to delete dataset \"{}\"?", name));
                ui.label("This cannot be undone.");
                
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.confirm_delete_dataset = None;
                    }
                    
                    if ui.button("Delete").clicked() {
                        #[cfg(target_arch = "wasm32")]
                        {
                            // Schedule async deletion
                            self.delete_dataset(name);
                        }
                        
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            // Use filesystem deletion
                            if let Err(e) = self.delete_dataset_from_fs(name) {
                                self.error = Some(format!("Failed to delete dataset: {}", e));
                            }
                        }
                        
                        self.confirm_delete_dataset = None;
                    }
                });
            });
    }
}
```

### 4. Add Storage Usage Display

```rust
// At the bottom of dataset list
fn render_dataset_list(&mut self, ui: &mut Ui) {
    // Existing code to display dataset list
    
    // Add storage usage at bottom
    ui.separator();
    
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(total_size) = self.get_total_storage_used() {
            ui.horizontal(|ui| {
                ui.label(format!("Storage usage: {}", format_bytes(total_size)));
                let quota = self.get_storage_quota().unwrap_or(0);
                if quota > 0 {
                    let percentage = (total_size as f64 / quota as f64 * 100.0).min(100.0);
                    ui.add(egui::ProgressBar::new(percentage as f32 / 100.0)
                        .desired_width(100.0)
                        .text(format!("{:.1}%", percentage)));
                }
            });
        }
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(total_size) = self.get_disk_usage() {
            ui.label(format!("Storage usage: {}", format_bytes(total_size)));
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn get_storage_quota(&self) -> Option<u64> {
    // Use navigator.storage.estimate() to get quota information
    // Implement with web_sys
}
```

## Phase 3: Cloud Storage Placeholder

### 1. Add Cloud Storage Types

```rust
// In a new module: src/storage/cloud.rs
pub enum CloudStorageProvider {
    S3 {
        bucket: String,
        region: String,
        // Credentials would be handled securely
    },
    GCS {
        bucket: String,
        project: String,
    },
    Custom {
        url: String,
        api_key: Option<String>,
    }
}

pub struct CloudStorageConfig {
    provider: CloudStorageProvider,
    enabled: bool,
    // Additional configuration
}

impl CloudStorageConfig {
    pub fn is_available() -> bool {
        // In initial implementation, always return false
        false
    }
    
    pub fn get_provider_name(&self) -> &'static str {
        match self.provider {
            CloudStorageProvider::S3 { .. } => "Amazon S3",
            CloudStorageProvider::GCS { .. } => "Google Cloud Storage",
            CloudStorageProvider::Custom { .. } => "Custom",
        }
    }
}
```

### 2. Update UI for Cloud Storage (Placeholder)

```rust
// In settings_panel.rs or a new cloud_settings.rs
fn show_cloud_storage_settings(&mut self, ui: &mut Ui) {
    ui.heading("Cloud Storage");
    
    ui.label("Cloud storage support is coming soon!");
    ui.label("Store your datasets securely in the cloud and access them from anywhere.");
    
    ui.add_space(10.0);
    
    Frame::dark().show(ui, |ui| {
        ui.set_enabled(false); // Disable controls since this is placeholder
        
        ui.checkbox(&mut self.cloud_enabled, "Enable cloud storage");
        
        ui.add_space(5.0);
        
        egui::ComboBox::from_label("Provider")
            .selected_text(self.get_provider_name())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.provider_type, ProviderType::S3, "Amazon S3");
                ui.selectable_value(&mut self.provider_type, ProviderType::GCS, "Google Cloud Storage");
                ui.selectable_value(&mut self.provider_type, ProviderType::Custom, "Custom");
            });
            
        match self.provider_type {
            ProviderType::S3 => {
                ui.horizontal(|ui| {
                    ui.label("Bucket:");
                    ui.text_edit_singleline(&mut self.s3_bucket);
                });
                ui.horizontal(|ui| {
                    ui.label("Region:");
                    ui.text_edit_singleline(&mut self.s3_region);
                });
            },
            ProviderType::GCS => {
                // Similar fields for GCS
            },
            ProviderType::Custom => {
                // Custom URL and API setup
            }
        }
        
        ui.add_space(10.0);
        ui.button("Connect").on_hover_text("Feature coming soon");
    });
}
```

## Phase 4: Integration

### 1. Modify Dataset Loading/Saving Logic

```rust
// In app.rs or similar main control logic
async fn load_dataset(&mut self, name: &str) -> Result<Dataset, Error> {
    #[cfg(target_arch = "wasm32")]
    {
        let data = self.storage.load_dataset(name).await?;
        let dataset = Dataset::from_bytes(&data)?;
        Ok(dataset)
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Load from filesystem
    }
}

async fn save_dataset(&mut self, dataset: &Dataset) -> Result<(), Error> {
    #[cfg(target_arch = "wasm32")]
    {
        let data = dataset.to_bytes()?;
        self.storage.save_dataset(&dataset.name, &data).await?;
        Ok(())
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Save to filesystem
    }
}
```

### 2. Update Dataset Folder Path Handling

```rust
// In dataset_detail.rs

// Modify the dataset folder path handling for web
fn handle_folder_selection(&mut self) {
    #[cfg(target_arch = "wasm32")]
    {
        // For web, instead of folder selection:
        self.show_storage_info_panel = true;
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Original folder selection code
    }
}

// New method to show storage info for web
#[cfg(target_arch = "wasm32")]
fn show_storage_info(&mut self, ui: &mut Ui) {
    Window::new("Storage Information")
        .open(&mut self.show_storage_info_panel)
        .show(ui.ctx(), |ui| {
            ui.heading("Browser Storage");
            
            if let Some(usage) = self.get_total_storage_used() {
                ui.label(format!("Current usage: {}", format_bytes(usage)));
            }
            
            if let Some(quota) = self.get_storage_quota() {
                ui.label(format!("Available quota: {}", format_bytes(quota)));
                
                let percentage = (usage as f64 / quota as f64 * 100.0).min(100.0);
                ui.add(ProgressBar::new(percentage as f32 / 100.0)
                    .text(format!("{:.1}%", percentage)));
            }
            
            ui.separator();
            ui.label("Web browsers limit how much data can be stored.");
            ui.label("You may need to delete unused datasets if you reach your limit.");
            
            ui.separator();
            ui.label("Coming soon: Cloud storage options for unlimited datasets!");
        });
}
```

## Technical Considerations

1. **Platform-Specific Code**:
   - Use `#[cfg(target_arch = "wasm32")]` and `#[cfg(not(target_arch = "wasm32"))]` consistently
   - Keep platform-specific code in separate modules where possible
   - Use trait abstractions to minimize duplication

2. **Storage Limits**:
   - Monitor and respect browser storage limits
   - Implement graceful degradation when limits are reached
   - Provide clear feedback to users about storage usage

3. **Asynchronous Operations**:
   - All IndexedDB operations are async and should use Future/await
   - Ensure UI remains responsive during data operations
   - Add loading indicators for longer operations

4. **User Experience**:
   - Make it clear when datasets are stored vs. in-memory only
   - Provide appropriate warnings about data persistence
   - Ensure storage usage information is accurate and visible

5. **Error Handling**:
   - Robust error handling for storage operations
   - Clear error messages for users
   - Fallback mechanisms when storage operations fail

## Implementation Steps Summary

1. Implement IndexedDB storage module with metadata tracking
2. Update UI to support in-memory processing vs. saved datasets
3. Add storage usage display and dataset management features
4. Add dataset deletion with confirmation dialog
5. Create placeholders for future cloud storage features
6. Integrate with existing dataset processing workflow
7. Implement platform-specific dataset path handling

This plan provides a structured approach to implementing browser-based storage for Brush while maintaining the option for in-memory-only processing. It's designed to be implemented incrementally over multiple sessions, with each phase building on the previous one. 