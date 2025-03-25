# Command-Line Interface (CLI) 🖥️

This document provides a comprehensive guide to Brush's command-line interface, which allows you to perform various operations without using the graphical user interface.

## Overview 🔍

The CLI module (`brush-cli`) offers a range of commands for:

- Training 3D Gaussian models
- Converting data formats
- Rendering and exporting images
- Evaluating model quality
- Manipulating datasets

## Basic Usage 📝

The basic syntax for Brush CLI commands is:

```bash
brush [OPTIONS] <COMMAND> [COMMAND_OPTIONS]
```

For help on specific commands:

```bash
brush help <COMMAND>
```

## Core Commands 🧰

### Training

```bash
# Basic training
brush train --dataset <PATH> [OPTIONS]

# Training with specific parameters
brush train --dataset <PATH> --iterations 30000 --learning-rate 0.001 --sh-degree 3
```

### Viewing

```bash
# View a PLY file
brush view --ply <PLY_FILE>

# View with custom camera settings
brush view --ply <PLY_FILE> --fov 60.0 --background-color 0.1,0.1,0.1
```

### Conversion

```bash
# Convert COLMAP to a format Brush can train on
brush convert colmap --input <COLMAP_DIR> --output <OUTPUT_DIR>

# Convert PLY file from one format to another
brush convert ply --input <INPUT_PLY> --output <OUTPUT_PLY> --format binary
```

### Evaluation

```bash
# Evaluate metrics on test set
brush eval --model <PLY_FILE> --dataset <TEST_DATA> --metrics psnr,ssim
```

## Command Reference 📖

### Global Options

| Option | Description | Default |
|--------|-------------|---------|
| `-v, --verbose` | Enable verbose output | - |
| `-q, --quiet` | Suppress most output | - |
| `--with-viewer` | Open GUI viewer during operation | - |
| `--log-level <LEVEL>` | Set log level (debug, info, warn, error) | info |
| `--config <FILE>` | Use configuration file | - |

### `train` Command

Train a 3D Gaussian model from image data.

```bash
brush train --dataset <PATH> [OPTIONS]
```

#### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--dataset <PATH>` | Path to dataset directory | - |
| `--output <DIR>` | Output directory | ./output |
| `--iterations <NUM>` | Number of training iterations | 30000 |
| `--learning-rate <RATE>` | Base learning rate | 0.001 |
| `--sh-degree <DEGREE>` | Spherical harmonics degree | 3 |
| `--batch-size <SIZE>` | Number of views per batch | 4 |
| `--resolution <WxH>` | Training resolution | dataset default |
| `--seed <NUM>` | Random seed | 42 |
| `--eval-interval <NUM>` | Evaluation interval in iterations | 1000 |
| `--save-interval <NUM>` | Model saving interval in iterations | 5000 |
| `--densify-interval <NUM>` | Gaussians density control interval | 100 |
| `--mask-path <PATH>` | Path to masks directory | - |
| `--eval-views <INDICES>` | View indices to use for evaluation | - |
| `--resume <PATH>` | Resume training from checkpoint | - |

#### Example

```bash
# Train with customized parameters
brush train \
  --dataset ./data/garden \
  --output ./models/garden \
  --iterations 50000 \
  --learning-rate 0.0005 \
  --sh-degree 4 \
  --batch-size 8 \
  --resolution 1024x768 \
  --eval-interval 2000 \
  --save-interval 10000
```

### `view` Command

View a 3D Gaussian model or PLY file in the interactive viewer.

```bash
brush view --ply <FILE> [OPTIONS]
```

#### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--ply <FILE>` | Path to PLY file | - |
| `--camera-config <FILE>` | Camera configuration file | - |
| `--fov <DEGREES>` | Field of view in degrees | 45.0 |
| `--background-color <R,G,B>` | Background color | 0,0,0 |
| `--width <PIXELS>` | Window width | 1280 |
| `--height <PIXELS>` | Window height | 720 |
| `--fullscreen` | Launch in fullscreen mode | - |
| `--screenshot <FILE>` | Save screenshot to file and exit | - |
| `--animation <FOLDER>` | Load animation frames from folder | - |

#### Example

```bash
# View model with specific settings
brush view \
  --ply ./models/garden/gaussian_model.ply \
  --fov 60.0 \
  --background-color 0.2,0.2,0.2 \
  --width 1920 \
  --height 1080
```

### `convert` Command

Convert between different data formats.

```bash
brush convert <TYPE> [TYPE_OPTIONS]
```

Where `<TYPE>` can be:
- `colmap`: Convert COLMAP format to Brush format
- `ply`: Convert between PLY formats or modify PLY files
- `nerf`: Convert NeRF dataset format to Brush format

#### COLMAP Conversion Options

| Option | Description | Default |
|--------|-------------|---------|
| `--input <DIR>` | COLMAP input directory | - |
| `--output <DIR>` | Output directory | - |
| `--image-dir <DIR>` | Custom image directory (if not in COLMAP dir) | - |
| `--image-extension <EXT>` | Image extension to look for | .jpg |
| `--scale <FACTOR>` | Scale factor for positions | 1.0 |
| `--recompute-poses` | Recompute poses from scratch | - |

#### PLY Conversion Options

| Option | Description | Default |
|--------|-------------|---------|
| `--input <FILE>` | Input PLY file | - |
| `--output <FILE>` | Output PLY file | - |
| `--format <FORMAT>` | Output format (ascii, binary) | binary |
| `--scale <FACTOR>` | Scale factor for positions | 1.0 |
| `--filter-opacity <THRESHOLD>` | Filter out Gaussians below opacity threshold | - |
| `--simplify <PERCENT>` | Simplify model to percentage of original | - |

#### Example

```bash
# Convert COLMAP data to Brush format
brush convert colmap \
  --input ./colmap_data \
  --output ./brush_dataset \
  --image-extension .png \
  --scale 0.5

# Convert and simplify PLY file
brush convert ply \
  --input ./large_model.ply \
  --output ./simplified_model.ply \
  --simplify 50 \
  --filter-opacity 0.01
```

### `eval` Command

Evaluate a trained model against ground truth data.

```bash
brush eval --model <PLY_FILE> --dataset <PATH> [OPTIONS]
```

#### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--model <FILE>` | Path to model PLY file | - |
| `--dataset <PATH>` | Path to evaluation dataset | - |
| `--metrics <LIST>` | Comma-separated metrics (psnr,ssim,lpips) | psnr,ssim |
| `--output-dir <DIR>` | Directory to save evaluation images | - |
| `--resolution <WxH>` | Evaluation resolution | dataset default |
| `--views <INDICES>` | Specific view indices to evaluate | all |
| `--save-diff` | Save difference images | - |
| `--save-side-by-side` | Save side-by-side comparison images | - |

#### Example

```bash
# Comprehensive evaluation with all metrics
brush eval \
  --model ./models/garden/gaussian_model.ply \
  --dataset ./data/garden \
  --metrics psnr,ssim,lpips \
  --output-dir ./evaluation \
  --resolution 1024x768 \
  --save-side-by-side
```

### `export` Command

Export renderings from a trained model.

```bash
brush export --model <PLY_FILE> --output <DIR> [OPTIONS]
```

#### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--model <FILE>` | Path to model PLY file | - |
| `--output <DIR>` | Output directory | - |
| `--camera-path <FILE>` | Camera path JSON file | - |
| `--frame-count <NUM>` | Number of frames to generate | 60 |
| `--resolution <WxH>` | Output resolution | 1920x1080 |
| `--format <FORMAT>` | Output format (png, jpg, mp4) | png |
| `--quality <NUM>` | Output quality (1-100 for jpg) | 95 |
| `--fps <NUM>` | Frames per second for videos | 30 |
| `--background-color <R,G,B>` | Background color | 0,0,0 |

#### Example

```bash
# Export a video with custom camera path
brush export \
  --model ./models/garden/gaussian_model.ply \
  --output ./renders \
  --camera-path ./camera_paths/orbit.json \
  --frame-count 120 \
  --resolution 1920x1080 \
  --format mp4 \
  --fps 60
```

## Configuration Files 📄

You can store common settings in a configuration file (JSON or YAML) and reference it with `--config`:

```yaml
# Example config.yaml
global:
  verbose: true
  log_level: info

train:
  iterations: 50000
  learning_rate: 0.0005
  sh_degree: 4
  batch_size: 8
  
view:
  fov: 60.0
  background_color: [0.2, 0.2, 0.2]
```

Usage:
```bash
brush --config config.yaml train --dataset ./data/garden
```

## Environment Variables 🔧

Brush CLI respects the following environment variables:

| Variable | Description |
|----------|-------------|
| `BRUSH_LOG_LEVEL` | Default log level |
| `BRUSH_CONFIG_PATH` | Default configuration file path |
| `BRUSH_OUTPUT_DIR` | Default output directory |
| `BRUSH_DEVICE` | Preferred GPU device (e.g., "0" for first GPU) |
| `BRUSH_NUM_THREADS` | Number of threads for CPU operations |

## Scripting Examples 📜

### Training Multiple Models

```bash
#!/bin/bash

# Train models for multiple datasets
DATASETS=("garden" "bicycle" "room")

for dataset in "${DATASETS[@]}"; do
  echo "Training $dataset..."
  brush train \
    --dataset "./data/$dataset" \
    --output "./models/$dataset" \
    --iterations 30000
done

echo "All training complete!"
```

### Batch Evaluation

```bash
#!/bin/bash

# Evaluate models on all test sets
MODEL_DIR="./models"
EVAL_DIR="./evaluation"
DATASETS=("garden" "bicycle" "room")

for dataset in "${DATASETS[@]}"; do
  echo "Evaluating $dataset..."
  brush eval \
    --model "$MODEL_DIR/$dataset/gaussian_model.ply" \
    --dataset "./data/$dataset" \
    --metrics psnr,ssim,lpips \
    --output-dir "$EVAL_DIR/$dataset"
done

echo "All evaluations complete!"
```

## Exit Codes 🚪

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Command line parsing error |
| 3 | I/O error |
| 4 | GPU/hardware error |
| 5 | Data processing error |

## Next Steps 🔍

- Explore the [Training Module](training_module.md) for details on training options
- Learn about the [Rendering Module](rendering_module.md) for visualization
- Understand the [Cross-Platform Framework](cross_platform_framework.md) for platform-specific considerations 