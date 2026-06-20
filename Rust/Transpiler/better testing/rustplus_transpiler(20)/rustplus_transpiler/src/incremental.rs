use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

pub const CACHE_VERSION: u32 = 1;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IncrementalCache {
    pub version: u32,
    pub entries: BTreeMap<String, CacheEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub source_hash: String,
    pub config_hash: String,
    pub generated_hash: String,
    pub output_path: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CacheStats {
    pub hits: usize,
    pub misses: usize,
    pub writes: usize,
}

impl IncrementalCache {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self {
                version: CACHE_VERSION,
                entries: BTreeMap::new(),
            });
        }

        let source = fs::read_to_string(path).with_context(|| format!("failed to read incremental cache {}", path.display()))?;
        let mut cache = toml::from_str::<Self>(&source)
            .with_context(|| format!("failed to parse incremental cache {}", path.display()))?;

        if cache.version != CACHE_VERSION {
            cache.version = CACHE_VERSION;
            cache.entries.clear();
        }

        return Ok(cache);
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| format!("failed to create cache directory {}", parent.display()))?;
        }

        let serialized = toml::to_string_pretty(self).context("failed to serialize incremental cache")?;
        fs::write(path, serialized).with_context(|| format!("failed to write incremental cache {}", path.display()))?;
        return Ok(());
    }

    pub fn is_fresh(&self, input_path: &Path, output_path: &Path, source: &str, config_fingerprint: &str) -> bool {
        if !output_path.exists() {
            return false;
        }

        let key = cache_key(input_path);
        let Some(entry) = self.entries.get(&key) else {
            return false;
        };

        return entry.source_hash == hash_string(source)
            && entry.config_hash == config_fingerprint
            && entry.output_path == output_path.display().to_string();
    }

    pub fn update(
        &mut self,
        input_path: &Path,
        output_path: &Path,
        source: &str,
        generated: &str,
        config_fingerprint: &str,
    ) {
        self.entries.insert(
            cache_key(input_path),
            CacheEntry {
                source_hash: hash_string(source),
                config_hash: config_fingerprint.to_string(),
                generated_hash: hash_string(generated),
                output_path: output_path.display().to_string(),
            },
        );
    }
}

pub fn default_cache_path(manifest_dir: &Path) -> PathBuf {
    return manifest_dir.join("target/rustplus/cache.toml");
}

pub fn config_fingerprint(value: &impl std::fmt::Debug) -> String {
    return hash_string(&format!("{value:#?}"));
}

pub fn hash_string(value: &str) -> String {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    return format!("{:016x}", hasher.finish());
}

fn cache_key(path: &Path) -> String {
    return path.display().to_string().replace('\\', "/");
}
