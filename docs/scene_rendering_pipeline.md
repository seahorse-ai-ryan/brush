# Scene Rendering Pipeline

This document explains how the 3D Gaussian Splat rendering pipeline works in the Scene pane of the Brush application, including real-time updates during training and camera interaction.

## Overview

The Scene panel is the primary visualization component of Brush, responsible for rendering 3D Gaussian Splats in real-time. It provides an interactive view of the model during training and allows users to explore the 3D scene through camera controls.

## Key Components

### Core Libraries and Imports

```rust
use brush_dataset::splat_export;
use brush_process::process_loop::{ControlMessage, ProcessMessage};
use brush_train::{scene::ViewImageType, train::TrainBack};
use brush_ui::burn_texture::BurnTexture;
use brush_render::{
    camera::{focal_to_fov, fov_to_focal},
    gaussian_splats::Splats,
};
use eframe::egui_wgpu::Renderer;
use egui::{Color32, Rect};
use glam::{Quat, UVec2, Vec3};
use web_time::Instant;
```

These imports highlight the key dependencies:
- **brush_render**: Handles the core rendering of Gaussian Splats
- **brush_train**: Provides training functionality and scene representation
- **egui_wgpu**: Integrates WGPU rendering with the egui UI framework
- **glam**: Provides vector math for 3D transformations

## Rendering Pipeline

The rendering pipeline in the Scene panel follows these steps:

1. **Initialization**: The `ScenePanel` is created with a WGPU device and queue for GPU rendering
2. **Texture Management**: A `BurnTexture` is used to manage the GPU texture for displaying the rendered scene
3. **Frame Rendering**: Each frame is rendered based on the current camera position and splat data
4. **UI Integration**: The rendered texture is displayed in the egui UI

### ScenePanel Structure

```rust
pub(crate) struct ScenePanel {
    pub(crate) backbuffer: BurnTexture,
    pub(crate) last_draw: Option<Instant>,
    view_splats: Vec<Splats<<TrainBack as AutodiffBackend>::InnerBackend>>,
    frame_count: u32,
    frame: f32,
    live_update: bool,
    paused: bool,
    err: Option<ErrorDisplay>,
    zen: bool,
    last_state: Option<RenderState>,
}
```

The `ScenePanel` maintains:
- A backbuffer texture for rendering
- The current splat data
- UI state (paused, live update)
- Timing information for animation

## Real-time Updates During Training

The Scene panel receives real-time updates from the training process through a message-passing system:

1. **Message Reception**: The `on_message` method processes incoming messages from the training loop
2. **Splat Updates**: When new splat data is available, it's stored in the `view_splats` vector
3. **Rendering Trigger**: The UI is marked for redraw when new data arrives

```rust
fn on_message(&mut self, message: &ProcessMessage, context: &mut AppContext) {
    match message {
        ProcessMessage::Splats { splats, .. } => {
            // Update splats data for rendering
            self.view_splats = vec![splats.clone()];
            // Reset frame counter for animation
            self.frame_count = 0;
        }
        // Handle other message types...
    }
}
```

## Camera Controls and Interaction

The Scene panel integrates with the `CameraController` from `orbit_controls.rs` to provide interactive camera movement:

### Camera Model

The camera uses:
- Position (Vec3)
- Rotation (Quaternion)
- Field of View (FOV)
- Focus distance

### Interaction Methods

1. **Orbit Control**: Click and drag to orbit around the model
2. **Pan**: Right-click and drag (or middle-click) to pan the view
3. **Zoom**: Mouse wheel to zoom in/out
4. **Reset**: Button to reset camera to default position

The camera controller translates user input into camera transformations:

```rust
// In orbit_controls.rs
pub struct CameraController {
    pub orbit_button: egui::PointerButton,
    pub pan_button: egui::PointerButton,
    // Camera parameters and state...
}

impl CameraController {
    pub fn update(&mut self, ui: &egui::Ui, camera: &mut Camera) {
        // Process input and update camera position/rotation
    }
}
```

## Rendering Process

The actual rendering of Gaussian Splats happens in the `draw_splats` method:

1. **Camera Setup**: The current camera parameters are used to set up the view
2. **Splat Rendering**: The splats are rendered using the WGPU pipeline
3. **Texture Update**: The rendered image is transferred to the backbuffer texture
4. **UI Display**: The texture is displayed in the UI

```rust
pub(crate) fn draw_splats(
    &mut self,
    ui: &mut egui::Ui,
    context: &mut AppContext,
    splats: &Splats<<TrainBack as AutodiffBackend>::InnerBackend>,
) {
    // Set up rendering parameters
    // Render splats to texture
    // Update UI with rendered texture
}
```

## Performance Considerations

The rendering pipeline includes several optimizations:

1. **Frame Skipping**: Rendering is skipped if the camera hasn't moved and no new data is available
2. **Asynchronous Updates**: Rendering happens asynchronously from the UI thread
3. **GPU Acceleration**: WGPU is used for hardware-accelerated rendering
4. **Adaptive Quality**: Rendering quality can adjust based on performance

## Integration with Training Loop

The Scene panel integrates with the training loop through:

1. **Process Messages**: The panel receives updates via `ProcessMessage` events
2. **Control Messages**: The panel can send `ControlMessage` events to control the training process
3. **Shared Context**: The `AppContext` provides shared state between panels

This bidirectional communication allows for interactive training and visualization, where users can see the model evolve in real-time and adjust parameters as needed.

## Conclusion

The Scene rendering pipeline in Brush provides a powerful, interactive visualization of 3D Gaussian Splats. By leveraging WGPU for GPU acceleration and integrating tightly with the training process, it enables real-time feedback during model training while providing intuitive camera controls for exploring the 3D scene.

The modular design separates rendering concerns from UI and training logic, allowing for efficient updates and a responsive user experience across different platforms. 