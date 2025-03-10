use crate::app::AppContext;
use egui::{Context, Pos2, Vec2, pos2};
use brush_process::process_loop::ProcessMessage;
use burn_cubecl::cubecl::Runtime;
use burn_wgpu::{WgpuDevice, WgpuRuntime};
use std::time::Duration;
use web_time::Instant;
use wgpu::AdapterInfo;

pub(crate) struct StatsDetailOverlay {
    // Stats fields
    device: WgpuDevice,
    last_train_step: (Instant, u32),
    train_iter_per_s: f32,
    last_eval: Option<String>,
    cur_sh_degree: u32,
    training_started: bool,
    num_splats: u32,
    frames: u32,
    start_load_time: Instant,
    adapter_info: AdapterInfo,
    
    // UI state
    open: bool,
    position: Pos2,
    size: Vec2,
}

impl StatsDetailOverlay {
    pub(crate) fn new(device: WgpuDevice, adapter_info: AdapterInfo) -> Self {
        Self {
            // Stats fields
            device,
            last_train_step: (Instant::now(), 0),
            train_iter_per_s: 0.0,
            last_eval: None,
            training_started: false,
            num_splats: 0,
            frames: 0,
            cur_sh_degree: 0,
            start_load_time: Instant::now(),
            adapter_info,
            
            // UI state
            open: false, // Start with window closed
            position: pos2(200.0, 200.0), // Offset position to avoid overlap
            size: Vec2::new(500.0, 700.0), // Increased default size
        }
    }
    
    pub(crate) fn is_open(&self) -> bool {
        self.open
    }
    
    pub(crate) fn set_open(&mut self, open: bool) {
        self.open = open;
    }
    
    pub(crate) fn on_message(&mut self, message: &ProcessMessage) {
        match message {
            ProcessMessage::NewSource => {
                *self = Self::new(self.device.clone(), self.adapter_info.clone());
            }
            ProcessMessage::StartLoading { training } => {
                self.start_load_time = Instant::now();
                self.last_train_step = (Instant::now(), 0);
                self.train_iter_per_s = 0.0;
                self.num_splats = 0;
                self.cur_sh_degree = 0;
                self.last_eval = None;
                self.training_started = *training;
            }
            ProcessMessage::ViewSplats {
                up_axis: _,
                splats,
                frame,
                total_frames: _,
            } => {
                self.num_splats = splats.num_splats();
                self.frames = *frame;
                self.cur_sh_degree = splats.sh_degree();
            }
            ProcessMessage::TrainStep {
                splats,
                stats: _,
                iter,
                timestamp,
            } => {
                self.cur_sh_degree = splats.sh_degree();
                self.num_splats = splats.num_splats();
                let current_iter_per_s = (iter - self.last_train_step.1) as f32
                    / (*timestamp - self.last_train_step.0).as_secs_f32();
                self.train_iter_per_s = 0.95 * self.train_iter_per_s + 0.05 * current_iter_per_s;
                self.last_train_step = (*timestamp, *iter);
            }
            ProcessMessage::EvalResult {
                iter: _,
                avg_psnr,
                avg_ssim,
            } => {
                self.last_eval = Some(format!("{avg_psnr:.2} PSNR, {avg_ssim:.3} SSIM"));
            }
            _ => {}
        }
    }
    
    fn bytes_format(bytes: u64) -> String {
        let unit = 1000;

        if bytes < unit {
            format!("{bytes} B")
        } else {
            let size = bytes as f64;
            let exp = match size.log(1000.0).floor() as usize {
                0 => 1,
                e => e,
            };
            let unit_prefix = b"KMGTPEZY";
            format!(
                "{:.2} {}B",
                (size / unit.pow(exp as u32) as f64),
                unit_prefix[exp - 1] as char,
            )
        }
    }
    
    pub(crate) fn show(&mut self, ctx: &Context, _context: &mut AppContext) {
        if !self.open {
            return;
        }
        
        // Create a unique window ID - make it static to maintain window state
        let window_id = egui::Id::new("stats_detail_window");
        
        // Track open state locally to avoid borrow issues
        let mut window_open = self.open;
        
        // Create the window with settings to ensure proper resizability
        let window = egui::Window::new("Stats")
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
            
            egui::Grid::new("stats_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Splats");
                    ui.label(format!("{}", self.num_splats));
                    ui.end_row();

                    ui.label("SH Degree");
                    ui.label(format!("{}", self.cur_sh_degree));
                    ui.end_row();

                    if self.frames > 0 {
                        ui.label("Frames");
                        ui.label(format!("{}", self.frames));
                        ui.end_row();
                    }

                    if self.training_started {
                        ui.label("Train step");
                        ui.label(format!("{}", self.last_train_step.1));
                        ui.end_row();

                        ui.label("Steps/s");
                        ui.label(format!("{:.1}", self.train_iter_per_s));
                        ui.end_row();

                        ui.label("Last eval:");
                        ui.label(if let Some(eval) = self.last_eval.as_ref() {
                            eval
                        } else {
                            "--"
                        });
                        ui.end_row();

                        ui.label("Training time");
                        // Round duration to seconds.
                        let elapsed = Duration::from_secs(self.start_load_time.elapsed().as_secs());
                        ui.label(format!("{}", humantime::Duration::from(elapsed)));
                        ui.end_row();
                    }

                    let client = WgpuRuntime::client(&self.device);
                    let memory = client.memory_usage();

                    ui.label("GPU memory");
                    ui.end_row();

                    ui.label("Bytes in use");
                    ui.label(Self::bytes_format(memory.bytes_in_use));
                    ui.end_row();

                    ui.label("Bytes reserved");
                    ui.label(Self::bytes_format(memory.bytes_reserved));
                    ui.end_row();

                    ui.label("Active allocations");
                    ui.label(format!("{}", memory.number_allocs));
                    ui.end_row();
                });

            // On WASM, adapter info is mostly private, not worth showing.
            if !cfg!(target_family = "wasm") {
                egui::Grid::new("gpu_grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("GPU");
                        ui.end_row();

                        ui.label("Name");
                        ui.label(&self.adapter_info.name);
                        ui.end_row();

                        ui.label("Type");
                        ui.label(format!("{:?}", self.adapter_info.device_type));
                        ui.end_row();

                        ui.label("Driver");
                        ui.label(format!(
                            "{}, {}",
                            self.adapter_info.driver, self.adapter_info.driver_info
                        ));
                        ui.end_row();
                    });
            }
        });
        
        // Update self.open based on window_open
        if self.open != window_open {
            self.open = window_open;
        }
        
        // Store window size for next frame if available
        if let Some(response) = response {
            self.size = response.response.rect.size();
        }
    }
} 