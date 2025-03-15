use core::f32;

use egui::Response;
use glam::{Quat, Vec2, Vec3};

pub struct CameraController {
    pub position: Vec3,
    pub rotation: Quat,
    pub focus_distance: f32,

    min_focus_distance: Option<f32>,
    max_focus_distance: Option<f32>,

    min_pitch: Option<f32>,
    max_pitch: Option<f32>,

    roll: Quat,
    fly_velocity: Vec3,
    orbit_velocity: Vec2,
    speed_scale: f32,
}

pub fn smooth_orbit(
    position: Vec3,
    rotation: Quat,
    base_roll: Quat,
    delta_yaw: f32,
    delta_pitch: f32,

    min_pitch: Option<f32>,
    max_pitch: Option<f32>,
    dt: f32,

    distance: f32,
) -> (Vec3, Quat) {
    // Calculate focal point (where we're looking at)
    let focal_point = position + rotation * Vec3::Z * distance;

    // Extract current pitch angle from rotation
    // We need to determine the current pitch relative to the base orientation
    let forward = rotation * Vec3::Z;
    let current_pitch = -forward.y.asin();

    // Clamp the new pitch angle
    let new_pitch = smooth_clamp(
        current_pitch - delta_pitch,
        min_pitch.map(|x| x.to_radians()),
        max_pitch.map(|x| x.to_radians()),
        dt,
        50.0,
    );

    // New delta clamped to not go over min/max pitch.
    let delta_pitch = current_pitch - new_pitch;

    // Create rotation quaternions in camera's local space
    let pitch_axis = rotation * Vec3::X;
    let pitch = Quat::from_axis_angle(pitch_axis, -delta_pitch);

    let yaw_axis = base_roll * Vec3::NEG_Y;
    let yaw = Quat::from_axis_angle(yaw_axis, -delta_yaw);

    let new_rotation = (yaw * pitch * rotation).normalize();
    let new_position = focal_point - new_rotation * Vec3::Z * distance;
    (new_position, new_rotation)
}

fn exp_lerp(a: f32, b: f32, dt: f32, lambda: f32) -> f32 {
    let lerp_exp = (-lambda * dt).exp();
    a * lerp_exp + b * (1.0 - lerp_exp)
}

fn exp_lerp2(a: Vec2, b: Vec2, dt: f32, lambda: f32) -> Vec2 {
    glam::vec2(
        exp_lerp(a.x, b.x, dt, lambda),
        exp_lerp(a.y, b.y, dt, lambda),
    )
}

fn exp_lerp3(a: Vec3, b: Vec3, dt: f32, lambda: f32) -> Vec3 {
    glam::vec3(
        exp_lerp(a.x, b.x, dt, lambda),
        exp_lerp(a.y, b.y, dt, lambda),
        exp_lerp(a.z, b.z, dt, lambda),
    )
}

fn smooth_clamp(val: f32, min: Option<f32>, max: Option<f32>, dt: f32, lambda: f32) -> f32 {
    let mut target = val;
    if let Some(min) = min {
        target = target.max(min);
    }
    if let Some(max) = max {
        target = target.min(max);
    }
    exp_lerp(val, target, dt, lambda)
}

impl CameraController {
    pub fn new(
        radius: f32,
        focus_distance: f32,
        speed_scale: f32,
        min_focus_distance: Option<f32>,
        max_focus_distance: Option<f32>,
        min_pitch: Option<f32>,
        max_pitch: Option<f32>,
    ) -> Self {
        Self {
            position: -Vec3::Z * radius,
            rotation: Quat::IDENTITY,
            roll: Quat::IDENTITY,
            fly_velocity: Vec3::ZERO,
            orbit_velocity: Vec2::ZERO,

            focus_distance,

            min_focus_distance,
            max_focus_distance,

            min_pitch,
            max_pitch,

            speed_scale,
        }
    }

    pub fn tick(&mut self, response: &Response, ui: &egui::Ui) {
        let delta_time = ui.input(|r| r.predicted_dt);

        let lmb = response.dragged_by(egui::PointerButton::Primary);
        let rmb = response.dragged_by(egui::PointerButton::Secondary);
        let mmb = response.dragged_by(egui::PointerButton::Middle);

        let look_pan = mmb || lmb && ui.input(|r| r.modifiers.ctrl);
        let look_fps = rmb || lmb && ui.input(|r| r.key_down(egui::Key::Space));
        let look_orbit = lmb;

        let mouselook_speed = 0.002;

        let right = self.rotation * Vec3::X;
        let up = self.rotation * Vec3::NEG_Y;
        let forward = self.rotation * Vec3::Z;

        if response.hovered() {
            if ui.input(|r| r.modifiers.ctrl) {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Move);
            } else if ui.input(|r| r.key_down(egui::Key::Space)) {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
            } else {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
        }

        if look_pan {
            let drag_mult = self.focus_distance / response.rect.width().max(response.rect.height());
            self.position -= right * response.drag_delta().x * drag_mult;
            self.position += up * response.drag_delta().y * drag_mult;
            ui.ctx().set_cursor_icon(egui::CursorIcon::Move);
        } else if look_fps {
            let axis = response.drag_delta();
            let yaw = Quat::from_axis_angle(self.roll * Vec3::NEG_Y, -axis.x * mouselook_speed);
            let pitch = Quat::from_rotation_x(-axis.y * mouselook_speed);
            self.rotation = yaw * self.rotation * pitch;
            ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
        } else if look_orbit {
            let delta_yaw = response.drag_delta().x * mouselook_speed;
            let delta_pitch = response.drag_delta().y * mouselook_speed;
            self.orbit_velocity = glam::vec2(delta_yaw, delta_pitch);
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        (self.position, self.rotation) = smooth_orbit(
            self.position,
            self.rotation,
            self.roll,
            self.orbit_velocity.x,
            self.orbit_velocity.y,
            self.min_pitch,
            self.max_pitch,
            delta_time,
            self.focus_distance,
        );

        let fly_moment_lambda = 0.8;

        let move_speed = 25.0
            * self.speed_scale
            * if ui.input(|r| r.modifiers.shift) {
                4.0
            } else {
                1.0
            };

        if ui.input(|r| r.key_down(egui::Key::W) || r.key_down(egui::Key::ArrowUp)) {
            self.fly_velocity = exp_lerp3(
                self.fly_velocity,
                Vec3::Z * move_speed,
                delta_time,
                fly_moment_lambda,
            );
        }
        if ui.input(|r| r.key_down(egui::Key::A) || r.key_down(egui::Key::ArrowLeft)) {
            self.fly_velocity = exp_lerp3(
                self.fly_velocity,
                -Vec3::X * move_speed,
                delta_time,
                fly_moment_lambda,
            );
        }
        if ui.input(|r| r.key_down(egui::Key::S) || r.key_down(egui::Key::ArrowDown)) {
            self.fly_velocity = exp_lerp3(
                self.fly_velocity,
                -Vec3::Z * move_speed,
                delta_time,
                fly_moment_lambda,
            );
        }
        if ui.input(|r| r.key_down(egui::Key::D) || r.key_down(egui::Key::ArrowRight)) {
            self.fly_velocity = exp_lerp3(
                self.fly_velocity,
                Vec3::X * move_speed,
                delta_time,
                fly_moment_lambda,
            );
        }

        if ui.input(|r| r.modifiers.alt) {
        } else {
            // Move _down_ with Q
            if ui.input(|r| r.key_down(egui::Key::Q)) {
                self.fly_velocity = exp_lerp3(
                    self.fly_velocity,
                    -Vec3::Y * move_speed,
                    delta_time,
                    fly_moment_lambda,
                );
            }
            // Move up with E
            if ui.input(|r| r.key_down(egui::Key::E)) {
                self.fly_velocity = exp_lerp3(
                    self.fly_velocity,
                    Vec3::Y * move_speed,
                    delta_time,
                    fly_moment_lambda,
                );
            }
        }

        // Roll with alt + Q&E.
        if ui.input(|r| r.key_down(egui::Key::Z)) {
            let roll = Quat::from_axis_angle(forward, move_speed * 0.025 * delta_time);
            self.rotation = roll * self.rotation;
            self.roll = roll * self.roll;
        }
        if ui.input(|r| r.key_down(egui::Key::X)) {
            self.rotation = self.roll.inverse() * self.rotation;
            self.roll = Quat::IDENTITY;
        }
        if ui.input(|r| r.key_down(egui::Key::C)) {
            let roll = Quat::from_axis_angle(forward, -move_speed * 0.025 * delta_time);
            self.rotation = roll * self.rotation;
            self.roll = roll * self.roll;
        }

        let delta = self.fly_velocity * delta_time;
        self.position += delta.x * right + delta.y * up + delta.z * forward;

        // Damp velocities towards zero.
        self.orbit_velocity = exp_lerp2(self.orbit_velocity, Vec2::ZERO, delta_time, 8.0);
        self.fly_velocity = exp_lerp3(self.fly_velocity, Vec3::ZERO, delta_time, 7.0);

        // Handle scroll wheel: move back, and adjust focus distance.
        let scrolled = ui.input(|r| r.smooth_scroll_delta.y);
        let scroll_speed = 0.001;

        let old_pivot = self.position + self.rotation * Vec3::Z * self.focus_distance;

        // Scroll speed depends on how far zoomed out we are.
        self.focus_distance -= scrolled * scroll_speed * self.focus_distance;
        self.focus_distance = self.focus_distance.max(0.01);

        self.focus_distance = smooth_clamp(
            self.focus_distance,
            self.min_focus_distance,
            self.max_focus_distance,
            delta_time,
            50.5,
        );

        self.position = old_pivot - (self.rotation * Vec3::Z * self.focus_distance);
    }

    pub fn local_to_world(&self) -> glam::Affine3A {
        glam::Affine3A::from_rotation_translation(self.rotation, self.position)
    }

    pub(crate) fn stop_movement(&mut self) {
        self.orbit_velocity = Vec2::ZERO;
        self.fly_velocity = Vec3::ZERO;
    }
}
