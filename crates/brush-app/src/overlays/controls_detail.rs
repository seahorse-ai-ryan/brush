use crate::app::AppContext;
use brush_process::process_loop::{ControlMessage, ProcessMessage};
use egui::{Context, Pos2, Vec2, pos2, Align2, Color32};

pub(crate) struct ControlsDetailOverlay {
    // UI state
    open: bool,
    position: Pos2,
    size: Vec2,
    
    // Control state
    paused: bool,
    live_update: bool,
}

impl ControlsDetailOverlay {
    pub(crate) fn new() -> Self {
        Self {
            // UI state
            open: false, // Start with window closed
            position: pos2(300.0, 300.0),
            size: Vec2::new(229.5, 64.0), // Reduced height to 50% of previous value
            
            // Control state
            paused: false,
            live_update: true,
        }
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
        
        // Create a unique window ID - make it static to maintain window state
        let window_id = egui::Id::new("controls_detail_window");
        
        // Track open state locally to avoid borrow issues
        let mut window_open = self.open;
        
        // Create the window with settings to ensure proper resizability
        let window = egui::Window::new("🎮 Controls")
            .id(window_id)
            .open(&mut window_open)
            .resizable(true)
            .movable(true)
            .collapsible(true)
            .default_pos(self.position)
            .default_size(self.size)
            .min_width(180.0)
            .min_height(80.0);
        
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
                    ui.add_space(5.0);
                    
                    // Check the AppContext's training state directly
                    if context.training() {
                        ui.horizontal(|ui| {
                            let label = if self.paused {
                                "⏸ paused"
                            } else {
                                "⏵ training"
                            };
                            
                            if ui.selectable_label(!self.paused, label).clicked() {
                                self.paused = !self.paused;
                                context.control_message(ControlMessage::Paused(self.paused));
                            }
                            
                            ui.add_space(8.0);
                            
                            ui.scope(|ui| {
                                ui.style_mut().visuals.selection.bg_fill = Color32::DARK_RED;
                                if ui
                                    .selectable_label(self.live_update, "🔴 Live update")
                                    .clicked()
                                {
                                    self.live_update = !self.live_update;
                                    
                                    // Send a message to update the Scene panel's live_update flag
                                    context.control_message(ControlMessage::LiveUpdate(self.live_update));
                                }
                            });
                        });
                        
                        ui.add_space(5.0);
                        
                        ui.horizontal(|ui| {
                            let export_button = ui.button("⬆ Export");
                            if export_button.clicked() {
                                // Use the export_splats_with_service method on the AppContext
                                context.export_splats_with_service();
                            }
                            
                            if export_button.hovered() {
                                export_button.on_hover_text("Export the current 3D model to a PLY file");
                            }
                            
                            ui.add_space(5.0);
                            
                            // Replace the Navigation Controls button with a question mark icon
                            let help_button = ui.button("❓");
                            if help_button.hovered() {
                                help_button.on_hover_ui(|ui| {
                                    ui.heading("Navigation Controls");
                                    ui.add_space(5.0);
                                    
                                    ui.label("• Left click and drag to orbit");
                                    ui.label("• Right click, or left click + spacebar, and drag to look around");
                                    ui.label("• Middle click, or left click + control, and drag to pan");
                                    ui.label("• Scroll to zoom");
                                    ui.label("• WASD to fly, Q&E to move up & down");
                                    ui.label("• Z&C to roll, X to reset roll");
                                    ui.label("• Shift to move faster");
                                });
                            }
                        });
                    } else {
                        // Display a message when not training
                        ui.label("No active training session.");
                        
                        // Still show the help button for navigation controls
                        ui.add_space(5.0);
                        
                        let help_button = ui.button("❓");
                        if help_button.hovered() {
                            help_button.on_hover_ui(|ui| {
                                ui.heading("Navigation Controls");
                                ui.add_space(5.0);
                                
                                ui.label("• Left click and drag to orbit");
                                ui.label("• Right click, or left click + spacebar, and drag to look around");
                                ui.label("• Middle click, or left click + control, and drag to pan");
                                ui.label("• Scroll to zoom");
                                ui.label("• WASD to fly, Q&E to move up & down");
                                ui.label("• Z&C to roll, X to reset roll");
                                ui.label("• Shift to move faster");
                            });
                        }
                    }
                });
        });
        
        // Check if the window was moved and update our stored position
        if let Some(inner_response) = response {
            let new_pos = inner_response.response.rect.min;
            if new_pos != self.position {
                self.position = new_pos;
            }
            
            // Update size if it changed
            let new_size = inner_response.response.rect.size();
            if new_size != self.size {
                self.size = new_size;
            }
            
            // Update window open state
            if window_open != self.open {
                self.set_open(window_open);
            }
        }
    }
    
    pub(crate) fn get_live_update(&self) -> bool {
        self.live_update
    }
    
    pub(crate) fn set_live_update(&mut self, live_update: bool) {
        self.live_update = live_update;
    }
    
    pub(crate) fn get_paused(&self) -> bool {
        self.paused
    }
    
    pub(crate) fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }
    
    /// Set the position of the Controls overlay
    pub(crate) fn set_position(&mut self, position: Pos2) {
        self.position = position;
    }
    
    /// Reset the Controls overlay state when a new dataset is loaded
    pub(crate) fn reset_state(&mut self) {
        self.paused = false;
        self.live_update = true;
    }

    /// Handle process messages to update the Controls overlay state
    pub(crate) fn on_message(&mut self, message: &brush_process::process_loop::ProcessMessage) {
        match message {
            brush_process::process_loop::ProcessMessage::NewSource => {
                // Reset control state but preserve open state
                let was_open = self.open;
                self.paused = false;
                self.live_update = true;
                self.open = was_open;
            },
            brush_process::process_loop::ProcessMessage::StartLoading { training } => {
                // Update state based on training mode
                if *training {
                    // Keep the panel open when training starts
                    self.open = true;
                    
                    // Reset control state
                    self.paused = false;
                    self.live_update = true;
                }
            },
            brush_process::process_loop::ProcessMessage::DoneLoading { training } => {
                // Update state based on training mode
                if *training {
                    // Keep the panel open when training is done loading
                    self.open = true;
                }
            },
            brush_process::process_loop::ProcessMessage::TrainStep { .. } => {
                // No longer force the panel to open on every train step
                // This allows users to close the panel if they wish
            },
            _ => {}
        }
    }
} 