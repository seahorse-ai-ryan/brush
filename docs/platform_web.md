# Web Platform 🌐

This document provides information about building, running, and optimizing Brush on the web platform.

## Overview

Brush works in modern web browsers through WebAssembly (WASM) compilation of the core Rust codebase. The web version offers most features of the desktop application with adaptations for the browser environment.

## Browser Requirements

Brush on the web requires:

- **Modern browsers** with WebGPU support:
  - Chrome 113+ or Chrome Canary with WebGPU flag enabled
  - Firefox Nightly with WebGPU flag enabled
  - Edge 113+ or Edge Canary with WebGPU flag enabled
- **WebAssembly support** (all modern browsers)
- **Hardware acceleration** enabled

## Building for Web

### Development Environment Setup

1. Install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install WebAssembly target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. Install Trunk (WASM bundler):
   ```bash
   cargo install trunk
   ```

### Building the Web Version

1. Clone the repository:
   ```bash
   git clone https://github.com/ArthurBrussee/brush.git
   cd brush
   ```

2. Build with Trunk:
   ```bash
   cd crates/brush-web
   trunk build --release
   ```

3. The output will be in the `dist` directory.

### Running Locally

1. Start a local development server:
   ```bash
   trunk serve
   ```

2. Open `http://localhost:8080` in your browser.

## Web-Specific Adaptations

### File Handling

The web version uses browser APIs for file operations:

```rust
#[cfg(target_arch = "wasm32")]
async fn load_file() -> Result<Vec<u8>, Error> {
    // Use file picker API
    let file_promise = web_sys::window()
        .unwrap()
        .fetch_with_str("/path/to/file");
    
    // Handle Promise and convert to Rust types
    let resp = wasm_bindgen_futures::JsFuture::from(file_promise).await?;
    let resp = resp.dyn_into::<web_sys::Response>()?;
    
    // Get the array buffer
    let array_buf = wasm_bindgen_futures::JsFuture::from(resp.array_buffer()?).await?;
    let array = js_sys::Uint8Array::new(&array_buf);
    
    // Convert to Rust Vec<u8>
    let mut result = vec![0; array.length() as usize];
    array.copy_to(&mut result);
    
    Ok(result)
}
```

### WebGPU Integration

Brush uses the browser's WebGPU implementation:

```rust
#[cfg(target_arch = "wasm32")]
async fn initialize_webgpu() -> Result<(), Error> {
    // Get WebGPU adapter
    let adapter = wgpu::Instance::new(wgpu::Backends::all())
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .ok_or(Error::AdapterNotFound)?;
    
    // Create device and queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Brush WebGPU Device"),
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        )
        .await?;
    
    // Store device and queue for later use
    // ...
    
    Ok(())
}
```

### Responsive Design

The web version adapts to different screen sizes:

```rust
fn handle_resize_event() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("brush-canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    
    // Get new size
    let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
    
    // Update canvas size
    canvas.set_width(width);
    canvas.set_height(height);
    
    // Update renderer
    // ...
}
```

## Mobile Web Support

The web version works on mobile browsers with WebGPU support and includes:

- Touch controls for camera manipulation
- Responsive layout for different screen sizes
- Performance optimizations for mobile GPUs

## Performance Considerations

### Memory Management

Web browsers have memory constraints, so Brush implements:

```rust
#[cfg(target_arch = "wasm32")]
fn check_memory_limits(required_bytes: usize) -> Result<(), Error> {
    // Simple check against browser memory limits
    if required_bytes > 1_000_000_000 {  // 1GB limit
        return Err(Error::MemoryLimitExceeded);
    }
    
    Ok(())
}
```

### Browser Feature Detection

Brush detects WebGPU support at runtime:

```rust
#[cfg(target_arch = "wasm32")]
async fn check_webgpu_support() -> bool {
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();
    
    // Check for GPU object
    if js_sys::Reflect::has(&navigator, &JsValue::from_str("gpu")).unwrap_or(false) {
        // Check if adapter can be requested
        let adapter = wgpu::Instance::new(wgpu::Backends::all())
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await;
            
        adapter.is_some()
    } else {
        false
    }
}
```

## Debugging

### Console Logging

Use the browser console for debugging:

```rust
#[cfg(target_arch = "wasm32")]
fn setup_logging() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    
    log::info!("Brush web logger initialized");
}
```

### Performance Monitoring

Use browser developer tools for performance analysis:

```rust
#[cfg(target_arch = "wasm32")]
fn log_performance_mark(name: &str) {
    let window = web_sys::window().unwrap();
    if let Some(performance) = window.performance() {
        let _ = performance.mark(name);
        log::debug!("Performance mark: {}", name);
    }
}
```

## Troubleshooting

### Common Issues

**Problem**: "WebGPU not supported" error.  
**Solution**: Use Chrome 113+ or enable WebGPU flags in your browser.

**Problem**: Poor performance.  
**Solution**: Check if hardware acceleration is enabled in your browser settings.

**Problem**: Out of memory errors.  
**Solution**: Try with smaller datasets or models.

## Related Documentation

- [Cross-Platform Framework](/docs/cross_platform_framework.md)
- [Web Platform Roadmap](/project/web_platform_roadmap.md) - Future plans for web support 