# Windows Platform Guide 🪟

This guide provides detailed instructions for building, running, and optimizing Brush on Windows systems. Windows offers excellent hardware compatibility and is a common platform for both developers and end-users of 3D applications.

## ⚙️ System Requirements

- **Operating System**: Windows 10 (20H2+) or Windows 11
- **CPU**: Multi-core processor (4+ cores recommended for training)
- **RAM**: 8GB minimum, 16GB+ recommended for larger datasets
- **GPU**: NVIDIA, AMD, or Intel GPU with up-to-date drivers
- **Disk Space**: 2GB+ for Brush and its dependencies
- **Graphics API**: DirectX 12 or Vulkan 1.2+

## 🛠️ Setting Up the Development Environment

### Installing Rust

1. Download and run the Rust installer from [rustup.rs](https://rustup.rs/)
2. During installation, select the default installation options
3. Open a new command prompt and verify the installation:

```cmd
rustc --version
cargo --version
```

### Installing Required Dependencies

#### Visual Studio Build Tools

1. Download and install the [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
2. Select the "Desktop development with C++" workload
3. Ensure the following components are selected:
   - MSVC C++ build tools
   - Windows 10/11 SDK
   - C++ CMake tools for Windows

#### Additional Tools

```cmd
:: Install Git if not already installed
winget install Git.Git

:: Install Python (optional, used by some build scripts)
winget install Python.Python.3
```

### GPU Driver Setup

Ensure you have the latest GPU drivers installed:

- **NVIDIA**: Download from [NVIDIA Driver Downloads](https://www.nvidia.com/Download/index.aspx)
- **AMD**: Download from [AMD Driver Support](https://www.amd.com/en/support)
- **Intel**: Download from [Intel Driver & Support Assistant](https://www.intel.com/content/www/us/en/download/driver-support-assistant.html)

## 📦 Building Brush

### Clone the Repository

```cmd
git clone https://github.com/ArthurBrussee/brush.git
cd brush
```

### Debug Build

```cmd
cargo build
```

### Release Build (Recommended for Performance)

```cmd
cargo build --release
```

### Running Brush

```cmd
:: Run the application
cargo run --release

:: Or run the CLI with specific command
cargo run --release -- --help
```

## 🧪 Windows-Specific Configurations

### Graphics API Selection

Brush on Windows can use DirectX 12, Vulkan, or WebGPU backends. You can select the backend using environment variables:

```cmd
:: Force DirectX 12 backend
set WGPU_BACKEND=dx12
cargo run --release

:: Force Vulkan backend
set WGPU_BACKEND=vulkan
cargo run --release

:: Force WebGPU backend
set WGPU_BACKEND=webgpu
cargo run --release
```

### Path Length Limitations

Windows has a default path length limit of 260 characters, which can cause issues with Rust projects. To avoid problems:

1. Enable long path support in Windows (requires admin privileges):

```cmd
reg add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\FileSystem" /v LongPathsEnabled /t REG_DWORD /d 1 /f
```

2. Enable long paths in Git:

```cmd
git config --system core.longpaths true
```

### High-DPI Display Support

For high-DPI displays, you may need to adjust Windows scaling settings:

1. Right-click on the Brush executable (.exe) file
2. Select Properties
3. Go to the Compatibility tab
4. Click on "Change high DPI settings"
5. Check "Override high DPI scaling behavior"
6. Select "Application" from the dropdown

## 📊 Performance Optimization

### Windows Power Plan

Set your Windows power plan to "High Performance" for better training performance:

1. Open Control Panel
2. Go to Power Options
3. Select "High Performance" power plan

If not visible, click on "Show additional plans" or create a custom plan.

### GPU Settings

#### NVIDIA Control Panel

For NVIDIA GPUs, optimize settings in the NVIDIA Control Panel:

1. Right-click on desktop and select "NVIDIA Control Panel"
2. Go to "Manage 3D settings"
3. Select "Program Settings" tab
4. Add Brush (browse to the .exe file)
5. Set "Power management mode" to "Prefer maximum performance"
6. Set "Threaded optimization" to "On"
7. Click Apply

#### AMD Radeon Software

For AMD GPUs:

1. Open AMD Radeon Software
2. Go to "Performance" or "Gaming" tab
3. Add Brush application
4. Set "Graphics Profile" to "Performance"
5. Disable "Radeon Chill" and "Frame Rate Target Control"

### Memory Management

For handling large datasets:

1. Increase Windows page file size:
   - Right-click on "This PC" and select "Properties"
   - Click on "Advanced system settings"
   - Under Performance, click "Settings"
   - Select "Advanced" tab
   - Under Virtual memory, click "Change"
   - Uncheck "Automatically manage paging file size for all drives"
   - Select a drive with plenty of space
   - Select "Custom size"
   - Set Initial size and Maximum size (recommended 1.5x your RAM)
   - Click "Set" and then "OK"

2. Close unnecessary background applications:
   - Use Task Manager (Ctrl+Shift+Esc) to identify and close resource-intensive applications
   - Consider using Windows Game Mode (Win+G) to reduce background processes

## 👁️ Using Rerun for Visualization

To use Rerun with Brush on Windows:

```cmd
:: Install Rerun
cargo install rerun-cli

:: Open Rerun viewer
rerun .\brush_blueprint.rbl
```

## 🔧 Troubleshooting Common Issues

### Build Errors

#### Missing MSVC Toolchain

```
error: linker `link.exe` not found
```

**Solution:**
- Install Visual Studio Build Tools with C++ components
- Ensure the PATH environment variable includes MSVC bin directory

#### Cargo Network Errors

If you're behind a corporate firewall or proxy:

```cmd
:: Set HTTP proxy for Cargo
set HTTP_PROXY=http://proxy.example.com:8080
set HTTPS_PROXY=http://proxy.example.com:8080
```

### Runtime Issues

#### GPU Not Detected

**Solutions:**
- Update GPU drivers to the latest version
- Check if your GPU supports DirectX 12 or Vulkan
- Try switching graphics backends using the WGPU_BACKEND environment variable

#### Application Crashes on Startup

**Solutions:**
- Check Windows Event Viewer for detailed error logs
- Try running in debug mode: `cargo run`
- Run with tracing enabled: `cargo run --release --features=tracy`

#### Performance Issues

**Solutions:**
- Check for thermal throttling using tools like HWiNFO or GPU-Z
- Ensure you're running the Release build, not Debug
- Close other GPU-intensive applications
- Check Windows Task Manager for resource constraints

## 🔒 Windows Security Considerations

### Firewall Settings

If Brush needs network access for any features:

1. When prompted by Windows Defender Firewall, allow access
2. To manually configure:
   - Open Windows Defender Firewall
   - Click "Allow an app or feature through Windows Defender Firewall"
   - Click "Change settings" and then "Allow another app..."
   - Browse to the Brush executable
   - Select appropriate networks (Private/Public)

### Windows Defender SmartScreen

When running a newly built executable, Windows might display a SmartScreen warning:

1. Click "More info"
2. Click "Run anyway"

For development builds, you can temporarily disable SmartScreen:
1. Open Windows Security
2. Go to "App & browser control"
3. Under "Check apps and files", select "Warn" instead of "Block"

## 📈 Profiling on Windows

### Using Tracy Profiler

1. Build Brush with the tracy feature:

```cmd
cargo build --release --features=tracy
```

2. Download and run [Tracy Profiler](https://github.com/wolfpld/tracy/releases)
3. Launch Brush
4. Connect Tracy to view real-time performance data

### Using Windows Performance Analyzer

1. Install the [Windows Performance Toolkit](https://docs.microsoft.com/en-us/windows-hardware/test/wpt/)
2. Record a trace:

```cmd
wpr -start CPU
:: Run Brush and perform the operations you want to profile
wpr -stop CPU_trace.etl
```

3. Open the trace in Windows Performance Analyzer:

```cmd
wpa CPU_trace.etl
```

## 🔗 Additional Resources

- [Rust on Windows](https://docs.microsoft.com/en-us/windows/dev-environment/rust/setup)
- [DirectX Developer Blog](https://devblogs.microsoft.com/directx/)
- [Vulkan SDK for Windows](https://vulkan.lunarg.com/sdk/home#windows)
- [Windows Dev Center](https://developer.microsoft.com/en-us/windows/)

## 🚀 Distribution

### Creating a Windows Installer

You can create an installer using tools like [Inno Setup](https://jrsoftware.org/isinfo.php):

1. Install Inno Setup
2. Create a script (brush_setup.iss):

```
[Setup]
AppName=Brush
AppVersion=0.2.0
DefaultDirName={pf}\Brush
DefaultGroupName=Brush
OutputDir=installer

[Files]
Source: "target\release\brush.exe"; DestDir: "{app}"
Source: "LICENSE"; DestDir: "{app}"
Source: "README.md"; DestDir: "{app}"
Source: "assets\*"; DestDir: "{app}\assets"; Flags: recursesubdirs

[Icons]
Name: "{group}\Brush"; Filename: "{app}\brush.exe"
Name: "{group}\Uninstall Brush"; Filename: "{uninstallexe}"
```

3. Compile the installer:

```cmd
"C:\Program Files (x86)\Inno Setup 6\ISCC.exe" brush_setup.iss
```

### Creating a Portable ZIP

For a portable version:

```cmd
mkdir brush_portable
copy target\release\brush.exe brush_portable\
xcopy /E assets brush_portable\assets\
copy LICENSE brush_portable\
copy README.md brush_portable\
powershell Compress-Archive -Path brush_portable\* -DestinationPath brush_portable.zip
```

## 💻 Windows Shell Integration

### Creating Desktop Shortcut

```cmd
powershell "$s=(New-Object -COM WScript.Shell).CreateShortcut('%userprofile%\Desktop\Brush.lnk');$s.TargetPath='%cd%\target\release\brush.exe';$s.Save()"
```

### File Associations for .ply Files

To associate Brush with .ply files, create a registry script (brush_file_assoc.reg):

```
Windows Registry Editor Version 5.00

[HKEY_CLASSES_ROOT\.ply]
@="Brush.PLYFile"

[HKEY_CLASSES_ROOT\Brush.PLYFile]
@="PLY 3D Model"

[HKEY_CLASSES_ROOT\Brush.PLYFile\DefaultIcon]
@="C:\\Path\\To\\brush.exe,0"

[HKEY_CLASSES_ROOT\Brush.PLYFile\shell\open\command]
@="\"C:\\Path\\To\\brush.exe\" \"%1\""
```

Replace `C:\\Path\\To\\brush.exe` with the actual path, then double-click the .reg file to apply it. 