# Build Process 🏗️

This document describes the process for building Brush from source for different platforms, covering the various build configurations and environment setups.

## Prerequisites 📋

Before building Brush, ensure you have the following installed:

- **Rust** 1.82+ (specified in `rust-toolchain.toml`)
- **Cargo** (included with Rust)
- **Git** for cloning the repository
- **GPU with WebGPU support** for running the application

### Platform-Specific Prerequisites

| Platform | Additional Requirements |
|----------|-------------------------|
| **Windows** | - Visual Studio build tools<br>- Windows 10/11 |
| **macOS** | - Xcode Command Line Tools<br>- macOS 11+ |
| **Linux** | - GCC or Clang<br>- X11 or Wayland dev libraries<br>- OpenGL dev libraries |
| **Web** | - [Trunk](https://trunkrs.dev/)<br>- WebGPU-compatible browser |
| **Android** | - Android SDK<br>- Android NDK<br>- Cargo-apk |
| **iOS** | - Xcode<br>- iOS SDK |

## Build Configuration ⚙️

Brush uses Cargo's workspace feature to manage multiple crates. The main configuration can be found in the following files:

- **Cargo.toml**: Defines workspace members and common dependencies
- **rust-toolchain.toml**: Specifies the required Rust version
- **Trunk.toml**: Configures the web build process
- **deny.toml**: Sets dependency audit rules

### Build Profiles

Brush defines several build profiles in `Cargo.toml`:

```toml
[profile.dev]
opt-level = 1
debug = true

[profile.dist]
inherits = "release"
lto = "thin"
# debug = true # good for profilers
```

- **dev**: Development build with minimal optimizations for faster compilation
- **release**: Standard optimized release build
- **dist**: Distribution build with additional optimizations

## Building for Desktop 🖥️

### Standard Build

To build Brush for desktop platforms (Windows, macOS, Linux):

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release
```

The compiled binary will be located at:
- Debug build: `target/debug/brush-app`
- Release build: `target/release/brush-app`

### Running the Desktop Application

```bash
# Development build
cargo run

# Release build
cargo run --release
```

### Building with Tracing

To enable performance tracing with Tracy:

```bash
cargo build --release --features=tracy
```

## Building for Web 🌐

### Setting Up Trunk

First, install Trunk:

```bash
cargo install trunk
```

### Building and Serving the Web Version

```bash
# Development build with auto-reload
trunk serve

# Release build
trunk build --release
```

The web build output will be in the `dist/` directory.

### Web Build Process

The web build process:

1. Compiles Rust code to WebAssembly (WASM)
2. Processes the `index.html` entry point
3. Bundles assets and generates JavaScript bindings
4. Creates a distribution package

## Building for Android 📱

### Setting Up Android Build Environment

1. Install Android SDK and NDK
2. Set up environment variables:
   ```bash
   export ANDROID_HOME=/path/to/android/sdk
   export ANDROID_NDK_HOME=/path/to/android/ndk
   ```
3. Install cargo-apk:
   ```bash
   cargo install cargo-apk
   ```

### Building Android APK

```bash
cd crates/brush-android
cargo apk build --release
```

The APK will be generated in `target/release/apk/`.

## Building for iOS 📱

iOS support is experimental. Refer to platform-specific documentation in [platform_ios.md](platform_ios.md).

## Build Customization 🛠️

### Feature Flags

Brush supports several feature flags to customize the build:

- **tracy**: Enables Tracy profiler integration
- **rerun**: Enables Rerun visualization

Enable features with:

```bash
cargo build --release --features=tracy,rerun
```

### Build Scripts

Several crates use `build.rs` scripts to:

- Compile shaders
- Generate code
- Configure platform-specific settings

For example, `brush-render/build.rs` processes shader files before compilation.

## Cross-Compilation 🔄

### Cross-Compiling for Windows from Linux

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

### Cross-Compiling for Linux from macOS

```bash
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```

## Dependency Management 📦

Brush manages dependencies through:

- **Workspace dependencies** in the root `Cargo.toml`
- **Direct dependencies** in individual crate manifests
- **Git dependencies** for libraries requiring specific versions

Key dependency configurations:

```toml
# From root Cargo.toml
[workspace.dependencies]
burn = { git = "https://github.com/tracel-ai/burn", features = [
    'wgpu',
    'autodiff',
] }
wgpu = { version = "24", features = ["naga-ir"] }
```

## Build Versioning 🏷️

Version information is defined in `Cargo.toml`:

```toml
[workspace.package]
edition = "2024"
version = "0.2.0"
readme = "README.md"
license = "Apache-2.0"
repository = "https://github.com/ArthurBrussee/brush"
```

## Common Build Issues 🐛

### Missing WebGPU Support

**Problem**: Build succeeds but application fails with "WebGPU not supported"  
**Solution**: Ensure you have recent graphics drivers installed, or use Chrome 131+ for web builds

### Shader Compilation Errors

**Problem**: Build fails with shader compilation errors  
**Solution**: Check WGSL shader files and ensure they comply with the WebGPU spec

### Dependency Resolution Failures

**Problem**: `cargo build` fails with dependency resolution errors  
**Solution**: Ensure you're using Rust 1.82+ and try cleaning the build with `cargo clean`

## Continuous Integration 🔄

Brush uses GitHub Actions for continuous integration:

- Automated testing on multiple platforms
- Linting checks
- Build verification
- Web deployment

## Next Steps 🔍

- Learn about the [Development Workflow](development_workflow.md)
- Explore platform-specific guides:
  - [Windows](platform_windows.md)
  - [macOS](platform_macos.md)
  - [Linux](platform_linux.md)
  - [Web](platform_web.md)
  - [Android](platform_android.md)
  - [iOS](platform_ios.md) 