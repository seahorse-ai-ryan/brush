#[cfg(target_os = "android")]
pub mod android;

#[allow(unused)]
use anyhow::Context;
use anyhow::Result;
use std::path::PathBuf;
use std::io::Error;
use std::io::ErrorKind;

pub enum FileHandle {
    #[cfg(not(target_os = "android"))]
    Rfd(rfd::FileHandle),
    #[cfg(target_os = "android")]
    Android(tokio::fs::File),
}

impl FileHandle {
    pub async fn write(&self, data: &[u8]) -> std::io::Result<()> {
        match self {
            #[cfg(not(target_os = "android"))]
            Self::Rfd(file_handle) => file_handle.write(data).await,
            #[cfg(target_os = "android")]
            Self::Android(_) => {
                let _ = data;
                unimplemented!("No saving on Android yet.")
            }
        }
    }

    pub async fn read(mut self) -> Vec<u8> {
        match &mut self {
            #[cfg(not(target_os = "android"))]
            Self::Rfd(file_handle) => file_handle.read().await,
            #[cfg(target_os = "android")]
            Self::Android(file) => {
                use tokio::io::AsyncReadExt;

                let mut buf = vec![];
                file.read_to_end(&mut buf).await.unwrap();
                buf
            }
        }
    }
    
    /// Get the file name if available
    pub fn name(&self) -> Option<String> {
        match self {
            #[cfg(not(target_os = "android"))]
            Self::Rfd(file_handle) => {
                Some(file_handle.file_name())
            },
            #[cfg(target_os = "android")]
            Self::Android(_) => None,
        }
    }
}

/// Pick a file and return the name & bytes of the file.
pub async fn pick_file() -> Result<FileHandle> {
    #[cfg(not(target_os = "android"))]
    {
        let file = rfd::AsyncFileDialog::new()
            .pick_file()
            .await
            .context("No file selected")?;
        Ok(FileHandle::Rfd(file))
    }

    #[cfg(target_os = "android")]
    {
        android::pick_file().await.map(FileHandle::Android)
    }
}

pub async fn pick_directory() -> Result<PathBuf> {
    #[cfg(all(not(target_os = "android"), not(target_family = "wasm")))]
    {
        let dir = rfd::AsyncFileDialog::new()
            .pick_folder()
            .await
            .context("No folder selected")?;

        Ok(dir.path().to_path_buf())
    }

    #[cfg(any(target_os = "android", target_family = "wasm"))]
    {
        panic!("No folder picking on Android or wasm yet.")
    }
}

/// Saves data to a file and returns the filename the data was saved too.
///
/// Nb: Does not work on Android currently.
pub async fn save_file(default_name: &str) -> Result<FileHandle> {
    #[cfg(not(target_os = "android"))]
    {
        let file = rfd::AsyncFileDialog::new()
            .set_file_name(default_name)
            .save_file()
            .await
            .context("No file selected")?;
        Ok(FileHandle::Rfd(file))
    }

    #[cfg(target_os = "android")]
    {
        let _ = default_name;
        panic!("No saving on Android yet.")
    }
}

/// Pick multiple files and return the handles
pub async fn pick_files() -> Result<Vec<FileHandle>> {
    #[cfg(target_os = "android")]
    {
        anyhow::bail!("Multiple file picking is not supported on Android")
    }

    #[cfg(not(target_os = "android"))]
    {
        let file_handles = rfd::AsyncFileDialog::new()
            .add_filter("Dataset Files", &["zip", "ply"])
            .pick_files()
            .await;

        match file_handles {
            Some(handles) => {
                let handles = handles
                    .into_iter()
                    .map(|handle| FileHandle::Rfd(handle))
                    .collect();
                Ok(handles)
            }
            None => anyhow::bail!("User canceled file picking"),
        }
    }
}

/// Pick multiple directories and return their paths
pub async fn pick_directories() -> Result<Vec<PathBuf>> {
    #[cfg(any(target_os = "android", target_family = "wasm"))]
    {
        anyhow::bail!("Multiple folder picking is not supported on this platform")
    }

    #[cfg(not(any(target_os = "android", target_family = "wasm")))]
    {
        let folders = rfd::AsyncFileDialog::new()
            .pick_folders()
            .await;

        match folders {
            Some(folders) => {
                // Convert FileHandle to PathBuf
                let paths = folders.into_iter()
                    .map(|folder| folder.path().to_path_buf())
                    .collect();
                Ok(paths)
            },
            None => anyhow::bail!("User canceled folder picking"),
        }
    }
}
