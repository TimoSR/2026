//! Utilities for path management, processes, and tool detection.

use anyhow::{anyhow, Context, Result};
use directories::ProjectDirs;
use std::path::{Path, PathBuf};
use std::fs;

pub fn ccargo_home() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("io", "ccargo", "ccargo")
        .ok_or_else(|| anyhow!("failed to determine ccargo home"))?;
    return Ok(dirs.data_local_dir().to_path_buf());
}

pub fn ensure_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path).with_context(|| format!("failed to create {}", path.display()))?;
    }
    return Ok(());
}

pub fn pretty_path<P: AsRef<Path>>(p: P) -> String {
    return p.as_ref().to_string_lossy().to_string();
}

pub fn which_tool(tool: &str) -> Option<String> {
    if let Ok(path) = which::which(tool) {
        return Some(path.to_string_lossy().to_string());
    }
    return None;
}

pub fn run_cmd(cmd: &str, args: &[&str]) -> Result<()> {
    let mut c = std::process::Command::new(cmd);
    c.args(args);
    let status = c.status().with_context(|| format!("failed to run {} {:?}", cmd, args))?;
    if !status.success() {
        return Err(anyhow!("command failed: {} {:?}", cmd, args));
    }
    return Ok(());
}

pub fn run_cmd_capture(cmd: &str, args: &[&str]) -> Result<String> {
    let out = std::process::Command::new(cmd)
        .args(args)
        .output()
        .with_context(|| format!("failed to run {} {:?}", cmd, args))?;
    if !out.status.success() {
        return Err(anyhow!("command failed: {} {:?}\nstdout: {}\nstderr: {}",
            cmd, args,
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)));
    }
    return Ok(String::from_utf8_lossy(&out.stdout).to_string());
}

pub fn has_cmake() -> Result<bool> {
    return Ok(which_tool("cmake").is_some());
}
