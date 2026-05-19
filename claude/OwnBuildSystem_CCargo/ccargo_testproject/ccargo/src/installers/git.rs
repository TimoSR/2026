//! Git-based installer: clones the repo into ccargo home and (optionally) builds via CMake.

use anyhow::{anyhow, Context, Result};
use git2::{Repository, Oid};
use std::path::{Path, PathBuf};
use std::fs;
use crate::manifest::{Dependency, DepSource, CMakeConfig};
use crate::installers::{InstallOutcome};
use crate::util::{ccargo_home, ensure_dir, pretty_path, run_cmd, has_cmake};

#[derive(Clone)]
pub struct GitInstaller {
    root: PathBuf,
}

impl GitInstaller {
    pub fn new(root: PathBuf) -> Self {
        return Self { root };
    }

    fn dep_dir(&self, name: &str) -> PathBuf {
        return self.root.join(name);
    }

    fn clone_or_fetch(&self, url: &str, dir: &Path) -> Result<Repository> {
    if dir.exists() {
        let repo = Repository::open(dir)
            .with_context(|| format!("failed to open existing repository at {}", dir.display()))?;

        // Remote borrows from `repo`. Keep it in a smaller scope so it drops
        // before we return `repo`.
        {
            let mut remote = repo.find_remote("origin")
                .with_context(|| format!("failed to find 'origin' in {}", dir.display()))?;

            // Fetch both branches and tags so tag/branch checkouts succeed later.
            remote
                .fetch(
                    &[
                        "refs/heads/*:refs/remotes/origin/*",
                        "refs/tags/*:refs/tags/*",
                    ],
                    None,
                    None,
                )
                .with_context(|| format!("git fetch failed for {}", url))?;
        } // <- `remote` dropped here, borrow ends, now we can move `repo`.

        return Ok(repo);
    } else {
        let repo = Repository::clone(url, dir)
            .with_context(|| format!("git clone failed for {} into {}", url, dir.display()))?;
        return Ok(repo);
    }
}


    fn checkout_ref(repo: &Repository, tag: &Option<String>, branch: &Option<String>, rev: &Option<String>) -> Result<()> {
        if let Some(sha) = rev {
            let oid = Oid::from_str(sha)?;
            let obj = repo.find_object(oid, None)?;
            repo.checkout_tree(&obj, None)?;
            repo.set_head_detached(oid)?;
            return Ok(());
        }
        if let Some(t) = tag {
            let (obj, reference) = repo.revparse_ext(&format!("refs/tags/{}", t))?;
            repo.checkout_tree(&obj, None)?;
            if let Some(r) = reference { repo.set_head(r.name().unwrap())?; }
            return Ok(());
        }
        if let Some(b) = branch {
            let (obj, reference) = repo.revparse_ext(&format!("origin/{}", b))?;
            repo.checkout_tree(&obj, None)?;
            if let Some(r) = reference { repo.set_head(r.name().unwrap())?; }
            return Ok(());
        }
        // default: leave as-is (e.g., default branch)
        return Ok(());
    }

    fn maybe_cmake(dir: &Path, cfg: &Option<CMakeConfig>) -> Result<(Vec<PathBuf>, Vec<PathBuf>)> {
        if cfg.is_none() {
            let inc = dir.join("include");
            let lib = dir.join("lib");
            let include_paths = if inc.exists() { vec![inc] } else { vec![] };
            let lib_paths = if lib.exists() { vec![lib] } else { vec![] };
            return Ok((include_paths, lib_paths));
        }

        if !has_cmake()? {
            return Err(anyhow!("CMake not found on PATH, required by dependency with `cmake = true`"));
        }

        let cfg = cfg.as_ref().unwrap();
        let build_dir = dir.join("ccargo-build");
        ensure_dir(&build_dir)?;

        // Configure
        let mut args = vec!["-S", dir.to_str().unwrap(), "-B", build_dir.to_str().unwrap()];
        if let Some(g) = &cfg.generator {
            args.push("-G");
            args.push(g);
        }
        for opt in &cfg.options {
            args.push(opt);
        }
        run_cmd("cmake", &args)?;

        // Build (always pass --config Release; single-config generators ignore it)
        let build_args = ["--build", build_dir.to_str().unwrap(), "--config", "Release"];
        run_cmd("cmake", &build_args)?;

        // Discover include/lib paths:
        // include candidates
        let inc_candidates = vec![
            dir.join("include"),
            build_dir.join("include"),
            build_dir.join("Release").join("include"),
            build_dir.join("Debug").join("include"),
        ];
        // lib candidates — MSVC places .lib under <build>/Release or <build>/Debug
        let lib_candidates = vec![
            dir.join("lib"),
            build_dir.join("lib"),
            build_dir.join("Release"),
            build_dir.join("Debug"),
        ];

        let include_paths: Vec<PathBuf> = inc_candidates.into_iter().filter(|p| p.exists()).collect();
        let lib_paths: Vec<PathBuf> = lib_candidates.into_iter().filter(|p| p.exists()).collect();

        if include_paths.is_empty() && lib_paths.is_empty() {
            return Err(anyhow!("CMake finished but no include/lib directories were found under {}", build_dir.display()));
        }
        return Ok((include_paths, lib_paths));
    }

}

impl super::Installer for GitInstaller {
    fn install(&self, dep: &Dependency) -> Result<InstallOutcome> {
        let DepSource::Git { url, tag, branch, rev, cmake } = &dep.source else {
            return Err(anyhow!("GitInstaller received non-git dependency"));
        };

        let dir = self.dep_dir(&dep.name);
        ensure_dir(&self.root)?;
        let existed = dir.exists();

        let repo = self.clone_or_fetch(url, &dir)
            .with_context(|| format!("git fetch/clone failed for {}", url))?;
        Self::checkout_ref(&repo, tag, branch, rev)?;
        let (inc, libs) = Self::maybe_cmake(&dir, cmake)?;

        if existed {
            return Ok(InstallOutcome::AlreadyPresent { include_paths: inc, lib_paths: libs });
        } else {
            return Ok(InstallOutcome::Installed { include_paths: inc, lib_paths: libs });
        }
    }
}
