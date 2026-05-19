//! Build system: compile src/**/*.cpp with discovered include/lib paths and link libs.

use anyhow::{anyhow, Context, Result};
use glob::glob;
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::info;

use crate::manifest::Manifest;
use crate::util::{ensure_dir, pretty_path, which_tool};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildMode {
    Debug,
    Release,
}

#[derive(Debug, Clone)]
pub struct BuildRequest {
    pub mode: BuildMode,
    pub cxx: Option<String>,
    pub cpp_std: String,
    pub manifest: Manifest,
}

pub fn artifact_path(release: bool) -> Result<PathBuf> {
    let manifest = Manifest::read(&std::env::current_dir()?.join("ccargo.toml"))?;
    let out_dir = Path::new(&manifest.project.out_dir);
    let exe_name = if cfg!(windows) {
        format!("{}.exe", manifest.project.target)
    } else {
        manifest.project.target.clone()
    };
    return Ok(out_dir.join(exe_name));
}

pub fn build_project(req: &BuildRequest) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let src_glob = cwd.join("src").join("**").join("*.cpp");
    let pattern = src_glob.to_string_lossy().to_string();
    let sources: Vec<PathBuf> = glob(&pattern)?.filter_map(|e| e.ok()).collect();

    if sources.is_empty() {
        return Err(anyhow!("no C++ sources found under src/**/*.cpp"));
    }

    let out_dir = cwd.join(&req.manifest.project.out_dir);
    ensure_dir(&out_dir)?;

    let obj_dir = out_dir.join("obj");
    ensure_dir(&obj_dir)?;

    // Compile each source into an object file
    let cxx = select_compiler(req.cxx.as_deref())?;
    info!("Using compiler: {}", cxx);

    let mut include_args: Vec<String> = vec![];
    for inc in &req.manifest.install.include_paths {
        include_args.push(format!("-I{}", inc.path));
    }

    // Windows cl.exe uses /I instead of -I; adapt flags below
    let is_msvc = cxx.to_lowercase().ends_with("cl.exe") || cxx.to_lowercase() == "cl";

    let mut obj_files = Vec::new();
    for src in &sources {
        let obj_name = src.file_stem().unwrap().to_string_lossy().to_string() + ".o";
        let obj_path = obj_dir.join(obj_name);
        compile_one(&cxx, src, &obj_path, &include_args, &req.cpp_std, req.mode, is_msvc)?;
        obj_files.push(obj_path);
    }

    // Link step
    let out = artifact_path(req.mode == BuildMode::Release)?;
    link_all(&cxx, &obj_files, &out, &req.manifest, is_msvc)?;
    info!("Built {}", pretty_path(&out));
    return Ok(());
}

fn select_compiler(preferred: Option<&str>) -> Result<String> {
    if let Some(p) = preferred {
        return Ok(p.to_string());
    }
    // detection order: cl (Windows), clang++, g++
    if cfg!(windows) {
        if let Some(p) = which_tool("cl.exe") { return Ok(p); }
        if let Some(p) = which_tool("clang++") { return Ok(p); }
        if let Some(p) = which_tool("g++") { return Ok(p); }
    } else {
        if let Some(p) = which_tool("clang++") { return Ok(p); }
        if let Some(p) = which_tool("g++") { return Ok(p); }
    }
    return Err(anyhow!("no suitable C++ compiler found (tried cl.exe/clang++/g++)"));
}

fn compile_one(
    cxx: &str,
    src: &Path,
    obj: &Path,
    include_args: &[String],
    cpp_std: &str,
    mode: BuildMode,
    is_msvc: bool,
) -> Result<()> {
    let mut cmd = Command::new(cxx);

    if is_msvc {
        // MSVC expects /std:c++20 (no space).
        if !cpp_std.trim().is_empty() {
            cmd.arg(format!("/std:{}", cpp_std));
        }
        cmd.arg("/nologo")
            .arg("/c")
            .arg(format!("/Fo:{}", obj.to_str().unwrap()));
        for inc in include_args {
            // convert -Ipath -> /Ipath
            let val = inc.replacen("-I", "/I", 1);
            cmd.arg(val);
        }
        if mode == BuildMode::Debug { cmd.arg("/Zi"); } else { cmd.arg("/O2"); }
        cmd.arg(src);
    } else {
        // GCC/Clang expects a single argument: -std=c++20
        if !cpp_std.trim().is_empty() {
            cmd.arg(format!("-std={}", cpp_std));
        }
        cmd.arg("-c")
            .arg(src)
            .arg("-o").arg(obj);
        if mode == BuildMode::Debug { cmd.arg("-g"); } else { cmd.arg("-O3"); }
        for inc in include_args {
            cmd.arg(inc);
        }
    }

    let status = cmd.status().with_context(|| format!("compile failed: {}", src.display()))?;
    if !status.success() {
        return Err(anyhow!("compile failed for {}", src.display()));
    }
    return Ok(());
}

fn link_all(
    cxx: &str,
    objs: &[PathBuf],
    out: &Path,
    manifest: &Manifest,
    is_msvc: bool,
) -> Result<()> {
    let mut cmd = std::process::Command::new(cxx);

    if is_msvc {
        // cl.exe linking: cl <objs> /link /OUT:app.exe /LIBPATH:... libname.lib
        for o in objs { cmd.arg(o); }
        cmd.arg("/link").arg(format!("/OUT:{}", out.to_str().unwrap()));
        for lp in &manifest.install.lib_paths {
            cmd.arg(format!("/LIBPATH:{}", lp.path));
        }
        for lib in &manifest.install.link_libs {
            // For MSVC, link libs usually "SDL2.lib", "SDL2main.lib", etc.
            cmd.arg(&lib.value);
        }
    } else {
        // POSIX-style link via g++/clang++
        cmd.args(objs);
        cmd.arg("-o").arg(out);
        for lp in &manifest.install.lib_paths {
            cmd.arg("-L").arg(&lp.path);
        }
        for lib in &manifest.install.link_libs {
            // For GCC/Clang, accept either "-lX" directly or plain "X" (we add -l)
            if lib.value.starts_with("-l") {
                cmd.arg(&lib.value);
            } else {
                cmd.arg(format!("-l{}", lib.value));
            }
        }
    }

    let status = cmd.status().context("link step failed")?;
    if !status.success() {
        return Err(anyhow!("linker failed"));
    }
    return Ok(());
}
