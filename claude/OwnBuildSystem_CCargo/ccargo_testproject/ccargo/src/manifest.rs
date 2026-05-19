//! Manifest handling: ccargo.toml schema and helpers.
//!
//! Supports dep declaration in FLAT form (kind at top-level), e.g.:
//! [[dependencies]]
//! name = "sdl2"
//! kind = "Vcpkg"
//! package = "sdl2"
//! triplet = "x64-windows"

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::fs;
use std::path::{Path, PathBuf};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub project: Project,
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
    /// Derived install metadata written by `resolve` (include/lib paths)
    #[serde(default)]
    pub install: InstallIndex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    #[serde(default = "default_out_dir")]
    pub out_dir: String,
    #[serde(default = "default_target_name")]
    pub target: String,
}

fn default_out_dir() -> String { return "build".to_string(); }
fn default_target_name() -> String { return "app".to_string(); }

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct Dependency {
    pub name: String,
    /// Flattened tagged enum => `kind="Git"|"Choco"|"Vcpkg"` + fields at same level.
    #[serde(flatten)]
    pub source: DepSource,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum DependencyWire {
    Flat { name: String, #[serde(flatten)] source: DepSource },
    Nested { name: String, source: DepSource },
}

impl<'de> Deserialize<'de> for Dependency {
    fn deserialize<D>(de: D) -> std::result::Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let w = DependencyWire::deserialize(de)?;
        let d = match w {
            DependencyWire::Flat { name, source } => Dependency { name, source },
            DependencyWire::Nested { name, source } => Dependency { name, source },
        };
        return Ok(d);
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum DepSource {
    Git {
        url: String,
        tag: Option<String>,
        branch: Option<String>,
        rev: Option<String>,
        /// If present, run a simple CMake configure+build
        cmake: Option<CMakeConfig>,
    },
    /// Windows-only installer (kept for other libs; SDL2 is usually not here)
    Choco {
        package: String,
        version: Option<String>,
    },
    /// Cross-platform via Microsoft vcpkg (recommended for SDL2 on Windows)
    Vcpkg {
        /// e.g. "sdl2", "sdl2-image"
        package: String,
        /// e.g. "x64-windows", "x64-windows-static", "x86-windows", "arm64-windows"
        triplet: String,
        /// Optional vcpkg features, e.g. ["vulkan","opengl"]
        #[serde(default)]
        features: Vec<String>,
        /// Optional version constraint (vcpkg versioning) e.g. "=2.32.8"
        version: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CMakeConfig {
    pub generator: Option<String>,
    #[serde(default)]
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstallIndex {
    #[serde(default)]
    pub include_paths: Vec<NamedPath>,
    #[serde(default)]
    pub lib_paths: Vec<NamedPath>,
    #[serde(default)]
    pub link_libs: Vec<NamedString>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedPath {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedString {
    pub name: String,
    pub value: String,
}

impl Manifest {
    pub fn new(name: String) -> Self {
        return Self {
            project: Project { name, out_dir: default_out_dir(), target: default_target_name() },
            dependencies: vec![],
            install: InstallIndex::default(),
        };
    }

    pub fn read(path: &Path) -> Result<Self> {
        let text = fs::read_to_string(path)?;
        let parsed: Manifest = toml::from_str(&text)?;
        return Ok(parsed);
    }

    pub fn write(&self, path: &Path) -> Result<()> {
        let text = toml::to_string_pretty(self)?;
        fs::write(path, text)?;
        return Ok(());
    }

    pub fn add_dependency(&mut self, dep: Dependency) -> Result<()> {
        if self.dependencies.iter().any(|d| d.name == dep.name) {
            return Err(anyhow!("dependency '{}' already exists", dep.name));
        }
        self.dependencies.push(dep);
        return Ok(());
    }

    pub fn update_install_index(&mut self, include: Vec<NamedPath>, libs: Vec<NamedPath>, link_libs: Vec<NamedString>) {
        self.install.include_paths = include;
        self.install.lib_paths = libs;
        self.install.link_libs = link_libs;
        return;
    }

    pub fn save_index_only(&self, path: &Path, include: Vec<NamedPath>, libs: Vec<NamedPath>, link_libs: Vec<NamedString>) -> Result<()> {
        let mut clone = self.clone();
        clone.install.include_paths = include;
        clone.install.lib_paths = libs;
        clone.install.link_libs = link_libs;
        let text = toml::to_string_pretty(&clone)?;
        fs::write(path, text)?;
        return Ok(());
    }
}
