# Training a Scene

This guide walks through the process of training a new 3D Gaussian Splatting scene from your own data using the Brush desktop application (`brush_app`).

> **Prerequisites** ⚙️
>
> *   Brush installed (see [Installing Brush](./installing-brush.md)).
> *   A dataset suitable for training. Brush supports:
>     *   **COLMAP format:** A directory containing `images/` and `sparse/0/` subdirectories from a COLMAP reconstruction.
>     *   **Nerfstudio format (Synthetic NeRF):** A directory containing `images/` and a `transforms.json` file.
>     *   Datasets packaged as `.zip` archives containing either of the above structures.

## Steps

1.  **Launch Brush:** Run the `brush_app` executable (or use `cargo run --bin brush_app --release` if built from source).
    *(The initial UI displays placeholder text and default settings)*
    ![Initial Brush desktop application UI on macOS](../media/Brush_desktop_macos.png)

    *(The UI during training looks like this)*
    ![Brush desktop application showing panels during training](../media/Brush_training_room_scene.png)

2.  **Load Your Dataset:**
    *   Use the **`Settings`** or **`Presets`** tab in the left-hand panel:
        *   **`Settings` Tab:**
            *   Click **`Load file`** to select a `.zip` archive containing your dataset.
            *   Click **`Load directory`** (Desktop only) to select the root folder of your COLMAP or Nerfstudio dataset.
            *   Enter a URL to a `.zip` file in the text box and click **`Load URL`**.
        *   **`Presets` Tab:**
            *   Click on the name of one of the listed example datasets (e.g., `bicycle`, `lego`) to load it automatically.
    *   > **Note:** A "Loading..." message will appear in the **`Scene`** panel while Brush processes your data. This might take some time for large datasets.

3.  **(Optional) Adjust Settings:**
    *   Before or after loading, you can tweak parameters in the **`Settings`** tab. These correspond to [CLI options](./cli-usage.md) as well.
        *   **Model Settings:** `Spherical Harmonics Degree`, `Max image resolution`, `Max Splats`.
        *   **Load Settings:** `Limit max frames`, `Split dataset for evaluation` (if checked, enables the `eval` tab in the Dataset panel).
        *   **Training Settings:** `Train ... steps`.
        *   **Process Settings:** `Evaluate every ... steps`, `Export every ... steps` (Desktop only, saves to CWD or `--export-path`).
        *   **Rerun Settings** (Desktop only, requires `--features=rerun` build):
            *   `Enable rerun`: Connects to a running Rerun viewer.
            *   `Log train stats every`: Controls frequency of scalar logging (loss, iter/s).
            *   `Visualize splats every`: Controls frequency of logging the 3D splat point cloud (can be heavy).
            *   *(Includes a link to `rerun.io` and a note about using the `brush_blueprint.rbl` file for layout)*.
            <video src="https://github.com/user-attachments/assets/f679fec0-935d-4dd2-87e1-c301db9cdc2c" controls width="100%"></video>
            *Rerun viewer showing detailed training visualization for the LEGO dataset.*

4.  **Monitor Training:**
    *   **`