//! Filesystem-based storage implementation for datasets.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::{Result, Context};
use super::{DatasetStorage, DatasetInfo, StorageType};

/// Filesystem-based storage for datasets
pub struct FilesystemStorage {
    /// Base directory for storing datasets
    dataset_dir: PathBuf,
}

impl FilesystemStorage {
    /// Create a new filesystem storage with the given dataset directory
    pub fn new(dataset_dir: PathBuf) -> Self {
        Self { dataset_dir }
    }
    
    /// Get the path to a dataset with the given name
    fn get_dataset_path(&self, name: &str) -> PathBuf {
        self.dataset_dir.join(name)
    }
    
    /// Get information about a dataset
    pub fn get_dataset_info(&self, name: &str) -> Result<DatasetInfo> {
        let path = self.get_dataset_path(name);
        let metadata = fs::metadata(&path).context("Failed to get dataset metadata")?;
        
        let created_at = metadata
            .created()
            .unwrap_or(SystemTime::now())
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
            
        let modified_at = metadata
            .modified()
            .unwrap_or(SystemTime::now())
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
            
        Ok(DatasetInfo {
            name: name.to_string(),
            size_bytes: metadata.len(),
            created_at,
            modified_at,
            storage_type: StorageType::Local,
        })
    }

    /// Initialize the storage system
    pub fn initialize(&mut self) -> Result<()> {
        #[cfg(not(target_family = "wasm"))]
        {
            if !self.dataset_dir.exists() {
                fs::create_dir_all(&self.dataset_dir)?;
            }
        }
        Ok(())
    }

    /// Save a dataset to the filesystem
    pub fn save_dataset(&self, _name: &str, _data: &[u8]) -> Result<()> {
        #[cfg(not(target_family = "wasm"))]
        {
            let dataset_path = self.dataset_dir.join(_name);
            fs::write(dataset_path, _data)?;
        }
        Ok(())
    }

    /// Load a dataset from the filesystem
    pub fn load_dataset(&self, _name: &str) -> Result<Vec<u8>> {
        #[cfg(not(target_family = "wasm"))]
        {
            let dataset_path = self.dataset_dir.join(_name);
            let data = fs::read(dataset_path)?;
            return Ok(data);
        }
        
        #[cfg(target_family = "wasm")]
        {
            // Return empty data for WASM as filesystem operations are not supported
            return Ok(Vec::new());
        }
    }

    /// List all datasets in the storage
    pub fn list_datasets(&self) -> Result<Vec<String>> {
        #[cfg(not(target_family = "wasm"))]
        {
            let mut datasets = Vec::new();
            if self.dataset_dir.exists() {
                for entry in fs::read_dir(&self.dataset_dir)? {
                    let entry = entry?;
                    if let Some(name) = entry.file_name().to_str() {
                        datasets.push(name.to_string());
                    }
                }
            }
            return Ok(datasets);
        }
        
        #[cfg(target_family = "wasm")]
        {
            // Return empty list for WASM as filesystem operations are not supported
            return Ok(Vec::new());
        }
    }

    /// Delete a dataset from the filesystem
    pub fn delete_dataset(&self, _name: &str) -> Result<()> {
        #[cfg(not(target_family = "wasm"))]
        {
            let dataset_path = self.dataset_dir.join(_name);
            if dataset_path.exists() {
                fs::remove_file(dataset_path)?;
            }
        }
        Ok(())
    }
    
    fn get_total_storage_used(&self) -> Result<u64> {
        let mut total_size = 0;
        
        for entry in fs::read_dir(&self.dataset_dir)
            .context("Failed to read dataset directory")?
        {
            let entry = entry.context("Failed to read directory entry")?;
            let metadata = entry.metadata().context("Failed to get file metadata")?;
            
            if metadata.is_file() {
                total_size += metadata.len();
            }
        }
        
        Ok(total_size)
    }
} 