// Utility functions for the brush-app crate
use std::sync::Once;

/// Sets up the console error panic hook for WASM targets.
/// This ensures that Rust panics are properly logged to the browser console
/// in a way that can be captured by debugging tools like BrowserTools MCP.
pub fn set_panic_hook() {
    #[cfg(target_family = "wasm")]
    {
        // Set up the standard panic hook
        console_error_panic_hook::set_once();
    }
}

/// Logs a message to the console with a "BRUSH_DEBUG" prefix
/// This helps identify Brush-specific messages in the console
#[cfg(target_family = "wasm")]
pub fn log_debug(message: &str) {
    web_sys::console::log_1(&format!("BRUSH_DEBUG: {}", message).into());
}

/// Logs an error message to the console with a "BRUSH_ERROR" prefix
#[cfg(target_family = "wasm")]
pub fn log_error(message: &str) {
    // Log as both error and regular log to ensure it's captured by MCP
    web_sys::console::error_1(&format!("BRUSH_ERROR: {}", message).into());
    // Also log as regular console.log to ensure it's captured by MCP
    web_sys::console::log_1(&format!("BRUSH_ERROR_CAPTURED: {}", message).into());
}

/// Logs a warning message to the console with a "BRUSH_WARN" prefix
#[cfg(target_family = "wasm")]
pub fn log_warn(message: &str) {
    web_sys::console::warn_1(&format!("BRUSH_WARN: {}", message).into());
    // Also log as regular log to ensure it's captured by MCP
    web_sys::console::log_1(&format!("BRUSH_WARN_CAPTURED: {}", message).into());
}

/// Logs an info message to the console with a "BRUSH_INFO" prefix
#[cfg(target_family = "wasm")]
pub fn log_info(message: &str) {
    web_sys::console::info_1(&format!("BRUSH_INFO: {}", message).into());
}

/// Logs a message to the console for non-WASM targets
#[cfg(not(target_family = "wasm"))]
pub fn log_debug(message: &str) {
    println!("BRUSH_DEBUG: {}", message);
}

/// Logs an error message to the console for non-WASM targets
#[cfg(not(target_family = "wasm"))]
pub fn log_error(message: &str) {
    eprintln!("BRUSH_ERROR: {}", message);
}

/// Logs a warning message to the console for non-WASM targets
#[cfg(not(target_family = "wasm"))]
pub fn log_warn(message: &str) {
    println!("BRUSH_WARN: {}", message);
}

/// Logs an info message to the console for non-WASM targets
#[cfg(not(target_family = "wasm"))]
pub fn log_info(message: &str) {
    println!("BRUSH_INFO: {}", message);
}

/// Auto-loads a test PLY file for debugging purposes
/// This function can be triggered by URL parameters or debug flags
#[cfg(target_family = "wasm")]
pub fn auto_load_test_ply(context: &mut crate::app::AppContext) {
    log_info("🧪 DEBUG: Auto-loading test PLY file...");
    
    // Create a mock file path
    use std::path::PathBuf;
    let test_file = PathBuf::from("test_data/sample.ply");
    
    // Note: We need to modify this to work with the App struct instead 
    // since AppContext doesn't have dataset_detail_overlay
    
    // This function is used by URL parameter handlers and should be updated
    // to match the actual structure of the application
    
    log_info("🧪 DEBUG: Auto-load test PLY file feature needs updating for the new UI structure");
} 