# Getting Started with Brush 🖌️

Welcome to Brush, a powerful cross-platform 3D reconstruction framework using Gaussian splatting. This guide will help you set up your development environment and get started with the project.

## Prerequisites 📋

Before you begin, ensure you have the following installed:

- **Rust** 1.82+ - [Install Rust](https://www.rust-lang.org/tools/install)
- **Git** - [Install Git](https://git-scm.com/downloads)
- **GPU with WebGPU support** - Brush works on AMD, NVIDIA, and Intel graphics cards

### Optional Dependencies

- **Rerun Viewer** - For additional visualizations during training
  ```bash
  cargo install rerun-cli
  ```
- **Trunk** - For building the web version
  ```bash
  cargo install trunk
  ```

## Installation 🔧

1. Clone the repository:
   ```bash
   git clone https://github.com/ArthurBrussee/brush.git
   cd brush
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Running Brush 🚀

### Desktop Application

To run the desktop application:

```bash
cargo run --release
```

### Web Version

To run the web version (requires Trunk):

```bash
cd brush  # Make sure you're in the root directory
trunk serve --release
```

Then open your browser and navigate to `http://localhost:8080`.

> ⚠️ **Note**: WebGPU is still a new standard and only works on recent versions of Chrome. You may need to enable the "Unsafe WebGPU support" flag in Chrome.

### Command Line Interface

Brush can also be used as a command line tool:

```bash
cargo run --release -- --help
```

This will show you all available commands and options.

## Your First 3D Reconstruction ✨

### Preparing Your Data

Brush works with posed image data. You can use:

1. **COLMAP data** - A standard format for Structure from Motion
2. **Nerfstudio format** - With a transforms.json file

### Basic Training Workflow

1. **Organize your data** in a directory with the following structure:
   ```
   dataset/
   ├── images/
   │   ├── image1.jpg
   │   ├── image2.jpg
   │   └── ...
   └── transforms.json  # If using Nerfstudio format
   ```

2. **Start training**:
   ```bash
   cargo run --release -- train --dataset path/to/dataset
   ```

3. **Visualize progress** with Rerun:
   ```bash
   rerun brush_blueprint.rbl
   ```

## Development Tips 💡

- Use `cargo test --all` to run all tests
- Enable tracing with `--feature=tracy` for detailed performance profiling
- For development builds, use `cargo run` without the `--release` flag for faster compilation but slower execution

## Next Steps 🛣️

- Learn about the [Core Concepts](core_concepts.md)
- Explore the [Architecture](architecture.md)
- Check out the [Command-Line Interface](cli.md)

## Troubleshooting 🔍

- If you encounter compilation errors, make sure you're using Rust 1.82+
- For WebGPU issues on the desktop app, make sure your graphics drivers are up to date
- For additional help, join the [Brush Discord server](https://discord.gg/TbxJST2BbC)

---

*For more detailed information on specific platforms, see:*
- [Windows](platform_windows.md)
- [macOS](platform_macos.md)
- [Linux](platform_linux.md)
- [Android](platform_android.md)
- [Web](platform_web.md)
- [iOS](platform_ios.md) 