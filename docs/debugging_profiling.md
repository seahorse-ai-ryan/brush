# Debugging and Profiling 🔍

This document provides techniques and tools for debugging and profiling Brush, helping contributors identify and resolve issues efficiently.

## Debugging Approaches 🐛

### Log-Based Debugging

Brush uses the `log` crate for logging, with different severity levels:

```rust
use log::{debug, error, info, trace, warn};

// Basic usage
info!("Processing dataset: {}", dataset_path);
debug!("Gaussian count: {}", gaussians.len());
warn!("Unusual parameter value: {}", value);
error!("Failed to load file: {}", e);
```

#### Log Level Configuration

Configure log levels in the application:

```rust
// In application initialization
env_logger::Builder::new()
    .filter_level(log::LevelFilter::Debug)
    .init();
```

Or via environment variables:

```bash
# On Linux/macOS
RUST_LOG=debug cargo run

# On Windows PowerShell
$env:RUST_LOG="debug"; cargo run
```

Log levels from most to least verbose:
- `trace`: Very detailed information
- `debug`: Useful for debugging
- `info`: General information
- `warn`: Warning conditions
- `error`: Error conditions

### Rust Debugging

#### Using `println!` Debugging

For quick debugging, you can use `println!` or `dbg!`:

```rust
// Simple print statement
println!("Value: {:?}", value);

// dbg! macro returns the value and prints file/line
let result = dbg!(compute_something(input));
```

#### Debugger Integration

For step-by-step debugging:

1. In VS Code, install the "CodeLLDB" extension
2. Create a `.vscode/launch.json` file (sample provided in the repo)
3. Set breakpoints in your code
4. Start debugging with F5

Common debugging operations:
- Step Over (F10): Execute current line and stop at next line
- Step Into (F11): Step into function calls
- Step Out (Shift+F11): Complete current function and return
- Continue (F5): Run until next breakpoint

### GPU Debugging

For GPU code issues:

#### Shader Debugging

1. Add debug prints in WGSL shaders:
   ```wgsl
   fn main() {
       // Only works in some browsers/native apps with appropriate flags
       dbg_print_i32(value);
   }
   ```

2. Monitor WebGPU validation errors in the console

3. Use buffer readbacks for validation:
   ```rust
   // Read back GPU buffer for debugging
   let buffer_slice = buffer.slice(..);
   let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
   buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
   device.poll(wgpu::Maintain::Wait);
   
   if let Ok(Ok(())) = receiver.receive().await {
       let data = buffer_slice.get_mapped_range();
       let result: Vec<u32> = bytemuck::cast_slice(&data).to_vec();
       drop(data);
       buffer.unmap();
       println!("Buffer contents: {:?}", result);
   }
   ```

#### RenderDoc Integration

For in-depth graphics debugging:

1. Install [RenderDoc](https://renderdoc.org/)
2. Launch your application through RenderDoc
3. Capture a frame
4. Analyze resources, shaders, and draw calls

### Runtime Error Debugging

For runtime errors and crashes:

1. Use Rust's error handling to provide detailed error information:
   ```rust
   use anyhow::{Context, Result};
   
   fn load_file(path: &str) -> Result<Data> {
       let content = std::fs::read(path)
           .with_context(|| format!("Failed to read file: {}", path))?;
       // Process content
       Ok(data)
   }
   ```

2. Add panic hooks for better crash reporting:
   ```rust
   std::panic::set_hook(Box::new(|panic_info| {
       let location = panic_info.location().unwrap();
       let msg = match panic_info.payload().downcast_ref::<&'static str>() {
           Some(s) => *s,
           None => match panic_info.payload().downcast_ref::<String>() {
               Some(s) => &s[..],
               None => "Unknown panic",
           },
       };
       log::error!("Panic at {}:{}: {}", location.file(), location.line(), msg);
   }));
   ```

## Profiling Tools 📊

### Tracy Profiler

Brush integrates with [Tracy](https://github.com/wolfpld/tracy), a real-time profiler:

1. Build with Tracy enabled:
   ```bash
   cargo run --release --features=tracy
   ```

2. Launch the Tracy profiler application

3. Connect to your running Brush instance

4. Analyze performance data:
   - Frame timings
   - CPU usage
   - GPU timing
   - Memory allocation

#### Adding Custom Tracy Zones

Add custom zones to profile specific code sections:

```rust
use tracing_tracy::tracy_zone;

fn expensive_operation() {
    let _zone = tracy_zone!("expensive_operation");
    // Code to profile
}
```

### Burn Profiling

Profile Burn tensor operations:

```rust
use burn::tensor::backend::Backend;

// Enable profiling for a backend
type MyBackend = burn_wgpu::Wgpu;
let device = <MyBackend as Backend>::Device::default();
device.set_profiling(true);

// After operations
let report = device.profiling_report();
println!("{}", report);
```

### WebGPU Timing

For GPU operations timing:

```rust
// Create timestamp query set
let query_set = device.create_query_set(&wgpu::QuerySetDescriptor {
    label: Some("timing_queries"),
    ty: wgpu::QueryType::Timestamp,
    count: 2, // Start and end timestamps
});

// In command encoder
let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
    label: Some("timing_encoder"),
});
encoder.write_timestamp(&query_set, 0);
// GPU operations here
encoder.write_timestamp(&query_set, 1);

// Read back query results
// ...

// Calculate elapsed time
let elapsed_ns = end_time - start_time;
```

### Memory Profiling

Monitor memory usage:

1. Use `TRACE=memory` environment variable to track Rust allocations
2. Track GPU memory allocation:
   ```rust
   // Create a buffer with debug info
   let buffer = device.create_buffer(&wgpu::BufferDescriptor {
       label: Some("Important_Buffer"),
       size: bytes,
       usage: wgpu::BufferUsages::STORAGE,
       mapped_at_creation: false,
   });
   ```
3. Use OS-specific tools like Windows Task Manager, macOS Activity Monitor, or Linux `top`/`htop`

## Visual Debugging with Rerun 🖼️

Brush integrates with [Rerun](https://github.com/rerun-io/rerun) for algorithm visualization:

1. Install Rerun:
   ```bash
   cargo install rerun-cli
   ```

2. Run Brush with Rerun logging:
   ```bash
   cargo run --release
   ```

3. Open the Rerun viewer:
   ```bash
   rerun brush_blueprint.rbl
   ```

4. Visualize data such as:
   - Gaussian positions and attributes
   - Training progress
   - Camera positions
   - Density distributions

### Adding Custom Rerun Logging

Add custom visualizations:

```rust
use rerun::RecordingStream;

let rec = RecordingStream::new("my_app");

// Log 3D points
rec.log_points("my_points", &points);

// Log images
rec.log_image("rendered_view", &image);

// Log tensors
rec.log_tensor("density_field", &tensor);
```

## Performance Optimization Tips 🚀

### CPU Optimization

1. **Parallelism**: Use Rust's `rayon` for CPU parallelism:
   ```rust
   use rayon::prelude::*;
   
   let results: Vec<_> = data.par_iter()
       .map(|item| process(item))
       .collect();
   ```

2. **Avoid Allocations**: Reuse buffers where possible:
   ```rust
   // Reuse vector instead of creating new ones
   let mut buffer = Vec::with_capacity(1000);
   for _ in 0..iterations {
       buffer.clear();
       fill_buffer(&mut buffer);
       process(&buffer);
   }
   ```

3. **Profile-guided optimization**: Use Rust's PGO for release builds:
   ```bash
   # Build with instrumentation
   RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" cargo build --release
   
   # Run the application for profile data collection
   ./target/release/brush-app
   
   # Build with collected profile data
   RUSTFLAGS="-Cprofile-use=/tmp/pgo-data" cargo build --release
   ```

### GPU Optimization

1. **Batch Processing**: Process data in batches to reduce API overhead

2. **Async Compute**: Overlap compute and graphics operations:
   ```rust
   // Create compute and render command encoders
   let mut compute_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
       label: Some("compute_encoder"),
   });
   let mut render_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
       label: Some("render_encoder"),
   });
   
   // Submit both for potential overlap
   queue.submit([
       compute_encoder.finish(),
       render_encoder.finish(),
   ]);
   ```

3. **Workgroup Optimization**: Fine-tune workgroup sizes for compute shaders:
   ```wgsl
   @compute @workgroup_size(64, 1, 1)  // Adjust for your GPU
   fn main() {
       // Shader code
   }
   ```

4. **Memory Access Patterns**: Optimize for coalesced memory access in shaders

## Common Issues and Solutions 🩹

### WebGPU Validation Errors

**Issue**: WebGPU operations failing with validation errors  
**Solution**: Check buffer sizes, bind group layouts, and shader compatibility

### Out of Memory

**Issue**: GPU out of memory errors  
**Solution**: Reduce batch sizes, optimize memory usage, or implement progressive processing

### Performance Degradation

**Issue**: Slow rendering or training  
**Solution**: Profile with Tracy, optimize hotspots, and use appropriate algorithms

### Cross-Platform Issues

**Issue**: Code works on one platform but fails on another  
**Solution**: Use platform-specific feature detection and adapt behavior accordingly

## Next Steps 🔍

- Learn about [Testing](testing.md) strategies
- Explore the [Training Module](training_module.md)
- Understand the [Rendering Module](rendering_module.md)

## References 📚

- [Tracy Profiler Documentation](https://github.com/wolfpld/tracy)
- [Rerun Documentation](https://www.rerun.io/docs)
- [RenderDoc Documentation](https://renderdoc.org/docs/index.html)
- [WebGPU Specification](https://www.w3.org/TR/webgpu/) 