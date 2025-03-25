# Upstream Brush Testing Approach 🧪

This document summarizes the testing approach used in the upstream Brush project, providing guidance for maintaining compatibility and following best practices in our fork.

## Testing Overview

The upstream Brush project employs several testing approaches to ensure code quality:

- **Unit Tests**: Test individual functions and components
- **Integration Tests**: Test interactions between components
- **Visual Tests**: Verify rendered outputs
- **Regression Tests**: Ensure fixes don't reintroduce bugs
- **Cross-Platform Tests**: Verify functionality across platforms

## Running Tests

The upstream project provides several methods for running tests:

### Running All Tests

```bash
cargo test --all
```

### Running Tests for a Specific Crate

```bash
cargo test -p brush-render
```

### Running a Specific Test

```bash
cargo test -p brush-render test_gaussian_projection
```

### Running Tests with Features

```bash
cargo test --all --features=tracy
```

## Test Organization

Brush organizes tests in several ways:

### 1. Module-Level Tests

Tests are often included within the same file as the code being tested:

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

Dedicated test modules exist in many crates:

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

## Types of Tests

### Unit Tests

Unit tests focus on individual functions or components. The project includes utility functions for creating test environments:

```rust
pub fn create_test_device() -> Device {
    // Create a test device with appropriate configuration
}

pub fn create_test_scene() -> Scene {
    // Create a scene with predefined test data
}
```

### Assertion Helpers

The project uses custom assertion functions for more complex comparisons:

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

## Testing Best Practices

The upstream project follows these testing best practices:

1. **Test One Thing**: Each test focuses on one specific behavior
2. **Arrange-Act-Assert**: Tests are structured with setup, action, and verification
3. **Descriptive Names**: Test names describe what is being tested
4. **Independent Tests**: Tests don't depend on each other
5. **Avoid Flakiness**: Tests produce consistent results

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

## Automated Testing

The upstream project uses GitHub Actions for continuous integration. This includes automated runs of the test suite on commits and pull requests.

## Testing Frameworks Used

Based on the research of the upstream project and industry best practices, these frameworks are used or recommended:

### Desktop Testing
- **Unit Tests**: Rust's built-in testing framework
- **UI Testing**: egui_kittest for EGUI-based UI components
- **Integration Testing**: Custom test harnesses built on Rust's test framework

### Web Testing
- **Unit Tests**: wasm-bindgen-test for WebAssembly components
- **UI Testing**: Selenium or Playwright for browser automation
- **Console Logging**: Enhanced MCP integration for better debugging visibility

### Android Testing
- **UI Testing**: Espresso (Google's native UI testing framework) or UI Automator
- **Cross-platform Testing**: Appium for unified testing approach
- **Integration with CI**: GitHub Actions for automated testing

### iOS Testing
- **UI Testing**: XCTest/XCUITest or EarlGrey
- **Cross-platform Testing**: Appium as an alternative
- **Integration with CI**: GitHub Actions for automated testing

## Implications for Our Fork

For our fork of Brush, we should:

1. **Maintain Test Compatibility**: Ensure our changes pass the existing test suite
2. **Add Tests for New Features**: Write comprehensive tests for any new functionality
3. **Adopt the Same Test Structure**: Follow the same test organization and best practices
4. **Leverage CI/CD**: Use GitHub Actions for automated testing
5. **Extend Cross-Platform Testing**: Ensure tests run on all target platforms

## Recommendations

1. Before making changes, run the existing test suite to establish a baseline
2. Create test fixtures for common scenarios in our extended functionality
3. For UI changes, add visual tests to ensure consistency across platforms
4. Implement automated cross-platform testing early in the development process
5. Use test-driven development (TDD) when implementing new features 