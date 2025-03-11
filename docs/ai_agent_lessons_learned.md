# AI Agent Lessons Learned - Brush Project

```json
{
  "document_type": "lessons_learned_log",
  "project_name": "Brush",
  "purpose": "Document non-obvious bugs and solutions to help AI agents learn from past mistakes",
  "usage": "Insert new entries at the top of the file, below this header section",
  "entry_format": "structured markdown with metadata and content sections",
  "last_updated": "2024-03-11"
}
```

This document serves as a knowledge base of lessons learned by AI agents while contributing to the Brush project. Each entry documents a non-obvious bug or issue that required significant effort to resolve, along with the solution and insights gained.

## How to Use This Document

- **AI Agents**: Before suggesting solutions to complex errors, check this document for similar patterns
- **Human Developers**: Review these lessons to understand common pitfalls in the Brush codebase
- **New entries**: Should be added at the top of the file, immediately below this header section

## Entry Format

Each entry should follow this format:

```markdown
---
timestamp: "YYYY-MM-DD HH:MM:SS UTC"
agent: "Agent Name and Version"
issue_category: ["ownership", "lifetime", "cross-platform", "performance", "dependency", "other"]
files_affected: ["path/to/file1.rs", "path/to/file2.rs"]
---

### Issue Description

Brief description of the issue encountered

### Root Cause

Analysis of what caused the issue

### Solution

How the issue was resolved

### Lessons Learned

Generalizable insights for future development
```

---

<!-- New entries should be added BELOW this line and ABOVE existing entries -->

<!-- ENTRIES START -->

---
timestamp: "2024-03-11 09:40:00 UTC"
agent: "Claude 3.7 Sonnet"
issue_category: ["ownership", "cross-platform", "wasm", "file handling"]
files_affected: [
  "crates/brush-app/src/app.rs",
  "crates/brush-app/src/overlays/dataset_detail.rs",
  "crates/rrfd/src/lib.rs"
]
---

### Issue: File handling issues in cross-platform Rust application

**Context**: Implementing file dialog functionality for both desktop and web (WASM) versions of the Brush application.

**Error Symptoms**:
1. Borrow checker error in `set_selected_files` method: 
   ```
   error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable
   ```
2. "Borrow of moved value" error in WASM file handling:
   ```
   error[E0382]: borrow of moved value: `file_handle`
   ```
3. Pattern matching issues with JavaScript integration:
   ```
   error: mismatched types - expected JsValue but found Option<_>
   ```

**Root Cause**:
1. In `set_selected_files`, we were trying to mutably borrow `self` while an immutable borrow was still active from accessing `self.datasets_folder`.
2. The `FileHandle::read()` method in the RRFD crate consumes the handle (takes ownership of `self`), but we were trying to use it again after calling `read()`.
3. JavaScript integration was using incorrect event patterns and not properly formatting JSON strings.

**Solution**:
1. Fixed borrow checker issues in `set_selected_files` by cloning the datasets folder first:
   ```rust
   if let Some(datasets_folder) = self.datasets_folder.clone() {
       let files_to_process: Vec<_> = file_paths.iter()
           .take(file_paths.len().saturating_sub(1))
           .map(|p| (p.clone(), datasets_folder.clone()))
           .collect();
           
       for (file_path, folder) in files_to_process {
           self.process_selected_file(file_path, folder);
       }
   }
   ```

2. Fixed the "borrow of moved value" issue by getting the file name before calling `read()`:
   ```rust
   // Get the file name first before consuming the file_handle
   let file_name = file_handle.name().unwrap_or("uploaded_file.zip".to_string());
   
   // Read the file data (this consumes file_handle)
   let data = file_handle.read().await;
   ```

3. Fixed JavaScript integration by properly formatting JSON and using correct event flags:
   ```rust
   let _ = js_sys::eval(&format!(
       "window.selectedFilePaths = '{}'; window.filesSelectedEventOccurred = true;",
       paths_json
   ));
   ```

**Better Approach**:
1. Design APIs that don't consume values when possible, or clearly document when methods take ownership.
2. Use a more structured approach to cross-platform file handling, possibly with trait abstractions.
3. Create a dedicated JavaScript interface layer rather than direct `js_sys::eval` calls.

**Generalizable Lessons**:
1. When working with file handling in Rust:
   - Be aware of ownership semantics in file handling APIs
   - Check if methods consume their receivers by looking at the signature (`self` vs `&self` vs `&mut self`)
   - Get all needed information from an object before calling methods that consume it

2. For cross-platform (WASM/Desktop) development:
   - Create clear abstractions for platform-specific code
   - Test both platforms early and often
   - Be careful with JavaScript integration, especially string formatting and event handling

3. When fixing borrow checker issues:
   - Look for opportunities to clone data to avoid borrowing conflicts
   - Consider restructuring code to avoid nested borrows
   - Use explicit scopes to limit the lifetime of borrows

---
timestamp: "2024-05-27 16:45:00 UTC"
agent: "Claude 3.7 Sonnet"
issue_category: ["UI", "state management", "message passing", "async"]
files_affected: [
  "crates/brush-app/src/overlays/controls_detail.rs", 
  "crates/brush-app/src/app.rs",
  "crates/brush-process/src/process_loop/process.rs"
]
---

### Issue: UI toggle state not properly synchronized with backend processing

**Context**: The application had a "Live update splats" toggle button in the Controls overlay that visually toggled but didn't actually affect the rendering updates. Additionally, the training toggle state wasn't properly reset when switching to a new dataset.

**Error Symptoms**: 
- The "Live update splats" button would visually toggle on/off, but splats continued to update regardless
- When switching to a new dataset while training was paused, the paused state persisted incorrectly
- UI state and backend processing state were out of sync

**Root Cause**: 
1. The Controls overlay maintained its own state for UI toggles, but this state wasn't properly communicated to the backend processing
2. The message passing system needed a new message type (`ControlMessage::LiveUpdate`) to handle the live update toggle
3. The training process loop needed to filter messages based on the live update state
4. State reset wasn't happening at the right point in the workflow (needed to happen on `StartLoading` not just `NewSource`)

**Solution**: 
1. Added a new control message type for live updates:
```rust
pub enum ControlMessage {
    Paused(bool),
    LiveUpdate(bool),
}
```

2. Modified the process loop to filter TrainStep messages based on the live update state:
```rust
// Send the message if live_update is true or if it's not a TrainStep message
let should_send = match &msg {
    ProcessMessage::TrainStep { .. } => live_update,
    _ => true,
};

if should_send {
    if output.send(msg).await.is_err() {
        return Ok(());
    }
}
```

3. Added state reset on both `NewSource` and `StartLoading` messages:
```rust
ProcessMessage::StartLoading { training } => {
    context.training = training;
    context.loading = true;
    
    // Reset the Controls overlay state when a new dataset starts loading
    self.controls_detail_overlay.reset_state();
}
```

**Better Approach**: A more robust approach would be to implement a proper state management system that:
1. Maintains a single source of truth for application state
2. Provides clear interfaces for UI components to read and update state
3. Automatically propagates state changes to all affected components
4. Handles state persistence and restoration consistently

**Generalizable Lesson**: When implementing UI controls that affect backend processing:
1. Ensure there's a clear message passing mechanism between UI and backend
2. Consider all the points in the workflow where state needs to be reset
3. Design UI components to reflect the actual state of the system, not just their local state
4. For toggles and controls, implement bidirectional state synchronization
5. When adding new features, consider how they interact with existing state management patterns

---
timestamp: "2024-05-26 15:30:00 UTC"
agent: "Claude 3.7 Sonnet"
issue_category: ["UI", "egui", "window management", "state persistence"]
files_affected: [
  "crates/brush-app/src/overlays/dataset_detail.rs", 
  "crates/brush-app/src/overlays/settings_detail.rs", 
  "crates/brush-app/src/overlays/stats_detail.rs", 
  "crates/brush-app/src/app.rs",
  "crates/brush-cli/src/lib.rs"
]
---

### Issue: Window resizability issues and inefficient approach to resetting window sizes

**Context**: The application had multiple overlay windows (Datasets, Settings, Stats) that weren't properly resizable, and we needed to implement a way to reset window sizes to their defaults.

**Error Symptoms**: 
- Windows could only be partially resized or not at all
- Initial approach to reset window sizes involved creating duplicate constructors (`new_with_default_size()`) for each overlay
- Window content didn't properly adapt to window size changes
- Some windows appeared too tall and went off-screen

**Root Cause**: 
1. Egui windows weren't properly configured to allow full resizability
2. Content within windows wasn't properly adapting to available space
3. Window state persistence was handled by egui's built-in memory system, but we weren't leveraging it correctly

**Solution**: 
1. Properly configure window content to fill available space:
```rust
// Show the window and get the response
let response = window.show(ctx, |ui| {
    // Set content to fill available space
    ui.set_width(ui.available_width());
    ui.set_height(ui.available_height());
    
    // Use ScrollArea that adapts to available space
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show_viewport(ui, |ui, _viewport| {
            // Window content here
        });
});
```

2. Use egui's memory system to reset window states instead of duplicate constructors:
```rust
// In App constructor
if reset_windows {
    cc.egui_ctx.memory_mut(|mem| {
        mem.data.clear();
    });
}
```

3. Add a CLI flag to trigger window reset:
```rust
// In CLI struct
/// Reset all window sizes and positions to their default values
#[arg(long, help = "Reset all window sizes and positions to their default values")]
pub reset_windows: bool,
```

**Better Approach**: 
- From the beginning, understand that egui handles window state persistence automatically
- Design window content to adapt to available space using proper layout techniques
- Use egui's memory system for state management rather than creating custom solutions

**Generalizable Lesson**: 
1. When working with egui windows, ensure content adapts to available space:
   - Use `ui.set_width(ui.available_width())` and `ui.set_height(ui.available_height())` to fill space
   - Use `ScrollArea` with `.auto_shrink([false, false])` to prevent content from collapsing
   - Ensure nested layouts properly propagate size constraints

2. For state persistence and reset:
   - Egui automatically persists window positions and sizes in its memory system
   - To reset all window states, clear the memory with `ctx.memory_mut(|mem| { mem.data.clear(); })`
   - Avoid creating duplicate constructors or complex reset mechanisms

3. For proper window configuration:
   - Use `.resizable(true)` to make windows resizable
   - Set reasonable `.min_width()` and `.min_height()` values
   - Use `.default_size()` to set initial size
   - Consider `.default_pos()` to set initial position

4. When adding CLI options:
   - Use clap's `#[arg]` attributes for clear documentation
   - Pass CLI options through the application initialization chain
   - Consider how CLI options interact with persistent state

---
timestamp: "2024-05-25 19:00:00 UTC"
agent: "Claude 3.7 Sonnet"
issue_category: ["types", "UI", "debugging", "egui"]
files_affected: ["crates/brush-app/src/overlays/dataset_detail.rs"]
---

### Issue: Type mismatches and debugging challenges with egui window responses

**Context**: While fixing the dataset window, we encountered subtle type mismatch errors with egui window responses and difficulties diagnosing window visibility issues without proper debugging.

**Error Symptoms**: 
- Compiler error: `expected bool, found Option<bool>` when trying to access window response data
- Window not appearing despite `open: true` being set
- Difficulty tracking why/when window visibility changed

**Root Cause**: 
1. Egui's window response types are nested and sometimes change between versions
2. The structure of `inner_response.inner` was an `Option<bool>` but code was treating it as `bool`
3. Debugging was insufficient to track window visibility state changes

**Solution**: 
1. Fixed type handling with pattern matching for window responses:
```rust
// Log window response
if let Some(inner_response) = &response {
    println!("DATASET DEBUG: Window response rect: {:?}", inner_response.response.rect);
    
    // Check if the close button was clicked - proper type handling
    if let Some(close_clicked) = inner_response.inner {
        if close_clicked {
            println!("DATASET DEBUG: Close button clicked");
            self.open = false;
        }
    }
} else {
    // Window was closed if no response
    self.open = false;
}
```

2. Added comprehensive visibility debugging:
```rust
// Debugging window visibility state
println!("DATASET DEBUG: is_open() called, returning: {}", self.open);
println!("DATASET DEBUG: set_open({}) called, was: {}", open, self.open);
println!("DATASET DEBUG: show() called, open state: {}", self.open);

// For window changes
if self.open != window_open {
    println!("DATASET DEBUG: Window open state changed: {} -> {}", self.open, window_open);
}
```

**Better Approach**: 
- Use proper pattern matching from the start when working with egui responses
- Add structured debug logging for UI state changes
- Consider creating helper methods to encapsulate egui's response handling patterns

**Generalizable Lesson**: 
1. Egui responses often have nested Option types that require proper unwrapping
2. When UI elements aren't appearing, add debug logging at these key points:
   - Construction (before showing)
   - Inside the rendering closure
   - After receiving the response
   - When state changes
3. Track window and panel sizes at each level of nesting to diagnose layout issues
4. For complex UI bugs, a systematic debugging approach with explicit state logging is essential
5. Create helper functions for common egui patterns to avoid repetitive and error-prone code

---
timestamp: "2024-05-25 18:45:00 UTC"
agent: "Claude 3.7 Sonnet"
issue_category: ["ownership", "UI", "borrow checker", "egui"]
files_affected: ["crates/brush-app/src/overlays/dataset_detail.rs"]
---

### Issue: Borrow checker errors when using `self` inside egui window closures with window state management

**Context**: While fixing the dataset detail overlay, we encountered multiple borrow checker errors when trying to use `self` inside egui closures while also passing `&mut self.open` to the window's `.open()` method.

**Error Symptoms**: 
- Rust compiler error `E0500`: "closure requires unique access to `*self` but it is already borrowed"
- Specifically: 
  ```
  error[E0500]: closure requires unique access to `*self` but it is already borrowed
  --> src/overlays/dataset_detail.rs:308:41
     |
  297 |             .open(&mut self.open) // Critical - this allows the window to close properly
     |                   -------------- borrow occurs here
  ...
  308 |         let response = window.show(ctx, |ui| {
     |                               ----      ^^^^ closure construction occurs here
     |                               |
     |                               first borrow later used by call
  ...
  400 |                                         self.select_folder();
     |                                         ---- second borrow occurs due to use of `*self` in closure
  ```

**Root Cause**: Egui's window creation pattern creates a conflict between:
1. Passing `&mut self.open` to the window's `.open()` method, which borrows `self` mutably
2. Using `self` inside the window's content closure, which requires another mutable borrow of `self`

This creates an overlapping mutable borrow scenario that Rust's borrow checker forbids.

**Solution**: 
- Use local variables to avoid the double borrow
- Track window state in a local variable and synchronize back to `self` after the closure
- For callbacks like "select folder", use flag variables that are set inside the closure and checked afterward:

```rust
// Track open state locally to avoid borrow issues
let mut window_open = self.open;

// Flag for callbacks
let mut should_select_folder = false;

// Create the window with local variable reference
let window = egui::Window::new("Datasets")
    .id(window_id)
    .open(&mut window_open) // Use local variable instead of self.open
    .resizable(true)
    // ...more window options...
    
let response = window.show(ctx, |ui| {
    // Inside closure - set flags instead of calling self methods
    if ui.button("Select Folder").clicked() {
        should_select_folder = true;
    }
    // ...more UI code...
});

// After closure - update self based on local variables
self.open = window_open;

// Handle actions that were flagged inside the closure
if should_select_folder {
    self.select_folder();
}
```

**Better Approach**: Design UI components with the borrow checker in mind from the beginning:
- Keep all UI state in separate fields that can be borrowed independently
- Use a design pattern where UI rendering doesn't require extensive self-referencing
- Consider separating UI state from application logic more explicitly

**Generalizable Lesson**: When working with egui in Rust:
1. Be careful of the pattern "window.open(&mut self.field).show(|ui| { self.other_method() })" - this is a borrow checker violation
2. Use local variables and flags to track UI state and actions during rendering
3. Update the component state from these local variables after the UI closure completes
4. Remember that closures capture their environment, creating hidden borrows the compiler must track
5. Design your component API to minimize self-referential patterns that could create borrow conflicts

---
timestamp: "2024-05-25 18:30:00 UTC"
agent: "Claude 3.7 Sonnet"
issue_category: ["UI", "layout", "egui"]
files_affected: ["crates/brush-app/src/overlays/dataset_detail.rs"]
---

### Issue: Window only resized horizontally but not vertically despite being configured as resizable

**Context**: The user was working with a dataset browser window in an egui-based application. The window was intended to be fully resizable in both dimensions, but users could only resize it horizontally.

**Error Symptoms**: 
- Window could be dragged to resize horizontally, but vertical resizing had no effect
- No error messages in the console, making the issue hard to diagnose
- Debug logs showed layout calculations resulted in 0 or negative available height for data area
- Example debug output: `Available height: 0, reserved: 150` despite window being 410px tall

**Root Cause**: Improper layout hierarchy in egui. The window used nested horizontal and vertical layouts, but the horizontal layout wasn't propagating vertical space correctly to its children. This caused inner components to have zero available height despite the window having sufficient height.

**Solution**: 
- Replaced manual horizontal+vertical layout hierarchy with egui's purpose-built `SidePanel` component
- Added explicit height constraints to make components claim their proper vertical space
- Applied the following key changes:
  ```rust
  // CRITICAL CHANGE: Use a main vertical layout for the entire window content
  ui.vertical(|ui| {
      // Force UI to take all available space
      ui.set_min_size(egui::vec2(ui.available_width(), ui.available_height()));
      
      // Use SidePanel for left side which handles resizing properly
      egui::SidePanel::left("dataset_browser_panel")
          .resizable(true)
          .min_width(280.0)
          .default_width(300.0)
          .max_width(ui.available_width() * 0.8)
          .show_inside(ui, |ui| {
              // Force panel to use all available height
              ui.set_min_height(ui.available_height());
              // ...content...
          });
  });
  ```

**Better Approach**: From the beginning, use egui's specialized layout components (`SidePanel`, `CentralPanel`, etc.) rather than manually nesting horizontal and vertical layouts. These built-in components handle sizing and resizing correctly in both dimensions.

**Generalizable Lesson**: When working with egui (or any immediate mode GUI):
1. Prefer specialized layout components over manual nested layouts when available
2. Add comprehensive debug logging for layout dimensions when resizing issues occur
3. Pay close attention to how height/width constraints propagate through nested layouts
4. Use `.set_min_size()` and `.set_min_height()` explicitly to claim available space
5. For scroll areas, use `.auto_shrink([false; 2])` to prevent them from collapsing when empty

---
timestamp: "2024-05-28 15:30:00 UTC"
agent: "Claude 3.7 Sonnet"
issue_category: ["UI", "state management", "message handling", "egui"]
files_affected: [
  "crates/brush-app/src/overlays/controls_detail.rs", 
  "crates/brush-app/src/overlays/stats_detail.rs", 
  "crates/brush-app/src/app.rs"
]
---

### Issue: UI overlays losing state and not properly reflecting application state

**Context**: The application had multiple overlay windows (Stats, Controls) that were losing their state when new datasets were loaded or training started. The Stats window would disappear, and the Controls panel would incorrectly show "No active training session" even when training was in progress.

**Error Symptoms**: 
- Stats window would disappear when a new dataset started processing
- Controls panel would show "No active training session" despite active training
- Export button and other training controls were not available when they should be
- Window state (open/closed) was not preserved across state transitions

**Root Cause**: 
1. When handling messages like `NewSource` and `StartLoading`, the overlays were being reset without preserving their open/closed state
2. The message forwarding system was inconsistent - some messages were sent to panels but not to overlays
3. The Controls panel was relying on its internal state rather than checking the AppContext's training state directly
4. Message handling was happening at different points in the update cycle, causing state inconsistencies

**Solution**: 
1. Modified the `on_message` methods to preserve window state when resetting:
```rust
pub(crate) fn on_message(&mut self, message: &ProcessMessage) {
    match message {
        ProcessMessage::NewSource => {
            // Save the current open state
            let was_open = self.open;
            let position = self.position;
            
            // Reset the overlay but preserve open state
            *self = Self::new(self.device.clone(), self.adapter_info.clone());
            
            // Restore the open state and position
            self.open = was_open;
            self.position = position;
        }
        // ...
    }
}
```

2. Updated the App to forward messages to all overlays consistently:
```rust
for message in messages {
    // Forward message to the Controls overlay
    self.controls_detail_overlay.on_message(&message);
    
    match &message {
        // ...
    }
}
```

3. Modified the Controls panel UI to check the AppContext's state directly:
```rust
// Check the AppContext's training state directly
if context.training() {
    // Show training controls
} else {
    // Show non-training message
}
```

4. Added explicit handling for training-related messages in the Controls overlay:
```rust
ProcessMessage::TrainStep { .. } => {
    // No longer force the panel to open on every train step
    // This allows users to close the panel if they wish
},
```

**Better Approach**: A more robust approach would be to:
1. Implement a centralized state management system that all UI components observe
2. Use a proper observer pattern where UI components register for state changes
3. Separate UI state (window positions, sizes) from application state (training status)
4. Use a more declarative UI approach where components render based on application state

**Generalizable Lesson**: 
1. When implementing UI overlays and panels that respond to application state:
   - Always check the source of truth (AppContext) directly in the render method
   - Don't rely solely on internal state that might become stale
   - Preserve UI state (open/closed, position) when resetting content state

2. For message handling:
   - Forward messages consistently to all components that need them
   - Handle messages at a consistent point in the update cycle
   - Be explicit about which state is reset and which is preserved

3. For debugging UI state issues:
   - Add logging for state transitions
   - Check both the UI component's internal state and the application state
   - Test all possible state transitions (loading → training → paused → resumed)

4. For export functionality:
   - Generate meaningful default filenames that include context (dataset name, timestamp)
   - Navigate directory structures intelligently to extract meaningful names
   - Provide fallbacks when expected structure isn't found

---
timestamp: "2024-03-11 08:30:00 UTC"
agent: "Claude 3.7 Sonnet"
issue_category: ["cross-platform", "file-handling", "user-interface"]
files_affected: ["crates/rrfd/src/lib.rs", "crates/brush-app/src/app.rs", "crates/brush-app/src/overlays/dataset_detail.rs", "crates/brush-app/index.html"]
---

### Issue Description

Implementing multiple file and folder selection with cross-platform support (native and WASM) required careful handling of platform-specific code and asynchronous operations. The feature needed to support both ZIP and PLY files while maintaining a consistent user experience across platforms.

### Root Cause

The existing file dialog implementation only supported selecting a single file or folder at a time. Additionally, the code for handling file selection was scattered across multiple components with platform-specific implementations that needed to be updated consistently.

### Solution

1. Added multiple file and folder selection methods to the `rrfd` crate:
   - Implemented `pick_files()` and `pick_directories()` functions with proper error handling
   - Added platform-specific conditionals to handle Android and WASM limitations

2. Updated the `DatasetDetailOverlay` struct:
   - Added `#[derive(Clone)]` to enable cloning for async operations
   - Implemented helper methods for handling multiple selections
   - Updated file processing to handle both ZIP and PLY files

3. Modified the `App` struct to use the new multiple selection methods:
   - Updated the file and folder selection handlers
   - Added WASM-specific event handling for multiple file selection

4. Enhanced the web implementation:
   - Added JavaScript code to handle custom events for multiple file selection
   - Implemented proper error handling and user feedback

### Lessons Learned

1. **Platform-specific code organization**: When implementing cross-platform features, organize code with clear conditional compilation blocks (`#[cfg(...)]`) and keep platform-specific logic isolated.

2. **Consistent error handling**: Use consistent error types and handling patterns across the codebase. The initial implementation had inconsistent error handling between `pick_files()` and `pick_directories()`.

3. **Cloning for async operations**: When working with async operations in Rust, ensure that data structures are properly cloned or moved into closures to avoid lifetime issues.

4. **Event-based communication in WASM**: For WASM implementations, use custom events and JavaScript interop to communicate between Rust and the browser environment.

5. **Generic file handling**: Design file handling code to be generic and extensible. The refactoring from `copy_zip_file_as_is` to `copy_file_as_is` made the code more maintainable and easier to extend for new file types.

6. **Progressive enhancement**: Implement features with graceful degradation for platforms with limitations (like WASM not supporting folder selection), providing clear feedback to users about platform limitations.

---
timestamp: "2025-03-11 07:30:00 UTC"
agent: "Claude 3.7 Sonnet"
issue_category: ["lifetime", "WASM", "async", "cross-platform"]
files_affected: ["crates/brush-app/src/bin/bin.rs"]
---

### Issue: WASM compilation errors due to variable lifetime issues in async closures

**Context**: Fixing compilation errors in the web application build process, specifically related to variable lifetime issues in the WASM target.

**Error Symptoms**: 
- Error message: `error[E0425]: cannot find value 'start_uri' in this scope`
- Error message: `error[E0425]: cannot find value 'rec' in this scope`
- Error message: `error[E0433]: failed to resolve: use of undeclared type 'EmbeddedCommands'`
- The build process failed with exit code 101

**Root Cause**: 
1. Duplicate code in the non-embedded and embedded sections of `bin.rs` causing namespace conflicts
2. Variable lifetime issues with `start_uri` not living long enough in async closures
3. Confusion between the embedded module's implementation and the main code path

**Solution**: 
1. Removed the duplicate code in the non-embedded section that was trying to replicate the embedded module's functionality:
```rust
// Simplified the non-embedded section to avoid conflicts
if let Some(canvas) = document
    .get_element_by_id("main_canvas")
    .and_then(|x| x.dyn_into::<web_sys::HtmlCanvasElement>().ok())
{
    // This section is handled by the embedded module
    // We'll just create a simple app without the command channel
    let (send, rec) = tokio::sync::oneshot::channel::<AppCreateCb>();
    
    // On wasm, run as a local task.
    tokio_wasm::task::spawn(async {
        eframe::WebRunner::new()
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(move |cc| {
                    Ok(Box::new(App::new(cc, send, None, false)))
                }),
            )
            .await
            .expect("Failed to start eframe");
    });
}
```

2. In the embedded module, ensured proper ownership of the `start_uri` variable by converting it to a String and cloning it inside the closure:
```rust
// Create a String from the &str to own the data
let start_uri = start_uri.to_string();

// On wasm, run as a local task.
tokio_wasm::task::spawn(async move {
    eframe::WebRunner::new()
        .start(
            canvas,
            eframe::WebOptions::default(),
            Box::new(move |cc| {
                // Clone start_uri inside the closure to avoid lifetime issues
                let uri_clone = start_uri.clone();
                Ok(Box::new(App::new(cc, send, Some(uri_clone), false)))
            }),
        )
        .await
        .expect("Failed to start eframe");
});
```

**Better Approach**: 
1. Clearly separate the embedded module's functionality from the main code path
2. Use a more structured approach to handle WASM-specific code, possibly with feature flags
3. Design the application with a clearer separation between platform-specific and shared code
4. Use stronger typing for async channels and message passing

**Generalizable Lesson**: 
1. When working with WASM and async code in Rust:
   - Be careful with variable lifetimes in closures, especially when using `move` closures
   - Clone data that needs to be used across async boundaries to avoid lifetime issues
   - Convert borrowed string slices (`&str`) to owned strings (`String`) when they need to outlive their original scope
   
2. For cross-platform code organization:
   - Use clear module boundaries for platform-specific code
   - Avoid duplicating functionality between different platform implementations
   - Consider using feature flags to conditionally compile platform-specific code
   
3. For debugging WASM compilation issues:
   - Pay close attention to lifetime errors, as they can be subtle in async contexts
   - Use verbose build output to identify the exact location of errors
   - Simplify complex code paths to isolate the source of the issue

<!-- ENTRIES END --> 