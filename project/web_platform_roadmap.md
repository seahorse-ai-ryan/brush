# Web Platform Roadmap

This document outlines planned enhancements for the web version of Brush, focusing on features that are currently in development or planned for future releases.

## Persistent Storage

### IndexedDB Integration

Future versions of Brush will implement persistent storage using the browser's IndexedDB API:

```rust
enum FileSource {
    Local,    // File selected through browser dialog
    Remote,   // File loaded from URL
    IndexedDb, // File stored in browser's IndexedDB
}

// Example function for storing data in IndexedDB
async fn store_in_indexed_db(key: &str, data: &[u8]) -> Result<(), Error> {
    let window = web_sys::window().ok_or(Error::NoWindow)?;
    let indexed_db = window.indexed_db().map_err(|_| Error::IndexedDbNotSupported)?;
    
    // Open/create database
    let open_request = indexed_db.open("brush-data", 1)?;
    
    // Create object store on first open
    let on_upgrade_needed = Closure::wrap(Box::new(move |event: web_sys::IdbVersionChangeEvent| {
        let db = event.target().unwrap().dyn_into::<web_sys::IdbDatabase>().unwrap();
        db.create_object_store("datasets").unwrap();
    }) as Box<dyn FnMut(_)>);
    
    open_request.set_onupgradeneeded(Some(on_upgrade_needed.as_ref().unchecked_ref()));
    on_upgrade_needed.forget();
    
    // Handle successful open
    let db_promise = js_sys::Promise::new(&mut |resolve, reject| {
        let on_success = Closure::wrap(Box::new(move |event: web_sys::Event| {
            let db = event.target().unwrap().dyn_into::<web_sys::IdbRequest>().unwrap()
                .result().unwrap().dyn_into::<web_sys::IdbDatabase>().unwrap();
            resolve.call1(&JsValue::NULL, &db).unwrap();
        }) as Box<dyn FnMut(_)>);
        
        let on_error = Closure::wrap(Box::new(move |event: web_sys::Event| {
            reject.call1(&JsValue::NULL, &event).unwrap();
        }) as Box<dyn FnMut(_)>);
        
        open_request.set_onsuccess(Some(on_success.as_ref().unchecked_ref()));
        open_request.set_onerror(Some(on_error.as_ref().unchecked_ref()));
        
        on_success.forget();
        on_error.forget();
    });
    
    // Wait for database to open
    let db = wasm_bindgen_futures::JsFuture::from(db_promise).await?
        .dyn_into::<web_sys::IdbDatabase>()?;
    
    // Store data
    let transaction = db.transaction_with_str_and_mode("datasets", "readwrite")?;
    let store = transaction.object_store("datasets")?;
    let data_js = js_sys::Uint8Array::from(data);
    store.put_with_key(&data_js, &JsValue::from_str(key))?;
    
    Ok(())
}

// Example function for loading data from IndexedDB
async fn load_from_indexed_db(key: &str) -> Result<Vec<u8>, Error> {
    let window = web_sys::window().ok_or(Error::NoWindow)?;
    let indexed_db = window.indexed_db().map_err(|_| Error::IndexedDbNotSupported)?;
    
    let open_request = indexed_db.open("brush-data", 1)?;
    
    // Handle successful open
    let db_promise = js_sys::Promise::new(&mut |resolve, reject| {
        let on_success = Closure::wrap(Box::new(move |event: web_sys::Event| {
            let db = event.target().unwrap().dyn_into::<web_sys::IdbRequest>().unwrap()
                .result().unwrap().dyn_into::<web_sys::IdbDatabase>().unwrap();
            resolve.call1(&JsValue::NULL, &db).unwrap();
        }) as Box<dyn FnMut(_)>);
        
        open_request.set_onsuccess(Some(on_success.as_ref().unchecked_ref()));
        on_success.forget();
    });
    
    let db = wasm_bindgen_futures::JsFuture::from(db_promise).await?
        .dyn_into::<web_sys::IdbDatabase>()?;
    
    // Read data
    let transaction = db.transaction_with_str("datasets")?;
    let store = transaction.object_store("datasets")?;
    let get_request = store.get(&JsValue::from_str(key))?;
    
    let data_promise = js_sys::Promise::new(&mut |resolve, reject| {
        let on_success = Closure::wrap(Box::new(move |event: web_sys::Event| {
            let result = event.target().unwrap().dyn_into::<web_sys::IdbRequest>().unwrap().result();
            resolve.call1(&JsValue::NULL, &result).unwrap();
        }) as Box<dyn FnMut(_)>);
        
        get_request.set_onsuccess(Some(on_success.as_ref().unchecked_ref()));
        on_success.forget();
    });
    
    let data_js = wasm_bindgen_futures::JsFuture::from(data_promise).await?;
    if data_js.is_undefined() {
        return Err(Error::FileNotFound);
    }
    
    let array = js_sys::Uint8Array::new(&data_js);
    let mut data = vec![0; array.length() as usize];
    array.copy_to(&mut data);
    
    Ok(data)
}
```

## Background Processing

### Web Worker Integration

Future versions will use Web Workers for background processing:

```rust
// Setup web worker for background processing
fn setup_web_worker() -> Result<web_sys::Worker, Error> {
    let worker = web_sys::Worker::new("worker.js")
        .map_err(|_| Error::WebWorkerCreationFailed)?;
    
    // Set up message handler
    let callback = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
        // Handle messages from worker
        let data = event.data();
        // Process data
        // ...
    }) as Box<dyn FnMut(_)>);
    
    worker.set_onmessage(Some(callback.as_ref().unchecked_ref()));
    callback.forget();
    
    Ok(worker)
}

// Send work to the worker
fn offload_task(worker: &web_sys::Worker, data: &[u8]) -> Result<(), Error> {
    let array = js_sys::Uint8Array::new_with_length(data.len() as u32);
    array.copy_from(data);
    
    worker.post_message(&array.buffer())
        .map_err(|_| Error::MessagePostFailed)?;
        
    Ok(())
}
```

## Performance Monitoring

### Advanced Performance Tracking

Future versions will implement detailed performance monitoring:

```rust
// Web performance monitoring
#[cfg(target_arch = "wasm32")]
fn track_performance(name: &str) -> PerformanceMarker {
    let performance = web_sys::window()
        .unwrap()
        .performance()
        .unwrap();
    
    let start_time = performance.now();
    let mark_name = format!("start-{}", name);
    
    // Mark the beginning of the operation
    performance.mark(&mark_name);
    
    PerformanceMarker {
        name: name.to_string(),
        start_time,
    }
}

struct PerformanceMarker {
    name: String,
    start_time: f64,
}

impl Drop for PerformanceMarker {
    fn drop(&mut self) {
        let performance = web_sys::window()
            .unwrap()
            .performance()
            .unwrap();
        
        let end_time = performance.now();
        let duration = end_time - self.start_time;
        
        log::info!("{} took {} ms", self.name, duration);
        
        // Record the measure
        let mark_name = format!("end-{}", self.name);
        performance.mark(&mark_name);
        performance.measure(&self.name, &format!("start-{}", self.name), &mark_name);
    }
}
```

## Feature Detection

### Enhanced Browser Feature Detection

Future versions will have improved browser feature detection:

```rust
// Detect browser features
#[cfg(target_arch = "wasm32")]
fn detect_browser_features() -> BrowserFeatures {
    let window = web_sys::window().unwrap();
    
    // Check for WebGPU support
    let webgpu_supported = js_sys::Reflect::has(
        &window,
        &JsValue::from_str("navigator.gpu"),
    ).unwrap_or(false);
    
    // Check for File System Access API
    let fs_access_api = js_sys::Reflect::has(
        &window,
        &JsValue::from_str("showOpenFilePicker"),
    ).unwrap_or(false);
    
    // Check for SharedArrayBuffer support
    let shared_array_buffer = js_sys::Reflect::has(
        &window,
        &JsValue::from_str("SharedArrayBuffer"),
    ).unwrap_or(false);
    
    BrowserFeatures {
        webgpu_supported,
        fs_access_api,
        shared_array_buffer,
    }
}
```

## Timeline

These features are prioritized as follows:

1. **Short-term** (1-3 months):
   - IndexedDB integration for persistent storage
   - Basic performance monitoring

2. **Medium-term** (3-6 months):
   - Web Worker integration for background processing
   - Enhanced browser feature detection

3. **Long-term** (6+ months):
   - Advanced performance tracking
   - Offline support with Service Workers
   - Web Assembly SIMD optimizations

## Related Documentation

- [Web Platform Guide](/docs/platform_web.md) - Current web implementation
- [Cross-Platform Framework](/docs/cross_platform_framework.md) - Current cross-platform approach 