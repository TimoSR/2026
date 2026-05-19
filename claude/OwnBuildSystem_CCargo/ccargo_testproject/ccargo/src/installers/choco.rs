//! Chocolatey-based installer (Windows only).
//! Installs packages like SDL2 and discovers include/lib/link names.

use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;
use crate::manifest::{Dependency, DepSource};
use crate::installers::InstallOutcome;
use crate::util::{run_cmd, which_tool};

#[cfg(windows)]
use winreg::enums::*;
#[cfg(windows)]
use winreg::RegKey;

#[derive(Default, Clone)]
pub struct ChocoInstaller;

impl super::Installer for ChocoInstaller {
    fn install(&self, dep: &Dependency) -> Result<InstallOutcome> {
        let DepSource::Choco { package, version } = &dep.source else {
            return Err(anyhow!("ChocoInstaller received non-choco dependency"));
        };

        #[cfg(not(windows))]
        {
            return Err(anyhow!("Chocolatey is only supported on Windows"));
        }

        #[cfg(windows)]
        {
            // Ensure choco exists
            if which_tool("choco").is_none() {
                return Err(anyhow!("Chocolatey (choco) was not found on PATH"));
            }

            // Install or upgrade
            let mut install_args = vec!["install", package.as_str(), "-y"];
            if let Some(v) = version {
                install_args.push("--version");
                install_args.push(v.as_str());
            }
            // We don't treat a re-install as fatal; try install, if fails, try upgrade.
            if let Err(e) = run_cmd("choco", &install_args) {
                // Fallback to upgrade if already installed
                let _ = run_cmd("choco", &["upgrade", package.as_str(), "-y"]);
                // If both fail, bubble the original error
                if which_tool("choco").is_none() {
                    return Err(e);
                }
            }

            // Attempt to discover typical include/lib paths for dev packages.
            // For SDL2, often installed under C:\ProgramData\chocolatey\lib\SDL2*\
            let (include_paths, lib_paths) = discover_paths_windows(package)
                .with_context(|| format!("failed to discover include/lib paths for '{}'", package))?;

            return Ok(InstallOutcome::Installed { include_paths, lib_paths });
        }
    }
}

#[cfg(windows)]
fn discover_paths_windows(package: &str) -> Result<(Vec<PathBuf>, Vec<PathBuf>)> {
    let mut include_paths = Vec::new();
    let mut lib_paths = Vec::new();

    // Chocolatey lib location
    let base = std::path::Path::new("C:\\ProgramData\\chocolatey\\lib");
    if base.exists() {
        for entry in std::fs::read_dir(base)? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if !name.to_lowercase().starts_with(&package.to_lowercase()) {
                continue;
            }
            // Heuristics for common layouts: <pkg>\build\include, <pkg>\include, <pkg>\lib, <pkg>\build\lib
            let cand_inc = [
                path.join("build").join("include"),
                path.join("include"),
            ];
            let cand_lib = [
                path.join("build").join("lib"),
                path.join("lib"),
            ];
            for p in cand_inc {
                if p.exists() { include_paths.push(p); }
            }
            for p in cand_lib {
                if p.exists() { lib_paths.push(p); }
            }
        }
    }

    if include_paths.is_empty() && lib_paths.is_empty() {
        return Err(anyhow!("no include/lib directories found under Chocolatey for '{}'", package));
    }

    return Ok((include_paths, lib_paths));
}
