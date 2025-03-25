# iOS Platform Guide 📱

This guide provides detailed instructions for building and running Brush on iOS devices. The iOS platform support allows users to view and potentially train Gaussian splatting models directly on Apple mobile devices.

## ⚙️ Requirements

To build Brush for iOS, you'll need:

- A Mac computer running macOS Ventura (13.0) or later
- Xcode 14.0 or later
- Rust 1.82 or later
- iOS 15.0+ target device or simulator
- [Cargo-xcode](https://github.com/BrainiumLLC/cargo-xcode) tool for Rust-Xcode integration

## 🛠️ Setting Up the Development Environment

### 1. Install Xcode and Command Line Tools

```bash
xcode-select --install
```

Ensure Xcode is properly installed and configured.

### 2. Install Rust iOS Targets

```bash
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
```

### 3. Install Cargo-xcode

```bash
cargo install cargo-xcode
```

### 4. Clone the Brush Repository

```bash
git clone https://github.com/ArthurBrussee/brush.git
cd brush
```

## 📦 Building Brush for iOS

### 1. Generate the Xcode Project

From the root of the Brush repository:

```bash
cd crates/brush-ios
cargo xcode
```

This will generate an Xcode project in the current directory.

### 2. Open the Project in Xcode

```bash
open brush-ios.xcodeproj
```

### 3. Configure the Project

In Xcode:
1. Select the appropriate development team in the "Signing & Capabilities" tab
2. Choose your target device or simulator
3. Update bundle identifier if needed

### 4. Build and Run

Click the Run button in Xcode to build and deploy to your iOS device or simulator.

## 🧪 iOS-Specific Considerations

### Metal Integration

Brush on iOS uses Metal through wgpu's Metal backend. This provides optimal performance on Apple devices.

```rust
// Example of iOS-specific Metal configuration
#[cfg(target_os = "ios")]
fn configure_surface(instance: &Instance, surface: &Surface) -> SurfaceConfiguration {
    // iOS-specific Metal surface configuration
    SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_capabilities(&adapter).formats[0],
        width: size.width,
        height: size.height,
        present_mode: PresentMode::Fifo, // VSync for iOS
        alpha_mode: CompositeAlphaMode::Auto,
        view_formats: vec![],
    }
}
```

### Memory Management

iOS devices have more limited memory compared to desktop platforms. Consider these guidelines:

1. Limit dataset sizes when running on iOS
2. Monitor memory usage during training
3. Implement progressive loading for large models
4. Consider implementing memory warnings handlers

```swift
// Example of handling memory warnings in Swift UIKit layer
override func didReceiveMemoryWarning() {
    super.didReceiveMemoryWarning()
    // Signal Rust code to reduce memory usage
    rustBridge.handleMemoryWarning()
}
```

### UI Adaptations

The iOS version of Brush requires UI adaptations for touch controls:

1. Touch-based camera controls replace mouse controls
2. Pinch-to-zoom and two-finger rotation gestures
3. Simplified UI for smaller screens
4. Support for different orientations

## 📊 Performance Optimization

To ensure optimal performance on iOS:

1. Use Metal Performance Shaders (MPS) where available
2. Implement progressive rendering for complex scenes
3. Consider down-scaling rendering resolution on older devices
4. Monitor thermal state and reduce workload during thermal throttling

## 🔍 Troubleshooting

### Common Issues

#### App crashes immediately on launch

**Solution:**
- Check device compatibility (iOS 15.0+ required)
- Ensure proper Metal compatibility
- Verify entitlements and permissions

#### Poor performance or high battery usage

**Solution:**
- Reduce rendering resolution
- Limit training iterations
- Check for GPU-intensive operations in hot loops

#### Cannot access camera or photos

**Solution:**
- Add required privacy permissions in Info.plist:
  - `NSCameraUsageDescription`
  - `NSPhotoLibraryUsageDescription`

## 🚀 Distribution

### TestFlight

To distribute test builds:

1. Configure App Store Connect
2. Upload build through Xcode
3. Add test users
4. Use TestFlight for distribution

### App Store

For App Store distribution:

1. Create app listing on App Store Connect
2. Prepare screenshots and description
3. Configure in-app purchases if applicable
4. Archive and upload through Xcode

## 🔗 Additional Resources

- [Apple Developer Documentation](https://developer.apple.com/documentation/)
- [Metal Programming Guide](https://developer.apple.com/metal/)
- [iOS Human Interface Guidelines](https://developer.apple.com/design/human-interface-guidelines/ios/overview/themes/)
- [Rust for iOS Documentation](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-ios.html)

## 🛣️ Future Improvements

- Full training support on Apple Silicon devices
- ARKit integration for capture and reconstruction
- LiDAR scanner integration on supported devices
- Support for SharePlay collaborative viewing
- Integration with Swift UI for modern iOS UI components 