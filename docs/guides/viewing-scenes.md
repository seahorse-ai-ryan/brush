# Viewing Pre-Trained Scenes

Brush can be used as a viewer for Gaussian Splatting models saved in the standard `.ply` format. This is useful for inspecting results from previous training runs (see [Training a Scene](./training-a-scene.md)) or models obtained from other sources.

## Using the Desktop App (`brush_app`)

1.  **Launch Brush:** Run the `brush_app` executable (or use `cargo run --bin brush_app --release` if built from source).
2.  **Load PLY File:**
    *   In the **`Settings`** panel, click the **`Load file`** button.
    *   Select the `.ply` file you want to view.
    *   Alternatively, if the `.ply` file is hosted online, enter its URL in the text box and click **`Load URL`**.
3.  **Interact with the Scene:**
    *   The model will load and appear in the **`Scene`** panel.
    *   Use the mouse and keyboard controls to navigate (Hover over the "Controls" text in the `Scene` panel for a reminder: Orbit, Look, Pan, Zoom, Fly, Roll).
    *   The **`Stats`** panel will show the number of splats and SH degree of the loaded model. GPU memory usage will be minimal.

## Using the Web Demo

1.  **Access the Demo:** Open the Brush Web Demo at [**arthurbrussee.github.io/brush-demo/**](https://arthurbrussee.github.io/brush-demo/) in a compatible browser (see [Installing Brush](./installing-brush.md#web-demo)).
    > **Warning:** The public web demo is experimental and may have limitations.
2.  **Load PLY File:**
    *   **Via URL Parameter (Recommended):** Append `?url=<YOUR_PLY_FILE_URL>` to the demo URL. The model should load automatically.
        *   Example: `https://arthurbrussee.github.io/brush-demo/?url=https://example.com/path/to/your/model.ply`
        *   Make sure the URL is publicly accessible and properly encoded.
        *   You can also add camera parameters like `&focal=1.2` or hide UI panels with `&zen=true`.
    *   **Via UI:** The **`Settings`** panel in the web demo also has **`Load file`** (requires user selection; verified on Chrome, may vary on other browsers) and **`Load URL`** buttons. <!-- Resolved: Verified on Chrome, leaving note for other browsers -->
3.  **Interact:** Use the same mouse/keyboard controls as the desktop app to navigate the scene in the **`Scene`** panel.
    ![Brush web demo showing garden scene in zen mode](../media/Brush_demo_pretrained_garden_scene.png)

> **Tip:** Loading large `.ply` files directly into the web demo might be slow or hit browser memory limits.

## Next Steps

*   Learn how to [Train a Scene](./training-a-scene.md).
*   Explore the [Project Architecture](../development/architecture.md) to understand how the viewer works. 