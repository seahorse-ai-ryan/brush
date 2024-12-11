//
// This class helps working with an archive as a somewhat more regular filesystem.
//
// [1] really we want to just read directories.
// The reason is that picking directories isn't supported on
// rfd on wasm, nor is drag-and-dropping folders in egui.
use std::{
    collections::HashMap,
    io::{Cursor, Read},
    path::{Component, Path, PathBuf},
    sync::Arc,
};

use anyhow::Context;
use tokio::{
    io::{AsyncRead, AsyncReadExt},
    sync::Mutex,
};

use zip::{
    result::{ZipError, ZipResult},
    ZipArchive,
};

type DynRead = Box<dyn AsyncRead + Send + Unpin>;

#[derive(Clone)]
pub struct ZipData {
    data: Arc<Vec<u8>>,
}

impl AsRef<[u8]> for ZipData {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

pub(crate) fn normalized_path(path: &Path) -> PathBuf {
    path.components()
        .filter(|c| !matches!(c, Component::CurDir | Component::ParentDir))
        .collect()
}

#[derive(Clone, Default)]
pub struct PathReader {
    paths: HashMap<PathBuf, Arc<Mutex<Option<DynRead>>>>,
}

impl PathReader {
    fn paths(&self) -> impl Iterator<Item = &PathBuf> {
        self.paths.keys()
    }

    pub fn add(&mut self, path: &Path, reader: impl AsyncRead + Send + Unpin + 'static) {
        self.paths.insert(
            path.to_path_buf(),
            Arc::new(Mutex::new(Some(Box::new(reader)))),
        );
    }

    async fn open(&mut self, path: &Path) -> anyhow::Result<Box<dyn AsyncRead + Send + Unpin>> {
        let entry = self.paths.remove(path).context("File not found")?;
        let reader = entry.lock().await.take();
        reader.context("Missing reader")
    }
}

#[derive(Clone)]
pub enum BrushVfs {
    Zip(ZipArchive<Cursor<ZipData>>),
    Manual(PathReader),
    #[cfg(not(target_family = "wasm"))]
    Directory(PathBuf, Vec<PathBuf>),
}

// TODO: This is all awfully ad-hoc.
impl BrushVfs {
    pub async fn from_zip_reader(reader: impl AsyncRead + Unpin) -> ZipResult<Self> {
        let mut bytes = vec![];
        let mut reader = reader;
        reader.read_to_end(&mut bytes).await?;

        let zip_data = ZipData {
            data: Arc::new(bytes),
        };
        let archive = ZipArchive::new(Cursor::new(zip_data))?;
        Ok(BrushVfs::Zip(archive))
    }

    pub fn from_paths(paths: PathReader) -> Self {
        BrushVfs::Manual(paths)
    }

    pub async fn from_directory(dir: &Path) -> anyhow::Result<Self> {
        #[cfg(not(target_family = "wasm"))]
        {
            async fn walk_dir(dir: impl AsRef<Path>) -> std::io::Result<Vec<PathBuf>> {
                let dir = PathBuf::from(dir.as_ref());

                let mut paths = Vec::new();
                let mut stack = vec![dir.clone()];

                while let Some(path) = stack.pop() {
                    let mut read_dir = tokio::fs::read_dir(&path).await?;

                    while let Some(entry) = read_dir.next_entry().await? {
                        let path = entry.path();
                        if path.is_dir() {
                            stack.push(path.clone());
                        }
                        paths.push(path.strip_prefix(dir.clone()).unwrap().to_path_buf());
                    }
                }
                Ok(paths)
            }

            Ok(BrushVfs::Directory(dir.to_path_buf(), walk_dir(dir).await?))
        }

        #[cfg(target_family = "wasm")]
        {
            let _ = dir;
            unimplemented!("No reading on wasm");
        }
    }

    pub fn file_names(&self) -> impl Iterator<Item = &Path> + '_ {
        let iterator: Box<dyn Iterator<Item = &Path>> = match self {
            BrushVfs::Zip(archive) => Box::new(archive.file_names().map(Path::new)),
            BrushVfs::Manual(map) => Box::new(map.paths().map(|p| p.as_path())),
            #[cfg(not(target_family = "wasm"))]
            BrushVfs::Directory(_, paths) => Box::new(paths.iter().map(|p| p.as_path())),
        };
        // stupic macOS.
        iterator.filter(|p| !p.starts_with("__MACOSX"))
    }

    pub async fn open_path(&mut self, path: &Path) -> anyhow::Result<DynRead> {
        match self {
            BrushVfs::Zip(archive) => {
                let name = archive
                    .file_names()
                    .find(|name| path == Path::new(name))
                    .ok_or(ZipError::FileNotFound)?;
                let name = name.to_owned();
                let mut buffer = vec![];
                archive.by_name(&name)?.read_to_end(&mut buffer)?;
                Ok(Box::new(Cursor::new(buffer)))
            }
            BrushVfs::Manual(map) => map.open(path).await,
            #[cfg(not(target_family = "wasm"))]
            BrushVfs::Directory(dir, _) => {
                let total_path = dir.join(path);
                let file = tokio::fs::File::open(total_path).await?;
                let file = tokio::io::BufReader::new(file);
                Ok(Box::new(file))
            }
        }
    }
}
