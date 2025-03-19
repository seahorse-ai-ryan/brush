//! Storage module for dataset persistence.
//! 
//! This module provides platform-specific storage implementations:
//! - `filesystem`: For native platforms using the filesystem
//! - `indexed_db`: For web platforms using IndexedDB

use std::path::Path;
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[cfg(feature = "web-storage")]
pub mod indexed_db;

pub mod filesystem;

#[cfg(all(target_arch = "wasm32", feature = "web-storage"))]
pub use indexed_db as storage_impl;

#[cfg(not(target_arch = "wasm32"))]
pub use filesystem as storage_impl;

/// Common trait for dataset storage implementations
pub trait DatasetStorage {
    /// Initialize the storage system
    fn initialize(&mut self) -> Result<()>;
    
    /// Save a dataset with the given name and data
    fn save_dataset(&mut self, name: &str, data: &[u8]) -> Result<()>;
    
    /// Load a dataset with the given name
    fn load_dataset(&self, name: &str) -> Result<Vec<u8>>;
    
    /// Delete a dataset with the given name
    fn delete_dataset(&mut self, name: &str) -> Result<()>;
    
    /// Get the total storage used in bytes
    fn get_total_storage_used(&self) -> Result<u64>;
    
    /// List all available datasets
    fn list_datasets(&self) -> Result<Vec<String>>;
}

/// Storage type for datasets
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageType {
    /// Local storage (filesystem or IndexedDB)
    Local,
    /// Cloud storage (future implementation)
    Cloud {
        /// URL to the cloud storage
        url: String,
        /// Provider type
        provider: CloudProvider,
    },
}

impl Default for StorageType {
    fn default() -> Self {
        StorageType::Local
    }
}

/// Cloud storage provider types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CloudProvider {
    /// No cloud provider
    None,
    /// Amazon S3
    S3,
    /// Google Cloud Storage
    GCS,
    /// Custom provider
    Custom,
}

impl Default for CloudProvider {
    fn default() -> Self {
        CloudProvider::None
    }
}

/// Information about a stored dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetInfo {
    /// Name of the dataset
    pub name: String,
    /// Size of the dataset in bytes
    pub size_bytes: u64,
    /// Timestamp when the dataset was created
    pub created_at: u64,
    /// Timestamp when the dataset was last modified
    pub modified_at: u64,
    /// Storage type for the dataset
    pub storage_type: StorageType,
}

/// Helper function to format bytes in a human-readable format
pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    
    if bytes < KB {
        format!("{} B", bytes)
    } else if bytes < MB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else if bytes < GB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    }
} 