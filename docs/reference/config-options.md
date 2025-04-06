# Configuration Options Reference

This page provides a detailed reference for the configuration parameters used in Brush, primarily defined across `TrainConfig`, `ModelConfig`, `LoadDataseConfig`, `ProcessConfig`, and `RerunConfig`.

These parameters can typically be set via:

*   **Command-Line Arguments:** When launching `brush_app` (see [CLI Usage Guide](../guides/cli-usage.md)). Flags usually follow the pattern `--parameter-name <VALUE>`.
*   **UI Controls:** Many options are exposed graphically in the `Settings` panel of the `brush_app` UI.
*   **Config Files:** Although the underlying structs derive `burn::Config` (which *can* support file loading), Brush **does not** currently implement or expose a mechanism to load these operational configurations from external files (e.g., YAML). Settings are managed via CLI flags or UI controls. <!-- TODO: Verify if Brush uses/supports config file loading --> <!-- Resolved: No explicit support found -->

Default values are indicated where known. See the [Glossary](./glossary.md) for definitions of terms like [SH](./glossary.md#spherical-harmonics-sh), [Splat](./glossary.md#splat), etc.

## Training Options (`TrainConfig`)

Controls the optimization process. Defined in `crates/brush-train/src/train.rs`.

*   `--total-steps <STEPS>`
    *   Total number of optimization steps to perform. (Default: 30000)
*   `--ssim-weight <WEIGHT>`
    *   Weight of [SSIM](./glossary.md#core-technologies) loss relative to L1 loss in the combined RGB loss. (Default: 0.2)
*   `--ssim-window-size <SIZE>`
    *   Window size used for SSIM calculation. (Default: 11)
*   `--lr-mean <RATE>`
    *   Starting learning rate for Gaussian means (position). (Default: 4e-5)
*   `--lr-mean-end <RATE>`
    *   Target ending learning rate for Gaussian means (reached via exponential decay over `total_steps`). (Default: 4e-7)
*   `--mean-noise-weight <WEIGHT>`
    *   Controls the amount of noise added to the means of low-opacity Gaussians during the growth phase (until `--growth-stop-iter`) to encourage exploration. Higher values add more noise. (Default: 1e4)
*   `--lr-coeffs-dc <RATE>`
    *   Learning rate for the base (DC, 0th order) [Spherical Harmonics (SH)](./glossary.md#3d-reconstruction-rendering) coefficients (main color). (Default: 3e-3)
*   `--lr-coeffs-sh-scale <SCALE>`
    *   Factor to divide the base learning rate (`--lr-coeffs-dc`) by for higher-order (degree > 0) SH coefficients. Lower values train higher-order coefficients faster. (Default: 20.0)
*   `--lr-opac <RATE>`
    *   Learning rate for the raw opacity parameter (before sigmoid). (Default: 3e-2)
*   `--lr-scale <RATE>`
    *   Starting learning rate for log-scale parameters. (Default: 1e-2)
*   `--lr-scale-end <RATE>`
    *   Target ending learning rate for log-scale parameters (via exponential decay). (Default: 6e-3)
*   `--lr-rotation <RATE>`
    *   Learning rate for rotation quaternion parameters. (Default: 1e-3)
*   `--opac-loss-weight <WEIGHT>`
    *   Weight for the opacity regularization loss, encouraging transparency. Applied more strongly early in training. (Default: 1e-8)
*   `--refine-every <STEPS>`
    *   Frequency (in steps) to perform density control (pruning and densification). (Default: 150)
*   `--growth-grad-threshold <THRESHOLD>`
    *   View-space positional gradient magnitude threshold. Gaussians exceeding this are candidates for densification. (Default: 0.00085)
*   `--growth-select-fraction <FRACTION>`
    *   Fraction of densification candidates (those above threshold) that are actually cloned/split. (Default: 0.1)
*   `--growth-stop-iter <STEP>`
    *   Training step after which densification (cloning/splitting) stops. Pruning may continue. (Default: 12500)
*   `--match-alpha-weight <WEIGHT>`
    *   Weight applied to the L1 loss on the alpha channel if input images have transparency (and are not just masks). (Default: 0.1)
*   `--max-splats <COUNT>`
    *   Target maximum number of [Splats](./glossary.md#3d-reconstruction-rendering). The densification process will attempt not to exceed this limit. (Default: 10,000,000)
    *   > **Note:** The UI slider for this setting currently limits the range to 1,000,000 - 10,000,000.

## Model Options (`ModelConfig`)

Defines structural properties of the Gaussian Splatting model. Defined in `crates/brush-dataset/src/lib.rs`.

*   `--sh-degree <DEGREE>`
    *   Degree of [Spherical Harmonics (SH)](./glossary.md#3d-reconstruction-rendering) used for representing view-dependent color. Higher values (max 4 supported by some components) capture more detail but increase memory/compute. (Default: 3)

## Dataset Load Options (`LoadDataseConfig`)

Controls how input data is loaded and processed. Defined in `crates/brush-dataset/src/lib.rs`.

*   `--max-frames <COUNT>`
    *   Load only the first N frames (views) from the dataset sequence.
*   `--max-resolution <PIXELS>`
    *   Resize loaded images so their longest dimension does not exceed this value. Affects memory usage and detail. (Default: 1920)
*   `--eval-split-every <N>`
    *   If set, reserves 1 out of every N images for the evaluation set, removing them from the training set.
*   `--subsample-frames <N>`
    *   Load only every Nth frame from the dataset sequence (applied before `max-frames`).
*   `--subsample-points <N>`
    *   If loading a [COLMAP](./glossary.md#3d-reconstruction-rendering) dataset with points, load only every Nth point from the initial point cloud.

## Process Options (`ProcessConfig`)

General process control, including evaluation and export. Defined in `crates/brush-process/src/process_loop/process_args.rs`.

*   `--seed <SEED>`
    *   Seed for random number generators, used for reproducibility (e.g., in sampling, noise). (Default: 42)
*   `--eval-every <STEPS>`
    *   Frequency (in steps) to run evaluation (calculate PSNR/SSIM) on the evaluation set. Requires `--eval-split-every` to be set. (Default: 1000)
*   `--eval-save-to-disk`
    *   If set, saves the rendered images from evaluation runs to the directory specified by `--export-path`. (Default: false)
*   `--export-every <STEPS>`
    *   Frequency (in steps) to automatically export a snapshot of the trained `.ply` model. (Default: 5000)
*   `--export-path <PATH>`
    *   Directory where automatic `.ply` exports and evaluation images (if `--eval-save-to-disk` is set) are saved. Can be relative to the current working directory. (Default: current working directory)
*   `--export-name <TEMPLATE>`
    *   Filename template for automatically exported `.ply` files. The placeholder `{iter}` is replaced with the current training step count. (Default: `./export_{iter}.ply`)
*   `--start-iter <ITER>`
    *   Iteration step count to *begin* training from. Primarily affects learning rate schedules and timing for density control logic (e.g., relative to `--growth-stop-iter`). (Default: 0)
    *   > **Note:** This flag sets the initial *step number*. Resuming training requires providing the saved `.ply` checkpoint as the input `DataSource` when launching Brush. <!-- Resolved: Requires providing PLY + start_iter -->

## Rerun Options (`RerunConfig`)

Controls logging to the [Rerun](./glossary.md#core-technologies) visualization tool. Requires building Brush with the `--features=rerun` flag. Defined in `crates/brush-process/src/process_loop/process_args.rs`.

*   `--rerun-enabled`
    *   Enable connection and logging to a running Rerun viewer instance. (Default: false)
*   `--rerun-log-train-stats-every <STEPS>`
    *   Frequency (in steps) to log scalar training statistics (e.g., loss, learning rates, step time) to Rerun. (Default: 50)
*   `--rerun-log-splats-every <STEPS>`
    *   Frequency (in steps) to log the entire set of 3D Gaussian splats (position, color, scale, etc.) to Rerun. Can be performance-intensive.
*   `--rerun-max-img-size <PIXELS>`
    *   Maximum size (width or height) for dataset images logged to Rerun during evaluation runs. (Default: 512)

## Next Steps

*   See how these options are used in practice in the [Training a Scene Guide](../guides/training-a-scene.md).
*   Learn how to set these options via the command line in the [CLI Usage Guide](../guides/cli-usage.md).
*   Understand the algorithms these parameters control in the [Training and Rendering Pipeline](../development/training-and-rendering.md) document. 