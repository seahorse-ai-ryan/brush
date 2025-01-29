# Release Notes

## Unreleased

Brush 0.2 goes from a proof of concept to a tool ready for real world data! It still only implements the “basics” of Gaussian Splatting, but trains as fast as gsplat to a (slightly) higher quality than gsplat. It also comes with nicer workflows, a CLI, dynamic gaussian rendering, and lots of other new features.

The next release will focus on going beyond the basics of Gaussian Splatting, and implementing extensions that help to make Brush more robust, faster, and higher quality than other splatting alternatives. This might mean that the outputs are no longer 100% compatible with other splat viewers, so more work will also be done to make the Brush web viewer a great experience.

### Features

- Brush now measures higher PSNR/SSIM than gsplat on the mipnerf360 scenes. Of course, gsplat with some more tuned settings might reach these numbers as well, but this shows Brush is grown up now!
  - See the [results table](https://github.com/ArthurBrussee/brush?tab=readme-ov-file#results)

- Faster training overall by optimizing the kernels, fixing various slowdowns, and reducing memory use.

- Brush now has a CLI!
  - Simply run brush –help to get an overview. The basic usage is brush PATH –args.
  - Any command works with `--with-viewer` which opens the UI for easy debugging.

- Add flythrough controls supporting both orbiting, FPS controls, flythrough controls, and panning.
  - See the ‘controls’ popout in the scene view for a full overview.

- Load data from a URL. If possible the data will be streamed in, and the splat will update in real-time.
  -For a web version, just pass in ?url=

- On the web, pass in ?zen=true to enable ‘zen’ mode which makes the viewer fullscreen.

- Add support for viewing dynamic splats
  - Either loaded as a sequence of PLY files (in a folder or zip)
  - Or as a custom data format “ply with delta frames”
  - This was used for [Cat4D](https://cat-4d.github.io/) and for [Cap4D](https://felixtaubner.github.io/cap4d/)
  - Felix kindly shared [their script](https://github.com/felixtaubner/brush_avatar/) to export this data for reference.

- Open directories directly, instead of only zip files.
  - ZIP files are still supported for all operations - as this is important for the web version.

- Support transparent images.
  - Images with alpha channels will force the output splat to _match_ this transparency.
  - Alternatively, you can include a folder of ‘masks’. This will _ignore_ those parts of the image while training.

- More flexible COLMAP & nerfstudio dataset format
  - Support more of the various options, and differing file structures.
  - If your dataset has a single ply file, it will be used for the initial point cloud.

  - While training, the up-axis is rotated such that the ground is flat (thanks to @fhahlbohm)
    - Note: The exported ply will however still match your input data. I’m investigating how to best handle this in the future - either as an option to rotate the splat, or by writing metadata into the exported splat.

### Enhancements

- Add alpha_loss_weight arg to control how heavy to weigh the alpha loss
  - Nb: not applicable to masks mode
- Log memory usage to rerun while training
- Fix SH clamping values to 0 ([#76](https://github.com/ArthurBrussee/brush/pull/76) thanks to @fhahlbohm)
- Better logic to pick ‘nearest’ dataset view
- Better splat pruning logic
- Remove ESC to close
- The web version has SSIM enabled again
- Display more detailled error traces in the UI and CLI when something goes wrong
- Different method of emitting tile intersections ([#63](https://github.com/ArthurBrussee/brush/pull/63) for details)
  - Fixes some potential corruptions depending on your driver/shader compiler.
- Read up-axis from PLY file if it’s included
- Eval PSNR/SSIM now simulate a 8 bit roundtrip for fair comparison
- Add an option `--export-every` to export a ply file every so many steps
  - See `--export-path` and `--export-name` for the location of the ply
- Add an option `--eval-save-to-disk` to save eval images to disk
  - See `–export-path` for
- Add notes in CLI & UI about running in debug mode (advising to compile with  `--release`).
- Relax camera constraints, allow further zoom in/out
- Relax constraints on fields in the UI - now can enter values outside of slider range.
- Improvements to the UI, less unnecessary padding.

### Highlighted Fixes
- Dataset and scene view now match exactly 1:1
- Fix UI sometimes not updating when starting a new training run.
- Sort eval images to be consistent with the MipNeRF eval images
- Fix a crash from the KNN initialization

### Demo (Chrome only currently)

[Reference Garden scene (650MB)](https://arthurbrussee.github.io/brush-demo/?url=https://f005.backblazeb2.com/file/brush-splats-bakfiets/garden.ply&focal=1.0&zen=true)

[Mushroom I captured on a walk - only 50 images or so, a bit blurry!](https://arthurbrussee.github.io/brush-demo/?url=https://f005.backblazeb2.com/file/brush-splats-bakfiets/mushroom_centered.ply&zen=true&focal=1.5)

### Thanks

Thanks to everybody in the Brush discord, in particular @fasteinke for reporting many breakages along the way, @fhahlbohm for contributions and helping me fix my results table, @Simon.Bethke and @Gradeeterna for test data, @felixtaubner for the 4D splat export script.
