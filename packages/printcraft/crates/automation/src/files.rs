//! Per-session byte files for hosts without a filesystem. Native calls keep their existing
//! root confinement and atomic-write implementation. Memory sessions never fall back to disk.
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io;
use std::path::{Component, Path, PathBuf};

use crate::{Automation, Result, failed};

#[derive(Default)]
pub(super) enum Files {
    #[default]
    Native,
    Memory(RefCell<BTreeMap<PathBuf, Vec<u8>>>),
}

impl Files {
    pub(super) fn is_memory(&self) -> bool {
        matches!(self, Self::Memory(_))
    }
}

pub(super) fn memory_path(path: &Path) -> io::Result<PathBuf> {
    let mut out = PathBuf::from("/");
    for part in path.components() {
        match part {
            Component::RootDir | Component::CurDir => {}
            Component::Normal(s) => {
                if s.to_string_lossy().contains('\0') {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "file paths cannot contain NUL"));
                }
                out.push(s);
            }
            Component::ParentDir if out.pop() => {}
            _ => return Err(io::Error::new(io::ErrorKind::PermissionDenied, "file path escapes the virtual root")),
        }
    }
    if out == Path::new("/") {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "a file name is required"));
    }
    Ok(out)
}

impl Automation {
    /// A request-local filesystem. Paths address byte files, never the host's disk.
    pub fn in_memory() -> Self {
        Self { files: Files::Memory(RefCell::new(BTreeMap::new())), ..Self::new() }
    }

    /// Upload bytes to a memory session. Does not silently replace an existing upload/output.
    pub fn put_file(&self, path: &str, bytes: Vec<u8>) -> Result<()> {
        let Files::Memory(files) = &self.files else { return Err(failed("put_file requires a memory session")) };
        let path = memory_path(Path::new(path)).map_err(failed)?;
        let mut files = files.try_borrow_mut().map_err(failed)?;
        if files.contains_key(&path) {
            return Err(failed(format!("{} already exists", path.display())));
        }
        files.insert(path, bytes);
        Ok(())
    }

    /// Download a memory file (saved PDFs, exports, data files, etc.).
    pub fn file_bytes(&self, path: &str) -> Result<Vec<u8>> {
        if !self.files.is_memory() {
            return Err(failed("file_bytes requires a memory session"));
        }
        self.read_file(Path::new(path)).map_err(failed)
    }

    pub fn file_names(&self) -> Result<Vec<String>> {
        let Files::Memory(files) = &self.files else { return Err(failed("file_names requires a memory session")) };
        Ok(files.try_borrow().map_err(failed)?.keys().map(|p| p.to_string_lossy().into_owned()).collect())
    }

    pub(super) fn read_file(&self, path: &Path) -> io::Result<Vec<u8>> {
        match &self.files {
            Files::Native => std::fs::read(path),
            Files::Memory(files) => files
                .try_borrow()
                .map_err(io::Error::other)?
                .get(&memory_path(path)?)
                .cloned()
                .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, format!("{}: upload this file first", path.display()))),
        }
    }

    pub(super) fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        match &self.files {
            Files::Native => std::fs::create_dir_all(path),
            Files::Memory(_) => Ok(()), // Directories are implicit in the flat byte store.
        }
    }

    pub(super) fn write_file(&self, path: &Path, bytes: &[u8]) -> Result<()> {
        match &self.files {
            Files::Native => crate::write_atomic(path, bytes),
            Files::Memory(files) => {
                let path = memory_path(path).map_err(failed)?;
                files.try_borrow_mut().map_err(failed)?.insert(path, bytes.to_vec());
                Ok(())
            }
        }
    }
}
