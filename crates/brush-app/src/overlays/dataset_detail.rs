use crate::app::AppContext;
use brush_train::scene::{Scene, SceneView, ViewType};
use egui::{Color32, Context, Hyperlink, Pos2, RichText, TextureHandle, Vec2, pos2};
use std::path::{PathBuf, Path};
use std::time::SystemTime;
use std::fs;
use brush_process::data_source::DataSource;
use brush_process::process_loop::{ProcessArgs, start_process};
use dirs;
use std::io::{self};
use zip::ZipArchive;
use notify::{Watcher, RecursiveMode, Result as NotifyResult, Event, recommended_watcher};
use std::sync::mpsc::{channel, Receiver};

struct SelectedView {
    index: usize,
    view_type: ViewType,
    texture_handle: TextureHandle,
}

#[derive(Clone, Debug)]
struct DatasetEntry {
    name: String,
    path: PathBuf,
    size: u64,
    modified: SystemTime,
    processed: bool,
}

fn selected_scene(t: ViewType, context: &AppContext) -> &Scene {
    if let Some(eval_scene) = context.dataset.eval.as_ref() {
        match t {
            ViewType::Train => &context.dataset.train,
            _ => eval_scene,
        }
    } else {
        &context.dataset.train
    }
}

impl SelectedView {
    fn get_view<'a>(&'a self, context: &'a AppContext) -> &'a SceneView {
        &selected_scene(self.view_type, context).views[self.index]
    }
}

pub(crate) struct DatasetDetailOverlay {
    // Dataset browser fields
    datasets_folder: Option<PathBuf>,
    datasets: Vec<DatasetEntry>,
    show_folder_dialog: bool,
    folder_selection_in_progress: bool,
    show_file_dialog: bool,        // New field for file selection
    file_selection_in_progress: bool, // New field for file selection
    copy_datasets_to_local: bool,  // New field for dataset copy preference
    show_dataset_folder_dialog: bool, // New field for dataset folder selection
    dataset_folder_selection_in_progress: bool, // New field for dataset folder selection
    
    // Detail view fields
    view_type: ViewType,
    selected_view: Option<SelectedView>,
    selected_dataset: Option<PathBuf>,
    
    // UI state
    open: bool,
    position: Pos2,
    size: Vec2,
    
    // For dynamic sizing
    last_table_height: f32,
    auto_open_done: bool,          // Flag for auto-open on startup
    height_changed: bool,          // Flag to track height changes
    last_dataset_count: usize,     // Track dataset count changes
    prev_size: Vec2,               // Track previous size to detect resize attempts
    pending_file_import: Option<PathBuf>, // New field for pending file import
    
    // File system watcher
    file_watcher: Option<Box<dyn Watcher>>,
    file_watcher_receiver: Option<Receiver<NotifyResult<Event>>>,
}

// Helper function for URL buttons
fn url_button(label: &str, url: &str, ui: &mut egui::Ui) {
    ui.add(Hyperlink::from_label_and_url(label, url).open_in_new_tab(true));
}

impl DatasetDetailOverlay {
    pub(crate) fn new() -> Self {
        // Set default datasets folder based on OS
        let default_datasets_folder = Self::get_default_datasets_folder();
        
        let mut overlay = Self {
            // Dataset browser fields
            datasets_folder: Some(default_datasets_folder),
            datasets: Vec::new(),
            show_folder_dialog: false,
            folder_selection_in_progress: false,
            show_file_dialog: false,        // New field for file selection
            file_selection_in_progress: false, // New field for file selection
            copy_datasets_to_local: true,  // New field for dataset copy preference
            show_dataset_folder_dialog: false, // New field for dataset folder selection
            dataset_folder_selection_in_progress: false, // New field for dataset folder selection
            
            // Detail view fields
            view_type: ViewType::Train,
            selected_view: None,
            selected_dataset: None,
            
            // UI state
            open: false, // Start with window closed
            position: pos2(100.0, 100.0),
            size: Vec2::new(600.0, 400.0), // Further reduced height to fit on screen better
            
            // For dynamic sizing
            last_table_height: 150.0, // Reduced table height
            auto_open_done: false,
            height_changed: false,
            last_dataset_count: 0,
            prev_size: Vec2::new(600.0, 400.0), // Match the default size
            pending_file_import: None,
            
            // File system watcher
            file_watcher: None,
            file_watcher_receiver: None,
        };
        
        // Setup the file watcher for the default folder
        overlay.update_file_watcher();
        
        // Initial dataset refresh
        overlay.refresh_datasets();
        
        overlay
    }
    
    // Helper function to get the default datasets folder
    fn get_default_datasets_folder() -> PathBuf {
        // Try to use the user's documents folder first
        let path = if let Some(docs_dir) = dirs::document_dir() {
            let brush_dir = docs_dir.join("Brush").join("Datasets");
            if !brush_dir.exists() {
                // Create the directory if it doesn't exist
                let _ = std::fs::create_dir_all(&brush_dir);
            }
            brush_dir
        } else if let Some(home_dir) = dirs::home_dir() {
            // Fall back to home directory if documents not available
            let brush_dir = home_dir.join("Brush").join("Datasets");
            if !brush_dir.exists() {
                // Create the directory if it doesn't exist
                let _ = std::fs::create_dir_all(&brush_dir);
            }
            brush_dir
        } else {
            // Last resort: use current directory
            let brush_dir = PathBuf::from("Brush_Datasets");
            if !brush_dir.exists() {
                // Create the directory if it doesn't exist
                let _ = std::fs::create_dir_all(&brush_dir);
            }
            brush_dir
        };
        
        path
    }
    
    // Function to set the selected folder
    pub(crate) fn set_selected_folder(&mut self, folder: PathBuf) {
        // Store the new folder
        self.datasets_folder = Some(folder);
        
        // Cancel any pending folder selection
        self.folder_selection_in_progress = false;
        self.show_folder_dialog = false;
        
        // Update the file watcher for the new folder
        self.update_file_watcher();
        
        // Refresh the dataset list
        self.refresh_datasets();
        
        // Process any pending file import
        if let Some(file_path) = self.pending_file_import.clone() {
            if self.copy_datasets_to_local {
                // Copy to local datasets folder
                if let Some(dataset_folder) = &self.datasets_folder {
                    self.process_selected_file(file_path.clone(), dataset_folder.clone());
                }
            }
        }
    }
    
    // Calculate desired window height based on dataset count
    fn calculate_window_height(&mut self) {
        // Store previous dataset count
        let old_count = self.last_dataset_count;
        let new_count = self.datasets.len();
        
        // Only recalculate if dataset count changed
        if old_count != new_count {
            println!("DATASET DEBUG: Dataset count changed from {} to {}", old_count, new_count);
            
            // Base height calculation
            let base_height = 450.0;
            let height_per_dataset = 70.0;
            
            // Determine appropriate height with reasonable limits
            let dataset_height = if new_count == 0 {
                200.0 // Minimum for empty state
            } else {
                // Calculate height based on dataset count (up to 12 datasets)
                let count = new_count.min(12);
                count as f32 * height_per_dataset
            };
            
            // Calculate total height with minimum and maximum constraints
            let new_height = (base_height + dataset_height).max(600.0).min(1200.0);
            
            // Debug logs
            println!("DATASET DEBUG: Calculated new window height:");
            println!("  - Base height: {}", base_height);
            println!("  - Dataset count: {}", new_count);
            println!("  - Dataset area height: {}", dataset_height);
            println!("  - Current height: {}", self.size.y);
            println!("  - New height: {}", new_height);
            
            // Set the new height and mark as changed
            self.size.y = new_height;
            self.height_changed = true;
            self.last_dataset_count = new_count;
        }
    }
    
    // Public method to refresh the dataset list
    pub(crate) fn refresh_datasets(&mut self) {
        self.refresh_datasets_internal();
        self.calculate_window_height();
    }
    
    // Internal method that performs the actual refresh
    fn refresh_datasets_internal(&mut self) {
        // If no folder is set, use the default
        if self.datasets_folder.is_none() {
            self.datasets_folder = Some(Self::get_default_datasets_folder());
        }
        
        if let Some(folder) = &self.datasets_folder {
            // Ensure the directory exists
            if !folder.exists() {
                if let Err(e) = std::fs::create_dir_all(folder) {
                    println!("Error creating datasets folder: {}", e);
                    return;
                }
            }
            
            self.datasets.clear();
            let mut dataset_count = 0;
            
            // Special case for the lego folder - check if it exists in the datasets folder
            let lego_folder = folder.join("lego");
            if lego_folder.exists() && lego_folder.is_dir() {
                if self.is_valid_dataset_folder(&lego_folder) || true { // Force include lego folder for debugging
                    if let Ok(metadata) = std::fs::metadata(&lego_folder) {
                        let size = self.get_folder_size(&lego_folder).unwrap_or(0);
                        self.datasets.push(DatasetEntry {
                            name: "lego".to_string(),
                            path: lego_folder.clone(),
                            size,
                            modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                            processed: false,
                        });
                        dataset_count += 1;
                    }
                }
            }
            
            if let Ok(entries) = std::fs::read_dir(folder) {
                // First, collect all entries to process them
                let mut all_entries = Vec::new();
                for entry in entries.flatten() {
                    // Skip the lego folder since we already handled it
                    if entry.path().file_name().map_or(false, |name| name == "lego") {
                        continue;
                    }
                    all_entries.push(entry);
                }
                
                // Process folders first (our preferred format)
                for entry in &all_entries {
                    let path = entry.path();
                    if path.is_dir() {
                        // Check if this folder contains dataset files (simple heuristic)
                        let is_dataset_folder = self.is_valid_dataset_folder(&path);
                        
                        if is_dataset_folder {
                            if let (Ok(metadata), Some(dirname)) = (entry.metadata(), path.file_name()) {
                                // Calculate folder size
                                let size = self.get_folder_size(&path).unwrap_or(0);
                                
                                self.datasets.push(DatasetEntry {
                                    name: dirname.to_string_lossy().to_string(),
                                    path: path.clone(),
                                    size,
                                    modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                                    processed: false, // For now, assume not processed
                                });
                                
                                dataset_count += 1;
                            }
                        }
                    }
                }
                
                // Then process zip files (legacy format)
                for entry in &all_entries {
                    let path = entry.path();
                    if path.is_file() && path.extension().map_or(false, |ext| ext == "zip") {
                        // Check if we already have a folder with the same name (stem)
                        let file_stem = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                        let folder_exists = self.datasets.iter().any(|d| {
                            d.path.file_name().map_or(false, |name| name.to_string_lossy() == file_stem)
                        });
                        
                        // Only add the zip if we don't have a corresponding folder
                        if !folder_exists {
                            if let (Ok(metadata), Some(filename)) = (entry.metadata(), path.file_name()) {
                                self.datasets.push(DatasetEntry {
                                    name: filename.to_string_lossy().to_string(),
                                    path: path.clone(),
                                    size: metadata.len(),
                                    modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                                    processed: false, // For now, assume not processed
                                });
                                
                                dataset_count += 1;
                            }
                        }
                    }
                }
                
                // Sort datasets by modified time (newest first)
                self.datasets.sort_by(|a, b| b.modified.cmp(&a.modified));
            }
            
            println!("Loaded {} datasets from folder: {}", dataset_count, folder.to_string_lossy());
        }
    }
    
    // Helper method to check if a folder is a valid dataset folder
    fn is_valid_dataset_folder(&self, folder_path: &PathBuf) -> bool {
        // Check if the folder exists and is a directory
        if !folder_path.exists() || !folder_path.is_dir() {
            return false;
        }
        
        // Check for common dataset folder structures
        let has_images = folder_path.join("images").exists() && folder_path.join("images").is_dir();
        let has_colmap = folder_path.join("sparse").exists() && folder_path.join("sparse").is_dir();
        let has_transforms = folder_path.join("transforms.json").exists();
        
        // Valid if it has images and either colmap or transforms.json
        let is_valid = has_images && (has_colmap || has_transforms);
        
        if is_valid {
            // This is a valid dataset folder
            return true;
        }
        
        // Not a valid dataset folder
        false
    }
    
    // Function to check if the overlay wants to select a folder
    pub(crate) fn wants_to_select_folder(&self) -> bool {
        self.show_folder_dialog && !self.folder_selection_in_progress
    }
    
    // Function to mark that folder selection has started
    pub(crate) fn folder_selection_started(&mut self) {
        self.folder_selection_in_progress = true;
    }
    
    // Function to cancel folder selection
    pub(crate) fn cancel_folder_selection(&mut self) {
        self.show_folder_dialog = false;
        self.folder_selection_in_progress = false;
    }
    
    // New method to handle adding a dataset file
    pub(crate) fn add_dataset_file(&mut self) {
        // Set a flag to indicate we want to select a file
        self.show_file_dialog = true;
    }
    
    // New method to handle the file selection result
    pub(crate) fn file_selection_started(&mut self) {
        self.file_selection_in_progress = true;
    }
    
    // New method to cancel file selection
    pub(crate) fn cancel_file_selection(&mut self) {
        self.show_file_dialog = false;
        self.file_selection_in_progress = false;
        // Clear any pending file import to be safe
        self.pending_file_import = None;
    }
    
    // New method to check if we want to select a file
    pub(crate) fn wants_to_select_file(&self) -> bool {
        self.show_file_dialog && !self.file_selection_in_progress
    }
    
    // New method to handle the selected file
    pub(crate) fn set_selected_file(&mut self, file_path: PathBuf) {
        // Reset all file selection flags
        self.show_file_dialog = false;
        self.file_selection_in_progress = false;
        
        // Store the file path for processing
        self.pending_file_import = Some(file_path);
    }
    
    // Helper method to process a selected file
    fn process_selected_file(&mut self, file_path: PathBuf, dataset_folder: PathBuf) {
        // Get the filename and stem (name without extension)
        let _filename = file_path.file_name().unwrap_or_default().to_string_lossy().to_string();
        let file_stem = file_path.file_stem().unwrap_or_default().to_string_lossy().to_string();
        
        // For zip files, we'll extract them to a folder
        if file_path.extension().map_or(false, |ext| ext == "zip") {
            // Create the destination folder path
            let dest_folder = dataset_folder.join(&file_stem);
            
            // Check if the folder already exists
            if dest_folder.exists() {
                println!("DATASET DEBUG: Destination folder already exists: {:?}", dest_folder);
                
                // For now, just use the existing folder
                // In the future, we'll handle naming conflicts here
                
                // Add the folder to the dataset list if it's not already there
                let already_in_list = self.datasets.iter().any(|d| d.path == dest_folder);
                if !already_in_list {
                    if let Ok(size) = self.get_folder_size(&dest_folder) {
                        // Add the folder to the list
                        self.datasets.push(DatasetEntry {
                            name: file_stem,
                            path: dest_folder,
                            size,
                            modified: SystemTime::now(), // Use current time since we're adding it now
                            processed: false,
                        });
                        
                        // Sort datasets by modified time (newest first)
                        self.datasets.sort_by(|a, b| b.modified.cmp(&a.modified));
                    }
                }
            } else {
                // Extract the zip file to the destination folder
                match self.extract_zip_file(&file_path, &dest_folder) {
                    Ok(_) => {
                        println!("DATASET DEBUG: Extracted zip file to folder: {:?}", dest_folder);
                        
                        // Calculate the folder size
                        if let Ok(size) = self.get_folder_size(&dest_folder) {
                            // Add the folder to the list
                            self.datasets.push(DatasetEntry {
                                name: file_stem,
                                path: dest_folder,
                                size,
                                modified: SystemTime::now(),
                                processed: false,
                            });
                            
                            // Sort datasets by modified time (newest first)
                            self.datasets.sort_by(|a, b| b.modified.cmp(&a.modified));
                        }
                    }
                    Err(err) => {
                        println!("DATASET DEBUG: Failed to extract zip file: {}", err);
                        
                        // Fall back to copying the zip file as-is
                        self.copy_zip_file_as_is(&file_path, &dataset_folder);
                    }
                }
            }
        } else {
            // For non-zip files, just copy them as before
            self.copy_zip_file_as_is(&file_path, &dataset_folder);
        }
    }
    
    // Helper method to copy a zip file as-is (fallback method)
    fn copy_zip_file_as_is(&mut self, file_path: &PathBuf, dataset_folder: &PathBuf) {
        // Get the filename
        let filename = file_path.file_name().unwrap_or_default().to_string_lossy().to_string();
        
        // Create the destination path
        let dest_path = dataset_folder.join(&filename);
        
        // Check if the file is already in the dataset folder
        if file_path != &dest_path {
            // Copy the file to the dataset folder
            match fs::copy(file_path, &dest_path) {
                Ok(_) => {
                    println!("DATASET DEBUG: Copied file to dataset folder: {:?}", dest_path);
                    
                    // Get file metadata
                    if let Ok(metadata) = fs::metadata(&dest_path) {
                        // Add the new dataset to the list
                        self.datasets.push(DatasetEntry {
                            name: filename,
                            path: dest_path,
                            size: metadata.len(),
                            modified: metadata.modified().unwrap_or(SystemTime::now()),
                            processed: false,
                        });
                        
                        // Sort datasets by modified time (newest first)
                        self.datasets.sort_by(|a, b| b.modified.cmp(&a.modified));
                    }
                }
                Err(err) => {
                    println!("DATASET DEBUG: Failed to copy file: {}", err);
                }
            }
        } else {
            println!("DATASET DEBUG: File is already in the dataset folder");
            
            // Check if it's already in our list
            let already_in_list = self.datasets.iter().any(|d| d.path == dest_path);
            if !already_in_list {
                // Add it to our list
                if let Ok(metadata) = fs::metadata(&dest_path) {
                    self.datasets.push(DatasetEntry {
                        name: filename,
                        path: dest_path,
                        size: metadata.len(),
                        modified: metadata.modified().unwrap_or(SystemTime::now()),
                        processed: false,
                    });
                    
                    // Sort datasets by modified time (newest first)
                    self.datasets.sort_by(|a, b| b.modified.cmp(&a.modified));
                }
            }
        }
    }
    
    fn format_size(size: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;
        
        if size < KB {
            format!("{} B", size)
        } else if size < MB {
            format!("{:.1} KB", size as f64 / KB as f64)
        } else if size < GB {
            format!("{:.1} MB", size as f64 / MB as f64)
        } else {
            format!("{:.1} GB", size as f64 / GB as f64)
        }
    }
    
    fn format_time(time: SystemTime) -> String {
        let now = SystemTime::now();
        if let Ok(duration) = now.duration_since(time) {
            let seconds = duration.as_secs();
            if seconds < 60 {
                "Just now".to_owned()
            } else if seconds < 3600 {
                format!("{} minutes ago", seconds / 60)
            } else if seconds < 86400 {
                format!("{} hours ago", seconds / 3600)
            } else {
                format!("{} days ago", seconds / 86400)
            }
        } else {
            "Unknown date".to_owned()
        }
    }
    
    // Function to request folder selection
    fn select_folder(&mut self) {
        // Just set the flag to request folder selection
        // The actual selection will be handled by the App
        self.show_folder_dialog = true;
    }
    
    pub(crate) fn is_open(&self) -> bool {
        self.open
    }
    
    pub(crate) fn set_open(&mut self, open: bool) {
        self.open = open;
    }
    
    pub(crate) fn show(&mut self, ctx: &Context, context: &mut AppContext) {
        if !self.open {
            return;
        }
        
        // Ensure we have a datasets folder set, use default if not
        if self.datasets_folder.is_none() {
            self.datasets_folder = Some(Self::get_default_datasets_folder());
            
            // Setup the file watcher for the default folder
            self.update_file_watcher();
        }
        
        // Check for file system changes
        self.check_for_file_changes();
        
        // Check if we have a pending file import
        if let Some(file_path) = self.pending_file_import.take() {
            if self.copy_datasets_to_local {
                // Copy to local datasets folder
                if let Some(folder) = &self.datasets_folder {
                    self.process_selected_file(file_path.clone(), folder.clone());
                }
            }
            
            // Always process the dataset directly, regardless of copy setting
            self.process_dataset(&file_path, context);
        }
        
        // Create a unique window ID - make it static to maintain window state
        let window_id = egui::Id::new("dataset_detail_window");
        
        // Track open state locally to avoid borrow issues
        let mut window_open = self.open;
        
        // Variables to track UI actions
        let mut should_select_folder = false;
        let mut should_refresh = false;
        let mut should_add_dataset = false;
        let mut dataset_to_process: Option<PathBuf> = None;
        
        // Create the window with settings to ensure proper resizability
        let window = egui::Window::new("Local Datasets")
            .id(window_id)
            .open(&mut window_open)
            .resizable(true)
            .movable(true)
            .collapsible(true)
            .default_pos(self.position)
            .default_size(self.size)
            .min_width(300.0)
            .min_height(300.0);
        
        // Show the window and get the response
        let response = window.show(ctx, |ui| {
            // Set a specific size for the content to ensure the window is resizable
            let available_size = ui.available_size();
            ui.set_max_width(available_size.x);
            ui.set_max_height(available_size.y);
            
            // Get the actual window size for debugging
            let _window_size = ui.available_size();
            
            // Create a vertical layout for the entire window content
            ui.vertical(|ui| {
                // Left panel with dataset list
                ui.vertical(|ui| {
                    // Add dataset section - MOVED TO TOP
                    ui.heading("Add Datasets");
                    ui.add_space(5.0);
                    
                    // Zip file selection
                    ui.horizontal(|ui| {
                        ui.label("Select a .zip file to add to your dataset collection.");
                        
                        // Always enable the Browse button
                        if ui.button("Browse Zip").clicked() {
                            should_add_dataset = true;
                        }
                    });
                    
                    // Folder selection
                    ui.horizontal(|ui| {
                        ui.label("Or select an existing dataset folder.");
                        
                        // Always enable the Browse Folder button
                        if ui.button("Browse Folder").clicked() {
                            self.select_dataset_folder();
                        }
                    });
                    
                    // Add checkbox for copying datasets
                    ui.checkbox(&mut self.copy_datasets_to_local, "Copy datasets to local folder");
                    ui.add_enabled_ui(!self.copy_datasets_to_local, |ui| {
                        ui.label(RichText::new("Dataset will be processed directly from source location").italics().small());
                    });
                    
                    // Show spinner when selecting a file
                    if self.file_selection_in_progress {
                        ui.horizontal(|ui| {
                            ui.spinner();
                            ui.label("Selecting file...");
                        });
                    }
                    
                    // Show spinner when selecting a folder
                    if self.dataset_folder_selection_in_progress {
                        ui.horizontal(|ui| {
                            ui.spinner();
                            ui.label("Selecting folder...");
                        });
                    }
                    
                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);
                    
                    // Local datasets section - MOVED AFTER ADD DATASETS
                    ui.horizontal(|ui| {
                        ui.heading("Local Datasets");
                        
                        // Add refresh button on the right
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            // Add refresh button
                            let refresh_button = ui.small_button("🔄");
                            if refresh_button.clicked() {
                                should_refresh = true;
                            }
                            
                            // Tooltip for refresh button
                            if refresh_button.hovered() {
                                refresh_button.on_hover_text("Manually refresh dataset list");
                            }
                        });
                    });
                    
                    // Show current folder in a compact display under header
                    if let Some(folder) = &self.datasets_folder {
                        // Clone the path to avoid borrow issues
                        let folder_path = folder.to_string_lossy().to_string();
                        
                        ui.horizontal(|ui| {
                            ui.style_mut().spacing.item_spacing = Vec2::new(4.0, 0.0);
                            ui.label(RichText::new("📁").small());
                            
                            // Make the path clickable to change the folder
                            let path_label = ui.add(
                                egui::Label::new(RichText::new(&folder_path).small().weak().underline())
                                    .sense(egui::Sense::click())
                            );
                            
                            if path_label.clicked() {
                                should_select_folder = true;
                            }
                            
                            if path_label.hovered() {
                                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                                path_label.on_hover_text("Click to change datasets folder");
                            }
                        });
                    } else {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("No datasets folder selected. ").italics().small().weak());
                            
                            // Add a button to select a folder
                            if ui.small_button("Select Folder").clicked() {
                                should_select_folder = true;
                            }
                        });
                    }
                    
                    if self.folder_selection_in_progress {
                        ui.horizontal(|ui| {
                            ui.spinner();
                            ui.label("Selecting folder...");
                        });
                    }
                    
                    ui.add_space(5.0);
                    
                    // Show dataset count and filter options in a horizontal bar
                    ui.horizontal(|ui| {
                        // Dataset count
                        let dataset_count_text = format!("{} datasets", self.datasets.len().max(0));
                        ui.label(RichText::new(dataset_count_text).strong());
                    });
                    
                    // Get remaining height after UI elements above
                    let _remaining_height = ui.available_height();
                    
                    // Create a scrollable area with a fixed maximum height
                    egui::ScrollArea::vertical()
                        .auto_shrink([false; 2]) // Don't shrink in either direction
                        .max_height(150.0) // Limit the height to ensure presets are visible
                        .show(ui, |ui| {
                            // Special case for empty datasets
                            if self.datasets.is_empty() {
                                ui.vertical_centered(|ui| {
                                    ui.add_space(20.0);
                                    ui.label("No datasets found");
                                    
                                    if let Some(_) = &self.datasets_folder {
                                        ui.label("Add .zip files to your datasets folder");
                                        if ui.button("Refresh List").clicked() {
                                            should_refresh = true;
                                        }
                                    } else {
                                        ui.label("Select a datasets folder to get started");
                                        if ui.button("Select Folder").clicked() {
                                            should_select_folder = true;
                                        }
                                    }
                                    ui.add_space(20.0);
                                });
                            } else {
                                // Draw the dataset list with a closure that can capture dataset_to_process
                                self.draw_dataset_list(ui, &mut dataset_to_process);
                            }
                        });
                        
                    // Add presets section at the bottom
                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(5.0);
                    
                    // Collapsible presets section - default to open
                    let presets_header = egui::CollapsingHeader::new("Presets")
                        .default_open(true) // Open by default
                        .show(ui, |ui| {
                        // Mipnerf scenes section
                        ui.heading("Mipnerf scenes");
                        
                        egui::Grid::new("mip_grid")
                            .num_columns(3)
                            .spacing([40.0, 4.0])
                            .striped(true)
                            .show(ui, |ui| {
                                self.add_url_button("bicycle", "https://drive.google.com/file/d/1LawlC-YjHSMl5rwRmEOMQEbJUioaYI5p/view?usp=drive_link", ui);
                                self.add_url_button("bonsai", "https://drive.google.com/file/d/1IWhmM49q_pfUZzJhA_vXv4POBODSAh32/view?usp=drive_link", ui);
                                self.add_url_button("counter", "https://drive.google.com/file/d/1564FHRsObZDGUlRx4RTFBTCi8jDPzTjj/view?usp=drive_link", ui);
                                ui.end_row();

                                self.add_url_button("garden", "https://drive.google.com/file/d/1WROBCrVu3YqA60mbRGmSRYXOJB4N5KAk/view?usp=drive_link", ui);
                                self.add_url_button("kitchen", "https://drive.google.com/file/d/1VSJM4b3pcQYiZj4xWSIIzHhwbzMcFWZv/view?usp=drive_link", ui);
                                self.add_url_button("room", "https://drive.google.com/file/d/1ieRBqlouADIAbCy8ryjI7M2PsfSNR23u/view?usp=drive_link", ui);
                                ui.end_row();

                                self.add_url_button("stump", "https://drive.google.com/file/d/1noPG4AowuT__xFV4uHODzOW7te9Kbb-T/view?usp=drive_link", ui);
                                ui.end_row();
                            });

                        // Synthetic blender scenes section
                        ui.heading("Synthetic blender scenes");
                        egui::Grid::new("blend_grid")
                            .num_columns(3)
                            .spacing([40.0, 4.0])
                            .striped(true)
                            .show(ui, |ui| {
                                self.add_url_button("chair", "https://drive.google.com/file/d/1EUcmoo5c2Ab9SiyWc8dZxbOxkEKWTU4C/view?usp=drive_link", ui);
                                self.add_url_button("drums", "https://drive.google.com/file/d/1UpBQoUJ9ShKgsyM7WaPy0a6qqtUMSOCx/view?usp=drive_link", ui);
                                self.add_url_button("ficus", "https://drive.google.com/file/d/1hwE1z0GSRHfMGXx3TyhuyqT-pDReeRik/view?usp=drive_link", ui);
                                ui.end_row();

                                self.add_url_button("hotdog", "https://drive.google.com/file/d/1EtIyCOyFAbTKHlMvNSwCFr5C1peyI107/view?usp=drive_link", ui);
                                self.add_url_button("lego", "https://drive.google.com/file/d/16TY5KxWUq7OzjkkLDBGNKZ0P5Laf-oaL/view?usp=drive_link", ui);
                                self.add_url_button("materials", "https://drive.google.com/file/d/1MWxV_NReK-UW4zKMbDIxQNiPwALZGSpd/view?usp=drive_link", ui);
                                ui.end_row();

                                self.add_url_button("mic", "https://drive.google.com/file/d/1s1PpJe71OECKnrUeNVdzjhKk-JXKlngI/view?usp=drive_link", ui);
                                self.add_url_button("ship", "https://drive.google.com/file/d/1Wvne6m7voRj8LvSosvq9vKMp8UYMCrER/view?usp=drive_link", ui);
                                ui.end_row();
                            });
                    });
                    
                    // Add hover effect for the header
                    if presets_header.header_response.hovered() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }
                });
            });
            
            // Return the dataset to process (if any)
            dataset_to_process
        });
        
        // Update self.open based on window_open
        if self.open != window_open {
            self.open = window_open;
        }
        
        // If select folder was requested, trigger it now (outside the closure)
        if should_select_folder {
            self.select_folder();
        }
        
        // If refresh was requested, trigger it now
        if should_refresh {
            self.refresh_datasets();
        }
        
        // If add dataset was requested, trigger it now
        if should_add_dataset {
            self.add_dataset_file();
        }
        
        // If a dataset was selected for processing, process it
        if let Some(inner_response) = response {
            if let Some(Some(dataset_path)) = inner_response.inner {
                self.process_dataset(&dataset_path, context);
                
                // Refresh the dataset list after processing to show any newly extracted folders
                self.refresh_datasets();
            }
            
            // Store window size for next frame
            self.size = inner_response.response.rect.size();
        }
    }

    // Helper method to draw the dataset list - updated to accept a mutable reference to dataset_to_process
    fn draw_dataset_list(&self, ui: &mut egui::Ui, dataset_to_process: &mut Option<PathBuf>) {
        // Get available height to adjust UI density
        let available_height = ui.available_height();
        let using_large_window = available_height > 600.0;
        
        // Get available width for dataset items
        let available_width = ui.available_width();
        let is_wide_layout = available_width > 400.0;
        
        // Define consistent styling
        let _row_height = if using_large_window { 70.0 } else { 50.0 }; // Unused but kept for reference
        let spacing = if using_large_window { 5.0 } else { 3.0 };
        let text_size = if using_large_window { 16.0 } else { 14.0 };
        
        // Consistent spacing
        ui.style_mut().spacing.item_spacing = Vec2::new(spacing, spacing);
        
        for dataset in &self.datasets {
            let is_selected = self.selected_dataset.as_ref().map_or(false, |sel| sel == &dataset.path);
            let path_clone = dataset.path.clone();
            
            // Create a card-like UI for each dataset
            let bg_color = if is_selected {
                Color32::from_rgba_premultiplied(40, 92, 189, 30)
            } else {
                Color32::from_rgba_premultiplied(0, 0, 0, 0)
            };
            
            // Use hover color for better interactivity feedback
            let hover_color = Color32::from_rgba_premultiplied(60, 100, 220, 20);
            
            // Create a custom frame for the dataset row
            let frame = egui::Frame::default()
                .fill(bg_color)
                .outer_margin(egui::Margin { left: 0, right: 0, top: 2, bottom: 2 })
                .inner_margin(egui::Margin { left: 8, right: 8, top: 6, bottom: 6 })
                .corner_radius(4.0);
                
            // Main dataset row
            let response = frame.show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Status indicator for dataset processing state
                    let status_color = if dataset.processed { 
                        Color32::GREEN 
                    } else { 
                        Color32::from_rgb(140, 140, 140)
                    };
                    
                    let (status_icon, status_text) = if dataset.processed {
                        ("✓", "Processed")
                    } else {
                        ("●", "Ready")
                    };
                    
                    // Determine if this is a folder or zip file
                    let is_folder = dataset.path.is_dir();
                    let type_icon = if is_folder { "📁" } else { "🗄️" };
                    
                    // Show type icon first
                    ui.label(RichText::new(type_icon).size(text_size));
                    
                    // Then status indicator
                    ui.label(RichText::new(status_icon).color(status_color).size(text_size));
                    
                    // Dataset details in vertical layout
                    ui.vertical(|ui| {
                        // Dataset name with larger text - for folders, show without extension
                        let display_name = if is_folder {
                            dataset.name.clone()
                        } else {
                            // For zip files, try to show without extension if possible
                            PathBuf::from(&dataset.name)
                                .file_stem()
                                .map_or_else(|| dataset.name.clone(), |s| s.to_string_lossy().to_string())
                        };
                        
                        ui.label(RichText::new(&display_name).size(text_size).strong());
                        
                        // Show dataset details in a horizontal layout for better density
                        ui.horizontal(|ui| {
                            ui.style_mut().spacing.item_spacing = Vec2::new(6.0, 0.0);
                            
                            // Show dataset type
                            let type_text = if is_folder { "Folder" } else { "Zip" };
                            ui.label(RichText::new(type_text).small().weak());
                            ui.label(RichText::new("•").small().weak());
                            
                            // Only show status text in wide layouts
                            if is_wide_layout {
                                ui.label(RichText::new(status_text).color(status_color).small());
                                ui.label(RichText::new("•").small().weak());
                            }
                            
                            // Always show size
                            ui.label(RichText::new(Self::format_size(dataset.size)).small().weak());
                            
                            // Always show last modified date
                            ui.label(RichText::new("•").small().weak());
                            ui.label(RichText::new(Self::format_time(dataset.modified)).small().weak());
                        });
                    });
                    
                    // Process button on the right
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Different button styles based on processing state
                        if dataset.processed {
                            if ui.button("View").clicked() {
                                // Handle view action
                                println!("Viewing dataset: {}", dataset.path.display());
                                // Set the dataset to be processed
                                *dataset_to_process = Some(dataset.path.clone());
                            }
                        } else {
                            let button_text = if using_large_window { "Process" } else { "▶" };
                            if ui.button(button_text).clicked() {
                                // Set the dataset to be processed
                                *dataset_to_process = Some(dataset.path.clone());
                            }
                        }
                    });
                });
            }).response;
            
            // Handle hover effect
            if response.hovered() {
                ui.painter().rect_filled(
                    response.rect,
                    4.0,
                    hover_color
                );
            }
            
            // Handle click to select dataset
            if response.clicked() {
                // Store response in UI memory for processing in the next frame
                ui.ctx().data_mut(|d| {
                    d.insert_temp(egui::Id::new("selected_dataset"), path_clone);
                });
            }
            
            // Subtle separator between datasets
            if !is_selected && !response.hovered() {
                let rect = response.rect;
                ui.painter().line_segment(
                    [egui::pos2(rect.left(), rect.bottom() + 1.0), egui::pos2(rect.right(), rect.bottom() + 1.0)],
                    egui::Stroke::new(1.0, Color32::from_rgba_premultiplied(100, 100, 100, 20))
                );
            }
            
            // Add a tiny bit of spacing between dataset rows
            ui.add_space(1.0);
        }
    }

    // New method to process a dataset file
    pub(crate) fn process_dataset(&self, dataset_path: &PathBuf, context: &mut AppContext) {
        // Check if the dataset path is a zip file
        if dataset_path.extension().map_or(false, |ext| ext == "zip") {
            // Create a folder name from the zip file name
            let folder_name = dataset_path.file_stem().unwrap_or_default().to_string_lossy().to_string();
            
            // Create a folder path in the same directory as the zip file
            let parent_dir = dataset_path.parent().unwrap_or_else(|| Path::new("."));
            let folder_path = parent_dir.join(&folder_name);
            
            // Check if the folder already exists and has content
            let folder_exists = folder_path.exists() && folder_path.is_dir();
            let folder_has_content = folder_exists && 
                std::fs::read_dir(&folder_path).map(|mut d| d.next().is_some()).unwrap_or(false);
            
            if folder_exists && folder_has_content {
                // Folder exists and has content, use it directly
                self.process_extracted_dataset(&folder_path, context);
            } else {
                // Folder doesn't exist or is empty, extract the zip file
                if !folder_exists {
                    if let Err(e) = std::fs::create_dir_all(&folder_path) {
                        println!("Error creating folder for zip extraction: {}", e);
                        return;
                    }
                }
                
                // Extract the zip file
                match self.extract_zip_file(dataset_path, &folder_path) {
                    Ok(_) => {
                        // Process the extracted dataset
                        self.process_extracted_dataset(&folder_path, context);
                    }
                    Err(e) => {
                        println!("Error extracting zip file: {}", e);
                        // Fall back to processing the zip directly
                        self.process_dataset_direct(dataset_path, context);
                    }
                }
            }
        } else {
            // Not a zip file, process directly
            self.process_dataset_direct(dataset_path, context);
        }
    }
    
    // Helper method to process a dataset directly
    fn process_dataset_direct(&self, dataset_path: &PathBuf, context: &mut AppContext) {
        // Create a DataSource from the file path
        let source = DataSource::Path(dataset_path.to_string_lossy().to_string());
        
        // Start the processing pipeline
        let process = start_process(
            source,
            ProcessArgs::default(), // Use default processing args
            context.device.clone(),
        );
        
        // Connect to the process
        context.connect_to(process);
    }
    
    // Helper method to process an extracted dataset
    fn process_extracted_dataset(&self, folder_path: &PathBuf, context: &mut AppContext) {
        // Create a DataSource from the folder path
        let source = DataSource::Path(folder_path.to_string_lossy().to_string());
        
        // Start the processing pipeline
        let process = start_process(
            source,
            ProcessArgs::default(), // Use default processing args
            context.device.clone(),
        );
        
        // Connect to the process
        context.connect_to(process);
    }

    // Helper method to extract a zip file to a folder
    fn extract_zip_file(&self, zip_path: &PathBuf, dest_folder: &PathBuf) -> io::Result<()> {
        println!("DATASET DEBUG: Extracting zip file: {} to {}", 
            zip_path.display(), dest_folder.display());
        
        // Create the destination folder if it doesn't exist
        if !dest_folder.exists() {
            fs::create_dir_all(dest_folder)?;
        }
        
        // Open the zip file
        let file = fs::File::open(zip_path)?;
        let mut archive = ZipArchive::new(file)?;
        
        // Extract each file
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = match file.enclosed_name() {
                Some(path) => dest_folder.join(path),
                None => continue,
            };
            
            if file.name().ends_with('/') {
                // Create directory
                fs::create_dir_all(&outpath)?;
            } else {
                // Create parent directory if needed
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p)?;
                    }
                }
                
                // Create file
                let mut outfile = fs::File::create(&outpath)?;
                io::copy(&mut file, &mut outfile)?;
            }
        }
        
        println!("DATASET DEBUG: Extraction complete");
        Ok(())
    }
    
    // Helper method to calculate the size of a folder
    fn get_folder_size(&self, folder_path: &PathBuf) -> io::Result<u64> {
        let mut total_size = 0;
        
        if folder_path.is_dir() {
            for entry in fs::read_dir(folder_path)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_file() {
                    total_size += entry.metadata()?.len();
                } else if path.is_dir() {
                    total_size += self.get_folder_size(&path)?;
                }
            }
        }
        
        Ok(total_size)
    }
    
    // Helper method to copy a folder to the datasets folder
    fn copy_folder(&self, source_folder: &PathBuf, dest_folder: &PathBuf) -> io::Result<()> {
        println!("DATASET DEBUG: Copying folder: {} to {}", 
            source_folder.display(), dest_folder.display());
        
        // Create the destination folder if it doesn't exist
        if !dest_folder.exists() {
            fs::create_dir_all(dest_folder)?;
        }
        
        // Copy each file and subdirectory
        for entry in fs::read_dir(source_folder)? {
            let entry = entry?;
            let path = entry.path();
            let dest_path = dest_folder.join(path.file_name().unwrap());
            
            if path.is_file() {
                fs::copy(&path, &dest_path)?;
            } else if path.is_dir() {
                self.copy_folder(&path, &dest_path)?;
            }
        }
        
        println!("DATASET DEBUG: Folder copy complete");
        Ok(())
    }

    // Methods for dataset folder selection
    pub(crate) fn wants_to_select_dataset_folder(&self) -> bool {
        self.show_dataset_folder_dialog
    }
    
    pub(crate) fn dataset_folder_selection_started(&mut self) {
        self.dataset_folder_selection_in_progress = true;
        self.show_dataset_folder_dialog = false;
    }
    
    pub(crate) fn cancel_dataset_folder_selection(&mut self) {
        self.dataset_folder_selection_in_progress = false;
        self.show_dataset_folder_dialog = false;
    }
    
    // Method to request dataset folder selection
    fn select_dataset_folder(&mut self) {
        self.show_dataset_folder_dialog = true;
    }
    
    // Method to handle the selected dataset folder
    pub(crate) fn set_selected_dataset_folder(&mut self, folder_path: PathBuf) {
        // Reset selection flags
        self.dataset_folder_selection_in_progress = false;
        self.show_dataset_folder_dialog = false;
        
        // Store the original path for processing
        let original_path = folder_path.clone();
        
        // Process the folder based on the copy preference
        if self.copy_datasets_to_local {
            // Copy to local datasets folder
            if let Some(dataset_folder) = self.datasets_folder.clone() {
                // Get the folder name
                let folder_name = original_path.file_name().unwrap_or_default().to_string_lossy().to_string();
                
                // Create the destination folder path
                let dest_folder = dataset_folder.join(&folder_name);
                
                // Process the folder (copy it to the datasets folder)
                self.process_selected_dataset_folder(folder_path, dataset_folder);
                
                // Refresh the dataset list to show the new folder
                self.refresh_datasets();
                
                // Store the destination path for processing
                self.pending_file_import = Some(dest_folder);
            } else {
                // No dataset folder selected, so we need to select one first
                // Store the folder path for later processing
                self.pending_file_import = Some(original_path);
                
                // Prompt the user to select a folder
                self.select_folder();
            }
        } else {
            // Process directly without copying
            // Get the folder name
            let folder_name = original_path.file_name().unwrap_or_default().to_string_lossy().to_string();
            
            // Add the folder to our list if it's not already there
            let already_in_list = self.datasets.iter().any(|d| d.path == original_path);
            if !already_in_list {
                if let Ok(size) = self.get_folder_size(&original_path) {
                    self.datasets.push(DatasetEntry {
                        name: folder_name,
                        path: original_path.clone(),
                        size,
                        modified: SystemTime::now(),
                        processed: false,
                    });
                    
                    // Sort datasets by modified time (newest first)
                    self.datasets.sort_by(|a, b| b.modified.cmp(&a.modified));
                }
            }
            
            // Store the folder path for processing
            self.pending_file_import = Some(original_path);
        }
        
        // Make sure the window is open to show progress
        self.open = true;
    }
    
    // Helper method to process a selected dataset folder
    fn process_selected_dataset_folder(&mut self, folder_path: PathBuf, dataset_folder: PathBuf) {
        // Get the folder name
        let folder_name = folder_path.file_name().unwrap_or_default().to_string_lossy().to_string();
        
        // Create the destination folder path
        let dest_folder = dataset_folder.join(&folder_name);
        
        // Check if the folder already exists
        if dest_folder.exists() {
            // For now, just use the existing folder
            // In the future, we'll handle naming conflicts here
            
            // Add the folder to the dataset list if it's not already there
            let already_in_list = self.datasets.iter().any(|d| d.path == dest_folder);
            if !already_in_list {
                if let Ok(size) = self.get_folder_size(&dest_folder) {
                    // Add the folder to the list
                    self.datasets.push(DatasetEntry {
                        name: folder_name,
                        path: dest_folder,
                        size,
                        modified: SystemTime::now(), // Use current time since we're adding it now
                        processed: false,
                    });
                    
                    // Sort datasets by modified time (newest first)
                    self.datasets.sort_by(|a, b| b.modified.cmp(&a.modified));
                }
            }
        } else {
            // Copy the folder to the destination
            match self.copy_folder(&folder_path, &dest_folder) {
                Ok(_) => {
                    // Calculate the folder size
                    if let Ok(size) = self.get_folder_size(&dest_folder) {
                        // Add the folder to the list
                        self.datasets.push(DatasetEntry {
                            name: folder_name,
                            path: dest_folder,
                            size,
                            modified: SystemTime::now(),
                            processed: false,
                        });
                        
                        // Sort datasets by modified time (newest first)
                        self.datasets.sort_by(|a, b| b.modified.cmp(&a.modified));
                    }
                }
                Err(err) => {
                    println!("Failed to copy folder: {}", err);
                }
            }
        }
    }

    // Setup file watcher for the datasets folder
    fn setup_file_watcher(&mut self) {
        // Clean up any existing watcher
        self.file_watcher = None;
        self.file_watcher_receiver = None;
        
        // Only setup watcher if we have a datasets folder
        if let Some(folder) = &self.datasets_folder {
            // Create a channel to receive file system events
            let (tx, rx) = channel();
            self.file_watcher_receiver = Some(rx);
            
            // Create a new watcher
            match recommended_watcher(tx) {
                Ok(mut watcher) => {
                    // Watch the datasets folder recursively
                    if let Err(e) = watcher.watch(folder, RecursiveMode::Recursive) {
                        println!("Error watching folder: {:?}", e);
                    } else {
                        self.file_watcher = Some(Box::new(watcher));
                    }
                }
                Err(e) => println!("Error creating watcher: {:?}", e),
            }
        }
    }
    
    // Check for file system events and refresh datasets if needed
    fn check_for_file_changes(&mut self) {
        if let Some(rx) = &self.file_watcher_receiver {
            // Non-blocking check for file system events
            let mut should_refresh = false;
            
            // Process all pending events
            while let Ok(event) = rx.try_recv() {
                match event {
                    Ok(_) => {
                        should_refresh = true;
                    }
                    Err(e) => println!("Watch error: {:?}", e),
                }
            }
            
            // Refresh datasets if changes were detected
            if should_refresh {
                self.refresh_datasets();
            }
        }
    }
    
    // Update the file watcher when the datasets folder changes
    fn update_file_watcher(&mut self) {
        if let Some(folder) = &self.datasets_folder {
            // Check if the folder exists
            if !folder.exists() {
                if let Err(e) = std::fs::create_dir_all(folder) {
                    println!("Error creating datasets folder: {}", e);
                    return;
                }
            }
            
            // Setup the file watcher for the new folder
            self.setup_file_watcher();
        }
    }

    // Helper method for URL buttons with tooltips
    fn add_url_button(&self, label: &str, url: &str, ui: &mut egui::Ui) {
        let hyperlink = ui.add(Hyperlink::from_label_and_url(label, url).open_in_new_tab(true));
        
        // Add tooltip on hover to show the full URL
        if hyperlink.hovered() {
            hyperlink.on_hover_text(url);
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
    }
} 