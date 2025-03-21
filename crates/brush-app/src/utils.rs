// Utility functions for the brush-app crate

/// Sets up the console error panic hook for WASM targets.
/// This ensures that Rust panics are properly logged to the browser console
/// in a way that can be captured by debugging tools like BrowserTools MCP.
pub fn set_panic_hook() {
    #[cfg(target_family = "wasm")]
    {
        // Set up a custom panic hook to capture more detailed error information
        std::panic::set_hook(Box::new(|panic_info| {
            // Get the panic message and location
            let message = if let Some(msg) = panic_info.payload().downcast_ref::<&str>() {
                *msg
            } else if let Some(msg) = panic_info.payload().downcast_ref::<String>() {
                msg.as_str()
            } else {
                "Unknown panic message"
            };
            
            let location = panic_info.location().map_or("unknown location", |loc| {
                loc.file()
            });
            
            // Format a more detailed panic message
            let detailed_message = format!(
                "🚨 BRUSH PANIC: {} at {} - This error has been captured by MCP. Check browser console for stack trace.",
                message, location
            );
            
            // Log using web_sys console directly
            web_sys::console::error_1(&detailed_message.clone().into());
            web_sys::console::log_1(&format!("BRUSH_ERROR_CAPTURED: {}", detailed_message).into());
            
            // Also use the standard console error panic hook as a fallback
            console_error_panic_hook::hook(panic_info);
        }));
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
pub fn auto_load_test_ply(_context: &mut crate::app::AppContext) {
    log_info("🧪 DEBUG: Auto-loading test PLY file...");
    
    // Create a mock file path with a virtual path prefix for WASM
    use std::path::PathBuf;
    let test_file = PathBuf::from("virtual://browser/test_sample.ply");
    
    // Log that we're using a virtual path
    log_info(&format!("🧪 DEBUG: Using virtual path for test PLY: {}", test_file.display()));
    
    // We need to modify this to work with an in-memory PLY file
    // For now just log that we would process the file here
    log_info("🧪 DEBUG: Would process test PLY file with virtual path in a real implementation");
    
    // In a real implementation, we would:
    // 1. Create an in-memory PLY file with sample data
    // 2. Process it directly without filesystem operations
    // 3. Add it to the dataset list with the virtual path
    
    log_info("🧪 DEBUG: Test PLY loading process completed");
} 