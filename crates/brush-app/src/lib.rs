#![recursion_limit = "256"]

pub mod app;
pub mod channel;
pub mod export_service;
pub mod orbit_controls;
pub mod panels;
pub mod overlays;

pub use app::App;
use burn::backend::Autodiff;
use burn_wgpu::Wgpu;
pub type MainBackend = Autodiff<Wgpu>;
