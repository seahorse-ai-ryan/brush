# Brush Development: Lessons Learned

This document captures practical solutions, patterns, and knowledge gained during Brush development. It serves as a reference for both developers and AI assistants.

## Web Development

### WASM Compatibility

#### Problem: Filesystem operations in WASM
- **Issue**: Native filesystem operations failing in web environment
- **Solution**: Use platform-specific code paths
  ```rust
  #[cfg(target_arch = "wasm32")]
  {
      // Web-specific implementation
      web_sys::console::log_1(&"Running in web environment".into());
      // Use web APIs for handling data
  }

  #[cfg(not(target_arch = "wasm32"))]
  {
      // Native implementation
      std::fs::create_dir_all(path)?;
      // Use filesystem APIs directly
  }
  ```

#### Problem: Memory limitations in browser
- **Issue**: Out of memory errors when processing large datasets
- **Solution**: Implement chunked processing
  ```rust
  #[cfg(target_arch = "wasm32")]
  fn process_large_data(data: &[u8]) {
      const CHUNK_SIZE: usize = 1024 * 1024; // 1MB chunks
      
      for chunk in data.chunks(CHUNK_SIZE) {
          // Process chunk
          // Allow browser to reclaim memory between chunks
          web_sys::console::log_1(&format!("Processed chunk: {} bytes", chunk.len()).into());
      }
  }
  ```

#### Problem: WebGL context management
- **Issue**: Memory leaks from WebGL resources
- **Solution**: Proper cleanup in Drop implementation
  ```rust
  impl Drop for WebGLResource {
      fn drop(&mut self) {
          #[cfg(target_arch = "wasm32")]
          {
              // Clean up WebGL context resources
              if let Some(gl) = &self.gl_context {
                  // Delete buffers, textures, etc.
              }
          }
      }
  }
  ```

### Asset Management

#### Problem: Loading example PLY files
- **Issue**: PLY files not accessible in web environment
- **Solution**: Add data-trunk directives and verify file URL
  ```html
  <!-- In index.html -->
  <link data-trunk rel="copy-file" href="../../examples/lego.ply" />
  ```
  
  Then test direct access before integration:
  ```bash
  curl -I http://localhost:8080/lego.ply
  ```

#### Problem: SRI (Subresource Integrity) warnings
- **Issue**: Development builds generating SRI warnings
- **Solution**: Added disabling script in index.html
  ```javascript
  // This is a development-only script to disable SRI checks
  (function() {
      document.querySelectorAll('[integrity]').forEach(element => {
          element.removeAttribute('integrity');
      });
  })();
  ```

## UI Development

### Layout Issues

#### Problem: Window resizability issues
- **Issue**: Windows not resizing properly, content cut off
- **Solution**: Proper egui window configuration
  ```rust
  Window::new("Title")
      .resizable(true)
      .min_width(280.0)
      .show(ctx, |ui| {
          ui.set_min_size(egui::vec2(ui.available_width(), ui.available_height()));
          
          ScrollArea::vertical()
              .auto_shrink([false, false])
              .show(ui, |ui| {
                  // Content here
              });
      });
  ```

#### Problem: Scroll area behavior
- **Issue**: Content collapsing or not filling available space
- **Solution**: Configure auto-shrink and set proper size
  ```rust
  ScrollArea::vertical()
      .auto_shrink([false, false])
      .show(ui, |ui| {
          // Content
      });
  ```

### State Management

#### Problem: Borrow checker conflicts in UI
- **Issue**: Multiple mutable borrows in UI closures
- **Solution**: Use local variables and flags
  ```rust
  // Good: Local state pattern
  let mut local_state = self.state.clone();
  let mut should_process = false;
  
  if ui.button("Process").clicked() {
      should_process = true;
  }
  
  if should_process {
      self.process_data();
      self.state = local_state;
  }
  ```

## Debugging Workflows

### Problem: Console log visibility
- **Issue**: Difficulty seeing console logs during development
- **Solution**: MCP server setup for capturing browser logs
  ```bash
  # Start MCP server in a terminal
  npx @agentdeskai/browser-tools-server --port 3025
  
  # Verify it's working
  curl -s localhost:3025/console-logs
  ```

### Problem: Trunk server auto-reload issues
- **Issue**: Automatic reloading causing inconsistent state
- **Solution**: Use manual reload when needed
  ```bash
  # Start Trunk
  trunk serve
  
  # Manual reload when needed
  curl -X POST http://localhost:8080/_trunk/reload
  ```

### Problem: Chat stalling with verbose output
- **Issue**: "Skip and continue" appearing when Trunk generates too much output
- **Solution**: Run Trunk in the background with output redirection
- **Benefits**: 
  - Prevents chat from becoming unresponsive
  - Allows monitoring progress without interrupting conversation
  - Maintains logs for later inspection
- **Implementation**: See `.cursor/rules/brush_debug.mdc` for specific commands

### Problem: Port conflicts
- **Issue**: "Address already in use" errors
- **Solution**: Kill existing processes before starting servers
  ```bash
  # Check if ports are in use
  lsof -i :3025  # MCP server port
  lsof -i :8080  # Trunk server port
  
  # Kill specific processes
  pkill -f "trunk serve" || true
  pkill -f "browser-tools-server" || true
  ```

## Performance Optimization

### Problem: Slow PLY file processing
- **Issue**: Large PLY files causing UI freezes
- **Solution**: Async processing with progress updates
  ```rust
  pub async fn process_ply_file(path: &Path) -> Result<Dataset> {
      let total_size = path.metadata()?.len();
      let file = File::open(path)?;
      
      // Process in chunks with progress updates
      let mut processed = 0;
      for chunk in reader.chunks(CHUNK_SIZE) {
          // Process chunk
          processed += chunk.len();
          
          // Update progress every 5%
          if processed % (total_size / 20) < CHUNK_SIZE {
              update_progress(processed as f32 / total_size as f32);
          }
          
          // Allow UI to update
          tokio::task::yield_now().await;
      }
      
      Ok(dataset)
  }
  ```

### Problem: GPU resource management
- **Issue**: Inefficient GPU resource usage
- **Solution**: Sharing buffers and creating them on-demand
  ```rust
  // Use Arc for shared ownership
  struct GpuResources {
      device: Arc<wgpu::Device>,
      queue: Arc<wgpu::Queue>,
      pipeline: Arc<wgpu::RenderPipeline>,
  }
  
  // Create buffers only when needed
  fn get_or_create_buffer(&mut self, data: &[u8]) -> &wgpu::Buffer {
      if self.buffer.is_none() || self.buffer_size < data.len() {
          self.buffer = Some(create_buffer(&self.device, data));
          self.buffer_size = data.len();
      }
      self.buffer.as_ref().unwrap()
  }
  ```

## Testing Strategies

### Problem: Testing web-specific functionality
- **Issue**: Difficulty testing WASM-specific code paths
- **Solution**: URL parameters for automated testing
  ```javascript
  // In JavaScript
  if (new URLSearchParams(window.location.search).get('test') === 'ply-loading') {
      console.log('🧪 Running PLY loading test');
      fetch('lego.ply')
          .then(response => {
              console.log(`✅ PLY file ${response.ok ? 'accessible' : 'inaccessible'}`);
          });
  }
  ```

### Problem: Cross-platform test consistency
- **Issue**: Tests passing on one platform but failing on another
- **Solution**: Platform-specific test configurations
  ```rust
  #[cfg_attr(not(target_arch = "wasm32"), test)]
  #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
  fn test_dataset_loading() {
      #[cfg(not(target_arch = "wasm32"))]
      {
          // Native-specific test code
      }
      
      #[cfg(target_arch = "wasm32")]
      {
          // WASM-specific test code
      }
      
      // Common assertions
  }
  ```

## Best Practices Summary

1. **Platform-specific code**: Always use `#[cfg(target_arch = "wasm32")]` for web/native differences
2. **Memory management**: Clean up resources, especially in WASM environment
3. **UI responsiveness**: Avoid long-running operations on the main thread
4. **Error handling**: Provide clear, user-friendly error messages
5. **Testing**: Verify on all target platforms
6. **Documentation**: Update this file with new lessons learned 