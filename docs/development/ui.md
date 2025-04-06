# UI Development Guide

This guide explains the structure and patterns used for developing the User Interface (UI) in the `crates/brush-app` crate, built using [`egui`](https://github.com/emilk/egui) and [`eframe`](https://github.com/emilk/egui/tree/master/crates/eframe).

> **Note:** Assumes completion of [Developer Setup](./setup.md) and basic familiarity with Rust and `egui`.

## Core Concepts

*   **Framework:** Uses `eframe` for windowing/platform integration and `egui` for UI widgets and layout. Main app logic is in `crates/brush-app/src/app.rs`.
*   **Panel System (`egui_tiles`):** UI is composed of dockable panels (e.g., `ScenePanel`, `SettingsPanel`, `StatsPanel`, `DatasetPanel`, `PresetsPanel`) managed by [`egui_tiles`](https://github.com/emilk/egui_tiles). Layout is configured in `App::new` (`app.rs`). Each panel implements the `AppPanel` trait defined in `app.rs`.
*   **Shared State (`AppContext`):** A central `struct` (`app.rs`) holding data shared across panels (loaded `Dataset`, `Camera`, `WgpuDevice`, process status, etc.), wrapped in `Arc<RwLock<>>` for safe concurrent access.
    *   > **Warning:** Keep write locks (`context.write()`) brief to avoid blocking the UI thread and causing freezes, especially during background processing.
*   **Background Communication (Messages):** Async message passing (`tokio::sync::mpsc`) between the UI (`brush-app`) and the background process (`brush-process`).
    *   **`ControlMessage`:** UI -> Background (defined in `crates/brush-process/src/process_loop/mod.rs`). Triggered by UI interactions (sliders, buttons like `Load file`, `▶ training`, etc.).
    *   **`ProcessMessage`:** Background -> UI (defined in `crates/brush-process/src/process_loop/process.rs`). Updates UI panels with status/data (e.g., populating `StatsPanel`, providing images for `DatasetPanel`, errors).
    *   **Dispatch:** `App::receive_messages` in `app.rs` calls the appropriate `AppPanel::on_message` method for relevant panels.

## Common Tasks

_**Tip:** Refer to Brush's existing panel implementations in `crates/brush-app/src/panels/` for practical examples._

### Modifying an Existing Panel

*   **Goal:** Change layout or controls (e.g., adding a setting to `SettingsPanel`).
*   **Steps:** Edit the panel's `ui` method in its corresponding file under `src/panels/`, read state from `AppContext`, and send `ControlMessage`s via `context.control_message(...)` within a brief write lock.

### Adding a New Panel

*   **Goal:** Create a new dockable view (e.g., Log Viewer).
*   **Steps:** Create a new `struct` in `src/panels/`, implement the `AppPanel` trait (`title`, `ui`, optional `on_message`), register the new panel module in `src/panels/mod.rs`, then instantiate and add it to the layout tree in `App::new` (`app.rs`).

### Changing Panel Layout (`egui_tiles`)

*   **Goal:** Modify the initial panel arrangement or allowed docking behavior.
*   **Focus:** The `egui_tiles::Tree::new_tabs` and related layout construction logic within `App::new` in `app.rs`. You can rearrange how `TileId`s are grouped into `egui_tiles` containers (`Linear`, `Tabs`, etc.).
*   **See Also:** The main [`egui` website](https://www.egui.rs/) showcases various capabilities and examples.

### UI Interaction Patterns

Key patterns for coordinating UI actions with background tasks and shared state:

1.  **Sending Commands/Config (`ControlMessage`):** The standard way for UI actions (button clicks, slider changes) to trigger background processing or modify settings used by the background process.
2.  **Displaying Data (`ProcessMessage`):** The standard way for the background process to send updates (status, results, errors) back to the UI. Panels update their internal state in `on_message`, and the `ui` method reads this state for display.
3.  **Starting/Restarting Process:** Loading datasets or initiating training/viewing often involves calling `start_process(...)` (from `crates/brush-process/src/running_process.rs`) to spawn the background task.
4.  **File System Operations (e.g., Export):** UI actions like manual export might trigger a direct async task within `brush-app` using `tokio::spawn`. Use with caution, as long-running tasks here can affect responsiveness, and filesystem access has platform limitations (especially on web).

## Platform Considerations

*   Most `egui` code is platform-agnostic.
*   **Filesystem Access:** Native vs. Web differences for file *dialogs* are handled by `rrfd` (with limitations). Saving files on the web relies on browser download mechanisms.
*   **Performance:** Web performance is generally lower than native due to WASM overhead and browser limitations. Specific bottlenecks may vary.
*   **Testing:** Test changes on both native (`cargo run --bin brush_app`) and web (`trunk serve`).

## Next Steps

*   Review the overall [Project Architecture](./architecture.md) to see how `brush-app` fits into the larger system.
*   Consult the official [Egui documentation](https://docs.rs/egui/) and website ([egui.rs](https://www.egui.rs/)) for detailed information on widgets, layouts, and best practices. 