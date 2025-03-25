# macOS Platform Guide 🍎

This guide provides detailed instructions for building, running, and optimizing Brush on macOS. Apple's ecosystem offers excellent performance for both rendering and training, particularly on newer Apple Silicon Macs with their unified memory architecture.

## ⚙️ System Requirements

- **Operating System**: macOS Monterey (12.0) or later
- **Hardware**: Intel Mac or Apple Silicon (M1/M2/M3)
- **RAM**: 8GB minimum, 16GB+ recommended for larger datasets
- **GPU**: Integrated Apple GPU (Apple Silicon) or discrete GPU (Intel Macs)
- **Disk Space**: 2GB+ for Brush and its dependencies

## 🛠️ Setting Up the Development Environment

### Installing Required Tools

#### Xcode Command Line Tools

```bash
xcode-select --install
```

Accept the license agreement when prompted. This installs essential development tools including Git and a C compiler.

#### Homebrew (Optional but Recommended)

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

#### Installing Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Select option 1 for default installation. After installation, configure your current shell:

```bash
source "$HOME/.cargo/env"
```

Verify the installation:

```bash
rustc --version
cargo --version
```

### Apple Silicon Specific Setup

If you're on an Apple Silicon Mac (M1/M2/M3), you may need to install Rosetta 2 for compatibility with some Intel-only tools:

```bash
softwareupdate --install-rosetta
```

## 📦 Building Brush

### Clone the Repository

```bash
git clone https://github.com/ArthurBrussee/brush.git
cd brush
```

### Debug Build

```bash
cargo build
```

### Release Build (Recommended for Performance)

```bash
cargo build --release
```

### Running Brush

```bash
# Run the application
cargo run --release

# Or run the CLI with specific command
cargo run --release -- --help
```

## 🧪 macOS-Specific Configurations

### Metal Configuration

Brush on macOS uses Metal through wgpu's Metal backend. This provides optimal performance on Apple devices. The system automatically configures Metal for the best performance.

```rust
// Example of macOS-specific Metal configuration
#[cfg(target_os = "macos")]
fn configure_surface(instance: &Instance, surface: &Surface) -> SurfaceConfiguration {
    // macOS-specific Metal surface configuration
    SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_capabilities(&adapter).formats[0],
        width: size.width,
        height: size.height,
        present_mode: PresentMode::Fifo, // VSync for smoother rendering
        alpha_mode: CompositeAlphaMode::Auto,
        view_formats: vec![],
    }
}
```

### Universal Binary (Intel and Apple Silicon)

To build a universal binary that runs natively on both Intel and Apple Silicon Macs:

```bash
# Install Rust targets for both architectures
rustup target add x86_64-apple-darwin aarch64-apple-darwin

# Build for both architectures
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Create a universal binary
lipo -create -output target/brush \
  target/x86_64-apple-darwin/release/brush \
  target/aarch64-apple-darwin/release/brush
```

### Notarization (for Distribution)

If you plan to distribute your Brush build, you should notarize it to avoid Gatekeeper warnings:

1. Create an app bundle structure:
```bash
mkdir -p Brush.app/Contents/MacOS
cp target/release/brush Brush.app/Contents/MacOS/
```

2. Create Info.plist in Brush.app/Contents/:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>brush</string>
    <key>CFBundleIconFile</key>
    <string>brush.icns</string>
    <key>CFBundleIdentifier</key>
    <string>com.example.brush</string>
    <key>CFBundleName</key>
    <string>Brush</string>
    <key>CFBundleDisplayName</key>
    <string>Brush</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>0.2.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSRequiresAquaSystemAppearance</key>
    <false/>
</dict>
</plist>
```

3. Sign the application with your Developer ID:
```bash
codesign --force --options runtime --sign "Developer ID Application: Your Name (XXXXXXXXXX)" Brush.app
```

4. Create a DMG for distribution:
```bash
hdiutil create -volname "Brush" -srcfolder Brush.app -ov -format UDZO Brush.dmg
```

5. Notarize the DMG:
```bash
xcrun altool --notarize-app --primary-bundle-id "com.example.brush" --username "your@email.com" --password "app-specific-password" --file Brush.dmg
```

6. Check notarization status:
```bash
xcrun altool --notarization-history 0 -u "your@email.com" -p "app-specific-password"
```

7. Staple the notarization ticket:
```bash
xcrun stapler staple Brush.dmg
```

## 📊 Performance Optimization

### Apple Silicon Optimization

On Apple Silicon Macs:

1. Ensure you're running the native ARM64 version for best performance
2. Leverage the unified memory architecture for better GPU performance
3. Use Metal Performance Shaders (MPS) where available

### Memory Pressure Management

macOS uses memory compression to optimize memory usage. To monitor memory pressure:

```bash
# Install memory pressure monitoring tool
brew install memory-pressure

# Run the tool
memory-pressure
```

For large datasets, consider:
1. Increasing swap file size (System Settings > General > Storage > Developer)
2. Closing memory-intensive applications
3. Restarting your Mac before processing very large datasets

### Energy Settings

For better performance during training:

1. Connect your Mac to power
2. Open System Settings > Energy Saver/Battery
3. Disable "Put hard disks to sleep when possible"
4. Set "Turn display off after" to a longer duration
5. Disable "Enable Power Nap" during training sessions

## 👁️ Using Rerun for Visualization

To use Rerun with Brush on macOS:

```bash
# Install Rerun
cargo install rerun-cli

# Open Rerun viewer
rerun ./brush_blueprint.rbl
```

## 🔧 Troubleshooting Common Issues

### Build Errors

#### Missing Libraries

```
error: failed to run custom build command for `system-deps v6.0.3`
```

**Solution:**
- Install pkg-config: `brew install pkg-config`

#### Metal Shader Compilation Errors

**Solution:**
- Update to the latest macOS version
- Ensure your Mac supports Metal 2.0+
- Check Console.app for detailed Metal error messages

### Runtime Issues

#### Application Security

If you see "App is damaged and can't be opened":

**Solution:**
- Open Terminal
- Run: `xattr -d com.apple.quarantine /path/to/brush`
- Or right-click the app and select "Open" to bypass Gatekeeper once

#### Performance Degradation Over Time

**Solution:**
- Check for thermal throttling using Activity Monitor (CPU tab)
- Ensure proper ventilation for your Mac
- Use a cooling pad for extended training sessions

## 🛡️ Security Considerations

### Camera and Photo Access

If Brush needs to access the camera or photos for importing datasets:

1. Add the required privacy descriptions to Info.plist:
```xml
<key>NSCameraUsageDescription</key>
<string>Brush needs camera access to capture new dataset images.</string>
<key>NSPhotoLibraryUsageDescription</key>
<string>Brush needs photo access to import images for dataset creation.</string>
```

2. Request permissions programmatically before accessing these resources

## 🔗 Additional Resources

- [Metal Developer Documentation](https://developer.apple.com/documentation/metal)
- [macOS Development Resources](https://developer.apple.com/macos/)
- [Rust on macOS](https://doc.rust-lang.org/book/ch01-01-installation.html#macos)
- [Apple Silicon Developer Transition](https://developer.apple.com/documentation/apple-silicon)

## 📱 Integration with iOS

Brush can seamlessly integrate with iOS devices for enhanced workflow:

### Continuity Camera

Use your iPhone as a high-quality camera for capturing new datasets:

1. Ensure both devices are on the same Wi-Fi network and have Bluetooth enabled
2. Sign in to the same Apple ID on both devices
3. Position your iPhone to capture the object/scene
4. In Brush, select "File > Import > Continuity Camera"

### AirDrop for Quick Dataset Transfer

Transfer processed models between devices:

1. Export your model from Brush (File > Export > PLY)
2. Right-click the file and select "Share > AirDrop"
3. Choose your iOS device from the AirDrop menu
4. Open the file on iOS in a compatible viewer

## 🔌 External GPU Support (Intel Macs)

For Intel Macs, you can enhance performance with an external GPU (eGPU):

1. Connect a compatible eGPU using Thunderbolt 3
2. macOS should automatically recognize the eGPU
3. To force Brush to use the eGPU:
   - Right-click on the Brush application
   - Select "Get Info"
   - Check "Prefer External GPU"

## 🚀 Optimizing for Apple Silicon

### Using Apple Neural Engine (ANE)

Future versions of Brush may leverage the Apple Neural Engine for accelerated ML operations:

```rust
#[cfg(target_arch = "aarch64")]
fn configure_ml_backend() -> MLBackend {
    // Use the ANE when available on Apple Silicon
    if has_neural_engine() {
        MLBackend::AppleNeuralEngine
    } else {
        MLBackend::Metal
    }
}
```

### Memory Optimization for Unified Architecture

On Apple Silicon, the GPU shares memory with the CPU, which can be leveraged for better performance:

```rust
#[cfg(target_arch = "aarch64")]
fn allocate_shared_buffer(size: usize) -> SharedBuffer {
    // Create memory that can be accessed by both CPU and GPU without copying
    SharedBuffer::new_unified(size)
}
``` 