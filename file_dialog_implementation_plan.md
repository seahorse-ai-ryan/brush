# File Dialog Implementation Plan

## Current Status (Checkpoint)

We've made significant progress implementing file dialog functionality in the Brush application:

1. **Desktop App Fixes**:
   - Added `#[derive(Clone)]` to the `SelectedView` struct
   - Added `url_input` field to the `DatasetDetailOverlay` struct
   - Implemented a manual `Clone` trait for `DatasetDetailOverlay` to handle non-cloneable fields
   - Fixed borrow checker issues in `set_selected_files` and `set_selected_folders` methods
   - Fixed the `FileHandle::read()` method issue by getting the file name before calling `read()`

2. **Web App Fixes**:
   - Fixed pattern matching in the `app.rs` file for JavaScript event handling
   - Updated JavaScript code to use the correct event flags
   - Properly formatted JSON strings in JavaScript code

## Known Issues

1. **Web UI Issues**:
   - Uploading an existing PLY file seems to crash the app
   - This may be related to the known bug with the Burn server
   - Further investigation needed to determine the exact cause

## Next Steps

1. **Web UI Debugging**:
   - Investigate why uploading PLY files crashes the web app
   - Add error handling for file uploads in the web interface
   - Test with different file types to isolate the issue

2. **URL Download Functionality**:
   - Complete implementation of URL download feature
   - Add proper error handling for URL downloads
   - Test with various file types and URLs

3. **UI Improvements**:
   - Add progress indicators for file uploads and downloads
   - Improve error messaging for failed operations
   - Enhance the user experience with better feedback

4. **Testing**:
   - Comprehensive testing across different platforms
   - Test with various file types and sizes
   - Verify that both desktop and web versions handle files consistently

## Implementation Details

### File Handling Flow

1. User selects file(s) through the UI
2. Files are processed based on type:
   - ZIP files are extracted to a folder
   - Other files are copied as-is
3. Files are added to the dataset list
4. User can select a dataset to process

### URL Download Flow

1. User enters a URL in the input field
2. Application downloads the file
3. File is processed based on type
4. File is added to the dataset list

## Lessons Learned

1. The `FileHandle::read()` method consumes the handle, so we need to get any needed information before calling it
2. Proper handling of borrowing is crucial when dealing with file operations
3. Web and desktop implementations require different approaches for file handling
4. JavaScript integration requires careful handling of event flags and JSON formatting 