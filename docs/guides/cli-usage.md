# Using the Command Line Interface (CLI)

While Brush offers an interactive UI via `brush_app`, it also incorporates command-line arguments to control its behavior, allowing for headless operation (running without the GUI) or specifying initial settings when launching the app.

**Note:** The CLI arguments are processed by the main `brush_app` executable. There isn't a separate dedicated CLI binary.

## Basic Usage

The general format is:

```bash
# If running from source:
cargo run --bin brush_app --release -- [OPTIONS] [DATA_SOURCE]

# If using a pre-built binary:
./brush_app [OPTIONS] [DATA_SOURCE]
```

*   `[OPTIONS]` are the flags described below.
*   `[DATA_SOURCE]` is an optional positional argument specifying the data to load immediately. It can be:
    *   A path to a local `.ply` file (for viewing).
    *   A path to a local directory containing a dataset (COLMAP or Nerfstudio format) for training.
    *   A path to a local `.zip` archive containing a dataset for training.
    *   A URL to a `.ply` file or a `.zip` archive containing a dataset.

## Key Options

Here are the primary command-line arguments, grouped by category. For a full, definitive list, run:

```bash
cargo run --bin brush_app --release -- --help
```

(This uses the standard `clap` library convention for displaying help.)

### Training Options

Controls the optimization process.

*   `--total-steps <STEPS>`
    *   Total number of steps to train for. (Default: 30000)
*   `--ssim-weight <WEIGHT>`
    *   Weight of SSIM loss (relative to L1 loss). (Default: 0.2)
*   `--ssim-window-size <SIZE>`
    *   Window size for SSIM calculation. (Default: 11)
*   `--lr-mean <RATE>`
    *   Starting learning rate for Gaussian means. (Default: 4e-5)
*   `--lr-mean-end <RATE>`
    *   Ending learning rate for Gaussian means (via exponential decay). (Default: 4e-7)
*   `--lr-coeffs-dc <RATE>`
    *   Learning rate for the base SH (DC) coefficients (color). (Default: 3e-3)
*   `--lr-coeffs-sh-scale <SCALE>`
    *   Factor to divide learning rate by for higher-order SH coefficients. (Default: 20.0)
*   `--lr-opac <RATE>`
    *   Learning rate for opacity. (Default: 3e-2)
*   `--lr-scale <RATE>`
    *   Starting learning rate for scale. (Default: 1e-2)
*   `--lr-scale-end <RATE>`
    *   Ending learning rate for scale (via exponential decay). (Default: 6e-3)
*   `--lr-rotation <RATE>`
    *   Learning rate for rotation. (Default: 1e-3)
*   `--opac-loss-weight <WEIGHT>`
    *   Weight for opacity regularization loss (encourages transparency). (Default: 1e-8)

### Refinement Options (Part of Training)

Controls how Gaussians are added (densified) and removed (pruned) during training.

*   `--refine-every <STEPS>`
    *   Frequency (in steps) for refinement operations. (Default: 150)
*   `--growth-grad-threshold <THRESHOLD>`
    *   Gradient magnitude threshold to trigger splat growth/cloning. (Default: 0.00085)
*   `--growth-select-fraction <FRACTION>`
    *   Fraction of splats exceeding the gradient threshold that are selected for growth. (Default: 0.1)
*   `--growth-stop-iter <STEP>`
    *   Step after which splat growth (cloning/splitting) stops. (Default: 12500)
*   `--match-alpha-weight <WEIGHT>`
    *   Weight of L1 loss on alpha channel if input images have transparency. (Default: 0.1)
*   `--max-splats <COUNT>`
    *   Target maximum number of splats. (Default: 10,000,000)

### Model Options

Defines structural properties of the Gaussian Splatting model.

*   `--sh-degree <DEGREE>`
    *   Spherical Harmonics degree (0-4). Higher values capture more view-dependent effects. (Default: 3)

### Dataset Load Options

Controls how input data is loaded and processed.

*   `--max-frames <COUNT>`
    *   Load only the first N frames from the dataset.
*   `--max-resolution <PIXELS>`
    *   Resize images to have at most this many pixels on the longer side. (Default: 1920)
*   `--eval-split-every <N>`
    *   Use 1 out of every N images for the evaluation set, instead of training.
*   `--subsample-frames <N>`
    *   Load only every Nth frame from the dataset sequence.
*   `--subsample-points <N>`
    *   Load only every Nth point from the initial SfM point cloud (if applicable).

### Process Options

General process control, including evaluation and export.

*   `--seed <SEED>`
    *   Random seed for reproducibility. (Default: 42)
*   `--eval-every <STEPS>`
    *   Run evaluation every N training steps. (Requires `--eval-split-every` to be set). (Default: 1000)
*   `--eval-save-to-disk`
    *   Save rendered evaluation images to disk. Uses `--export-path`.
*   `--export-every <STEPS>`
    *   Automatically export a `.ply` snapshot every N training steps. (Default: 5000)
*   `--export-path <PATH>`
    *   Directory to save exported `.ply` files and evaluation images. (Default: current working directory)
*   `--export-name <TEMPLATE>`
    *   Filename template for exported `.ply` files. `{iter}` is replaced with the step count. (Default: `./export_{iter}.ply`)
*   `--start-iter <ITER>`
    *   Iteration step count to *begin* training from. Affects learning rate schedules and refinement logic timing. (Default: 0)
    *   > **Note:** To resume training from a saved state, you must provide the corresponding exported `.ply` file as the `DATA_SOURCE` argument *in addition* to setting `--start-iter` to the step count at which that `.ply` was saved. <!-- TODO: Verify checkpoint loading mechanism and format --> <!-- Resolved: Requires providing PLY + start_iter -->

### Rerun Options (Requires building with `--features=rerun`)

Controls logging to the [Rerun](https://www.rerun.io/) visualization tool.

*   `--rerun-enabled`
    *   Enable logging to the Rerun visualization server.
*   `--rerun-log-train-stats-every <STEPS>`
    *   Log basic training stats to Rerun every N steps. (Default: 50)
*   `--rerun-log-splats-every <STEPS>`
    *   Log the full splat point cloud to Rerun every N steps (can be slow/memory intensive).
*   `--rerun-max-img-size <PIXELS>`
    *   Maximum size (width or height) for dataset images logged to Rerun. (Default: 512)

## Examples

*   **View a local PLY file:**
    ```bash
    cargo run --bin brush_app --release -- ./path/to/your/model.ply
    ```

*   **Start training a local dataset for 10,000 steps:**
    ```bash
    cargo run --bin brush_app --release -- --total-steps 10000 ./path/to/your/dataset_dir
    ```

*   **Train from a URL, exporting every 1000 steps, with lower SH degree:**
    ```bash
    cargo run --bin brush_app --release -- --sh-degree 1 --export-every 1000 --total-steps 30000 https://example.com/datasets/my_scene.zip
    ```

*   **Train with Rerun enabled:**
    ```bash
    cargo run --bin brush_app --release --features=rerun -- --rerun-enabled ./path/to/dataset
    ```

*   **Run headless (no UI):**
    > **Note:** Providing a `DATA_SOURCE` positional argument implicitly sets `--with-viewer` to false (unless explicitly overridden with `--with-viewer true`). This allows the processing (training or viewing-to-export) to run without launching the interactive GUI window. <!-- Resolved: Triggered by providing SOURCE argument -->
    ```bash
    cargo run --bin brush_app --release -- --total-steps 5000 ./path/to/dataset
    ```

*   **Set multiple options (lower resolution, fewer steps, specific export path):**
    ```bash
    cargo run --bin brush_app --release -- \
      --max-resolution 1024 \
      --total-steps 15000 \
      --export-path ./my_training_run/ \
      --export-name "scene_step_{iter}.ply" \
      ./path/to/dataset
    ```

## Next Steps

*   See the full list of options in the [Configuration Options Reference](../reference/config-options.md).
*   Learn how to use these options when [Training a Scene](./training-a-scene.md). 