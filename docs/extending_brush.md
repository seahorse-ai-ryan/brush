# Extending Brush 🧩

This guide provides instructions on how to add new features or modules to Brush, making it easier for developers to contribute to the project or customize it for their own needs.

## 🔄 Understanding Brush's Architecture

Before extending Brush, it's important to understand its modular architecture. Brush is organized into several crates, each with a specific purpose:

- **brush-app**: Main application that provides the UI and ties everything together
- **brush-train**: Training pipeline for Gaussian splatting
- **brush-render**: Rendering pipeline for visualizing Gaussian splats
- **brush-cli**: Command-line interface for Brush
- **brush-dataset**: Data loading and processing utilities
- **brush-kernel**: Core computational kernels and algorithms
- **brush-ui**: UI components and utilities
- **brush-wgsl**: WebGPU Shading Language code and utilities
- **brush-rerun**: Integration with the Rerun visualization library

This modular design makes it easier to extend specific functionality without affecting the entire system.

## 🌟 Types of Extensions

### 1. Adding New Training Algorithms

To add a new training algorithm or modify the existing one:

1. Study the implementation in `brush-train` crate
2. Create a new module or extend the existing algorithms
3. Implement the required traits and interfaces
4. Register your new algorithm with the training pipeline
5. Add UI controls if necessary

```rust
// Example of extending the training pipeline with a new loss function
pub struct CustomLoss;

impl LossFunction for CustomLoss {
    fn compute_loss(&self, rendered: &Image, target: &Image) -> (Tensor, f32) {
        // Your custom loss computation here
    }
}

// Then register it with the training pipeline
let mut training_options = TrainingOptions::default();
training_options.loss_function = Box::new(CustomLoss);
```

### 2. Adding New Rendering Features

To enhance the rendering capabilities:

1. Explore the `brush-render` crate to understand the rendering pipeline
2. Identify where your new feature fits (shaders, post-processing, etc.)
3. Implement your changes, ensuring compatibility with the existing pipeline
4. Add any necessary UI controls to expose your feature

```rust
// Example of adding a new post-processing effect
pub struct CustomPostProcess;

impl PostProcessor for CustomPostProcess {
    fn process(&self, input: &TextureView, output: &TextureView, encoder: &mut CommandEncoder) {
        // Your custom post-processing logic
    }
}

// Then add it to the rendering pipeline
renderer.add_post_processor(Box::new(CustomPostProcess));
```

### 3. Supporting New Data Formats

To add support for a new dataset format:

1. Study the existing loaders in the `brush-dataset` crate
2. Create a new module implementing the required traits
3. Register your loader with the dataset loading system

```rust
// Example of adding a new dataset format loader
pub struct CustomFormatLoader;

impl DatasetLoader for CustomFormatLoader {
    fn can_load(&self, path: &Path) -> bool {
        // Check if this loader can handle the given path
    }
    
    fn load_dataset(&self, path: &Path) -> Result<Dataset> {
        // Load the dataset from the custom format
    }
}

// Then register it with the dataset loading system
DatasetRegistry::register(Box::new(CustomFormatLoader));
```

### 4. Extending the User Interface

To add new UI elements or features:

1. Explore the `brush-ui` and `brush-app` crates
2. Understand the egui-based UI system
3. Add your new UI components or extend existing ones

```rust
// Example of adding a new UI panel
impl Panel for CustomPanel {
    fn name(&self) -> &str {
        "Custom Features"
    }
    
    fn show(&mut self, ui: &mut Ui, app_state: &mut AppState) {
        ui.heading("Custom Controls");
        
        // Add your custom controls here
        if ui.button("Custom Action").clicked() {
            // Handle the action
        }
    }
}

// Then register it with the UI system
app.add_panel(Box::new(CustomPanel::default()));
```

## 🛠️ Development Workflow for Extensions

### Step 1: Set Up Local Development Environment

1. Fork the Brush repository
2. Clone your fork locally
3. Set up the development environment as described in the [Build Process](build_process.md) document
4. Create a new branch for your extension: `git checkout -b feature/my-extension`

### Step 2: Plan Your Extension

1. Clearly define what your extension will do
2. Identify which crates need to be modified
3. Plan how your extension will integrate with the existing codebase
4. Consider writing a brief design document for complex extensions

### Step 3: Implement Your Extension

1. Follow Rust best practices and match the existing code style
2. Ensure your code is well-documented with comments
3. Write tests for your new functionality
4. Use feature flags if your extension is optional:

```rust
#[cfg(feature = "my_extension")]
mod my_extension {
    // Your extension code here
}
```

### Step 4: Test Your Extension

1. Write unit tests for your functionality
2. Ensure existing tests still pass: `cargo test --all`
3. Test on different platforms if your extension affects cross-platform behavior
4. Run with the tracy profiler to ensure no performance regressions: `cargo run --release --feature=tracy`

### Step 5: Submit Your Extension

1. Create a pull request to the main Brush repository
2. Provide a clear description of your extension
3. Include screenshots or videos if your extension has visual components
4. Respond to reviewer feedback and make necessary changes

## 📊 Best Practices for Extensions

1. **Maintain Cross-Platform Compatibility**: Ensure your extension works across all supported platforms
2. **Performance Considerations**: Profile your code to ensure it doesn't negatively impact performance
3. **Memory Management**: Be mindful of GPU and system memory usage
4. **Error Handling**: Use proper error handling with descriptive messages
5. **Configuration Options**: Make your extension configurable when appropriate
6. **Documentation**: Document your extension thoroughly for other developers

## 🔍 Examples of Possible Extensions

1. **Enhanced Material System**: Add support for more complex materials beyond standard Gaussian attributes
2. **Custom Visualization Tools**: Create specialized visualization tools for specific use cases
3. **Integration with Other 3D Formats**: Add importers/exporters for common 3D formats
4. **Progressive Loading**: Implement level-of-detail or progressive loading for large models
5. **Semantic Segmentation**: Enhance the existing semantic capabilities with custom models or algorithms
6. **Cloud Integration**: Add support for cloud storage or processing
7. **Collaborative Features**: Implement real-time collaboration capabilities

## 🔧 Debugging Extensions

1. Use the Rerun visualization tool for debugging complex algorithms
2. Add logging at appropriate levels using the tracing macros
3. Use Rust's debug formatting for complex data structures: `println!("{:#?}", my_struct)`
4. For GPU issues, use the WebGPU validation layers where available

```rust
// Example of adding debug visualization to Rerun
if let Some(rr) = app_state.rerun.as_mut() {
    rr.log("custom_extension", &my_data)?;
}
```

## 🚀 Publishing Your Extension

If your extension is useful but not suitable for merging into the main Brush repository:

1. Publish it as a separate crate on crates.io
2. Document how it integrates with Brush
3. Provide examples of usage
4. Share it with the Brush community on Discord or GitHub Discussions 