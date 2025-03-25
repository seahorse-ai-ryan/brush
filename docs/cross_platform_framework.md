# Cross-Platform Framework 🌍

This document provides detailed information about Brush's cross-platform architecture, which enables it to run on multiple operating systems and hardware configurations.

## Overview 🔍

Brush is designed to run on a wide range of platforms, including:

- **Desktop**: Windows, macOS, Linux
- **Web**: Modern browsers via WebAssembly
- **Mobile**: Android and iOS

This cross-platform capability is achieved through a carefully designed architecture that separates platform-specific code from core functionality.

## Architecture 🏗️

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          Platform-Agnostic Core                              │
│                                                                             │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌───────┐  │
│  │            │  │            │  │            │  │            │  │       │  │
│  │  Gaussian  │  │ Rendering  │  │  Training  │  │    Data    │  │  UI   │  │
│  │ Algorithms │  │   Core     │  │  Engine    │  │  Handling  │  │ Logic │  │
│  │            │  │            │  │            │  │            │  │       │  │
│  └────────────┘  └────────────┘  └────────────┘  └────────────┘  └───────┘  │
│                                                                             │
└───────────────────────────────────┬─────────────────────────────────────────┘
                                    │
                                    ▼
┌───────────────────────────────────────────────────────────────────────────────┐
│                                  WebGPU API                                    │
│                                  (via wgpu)                                    │
└───────────────────────────────────┬───────────────────────────────────────────┘
                                    │
                                    ▼
┌────────────────┬────────────────┬────────────────┬────────────────┬────────────────┐
│                │                │                │                │                │
│   Windows      │     macOS      │     Linux      │   WebAssembly  │     Mobile     │
│   Backend      │    Backend     │    Backend     │    Backend     │    Backend     │
│                │                │                │                │                │
└────────────────┴────────────────┴────────────────┴────────────────┴────────────────┘
```

### Key Components 🧩

1. **Platform-Agnostic Core**: Contains all the core algorithms and logic
2. **WebGPU API Layer**: Provides a unified graphics API across platforms
3. **Platform-Specific Backends**: Handle platform-specific implementations

## Technology Stack 🛠️

### Core Technologies

- **Rust**: The primary programming language, chosen for its performance, safety, and cross-platform support
- **wgpu**: WebGPU implementation in Rust that works across platforms
- **eframe/egui**: Cross-platform UI framework
- **Burn**: Cross-platform machine learning framework

### Platform Bridge Technologies

- **WebAssembly/WASM**: For web platform support
- **cargo-apk**: For Android packaging
- **Trunk**: For web builds
- **Android NDK**: For Android native code
- **iOS SDK**: For iOS support

## Cross-Platform Strategy 🔄

### Code Organization

The codebase is organized to maximize code sharing while isolating platform-specific code:

```
brush/
├── crates/
│   ├── brush-app/            # Main application logic
│   ├── brush-render/         # Cross-platform rendering
│   ├── brush-train/          # Cross-platform training
│   ├── brush-cli/            # Command line interface
│   ├── brush-android/        # Android-specific code
│   └── ... other modules
├── src/                      # Shared code
└── platform/                 # Platform-specific implementations
```

### Conditional Compilation

Rust's conditional compilation features are used extensively:

```rust
// Platform-specific code example
#[cfg(target_os = "windows")]
fn get_platform_data_directory() -> PathBuf {
    // Windows-specific implementation
}

#[cfg(target_os = "macos")]
fn get_platform_data_directory() -> PathBuf {
    // macOS-specific implementation
}

#[cfg(target_os = "linux")]
fn get_platform_data_directory() -> PathBuf {
    // Linux-specific implementation
}

#[cfg(target_arch = "wasm32")]
fn get_platform_data_directory() -> PathBuf {
    // Web-specific implementation
}
```

### Abstraction Layers

The codebase uses several abstraction layers to handle platform differences:

1. **Graphics Abstraction**: wgpu provides a WebGPU API that works on all platforms
2. **UI Abstraction**: egui/eframe provides a UI layer that works across platforms
3. **File System Abstraction**: Custom abstractions handle file system differences
4. **Input Handling**: Platform-specific input is normalized to a common format

## Platform-Specific Considerations 🖥️

### Desktop Platforms

#### Windows

- Uses DirectX 12 or Vulkan through wgpu
- Native window handling via winit
- Supports Windows 10 and newer

#### macOS

- Uses Metal through wgpu
- Native window handling via winit
- Supports macOS 10.14 (Mojave) and newer

#### Linux

- Uses Vulkan or OpenGL through wgpu
- Supports X11 and Wayland
- Tested on Ubuntu, Fedora, and other major distributions

### Web Platform

- Compiles to WebAssembly (WASM)
- Uses WebGPU via browser implementation
- Asset loading adapted for web environment
- Uses browser's IndexedDB for persistent storage
- Handles browser-specific input mechanisms

```rust
// Web-specific initialization example
#[cfg(target_arch = "wasm32")]
fn initialize_web_environment() {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();
    
    // Initialize WebGPU logging
    wasm_logger::init(wasm_logger::Config::default());
    
    // Set up WebAssembly memory handling
    // ...
}
```

### Mobile Platforms

#### Android

- Uses Vulkan through wgpu
- Activity lifecycle management
- Touch input handling
- Android-specific storage access
- APK packaging and resources management

```rust
// Android-specific app entry point
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn android_main(app: android_app) {
    // Android-specific initialization
    // ...
    
    // Run the application
    run_app(app);
}
```

#### iOS

- Uses Metal through wgpu
- iOS application lifecycle management
- Touch input and gesture recognition
- iOS-specific storage and permissions

## Shared GPU Pipeline 🎮

One of the core strengths of Brush is its unified GPU pipeline:

1. **Shader Code**: WGSL shaders work across all supported platforms
2. **Render Pipeline**: Same rendering code on all platforms
3. **Compute Pipeline**: Same GPU compute operations across platforms

```
┌─────────────────────────────────────────────────────────────────┐
│                     Shared GPU Pipeline                          │
│                                                                 │
│  ┌───────────┐   ┌────────────┐   ┌──────────┐   ┌───────────┐  │
│  │           │   │            │   │          │   │           │  │
│  │  Shaders  │──►│  Pipelines │──►│  Buffers │──►│  Texture  │  │
│  │  (WGSL)   │   │            │   │          │   │  Output   │  │
│  │           │   │            │   │          │   │           │  │
│  └───────────┘   └────────────┘   └──────────┘   └───────────┘  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐
│             │  │             │  │             │  │             │
│   DirectX   │  │    Metal    │  │   Vulkan    │  │   WebGPU    │
│             │  │             │  │             │  │   Browser   │
│             │  │             │  │             │  │             │
└─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘
```

### WGSL Shaders

WGSL (WebGPU Shading Language) is used across all platforms:

```wgsl
// Shared WGSL shader example that works on all platforms
@group(0) @binding(0) var<storage, read> u_gaussians: array<GaussianData>;
@group(0) @binding(1) var<storage, read_write> u_output: texture_storage_2d<rgba8unorm, write>;

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // Shader logic that works across all platforms
    // ...
}
```

## Cross-Platform UI 🖼️

Brush uses egui/eframe for consistent UI across platforms:

```rust
// Cross-platform UI code example
impl eframe::App for BrushApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            // Menu bar that works on all platforms
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        // Platform-specific file dialog is abstracted away
                        self.open_file_dialog();
                    }
                    // More menu items...
                });
                // More menus...
            });
        });
        
        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            // UI that adapts to platform screen size and input methods
            // ...
        });
    }
}
```

## Asset Management 📦

Assets and resources are handled differently across platforms:

```rust
// Cross-platform asset loading
pub fn load_asset(path: &str) -> Result<Vec<u8>, Error> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Desktop/mobile asset loading
        let full_path = get_asset_path(path);
        std::fs::read(full_path).map_err(Error::IoError)
    }
    
    #[cfg(target_arch = "wasm32")]
    {
        // Web asset loading
        let fetch_promise = web_sys::window()
            .unwrap()
            .fetch_with_str(path);
        
        // Async handling for web
        // ...
    }
}
```

## Performance Considerations ⚡

### Platform-Specific Optimizations

```rust
// Platform-specific performance optimizations
#[cfg(target_os = "windows")]
const WORKGROUP_SIZE: u32 = this_workgroup_size_best_for_windows;

#[cfg(target_os = "macos")]
const WORKGROUP_SIZE: u32 = this_workgroup_size_best_for_metal;

#[cfg(target_arch = "wasm32")]
const WORKGROUP_SIZE: u32 = this_workgroup_size_best_for_web;
```

### Memory Management

```rust
// Platform-specific memory management
#[cfg(not(target_arch = "wasm32"))]
fn allocate_large_buffer(size: usize) -> Vec<u8> {
    // Native platforms can allocate large amounts of memory
    Vec::with_capacity(size)
}

#[cfg(target_arch = "wasm32")]
fn allocate_large_buffer(size: usize) -> Result<Vec<u8>, Error> {
    // Web platform needs to check memory limits
    if size > WEB_MEMORY_LIMIT {
        // Use chunked processing instead
        return Err(Error::MemoryLimitExceeded);
    }
    Ok(Vec::with_capacity(size))
}
```

## Cross-Platform Testing 🧪

Brush employs a comprehensive testing strategy to ensure consistent behavior across platforms:

1. **Unit Tests**: Run on all platforms to verify core functionality
2. **Platform Tests**: Test platform-specific features
3. **Integration Tests**: Verify complete workflows across platforms
4. **CI/CD**: Automated testing on multiple platforms

```rust
// Cross-platform test example
#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn test_gaussian_rendering() {
    // Test that works on all platforms
    // ...
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn test_large_dataset_loading() {
    // Test only for native platforms
    // ...
}
```

## Platform-Specific Building and Deployment 📤

### Desktop Build Commands

```bash
# Windows
cargo build --release --target x86_64-pc-windows-msvc

# macOS
cargo build --release --target x86_64-apple-darwin

# Linux
cargo build --release --target x86_64-unknown-linux-gnu
```

### Web Build Process

```bash
# Install Trunk
cargo install trunk

# Build for web
trunk build --release

# The resulting build will be in the ./dist directory
```

### Android Build Process

```bash
# Install Cargo APK
cargo install cargo-apk

# Build APK
cd crates/brush-android
cargo apk build --release

# The resulting APK will be in target/release/apk/
```

## Common Challenges and Solutions 🤔

### Challenge: File System Access

**Solution**: Abstract file system operations through a common interface that adapts to each platform's specific file system access mechanisms.

```rust
pub trait FileSystem {
    fn read_file(&self, path: &str) -> Result<Vec<u8>, Error>;
    fn write_file(&self, path: &str, data: &[u8]) -> Result<(), Error>;
    fn list_directory(&self, path: &str) -> Result<Vec<String>, Error>;
    // Other file system operations...
}

// Platform-specific implementations
struct DesktopFileSystem;
struct WebFileSystem;
struct MobileFileSystem;
```

### Challenge: Input Handling

**Solution**: Normalize all input events to a common format before processing them in the core application logic.

```rust
// Common input event representation
pub enum InputEvent {
    MouseMove { x: f32, y: f32 },
    MouseButton { button: MouseButton, pressed: bool },
    Touch { id: u64, x: f32, y: f32, phase: TouchPhase },
    Key { key: KeyCode, pressed: bool },
    // Other input events...
}

// Platform-specific event conversion functions
#[cfg(not(target_arch = "wasm32"))]
fn convert_winit_event(event: winit::event::Event) -> Option<InputEvent> {
    // Convert winit events to InputEvent
    // ...
}

#[cfg(target_arch = "wasm32")]
fn convert_web_event(event: web_sys::Event) -> Option<InputEvent> {
    // Convert web events to InputEvent
    // ...
}
```

### Challenge: GPU Feature Detection

**Solution**: Detect available GPU features at runtime and adapt the rendering pipeline accordingly.

```rust
// Feature detection and adaptation
fn create_render_pipeline(device: &wgpu::Device) -> wgpu::RenderPipeline {
    // Check for platform support of features
    let supports_storage_textures = device
        .features()
        .contains(wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES);
    
    // Create appropriate pipeline based on supported features
    if supports_storage_textures {
        // Create optimal pipeline
        // ...
    } else {
        // Create fallback pipeline
        // ...
    }
}
```

## Future Platform Support 🚀

Brush's architecture is designed to easily extend to new platforms:

1. **WebGPU Native**: As WebGPU becomes available natively on more platforms
2. **Game Consoles**: Potential support for game consoles with WebGPU-compatible APIs
3. **VR/AR Platforms**: Integration with virtual and augmented reality frameworks

## Next Steps 🔍

- Explore platform-specific guides:
  - [Windows](platform_windows.md)
  - [macOS](platform_macos.md)
  - [Linux](platform_linux.md)
  - [Web](platform_web.md)
  - [Android](platform_android.md)
  - [iOS](platform_ios.md)
- Learn about [Performance Optimization](performance_optimization.md) techniques 