# Training a Scene

This guide walks through training a new 3D Gaussian Splatting scene from your own data using Brush.

> **Prerequisites:**
>
> *   The Brush application installed or available to run via CLI (see [Installing Brush](./installing-brush.md)).
> *   A dataset in a supported format (COLMAP directory, Nerfstudio JSON with images, or a `.zip` of either).

> **Warning: Web Training** ⚠️
> Training via the web browser interface is **not recommended** due to significantly lower performance and potential failures ([Burn #2901](https://github.com/tracel-ai/burn/issues/2901)). Use the desktop application or CLI for training.

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
    *   For a detailed explanation of all configuration parameters, refer to the [Configuration Options Reference](../reference/config-options.md).
        *   **Model Settings:** `Spherical Harmonics Degree`, `Max image resolution`, `Max Splats`.
        *   **Load Settings:** `Limit max frames`, `Split dataset for evaluation` (if checked, enables the `eval` tab in the Dataset panel).
        *   **Training Settings:** `Train ... steps` (sets the target number of training iterations).
        *   **Process Settings:** `Evaluate every ... steps`, `Export every ... steps` (enables automatic periodic export, see step 6 below).
        *   **Rerun Settings:** (See Step 4 below if using Rerun visualization).

4.  **(Optional) Setup Rerun Visualization:**

    Brush can send detailed training data to the **Rerun Viewer**, which runs as a **separate application**. This allows for powerful, time-scrubbing visualization of the training process but requires specific setup:

    1.  **Install Viewer:** Ensure the Rerun Viewer application is installed via `cargo install rerun-cli`. The Python `rerun-sdk` is **not** required for this Rust workflow.
    2.  **Launch Viewer First:** *Before* starting the training process in Brush, the Rerun Viewer application must be running. The recommended way is to launch it from your terminal (in the Brush project root) with the blueprint pre-loaded:
        ```bash
        rerun ./brush_blueprint.rbl &
        ```
    3.  **Run Brush with Feature:** Launch Brush with the feature enabled (`cargo run --bin brush_app --features=rerun`).
    4.  **Enable Logging in Brush UI:** In Brush's `Settings -> Rerun Settings`, check `Enable rerun` and set the desired logging frequencies (e.g., `Log train stats every: 50`, `Visualize splats every: 500`).
    5.  **Notes:**
        *   Data will appear in the separate Rerun Viewer window after training starts.
        *   Plots/eval data may take ~30 seconds or until the first evaluation cycle to populate.
        *   The blueprint layout works best if **`Split dataset for evaluation`** is enabled in Brush settings.

    **Rerun Visualization Example:**
    <video src="https://github.com/user-attachments/assets/f679fec0-935d-4dd2-87e1-c301db9cdc2c" controls width="100%"></video>
    *Rerun viewer showing detailed training visualization for the LEGO dataset.*

5.  **Monitor Training**

    Once a dataset is loaded and settings are configured, click the `⏵ training` button below the Scene view (if it doesn't start automatically). You can monitor and control it using the UI panels:

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

6.  **Export Results**

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
    *   **Naming:** Uses `export_{iter}.ply` by default, customizable via `--export-name`.

7.  **Next Steps**

    *   Once training is complete (or stopped) and you have `.ply` files, **view the results** following the [Viewing Scenes](./viewing-scenes.md) guide.
    *   To understand **command-line alternatives** for loading and training, see the [CLI Usage Guide](./cli-usage.md).
    *   For technical details on the **algorithms involved**, explore the [Training and Rendering Pipeline](../development/training-and-rendering.md) documentation.
    *   To learn about how the **user interface is built**, read the [UI Development Guide](../development/ui.md).