# Platform Expansion Roadmap

This document outlines plans for expanding Brush support to additional platforms beyond the currently supported environments. These expansions are in various stages of planning and development.

## Future Platform Support

Brush's architecture is designed to be extensible to new platforms as WebGPU and related technologies become more widely available. The following platforms are being explored for future support:

### 1. WebGPU Native

As WebGPU becomes available natively on more platforms, Brush will be extended to leverage these implementations:

- **Native WebGPU on Windows** - Integration with DirectX 12 backend
- **Native WebGPU on macOS** - Enhanced Metal backend support
- **Native WebGPU on Linux** - Improved Vulkan integration
- **Chrome OS** - Optimized for Chromebook hardware

Benefits of native WebGPU support include:
- Improved performance over browser-based WebGPU
- Access to more system resources
- Better integration with platform-specific features
- More consistent behavior across implementations

### 2. Game Consoles

Potential support for game consoles with WebGPU-compatible APIs:

- **PlayStation** - Using the PS5's advanced GPU capabilities
- **Xbox** - Leveraging DirectX integration on Xbox Series X|S
- **Nintendo Switch** - Custom implementation for portable use cases

This expansion would enable:
- Interactive 3D model viewing on gaming platforms
- Capture and reconstruction using console camera accessories
- Integration with game development workflows
- Collaborative viewing in multiplayer environments

### 3. VR/AR Platforms

Integration with virtual and augmented reality frameworks:

- **Oculus/Meta Quest** - Direct integration with Meta's VR ecosystem
- **SteamVR** - Support for various PC-connected VR headsets
- **Microsoft Mixed Reality** - Integration with Windows MR platform
- **Apple Vision Pro** - Native support for Apple's spatial computing platform

VR/AR integration will provide:
- Immersive viewing of reconstructed scenes and objects
- Natural interaction with 3D content
- Enhanced depth perception for model evaluation
- Intuitive 3D manipulation and editing

## Implementation Strategy

The approach for expanding to new platforms will follow these principles:

1. **Abstraction Layers** - Building robust platform abstraction to minimize platform-specific code
2. **Feature Parity** - Ensuring core functionality is consistent across all platforms
3. **Progressive Adoption** - Adding platforms based on user demand and market adoption
4. **Performance Optimization** - Tailoring implementations to each platform's strengths

## Timeline Considerations

Platform expansion will be prioritized based on:

- WebGPU adoption rates on target platforms
- Developer ecosystem maturity
- User demand for specific platforms
- Technical feasibility and resource requirements

## Related Documentation

For information about currently supported platforms, see the [Cross-Platform Framework](/docs/cross_platform_framework.md) documentation. 