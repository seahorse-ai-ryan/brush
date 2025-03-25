# Troubleshooting and FAQs 🛠️

This page provides solutions to common issues encountered when using Brush and answers to frequently asked questions.

## 🔍 Common Issues and Solutions

### Building and Installation

#### ❓ Problem: `cargo build` or `cargo run` fails with dependency errors

**Solution:**
- Ensure you're using Rust 1.82+ (`rustc --version`)
- Run `cargo clean` and then try building again
- Check if all dependencies match those specified in the Cargo.toml files
- Run `cargo update` to update dependencies to latest compatible versions

#### ❓ Problem: WebGPU-related build errors

**Solution:**
- For building web applications, ensure you have a recent version of Trunk installed (`trunk --version`)
- If building fails with WebGPU errors, make sure your GPU drivers are up to date
- On Linux, ensure you have the required development libraries (`libxcb-xfixes0-dev`, `libxkbcommon-dev`, etc.)

#### ❓ Problem: Android build fails

**Solution:**
- Verify you have the Android SDK and NDK installed correctly
- Ensure you've followed the detailed instructions in `crates/brush-android/README.md`
- Check for version mismatches between Android SDK tools

### Runtime Issues

#### ❓ Problem: Application crashes with GPU-related errors

**Solution:**
- Check if your GPU supports WebGPU (visit `chrome://gpu` in Chrome)
- Update your GPU drivers to the latest version
- For NVIDIA cards, make sure you're using the proprietary drivers, not open-source alternatives
- Check available GPU memory if you're processing large datasets

#### ❓ Problem: Training is extremely slow

**Solution:**
- Check if you're running in debug mode (`--release` flag is recommended for performance)
- Reduce the number of iterations or batch size if memory is constrained
- Ensure your GPU is being utilized (use tools like `nvidia-smi` or Task Manager)
- Consider using a smaller dataset for initial testing

#### ❓ Problem: Poor reconstruction quality

**Solution:**
- Check if your input images are properly posed by COLMAP
- Ensure sufficient overlap between images in your dataset
- Try using masks for complex scenes with moving objects or reflections
- Experiment with different training hyperparameters

#### ❓ Problem: Web viewer doesn't work

**Solution:**
- Ensure you're using Chrome 131+ or another browser that supports WebGPU
- Enable the "Unsafe WebGPU support" flag in Chrome if necessary
- Check the browser console for specific error messages
- Try with smaller datasets if experiencing performance issues

## 📋 Frequently Asked Questions

### General Questions

#### ❓ What is Brush?

Brush is a cross-platform 3D reconstruction engine using Gaussian splatting. It works on macOS, Windows, Linux, Android, and in browsers, leveraging WebGPU and the Burn machine learning framework.

#### ❓ What types of data can Brush process?

Brush works with posed image data. It can load COLMAP data or datasets in the Nerfstudio format with a transforms.json file. It also supports masking images either through transparency or with separate mask files.

#### ❓ Can Brush run on my GPU?

Brush is designed to work on a wide range of GPUs from NVIDIA, AMD, and Intel. As long as your GPU supports WebGPU (or can be used with wgpu in Rust), it should work. Performance will vary based on GPU capabilities.

### Training Questions

#### ❓ What's the minimum number of images needed for a good reconstruction?

For good quality reconstructions, at least 30-50 images with good overlap are recommended. More complex scenes may require 100+ images for best results.

#### ❓ How long does training typically take?

Training time varies greatly depending on hardware, dataset size, and desired quality. On a modern GPU (e.g., RTX 4070), a typical scene might take 20-40 minutes for training 30K iterations.

#### ❓ Can I continue or resume training from a checkpoint?

Yes, Brush supports loading existing models and continuing training from where you left off. Use the appropriate CLI command or UI option to load an existing model.

#### ❓ How do I know when training is complete?

Brush will display training metrics like PSNR and loss values. When these values plateau, training is typically complete. You can also visually inspect the reconstruction quality.

### Viewing and Export

#### ❓ What file formats can Brush export to?

Brush primarily works with .ply files for 3D Gaussian representation. It can also export various visualization formats through Rerun.

#### ❓ Can I view Brush models in other software?

The .ply files exported by Brush contain special attributes for Gaussian splatting that may not be fully compatible with standard 3D viewers. It's best to use Brush or other Gaussian splatting viewers to visualize the models.

#### ❓ How do I reduce the size of the output model?

You can use Brush's pruning capabilities to reduce the number of Gaussians while maintaining visual quality. This is accessible through CLI commands or the UI during/after training.

### Development and Contribution

#### ❓ How can I contribute to Brush?

Contributions are welcome! You can fork the repository, make your changes, and submit a pull request. Make sure to follow the development workflow guidelines and ensure your code passes all tests.

#### ❓ Where can I get help if I'm stuck?

Join the Brush Discord channel for community support, check GitHub Discussions, or review the existing issues on GitHub to see if others have encountered similar problems.

## 🔄 Error Messages and Their Meanings

| Error Message | Likely Cause | Solution |
|---------------|--------------|----------|
| `Error: Failed to create WebGPU adapter` | Browser doesn't support WebGPU or flag not enabled | Use Chrome 131+ or enable WebGPU flag |
| `Error: Out of memory` | Dataset too large for available GPU memory | Reduce dataset size or use a GPU with more memory |
| `Error: Failed to load dataset` | Incorrect dataset format or missing files | Check dataset structure and file paths |
| `Error: COLMAP data not found` | Missing camera or points3D files | Ensure COLMAP successfully processed your images |
| `Error: Wgpu surface creation failed` | Display or GPU compatibility issues | Update drivers or try different rendering backend |

If you encounter an issue not listed here, please check the [GitHub Issues](https://github.com/ArthurBrussee/brush/issues) or report it if it's new. 