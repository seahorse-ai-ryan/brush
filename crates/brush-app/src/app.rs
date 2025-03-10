use std::sync::{Arc, RwLock};

use crate::channel::reactive_receiver;
use crate::orbit_controls::CameraController;
use crate::overlays::{DatasetDetailOverlay, SettingsDetailOverlay, StatsDetailOverlay};
use crate::panels::{DatasetPanel, ScenePanel};
#[cfg(feature = "tracing")]
use crate::panels::TracingPanel;
use brush_dataset::Dataset;
use brush_process::data_source::DataSource;
use brush_process::process_loop::{
    ControlMessage, ProcessArgs, ProcessMessage, RunningProcess, start_process,
};
use brush_render::camera::Camera;
use brush_train::scene::SceneView;
use burn_wgpu::WgpuDevice;
use eframe::egui;
use egui_tiles::SimplificationOptions;
use egui_tiles::{Container, Tile, TileId, Tiles};
use glam::{Affine3A, Quat, Vec3};
use std::collections::HashMap;

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
            all_panes_must_have_tabs: !self.zen,
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
    select_folder_requested: bool,
    select_file_requested: bool,
    select_dataset_folder_requested: bool,
}

// TODO: Bit too much random shared state here.
pub struct AppContext {
    pub dataset: Dataset,
    pub camera: Camera,
    pub view_aspect: Option<f32>,
    pub controls: CameraController,
    pub model_local_to_world: Affine3A,
    pub device: WgpuDevice,

    loading: bool,
    training: bool,

    ctx: egui::Context,
    running_process: Option<RunningProcess>,
    cam_settings: CameraSettings,
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

        // Camera position will be controlled by the orbit controls.
        let camera = Camera::new(
            Vec3::ZERO,
            Quat::IDENTITY,
            cam_settings.focal,
            cam_settings.focal,
            glam::vec2(0.5, 0.5),
        );

        Self {
            camera,
            controls,
            model_local_to_world: model_transform,
            device,
            ctx,
            view_aspect: None,
            loading: false,
            training: false,
            dataset: Dataset::empty(),
            running_process: None,
            cam_settings: cam_settings.clone(),
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

    pub fn connect_to(&mut self, process: RunningProcess) {
        // reset context & view.
        *self = Self::new(self.device.clone(), self.ctx.clone(), &self.cam_settings);

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
        let dataset_detail_overlay = DatasetDetailOverlay::new();
        let settings_detail_overlay = SettingsDetailOverlay::new();
        let stats_detail_overlay = StatsDetailOverlay::new(device_for_stats, state.adapter.get_info());
        
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
            match message {
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
                }
                ProcessMessage::StartLoading { training } => {
                    context.training = training;
                    context.loading = true;
                }
                ProcessMessage::DoneLoading { training: _ } => {
                    context.loading = false;
                }
                _ => (),
            }

            for (_, pane) in self.tree.tiles.iter_mut() {
                match pane {
                    Tile::Pane(pane) => {
                        pane.on_message(&message, &mut context);
                    }
                    Tile::Container(_) => {}
                }
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.receive_messages();

        // Top bar menu
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                
                ui.menu_button("View", |ui| {
                    let mut detail_open = self.dataset_detail_overlay.is_open();
                    if ui.checkbox(&mut detail_open, "Datasets").clicked() {
                        self.dataset_detail_overlay.set_open(detail_open);
                    }
                    
                    let mut settings_open = self.settings_detail_overlay.is_open();
                    if ui.checkbox(&mut settings_open, "Settings").clicked() {
                        self.settings_detail_overlay.set_open(settings_open);
                    }
                    
                    let mut stats_open = self.stats_detail_overlay.is_open();
                    if ui.checkbox(&mut stats_open, "Stats").clicked() {
                        self.stats_detail_overlay.set_open(stats_open);
                    }
                });
                
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        // Show about dialog
                    }
                });
            });
        });

        // Left sidebar with icons
        egui::SidePanel::left("left_icon_bar")
            .resizable(false)
            .default_width(40.0)
            .width_range(40.0..=40.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    
                    // Datasets icon
                    let datasets_icon = "📁";
                    let datasets_button = ui.add(
                        egui::Button::new(datasets_icon)
                            .min_size(egui::vec2(30.0, 30.0))
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
                    let settings_icon = "⚙️";
                    let settings_button = ui.add(
                        egui::Button::new(settings_icon)
                            .min_size(egui::vec2(30.0, 30.0))
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
                        egui::Button::new(stats_icon)
                            .min_size(egui::vec2(30.0, 30.0))
                    );
                    
                    if stats_button.clicked() {
                        let is_open = self.stats_detail_overlay.is_open();
                        self.stats_detail_overlay.set_open(!is_open);
                    }
                    
                    // Tooltip for the stats button
                    if stats_button.hovered() {
                        stats_button.on_hover_text("Stats");
                    }
                });
            });

        let main_panel_frame = egui::Frame::central_panel(ctx.style().as_ref()).inner_margin(0.0);

        egui::CentralPanel::default()
            .frame(main_panel_frame)
            .show(ctx, |ui| {
                self.tree.ui(&mut self.tree_ctx, ui);
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
        
        // Show the dataset detail overlay if it's open
        let mut context = self.tree_ctx.context.write().expect("Lock poisoned");
        self.dataset_detail_overlay.show(ctx, &mut context);
        
        // Show the settings detail overlay if it's open
        self.settings_detail_overlay.show(ctx, &mut context);
        
        // Show the stats detail overlay if it's open
        self.stats_detail_overlay.show(ctx, &mut context);
        
        // Forward messages to the stats overlay
        if let Some(process) = context.running_process.as_mut() {
            // Use a loop with a counter to avoid infinite loops
            let mut count = 0;
            let max_messages = 100; // Limit to avoid infinite loops
            loop {
                match process.messages.try_recv() {
                    Ok(message) => {
                        self.stats_detail_overlay.on_message(&message);
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
