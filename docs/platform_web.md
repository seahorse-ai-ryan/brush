# Web Platform Guide 🌐

This document provides detailed information about running Brush in web browsers using WebAssembly and WebGPU.

## Overview 🔍

Brush can run directly in modern web browsers without any installation, providing:

- Full 3D reconstruction capabilities
- Interactive visualization
- Training directly in the browser
- Dataset management
- Collaborative sharing

## Requirements 📋

### Browser Support

Brush requires:

- **Chrome/Chromium** 113+ (recommended: version 131+)
- **Edge** 113+ (Chromium-based)
- **Firefox** Nightly (experimental WebGPU support)
- **Safari** 17+ (experimental WebGPU support)

### WebGPU Support

Brush uses WebGPU for GPU acceleration:

- Ensure your browser has WebGPU enabled
- For Chrome, you may need to enable the flag: `chrome://flags/#enable-unsafe-webgpu`
- Check if your browser supports WebGPU at [webgpureport.org](https://webgpureport.org/)

### Hardware Requirements

- **GPU**: Any GPU with WebGPU support
- **RAM**: 4GB+ recommended (8GB+ for larger datasets)
- **CPU**: Modern multi-core CPU recommended

## Running Brush on the Web 🚀

### Online Demo

You can try Brush directly at:
- [https://arthurbrussee.github.io/brush-demo](https://arthurbrussee.github.io/brush-demo)

### Local Development Server

To run Brush locally on a development server:

1. Install Trunk:
   ```bash
   cargo install trunk
   ```

2. Navigate to the project directory and run:
   ```bash
   trunk serve
   ```

3. Open your browser and navigate to `http://localhost:8080`

## Web-Specific Features 🎯

### URL Parameters

Brush supports URL parameters for easy sharing and configuration:

| Parameter | Description | Example |
|-----------|-------------|---------|
| `url` | URL to a PLY file to load | `?url=https://example.com/model.ply` |
| `dataset` | URL to a zipped dataset for training | `?dataset=https://example.com/dataset.zip` |
| `background` | Background color (hex) | `?background=1a1a1a` |
| `view` | Initial camera view preset | `?view=front` |
| `demo` | Load a specific demo | `?demo=garden` |

Examples:
```
https://arthurbrussee.github.io/brush-demo?url=https://example.com/model.ply&background=1a1a1a
```

### File System Access

Brush uses several approaches to handle files in the browser:

1. **Browser File API**: For loading local files
2. **IndexedDB**: For persistent storage
3. **Remote URLs**: For loading external resources
4. **Zip Processing**: For working with packaged datasets

```rust
// Example of Web file system handling
#[cfg(target_arch = "wasm32")]
async fn load_file_web(source: FileSource) -> Result<Vec<u8>, Error> {
    match source {
        FileSource::Local => {
            // Use browser file picker
            let file_promise = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("file-input")
                .unwrap()
                .dyn_into::<web_sys::HtmlInputElement>()
                .unwrap()
                .files()
                .unwrap();
            
            // Process file
            // ...
        }
        FileSource::Remote(url) => {
            // Fetch from URL
            let fetch_promise = web_sys::window()
                .unwrap()
                .fetch_with_str(&url);
                
            // Process response
            // ...
        }
        FileSource::IndexedDb(key) => {
            // Load from IndexedDB
            // ...
        }
    }
}
```

### Memory Management

Web browsers have memory limitations that require special handling:

```rust
// Web-specific memory management
#[cfg(target_arch = "wasm32")]
fn check_memory_availability(required_bytes: usize) -> Result<(), Error> {
    // Check current memory usage and available memory
    let memory_info = web_sys::window()
        .unwrap()
        .performance()
        .unwrap()
        .memory()
        .unwrap();
    
    let used_js_heap_size = memory_info.used_js_heap_size();
    let js_heap_size_limit = memory_info.js_heap_size_limit();
    
    // Ensure enough memory is available
    if used_js_heap_size + required_bytes as f64 > js_heap_size_limit {
        return Err(Error::InsufficientMemory);
    }
    
    Ok(())
}
```

## Building for the Web 🏗️

### Build Process

Brush uses Trunk for building the web version:

1. **Compilation**: Rust code is compiled to WebAssembly
2. **Asset Processing**: Static assets are processed and included
3. **HTML Generation**: Entry point HTML is generated
4. **Bundling**: All resources are bundled together

```
┌───────────────┐     ┌───────────────┐     ┌───────────────┐
│  Rust Source  │────►│  WASM Binary  │────►│    Bundled    │
│     Code      │     │               │     │    Assets     │
└───────────────┘     └───────────────┘     └───────────────┘
       │                                            ▲
       │                                            │
       ▼                                            │
┌───────────────┐     ┌───────────────┐     ┌───────────────┐
│  Static       │────►│  Processed    │────►│    Output     │
│  Assets       │     │  Resources    │     │    Directory  │
└───────────────┘     └───────────────┘     └───────────────┘
```

### Building with Trunk

```bash
# Development build with auto-reload
trunk serve

# Production build
trunk build --release

# Build with specific features
trunk build --release --features="web-performance-monitoring"
```

### Optimizing the Build

For optimal web performance:

1. **Code Size**: Enable Link Time Optimization (LTO) in `Cargo.toml`:
   ```toml
   [profile.release]
   lto = true
   ```

2. **Wasm-Opt**: Trunk can apply wasm-opt optimizations:
   ```toml
   # Trunk.toml
   [build]
   # Optimize wasm output for smaller size
   release = true
   
   # Enable wasm-opt
   [build.wasm-opt]
   level = 3
   ```

3. **Treeshaking**: Remove unused code:
   ```toml
   [package.metadata.wasm-pack.profile.release]
   wasm-opt = ['-Os']
   ```

## Web-Specific Implementation Details 🔧

### WebGPU Initialization

```rust
// Initialize WebGPU in the browser
async fn initialize_webgpu() -> Result<(wgpu::Device, wgpu::Queue), Error> {
    // Create instance
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });
    
    // Request adapter
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .ok_or(Error::NoWebGPUAdapter)?;
    
    // Create device and queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .map_err(|_| Error::WebGPUDeviceError)?;
    
    Ok((device, queue))
}
```

### Canvas Integration

```rust
// Setup WebGPU canvas in browser
fn setup_canvas() -> Result<web_sys::HtmlCanvasElement, Error> {
    let window = web_sys::window().ok_or(Error::NoWindow)?;
    let document = window.document().ok_or(Error::NoDocument)?;
    
    let canvas = document
        .get_element_by_id("brush-canvas")
        .ok_or(Error::NoCanvas)?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| Error::InvalidCanvas)?;
    
    // Set canvas size to window size
    let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
    
    canvas.set_width(width);
    canvas.set_height(height);
    
    Ok(canvas)
}
```

### Web Worker Integration

Brush uses Web Workers for background processing:

```rust
// Setup web worker for background processing
fn setup_web_worker() -> Result<web_sys::Worker, Error> {
    let worker = web_sys::Worker::new("worker.js")
        .map_err(|_| Error::WebWorkerCreationFailed)?;
    
    // Set up message handler
    let callback = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
        // Handle messages from worker
        let data = event.data();
        // Process data
        // ...
    }) as Box<dyn FnMut(_)>);
    
    worker.set_onmessage(Some(callback.as_ref().unchecked_ref()));
    callback.forget();
    
    Ok(worker)
}
```

## Performance Considerations ⚡

### Optimizing for Web Performance

1. **Asynchronous Loading**:
   - Load resources incrementally
   - Show loading progress indicators
   - Prioritize critical resources

2. **Memory Management**:
   - Dispose of unused resources promptly
   - Use streaming for large datasets
   - Monitor memory usage with `performance.memory`

3. **WebGPU Best Practices**:
   - Batch draw calls
   - Minimize buffer uploads
   - Use compute shaders efficiently

4. **Progressive Enhancement**:
   - Detect device capabilities
   - Adjust quality settings automatically
   - Provide fallbacks for unsupported features

## Web-Specific Debugging 🐛

### Browser Developer Tools

- **Chrome DevTools**: Use the Performance and Memory tabs
- **WebGPU Capture**: In Chrome 113+, enable "WebGPU Developer Features"
- **Console Logging**: Use `console.log`, `console.warn`, etc.

### Web-Specific Logging

```rust
// Web logging setup
#[cfg(target_arch = "wasm32")]
fn setup_logging() {
    // Initialize wasm-logger
    wasm_logger::init(wasm_logger::Config::default());
    
    // Set up panic hook
    console_error_panic_hook::set_once();
    
    // Log platform information
    log::info!("Web platform initialized");
    log::info!("Browser: {}", get_browser_info());
    log::info!("WebGPU support: {}", has_webgpu_support());
}
```

### Performance Monitoring

```rust
// Web performance monitoring
#[cfg(target_arch = "wasm32")]
fn track_performance(name: &str) -> PerformanceMarker {
    let performance = web_sys::window()
        .unwrap()
        .performance()
        .unwrap();
    
    let start_time = performance.now();
    let mark_name = format!("start-{}", name);
    
    // Mark the beginning of the operation
    performance.mark(&mark_name);
    
    PerformanceMarker {
        name: name.to_string(),
        start_time,
    }
}

struct PerformanceMarker {
    name: String,
    start_time: f64,
}

impl Drop for PerformanceMarker {
    fn drop(&mut self) {
        let performance = web_sys::window()
            .unwrap()
            .performance()
            .unwrap();
        
        let end_time = performance.now();
        let duration = end_time - self.start_time;
        
        log::info!("{} took {} ms", self.name, duration);
        
        // Record the measure
        let mark_name = format!("end-{}", self.name);
        performance.mark(&mark_name);
        performance.measure(&self.name, &format!("start-{}", self.name), &mark_name);
    }
}
```

## Web Deployment 🚀

### Hosting Options

Brush's web build can be hosted on:

1. **GitHub Pages**: Ideal for project demos
2. **Netlify/Vercel**: Easy deployment with CI/CD
3. **AWS S3/CloudFront**: Scalable hosting
4. **Custom Web Server**: For integrating with backend services

### GitHub Pages Deployment

```bash
# Build for GitHub Pages
trunk build --release --public-url "/brush-demo/"

# Copy to docs folder for GitHub Pages
cp -r dist/* docs/
```

### Required Server Configuration

For proper functioning, your web server should:

1. Serve the correct MIME types:
   ```
   .wasm -> application/wasm
   .js   -> application/javascript
   ```

2. Set CORS headers for loading resources:
   ```
   Access-Control-Allow-Origin: *
   ```

3. Enable compression for .wasm files:
   ```
   Content-Encoding: gzip
   ```

## Limitations and Workarounds 🚧

### Browser Limitations

| Limitation | Workaround |
|------------|------------|
| Memory limits | Use streaming processing, lazy loading |
| File system access | Use File System Access API or file pickers |
| Performance overhead | Optimize WebAssembly and GPU usage |
| Browser compatibility | Feature detection, fallbacks |
| Large file uploads | Chunked uploads, compression |

### Browser Feature Detection

```rust
// Detect browser features
#[cfg(target_arch = "wasm32")]
fn detect_browser_features() -> BrowserFeatures {
    let window = web_sys::window().unwrap();
    
    // Check for WebGPU support
    let webgpu_supported = js_sys::Reflect::has(
        &window,
        &JsValue::from_str("navigator.gpu"),
    ).unwrap_or(false);
    
    // Check for File System Access API
    let fs_access_api = js_sys::Reflect::has(
        &window,
        &JsValue::from_str("showOpenFilePicker"),
    ).unwrap_or(false);
    
    // Check for SharedArrayBuffer support
    let shared_array_buffer = js_sys::Reflect::has(
        &window,
        &JsValue::from_str("SharedArrayBuffer"),
    ).unwrap_or(false);
    
    BrowserFeatures {
        webgpu_supported,
        fs_access_api,
        shared_array_buffer,
    }
}
```

## Examples and Recipes 📚

### Loading a Model from URL

```javascript
// JavaScript side
const loadModelFromUrl = async (url) => {
  const response = await fetch(url);
  const arrayBuffer = await response.arrayBuffer();
  
  // Pass to Rust/WASM
  window.wasmInstance.load_model_from_buffer(new Uint8Array(arrayBuffer));
};
```

```rust
// Rust/WASM side
#[wasm_bindgen]
pub fn load_model_from_buffer(buffer: &[u8]) -> Result<(), JsValue> {
    // Parse PLY file
    let gaussians = parse_ply_buffer(buffer)
        .map_err(|e| JsValue::from_str(&format!("Error parsing PLY: {}", e)))?;
    
    // Load into renderer
    RENDERER.with(|renderer| {
        let mut renderer = renderer.borrow_mut();
        renderer.load_gaussians(gaussians);
    });
    
    Ok(())
}
```

### Training in the Browser

```rust
// Web-specific training configuration
#[cfg(target_arch = "wasm32")]
fn configure_web_training() -> TrainingConfig {
    // Check available memory and compute capability
    let memory_info = web_sys::window()
        .unwrap()
        .performance()
        .unwrap()
        .memory()
        .unwrap();
    
    let available_memory = memory_info.js_heap_size_limit() - memory_info.used_js_heap_size();
    
    // Adjust training parameters based on browser capabilities
    let batch_size = if available_memory > 2_000_000_000 { 4 } else { 2 };
    let resolution_scale = if available_memory > 1_000_000_000 { 1.0 } else { 0.5 };
    
    TrainingConfig {
        batch_size,
        resolution_scale,
        // Other parameters...
    }
}
```

## Next Steps 🔍

- Explore the [CLI](cli.md) capabilities
- Learn about [Performance Optimization](performance_optimization.md)
- Understand the [Cross-Platform Framework](cross_platform_framework.md) 