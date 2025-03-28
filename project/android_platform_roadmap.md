# Android Platform Roadmap

This document outlines planned enhancements for the Android version of Brush, focusing on features that are currently in development or planned for future releases.

## Performance Optimization

### Performance Tiers

Future versions of Brush for Android will implement automatic detection of device capabilities and adjust settings accordingly:

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

### Memory Management

Advanced memory management features are planned to better handle Android's limited resources:

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

Battery-aware features will help optimize usage during training:

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

Future versions will include protections against thermal throttling:

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

## UI Enhancements

### Touch Controls

Enhanced touch controls are planned for future versions:

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

Future versions will adapt to different screen sizes and orientations:

```kotlin
// In Activity
override fun onConfigurationChanged(newConfig: Configuration) {
    super.onConfigurationChanged(newConfig)
    
    val isLandscape = newConfig.orientation == Configuration.ORIENTATION_LANDSCAPE
    rustBinding.setOrientation(isLandscape)
}
```

### Material Design Integration

Complete Material Design integration is planned:

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

## Advanced Features

### Enhanced Storage Access

Improved storage access is planned for future versions:

```kotlin
// Improved file access with Storage Access Framework
private fun openDocument() {
    val intent = Intent(Intent.ACTION_OPEN_DOCUMENT).apply {
        addCategory(Intent.CATEGORY_OPENABLE)
        type = "*/*"
        putExtra(Intent.EXTRA_MIME_TYPES, arrayOf("application/zip", "application/octet-stream"))
    }
    startActivityForResult(intent, OPEN_DOCUMENT_REQUEST_CODE)
}
```

### Hardware Acceleration

Future versions will leverage device-specific accelerators:

```kotlin
// Neural Engine detection for supported devices
private fun detectNeuralAccelerators(): List<String> {
    return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
        val nnApiAccelerators = NnApiDelegate.getNnApiAccelerators()
        nnApiAccelerators.map { it.name }
    } else {
        emptyList()
    }
}
```

## Timeline

These features are prioritized as follows:

1. **Short-term** (1-3 months):
   - Memory management optimizations
   - Basic Material Design integration
   - Improved touch controls

2. **Medium-term** (3-6 months):
   - Battery-aware processing
   - Thermal throttling management
   - Enhanced storage access

3. **Long-term** (6+ months):
   - Hardware acceleration integration
   - Performance tier system
   - Full responsive layout adaptation

## Related Documentation

- [Android Platform Guide](/docs/platform_android.md) - Current Android implementation
- [Cross-Platform Framework](/docs/cross_platform_framework.md) - Current cross-platform approach 