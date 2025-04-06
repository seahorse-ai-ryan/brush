# Training a Scene

This guide walks through the process of training a new 3D Gaussian Splatting scene from your own data using the Brush **desktop application (`brush_app`)**. Due to current limitations and performance issues (detailed below), training via the web interface is not recommended.

**Prerequisites:**

*   Brush installed (see [Installing Brush](./installing-brush.md)).
*   A dataset suitable for training. Brush supports:
    *   **COLMAP format:** A directory containing `images/` and `sparse/0/` subdirectories from a COLMAP reconstruction.
    *   **Nerfstudio format (Synthetic NeRF):** A directory containing `images/` and a `transforms.json` file.
    *   Datasets packaged as `.zip` archives containing either of the above structures.

> **Warning: Browser Training Limitations** ⚠️
>
> While Brush can run in a web browser (WebAssembly), training performance and quality are significantly lower compared to the native desktop application. Furthermore, known issues in the underlying Burn framework ([e.g., Burn #2901](https://github.com/tracel-ai/burn/issues/2901)) can cause **training failures on WASM** due to shader generation problems. For any training, using the **desktop version is strongly recommended**. The browser version is primarily intended for viewing pre-trained scenes.

## Core Workflow

1.  **Launch Brush:** Run the `brush_app` executable (or use `cargo run --bin brush_app --release` if built from source).
    *(The initial UI displays placeholder text and default settings)*
    ![Initial Brush desktop application UI on macOS](../media/Brush_desktop_macos.png)

    *(The UI during training includes panels for settings, scene rendering, and dataset viewing)*
    ![Brush desktop application UI during training, showing the Settings/Stats panel (left), Scene view (center), and Dataset view (right)](../media/Brush_training_room_scene.png)

2.  **Load Your Dataset:**
    *   Use the **`Settings`** or **`Presets`** tab in the left-hand panel:
        *   **`Settings` Tab:**
            *   Click **`Load file`** to select a `.zip` archive or `.ply` file from your computer.
            *   Click **`Load directory`** (Desktop only) to select the root folder of your COLMAP or Nerfstudio dataset.
            *   Enter a URL to a `.zip` or `.ply` file in the text box and click **`Load URL`**.
        *   **`Presets` Tab:**
            *   This tab contains links to download example datasets (hosted on Google Drive).
            *   Clicking a preset name (e.g., `bicycle`, `lego`) will **open a download link in your browser**.
            *   You must first download the `.zip` file and then use the **`Load file`** button in the **`Settings`** tab to load it into Brush.
    *   Check the Scene panel for any error messages if loading fails.

3.  **(Optional) Adjust Settings:**
    *   Before or after loading data, you can tweak parameters in the **`Settings`** tab. These correspond to [CLI options](./cli-usage.md) as well.
        *   **Model Settings:** `Spherical Harmonics Degree`, `Max image resolution`, `Max Splats`.
        *   **Load Settings:** `Limit max frames`, `Split dataset for evaluation` (if checked, enables the `eval` tab in the Dataset panel).
        *   **Training Settings:** `Train ... steps` (sets the target number of training iterations).
        *   **Process Settings:** `Evaluate every ... steps`, `Export every ... steps` (enables automatic periodic export, see `## Exporting Results` below).
        *   **Rerun Settings** (Desktop only, requires `--features=rerun` build):
            *   `Enable rerun`: Connects to a running Rerun viewer.
            *   `Log train stats every`: Controls frequency of scalar logging (loss, iter/s) sent to Rerun.
            *   `Visualize splats every`: Controls frequency of logging the 3D splat point cloud to Rerun (can be heavy).
            *   *(Includes a link to `rerun.io` and a note about using the `brush_blueprint.rbl` file for layout)*.

    **Rerun Visualization Example:** When enabled, Rerun provides a detailed, time-scrubbing view of the training process:
    <video src="https://github.com/user-attachments/assets/f679fec0-935d-4dd2-87e1-c301db9cdc2c" controls width="100%"></video>
    *Rerun viewer showing detailed training visualization for the LEGO dataset.*

## Monitoring Training

Once a dataset is loaded, training typically starts automatically in the background. You can monitor and control it using the UI panels:

*   **`Stats` Panel:** Displays the current training iteration number (`Train step`) compared to the total steps planned (`Train ... steps`), along with the current training speed (`Steps/s`). It also shows other statistics like total splat count and estimated GPU memory usage.
*   **`Scene` Panel Controls & View:** The area below the main 3D view provides crucial controls and feedback:
    *   `⏵ training` / `⏸ paused`: Click this button to **pause or resume** the background training process. Training automatically runs unless paused here.
    *   `🔴 Live update splats`: This toggle controls whether the 3D view updates with the latest splats from the training process. Disabling this can improve UI responsiveness but doesn't pause the actual training. It defaults to enabled.
    *   `⬆ Export`: Manually exports the *currently visible* splats (see `## Exporting Results` below).
    *   `Controls`: Shows camera control hints on hover.
    *   The **main view** shows the Gaussian splats. If `Live update splats` is enabled, this view updates periodically. Check here for "Loading..." or error messages overlayed on the view.
*   **`Dataset` Panel:**
    *   Displays the dataset image closest to the current camera pose in the `Scene` panel.
    *   **Interaction:** Moving the camera in the `Scene` panel will update the image shown here. Conversely, using the slider or `⏪`/`⏩` buttons in this panel to change the displayed image will **snap the `Scene` panel's camera** to that image's recorded pose.
    *   If `Split dataset for evaluation` was checked, an `eval` tab appears here displaying evaluation metrics (PSNR, SSIM, LPIPS) calculated periodically based on the `Evaluate every ... steps` setting.
*   **Pausing/Stopping:**
    *   **Pausing:** Use the `⏸ paused` button in the **`Scene`** panel controls.
    *   **Stopping:** To permanently stop training for the current session, **close the Brush application window**. Progress may be saved via automatic checkpoints if configured.

## Exporting Results

Brush offers two ways to export the trained Gaussian splat data as `.ply` files:

*   **Manual Export (Current View):**
    *   While training or viewing, click the **`⬆ Export`** button located below the **`Scene`** panel.
    *   This opens a native "Save File" dialog.
    *   It saves the splat data *currently being displayed* in the Scene view.
*   **Automatic Periodic Export (Checkpoints):**
    *   Configure this in **`Settings -> Process Settings`** using the `Export every ... steps` slider (set > 0 to enable).
    *   Requires the desktop application.
    *   Saves checkpoint `.ply` files periodically during training.
    *   **Location:** Saves to the *current working directory* by default, or to the path specified by the `--export-path` CLI argument.
    *   **Naming:** Uses `splats_{iter}.ply` by default, customizable via `--export-name`.

## Next Steps

*   Once training is complete (or stopped) and you have `.ply` files, **view the results** following the [Viewing Scenes](./viewing-scenes.md) guide.
*   To understand **command-line alternatives** for loading and training, see the [CLI Usage Guide](./cli-usage.md).
*   For a detailed explanation of **all configuration parameters**, refer to the [Configuration Options Reference](../reference/config-options.md).
*   For technical details on the **algorithms involved**, explore the [Training and Rendering Pipeline](../development/training-and-rendering.md) documentation.
*   To learn about how the **user interface is built**, read the [UI Development Guide](../development/ui.md).