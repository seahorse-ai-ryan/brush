# Android Platform 📱

This document provides information about building, running, and optimizing Brush on Android devices.

## Overview

Brush currently has basic Android support with a minimal native wrapper around the core functionality. The primary focus is on providing the core Brush experience through a simple Android app.

For more advanced mobile features, please consider using the [web-based version](/docs/platform_web.md) of Brush, which has responsive design for mobile browsers.

## System Requirements

### Development Environment

To build Brush for Android, you need:

- Rust with Android targets configured
- Android SDK (API level 26+)
- Android NDK 
- Gradle for building

### Android Device Requirements

Brush for Android requires:
- Android 8.0 (API level 26) or higher
- Device with Vulkan or OpenGL ES 3.0+ support

## Building for Android

### Setting Up the Development Environment

1. Install Rust and add Android targets:
   ```bash
   rustup target add aarch64-linux-android armv7-linux-androideabi
   ```

2. Configure your environment variables:
   ```bash
   export ANDROID_HOME=/path/to/android/sdk
   export ANDROID_NDK_HOME=/path/to/android/ndk
   ```

### Building the Android App

1. Navigate to the Android crate:
   ```bash
   cd crates/brush-android
   ```

2. Build for the desired Android architecture:
   ```bash
   cargo build --target=aarch64-linux-android --release
   ```

3. The build process will generate the necessary libraries and include them in the Android project.

## Android-Specific Features

The current Android implementation includes:

### Basic Integration

- Android activity lifecycle management
- Simple file loading
- Touch input for camera control

### UI Adaptations

- Simple Android UI components
- Configuration for different screen sizes
- Basic permission handling

## Debugging

To debug the Android application:

1. Enable developer mode on your Android device
2. Connect your device and enable USB debugging
3. Use `adb logcat` to view logs:
   ```bash
   adb logcat -s Brush:* rust:*
   ```

## Known Limitations

The current Android implementation has some limitations:

1. Limited performance optimization
2. Basic UI with minimal platform-specific adaptations
3. No advanced memory management
4. No APK packaging automation

## Troubleshooting

### Common Issues

**Problem**: Build fails with NDK-related errors.  
**Solution**: Verify your NDK path and ensure compatible versions of NDK and Rust toolchain.

**Problem**: App crashes on startup.  
**Solution**: Check if your device supports the required Vulkan/OpenGL version.

## Related Documentation

- [Cross-Platform Framework](/docs/cross_platform_framework.md)
- [Android Platform Roadmap](/project/android_platform_roadmap.md) - Future plans for Android support 