//! ccargo: a user-first C++ build tool.
//!
//! Highlights:
//! - Self-documenting, readable code (explicit control flow and naming).
//! - Friendly CLI with clear defaults.
//! - Installs dependencies from Git and Chocolatey.
//! - Builds simple C++ projects without writing CMake by hand.
//!
//! Commands:
//!   ccargo init
//!   ccargo add <name> --git <url> [--tag <t> | --branch <b> | --rev <sha>] [--cmake]
//!   ccargo add <name> --choco <pkg> [--version <v>]
//!   ccargo resolve
//!   ccargo build [--release] [--compiler g++|clang++|cl] [--std c++20]
//!   ccargo run [--release] [-- <args...>]

mod manifest;
mod build;
mod util;
mod installers;

use crate::installers::{Installer, GitInstaller, ChocoInstaller, VcpkgInstaller, InstallOutcome};

use crate::manifest::{Manifest, Dependency, DepSource, CMakeConfig};
use crate::build::{BuildMode, BuildRequest, build_project};
use crate::util::{ccargo_home, ensure_dir, pretty_path};

use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{info, warn, error, Level};
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(name = "ccargo", version, about = "A user-first C++ build tool")]
struct Cli {
    /// Increase output verbosity (use multiple times for more detail)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new ccargo manifest and skeleton src/main.cpp
    Init {
        /// Project name (defaults to current dir name)
        #[arg(long)]
        name: Option<String>,
    },

    /// Add a dependency from git or a package manager
    Add {
        /// Local name for the dependency (used in include/lib resolution)
        name: String,

        /// Git URL (e.g. https://github.com/fmtlib/fmt.git)
        #[arg(long, conflicts_with = "choco")]
        git: Option<String>,

        /// Git reference selection
        #[arg(long, requires = "git")]
        tag: Option<String>,
        #[arg(long, requires = "git")]
        branch: Option<String>,
        #[arg(long, requires = "git")]
        rev: Option<String>,

        /// If set, attempt to configure & build with CMake after fetching
        #[arg(long, requires = "git")]
        cmake: bool,

        /// Extra CMake options (e.g. -DFOO=ON)
        #[arg(long, num_args=1.., requires = "cmake")]
        cmake_opts: Vec<String>,

        /// Chocolatey package name (Windows)
        #[arg(long, conflicts_with = "git")]
        choco: Option<String>,

        /// Package version (Windows)
        #[arg(long)]
        version: Option<String>,
    },

    /// Resolve and install all dependencies into the local cache
    Resolve {},

    /// Compile the project
    Build {
        #[arg(long)]
        release: bool,

        /// Choose compiler explicitly
        #[arg(long)]
        compiler: Option<String>,

        /// C++ standard, e.g. c++17, c++20, c++23
        #[arg(long, default_value = "c++20")]
        std: String,
    },

    /// Build (if needed) and run the produced binary
    Run {
        #[arg(long)]
        release: bool,

        /// Pass remaining args to the program
        #[arg(last = true)]
        prog_args: Vec<String>,
    },
}

fn main() -> Result<()> {
    // Tracing setup with sensible defaults, override with RUST_LOG
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,ccargo=info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => {
            run_init(name.as_deref())?;
            return Ok(());
        }
        Commands::Add { name, git, tag, branch, rev, cmake, cmake_opts, choco, version } => {
            run_add(&name, git.as_deref(), tag.as_deref(), branch.as_deref(), rev.as_deref(), cmake, &cmake_opts, choco.as_deref(), version.as_deref())?;
            return Ok(());
        }
        Commands::Resolve {} => {
            run_resolve()?;
            return Ok(());
        }
        Commands::Build { release, compiler, std } => {
            run_build(release, compiler.as_deref(), &std)?;
            return Ok(());
        }
        Commands::Run { release, prog_args } => {
            run_build(release, None, "c++20")?;
            let out = build::artifact_path(release)?;
            info!("Running {}", pretty_path(&out));
            let status = std::process::Command::new(out)
                .args(&prog_args)
                .status()
                .context("failed to execute program")?;
            if !status.success() {
                return Err(anyhow!("program exited with status {}", status));
            }
            return Ok(());
        }
    }
}

fn run_init(name: Option<&str>) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let project_name = match name {
        Some(n) => n.to_string(),
        None => cwd.file_name().unwrap().to_string_lossy().to_string(),
    };

    // Write manifest if absent
    let manifest_path = cwd.join("ccargo.toml");
    if !manifest_path.exists() {
        let manifest = Manifest::new(project_name.clone());
        let text = toml::to_string_pretty(&manifest)?;
        fs::write(&manifest_path, text)?;
        info!("Created {}", pretty_path(&manifest_path));
    } else {
        warn!("Manifest already exists: {}", pretty_path(&manifest_path));
    }

    // Create src/main.cpp if absent
    let src = cwd.join("src");
    ensure_dir(&src)?;
    let main_cpp = src.join("main.cpp");
    if !main_cpp.exists() {
        fs::write(&main_cpp, DEFAULT_MAIN_CPP)?;
        info!("Created {}", pretty_path(&main_cpp));
    } else {
        warn!("Source already exists: {}", pretty_path(&main_cpp));
    }

    info!("Initialized project '{}'", project_name);
    return Ok(());
}

const DEFAULT_MAIN_CPP: &str = r#"#include <iostream>

int main() {
    std::cout << "hello from ccargo 👋\n";
    return 0;
}
"#;

fn run_add(
    name: &str,
    git: Option<&str>,
    tag: Option<&str>,
    branch: Option<&str>,
    rev: Option<&str>,
    cmake: bool,
    cmake_opts: &[String],
    choco: Option<&str>,
    version: Option<&str>,
) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let manifest_path = cwd.join("ccargo.toml");
    let mut manifest = Manifest::read(&manifest_path)
        .with_context(|| format!("unable to read {}", pretty_path(&manifest_path)))?;

    let dep = if let Some(url) = git {
        Dependency {
            name: name.to_string(),
            source: DepSource::Git {
                url: url.to_string(),
                tag: tag.map(|s| s.to_string()),
                branch: branch.map(|s| s.to_string()),
                rev: rev.map(|s| s.to_string()),
                cmake: if cmake { Some(CMakeConfig { generator: None, options: cmake_opts.to_vec() }) } else { None },
            },
        }
    } else if let Some(pkg) = choco {
        Dependency {
            name: name.to_string(),
            source: DepSource::Choco {
                package: pkg.to_string(),
                version: version.map(|s| s.to_string()),
            },
        }
    } else {
        return Err(anyhow!("specify either --git <url> or --choco <pkg>"));
    };

    manifest.add_dependency(dep)?;
    manifest.write(&manifest_path)?;
    info!("Updated {}", pretty_path(&manifest_path));
    info!("Run `ccargo resolve` to install new dependencies.");
    return Ok(());
}

fn run_resolve() -> Result<()> {
    let cwd = std::env::current_dir()?;
    let manifest_path = cwd.join("ccargo.toml");
    let manifest = Manifest::read(&manifest_path)?;

    let home = ccargo_home()?;
    ensure_dir(&home)?;

    let dep_cache = home.join("deps");
    ensure_dir(&dep_cache)?;

    for dep in &manifest.dependencies {
        match &dep.source {
            DepSource::Git { .. } => {
                let installer = GitInstaller::new(dep_cache.clone());
                let res = installer.install(dep)?;
                log_outcome(dep, &res);
            }
            DepSource::Choco { .. } => {
                #[cfg(windows)]
                {
                    let installer = ChocoInstaller::default();
                    let res = installer.install(dep)?;
                    log_outcome(dep, &res);
                }
                #[cfg(not(windows))]
                {
                    return Err(anyhow!("Chocolatey installer is only available on Windows"));
                }
            }
            DepSource::Vcpkg { .. } => {
                let installer = VcpkgInstaller::default();
                let res = installer.install(dep)?;
                log_outcome(dep, &res);
            }
        }
    }

    info!("Resolve complete.");
    return Ok(());
}

fn log_outcome(dep: &Dependency, res: &InstallOutcome) {
    match res {
        InstallOutcome::AlreadyPresent { include_paths, lib_paths } => {
            info!("{}: already installed", dep.name);
            for p in include_paths { info!("  include: {}", p.display()); }
            for p in lib_paths { info!("  lib:     {}", p.display()); }
        }
        InstallOutcome::Installed { include_paths, lib_paths } => {
            info!("{}: installed", dep.name);
            for p in include_paths { info!("  include: {}", p.display()); }
            for p in lib_paths { info!("  lib:     {}", p.display()); }
        }
    }
}

fn run_build(release: bool, compiler: Option<&str>, std: &str) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let manifest_path = cwd.join("ccargo.toml");
    let manifest = Manifest::read(&manifest_path)?;

    let req = BuildRequest {
        mode: if release { BuildMode::Release } else { BuildMode::Debug },
        cxx: compiler.map(|s| s.to_string()),
        cpp_std: std.to_string(),
        manifest,
    };

    build_project(&req)?;
    return Ok(());
}
