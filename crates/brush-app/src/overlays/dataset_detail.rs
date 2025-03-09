use crate::app::AppContext;
use brush_train::scene::{Scene, SceneView, ViewType};
use egui::{Color32, Context, Hyperlink, Pos2, RichText, TextureHandle, Vec2, pos2};
use std::path::PathBuf;
use std::time::SystemTime;
use std::fs;
use brush_process::data_source::DataSource;
use brush_process::process_loop::{ProcessArgs, start_process};

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
    auto_refresh_enabled: bool,    // New field for auto-refresh setting
    show_file_dialog: bool,        // New field for file selection
    file_selection_in_progress: bool, // New field for file selection
    
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
}

// Helper function for URL buttons
fn url_button(label: &str, url: &str, ui: &mut egui::Ui) {
    ui.add(Hyperlink::from_label_and_url(label, url).open_in_new_tab(true));
}

impl DatasetDetailOverlay {
    pub(crate) fn new() -> Self {
        println!("DATASET DEBUG: Creating new DatasetDetailOverlay");
        Self {
            // Dataset browser fields
            datasets_folder: None,
            datasets: Vec::new(),
            show_folder_dialog: false,
            folder_selection_in_progress: false,
            auto_refresh_enabled: true,    // Enable auto-refresh by default
            show_file_dialog: false,        // New field for file selection
            file_selection_in_progress: false, // New field for file selection
            
            // Detail view fields
            view_type: ViewType::Train,
            selected_view: None,
            selected_dataset: None,
            
            // UI state
            open: true, // Start with window open
            position: pos2(100.0, 100.0),
            size: Vec2::new(800.0, 600.0), // Start with medium size
            
            // For dynamic sizing
            last_table_height: 300.0,
            auto_open_done: false,
            height_changed: false,
            last_dataset_count: 0,
            prev_size: Vec2::new(0.0, 0.0),
            pending_file_import: None, // Initialize pending_file_import
        }
    }
    
    // Function to set the selected folder
    pub(crate) fn set_selected_folder(&mut self, folder: PathBuf) {
        println!("DATASET DEBUG: Before setting folder - window size is: {}x{}", self.size.x, self.size.y);
        
        self.datasets_folder = Some(folder.clone());
        self.show_folder_dialog = false;
        self.folder_selection_in_progress = false;
        
        // Refresh datasets and calculate new height
        self.refresh_datasets_internal();
        
        // Calculate and update window height
        self.calculate_window_height();
        
        // Check if we have a pending file import
        if let Some(file_path) = self.pending_file_import.take() {
            println!("DATASET DEBUG: Processing pending file import: {:?}", file_path);
            self.process_selected_file(file_path, folder);
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
        if let Some(folder) = &self.datasets_folder {
            // Store folder path for logging to avoid borrow issues
            let folder_path = folder.to_string_lossy().to_string();
            
            self.datasets.clear();
            let mut dataset_count = 0;
            
            if let Ok(entries) = std::fs::read_dir(folder) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() && path.extension().map_or(false, |ext| ext == "zip") {
                        if let (Ok(metadata), Some(filename)) = (entry.metadata(), path.file_name()) {
                            self.datasets.push(DatasetEntry {
                                name: filename.to_string_lossy().to_string(),
                                path: path.clone(),
                                size: metadata.len(),
                                modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                                processed: false, // For now, assume not processed
                            });
                        }
                    }
                }
                
                // Sort alphabetically - CHANGED to sort by date (newest first)
                self.sort_datasets_by_date();
                dataset_count = self.datasets.len();
            }
            
            // Log the dataset count for debugging
            println!("Loaded {} datasets from folder: {}", dataset_count, folder_path);
        }
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
    }
    
    // New method to check if we want to select a file
    pub(crate) fn wants_to_select_file(&self) -> bool {
        self.show_file_dialog && !self.file_selection_in_progress
    }
    
    // New method to handle the selected file
    pub(crate) fn set_selected_file(&mut self, file_path: PathBuf) {
        println!("DATASET DEBUG: Selected file: {:?}", file_path);
        
        // Reset file selection flags
        self.show_file_dialog = false;
        self.file_selection_in_progress = false;
        
        // Check if the file is a zip file
        if file_path.extension().map_or(false, |ext| ext == "zip") {
            // Check if we have a dataset folder
            if let Some(dataset_folder) = &self.datasets_folder {
                self.process_selected_file(file_path, dataset_folder.clone());
            } else {
                // No dataset folder selected, so we need to select one first
                println!("DATASET DEBUG: No dataset folder selected, prompting user to select one");
                
                // Store the file path for later processing
                self.pending_file_import = Some(file_path);
                
                // Prompt the user to select a folder
                self.select_folder();
            }
        } else {
            println!("DATASET DEBUG: Selected file is not a zip file");
        }
    }
    
    // Helper method to process a selected file
    fn process_selected_file(&mut self, file_path: PathBuf, dataset_folder: PathBuf) {
        // Get the filename
        let filename = file_path.file_name().unwrap_or_default().to_string_lossy().to_string();
        
        // Create the destination path
        let dest_path = dataset_folder.join(&filename);
        
        // Check if the file is already in the dataset folder
        if file_path != dest_path {
            // Copy the file to the dataset folder
            match fs::copy(&file_path, &dest_path) {
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
                        self.sort_datasets_by_date();
                    }
                }
                Err(err) => {
                    println!("DATASET DEBUG: Failed to copy file: {}", err);
                }
            }
        } else {
            println!("DATASET DEBUG: File is already in the dataset folder");
            
            // Check if the file is already in our list
            let already_in_list = self.datasets.iter().any(|d| d.path == dest_path);
            
            if !already_in_list {
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
                    self.sort_datasets_by_date();
                }
            }
        }
        
        // Recalculate window height
        self.calculate_window_height();
    }
    
    // Helper method to sort datasets by date (newest first)
    fn sort_datasets_by_date(&mut self) {
        self.datasets.sort_by(|a, b| b.modified.cmp(&a.modified));
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
        println!("DATASET DEBUG: is_open() called, returning: {}", self.open);
        self.open
    }
    
    pub(crate) fn set_open(&mut self, open: bool) {
        println!("DATASET DEBUG: set_open({}) called, was: {}", open, self.open);
        self.open = open;
    }
    
    pub(crate) fn show(&mut self, ctx: &Context, context: &mut AppContext) {
        println!("DATASET DEBUG: show() called, open state: {}", self.open);
        
        if !self.open {
            println!("DATASET DEBUG: Window is closed, returning early");
            return;
        }
        
        // Create a unique window ID - make it static to maintain window state
        let window_id = egui::Id::new("dataset_detail_window");
        
        println!("DATASET DEBUG: Creating window with ID: {:?}", window_id);
        
        // Track open state locally to avoid borrow issues
        let mut window_open = self.open;
        
        // Create the window with absolutely minimal settings to allow full resizing freedom
        let window = egui::Window::new("Datasets")
            .id(window_id)
            .open(&mut window_open) // Use local variable instead of self.open
            .resizable(true)
            .movable(true)
            .collapsible(false)
            .default_pos(self.position)
            .default_size(self.size)
            .min_width(400.0)
            .min_height(300.0); // Increased minimum height
            
        println!("DATASET DEBUG: Window configured, about to show");
        
        // Create a mutable copy of self.select_folder for the closure
        let mut should_select_folder = false;
        let mut should_refresh = false;
        let mut should_add_dataset = false;
        let mut dataset_to_process: Option<PathBuf> = None;
        
        let response = window.show(ctx, |ui| {
            println!("DATASET DEBUG: Inside window content closure");
            
            // Store the window position
            self.position = ui.max_rect().left_top();
            
            // Get the actual window size and ALWAYS update it
            let actual_size = ui.max_rect().size();
            
            // Debug log the actual window size
            println!("WINDOW DEBUG: Actual size: {}x{}", actual_size.x, actual_size.y);
            println!("WINDOW DEBUG: Available size: {}x{}", ui.available_width(), ui.available_height());
            
            // Store for next frame to detect changes - ALWAYS update
            self.size = actual_size;
            
            // CRITICAL CHANGE: Use a main vertical layout for the entire window content
            ui.vertical(|ui| {
                // Force UI to take all available space
                ui.set_min_size(egui::vec2(ui.available_width(), ui.available_height()));
                
                // Use SidePanel for left side which handles resizing properly
                egui::SidePanel::left("dataset_browser_panel")
                    .resizable(true)
                    .min_width(280.0)
                    .default_width(300.0)
                    .max_width(ui.available_width() * 0.8)
                    .show_inside(ui, |ui| {
                        // Force panel to use all available height
                        ui.set_min_height(ui.available_height());
                        println!("LAYOUT DEBUG: Left panel available height: {}", ui.available_height());
                        
                        // Add dataset section - MOVED TO TOP
                        ui.heading("Add Datasets");
                        ui.add_space(5.0);
                        
                        ui.horizontal(|ui| {
                            ui.label("Select a .zip file to add to your dataset collection.");
                            
                            // Always enable the Browse button
                            if ui.button("Browse").clicked() {
                                should_add_dataset = true;
                            }
                        });
                        
                        if self.file_selection_in_progress {
                            ui.horizontal(|ui| {
                                ui.spinner();
                                ui.label("Selecting file...");
                            });
                        }
                        
                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(10.0);
                        
                        // Local datasets section - MOVED AFTER ADD DATASETS
                        ui.horizontal(|ui| {
                            ui.heading("Local Datasets");
                            
                            // Add folder selection button on the right
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                // Just use the gear icon without a square button
                                let gear_btn = ui.link("⚙️");
                                if gear_btn.clicked() {
                                    ui.memory_mut(|mem| mem.toggle_popup(egui::Id::new("folder_settings")));
                                }
                                
                                // Use a simpler approach with popup menu - only for folder settings
                                egui::popup::popup_below_widget(ui, egui::Id::new("folder_settings"), &gear_btn, egui::popup::PopupCloseBehavior::CloseOnClickOutside, |ui: &mut egui::Ui| {
                                    ui.set_max_width(300.0);
                                    ui.set_min_width(200.0);
                                    
                                    ui.vertical(|ui| {
                                        ui.heading("Dataset Folder");
                                        ui.separator();
                                        
                                        // Current folder display
                                        if let Some(folder) = &self.datasets_folder {
                                            ui.horizontal(|ui| {
                                                ui.label("Current:");
                                                ui.label(RichText::new(folder.to_string_lossy()).monospace());
                                            });
                                        } else {
                                            ui.label("No folder selected");
                                        }
                                        
                                        // Folder selection button - use the local flag
                                        if ui.button("Select Folder").clicked() {
                                            should_select_folder = true;
                                            ui.close_menu();
                                        }
                                        
                                        ui.add_space(5.0);
                                        ui.separator();
                                        ui.add_space(5.0);
                                        
                                        // Auto-refresh setting
                                        ui.heading("Settings");
                                        let mut auto_refresh = self.auto_refresh_enabled;
                                        if ui.checkbox(&mut auto_refresh, "Auto-check for new files").changed() {
                                            self.auto_refresh_enabled = auto_refresh;
                                        }
                                        ui.label(RichText::new("Automatically scan for new datasets when app launches").small().weak());
                                        
                                        ui.add_space(5.0);
                                        
                                        // Manual refresh button
                                        if ui.button("Refresh Now").clicked() {
                                            should_refresh = true;
                                            ui.close_menu();
                                        }
                                    });
                                });
                            });
                        });
                        
                        // Show current folder in a compact display under header
                        if let Some(folder) = &self.datasets_folder {
                            // Clone the path to avoid borrow issues
                            let folder_path = folder.to_string_lossy().to_string();
                            
                            ui.horizontal(|ui| {
                                ui.style_mut().spacing.item_spacing = Vec2::new(4.0, 0.0);
                                ui.label(RichText::new("📁").small());
                                ui.label(RichText::new(&folder_path).small().weak());
                                
                                // Only show refresh button when we have a folder
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.small_button("🔄").clicked() {
                                        should_refresh = true;
                                    }
                                });
                            });
                        } else {
                            ui.label(RichText::new("Select a datasets folder using the settings button").italics().small().weak());
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
                        let remaining_height = ui.available_height();
                        println!("LAYOUT DEBUG: Remaining height for dataset list: {}", remaining_height);
                        
                        // Create a scrollable area that fills the remaining space
                        let dataset_area = egui::ScrollArea::vertical()
                            .auto_shrink([false; 2]) // Don't shrink in either direction
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
                        
                        // Log the actual size of the dataset area for debugging
                        println!("LAYOUT DEBUG: Dataset area size: {}x{}", 
                            dataset_area.inner_rect.width(), 
                            dataset_area.inner_rect.height());
                    });
                
                // Right panel for dataset detail view (if any)
                if let Some(_selected_path) = &self.selected_dataset {
                    egui::CentralPanel::default().show_inside(ui, |ui| {
                        ui.heading("Dataset Details");
                        // Dataset details would go here
                    });
                }
            });
            
            // Return close button state - no longer needed since we removed the button
            false
        });
        
        // Update self.open based on window_open
        if self.open != window_open {
            println!("DATASET DEBUG: Window open state changed: {} -> {}", self.open, window_open);
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
        if let Some(dataset_path) = dataset_to_process {
            self.process_dataset(&dataset_path, context);
        }
        
        // Log window response
        if let Some(inner_response) = &response {
            println!("DATASET DEBUG: Window response rect: {:?}", inner_response.response.rect);
            println!("DATASET DEBUG: Window inner: {:?}", inner_response.inner);
            
            // Store window size for next frame
            self.size = inner_response.response.rect.size();
        } else {
            println!("DATASET DEBUG: Window response is None (window was closed)");
            // Window was closed if no response
            self.open = false;
        }
        
        // Store the current size for the next frame to detect changes
        self.prev_size = self.size;
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
                    
                    ui.label(RichText::new(status_icon).color(status_color).size(text_size));
                    
                    // Dataset details in vertical layout
                    ui.vertical(|ui| {
                        // Dataset name with larger text
                        ui.label(RichText::new(&dataset.name).size(text_size).strong());
                        
                        // Show dataset details in a horizontal layout for better density
                        ui.horizontal(|ui| {
                            ui.style_mut().spacing.item_spacing = Vec2::new(6.0, 0.0);
                            
                            // Only show status text in wide layouts
                            if is_wide_layout {
                                ui.label(RichText::new(status_text).color(status_color).small());
                                ui.label(RichText::new("•").small().weak());
                            }
                            
                            // Always show size
                            ui.label(RichText::new(Self::format_size(dataset.size)).small().weak());
                            
                            // Always show last modified date (changed from only in large windows)
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
        println!("DATASET DEBUG: Processing dataset: {}", dataset_path.display());
        
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
} 