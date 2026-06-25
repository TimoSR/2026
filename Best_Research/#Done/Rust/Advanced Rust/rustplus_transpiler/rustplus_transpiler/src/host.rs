use anyhow::{Context, Result};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub trait RustPlusHost {
    fn read_to_string(&self, path: &Path) -> Result<String>;
    fn write_string(&self, path: &Path, content: &str) -> Result<()>;
    fn exists(&self, path: &Path) -> bool;
    fn create_dir_all(&self, path: &Path) -> Result<()>;
    fn canonicalize(&self, path: &Path) -> Result<PathBuf>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct RealFileSystemHost;

impl RustPlusHost for RealFileSystemHost {
    fn read_to_string(&self, path: &Path) -> Result<String> {
        return fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()));
    }

    fn write_string(&self, path: &Path, content: &str) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| format!("failed to create {}", parent.display()))?;
        }

        return fs::write(path, content).with_context(|| format!("failed to write {}", path.display()));
    }

    fn exists(&self, path: &Path) -> bool {
        return path.exists();
    }

    fn create_dir_all(&self, path: &Path) -> Result<()> {
        return fs::create_dir_all(path).with_context(|| format!("failed to create {}", path.display()));
    }

    fn canonicalize(&self, path: &Path) -> Result<PathBuf> {
        return fs::canonicalize(path).with_context(|| format!("failed to canonicalize {}", path.display()));
    }
}

#[derive(Debug, Default)]
pub struct MemoryHost {
    files: RefCell<HashMap<PathBuf, String>>,
}

impl MemoryHost {
    pub fn new() -> Self {
        return Self::default();
    }

    pub fn insert(&self, path: impl Into<PathBuf>, content: impl Into<String>) {
        self.files.borrow_mut().insert(path.into(), content.into());
    }

    pub fn snapshot(&self) -> HashMap<PathBuf, String> {
        return self.files.borrow().clone();
    }
}

impl RustPlusHost for MemoryHost {
    fn read_to_string(&self, path: &Path) -> Result<String> {
        return self
            .files
            .borrow()
            .get(path)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("memory host file does not exist: {}", path.display()));
    }

    fn write_string(&self, path: &Path, content: &str) -> Result<()> {
        self.files.borrow_mut().insert(path.to_path_buf(), content.to_string());
        return Ok(());
    }

    fn exists(&self, path: &Path) -> bool {
        return self.files.borrow().contains_key(path);
    }

    fn create_dir_all(&self, _path: &Path) -> Result<()> {
        return Ok(());
    }

    fn canonicalize(&self, path: &Path) -> Result<PathBuf> {
        return Ok(path.to_path_buf());
    }
}
