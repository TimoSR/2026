//! vcpkg-based installer (works with newer vcpkg that removed `root`).
//! Derives the vcpkg root from the executable path: <...>/vcpkg.exe -> <...>/
//! Then installs <package>:<triplet> and returns include/lib directories.

use anyhow::{anyhow, Context, Result};
use std::path::{Path, PathBuf};

use crate::installers::InstallOutcome;
use crate::manifest::{Dependency, DepSource};
use crate::util::{run_cmd, which_tool};

#[derive(Clone, Default)]
pub struct VcpkgInstaller;

impl super::Installer for VcpkgInstaller {
    fn install(&self, dep: &Dependency) -> Result<InstallOutcome> {
        let DepSource::Vcpkg { package, triplet, features, version } = &dep.source else {
            return Err(anyhow!("VcpkgInstaller received non-vcpkg dependency"));
        };

        // 1) Find vcpkg.exe
        let vcpkg = find_vcpkg()
            .ok_or_else(|| anyhow!("vcpkg not found on PATH or at %USERPROFILE%\\vcpkg\\vcpkg.exe"))?;

        // 2) Derive vcpkg root from the executable path
        let vcpkg_exe = PathBuf::from(&vcpkg);
        let vcpkg_root = vcpkg_exe.parent()
            .ok_or_else(|| anyhow!("failed to determine vcpkg root from {}", vcpkg_exe.display()))?
            .to_path_buf();

        // 3) Build spec: package[features]@version:triplet
        let mut spec = package.clone();
        if !features.is_empty() {
            spec.push('[');
            spec.push_str(&features.join(","));
            spec.push(']');
        }
        if let Some(ver) = version {
            spec.push('@');
            spec.push_str(ver);
        }
        spec.push(':');
        spec.push_str(triplet);

        // 4) Install (idempotent)
        run_cmd(&vcpkg, &["install", &spec]).with_context(|| format!("vcpkg install {}", spec))?;

        // 5) Discover include/lib
        let installed = vcpkg_root.join("installed").join(triplet);
        let include = installed.join("include");
        let lib_dir = installed.join("lib"); // vcpkg uses <triplet>/lib (and sometimes bin for DLLs)

        if !include.exists() {
            return Err(anyhow!("vcpkg include path not found: {}", include.display()));
        }
        if !lib_dir.exists() {
            return Err(anyhow!("vcpkg lib path not found: {}", lib_dir.display()));
        }

        return Ok(InstallOutcome::Installed {
            include_paths: vec![include],
            lib_paths: vec![lib_dir],
        });
    }
}

// Prefer PATH; fallback to %USERPROFILE%\vcpkg\vcpkg.exe
fn find_vcpkg() -> Option<String> {
    if let Some(p) = which_tool("vcpkg") { return Some(p); }
    if let Ok(home) = std::env::var("USERPROFILE") {
        let candidate = Path::new(&home).join("vcpkg").join("vcpkg.exe");
        if candidate.exists() {
            return Some(candidate.to_string_lossy().to_string());
        }
    }
    None
}
