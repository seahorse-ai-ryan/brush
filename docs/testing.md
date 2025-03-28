# Testing 🧪

This document outlines the testing strategy for Brush, including different types of tests, how to run them, and best practices for writing new tests.

## Testing Overview 🔎

Brush employs several testing approaches to ensure code quality:

- **Unit Tests**: Test individual functions and components
- **Integration Tests**: Test interactions between components
- **Visual Tests**: Verify rendered outputs
- **Regression Tests**: Ensure fixes don't reintroduce bugs
- **Cross-Platform Tests**: Verify functionality across platforms

## Running Tests 🏃

### Running All Tests

To run all tests in the workspace:

```bash
cargo test --all
```

### Running Tests for a Specific Crate

To test a specific crate:

```bash
cargo test -p brush-render
```

### Running a Specific Test

To run a specific test:

```bash
cargo test -p brush-render test_gaussian_projection
```

### Running Tests with Features

To run tests with specific features:

```bash
cargo test --all --features=tracy
```

## Test Organization 📁

Brush organizes tests in several ways:

### 1. Module-Level Tests

Tests within the same file as the code being tested:

```rust
// In file: src/gaussian_splats.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gaussian_creation() {
        // Test code here
    }
}
```

### 2. Test Modules

Dedicated test modules in a crate:

```
brush-render/
├── src/
│   ├── lib.rs
│   ├── render.rs
│   └── tests/            <- Test directory
│       ├── mod.rs
│       ├── render_tests.rs
│       └── util.rs       <- Test utilities
```

### 3. Test Cases

Data-driven test cases in dedicated directories:

```
brush-train/
├── src/
│   └── ...
└── test_cases/           <- Test data
    ├── simple_scene/
    │   ├── images/
    │   └── transforms.json
    └── complex_scene/
        ├── images/
        └── transforms.json
```

## Types of Tests 🧩

### Unit Tests

Unit tests focus on individual functions or components:

```rust
#[test]
fn test_quaternion_rotation() {
    let q = Quaternion::from_axis_angle(Vec3::unit_x(), std::f32::consts::PI / 2.0);
    let v = Vec3::unit_y();
    let result = q.rotate_vec3(v);
    
    assert_approx_eq!(result.x, 0.0, 1e-6);
    assert_approx_eq!(result.y, 0.0, 1e-6);
    assert_approx_eq!(result.z, 1.0, 1e-6);
}
```

### Integration Tests

Integration tests verify component interactions:

```rust
#[test]
fn test_end_to_end_pipeline() {
    // Setup
    let device = create_test_device();
    let mut renderer = GaussianRenderer::new(&device);
    let mut scene = Scene::new();
    
    // Add test gaussians
    scene.add_gaussian(Gaussian::new(/* params */));
    
    // Render
    let result = renderer.render(&scene, Camera::default());
    
    // Verify output
    assert_eq!(result.width, 640);
    assert_eq!(result.height, 480);
    // Verify pixel values
}
```

### GPU Tests

Tests for GPU operations require special handling:

```rust
#[test]
fn test_gpu_sorting() {
    // Skip if no GPU available
    if !has_gpu_device() {
        return;
    }
    
    let device = create_test_device();
    
    // Create test data
    let values = vec![5, 3, 8, 1, 2];
    
    // Create GPU buffer and upload data
    // ... (GPU buffer setup code)
    
    // Run GPU sort
    sort_on_gpu(&device, &buffer);
    
    // Read back results
    let result = read_buffer(&device, &buffer);
    
    // Verify
    assert_eq!(result, vec![1, 2, 3, 5, 8]);
}
```

### Visual Tests

Tests for rendering correctness:

```rust
#[test]
fn test_gaussian_rendering() {
    // Set up scene with known Gaussians
    let scene = create_test_scene();
    
    // Render to image
    let image = render_scene(&scene);
    
    // Compare with reference image
    let reference = load_reference_image("test_gaussian_reference.png");
    let similarity = compute_image_similarity(&image, &reference);
    
    assert!(similarity > 0.95, "Rendered image differs from reference");
}
```

## Test Utilities 🧰

Brush provides several utilities to simplify testing:

### Test Fixtures

```rust
// src/tests/fixtures.rs
pub fn create_test_device() -> Device {
    // Create a test device with appropriate configuration
}

pub fn create_test_scene() -> Scene {
    // Create a scene with predefined test data
}
```

### Assertion Helpers

```rust
// For approximate floating point comparisons
pub fn assert_approx_eq(a: f32, b: f32, epsilon: f32) {
    assert!((a - b).abs() < epsilon, 
            "Values not approximately equal: {} vs {}", a, b);
}

// For image comparisons
pub fn assert_images_similar(a: &Image, b: &Image, threshold: f32) {
    let similarity = compute_image_similarity(a, b);
    assert!(similarity >= threshold, 
           "Images differ by more than allowed threshold");
}
```

## Testing Best Practices 📝

### Writing Effective Tests

1. **Test One Thing**: Each test should focus on one specific behavior
2. **Arrange-Act-Assert**: Structure tests with setup, action, and verification
3. **Descriptive Names**: Use clear test names that describe what is being tested
4. **Independent Tests**: Tests should not depend on each other
5. **Avoid Flakiness**: Tests should produce consistent results

### Example Test Structure

```rust
#[test]
fn test_gaussian_splatting_with_large_scale() {
    // Arrange: Setup test data
    let camera = Camera::default();
    let gaussian = Gaussian::new()
        .with_position(Vec3::new(0.0, 0.0, -5.0))
        .with_scale(Vec3::new(2.0, 2.0, 2.0));
    
    // Act: Perform the operation being tested
    let projected = project_gaussian(&gaussian, &camera);
    
    // Assert: Verify the results
    assert!(projected.area > 100.0, "Large gaussian should project to large area");
    assert_approx_eq!(projected.depth, 5.0, 0.001);
}
```

## Automated Testing 🤖

Brush uses GitHub Actions for continuous integration:

```yaml
# Example GitHub Actions workflow (simplified)
name: Tests

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    
    steps:
    - uses: actions/checkout@v2
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Run tests
      run: cargo test --all
```

## Test Coverage 📊

To measure test coverage:

1. Install cargo-tarpaulin:
   ```bash
   cargo install cargo-tarpaulin
   ```

2. Run coverage analysis:
   ```bash
   cargo tarpaulin --out Html --output-dir coverage
   ```

3. View the coverage report in `coverage/tarpaulin-report.html`

## Testing GPU Code 🎮

Testing GPU code has special considerations:

1. **Skip tests when appropriate**:
   ```rust
   #[test]
   fn test_gpu_feature() {
       if !has_gpu_support() {
           println!("Skipping GPU test due to lack of support");
           return;
       }
       // Test code
   }
   ```

2. **Use headless rendering** for CI environments:
   ```rust
   let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
       backends: wgpu::Backends::all(),
       dx12_shader_compiler: Default::default(),
   });
   
   // Try finding an adapter that doesn't require a display
   let adapter = instance
       .request_adapter(&wgpu::RequestAdapterOptions {
           power_preference: wgpu::PowerPreference::default(),
           compatible_surface: None, // Headless
           force_fallback_adapter: false,
       })
       .await
       .expect("Failed to find adapter");
   ```

3. **Verify shader compilation** without execution:
   ```rust
   fn test_shader_compilation() {
       let shader_code = include_str!("../shaders/splat.wgsl");
       let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
           label: Some("Test Shader"),
           source: wgpu::ShaderSource::Wgsl(shader_code.into()),
       });
       
       // If we got here without panic, compilation succeeded
       assert!(true);
   }
   ```

## Writing New Tests 📝

When adding new functionality, follow these steps:

1. **Identify test cases**:
   - Normal cases
   - Edge cases
   - Error cases

2. **Write unit tests** for individual functions

3. **Write integration tests** for component interactions

4. **Add visual tests** for rendering features

5. **Verify cross-platform compatibility**

## Testing for Multiple Platforms 🌐

Ensure tests run on all supported platforms:

```rust
#[cfg(target_os = "windows")]
#[test]
fn test_windows_specific() {
    // Windows-specific test code
}

#[cfg(target_arch = "wasm32")]
#[test]
fn test_web_specific() {
    // Web-specific test code
}
```

## Next Steps 🔍

- Learn about the [Training Module](training_module.md)
- Explore the [Rendering Module](rendering_module.md)
- Understand how to use [AI-Assisted Development](/project/ai_assisted_development.md) in development 