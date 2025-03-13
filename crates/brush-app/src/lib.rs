#![recursion_limit = "256"]

mod camera_controls;
mod panels;

mod app;
mod channel;

pub use app::*;
use burn::backend::Autodiff;
use burn_wgpu::Wgpu;
pub type MainBackend = Autodiff<Wgpu>;
