use brush_dataset::splat_export;
use brush_process::process_loop::{ControlMessage, ProcessMessage};
use brush_train::{scene::ViewImageType, train::TrainBack};
use brush_ui::burn_texture::BurnTexture;
use burn::tensor::backend::AutodiffBackend;
use core::f32;
use egui::epaint::mutex::RwLock as EguiRwLock;
use std::sync::Arc;

use brush_render::{
    camera::{focal_to_fov, fov_to_focal},
    gaussian_splats::Splats,
};
use eframe::egui_wgpu::Renderer;
use egui::{Color32, Rect};
use glam::{Quat, UVec2, Vec3};
use tokio_with_wasm::alias as tokio_wasm;
use tracing::trace_span;
use web_time::Instant;

use crate::app::{AppContext, AppPanel};

#[derive(Debug, Clone, Copy, PartialEq)]
struct RenderState {
    size: UVec2,
    cam_pos: Vec3,
    cam_rot: Quat,

    frame: f32,
}

struct ErrorDisplay {
    headline: String,
    context: Vec<String>,
}

pub(crate) struct ScenePanel {
    pub(crate) backbuffer: BurnTexture,
    pub(crate) last_draw: Option<Instant>,

    view_splats: Vec<Splats<<TrainBack as AutodiffBackend>::InnerBackend>>,
    frame_count: u32,
    frame: f32,

    // Ui state.
    pub(crate) live_update: bool,
    paused: bool,
    err: Option<ErrorDisplay>,
    zen: bool,

    // Keep track of what was last rendered.
    last_state: Option<RenderState>,
}

impl ScenePanel {
    pub(crate) fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        renderer: Arc<EguiRwLock<Renderer>>,
        zen: bool,
    ) -> Self {
        Self {
            backbuffer: BurnTexture::new(renderer, device, queue),
            last_draw: None,
            err: None,
            view_splats: vec![],
            live_update: true,
            paused: false,
            last_state: None,
            zen,
            frame_count: 0,
            frame: 0.0,
        }
    }

    pub(crate) fn draw_splats(
        &mut self,
        ui: &mut egui::Ui,
        context: &mut AppContext,
        splats: &Splats<<TrainBack as AutodiffBackend>::InnerBackend>,
    ) {
        let size = brush_ui::size_for_splat_view(ui);

        if size.x < 8.0 || size.y < 8.0 {
            return;
        }

        let mut size = size.floor();

        if let Some(aspect_ratio) = context.view_aspect {
            if size.x / size.y > aspect_ratio {
                size.x = size.y * aspect_ratio;
            } else {
                size.y = size.x / aspect_ratio;
            }
        } else {
            let focal_y = fov_to_focal(context.camera.fov_y, size.y as u32) as f32;
            context.camera.fov_x = focal_to_fov(focal_y as f64, size.x as u32);
        }
        let size = glam::uvec2(size.x.round() as u32, size.y.round() as u32);

        let (rect, response) = ui.allocate_exact_size(
            egui::Vec2::new(size.x as f32, size.y as f32),
            egui::Sense::drag(),
        );

        context.controls.tick(&response, ui);

        let camera = &mut context.camera;

        // Create a camera that incorporates the model transform.
        let total_transform = context.model_local_to_world * context.controls.local_to_world();

        camera.position = total_transform.translation.into();
        camera.rotation = Quat::from_mat3a(&total_transform.matrix3);

        let state = RenderState {
            size,
            cam_pos: camera.position,
            cam_rot: camera.rotation,
            frame: self.frame,
        };

        let dirty = self.last_state != Some(state);

        if dirty {
            self.last_state = Some(state);

            // Check again next frame, as there might be more to animate.
            ui.ctx().request_repaint();
        }

        // If this viewport is re-rendering.
        if size.x > 0 && size.y > 0 && dirty {
            let _span = trace_span!("Render splats").entered();
            let (img, _) = splats.render(&context.camera, size, true);
            self.backbuffer.update_texture(img);
        }

        if let Some(id) = self.backbuffer.id() {
            ui.scope(|ui| {
                let mut background = false;
                if let Some(view) = context.dataset.train.views.first() {
                    if view.image.color().has_alpha() && view.img_type == ViewImageType::Alpha {
                        background = true;
                        // if training views have alpha, show a background checker. Masked images
                        // should still use a black background.
                        brush_ui::draw_checkerboard(ui, rect, Color32::WHITE);
                    }
                }

                // If a scene is opaque, it assumes a black background.
                if !background {
                    ui.painter().rect_filled(rect, 0.0, Color32::BLACK);
                }

                ui.painter().image(
                    id,
                    rect,
                    Rect {
                        min: egui::pos2(0.0, 0.0),
                        max: egui::pos2(1.0, 1.0),
                    },
                    Color32::WHITE,
                );
            });
        }
    }

    /// Get the current splats for export
    pub(crate) fn get_current_splats(&self) -> Option<&Splats<<TrainBack as AutodiffBackend>::InnerBackend>> {
        if self.view_splats.is_empty() {
            None
        } else {
            // Get the last frame
            let frame = (self.frame * 24.0)
                .rem_euclid(self.frame_count as f32)
                .floor() as usize;
            self.view_splats.get(frame)
        }
    }
}

impl AppPanel for ScenePanel {
    fn title(&self) -> String {
        "".to_owned()
    }

    fn on_message(&mut self, message: &ProcessMessage, context: &mut AppContext) {
        match message {
            ProcessMessage::NewSource => {
                self.view_splats = vec![];
                self.frame_count = 0;
                self.live_update = true;
                self.paused = false;
                self.err = None;
                self.last_state = None;
                self.frame = 0.0;
            }
            ProcessMessage::ViewSplats {
                up_axis,
                splats,
                frame,
                total_frames,
            } => {
                if let Some(up_axis) = up_axis {
                    context.set_model_up(*up_axis);
                }

                if self.live_update {
                    self.view_splats.truncate(*frame as usize);
                    self.view_splats.push(*splats.clone());
                }
                self.frame_count = *total_frames;
                self.last_state = None;
            }
            ProcessMessage::TrainStep {
                splats,
                stats: _,
                iter: _,
                timestamp: _,
            } => {
                self.last_state = None;

                let splats = *splats.clone();

                if self.live_update {
                    self.view_splats = vec![splats];
                }
            }
            ProcessMessage::Error(e) => {
                let headline = e.to_string();
                let context = e.chain().skip(1).map(|cause| format!("{cause}")).collect();
                self.err = Some(ErrorDisplay { headline, context });
            }
            _ => {}
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, context: &mut AppContext) {
        let cur_time = Instant::now();
        self.last_draw = Some(cur_time);

        // Debug: Show the available space in the panel
        let available_rect = ui.available_rect_before_wrap();
        ui.painter().rect_filled(
            available_rect,
            0.0,
            Color32::from_rgba_premultiplied(255, 0, 0, 10) // Very transparent red
        );
        
        // Debug: Add text showing dimensions
        ui.painter().text(
            available_rect.left_bottom() + egui::vec2(5.0, -5.0),
            egui::Align2::LEFT_BOTTOM,
            format!("Available: {}x{}", available_rect.width(), available_rect.height()),
            egui::FontId::proportional(10.0),
            Color32::RED
        );

        // Empty scene, nothing to show.
        if !context.training() && self.view_splats.is_empty() && self.err.is_none() && !self.zen {
            ui.heading("Load a ply file or dataset to get started.");
            ui.add_space(5.0);
            ui.label(
                r#"
Load a pretrained .ply file to view it

Or load a dataset to train on. These are zip files with:
    - a transforms.json and images, like the nerfstudio dataset format.
    - COLMAP data, containing the `images` & `sparse` folder."#,
            );

            ui.add_space(10.0);

            if cfg!(debug_assertions) {
                ui.scope(|ui| {
                    ui.visuals_mut().override_text_color = Some(Color32::LIGHT_BLUE);
                    ui.heading(
                        "Note: running in debug mode, compile with --release for best performance",
                    );
                });

                ui.add_space(10.0);
            }

            #[cfg(target_family = "wasm")]
            ui.scope(|ui| {
                ui.visuals_mut().override_text_color = Some(Color32::YELLOW);
                ui.heading("Note: Running in browser is still experimental");

                ui.label(
                    r#"
In browser training is slower, and lower quality than the native app.

For bigger training runs consider using the native app."#,
                );
            });

            return;
        }

        if let Some(err) = self.err.as_ref() {
            ui.heading(format!("❌ {}", err.headline));

            ui.indent("err_context", |ui| {
                for c in &err.context {
                    ui.label(format!("• {c}"));
                    ui.add_space(2.0);
                }
            });
        } else if !self.view_splats.is_empty() {
            const FPS: f32 = 24.0;

            if !self.paused {
                self.frame += ui.input(|r| r.predicted_dt);
            }
            if self.view_splats.len() as u32 != self.frame_count {
                let max_t = (self.view_splats.len() - 1) as f32 / FPS;
                self.frame = self.frame.min(max_t);
            }

            let frame = (self.frame * FPS)
                .rem_euclid(self.frame_count as f32)
                .floor() as usize;
            let splats = self.view_splats[frame].clone();

            self.draw_splats(ui, context, &splats);

            if self.view_splats.len() > 1 && self.view_splats.len() as u32 == self.frame_count {
                let label = if self.paused {
                    "⏸ paused"
                } else {
                    "⏵ playing"
                };

                if ui.selectable_label(!self.paused, label).clicked() {
                    self.paused = !self.paused;
                }
            }
        }
        
        // Debug: Show the remaining space at the end of the panel
        let remaining_rect = ui.available_rect_before_wrap();
        ui.painter().rect_filled(
            remaining_rect,
            0.0,
            Color32::from_rgba_premultiplied(0, 0, 255, 10) // Very transparent blue
        );
        
        // Debug: Add text showing dimensions
        if remaining_rect.height() > 0.0 {
            ui.painter().text(
                remaining_rect.left_top() + egui::vec2(5.0, 15.0),
                egui::Align2::LEFT_TOP,
                format!("Remaining: {}x{}", remaining_rect.width(), remaining_rect.height()),
                egui::FontId::proportional(10.0),
                Color32::BLUE
            );
        }
    }

    fn inner_margin(&self) -> f32 {
        0.0
    }
}
