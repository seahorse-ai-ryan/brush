use std::sync::{Arc, RwLock};

use crate::channel::reactive_receiver;
use crate::orbit_controls::CameraController;
use crate::overlays::{DatasetDetailOverlay, SettingsDetailOverlay, StatsDetailOverlay, ControlsDetailOverlay};
use crate::panels::{DatasetPanel, ScenePanel};
#[cfg(feature = "tracing")]
use crate::panels::TracingPanel;
use brush_dataset::Dataset;
use brush_process::data_source::DataSource;
use brush_process::process_loop::{
    ControlMessage, ProcessArgs, ProcessMessage, RunningProcess, start_process,
};
use brush_render::camera::Camera;
use brush_render::gaussian_splats::Splats;
use brush_train::scene::SceneView;
use brush_train::train::TrainBack;
use burn::tensor::backend::AutodiffBackend;
use burn_wgpu::WgpuDevice;
use eframe::egui;
use egui_tiles::SimplificationOptions;
use egui_tiles::{Container, Tile, TileId, Tiles};
use glam::{Affine3A, Quat, Vec3};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use egui::{Align2, RichText};
use brush_dataset::splat_export;
use tokio_with_wasm::alias as tokio_wasm;
use crate::export_service::{ExportService, ExportError, ExportFormat};

#[cfg(not(target_family = "wasm"))]
use rfd;

pub(crate) trait AppPanel {
    fn title(&self) -> String;

    /// Draw the pane's UI's content/
    fn ui(&mut self, ui: &mut egui::Ui, controls: &mut AppContext);

    /// Handle an incoming message from the UI.
    fn on_message(&mut self, message: &ProcessMessage, context: &mut AppContext) {
        let _ = message;
        let _ = context;
    }

    /// Override the inner margin for this panel.
    fn inner_margin(&self) -> f32 {
        12.0
    }
}

struct AppTree {
    zen: bool,
    context: Arc<RwLock<AppContext>>,
}

type PaneType = Box<dyn AppPanel>;

impl egui_tiles::Behavior<PaneType> for AppTree {
    fn tab_title_for_pane(&mut self, pane: &PaneType) -> egui::WidgetText {
        pane.title().into()
    }

    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut PaneType,
    ) -> egui_tiles::UiResponse {
        egui::Frame::new()
            .inner_margin(pane.inner_margin())
            .show(ui, |ui| {
                pane.ui(ui, &mut self.context.write().expect("Lock poisoned"));
            });
        egui_tiles::UiResponse::None
    }

    /// What are the rules for simplifying the tree?
    fn simplification_options(&self) -> SimplificationOptions {
        SimplificationOptions {
            all_panes_must_have_tabs: false,
            ..Default::default()
        }
    }

    /// Width of the gap between tiles in a horizontal or vertical layout,
    /// and between rows/columns in a grid layout.
    fn gap_width(&self, _style: &egui::Style) -> f32 {
        if self.zen { 0.0 } else { 0.5 }
    }
}

fn parse_search(search: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    let search = search.trim_start_matches('?');

    for pair in search.split('&') {
        // Split each pair on '=' to separate key and value
        if let Some((key, value)) = pair.split_once('=') {
            // URL decode the key and value and insert into HashMap
            params.insert(
                urlencoding::decode(key).unwrap_or_default().into_owned(),
                urlencoding::decode(value).unwrap_or_default().into_owned(),
            );
        }
    }
    params
}

pub struct App {
    tree: egui_tiles::Tree<PaneType>,
    tree_ctx: AppTree,
    datasets: Option<TileId>,
    dataset_detail_overlay: DatasetDetailOverlay,
    settings_detail_overlay: SettingsDetailOverlay,
    stats_detail_overlay: StatsDetailOverlay,
    controls_detail_overlay: ControlsDetailOverlay,
    select_folder_requested: bool,
    select_file_requested: bool,
    select_dataset_folder_requested: bool,
}

pub struct AppContext {
    pub dataset: Dataset,
    pub camera: Camera,
    pub view_aspect: Option<f32>,
    pub controls: CameraController,
    pub model_local_to_world: Affine3A,
    pub device: WgpuDevice,

    loading: bool,
    training: bool,
    // Track the current dataset name for export filenames
    current_dataset_name: Option<String>,

    ctx: egui::Context,
    running_process: Option<RunningProcess<TrainBack>>,
    cam_settings: CameraSettings,
    
    // Export service for handling splat exports
    export_service: ExportService,
}

#[derive(Clone)]
struct CameraSettings {
    focal: f64,
    radius: f32,
    focus_distance: f32,
    speed_scale: f32,
}

impl AppContext {
    fn new(device: WgpuDevice, ctx: egui::Context, cam_settings: &CameraSettings) -> Self {
        let model_transform = Affine3A::IDENTITY;

        let controls = CameraController::new(
            cam_settings.radius,
            cam_settings.focus_distance,
            cam_settings.speed_scale,
        );

        let camera = Camera::new(
            model_transform.translation.into(),
            Quat::IDENTITY,
            cam_settings.focal,
            cam_settings.focal,
            glam::vec2(0.5, 0.5),
        );

        Self {
            dataset: Dataset::empty(),
            camera,
            view_aspect: None,
            controls,
            model_local_to_world: model_transform,
            device,
            loading: false,
            training: false,
            current_dataset_name: None,
            ctx,
            running_process: None,
            cam_settings: cam_settings.clone(),
            export_service: ExportService::new(None),
        }
    }

    fn match_controls_to(&mut self, cam: &Camera) {
        // We want model * controls.transform() == view_cam.transform() ->
        //  controls.transform = model.inverse() * view_cam.transform.
        let transform = self.model_local_to_world.inverse() * cam.local_to_world();
        self.controls.position = transform.translation.into();
        self.controls.rotation = Quat::from_mat3a(&transform.matrix3);
    }

    pub fn set_model_up(&mut self, up_axis: Vec3) {
        self.model_local_to_world = Affine3A::from_rotation_translation(
            Quat::from_rotation_arc(up_axis, Vec3::NEG_Y),
            Vec3::ZERO,
        );

        let cam = self.camera.clone();
        self.match_controls_to(&cam);
    }

    pub fn focus_view(&mut self, view: &SceneView) {
        self.camera = view.camera.clone();
        self.match_controls_to(&view.camera);
        self.controls.stop_movement();
        self.view_aspect = Some(view.image.width() as f32 / view.image.height() as f32);

        if let Some(extent) = self.dataset.train.estimate_extent() {
            self.controls.focus_distance = extent / 3.0;
        } else {
            self.controls.focus_distance = self.cam_settings.focus_distance;
        }
    }

    pub fn connect_to(&mut self, process: RunningProcess<TrainBack>) {
        // Save the current dataset name before resetting
        let current_dataset_name = self.current_dataset_name.clone();
        
        // reset context & view.
        *self = Self::new(self.device.clone(), self.ctx.clone(), &self.cam_settings);
        
        // Restore the current dataset name
        self.current_dataset_name = current_dataset_name;

        // Convert the receiver to a "reactive" receiver that wakes up the UI.
        self.running_process = Some(RunningProcess {
            messages: reactive_receiver(process.messages, self.ctx.clone()),
            ..process
        });
    }

    pub(crate) fn control_message(&self, msg: ControlMessage) {
        if let Some(process) = self.running_process.as_ref() {
            let _ = process.control.send(msg);
        }
    }

    pub fn training(&self) -> bool {
        self.training
    }

    pub fn loading(&self) -> bool {
        self.loading
    }

    /// Get the current splats for export
    pub fn get_current_splats(&self) -> Option<Splats<<TrainBack as AutodiffBackend>::InnerBackend>> {
        // First try to get splats from the Scene panel if it exists
        if let Some(scene_panel) = self.get_scene_panel() {
            if let Some(splats) = scene_panel.get_current_splats() {
                return Some(splats.clone());
            }
        }
        
        // If no Scene panel or no splats in Scene panel, try to get splats from the dataset
        // This allows export to work even without a Scene panel
        if let Some(splats) = self.dataset.get_current_splats() {
            return Some(splats);
        }
        
        // If we have a running process, try to get the latest splats from it
        if let Some(process) = &self.running_process {
            if let Some(splats) = process.get_latest_splats() {
                return Some(splats);
            }
        }
        
        None
    }

    /// Export the current splats to a PLY file using the Export Service
    pub fn export_splats_with_service(&mut self) {
        // Get the current splats using the new method
        let splats = match self.get_current_splats() {
            Some(splats) => splats,
            None => {
                log::error!("No splats available for export");
                return;
            }
        };
        
        // Generate a default filename with dataset name and timestamp
        let default_filename = self.generate_export_filename();
        
        // Clone necessary data for the async operation
        let ctx_clone = self.ctx.clone();
        let splats_clone = splats.clone();
        let export_service = self.export_service.clone();
        let default_filename_clone = default_filename.clone();
        
        // Use tokio to handle the file save dialog and export asynchronously
        #[cfg(not(target_family = "wasm"))]
        {
            tokio::spawn(async move {
                let file = rfd::AsyncFileDialog::new()
                    .add_filter("PLY", &["ply"])
                    .set_file_name(&default_filename_clone)
                    .save_file()
                    .await;
                    
                if let Some(file) = file {
                    // Set the export directory to the parent directory of the selected file
                    if let Some(parent) = file.path().parent() {
                        let mut export_service = export_service;
                        export_service.set_export_dir(parent.to_path_buf());
                        
                        // Get the filename
                        let filename = file.file_name();
                        
                        // Export the splats
                        match export_service.export_ply(&splats_clone, &filename) {
                            Ok(path) => {
                                log::info!("Successfully exported splats to {}", path.display());
                            }
                            Err(e) => {
                                log::error!("Failed to export splats: {}", e);
                            }
                        }
                    }
                } else {
                    log::error!("No file selected for export");
                }
                
                // Request a UI update
                ctx_clone.request_repaint();
            });
        }
        
        #[cfg(target_family = "wasm")]
        {
            tokio_wasm::task::spawn(async move {
                let file = rrfd::save_file(&default_filename_clone).await;
                
                if let Some(_) = file {
                    // Export the splats with a fixed filename for WASM
                    let filename = default_filename_clone;
                    
                    // Export the splats
                    match export_service.export_ply(&splats_clone, &filename) {
                        Ok(path) => {
                            log::info!("Successfully exported splats to {}", path.display());
                        }
                        Err(e) => {
                            log::error!("Failed to export splats: {}", e);
                        }
                    }
                } else {
                    log::error!("No file selected for export");
                }
                
                // Request a UI update
                ctx_clone.request_repaint();
            });
        }
    }
    
    /// Generate a filename for export based on dataset name and current timestamp
    fn generate_export_filename(&self) -> String {
        // Use the current dataset name if available
        let dataset_name = if let Some(name) = self.current_dataset_name() {
            name.clone()
        } else {
            // Fall back to extracting from path if no current dataset name is set
            if !self.dataset.train.views.is_empty() {
                // Extract the dataset name from the path of the first view
                let path_str = &self.dataset.train.views[0].path;
                
                // Convert to PathBuf to use path manipulation methods
                let path = std::path::Path::new(path_str);
                
                // Try to extract the dataset name by going up the directory tree
                // First, get the parent directory (which might be "images")
                if let Some(parent) = path.parent() {
                    // Then get the parent of that directory (which should be the dataset name)
                    if let Some(dataset_dir) = parent.parent() {
                        if let Some(name) = dataset_dir.file_name() {
                            if let Some(name_str) = name.to_str() {
                                name_str.to_string()
                            } else {
                                "dataset".to_string()
                            }
                        } else {
                            // If we can't get the dataset directory name, use the parent directory name
                            if let Some(name) = parent.file_name() {
                                if let Some(name_str) = name.to_str() {
                                    name_str.to_string()
                                } else {
                                    "dataset".to_string()
                                }
                            } else {
                                "dataset".to_string()
                            }
                        }
                    } else {
                        // If we can't get the dataset directory, use the parent directory name
                        if let Some(name) = parent.file_name() {
                            if let Some(name_str) = name.to_str() {
                                name_str.to_string()
                            } else {
                                "dataset".to_string()
                            }
                        } else {
                            "dataset".to_string()
                        }
                    }
                } else {
                    // If we can't get the parent, try to extract something meaningful from the path
                    if let Some(file_stem) = path.file_stem() {
                        if let Some(name_str) = file_stem.to_str() {
                            name_str.to_string()
                        } else {
                            "dataset".to_string()
                        }
                    } else {
                        "dataset".to_string()
                    }
                }
            } else {
                "dataset".to_string()
            }
        };
        
        // Get current timestamp
        let now = chrono::Local::now();
        let timestamp = now.format("%Y%m%d_%H%M%S");
        
        // Combine dataset name and timestamp
        format!("{}_{}.ply", dataset_name, timestamp)
    }
    
    /// Get a reference to the export service
    pub fn export_service(&self) -> &ExportService {
        &self.export_service
    }
    
    /// Get a mutable reference to the export service
    pub fn export_service_mut(&mut self) -> &mut ExportService {
        &mut self.export_service
    }
    
    /// Check for auto-save during training
    pub fn on_training_step(&mut self, step: u32) {
        // Get the current splats using the new method
        let splats = match self.get_current_splats() {
            Some(splats) => splats,
            None => return,
        };
        
        // Check for auto-save
        if let Some(result) = self.export_service.check_auto_save(&splats, step) {
            match result {
                Ok(path) => {
                    log::info!("Auto-saved splats to {}", path.display());
                }
                Err(e) => {
                    log::error!("Failed to auto-save splats: {}", e);
                }
            }
        }
    }
    
    /// Get the Scene panel
    fn get_scene_panel(&self) -> Option<&ScenePanel> {
        // We need to use a different approach to get the ScenePanel
        None
    }
    
    /// Get a mutable reference to the Scene panel
    fn get_scene_panel_mut(&mut self) -> Option<&mut ScenePanel> {
        // We need to use a different approach to get the ScenePanel
        None
    }

    /// Set the current dataset name
    pub fn set_current_dataset_name(&mut self, name: String) {
        self.current_dataset_name = Some(name);
    }

    /// Get the current dataset name
    pub fn current_dataset_name(&self) -> Option<&String> {
        self.current_dataset_name.as_ref()
    }
}

pub struct AppCreateCb {
    // TODO: Use parking lot non-poisonable locks.
    pub context: Arc<RwLock<AppContext>>,
}

impl App {
    pub fn new(
        cc: &eframe::CreationContext,
        create_callback: tokio::sync::oneshot::Sender<AppCreateCb>,
        _start_uri_override: Option<String>,
        reset_windows: bool,
    ) -> Self {
        // For now just assume we're running on the default
        let state = cc
            .wgpu_render_state
            .as_ref()
            .expect("No wgpu renderer enabled in egui");
        let device = brush_render::burn_init_device(
            state.adapter.clone(),
            state.device.clone(),
            state.queue.clone(),
        );

        // Parse URL parameters.
        let mut zen = false;
        let search_params: HashMap<String, String> = HashMap::new();

        #[cfg(target_family = "wasm")]
        {
            use web_sys::window;
            if let Some(window) = window() {
                if let Ok(Some(location)) = window.location().search().map(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        Some(s[1..].to_string())
                    }
                }) {
                    for pair in location.split('&') {
                        let mut parts = pair.split('=');
                        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                            search_params.insert(key.to_string(), value.to_string());
                        }
                    }
                }
            }
        }

        if let Some(z) = search_params.get("zen") {
            zen = z.parse::<bool>().unwrap_or(false);
        }

        let focal = search_params
            .get("focal")
            .and_then(|f| f.parse().ok())
            .unwrap_or(0.8);
        let radius = search_params
            .get("radius")
            .and_then(|f| f.parse().ok())
            .unwrap_or(4.0);
        let focus_distance = search_params
            .get("focus_distance")
            .and_then(|f| f.parse().ok())
            .unwrap_or(4.0);
        let speed_scale = search_params
            .get("speed_scale")
            .and_then(|f| f.parse().ok())
            .unwrap_or(1.0);

        let settings = CameraSettings {
            focal,
            radius,
            focus_distance,
            speed_scale,
        };
        let context = AppContext::new(device.clone(), cc.egui_ctx.clone(), &settings);

        let mut tiles: Tiles<PaneType> = Tiles::default();
        let scene_pane = ScenePanel::new(
            state.device.clone(),
            state.queue.clone(),
            state.renderer.clone(),
            zen,
        );

        let scene_pane_id = tiles.insert_pane(Box::new(scene_pane));

        let root_container = if !zen {
            #[cfg(feature = "tracing")]
            {
                // If tracing is enabled, add the tracing panel
                let tracing_pane = tiles.insert_pane(Box::new(TracingPanel::default()));
                
                let mut lin = egui_tiles::Linear::new(
                    egui_tiles::LinearDir::Horizontal,
                    vec![tracing_pane, scene_pane_id],
                );
                lin.shares.set_share(tracing_pane, 0.2);
                tiles.insert_container(lin)
            }
            
            #[cfg(not(feature = "tracing"))]
            {
                // Just use the scene panel
                scene_pane_id
            }
        } else {
            scene_pane_id
        };

        let tree = egui_tiles::Tree::new("brush_tree", root_container, tiles);

        let context = Arc::new(RwLock::new(context));
        let _ = create_callback.send(AppCreateCb {
            context: context.clone(),
        });

        let tree_ctx = AppTree { zen, context };

        let url = search_params.get("url");
        
        // Clone the device for the stats overlay before it's potentially moved
        let device_for_stats = device.clone();
        
        let running = if let Some(url) = url {
            Some(start_process(
                DataSource::Url(url.to_owned()),
                ProcessArgs::default(),
                device,
            ))
        } else {
            None
        };

        // Create the overlays
        let mut dataset_detail_overlay = DatasetDetailOverlay::new();
        let mut settings_detail_overlay = SettingsDetailOverlay::new();
        let mut stats_detail_overlay = StatsDetailOverlay::new(device_for_stats, state.adapter.get_info());
        let mut controls_detail_overlay = ControlsDetailOverlay::new();
        
        // Set all overlays to open by default with specific positions
        dataset_detail_overlay.set_open(true);
        dataset_detail_overlay.set_position(egui::pos2(120.0, 40.0)); // Upper left, moved further right to avoid nav panel
        
        settings_detail_overlay.set_open(true);
        settings_detail_overlay.set_position(egui::pos2(cc.egui_ctx.screen_rect().right() - 350.0, 40.0)); // Upper right, just below top
        
        stats_detail_overlay.set_open(true);
        stats_detail_overlay.set_position(egui::pos2(cc.egui_ctx.screen_rect().right() - 350.0, 
                                                    cc.egui_ctx.screen_rect().bottom() - 250.0)); // Bottom right, with space below
        
        controls_detail_overlay.set_open(true);
        // Set the Controls window position to where the user manually positioned it
        controls_detail_overlay.set_position(egui::pos2(614.0, 794.0));
        
        // If reset_windows flag is set, clear the window state from storage
        if reset_windows {
            cc.egui_ctx.memory_mut(|mem| {
                mem.data.clear();
            });
        }
        
        if let Some(running) = running {
            tree_ctx
                .context
                .write()
                .expect("Lock poisoned")
                .connect_to(running);
        }

        Self {
            tree,
            tree_ctx,
            datasets: None,
            dataset_detail_overlay,
            settings_detail_overlay,
            stats_detail_overlay,
            controls_detail_overlay,
            select_folder_requested: false,
            select_file_requested: false,
            select_dataset_folder_requested: false,
        }
    }
}

impl App {
    #[allow(clippy::significant_drop_tightening)]
    fn receive_messages(&mut self) {
        let mut context = self.tree_ctx.context.write().expect("Lock poisoned");

        let Some(process) = context.running_process.as_mut() else {
            return;
        };

        let mut messages = vec![];
        while let Ok(message) = process.messages.try_recv() {
            messages.push(message);
        }

        for message in messages {
            // Forward message to the Controls overlay
            self.controls_detail_overlay.on_message(&message);
            
            match &message {
                ProcessMessage::NewSource => {
                    // Reset the Controls overlay state when a new dataset is loaded
                    self.controls_detail_overlay.reset_state();
                    
                    // Forward the message to all panels
                    for (_, tile) in self.tree.tiles.iter_mut() {
                        if let Tile::Pane(pane) = tile {
                            pane.on_message(&message, &mut context);
                        }
                    }
                }
                ProcessMessage::Dataset { data: _ } => {
                    // Show the dataset panel if we've loaded one.
                    if self.datasets.is_none() {
                        let pane_id = self.tree.tiles.insert_pane(Box::new(DatasetPanel::new()));
                        self.datasets = Some(pane_id);
                        if let Some(Tile::Container(Container::Linear(lin))) = self
                            .tree
                            .tiles
                            .get_mut(self.tree.root().expect("UI must have a root"))
                        {
                            lin.add_child(pane_id);
                        }
                    }
                    
                    // Forward the message to all panels
                    for (_, tile) in self.tree.tiles.iter_mut() {
                        if let Tile::Pane(pane) = tile {
                            pane.on_message(&message, &mut context);
                        }
                    }
                }
                ProcessMessage::StartLoading { training } => {
                    context.training = *training;
                    context.loading = true;
                    
                    // Forward the message to all panels
                    for (_, tile) in self.tree.tiles.iter_mut() {
                        if let Tile::Pane(pane) = tile {
                            pane.on_message(&message, &mut context);
                        }
                    }
                }
                ProcessMessage::DoneLoading { training: _ } => {
                    context.loading = false;
                    
                    // Forward the message to all panels
                    for (_, tile) in self.tree.tiles.iter_mut() {
                        if let Tile::Pane(pane) = tile {
                            pane.on_message(&message, &mut context);
                        }
                    }
                }
                ProcessMessage::TrainStep { splats, stats: _, iter, timestamp: _ } => {
                    // Update the latest splats in the running process
                    if let Some(process) = &mut context.running_process {
                        process.update_latest_splats(*splats.clone());
                    }
                    
                    // Check for auto-save during training
                    context.on_training_step(*iter);
                    
                    // Forward the message to all panels
                    for (_, tile) in self.tree.tiles.iter_mut() {
                        if let Tile::Pane(pane) = tile {
                            pane.on_message(&message, &mut context);
                        }
                    }
                }
                ProcessMessage::ViewSplats { splats, up_axis: _, frame: _, total_frames: _ } => {
                    // Update the latest splats in the running process
                    if let Some(process) = &mut context.running_process {
                        process.update_latest_splats(*splats.clone());
                    }
                    
                    // Forward the message to all panels
                    for (_, tile) in self.tree.tiles.iter_mut() {
                        if let Tile::Pane(pane) = tile {
                            pane.on_message(&message, &mut context);
                        }
                    }
                }
                _ => {
                    // Forward the message to all panels
                    for (_, tile) in self.tree.tiles.iter_mut() {
                        if let Tile::Pane(pane) = tile {
                            pane.on_message(&message, &mut context);
                        }
                    }
                },
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.receive_messages();

        // Left sidebar with icons
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .default_width(40.0)
            .width_range(40.0..=40.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    
                    // Datasets icon
                    let datasets_icon = "📁";
                    let datasets_button = ui.add(
                        egui::Button::new(egui::RichText::new(datasets_icon).size(20.0))
                            .min_size(egui::vec2(30.0, 30.0))
                            .corner_radius(5.0)
                            .fill(if self.dataset_detail_overlay.is_open() {
                                ui.visuals().selection.bg_fill
                            } else {
                                ui.visuals().widgets.inactive.bg_fill
                            })
                    );
                    
                    if datasets_button.clicked() {
                        let is_open = self.dataset_detail_overlay.is_open();
                        self.dataset_detail_overlay.set_open(!is_open);
                    }
                    
                    // Tooltip for the datasets button
                    if datasets_button.hovered() {
                        datasets_button.on_hover_text("Browse Datasets");
                    }
                    
                    ui.add_space(5.0);
                    
                    // Settings icon
                    let settings_icon = "⚙";
                    let settings_button = ui.add(
                        egui::Button::new(egui::RichText::new(settings_icon).size(20.0))
                            .min_size(egui::vec2(30.0, 30.0))
                            .corner_radius(5.0)
                            .fill(if self.settings_detail_overlay.is_open() {
                                ui.visuals().selection.bg_fill
                            } else {
                                ui.visuals().widgets.inactive.bg_fill
                            })
                    );
                    
                    if settings_button.clicked() {
                        let is_open = self.settings_detail_overlay.is_open();
                        self.settings_detail_overlay.set_open(!is_open);
                    }
                    
                    // Tooltip for the settings button
                    if settings_button.hovered() {
                        settings_button.on_hover_text("Settings");
                    }
                    
                    ui.add_space(5.0);
                    
                    // Stats icon
                    let stats_icon = "📊";
                    let stats_button = ui.add(
                        egui::Button::new(egui::RichText::new(stats_icon).size(20.0))
                            .min_size(egui::vec2(30.0, 30.0))
                            .corner_radius(5.0)
                            .fill(if self.stats_detail_overlay.is_open() {
                                ui.visuals().selection.bg_fill
                            } else {
                                ui.visuals().widgets.inactive.bg_fill
                            })
                    );
                    
                    if stats_button.clicked() {
                        let is_open = self.stats_detail_overlay.is_open();
                        self.stats_detail_overlay.set_open(!is_open);
                    }
                    
                    // Tooltip for the stats button
                    if stats_button.hovered() {
                        stats_button.on_hover_text("Statistics");
                    }
                    
                    ui.add_space(5.0);
                    
                    // Controls icon
                    let controls_icon = "🎮";
                    let controls_button = ui.add(
                        egui::Button::new(egui::RichText::new(controls_icon).size(20.0))
                            .min_size(egui::vec2(30.0, 30.0))
                            .corner_radius(5.0)
                            .fill(if self.controls_detail_overlay.is_open() {
                                ui.visuals().selection.bg_fill
                            } else {
                                ui.visuals().widgets.inactive.bg_fill
                            })
                    );
                    
                    if controls_button.clicked() {
                        let is_open = self.controls_detail_overlay.is_open();
                        self.controls_detail_overlay.set_open(!is_open);
                    }
                    
                    // Tooltip for the controls button
                    if controls_button.hovered() {
                        controls_button.on_hover_text("Training Controls");
                    }
                });
            });

        let main_panel_frame = egui::Frame::central_panel(ctx.style().as_ref()).inner_margin(0.0);

        // Main content area
        egui::CentralPanel::default()
            .frame(main_panel_frame)
            .show(ctx, |ui| {
            self.tree.ui(&mut self.tree_ctx, ui);
            
            // Add a resize indicator in the bottom-right corner of the main window
            // Only show on desktop platforms, not on mobile or web
            #[cfg(not(any(target_os = "android", target_os = "ios", target_family = "wasm")))]
            {
                let resize_rect = egui::Rect::from_min_size(
                    ui.max_rect().right_bottom() - egui::vec2(16.0, 16.0),
                    egui::vec2(16.0, 16.0)
                );
                if ui.rect_contains_pointer(resize_rect) {
                    ui.painter().text(
                        resize_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "↘",
                        egui::FontId::proportional(14.0),
                        ui.visuals().weak_text_color()
                    );
                }
            }
        });
        
        // Handle folder selection
        if self.dataset_detail_overlay.wants_to_select_folder() {
            self.dataset_detail_overlay.folder_selection_started();
            self.select_folder_requested = true;
        }
        
        if self.select_folder_requested {
            self.select_folder_requested = false;
            
            // Use native dialog
            #[cfg(not(target_family = "wasm"))]
            {
                // For native, use rfd directly (synchronous version)
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.dataset_detail_overlay.set_selected_folder(path);
                    self.dataset_detail_overlay.refresh_datasets();
                } else {
                    self.dataset_detail_overlay.cancel_folder_selection();
                }
            }
            
            #[cfg(target_family = "wasm")]
            {
                // For WASM, we would need a different approach
                // This is just a placeholder
                self.dataset_detail_overlay.cancel_folder_selection();
            }
        }
        
        // Handle file selection for adding datasets
        if self.dataset_detail_overlay.wants_to_select_file() {
            self.dataset_detail_overlay.file_selection_started();
            self.select_file_requested = true;
        }
        
        if self.select_file_requested {
            self.select_file_requested = false;
            
            // Use native dialog
            #[cfg(not(target_family = "wasm"))]
            {
                // For native, use rfd directly (synchronous version)
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Dataset", &["zip"])
                    .pick_file() {
                    self.dataset_detail_overlay.set_selected_file(path);
                } else {
                    self.dataset_detail_overlay.cancel_file_selection();
                }
            }
            
            #[cfg(target_family = "wasm")]
            {
                // For WASM, we would need a different approach
                // This is just a placeholder
                self.dataset_detail_overlay.cancel_file_selection();
            }
        }
        
        // Handle dataset folder selection
        if self.dataset_detail_overlay.wants_to_select_dataset_folder() {
            self.dataset_detail_overlay.dataset_folder_selection_started();
            self.select_dataset_folder_requested = true;
        }
        
        if self.select_dataset_folder_requested {
            self.select_dataset_folder_requested = false;
            
            // Use native dialog
            #[cfg(not(target_family = "wasm"))]
            {
                // For native, use rfd directly (synchronous version)
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.dataset_detail_overlay.set_selected_dataset_folder(path);
                } else {
                    self.dataset_detail_overlay.cancel_dataset_folder_selection();
                }
            }
            
            #[cfg(target_family = "wasm")]
            {
                // For WASM, we would need a different approach
                // This is just a placeholder
                self.dataset_detail_overlay.cancel_dataset_folder_selection();
            }
        }
        
        // Show the overlays
        let mut context = self.tree_ctx.context.write().expect("Lock poisoned");
        
        // Show the dataset detail overlay if it's open
        self.dataset_detail_overlay.show(ctx, &mut context);
        
        // Show the settings detail overlay if it's open
        self.settings_detail_overlay.show(ctx, &mut context);
        
        // Show the stats detail overlay if it's open
        self.stats_detail_overlay.show(ctx, &mut context);
        
        // Show the controls detail overlay if it's open
        self.controls_detail_overlay.show(ctx, &mut context);
        
        // Forward messages to the stats overlay
        if let Some(process) = context.running_process.as_mut() {
            // Use a loop with a counter to avoid infinite loops
            let mut count = 0;
            let max_messages = 100; // Limit to avoid infinite loops
            loop {
                match process.messages.try_recv() {
                    Ok(message) => {
                        // Forward message to both Stats and Controls overlays
                        self.stats_detail_overlay.on_message(&message);
                        self.controls_detail_overlay.on_message(&message);
                        count += 1;
                        if count >= max_messages {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    }
}
