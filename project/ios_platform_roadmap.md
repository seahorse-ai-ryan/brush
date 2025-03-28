# iOS Platform Roadmap

This document outlines the planned implementation of Brush for iOS. This platform is currently in the early planning stages and is not yet implemented. This document serves as a roadmap for future development.

## System Requirements and Dependencies

The iOS implementation will target:
- iOS 14.0 and above
- iPadOS 14.0 and above
- Devices with A12 Bionic chip (or newer) for acceptable ML performance

Development prerequisites will include:
- macOS 11.0+
- Xcode 13.0+
- Rust with iOS targets
- Swift 5.5+

## Architecture

### Integration Approach

The iOS implementation will use a hybrid architecture:

```
+-------------------+
|    iOS UI Layer   |
|     (SwiftUI)     |
+-------------------+
         |
+-------------------+
|  Swift Bindings   |
+-------------------+
         |
+-------------------+
|    Rust Core      |
|  (cross-platform) |
+-------------------+
```

### Native Integration

Integration with the Rust core will be done using Swift binding approaches:

```swift
import Brush

// Example Swift interface to Brush
struct BrushInterface {
    // Initialize the Brush engine
    static func initialize(withConfig config: BrushConfig) -> Result<BrushEngine, BrushError> {
        let configPtr = createConfig(
            maxSplatCount: config.maxSplatCount,
            initialViewDistance: config.initialViewDistance,
            optimizationLevel: config.optimizationLevel.rawValue
        )
        
        let enginePtr = initializeEngine(configPtr)
        if enginePtr != nil {
            return .success(BrushEngine(pointer: enginePtr!))
        } else {
            return .failure(BrushError.initializationFailed)
        }
    }
}

// Swift model representing configuration
struct BrushConfig {
    var maxSplatCount: Int32 = 1_000_000
    var initialViewDistance: Float = 5.0
    var optimizationLevel: OptimizationLevel = .balanced
    
    enum OptimizationLevel: Int32 {
        case performance = 0
        case balanced = 1
        case quality = 2
    }
}
```

## Metal Integration

Brush will leverage Metal for GPU acceleration:

```swift
import Metal

class MetalRenderer {
    private var device: MTLDevice
    private var commandQueue: MTLCommandQueue
    private var library: MTLLibrary
    private var pipelineState: MTLRenderPipelineState
    
    init() throws {
        // Get the default Metal device
        guard let device = MTLCreateSystemDefaultDevice() else {
            throw RenderError.deviceNotFound
        }
        self.device = device
        
        // Create the command queue
        guard let commandQueue = device.makeCommandQueue() else {
            throw RenderError.commandQueueCreationFailed
        }
        self.commandQueue = commandQueue
        
        // Load Metal shader functions from the default library
        guard let library = device.makeDefaultLibrary() else {
            throw RenderError.libraryCreationFailed
        }
        self.library = library
        
        // Create the render pipeline
        let vertexFunction = library.makeFunction(name: "vertexShader")
        let fragmentFunction = library.makeFunction(name: "fragmentShader")
        
        let pipelineDescriptor = MTLRenderPipelineDescriptor()
        pipelineDescriptor.vertexFunction = vertexFunction
        pipelineDescriptor.fragmentFunction = fragmentFunction
        
        // Configure pipeline format
        pipelineDescriptor.colorAttachments[0].pixelFormat = .bgra8Unorm
        
        // Create the pipeline state
        self.pipelineState = try device.makeRenderPipelineState(descriptor: pipelineDescriptor)
    }
    
    func render(splats: UnsafePointer<Splat>, count: Int, viewMatrix: matrix_float4x4, projectionMatrix: matrix_float4x4, to drawable: CAMetalDrawable) {
        // Create a command buffer for rendering
        guard let commandBuffer = commandQueue.makeCommandBuffer() else {
            return
        }
        
        // Create a render pass descriptor
        let renderPassDescriptor = MTLRenderPassDescriptor()
        renderPassDescriptor.colorAttachments[0].texture = drawable.texture
        renderPassDescriptor.colorAttachments[0].loadAction = .clear
        renderPassDescriptor.colorAttachments[0].clearColor = MTLClearColor(red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0)
        renderPassDescriptor.colorAttachments[0].storeAction = .store
        
        // Create an encoder for the render pass
        guard let encoder = commandBuffer.makeRenderCommandEncoder(descriptor: renderPassDescriptor) else {
            return
        }
        
        // Set the pipeline state
        encoder.setRenderPipelineState(pipelineState)
        
        // Set the buffers
        let splatBuffer = device.makeBuffer(bytes: splats, length: count * MemoryLayout<Splat>.stride, options: .storageModeShared)
        encoder.setVertexBuffer(splatBuffer, offset: 0, index: 0)
        
        // Set the uniforms
        var uniforms = Uniforms(viewMatrix: viewMatrix, projectionMatrix: projectionMatrix)
        encoder.setVertexBytes(&uniforms, length: MemoryLayout<Uniforms>.size, index: 1)
        
        // Draw the splats
        encoder.drawPrimitives(type: .point, vertexStart: 0, vertexCount: count)
        
        // End encoding
        encoder.endEncoding()
        
        // Present the drawable
        commandBuffer.present(drawable)
        
        // Commit the command buffer
        commandBuffer.commit()
    }
}
```

## Core ML Integration

For ML acceleration on iOS devices:

```swift
import CoreML

class MLAccelerator {
    private var model: MLModel
    
    init() throws {
        // Load the Core ML model
        let modelURL = Bundle.main.url(forResource: "BrushModel", withExtension: "mlmodel")!
        self.model = try MLModel(contentsOf: modelURL)
    }
    
    func processFeatures(input: [Float]) throws -> [Float] {
        // Create model input
        let inputFeatures = try MLMultiArray(shape: [NSNumber(value: input.count)], dataType: .float32)
        for (i, value) in input.enumerated() {
            inputFeatures[i] = NSNumber(value: value)
        }
        
        // Create model input dictionary
        let inputDict = ["features": MLFeatureValue(multiArray: inputFeatures)]
        let input = try MLDictionary(dictionary: inputDict)
        
        // Perform inference
        let output = try model.prediction(from: input)
        
        // Extract and return output
        let outputFeatures = output.featureValue(for: "output")!.multiArrayValue!
        var result = [Float](repeating: 0.0, count: outputFeatures.count)
        
        for i in 0..<outputFeatures.count {
            result[i] = outputFeatures[i].floatValue
        }
        
        return result
    }
}
```

## SwiftUI Integration

The user interface will be built with SwiftUI:

```swift
import SwiftUI

struct BrushView: UIViewRepresentable {
    @EnvironmentObject var brushViewModel: BrushViewModel
    
    func makeUIView(context: Context) -> MTKView {
        let mtkView = MTKView()
        mtkView.delegate = brushViewModel.renderer
        mtkView.device = brushViewModel.renderer.device
        mtkView.colorPixelFormat = .bgra8Unorm
        mtkView.depthStencilPixelFormat = .depth32Float
        mtkView.clearColor = MTLClearColor(red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0)
        mtkView.enableSetNeedsDisplay = false
        mtkView.isPaused = false
        
        // Set up gesture recognizers
        let panGesture = UIPanGestureRecognizer(target: context.coordinator, action: #selector(Coordinator.handlePan(_:)))
        mtkView.addGestureRecognizer(panGesture)
        
        let pinchGesture = UIPinchGestureRecognizer(target: context.coordinator, action: #selector(Coordinator.handlePinch(_:)))
        mtkView.addGestureRecognizer(pinchGesture)
        
        return mtkView
    }
    
    func updateUIView(_ uiView: MTKView, context: Context) {
        // Update view if needed
    }
    
    func makeCoordinator() -> Coordinator {
        Coordinator(self)
    }
    
    class Coordinator: NSObject {
        var parent: BrushView
        
        init(_ parent: BrushView) {
            self.parent = parent
        }
        
        @objc func handlePan(_ gesture: UIPanGestureRecognizer) {
            let translation = gesture.translation(in: gesture.view)
            parent.brushViewModel.rotate(dx: Float(translation.x), dy: Float(translation.y))
            gesture.setTranslation(.zero, in: gesture.view)
        }
        
        @objc func handlePinch(_ gesture: UIPinchGestureRecognizer) {
            parent.brushViewModel.zoom(factor: Float(gesture.scale))
            gesture.scale = 1.0
        }
    }
}
```

## User Interface Features

The iOS implementation will include features like:

- Touch-based camera controls
- Apple Pencil support for precise interaction
- iCloud Drive integration for file storage
- Sharing capabilities via Share Sheet
- Split View and Slide Over support on iPad

## Performance Optimizations

iOS-specific performance optimizations will include:

- Core ML acceleration for neural networks
- Metal Performance Shaders for compute-intensive tasks
- Thermal throttling detection and management
- Low-memory handling
- Background task suspension

## Timeline

The iOS implementation is planned in several phases:

1. **Phase 1** (3-6 months):
   - Basic SwiftUI interface
   - Metal rendering pipeline
   - Simple file loading

2. **Phase 2** (6-9 months):
   - Core ML integration
   - Touch controls
   - iCloud Drive integration

3. **Phase 3** (9-12 months):
   - Performance optimizations
   - Advanced UI features
   - Apple Pencil support

## Development Guidelines

Development will follow iOS best practices:

- Swift Style Guide adherence
- Testable architecture
- Support for Dark Mode
- Accessibility features
- Battery efficiency

## Related Documentation

- [Cross-Platform Framework](/docs/cross_platform_framework.md) - Current cross-platform approach 