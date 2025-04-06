# UI Development Guide

This guide explains the structure and patterns used for developing the User Interface (UI) in the `crates/brush-app` crate, built using [`egui`](https://github.com/emilk/egui) and [`eframe`](https://github.com/emilk/egui/tree/master/crates/eframe).

> **Prerequisites:**
>
> *   Completed [Developer Setup](./setup.md).
> *   Basic familiarity with Rust and the `egui` immediate mode GUI library. (See [egui docs](https://docs.rs/egui/) for basics).
> *   Understanding of the overall [Project Architecture](./architecture.md).

## Core Concepts

*   **Framework:** Uses `eframe` for windowing/platform integration and `egui` for UI widgets and layout. Main app logic in `crates/brush-app/src/app.rs`.
*   **Panel System (`egui_tiles`):** UI is composed of dockable panels (e.g., `ScenePanel`, `SettingsPanel`, `StatsPanel`, `DatasetPanel`, `PresetsPanel`) managed by [`egui_tiles`](https://github.com/emilk/egui_tiles). Layout is configurable (see `desktop-ui-vertical-split.png` example) and defined in `App::new`. Each panel implements the `AppPanel` trait (`app.rs`).
*   **Shared State (`AppContext`):** Central `struct` (`app.rs`) holding data shared across panels (loaded `Dataset`, `Camera`, `WgpuDevice`, process status, etc.), wrapped in `Arc<RwLock<>>` for safe concurrent access.
    *   > **Warning:** Keep write locks (`context.write()`) brief.
*   **Background Communication (Messages):** Async message passing (`tokio::sync::mpsc`) between `brush-app` and `brush-process`.
    *   **`ControlMessage`:** UI -> Background (`process_loop/mod.rs`). Triggered by UI interactions (sliders, buttons like `Load file`, `▶ training`, etc.).
    *   **`ProcessMessage`:** Background -> UI (`process_loop/process.rs`). Updates UI panels with status/data (e.g., populating `StatsPanel`, providing images for `DatasetPanel`, errors).
    *   **Dispatch:** `App::receive_messages` calls `AppPanel::on_message`.

## Common Tasks

_**Tip:** Refer to existing panels in `crates/brush-app/src/panels/` for examples._

### Modifying an Existing Panel

*   **Goal:** Change layout or controls (e.g., adding a setting to `SettingsPanel`).
*   **Steps:** Edit the panel's `ui` method (`src/panels/*.rs`), read state from `AppContext`, and send `ControlMessage`s via `context.control_message(...)` within a brief write lock.

### Adding a New Panel

*   **Goal:** Create a new dockable view (e.g., Log Viewer).
*   **Steps:** Create `struct`, implement `AppPanel` (`title`, `ui`, optional `on_message`), register in `panels/mod.rs`, instantiate and add to layout in `App::new` (`app.rs`).

### Changing Panel Layout (`egui_tiles`)

*   **Goal:** Modify the initial panel arrangement.
*   **Focus:** `App::new` in `app.rs`. Reorganize how `TileId`s are grouped into `egui_tiles` containers (`Linear`, `Tabs`).
*   **See Also:** [`egui_tiles` examples](https://github.com/emilk/egui_tiles/blob/master/examples/)

### UI Interaction Patterns

(Recap)

1.  **Sending Commands/Config (`ControlMessage`):** Standard UI->Background trigger.
2.  **Displaying Data (`ProcessMessage`):** Standard Background->UI update. Panel updates internal state in `on_message`, `ui` reads internal state.
3.  **Starting/Restarting Process:** Loading datasets via `start_process(...)`.
4.  **Direct Async Task (Use with Caution!):** For self-contained UI tasks (e.g., Export button). Risk of blocking, stale state, platform limits. <!-- TODO: Verify export task works reliably on web/android -->

## Platform Considerations

*   Most `egui` code is platform-agnostic.
*   **Filesystem Access:** Native vs. Web differences handled by `rrfd` (with limitations).
*   **Performance:** Web may differ; check browser console.
*   **Testing:** Test on native (`cargo run --bin brush_app`) and web (`trunk serve`).

## Key Files & Structs

*   `crates/brush-app/src/app.rs`: `App`, `AppContext`, `AppPanel` trait, `App::new` (layout), `App::receive_messages`.
*   `crates/brush-app/src/panels/`: Panel implementations (e.g., `settings.rs`, `scene.rs`, `stats.rs`, `dataset.rs`, `presets.rs`).
*   `crates/brush-process/src/process_loop/process.rs`: `ProcessMessage` enum.
*   `crates/brush-process/src/process_loop/mod.rs`: `ControlMessage` enum.
*   `crates/brush-process/src/running_process.rs`: `start_process` function.

## Next Steps

*   Review the overall [Project Architecture](./architecture.md) to see how `brush-app` fits in.
*   Explore specific panel implementations in `crates/brush-app/src/panels/`.
*   Consult the [Egui documentation](https://docs.rs/egui/) for details on widgets and layouts. 