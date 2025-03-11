use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use brush_render::gaussian_splats::Splats;
use burn::tensor::backend::AutodiffBackend;
use brush_train::train::TrainBack;
use log::{info, error};
use thiserror::Error;

/// Errors that can occur during export operations
#[derive(Error, Debug)]
pub enum ExportError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("Export directory not set")]
    NoExportDir,
    
    #[error("No splats available for export")]
    NoSplats,
    
    #[error("Invalid filename: {0}")]
    InvalidFilename(String),
    
    #[error("Export failed: {0}")]
    Other(String),
}

/// Supported export formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    PLY,
    // Add more formats here as needed
}

/// Configuration for auto-saving during training
#[derive(Debug, Clone)]
pub struct AutoSaveConfig {
    /// Whether auto-save is enabled
    pub enabled: bool,
    
    /// Interval between saves (in training steps)
    pub interval_steps: u32,
    
    /// Maximum number of auto-saves to keep (None for unlimited)
    pub max_saves: Option<u32>,
    
    /// Format to use for auto-saves
    pub format: ExportFormat,
    
    /// Prefix for auto-save filenames
    pub prefix: String,
}

impl Default for AutoSaveConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            interval_steps: 1000,
            max_saves: Some(5),
            format: ExportFormat::PLY,
            prefix: "autosave_".to_string(),
        }
    }
}

/// Central service for handling export operations
#[derive(Clone)]
pub struct ExportService {
    /// Directory where exports will be saved
    export_dir: Option<PathBuf>,
    
    /// Default export format
    default_format: ExportFormat,
    
    /// Auto-save configuration
    auto_save_config: Option<AutoSaveConfig>,
    
    /// Last training step when auto-save was performed
    last_auto_save_step: u32,
}

impl ExportService {
    /// Create a new ExportService with the specified export directory
    pub fn new(export_dir: Option<PathBuf>) -> Self {
        Self {
            export_dir,
            default_format: ExportFormat::PLY,
            auto_save_config: None,
            last_auto_save_step: 0,
        }
    }
    
    /// Set the export directory
    pub fn set_export_dir(&mut self, dir: PathBuf) {
        self.export_dir = Some(dir);
    }
    
    /// Get the current export directory
    pub fn export_dir(&self) -> Option<&Path> {
        self.export_dir.as_deref()
    }
    
    /// Configure auto-save functionality
    pub fn configure_auto_save(&mut self, config: AutoSaveConfig) {
        self.auto_save_config = Some(config);
    }
    
    /// Disable auto-save functionality
    pub fn disable_auto_save(&mut self) {
        if let Some(config) = &mut self.auto_save_config {
            config.enabled = false;
        }
    }
    
    /// Export splats in the specified format
    pub fn export_splats(
        &self,
        splats: &Splats<<TrainBack as AutodiffBackend>::InnerBackend>,
        format: ExportFormat,
        filename: &str,
    ) -> Result<PathBuf, ExportError> {
        match format {
            ExportFormat::PLY => self.export_ply(splats, filename),
        }
    }
    
    /// Export splats in PLY format
    pub fn export_ply(
        &self,
        splats: &Splats<<TrainBack as AutodiffBackend>::InnerBackend>,
        filename: &str,
    ) -> Result<PathBuf, ExportError> {
        // Ensure we have an export directory
        let export_dir = self.export_dir.as_ref()
            .ok_or(ExportError::NoExportDir)?;
        
        // Ensure the export directory exists
        if !export_dir.exists() {
            fs::create_dir_all(export_dir).map_err(ExportError::Io)?;
        }
        
        // Validate filename
        if filename.is_empty() {
            return Err(ExportError::InvalidFilename("Empty filename".to_string()));
        }
        
        // Ensure filename has .ply extension
        let filename = if !filename.ends_with(".ply") {
            format!("{}.ply", filename)
        } else {
            filename.to_string()
        };
        
        // Create the full output path
        let output_path = export_dir.join(filename);
        
        // Export the splats to PLY format
        self.export_splats_to_ply(splats, &output_path)?;
        
        info!("Exported splats to {}", output_path.display());
        Ok(output_path)
    }
    
    /// Check if auto-save should be performed and do it if needed
    pub fn check_auto_save(
        &mut self,
        splats: &Splats<<TrainBack as AutodiffBackend>::InnerBackend>,
        current_step: u32,
    ) -> Option<Result<PathBuf, ExportError>> {
        // Check if auto-save is enabled and configured
        let config = match &self.auto_save_config {
            Some(config) if config.enabled => config,
            _ => return None,
        };
        
        // Check if it's time to auto-save
        if current_step == 0 || current_step - self.last_auto_save_step < config.interval_steps {
            return None;
        }
        
        // It's time to auto-save
        self.last_auto_save_step = current_step;
        
        // Generate filename with step number
        let filename = format!("{}{}", config.prefix, current_step);
        
        // Perform the export
        let result = self.export_splats(splats, config.format, &filename);
        
        // Clean up old auto-saves if needed
        if let Some(max_saves) = config.max_saves {
            if let Err(err) = self.cleanup_old_autosaves(config.prefix.as_str(), max_saves) {
                error!("Failed to clean up old auto-saves: {:?}", err);
            }
        }
        
        Some(result)
    }
    
    /// Export splats to PLY format (implementation)
    fn export_splats_to_ply(
        &self,
        splats: &Splats<<TrainBack as AutodiffBackend>::InnerBackend>,
        output_path: &Path,
    ) -> Result<(), ExportError> {
        // Convert splats to PLY format directly without using block_on
        let data = match brush_dataset::splat_export::splat_to_ply_sync(splats.clone()) {
            Ok(data) => data,
            Err(e) => {
                error!("Failed to serialize splats to PLY: {}", e);
                return Err(ExportError::Other(format!("Failed to serialize splats: {}", e)));
            }
        };
        
        // Write the PLY data to the output file
        match fs::write(output_path, data) {
            Ok(_) => {
                info!("Successfully exported splats to {}", output_path.display());
                Ok(())
            },
            Err(e) => {
                error!("Failed to write PLY file: {}", e);
                Err(ExportError::Io(e))
            }
        }
    }
    
    /// Clean up old auto-saves, keeping only the most recent ones
    fn cleanup_old_autosaves(&self, prefix: &str, max_saves: u32) -> Result<(), ExportError> {
        // Ensure we have an export directory
        let export_dir = self.export_dir.as_ref()
            .ok_or(ExportError::NoExportDir)?;
        
        // List all files in the export directory
        let entries = fs::read_dir(export_dir).map_err(ExportError::Io)?;
        
        // Collect auto-save files
        let mut autosave_files: Vec<(PathBuf, u32)> = Vec::new();
        
        for entry in entries {
            let entry = entry.map_err(ExportError::Io)?;
            let path = entry.path();
            
            // Skip directories
            if path.is_dir() {
                continue;
            }
            
            // Get filename as string
            let filename = match path.file_name().and_then(|f| f.to_str()) {
                Some(name) => name,
                None => continue,
            };
            
            // Check if this is an auto-save file
            if !filename.starts_with(prefix) {
                continue;
            }
            
            // Extract step number from filename
            let step_str = filename.trim_start_matches(prefix)
                .trim_end_matches(".ply");
            
            let step = match step_str.parse::<u32>() {
                Ok(step) => step,
                Err(_) => continue,
            };
            
            autosave_files.push((path, step));
        }
        
        // Sort by step number (descending)
        autosave_files.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Remove excess files
        if autosave_files.len() > max_saves as usize {
            for (path, _) in autosave_files.iter().skip(max_saves as usize) {
                if let Err(err) = fs::remove_file(path) {
                    error!("Failed to remove old auto-save file {}: {}", path.display(), err);
                }
            }
        }
        
        Ok(())
    }
    
    /// Export all datasets in the specified format
    pub fn export_all_datasets(
        &self,
        datasets: &[Splats<<TrainBack as AutodiffBackend>::InnerBackend>],
        format: ExportFormat,
    ) -> Vec<Result<PathBuf, ExportError>> {
        let mut results = Vec::new();
        
        for (i, dataset) in datasets.iter().enumerate() {
            let filename = format!("dataset_{}", i);
            results.push(self.export_splats(dataset, format, &filename));
        }
        
        results
    }
} 