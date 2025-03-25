# Testing Environments 🧪

This document outlines the testing environments available for Brush development, including both general project testing requirements and the specific environments available for this fork's development.

## General Brush Testing Requirements

The Brush project is designed to work across multiple platforms and environments. When contributing to the project, testing should be performed on as many of these platforms as possible to ensure cross-platform compatibility.

### Standard Testing Platforms

- **Desktop**
  - Windows 10/11 with DirectX 12 or Vulkan
  - macOS 12+ (Monterey or later) with Metal
  - Linux with Vulkan or OpenGL

- **Mobile**
  - Android 8.0+ with OpenGL ES 3.1 or Vulkan
  - iOS 15.0+ with Metal

- **Web**
  - Chrome 131+ with WebGPU support
  - Firefox and Safari (when WebGPU support becomes available)

### GPU Compatibility

Brush is designed to work on a variety of GPU architectures:
- NVIDIA GPUs
- AMD GPUs
- Intel integrated graphics
- Apple Silicon integrated graphics

### Minimum Specifications

- **CPU**: Dual-core processor (quad-core recommended for training)
- **RAM**: 4GB minimum (8GB+ recommended)
- **GPU**: Any GPU supporting the required graphics APIs
- **Storage**: 1GB free space

## Available Testing Environments for This Fork

For this fork's development, we have access to the following specific testing environments:

### Primary Development Environment

- **MacBook Pro**
  - Used for primary development and testing
  - macOS with Metal graphics API
  - Apple Silicon architecture
  - Can test desktop macOS builds

### Mobile Testing Devices

- **Android Pixel Phone**
  - For testing Android builds
  - Tests OpenGL ES and Vulkan backends

- **iPad Pro**
  - For testing iOS builds
  - Tests Metal backend
  - Can test web version in Safari

### Additional Testing Resources

- **Windows Machine with NVIDIA RTX GPU**
  - For testing Windows builds with RTX acceleration
  - Tests DirectX 12 and Vulkan backends

### Remote/Cloud Testing Options

When needed, the following cloud testing options can be considered:

- **Browser Stack** or **LambdaTest** for web browser testing across platforms
- **AWS Device Farm** for mobile device testing on a wider range of devices
- **GitHub Actions** for CI/CD testing on various virtual environments

## Testing Workflow

When making changes to Brush, the following testing workflow is recommended:

1. **Local Testing**
   - Run unit tests locally on the primary development environment
   - Test basic functionality on the primary platform

2. **Cross-Platform Testing**
   - Deploy to available physical devices for testing
   - Use cloud testing services for platforms not physically available

3. **Performance Testing**
   - Test performance on low-end and high-end devices
   - Compare with baseline performance metrics

4. **Regression Testing**
   - Ensure existing functionality continues to work
   - Run the full test suite across platforms

## Platform-Specific Testing Notes

### macOS Testing

- Use Metal Debug tools for GPU performance analysis
- Test on both Intel and Apple Silicon when possible

### iOS Testing

- Test different display sizes (iPad vs iPhone)
- Verify touch controls work correctly

### Android Testing

- Test on different screen sizes and densities
- Verify performance on various GPU types

### Web Testing

- Test different viewport sizes
- Verify WebGPU fallbacks when applicable
- Check load times and memory usage

## Test Data

For consistent testing across environments, standard test datasets should be used:

- Small test scenes for quick function validation
- Medium test scenes for typical use case validation
- Large test scenes for performance testing

These datasets are available in the project repository under `test_cases/` or can be downloaded from the project's website.

## Documentation for Test Results

When reporting test results, include:

1. Environment details (OS, hardware, drivers)
2. Test dataset used
3. Performance metrics (FPS, memory usage, etc.)
4. Screenshots or videos demonstrating functionality
5. Any issues or anomalies observed

This information helps track platform-specific behaviors and ensures that improvements can be properly verified across all supported environments. 