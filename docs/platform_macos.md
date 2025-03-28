# macOS Platform 🍎

This document provides information about building, running, and optimizing Brush on macOS.

## System Requirements

### Hardware Requirements

- **Operating System**: macOS 10.14 (Mojave) or newer
- **CPU**: Intel or Apple Silicon
- **GPU**: Any Metal-compatible GPU
- **RAM**: 8GB minimum, 16GB recommended for larger datasets

### Development Requirements

- **Rust**: 1.70 or newer
- **Xcode Command Line Tools**
- **Metal SDK**

## Building on macOS

### Development Environment Setup

1. Install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install Xcode Command Line Tools:
   ```bash
   xcode-select --install
   ```

3. Clone the repository:
   ```bash
   git clone https://github.com/ArthurBrussee/brush.git
   cd brush
   ```

### Building

Build Brush with Cargo:

```bash
cargo build --release
```

For Apple Silicon Macs:

```bash
cargo build --release --target aarch64-apple-darwin
```

For Intel Macs:

```bash
cargo build --release --target x86_64-apple-darwin
```

## Metal Integration

Brush uses Metal for GPU acceleration on macOS:

```rust
#[cfg(target_os = "macos")]
fn initialize_metal(window: &Window) -> Result<RenderContext, Error> {
    // Create Metal surface
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::METAL,
        dx12_shader_compiler: Default::default(),
    });
    
    let surface = unsafe { instance.create_surface(window) }?;
    
    // Request adapter
    let adapter = instance.request_adapter(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        },
    ).await?;
    
    // Create device and queue
    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: Some("Metal Device"),
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
        },
        None,
    ).await?;
    
    Ok(RenderContext {
        instance,
        surface,
        adapter,
        device,
        queue,
    })
}
```

## macOS File System Integration

Brush integrates with macOS file system conventions:

```rust
#[cfg(target_os = "macos")]
fn get_app_data_directory() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "~".to_string());
    let path = PathBuf::from(home)
        .join("Library")
        .join("Application Support")
        .join("Brush");
    
    if !path.exists() {
        std::fs::create_dir_all(&path).ok();
    }
    
    path
}
```

## Performance Optimization

### Basic Metal Optimizations

```rust
#[cfg(target_os = "macos")]
fn optimize_metal_pipeline(device: &wgpu::Device, pipeline: &mut RenderPipeline) {
    // Configure Metal-specific settings for better performance
    if let Some(pipeline_layout) = pipeline.get_pipeline_layout() {
        pipeline_layout.set_vertex_buffer_layout(0, wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Normal
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // TexCoord
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        });
    }
}
```

### CPU Architecture Detection

Brush can detect the CPU architecture to optimize for Intel or Apple Silicon:

```rust
#[cfg(target_os = "macos")]
fn detect_architecture() -> Architecture {
    #[cfg(target_arch = "aarch64")]
    {
        return Architecture::AppleSilicon;
    }
    
    #[cfg(target_arch = "x86_64")]
    {
        return Architecture::Intel;
    }
    
    Architecture::Unknown
}
```

## Debugging

### Metal Debug Tools

Use Xcode's Metal debugging features:

1. Enable GPU Frame Capture in Xcode
2. Attach to Brush process
3. Capture Metal frames
4. Analyze shader performance

### Console Logging

```rust
#[cfg(target_os = "macos")]
fn setup_logging() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();
    
    log::info!("macOS logger initialized");
}
```

## Troubleshooting

### Common Issues

**Problem**: "Metal device not found" error.  
**Solution**: Verify macOS version is 10.14+ and check for Metal-compatible GPU.

**Problem**: Slow performance.  
**Solution**: Update GPU drivers and disable background applications.

**Problem**: Build errors.  
**Solution**: Ensure Xcode Command Line Tools are installed.

## Related Documentation

- [Cross-Platform Framework](/docs/cross_platform_framework.md)
- [macOS Platform Roadmap](/project/macos_platform_roadmap.md) - Future Apple Silicon optimizations 