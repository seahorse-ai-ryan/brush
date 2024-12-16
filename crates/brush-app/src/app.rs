use core::f32;
use std::ops::Range;

use brush_dataset::{self, Dataset, LoadDatasetArgs, LoadInitArgs};
use brush_render::camera::Camera;
use brush_train::train::TrainConfig;
use brush_ui::channel::{reactive_receiver, reactive_receiver_unbounded};
use burn_wgpu::WgpuDevice;
use eframe::egui;
use egui_tiles::{Container, Tile, TileId, Tiles};
use glam::{Affine3A, Quat, Vec3, Vec3A};

use ::tokio::sync::mpsc::channel;
use ::tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use ::tokio::sync::mpsc::{Receiver, UnboundedReceiver};
use std::collections::HashMap;
use tokio::task;
use tokio_with_wasm::alias as tokio;
use wgpu::Features;

use crate::data_source::DataSource;
use crate::process_loop::{self, ControlMessage, ProcessMessage};
use crate::{
    orbit_controls::OrbitControls,
    panels::{DatasetPanel, LoadDataPanel, PresetsPanel, ScenePanel, StatsPanel, TracingPanel},
};

use egui_tiles::SimplificationOptions;

pub(crate) trait AppPanel {
    fn title(&self) -> String;
    fn ui(&mut self, ui: &mut egui::Ui, controls: &mut AppContext);
    fn on_message(&mut self, message: &ProcessMessage, context: &mut AppContext) {
        let _ = message;
        let _ = context;
    }
}

struct AppTree {
    zen: bool,
    context: AppContext,
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
        pane.ui(ui, &mut self.context);
        egui_tiles::UiResponse::None
    }

    /// What are the rules for simplifying the tree?
    fn simplification_options(&self) -> SimplificationOptions {
        SimplificationOptions {
            all_panes_must_have_tabs: !self.zen,
            ..Default::default()
        }
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

pub enum UiControlMessage {
    LoadData(String),
}

pub struct App {
    tree: egui_tiles::Tree<PaneType>,
    datasets: Option<TileId>,
    tree_ctx: AppTree,
}

// TODO: Bit too much random shared state here.
pub(crate) struct AppContext {
    pub dataset: Dataset,
    pub camera: Camera,
    pub controls: OrbitControls,

    pub model_transform: Affine3A,

    device: WgpuDevice,
    ctx: egui::Context,

    rec_ui_control_msg: UnboundedReceiver<UiControlMessage>,
    send_train_msg: Option<UnboundedSender<ControlMessage>>,

    rec_process_msg: Option<Receiver<ProcessMessage>>,
}

struct CameraSettings {
    focal: f64,
    radius: f32,

    yaw_range: Range<f32>,
    pitch_range: Range<f32>,
    radius_range: Range<f32>,
}

impl AppContext {
    fn new(
        device: WgpuDevice,
        ctx: egui::Context,
        cam_settings: CameraSettings,
        ui_receiver: UnboundedReceiver<UiControlMessage>,
    ) -> Self {
        let model_transform = Affine3A::IDENTITY;

        let controls = OrbitControls::new(
            cam_settings.radius,
            cam_settings.radius_range,
            cam_settings.yaw_range,
            cam_settings.pitch_range,
        );

        // Camera position will be controller by controls.
        let camera = Camera::new(
            Vec3::ZERO,
            Quat::IDENTITY,
            cam_settings.focal,
            cam_settings.focal,
            glam::vec2(0.5, 0.5),
        );

        let rec_ui_control_msg = reactive_receiver_unbounded(ui_receiver, ctx.clone());

        Self {
            camera,
            controls,
            model_transform,
            device,
            ctx,
            dataset: Dataset::empty(),
            rec_process_msg: None,
            send_train_msg: None,
            rec_ui_control_msg,
        }
    }

    pub fn set_up_axis(&mut self, up_axis: Vec3) {
        let rotation = Quat::from_rotation_arc(Vec3::Y, up_axis);
        let model_transform = Affine3A::from_rotation_translation(rotation, Vec3::ZERO).inverse();
        self.model_transform = model_transform;
    }

    pub fn focus_view(&mut self, cam: &Camera) {
        // set the controls transform.
        let cam_transform = Affine3A::from_rotation_translation(cam.rotation, cam.position);
        let transform = self.model_transform.inverse() * cam_transform;
        let (_, rotation, position) = transform.to_scale_rotation_translation();
        self.controls.position = position.into();
        self.controls.rotation = rotation;

        // Copy the camera, mostly to copy over the intrinsics and such.
        self.controls.focus = transform.translation
            + transform.matrix3 * Vec3A::Z * self.dataset.train.bounds().extent.length() * 0.5;
        self.controls.dirty = true;
        self.camera = cam.clone();
    }

    pub(crate) fn start_data_load(
        &mut self,
        source: DataSource,
        load_data_args: LoadDatasetArgs,
        load_init_args: LoadInitArgs,
        train_config: TrainConfig,
    ) {
        let device = self.device.clone();
        log::info!("Start data load {source:?}");

        // Create a small channel. We don't want 10 updated splats to be stuck in the queue eating up memory!
        // Bigger channels could mean the train loop spends less time waiting for the UI though.
        // create a channel for the train loop.
        let (train_sender, train_receiver) = unbounded_channel();
        let (sender, receiver) = channel(1);
        let receiver = reactive_receiver(receiver, self.ctx.clone());

        self.rec_process_msg = Some(receiver);
        self.send_train_msg = Some(train_sender);

        self.dataset = Dataset::empty();

        task::spawn(async move {
            process_loop::process_loop(
                sender,
                source,
                device,
                train_receiver,
                load_data_args,
                load_init_args,
                train_config,
            )
            .await;
        });
    }

    pub fn control_message(&self, message: ControlMessage) {
        self.send_train_msg.as_ref().inspect(|send| {
            let _ = send.send(message);
        });
    }
}

impl App {
    pub fn new(
        cc: &eframe::CreationContext,
        start_uri: Option<String>,
        controller: UnboundedReceiver<UiControlMessage>,
    ) -> Self {
        // For now just assume we're running on the default
        let state = cc
            .wgpu_render_state
            .as_ref()
            .expect("No wgpu renderer enabled in egui");
        let device = brush_ui::create_wgpu_device(
            state.adapter.clone(),
            state.device.clone(),
            state.queue.clone(),
        );

        brush_render::render::set_hard_floats_available(
            state
                .adapter
                .features()
                .contains(Features::SHADER_FLT32_ATOMIC),
        );

        if cfg!(feature = "tracing") {
            // TODO: In debug only?
            #[cfg(target_family = "wasm")]
            {
                use tracing_subscriber::layer::SubscriberExt;

                tracing::subscriber::set_global_default(
                    tracing_subscriber::registry()
                        .with(tracing_wasm::WASMLayer::new(Default::default())),
                )
                .expect("Failed to set tracing subscriber");
            }

            #[cfg(all(feature = "tracy", not(target_family = "wasm")))]
            {
                use tracing_subscriber::layer::SubscriberExt;

                tracing::subscriber::set_global_default(
                    tracing_subscriber::registry()
                        .with(tracing_tracy::TracyLayer::default())
                        .with(sync_span::SyncLayer::<
                            burn_jit::JitBackend<burn_wgpu::WgpuRuntime, f32, i32, u32>,
                        >::new(device.clone())),
                )
                .expect("Failed to set tracing subscriber");
            }
        }

        #[cfg(target_family = "wasm")]
        let start_uri = start_uri.or(web_sys::window().and_then(|w| w.location().search().ok()));

        let search_params = parse_search(&start_uri.unwrap_or_default());

        let mut zen = false;
        if let Some(z) = search_params.get("zen") {
            zen = z.parse::<bool>().unwrap_or(false);
        }

        let focal = search_params
            .get("focal")
            .and_then(|f| f.parse().ok())
            .unwrap_or(0.5);
        let radius = search_params
            .get("radius")
            .and_then(|f| f.parse().ok())
            .unwrap_or(4.0);

        let min_radius = search_params
            .get("min_radius")
            .and_then(|f| f.parse().ok())
            .unwrap_or(1.0);
        let max_radius = search_params
            .get("max_radius")
            .and_then(|f| f.parse().ok())
            .unwrap_or(10.0);

        let min_yaw = search_params
            .get("min_yaw")
            .and_then(|f| f.parse::<f32>().ok())
            .map_or(f32::MIN, |d| d.to_radians());
        let max_yaw = search_params
            .get("max_yaw")
            .and_then(|f| f.parse::<f32>().ok())
            .map_or(f32::MAX, |d| d.to_radians());

        let min_pitch = search_params
            .get("min_pitch")
            .and_then(|f| f.parse::<f32>().ok())
            .map_or(f32::MIN, |d| d.to_radians());
        let max_pitch = search_params
            .get("max_pitch")
            .and_then(|f| f.parse::<f32>().ok())
            .map_or(f32::MAX, |d| d.to_radians());

        let settings = CameraSettings {
            focal,
            radius,
            radius_range: min_radius..max_radius,
            yaw_range: min_yaw..max_yaw,
            pitch_range: min_pitch..max_pitch,
        };

        let context = AppContext::new(device.clone(), cc.egui_ctx.clone(), settings, controller);

        let mut tiles: Tiles<PaneType> = Tiles::default();
        let scene_pane = ScenePanel::new(
            state.queue.clone(),
            state.device.clone(),
            state.renderer.clone(),
            zen,
        );

        let scene_pane_id = tiles.insert_pane(Box::new(scene_pane));

        let root_container = if !zen {
            let loading_subs = vec![
                tiles.insert_pane(Box::new(LoadDataPanel::new())),
                tiles.insert_pane(Box::new(PresetsPanel::new())),
            ];
            let loading_pane = tiles.insert_tab_tile(loading_subs);

            #[allow(unused_mut)]
            let mut sides = vec![
                loading_pane,
                tiles.insert_pane(Box::new(StatsPanel::new(device, &state.adapter))),
            ];

            #[cfg(all(not(target_family = "wasm"), not(target_os = "android")))]
            {
                sides.push(tiles.insert_pane(Box::new(crate::panels::RerunPanel::new())));
            }

            if cfg!(feature = "tracing") {
                sides.push(tiles.insert_pane(Box::new(TracingPanel::default())));
            }

            let side_panel = tiles.insert_vertical_tile(sides);

            let mut lin = egui_tiles::Linear::new(
                egui_tiles::LinearDir::Horizontal,
                vec![side_panel, scene_pane_id],
            );
            lin.shares.set_share(side_panel, 0.4);
            tiles.insert_container(lin)
        } else {
            scene_pane_id
        };

        let tree = egui_tiles::Tree::new("brush_tree", root_container, tiles);

        let mut tree_ctx = AppTree { zen, context };

        let url = search_params.get("url");
        if let Some(url) = url {
            tree_ctx.context.start_data_load(
                DataSource::Url(url.to_owned()),
                LoadDatasetArgs::default(),
                LoadInitArgs::default(),
                TrainConfig::default(),
            );
        }

        Self {
            tree,
            tree_ctx,
            datasets: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        while let Ok(m) = self.tree_ctx.context.rec_ui_control_msg.try_recv() {
            match m {
                UiControlMessage::LoadData(url) => {
                    self.tree_ctx.context.start_data_load(
                        DataSource::Url(url.clone()),
                        LoadDatasetArgs::default(),
                        LoadInitArgs::default(),
                        TrainConfig::default(),
                    );
                }
            }
        }

        if let Some(rec) = self.tree_ctx.context.rec_process_msg.as_mut() {
            let mut messages = vec![];

            while let Ok(message) = rec.try_recv() {
                messages.push(message);
            }

            for message in messages {
                if let ProcessMessage::Dataset { data: _ } = message {
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

                for (_, pane) in self.tree.tiles.iter_mut() {
                    match pane {
                        Tile::Pane(pane) => {
                            pane.on_message(&message, &mut self.tree_ctx.context);
                        }
                        Tile::Container(_) => {}
                    }
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Close when pressing escape (in a native viewer anyway).
            if ui.input(|r| r.key_pressed(egui::Key::Escape)) {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            self.tree.ui(&mut self.tree_ctx, ui);
        });
    }
}
