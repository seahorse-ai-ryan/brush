#![recursion_limit = "256"]

pub mod eval;
pub mod train;

mod adam_scaled;
mod multinomial;
mod quat_vec;
mod ssim;
mod stats;
mod stats_kernel;
#[cfg(all(test, not(target_family = "wasm")))]
mod tests;
