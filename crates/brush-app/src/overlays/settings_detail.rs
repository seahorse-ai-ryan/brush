use crate::app::AppContext;
use egui::{Context, Pos2, Vec2, pos2, Align2};
use brush_dataset::{LoadDataseConfig, ModelConfig};
use brush_process::{
    process_loop::{ProcessArgs, ProcessConfig, RerunConfig},
};
use brush_train::train::TrainConfig;
use egui::Slider;

pub(crate) struct SettingsDetailOverlay {
    // Settings fields
    args: ProcessArgs,
    url: String,
    
    // UI state
    open: bool,
    position: Pos2,
    size: Vec2,
}

impl SettingsDetailOverlay {
    pub(crate) fn new() -> Self {
        Self {
            // Nb: Important to just start with the default values here, so CLI and UI match defaults.
            args: ProcessArgs::new(
                TrainConfig::new(),
                ModelConfig::new(),
                LoadDataseConfig::new(),
                ProcessConfig::new(),
                RerunConfig::new(),
            ),
            url: "splat.com/example.ply".to_owned(),
            
            // UI state
            open: false, // Start with window closed
            position: pos2(200.0, 200.0),
            size: Vec2::new(320.0, 420.0), // Adjusted size to better fit content
        }
    }
    
    pub(crate) fn is_open(&self) -> bool {
        self.open
    }
    
    pub(crate) fn set_open(&mut self, open: bool) {
        self.open = open;
    }
    
    pub(crate) fn show(&mut self, ctx: &Context, _context: &mut AppContext) {
        if !self.open {
            return;
        }
        
        // Create a unique window ID - make it static to maintain window state
        let window_id = egui::Id::new("settings_detail_window");
        
        // Track open state locally to avoid borrow issues
        let mut window_open = self.open;
        
        // Create the window with settings to ensure proper resizability
        let window = egui::Window::new("⚙ Settings")
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
            // Use a ScrollArea that fills the available space
            ui.set_width(ui.available_width());
            ui.set_height(ui.available_height());
            
            // Add a subtle resize indicator in the bottom-right corner
            let resize_rect = egui::Rect::from_min_size(
                ui.max_rect().right_bottom() - egui::vec2(16.0, 16.0),
                egui::vec2(16.0, 16.0)
            );
            if ui.rect_contains_pointer(resize_rect) {
                ui.painter().text(
                    resize_rect.center(),
                    Align2::CENTER_CENTER,
                    "↘",
                    egui::FontId::proportional(14.0),
                    ui.visuals().weak_text_color()
                );
            }
            
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.heading("Application Settings");
                    ui.add_space(10.0);
                    
                    ui.heading("Model Settings");
                    ui.label("Spherical Harmonics Degree:");
                    ui.add(Slider::new(&mut self.args.model_config.sh_degree, 0..=4));

                    ui.label("Max image resolution");
                    ui.add(
                        Slider::new(&mut self.args.load_config.max_resolution, 32..=2048)
                            .clamping(egui::SliderClamping::Never),
                    );

                    let mut limit_frames = self.args.load_config.max_frames.is_some();
                    if ui.checkbox(&mut limit_frames, "Limit max frames").clicked() {
                        self.args.load_config.max_frames = if limit_frames { Some(32) } else { None };
                    }

                    if let Some(max_frames) = self.args.load_config.max_frames.as_mut() {
                        ui.add(Slider::new(max_frames, 1..=256).clamping(egui::SliderClamping::Never));
                    }

                    let mut use_eval_split = self.args.load_config.eval_split_every.is_some();
                    if ui
                        .checkbox(&mut use_eval_split, "Split dataset for evaluation")
                        .clicked()
                    {
                        self.args.load_config.eval_split_every =
                            if use_eval_split { Some(8) } else { None };
                    }

                    if let Some(eval_split) = self.args.load_config.eval_split_every.as_mut() {
                        ui.add(
                            Slider::new(eval_split, 2..=32)
                                .clamping(egui::SliderClamping::Never)
                                .prefix("1 out of ")
                                .suffix(" frames"),
                        );
                    }

                    ui.heading("Training Settings");

                    ui.horizontal(|ui| {
                        ui.label("Train");

                        ui.add(
                            egui::Slider::new(&mut self.args.train_config.total_steps, 1..=50000)
                                .clamping(egui::SliderClamping::Never)
                                .suffix(" steps"),
                        );
                    });

                    ui.heading("Process Settings");

                    ui.horizontal(|ui| {
                        ui.label("Evaluate");
                        ui.add(
                            egui::Slider::new(&mut self.args.process_config.eval_every, 1..=5000)
                                .clamping(egui::SliderClamping::Never)
                                .prefix("every ")
                                .suffix(" steps"),
                        );
                    });

                    #[cfg(not(target_family = "wasm"))]
                    {
                        ui.horizontal(|ui| {
                            ui.label("Export");
                            ui.add(
                                egui::Slider::new(&mut self.args.process_config.export_every, 1..=15000)
                                    .clamping(egui::SliderClamping::Never)
                                    .prefix("every ")
                                    .suffix(" steps"),
                            );
                        });
                    }

                    #[cfg(all(not(target_family = "wasm"), not(target_os = "android")))]
                    {
                        ui.heading("Rerun Settings");

                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            ui.hyperlink_to("Rerun.io", "https://rerun.io");
                            ui.label(" settings");
                        });
                        let rerun_config = &mut self.args.rerun_config;
                        ui.checkbox(&mut rerun_config.rerun_enabled, "Enable rerun");

                        if rerun_config.rerun_enabled {
                            ui.label(
                            "Open the brush_blueprint.rbl in the rerun viewer for a good default layout.",
                        );

                            ui.horizontal(|ui| {
                                ui.label("Log train stats");
                                ui.add(
                                    egui::Slider::new(
                                        &mut rerun_config.rerun_log_train_stats_every,
                                        1..=1000,
                                    )
                                    .clamping(egui::SliderClamping::Never)
                                    .prefix("every ")
                                    .suffix(" steps"),
                                );
                            });

                            let mut visualize_splats = rerun_config.rerun_log_splats_every.is_some();
                            ui.checkbox(&mut visualize_splats, "Visualize splats");
                            if visualize_splats != rerun_config.rerun_log_splats_every.is_some() {
                                rerun_config.rerun_log_splats_every =
                                    if visualize_splats { Some(500) } else { None };
                            }

                            if let Some(every) = rerun_config.rerun_log_splats_every.as_mut() {
                                ui.add(
                                    egui::Slider::new(every, 1..=5000)
                                        .clamping(egui::SliderClamping::Never)
                                        .text("Visualize splats every"),
                                );
                            }
                        }
                    }
                });
        });
        
        // Update open state and position/size if window was moved or resized
        if let Some(response) = response {
            self.open = window_open;
            self.position = response.response.rect.min;
            self.size = response.response.rect.size();
        }
    }
} 