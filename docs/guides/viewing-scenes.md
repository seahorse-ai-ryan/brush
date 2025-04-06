# Viewing Pre-Trained Scenes

Brush can view Gaussian Splatting models saved in the standard `.ply` format. This is useful for inspecting results from training runs (see [Training a Scene](./training-a-scene.md)) or models from other sources.

## Loading and Viewing

1.  **Launch Brush:**
    *   **Desktop:** Run the `brush_app` executable (or use `cargo run --bin brush_app --release`).
    *   **Web:** Open the [**Web Demo**](https://arthurbrussee.github.io/brush-demo/) in a compatible browser (see [Installing Brush](./installing-brush.md#web-demo)).
        > **Note:** The web demo is experimental and requires WebGPU support.

2.  **Load PLY File:**
    *   In the **`Settings`** panel, use the **`Load file`** button to select a local `.ply` file.
    *   Alternatively, enter a publicly accessible URL to a `.ply` file in the text box and click **`Load URL`**.

3.  **Interact with the Scene:**
    *   The model appears in the **`Scene`** panel.
    *   Use the mouse/keyboard controls to navigate (Orbit, Look, Pan, Zoom, Fly, Roll - hover over "Controls" for hints).
    *   The **`Stats`** panel shows model details.

## Tip: Web Demo URL Parameters

You can directly load a model and configure the web demo viewer using URL parameters:

*   `?url=<YOUR_PLY_FILE_URL>`: Loads the specified `.ply` file automatically.
*   `&focal=<NUMBER>`: Sets the initial focal length (e.g., `&focal=1.2`).
*   `&zen=true`: Hides most UI panels, maximizing the scene view.

**Example:** `https://arthurbrussee.github.io/brush-demo/?url=https://example.com/model.ply&zen=true`

![Brush web demo showing garden scene in zen mode](../media/Brush_demo_pretrained_garden%20_scene.png)

> **Note:** Loading large `.ply` files via URL might be slow or hit browser memory limits.

## Viewing Animated Scenes

Brush also supports viewing animated sequences of Gaussian Splats. As noted in the [upstream project README](https://github.com/ArthurBrussee/brush/blob/main/README.md):

> Brush also can load .zip of splat files to display them as an animation, or a special ply that includes delta frames. This was used for cat-4D and Cap4D!

*   **ZIP Archives:** Load a `.zip` file containing multiple numbered `.ply` files (e.g., `frame_0000.ply`, `frame_0001.ply`, ...). Brush will detect the sequence and basic playback controls (Play/Pause) will appear below the scene view.
*   **Delta PLY:** Load a single `.ply` file that contains base frame data plus delta information for subsequent frames (requires specific PLY structure, as used in projects like [cat-4D](https://github.com/zju3dv/cat-4d) and [Cap4D](https://github.com/yijiaguo/Cap4D)). Playback controls will also appear.

Loading is done via the same **`Load file`** or **`Load URL`** buttons described above.

## Next Steps

*   Learn how to [Train a Scene](./training-a-scene.md).
*   Explore the [Project Architecture](../development/architecture.md) to understand how the viewer works. 