mod data_source;
mod orbit_controls;
mod panels;
mod process_loop;
mod rerun_tools;

mod app;

pub use app::*;
use burn::backend::Autodiff;
use burn_wgpu::Wgpu;
pub type MainBackend = Autodiff<Wgpu>;
