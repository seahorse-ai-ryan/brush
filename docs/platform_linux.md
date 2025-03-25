# Linux Platform Guide 🐧

This guide provides detailed instructions for building, running, and optimizing Brush on Linux systems. Linux offers excellent performance for both training and rendering due to its efficient resource management and strong GPU support.

## ⚙️ System Requirements

- **Operating System**: Ubuntu 20.04+, Fedora 34+, or other modern Linux distributions
- **CPU**: Multi-core processor (4+ cores recommended for training)
- **RAM**: 8GB minimum, 16GB+ recommended for larger datasets
- **GPU**: NVIDIA (with CUDA support), AMD, or Intel GPU with up-to-date drivers
- **Disk Space**: 2GB+ for Brush and its dependencies

## 🛠️ Setting Up the Development Environment

### Installing Required Dependencies

```bash
# For Debian/Ubuntu-based distributions
sudo apt update
sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libfontconfig1-dev \
    libxcb-xfixes0-dev \
    libxkbcommon-dev \
    python3 \
    python3-pip \
    libudev-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev

# For Fedora/RHEL-based distributions
sudo dnf install -y \
    gcc \
    make \
    openssl-devel \
    fontconfig-devel \
    libxcb-devel \
    libxkbcommon-devel \
    python3 \
    python3-pip \
    systemd-devel \
    gtk3-devel \
    webkit2gtk3-devel
```

### Installing Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup default stable
rustup update
```

### GPU Drivers Setup

#### NVIDIA GPUs

```bash
# Add NVIDIA repository
sudo add-apt-repository ppa:graphics-drivers/ppa
sudo apt update

# Install drivers and CUDA
sudo apt install -y nvidia-driver-525 nvidia-cuda-toolkit
```

Verify installation with:
```bash
nvidia-smi
```

#### AMD GPUs

```bash
# For Ubuntu
sudo add-apt-repository ppa:oibaf/graphics-drivers
sudo apt update
sudo apt install -y mesa-vulkan-drivers
```

Verify installation with:
```bash
vulkaninfo
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

## 🧪 Linux-Specific Configurations

### Using Different Graphics APIs

Brush on Linux can use Vulkan or WebGPU backends. You can select the backend using environment variables:

```bash
# Force Vulkan backend
WGPU_BACKEND=vulkan cargo run --release

# Force WebGPU backend
WGPU_BACKEND=webgpu cargo run --release
```

### X11 vs Wayland Considerations

Brush works on both X11 and Wayland display servers, but there are some considerations:

#### X11
```bash
# Ensure X11 libraries are installed
sudo apt install -y libx11-dev libxcb-randr0-dev libxcb-xtest0-dev libxcb-xinerama0-dev libxcb-shape0-dev libxcb-xkb-dev
```

#### Wayland
```bash
# Ensure Wayland libraries are installed
sudo apt install -y libwayland-dev libwayland-egl1-dev wayland-protocols
```

If encountering issues with Wayland, you can force X11:
```bash
# Force X11 even on Wayland sessions
export WINIT_UNIX_BACKEND=x11
cargo run --release
```

## 📊 Performance Optimization

### CPU Optimization

```bash
# Use performance governor
sudo cpupower frequency-set -g performance

# Or for systems without cpupower
echo "performance" | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
```

### GPU Optimization for NVIDIA

```bash
# Set PowerMizer to prefer maximum performance
sudo nvidia-settings -a "[gpu:0]/GpuPowerMizerMode=1"

# Disable compositor for better performance
# For KDE Plasma
kwriteconfig5 --file kwinrc --group Compositing --key "Enabled" false
```

### Memory Management

For large datasets, consider increasing your swap space:

```bash
# Create a 16GB swap file
sudo fallocate -l 16G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile

# Make it permanent
echo '/swapfile none swap sw 0 0' | sudo tee -a /etc/fstab
```

## 👁️ Using Rerun for Visualization

To use Rerun with Brush on Linux:

```bash
# Install Rerun
cargo install rerun-cli

# Open Rerun viewer
rerun ./brush_blueprint.rbl
```

## 🔧 Troubleshooting Common Issues

### Missing Libraries

If you encounter "missing shared library" errors:

```bash
# Update the dynamic linker run-time bindings
sudo ldconfig

# Install common missing libraries
sudo apt install -y libvulkan1 libvulkan-dev
```

### GPU Detection Issues

If Brush cannot detect your GPU:

```bash
# Check if your GPU is detected by the system
lspci | grep -E 'VGA|3D'

# For NVIDIA GPUs, check if the driver is loaded
lsmod | grep nvidia

# For AMD GPUs, check if the driver is loaded
lsmod | grep amdgpu

# Verify Vulkan support
vulkaninfo | grep deviceName
```

### Display Issues

If you encounter display or window creation issues:

```bash
# Force software rendering to test if it's a GPU driver issue
LIBGL_ALWAYS_SOFTWARE=1 cargo run

# Check for compositor conflicts
# Temporarily disable compositor (KDE example)
qdbus org.kde.KWin /Compositor suspend
```

## 🔒 Security Considerations

### Limiting GPU Access

For multi-user systems, you might want to restrict GPU access:

```bash
# Create a group for GPU access
sudo groupadd -r gpuusers
sudo usermod -aG gpuusers $USER

# Set permissions for NVIDIA devices
sudo tee /etc/udev/rules.d/99-nvidia-gpu.rules > /dev/null << 'EOT'
SUBSYSTEM=="nvidia*", GROUP="gpuusers", MODE="0660"
SUBSYSTEM=="renderD*", GROUP="gpuusers", MODE="0660"
EOT

sudo udevadm control --reload-rules && sudo udevadm trigger
```

## 🔗 Additional Resources

- [Vulkan SDK Installation](https://vulkan.lunarg.com/doc/sdk/latest/linux/getting_started.html)
- [NVIDIA Developer Resources](https://developer.nvidia.com/cuda-downloads)
- [AMD GPU Open](https://gpuopen.com/)
- [Rust on Linux](https://www.rust-lang.org/tools/install)
- [WebGPU Linux Support](https://github.com/gfx-rs/wgpu/wiki/Implementation-Status)

## 🚀 Distribution Packaging

To create distribution packages for easier deployment:

### Debian/Ubuntu Package

```bash
# Install cargo-deb
cargo install cargo-deb

# Build Debian package
cargo deb --manifest-path=crates/brush-app/Cargo.toml
```

### AppImage Creation

```bash
# Install required tools
sudo apt install -y fuse libfuse2

# Build AppImage using linuxdeploy
# (This is a simplified example, actual process may require more steps)
cargo build --release
mkdir -p AppDir/usr/bin/
cp target/release/brush AppDir/usr/bin/
linuxdeploy --appdir=AppDir --output appimage
```

## 💻 Desktop Integration

### Creating a Desktop Entry

Create a file at `~/.local/share/applications/brush.desktop`:

```ini
[Desktop Entry]
Type=Application
Name=Brush
Comment=3D Gaussian Splatting Application
Exec=/path/to/brush
Icon=/path/to/brush/icon.png
Terminal=false
Categories=Graphics;3DGraphics;
```

Then update the desktop database:
```bash
update-desktop-database ~/.local/share/applications
``` 