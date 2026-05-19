//! Installer subsystem: pluggable resolvers for dependencies.

use anyhow::Result;
use std::path::PathBuf;

use crate::manifest::{Dependency, NamedPath, NamedString};

pub mod git;
pub mod choco;
pub mod vcpkg;

pub use git::GitInstaller;
pub use choco::ChocoInstaller;
pub use vcpkg::VcpkgInstaller;

#[derive(Debug, Clone)]
pub enum InstallOutcome {
    AlreadyPresent { include_paths: Vec<PathBuf>, lib_paths: Vec<PathBuf> },
    Installed { include_paths: Vec<PathBuf>, lib_paths: Vec<PathBuf> },
}

pub trait Installer {
    fn install(&self, dep: &crate::manifest::Dependency) -> Result<InstallOutcome>;
}

pub fn to_named_paths(name: &str, paths: &[PathBuf]) -> Vec<NamedPath> {
    return paths.iter().map(|p| NamedPath { name: name.to_string(), path: p.display().to_string() }).collect();
}

pub fn to_named_strings(name: &str, libs: &[String]) -> Vec<NamedString> {
    return libs.iter().map(|v| NamedString { name: name.to_string(), value: v.clone() }).collect();
}
