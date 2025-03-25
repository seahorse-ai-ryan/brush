# Android Platform Guide 📱

This guide provides detailed instructions for building, running, and optimizing Brush on Android devices. Android support allows users to view and even train Gaussian splatting models directly on mobile devices.

## ⚙️ System Requirements

### Android Device Requirements
- **Android Version**: Android 8.0 (API level 26) or later
- **GPU**: Device with OpenGL ES 3.1+ or Vulkan 1.1+ support
- **RAM**: 4GB minimum, 8GB+ recommended for training
- **Storage**: 1GB+ free space for the app and models
- **Architecture**: arm64-v8a (preferred), armeabi-v7a, x86_64, or x86

### Development Environment Requirements
- **Operating System**: Windows, macOS, or Linux
- **Android SDK**: Android Studio or standalone SDK
- **Android NDK**: r25 or later
- **Rust**: 1.82 or later
- **Cargo NDK**: For building Rust code for Android

## 🛠️ Setting Up the Development Environment

### Installing Required Tools

#### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### 2. Install Android Studio

Download and install [Android Studio](https://developer.android.com/studio) for your platform.

#### 3. Install Android SDK and NDK

In Android Studio:
1. Open SDK Manager (Tools > SDK Manager)
2. Under "SDK Platforms", select Android 8.0 (API level 26) or later
3. Under "SDK Tools", select:
   - Android SDK Build-Tools
   - NDK (r25+)
   - Android SDK Command-line Tools
   - CMake
4. Click "Apply" to install

#### 4. Set Environment Variables

Add these to your shell profile (.bashrc, .zshrc, etc.):

```bash
export ANDROID_HOME=$HOME/Library/Android/sdk  # macOS
# or
export ANDROID_HOME=$HOME/Android/Sdk  # Linux
# or
export ANDROID_HOME=%LOCALAPPDATA%\Android\Sdk  # Windows

export NDK_HOME=$ANDROID_HOME/ndk/<version>
export PATH=$PATH:$ANDROID_HOME/platform-tools
```

#### 5. Install Rust Android Targets

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
```

#### 6. Install cargo-ndk

```bash
cargo install cargo-ndk
```

## 📦 Building Brush for Android

### Clone the Repository

```bash
git clone https://github.com/ArthurBrussee/brush.git
cd brush
```

### Building with Cargo NDK

```bash
cd crates/brush-android
cargo ndk -t arm64-v8a -o app/src/main/jniLibs/arm64-v8a build --release
```

This builds for arm64-v8a architecture. For other architectures:
- `armv7-linux-androideabi` → `-t armeabi-v7a`
- `x86_64-linux-android` → `-t x86_64`
- `i686-linux-android` → `-t x86`

### Building with Android Studio

1. Open the `crates/brush-android` directory in Android Studio
2. Let Gradle sync complete
3. Select "Build > Make Project"
4. For a release build, select "Build > Generate Signed Bundle / APK"

## 🚀 Running on Android Devices

### Install via ADB

```bash
cd crates/brush-android
# Install debug build
./gradlew installDebug

# Or install release build
./gradlew installRelease
```

### Install APK Directly

1. Enable "Install from Unknown Sources" on your Android device
2. Transfer the APK to your device
3. Open the APK file on the device to install

### Running from Android Studio

1. Connect your Android device via USB
2. Enable USB debugging in Developer Options
3. In Android Studio, select your device from the target dropdown
4. Click the "Run" button

## 🧪 Android-Specific Configurations

### Performance Tiers

Brush for Android automatically detects the device capabilities and adjusts settings accordingly:

```kotlin
enum class DeviceTier {
    LOW,    // Minimal rendering, limited training
    MEDIUM, // Standard rendering, basic training
    HIGH    // Full features, advanced training
}

// Usage in app
val deviceTier = when {
    hasVulkan11Support() && hasAtLeast8GbRam() -> DeviceTier.HIGH
    hasOpenGLES31Support() && hasAtLeast4GbRam() -> DeviceTier.MEDIUM
    else -> DeviceTier.LOW
}
```

### Storage Management

Brush for Android handles storage in several locations:

```kotlin
// App-specific storage (cleared on uninstall)
val internalModelsDir = context.getDir("models", Context.MODE_PRIVATE)

// Shared storage (requires permissions)
val externalDir = Environment.getExternalStoragePublicDirectory(Environment.DIRECTORY_DOCUMENTS)
val externalModelsDir = File(externalDir, "Brush/Models")
```

### Permissions

Required permissions in the AndroidManifest.xml:

```xml
<!-- Basic permissions -->
<uses-permission android:name="android.permission.INTERNET" />
<uses-permission android:name="android.permission.ACCESS_NETWORK_STATE" />

<!-- For saving/loading from shared storage -->
<uses-permission android:name="android.permission.READ_EXTERNAL_STORAGE" />
<uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE" />
<uses-permission android:name="android.permission.MANAGE_EXTERNAL_STORAGE" />

<!-- For camera capture -->
<uses-permission android:name="android.permission.CAMERA" />

<!-- Hardware feature requirements -->
<uses-feature android:glEsVersion="0x00030001" android:required="true" />
<uses-feature android:name="android.hardware.vulkan.version" android:version="0x401000" android:required="false" />
```

## 📊 Performance Optimization

### GPU Selection and Configuration

Brush can use either Vulkan or OpenGL ES as a rendering backend:

```rust
pub fn select_best_backend(context: &mut AndroidContext) -> BackendType {
    if context.has_vulkan_support() {
        BackendType::Vulkan
    } else {
        BackendType::OpenGlEs
    }
}
```

### Memory Management

Android devices have limited RAM, so careful memory management is critical:

```rust
// Monitor memory usage
pub fn check_memory_pressure(activity: &Activity) -> MemoryPressure {
    let info = activity.get_memory_info();
    
    if info.available_memory_mb < 200 {
        MemoryPressure::Critical
    } else if info.available_memory_mb < 500 {
        MemoryPressure::High
    } else {
        MemoryPressure::Normal
    }
}

// Adjust parameters based on memory pressure
pub fn adjust_for_memory_pressure(pressure: MemoryPressure, config: &mut BrushConfig) {
    match pressure {
        MemoryPressure::Critical => {
            config.max_splat_count = 100_000;
            config.batch_size = 1;
        },
        MemoryPressure::High => {
            config.max_splat_count = 500_000;
            config.batch_size = 2;
        },
        MemoryPressure::Normal => {
            // Default values
        }
    }
}
```

### Battery Usage Optimization

To optimize battery usage during training:

```rust
// Configure training to pause when battery is low
pub fn configure_battery_awareness(activity: &Activity, trainer: &mut Trainer) {
    let battery_status = activity.get_battery_status();
    
    if battery_status.level < 15 && !battery_status.is_charging {
        trainer.set_pause_on_background(true);
        trainer.set_max_iterations_per_frame(10);
    } else if battery_status.is_charging {
        trainer.set_pause_on_background(false);
        trainer.set_max_iterations_per_frame(30);
    } else {
        trainer.set_pause_on_background(true);
        trainer.set_max_iterations_per_frame(20);
    }
}
```

### Thermal Throttling Management

To handle thermal throttling:

```kotlin
class ThermalMonitor(context: Context) {
    private val thermalManager = context.getSystemService(Context.THERMAL_SERVICE) as ThermalManager

    fun registerListener(callback: (ThermalStatus) -> Unit) {
        thermalManager.addThermalStatusListener { status ->
            when (status) {
                ThermalStatus.SEVERE, ThermalStatus.CRITICAL -> callback(ThermalStatus.CRITICAL)
                ThermalStatus.MODERATE -> callback(ThermalStatus.MODERATE)
                else -> callback(ThermalStatus.NORMAL)
            }
        }
    }
}

// In Rust
pub fn handle_thermal_status(status: ThermalStatus, trainer: &mut Trainer) {
    match status {
        ThermalStatus::Critical => {
            trainer.pause();
            trainer.set_iteration_speed(0.25); // 25% speed
        },
        ThermalStatus::Moderate => {
            trainer.set_iteration_speed(0.5); // 50% speed
        },
        ThermalStatus::Normal => {
            trainer.set_iteration_speed(1.0); // Full speed
        }
    }
}
```

## 📱 UI Adaptations for Android

Brush on Android uses a touch-optimized UI with these considerations:

### Touch Controls

Camera controls are adapted for touch:
- Single finger drag: Rotate camera
- Two finger pinch: Zoom
- Two finger pan: Translate

```rust
pub fn handle_touch_event(event: &TouchEvent, camera: &mut Camera) {
    match event.pointer_count {
        1 => handle_single_touch(event, camera),
        2 => handle_double_touch(event, camera),
        _ => {}
    }
}
```

### Responsive Layout

The UI automatically adapts to different screen sizes and orientations:

```kotlin
// In Activity
override fun onConfigurationChanged(newConfig: Configuration) {
    super.onConfigurationChanged(newConfig)
    
    val isLandscape = newConfig.orientation == Configuration.ORIENTATION_LANDSCAPE
    rustBinding.setOrientation(isLandscape)
}
```

### Material Design Integration

Brush on Android follows Material Design guidelines for a native feel:

```xml
<!-- styles.xml -->
<style name="BrushTheme" parent="Theme.MaterialComponents.DayNight.NoActionBar">
    <item name="colorPrimary">@color/primaryColor</item>
    <item name="colorPrimaryVariant">@color/primaryDarkColor</item>
    <item name="colorOnPrimary">@color/primaryTextColor</item>
    <item name="colorSecondary">@color/secondaryColor</item>
    <item name="colorSecondaryVariant">@color/secondaryDarkColor</item>
    <item name="colorOnSecondary">@color/secondaryTextColor</item>
    <item name="android:statusBarColor">@color/primaryDarkColor</item>
</style>
```

## 🔧 Troubleshooting Android-Specific Issues

### Common Problems and Solutions

#### App Crashes on Launch

**Solutions:**
- Check logcat output for detailed error messages
- Verify device meets minimum requirements
- Ensure all native libraries are properly bundled
- Check for OpenGL ES/Vulkan compatibility

#### Slow Performance

**Solutions:**
- Reduce dataset size or model complexity
- Check if the device is in battery saver mode
- Monitor for thermal throttling
- Ensure device has enough free RAM

#### Storage Access Issues

**Solutions:**
- For Android 10+, use the Storage Access Framework
- Request appropriate permissions at runtime
- Handle scoped storage restrictions properly

#### NDK Build Errors

**Solutions:**
- Ensure NDK version is compatible (r25+ recommended)
- Check that paths in gradle.properties are correct
- Verify cargo-ndk is properly installed

## 🔄 Continuous Integration for Android

Example GitHub Actions workflow for automated Android builds:

```yaml
name: Android Build

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  build:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Set up JDK
      uses: actions/setup-java@v3
      with:
        java-version: '17'
        distribution: 'temurin'
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        target: aarch64-linux-android
        override: true
    
    - name: Install cargo-ndk
      uses: actions-rs/cargo@v1
      with:
        command: install
        args: cargo-ndk
    
    - name: Build native libraries
      run: |
        cd crates/brush-android
        cargo ndk -t arm64-v8a -o app/src/main/jniLibs/arm64-v8a build --release
    
    - name: Build with Gradle
      run: |
        cd crates/brush-android
        ./gradlew assembleRelease
    
    - name: Upload APK
      uses: actions/upload-artifact@v3
      with:
        name: app-release
        path: crates/brush-android/app/build/outputs/apk/release/app-release-unsigned.apk
```

## 🔗 Additional Resources

- [Android NDK Documentation](https://developer.android.com/ndk/guides)
- [Rust on Android Guide](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html)
- [Vulkan on Android](https://developer.android.com/games/develop/vulkan)
- [Android Performance Patterns](https://developer.android.com/topic/performance)
- [Material Design Guidelines](https://material.io/design) 