use crate::app::AppContext;
use brush_train::scene::{Scene, SceneView, ViewImageType, ViewType};
use egui::{Context, Hyperlink, Pos2, Rect, Sense, Slider, TextureHandle, TextureOptions, Vec2, pos2};
use std::path::PathBuf;
use std::time::SystemTime;
use egui::{Color32, RichText};
use dirs;

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
    pending_reopen: bool,          // Flag to reopen window after sizing
    reopen_timer: f32,             // Timer for delayed reopen
    auto_open_done: bool,          // Flag for auto-open on startup
    window_state: i32,             // Window state for extreme resize demo
    resize_timer: f32,             // Timer for resize
}

// Helper function for URL buttons
fn url_button(label: &str, url: &str, ui: &mut egui::Ui) {
    ui.add(Hyperlink::from_label_and_url(label, url).open_in_new_tab(true));
}

impl DatasetDetailOverlay {
    pub(crate) fn new() -> Self {
        Self {
            // Dataset browser fields
            datasets_folder: None,
            datasets: Vec::new(),
            show_folder_dialog: false,
            folder_selection_in_progress: false,
            
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
            pending_reopen: false,
            reopen_timer: 0.0,
            auto_open_done: false,
            window_state: 0,  // Start with normal window
            resize_timer: 2.0, // Wait 2 seconds before first resize
        }
    }
    
    // Function to set the selected folder
    pub(crate) fn set_selected_folder(&mut self, folder: PathBuf) {
        self.datasets_folder = Some(folder);
        self.show_folder_dialog = false;
        self.folder_selection_in_progress = false;
        
        // Refresh datasets
        self.refresh_datasets_internal();
        
        // Request a window reopen for proper sizing
        self.request_window_reopen();
    }
    
    // Request a window reopen with a short delay
    fn request_window_reopen(&mut self) {
        println!("Requesting window reopen for {} datasets", self.datasets.len());
        self.pending_reopen = true;
        self.reopen_timer = 0.2; // Short delay before reopening
    }
    
    // Update the reopen process - called each frame
    fn update_reopen_process(&mut self, ctx: &Context) {
        if self.pending_reopen {
            // Decrement the timer
            let dt = ctx.input(|i| i.unstable_dt).min(0.1);
            self.reopen_timer -= dt;
            
            if self.reopen_timer <= 0.0 {
                // Calculate new window size based on dataset count
                let base_height = 400.0;
                let height_per_dataset = 80.0;
                
                // Determine appropriate height
                let dataset_section_height = if self.datasets.is_empty() {
                    100.0 // Minimum when empty
                } else {
                    // Use dataset count to determine height, with reasonable limits
                    (self.datasets.len() as f32 * height_per_dataset)
                        .min(600.0) // Cap at 600px for dataset area
                        .max(200.0) // At least 200px for dataset area
                };
                
                // Set new window size
                let new_height = base_height + dataset_section_height;
                println!("Reopening window with height: {}", new_height);
                
                // Store current window position
                let current_pos = self.position;
                
                // Close and prepare to reopen
                self.open = false;
                self.size = Vec2::new(800.0, new_height);
                self.position = current_pos;
                
                // Set a flag to reopen next frame
                ctx.data_mut(|d| {
                    d.insert_temp(egui::Id::new("reopen_datasets_window"), true);
                });
                
                // Reset reopen flag
                self.pending_reopen = false;
            }
        } else {
            // Check if we need to reopen the window
            let mut should_reopen = false;
            ctx.data_mut(|d| {
                if d.get_temp::<bool>(egui::Id::new("reopen_datasets_window")).unwrap_or(false) {
                    should_reopen = true;
                    d.remove::<bool>(egui::Id::new("reopen_datasets_window"));
                }
            });
            
            if should_reopen {
                println!("Reopening window");
                self.open = true;
            }
        }
    }
    
    // Public method to refresh the dataset list
    pub(crate) fn refresh_datasets(&mut self) {
        self.refresh_datasets_internal();
        // Request window reopen after dataset refresh
        self.request_window_reopen();
    }
    
    // Internal method that performs the actual refresh
    fn refresh_datasets_internal(&mut self) {
        if let Some(folder) = &self.datasets_folder {
            self.datasets.clear();
            
            // Store folder path for logging to avoid borrowing issues
            let folder_path = folder.clone();
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
                
                // Sort alphabetically
                self.datasets.sort_by(|a, b| a.name.cmp(&b.name));
                
                // Store count for logging
                dataset_count = self.datasets.len();
            }
            
            // Log the dataset count for debugging
            println!("Loaded {} datasets from folder: {}", 
                dataset_count, 
                folder_path.display());
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
        // Update resize timer
        if self.resize_timer > 0.0 {
            let dt = ctx.input(|i| i.unstable_dt).min(0.1);
            self.resize_timer -= dt;
            
            if self.resize_timer <= 0.0 {
                // Advance window state
                self.window_state += 1;
                
                match self.window_state {
                    1 => {
                        // Small window
                        println!("EXTREME RESIZE: Small window");
                        self.size = Vec2::new(800.0, 300.0);
                        self.resize_timer = 1.5;
                    },
                    2 => {
                        // Huge window
                        println!("EXTREME RESIZE: Large window");
                        self.size = Vec2::new(800.0, 1200.0);
                        self.resize_timer = 1.5;
                    },
                    3 => {
                        // Dataset-sized window
                        let height = 400.0 + (self.datasets.len() as f32 * 80.0).min(600.0);
                        println!("EXTREME RESIZE: Dataset window height {}", height);
                        self.size = Vec2::new(800.0, height);
                        self.resize_timer = 1.5;
                    },
                    _ => {
                        // Back to start
                        self.window_state = 0;
                    }
                }
                
                // Force window close and reopen
                self.open = false;
                ctx.request_repaint(); // Force immediate repaint
                return;
            }
        }
        
        // First update the reopen process - this may close the window
        self.update_reopen_process(ctx);
        
        // Auto-open on startup
        if !self.auto_open_done {
            self.open = true;
            self.auto_open_done = true;
            
            // Set default folder if none is set
            if self.datasets_folder.is_none() {
                // Try to find a likely dataset folder
                if let Some(home) = dirs::home_dir() {
                    // Check several possible locations for datasets
                    let possible_dirs = [
                        home.join("Downloads"),
                        home.join("Documents"),
                        home.join("Pictures"),
                    ];
                    
                    for dir in possible_dirs {
                        if dir.exists() {
                            println!("Setting default datasets folder to {}", dir.display());
                            self.set_selected_folder(dir);
                            break;
                        }
                    }
                }
            }
        }
        
        if !self.open {
            // If not open, check if we just closed in a resize process
            if self.window_state > 0 {
                self.open = true;
                ctx.request_repaint(); // Force immediate repaint
            }
            return;
        }
        
        // Create a unique window ID based on current state to force new windows each time
        let window_id = format!("dataset_detail_overlay_{}", self.window_state);
        
        // Create the window with our size
        let window = egui::Window::new(format!("Datasets - Size {}", self.size.y))
            .id(egui::Id::new(window_id))
            .resizable(true)
            .movable(true)
            .collapsible(false)
            .title_bar(true)
            .default_pos(self.position)
            .fixed_size(self.size)
            .min_width(600.0)
            .min_height(400.0);
            
        window.show(ctx, |ui| {
            // Force the actual UI size to match our desired size
            ui.set_min_size(self.size);
            ui.set_max_size(self.size);
            
            // Store the window position for next frame
            self.position = ui.max_rect().left_top();
            
            // In normal state, save the natural size
            if self.window_state == 0 {
                self.size = ui.max_rect().size();
            }
            
            // Show current dimensions in title
            ui.heading(format!("Window Dimensions: {} x {}", 
                ui.max_rect().width() as i32, 
                ui.max_rect().height() as i32));
            
            // Check if we have a dataset selection from the previous frame
            ctx.data_mut(|d| {
                if let Some(path) = d.get_temp::<PathBuf>(egui::Id::new("selected_dataset")) {
                    self.selected_dataset = Some(path.clone());
                    // Remove the data to avoid duplicating the selection
                    d.remove::<PathBuf>(egui::Id::new("selected_dataset"));
                }
            });
            
            // Add close button to the upper right
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    if ui.button("X").clicked() {
                        self.open = false;
                        // Reset window state to avoid weird behavior
                        self.window_state = 0;
                    }
                });
            });
            
            // Use a horizontal layout with left and right panels
            ui.horizontal(|ui| {
                // Left panel - Dataset browser
                ui.vertical(|ui| {
                    ui.set_min_width(280.0);
                    ui.set_max_width(350.0); // Increased from 300.0
                    
                    // Get the total available height for this panel
                    let total_height = ui.available_height();
                    
                    // Calculate reasonable heights for different sections
                    let header_height = 40.0; // Local Datasets heading
                    let folder_controls_height = 70.0; // Folder selection controls
                    let dataset_count_height = 30.0; // Dataset count text
                    let public_datasets_height = if total_height > 1000.0 { 200.0 } else { 0.0 }; // Public datasets section
                    let spacing_height = 20.0; // Various spacing
                    
                    // Calculate available height for dataset table
                    let max_dataset_table_height = total_height - (header_height + folder_controls_height + 
                        dataset_count_height + public_datasets_height + spacing_height);
                    
                    // Show debug info
                    let debug_label = format!(
                        "Window: {:.0}x{:.0}, Available: {:.0}, Table: {:.0}", 
                        self.size.x, self.size.y, total_height, max_dataset_table_height
                    );
                    ui.label(RichText::new(debug_label).weak().small());
                    
                    // Local Datasets Section
                    ui.heading("Local Datasets");
                    
                    ui.add_space(5.0);
                    
                    // Dataset folder configuration
                    ui.horizontal(|ui| {
                        let folder_text = if let Some(folder) = &self.datasets_folder {
                            folder.to_string_lossy().to_string()
                        } else {
                            "No folder selected".to_owned()
                        };
                        
                        if ui.small_button("📁").clicked() {
                            self.select_folder();
                        }
                        ui.label(RichText::new(folder_text).monospace().small());
                    });
                    
                    if ui.button("Select Dataset Folder").clicked() {
                        self.select_folder();
                    }
                    
                    if self.folder_selection_in_progress {
                        ui.horizontal(|ui| {
                            ui.spinner();
                            ui.label("Selecting folder...");
                        });
                    }
                    
                    ui.add_space(5.0);
                    ui.separator();
                    ui.add_space(5.0);
                    
                    // Local dataset listing
                    let dataset_count_text = if self.datasets.is_empty() {
                        "No datasets found".to_owned()
                    } else {
                        format!("{} datasets", self.datasets.len())
                    };
                    ui.label(RichText::new(dataset_count_text).strong());
                    
                    // DATASET TABLE: Force to use most of available height
                    let dataset_area = egui::ScrollArea::vertical()
                        .max_height(max_dataset_table_height)
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            self.draw_dataset_list(ui);
                        });
                    
                    // Store the actual height used
                    self.last_table_height = dataset_area.inner_rect.height();
                    
                    // Only show public datasets if we have enough space
                    if public_datasets_height > 0.0 {
                        ui.add_space(5.0);
                        ui.heading("Public Datasets");
                        ui.add_space(5.0);
                        
                        // Public datasets in collapsing sections - force compact display
                        egui::CollapsingHeader::new("MipNeRF Scenes")
                            .default_open(true)
                            .show(ui, |ui| {
                                ui.style_mut().spacing.item_spacing = egui::Vec2::new(5.0, 2.0);
                                egui::Grid::new("mip_grid")
                                    .num_columns(2)
                                    .spacing([5.0, 2.0])
                                    .striped(true)
                                    .show(ui, |ui| {
                                        url_button("bicycle", "https://drive.google.com/file/d/1LawlC-YjHSMl5rwRmEOMQEbJUioaYI5p/view?usp=drive_link", ui);
                                        url_button("bonsai", "https://drive.google.com/file/d/1IWhmM49q_pfUZzJhA_vXv4POBODSAh32/view?usp=drive_link", ui);
                                        ui.end_row();
                                        
                                        url_button("counter", "https://drive.google.com/file/d/1564FHRsObZDGUlRx4RTFBTCi8jDPzTjj/view?usp=drive_link", ui);
                                        url_button("garden", "https://drive.google.com/file/d/1WROBCrVu3YqA60mbRGmSRYXOJB4N5KAk/view?usp=drive_link", ui);
                                        ui.end_row();
                                        
                                        url_button("kitchen", "https://drive.google.com/file/d/1VSJM4b3pcQYiZj4xWSIIzHhwbzMcFWZv/view?usp=drive_link", ui);
                                        url_button("room", "https://drive.google.com/file/d/1ieRBqlouADIAbCy8ryjI7M2PsfSNR23u/view?usp=drive_link", ui);
                                        ui.end_row();
                                    });
                            });
                    }
                });
                
                ui.separator();
                
                // Right panel - Dataset details / viewer
                ui.vertical(|ui| {
                    if let Some(selected_path) = &self.selected_dataset {
                        ui.heading(format!("Dataset: {}", selected_path.file_name().unwrap_or_default().to_string_lossy()));
                        
                        // If the dataset is processed, show the detail view
                        if context.dataset.train.views.is_empty() {
                            ui.centered_and_justified(|ui| {
                                ui.label("Dataset not yet processed. Click 'Process' to begin.");
                            });
                        } else {
                            // Show the dataset detail view similar to the original implementation
                            let pick_scene = selected_scene(self.view_type, context).clone();
                            let mut nearest_view_ind = pick_scene.get_nearest_view(context.camera.local_to_world());

                            if let Some(nearest) = nearest_view_ind.as_mut() {
                                // Update image if dirty
                                let mut dirty = self.selected_view.is_none();

                                if let Some(view) = self.selected_view.as_ref() {
                                    dirty |= view.index != *nearest;
                                    dirty |= view.view_type != self.view_type;
                                }

                                if dirty {
                                    let view = &pick_scene.views[*nearest];
                                    let image = &view.image;
                                    let img_size = [image.width() as usize, image.height() as usize];
                                    let color_img = if image.color().has_alpha() {
                                        let data = image.to_rgba8().into_vec();
                                        egui::ColorImage::from_rgba_unmultiplied(img_size, &data)
                                    } else {
                                        egui::ColorImage::from_rgb(img_size, &image.to_rgb8().into_vec())
                                    };

                                    self.selected_view = Some(SelectedView {
                                        index: *nearest,
                                        view_type: self.view_type,
                                        texture_handle: ui.ctx().load_texture(
                                            "nearest_view_detail_tex",
                                            color_img,
                                            TextureOptions::default(),
                                        ),
                                    });
                                }

                                let view_count = pick_scene.views.len();

                                if let Some(selected) = self.selected_view.as_ref() {
                                    let selected_view = selected.get_view(context).clone();
                                    let texture_handle = &selected.texture_handle;

                                    let img_size = texture_handle.size();
                                    let available_size = ui.available_size() - Vec2::new(0.0, 100.0);
                                    
                                    // Calculate the size for the image to fit while maintaining aspect ratio
                                    let scale = (available_size.x / img_size[0] as f32)
                                        .min(available_size.y / img_size[1] as f32)
                                        .min(1.0); // Don't scale up images
                                        
                                    let size = Vec2::new(
                                        img_size[0] as f32 * scale,
                                        img_size[1] as f32 * scale,
                                    ).round();
                                    
                                    // Center the image horizontally
                                    let avail_width = ui.available_width();
                                    let margin = (avail_width - size.x).max(0.0) / 2.0;
                                    ui.add_space(10.0);
                                    ui.horizontal(|ui| {
                                        ui.add_space(margin);
                                        
                                        let min = ui.cursor().min;
                                        let rect = Rect::from_min_size(min, size);

                                        match selected_view.img_type {
                                            ViewImageType::Alpha => {
                                                brush_ui::draw_checkerboard(ui, rect, egui::Color32::WHITE);
                                            }
                                            ViewImageType::Masked => {
                                                brush_ui::draw_checkerboard(ui, rect, egui::Color32::DARK_RED);
                                            }
                                        }

                                        ui.painter().image(
                                            texture_handle.id(),
                                            rect,
                                            egui::Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                                            egui::Color32::WHITE,
                                        );

                                        ui.allocate_rect(rect, Sense::click());
                                    });

                                    ui.add_space(10.0);
                                    
                                    ui.horizontal(|ui| {
                                        let mut interacted = false;
                                        if ui.button("⏪").clicked() {
                                            *nearest = (*nearest + view_count - 1) % view_count;
                                            interacted = true;
                                        }
                                        if ui
                                            .add(
                                                Slider::new(nearest, 0..=view_count - 1)
                                                    .suffix(format!("/ {view_count}"))
                                                    .custom_formatter(|num, _| format!("{}", num as usize + 1))
                                                    .custom_parser(|s| s.parse::<usize>().ok().map(|n| n as f64 - 1.0)),
                                            )
                                            .dragged()
                                        {
                                            interacted = true;
                                        }
                                        if ui.button("⏩").clicked() {
                                            *nearest = (*nearest + 1) % view_count;
                                            interacted = true;
                                        }

                                        ui.add_space(10.0);

                                        if context.dataset.eval.is_some() {
                                            for (t, l) in [ViewType::Train, ViewType::Eval]
                                                .into_iter()
                                                .zip(["train", "eval"])
                                            {
                                                if ui.selectable_label(self.view_type == t, l).clicked() {
                                                    self.view_type = t;
                                                    *nearest = 0;
                                                    interacted = true;
                                                };
                                            }
                                        }

                                        if interacted {
                                            context.focus_view(&pick_scene.views[*nearest]);
                                        }
                                    });

                                    ui.add_space(8.0);
                                    
                                    // Image metadata section
                                    ui.group(|ui| {
                                        ui.heading("Image Details");
                                        
                                        let mask_info = if selected_view.image.color().has_alpha() {
                                            if selected_view.img_type == ViewImageType::Alpha {
                                                "RGB + Alpha transparency"
                                            } else {
                                                "RGB, masked"
                                            }
                                        } else {
                                            "RGB"
                                        };

                                        ui.label(format!("Filename: {}", selected_view.path));
                                        ui.label(format!("Resolution: {}x{}", selected_view.image.width(), selected_view.image.height()));
                                        ui.label(format!("Format: {}", mask_info));
                                        
                                        // Camera position info
                                        let camera_transform = selected_view.camera.local_to_world();
                                        let camera_pos = camera_transform.translation;
                                        ui.label(format!(
                                            "Camera position: ({:.2}, {:.2}, {:.2})",
                                            camera_pos.x, camera_pos.y, camera_pos.z
                                        ));
                                        
                                        // Focus on this view button
                                        if ui.button("Focus on This View").clicked() {
                                            context.focus_view(&selected_view);
                                        }
                                    });
                                }
                            } else {
                                ui.label("No views available in the current dataset.");
                            }

                            if context.loading() {
                                ui.label("Loading...");
                            }
                        }
                    } else {
                        ui.centered_and_justified(|ui| {
                            ui.heading("Select a dataset from the list");
                        });
                    }
                });
            });
        });
    }

    // Helper method to draw the dataset list
    fn draw_dataset_list(&self, ui: &mut egui::Ui) {
        if self.datasets.is_empty() {
            ui.label("No datasets found in the selected folder.");
        } else {
            // Detect if we have a large window to adjust spacing
            let available_height = ui.available_height();
            let using_large_window = available_height > 700.0;
            let extra_spacing = if using_large_window { 5.0 } else { 0.0 };
            
            // Use more compact spacing for dataset list items in small windows
            let text_size = if using_large_window { 14.0 } else { 13.0 };
            
            // Reduce spacing between items in small windows
            if !using_large_window {
                ui.style_mut().spacing.item_spacing.y = 2.0;
            }
            
            // Styling helper for more compact rows
            let make_compact = |ui: &mut egui::Ui| {
                if !using_large_window {
                    ui.style_mut().spacing.item_spacing = egui::Vec2::new(4.0, 1.0);
                }
            };
            
            for dataset in &self.datasets {
                let is_selected = self.selected_dataset.as_ref().map_or(false, |sel| sel == &dataset.path);
                let path_clone = dataset.path.clone();
                
                // In large windows, add extra spacing
                if using_large_window {
                    ui.add_space(extra_spacing);
                }
                
                let response = ui.horizontal(|ui| {
                    // Make row layout more compact in small windows
                    make_compact(ui);
                    
                    ui.add_space(4.0);
                    
                    // Status icon
                    let status_icon = if dataset.processed { "✓" } else { "📁" };
                    let status_color = if dataset.processed { Color32::GREEN } else { Color32::GRAY };
                    ui.label(RichText::new(status_icon).color(status_color));
                    
                    // Dataset name and details
                    ui.vertical(|ui| {
                        // Make vertical layout more compact in small windows
                        make_compact(ui);
                        
                        // Set text size based on available space
                        ui.label(RichText::new(&dataset.name).size(text_size));
                        
                        // Only show details if we have room
                        if using_large_window {
                            ui.label(
                                RichText::new(format!(
                                    "{}, {}",
                                    Self::format_size(dataset.size),
                                    Self::format_time(dataset.modified)
                                ))
                                .weak()
                                .small()
                            );
                        } else {
                            // Compact format for small windows
                            ui.label(
                                RichText::new(format!("{}", Self::format_size(dataset.size)))
                                .weak()
                                .small()
                            );
                        }
                    });
                }).response;
                
                if response.clicked() {
                    // Store response in UI memory for processing in the next frame
                    ui.ctx().data_mut(|d| {
                        d.insert_temp(egui::Id::new("selected_dataset"), path_clone);
                    });
                }
                
                // Draw background for selected item - more subtle in small windows
                if is_selected {
                    let expand = if using_large_window { 5.0 } else { 2.0 };
                    let rect = response.rect.expand(expand);
                    ui.painter().rect_filled(
                        rect,
                        3.0,
                        Color32::from_rgba_premultiplied(100, 120, 200, 60)
                    );
                }
                
                // Process button for this dataset
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    // In small windows, use a smaller button
                    let button_text = if using_large_window { "Process" } else { "Run" };
                    if ui.button(button_text).clicked() {
                        // Start processing the dataset
                        let path = dataset.path.clone();
                        
                        // We would connect to the processing system here
                        println!("Would load dataset: {}", path.display());
                    }
                });
                
                // Subtle separator in small windows
                if using_large_window {
                    ui.separator();
                } else {
                    let rect = ui.max_rect();
                    let line_height = 1.0;
                    let line_rect = egui::Rect::from_min_size(
                        egui::pos2(rect.left(), rect.bottom() - line_height),
                        egui::vec2(rect.width(), line_height)
                    );
                    ui.painter().rect_filled(
                        line_rect,
                        0.0,
                        Color32::from_rgba_premultiplied(100, 100, 100, 20)
                    );
                    
                    // Less spacing after separator in small windows
                    ui.add_space(1.0);
                }
                
                // In large windows, add extra spacing after
                if using_large_window {
                    ui.add_space(extra_spacing);
                }
            }
        }
    }
} 