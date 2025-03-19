#![recursion_limit = "256"]

#[allow(unused)]
use brush_app::App;
use brush_app::app::AppCreateCb;
use brush_app::utils::set_panic_hook;

use brush_process::process_loop::start_process;
#[allow(unused)]
use tokio::sync::oneshot::error::RecvError;

#[cfg(not(target_family = "wasm"))]
type MainResult = Result<(), clap::Error>;

#[cfg(target_family = "wasm")]
type MainResult = Result<(), ()>;

// Simple build timestamp for debugging
const BUILD_TIME: &str = "2025-03-17 (manual)";

#[allow(clippy::unnecessary_wraps)] // Error isn't need on wasm but that's ok.
fn main() -> MainResult {
    let wgpu_options = brush_ui::create_egui_options();

    #[cfg(not(target_family = "wasm"))]
    {
        let (send, rec) = tokio::sync::oneshot::channel::<AppCreateCb>();

        use brush_cli::Cli;
        use clap::Parser;

        let args = Cli::parse().validate()?;

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to initialize tokio runtime");

        runtime.block_on(async {
            env_logger::init();

            if args.with_viewer {
                let icon = eframe::icon_data::from_png_bytes(
                    &include_bytes!("../../assets/icon-256.png")[..],
                )
                .expect("Failed to load icon");

                let native_options = eframe::NativeOptions {
                    // Build app display.
                    viewport: egui::ViewportBuilder::default()
                        .with_inner_size(egui::Vec2::new(1450.0, 1200.0))
                        .with_active(true)
                        .with_icon(std::sync::Arc::new(icon)),
                    wgpu_options,
                    ..Default::default()
                };

                if let Some(source) = args.source {
                    if let Ok(context_cb) = rec.blocking_recv() {
                        let process_args = args.process.clone();
                        let device = {
                            let context = context_cb.context.read().expect("Lock poisoned");
                            context.device.clone()
                        };
                        
                        let process = start_process(source, process_args, device);
                        
                        let mut context = context_cb.context.write().expect("Lock poisoned");
                        context.connect_to(process);
                    }
                }

                let title = if cfg!(debug_assertions) {
                    format!("Brush - Debug [{}]", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"))
                } else {
                    "Brush".to_string()
                };

                eframe::run_native(
                    &title,
                    native_options,
                    Box::new(move |cc| Ok(Box::new(App::new(cc, send, None, args.reset_windows)))),
                )
                .expect("Failed to run egui app");
            } else {
                let Some(source) = args.source else {
                    panic!("Validation of args failed?");
                };

                let device = brush_render::burn_init_setup().await;
                let process = start_process(source, args.process, device);
                brush_cli::ui::process_ui(process).await;
            }
        });
    }

    #[cfg(target_family = "wasm")]
    {
        use tokio_with_wasm::alias as tokio_wasm;
        use wasm_bindgen::JsCast;
        use brush_app::utils::log_info;

        // Set up the panic hook to ensure errors are properly logged to console
        set_panic_hook();
        
        // Log application startup with a distinctive message that should be easy to spot
        log_info("🔴🔴🔴 BRUSH APPLICATION STARTING 🔴🔴🔴");
        
        // Also try direct web_sys console log
        web_sys::console::log_1(&"🟢🟢🟢 DIRECT CONSOLE.LOG TEST 🟢🟢🟢".into());
        
        if cfg!(debug_assertions) {
            eframe::WebLogger::init(log::LevelFilter::Debug).ok();
        }

        let document = web_sys::window()
            .expect("Failed to find web window (not running in a browser?")
            .document()
            .expect("Failed to find document body");

        if let Some(canvas) = document
            .get_element_by_id("main_canvas")
            .and_then(|x| x.dyn_into::<web_sys::HtmlCanvasElement>().ok())
        {
            // On wasm, run as a local task.
            tokio_wasm::task::spawn(async {
                let web_options = eframe::WebOptions {
                    wgpu_options,
                    ..Default::default()
                };

                let (send, _rec) = tokio::sync::oneshot::channel();

                // Set the document title with build timestamp in debug mode
                if cfg!(debug_assertions) {
                    if let Some(window) = web_sys::window() {
                        let document = window.document().unwrap();
                        document.set_title(&format!("Brush - Debug [{}]", 
                            js_sys::Date::new_0().to_iso_string().as_string().unwrap_or_default()));
                    }
                }

                eframe::WebRunner::new()
                    .start(
                        canvas,
                        web_options,
                        Box::new(|cc| Ok(Box::new(App::new(cc, send, None, false)))),
                    )
                    .await
                    .expect("failed to start eframe");
            });
        }
    }

    Ok(())
}

#[cfg(target_family = "wasm")]
mod embedded {
    use super::start_process;
    use brush_app::App;
    use brush_app::utils;
    use brush_app::utils::set_panic_hook;
    use brush_process::{data_source::DataSource, process_loop::ProcessArgs};
    use std::future::IntoFuture;
    use tokio::sync::mpsc::UnboundedSender;
    use tokio_with_wasm::alias as tokio_wasm;
    use wasm_bindgen::prelude::*;

    enum EmbeddedCommands {
        LoadDataSource(DataSource),
        SetUpVector(glam::Vec3),
    }

    #[wasm_bindgen]
    pub struct EmbeddedApp {
        command_channel: UnboundedSender<EmbeddedCommands>,
    }

    #[wasm_bindgen]
    impl EmbeddedApp {
        #[wasm_bindgen(constructor)]
        pub fn new(canvas_name: &str, start_uri: &str) -> Self {
            // Set up the panic hook to ensure errors are properly logged to console
            set_panic_hook();
            
            // Log embedded app initialization
            utils::log_info(&format!("Initializing embedded Brush app with canvas: {}", canvas_name));
            
            let wgpu_options = brush_ui::create_egui_options();
            let document = web_sys::window()
                .expect("Failed to get winow")
                .document()
                .expect("Failed to get document");
            let canvas = document
                .get_element_by_id(canvas_name)
                .unwrap_or_else(|| panic!("Failed to find canvas with id: {canvas_name}"))
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap_or_else(|_| panic!("Found canvas {canvas_name} was in fact not a canvas"));

            utils::log_debug("Canvas element found and validated");

            let (send, rec) = tokio::sync::oneshot::channel();

            let (cmd_send, mut cmd_rec) = tokio::sync::mpsc::unbounded_channel();

            let start_uri = start_uri.to_owned();

            // On wasm, run as a local task.
            tokio_wasm::task::spawn(async move {
                eframe::WebRunner::new()
                    .start(
                        canvas,
                        eframe::WebOptions::default(),
                        Box::new(move |cc| Ok(Box::new(App::new(cc, send, Some(start_uri.clone()), false)))),
                    )
                    .await
                    .expect("Failed to start eframe");
            });

            tokio_wasm::task::spawn(async move {
                let context = rec
                    .into_future()
                    .await
                    .expect("Failed to start Brush, failed to receive context")
                    .context;

                while let Some(command) = cmd_rec.recv().await {
                    let mut ctx = context.write().expect("Failed to lock context (poisoned)");

                    match command {
                        EmbeddedCommands::LoadDataSource(data_source) => {
                            let process = start_process(
                                data_source,
                                ProcessArgs::default(),
                                ctx.device.clone(),
                            );
                            ctx.connect_to(process);
                        }
                        EmbeddedCommands::SetUpVector(up_axis) => {
                            ctx.set_model_up(up_axis);
                        }
                    }
                }
            });
            Self {
                command_channel: cmd_send,
            }
        }

        #[wasm_bindgen]
        pub fn load_url(&self, url: &str) {
            self.command_channel
                .send(EmbeddedCommands::LoadDataSource(DataSource::Url(
                    url.to_owned(),
                )))
                .expect("Viewer was closed?");
        }

        #[wasm_bindgen]
        pub fn set_up_vec(&self, x: f32, y: f32, z: f32) {
            let vec = glam::vec3(x, y, z).normalize();

            self.command_channel
                .send(EmbeddedCommands::SetUpVector(vec))
                .expect("Viewer was closed?");
        }
    }
}

#[cfg(target_family = "wasm")]
pub use embedded::*;
