# Web Platform Enhancements

This document outlines planned enhancements for the web platform version of Brush, focusing on features that are currently in development or planned for future releases.

## Browser Storage Improvements

### IndexedDB Integration

Brush plans to implement IndexedDB for robust client-side storage in browsers, enabling:

- Persistent dataset storage between sessions
- Efficient model caching for faster loading
- User preference storage
- Training progress persistence

The planned implementation includes:

```rust
// Planned IndexedDB integration
enum FileSource {
    Local,
    Remote(String),
    IndexedDb(String),
}

// Example planned implementation for IndexedDB storage
async fn store_to_indexed_db(key: &str, data: &[u8]) -> Result<(), Error> {
    // Initialize IndexedDB
    let window = web_sys::window().ok_or(Error::NoWindow)?;
    let idb_factory = window.indexed_db().map_err(|_| Error::IndexedDbNotSupported)?;
    
    // Open database
    let db_name = "brush_storage";
    let open_request = idb_factory.open(db_name).map_err(|_| Error::IndexedDbOpenFailed)?;
    
    // Create store if needed and store data
    // ...
    
    Ok(())
}

async fn load_from_indexed_db(key: &str) -> Result<Vec<u8>, Error> {
    // Implementation to retrieve data from IndexedDB
    // ...
    Ok(Vec::new())
}
```

## WebGPU Memory Management

Future versions will include improved memory management for WebGPU contexts:

```rust
// Planned memory management improvements
fn check_memory_availability(required_bytes: usize) -> Result<(), Error> {
    // Check current memory usage and available memory
    let memory_info = web_sys::window()
        .unwrap()
        .performance()
        .unwrap()
        .memory()
        .unwrap();
    
    let used_js_heap_size = memory_info.used_js_heap_size();
    let js_heap_size_limit = memory_info.js_heap_size_limit();
    
    // Ensure enough memory is available
    if used_js_heap_size + required_bytes as f64 > js_heap_size_limit {
        return Err(Error::InsufficientMemory);
    }
    
    Ok(())
}
```

## Web Workers for Background Processing

To improve UI responsiveness, future versions will implement Web Workers:

```rust
// Planned web worker implementation
fn setup_processing_worker() -> Result<web_sys::Worker, Error> {
    let worker = web_sys::Worker::new("processing_worker.js")
        .map_err(|_| Error::WebWorkerCreationFailed)?;
    
    // Set up message handler for worker communication
    // ...
    
    Ok(worker)
}
```

## Progressive Web App (PWA) Support

Future releases will add support for Progressive Web App capabilities:

- Offline functionality
- Home screen installation
- Background sync for dataset uploads
- Notification support for long-running processes

## WebXR Integration

Integration with the WebXR API is planned to enable:

- Viewing models in AR directly from the browser
- Capturing environments using device cameras
- Interactive 3D manipulation in VR

## Related Documentation

For information about current web platform support, see the [Web Platform Guide](/docs/platform_web.md). 