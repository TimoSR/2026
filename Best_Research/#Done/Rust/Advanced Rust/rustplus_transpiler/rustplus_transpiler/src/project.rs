use crate::program::RustPlusProgram;
use crate::transpiler::SemanticContext;
use crate::{RustPlusConfig, Transpiler};
use anyhow::{anyhow, Context, Result};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

pub fn transpile_file(input: &Path, output: &Path, config: RustPlusConfig) -> Result<()> {
    return transpile_file_with_rustfmt(input, output, &config, is_rustfmt_available());
}

pub fn transpile_directory(input_dir: &Path, out_dir: &Path, config: RustPlusConfig) -> Result<()> {
    let rustfmt_available = is_rustfmt_available();
    let mut rp_sources = Vec::new();

    for entry in WalkDir::new(input_dir).into_iter() {
        let entry = entry.with_context(|| format!("failed to walk {}", input_dir.display()))?;
        let path = entry.path();

        if !path.is_file() || path.extension() != Some(OsStr::new("rp")) {
            continue;
        }

        let source = fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
        rp_sources.push((path.to_path_buf(), source));
    }

    rp_sources.sort_by(|left, right| left.0.cmp(&right.0));
    let mut program = RustPlusProgram::from_named_sources(rp_sources.clone(), config)?;

    for (index, (path, _source)) in rp_sources.iter().enumerate() {
        let relative_path = path.strip_prefix(input_dir).with_context(|| {
            format!(
                "failed to compute relative path for {} under {}",
                path.display(),
                input_dir.display()
            )
        })?;

        let mut output_path = out_dir.join(relative_path);
        output_path.set_extension("rs");
        let emit_output = program.emit_file(index, Some(output_path.clone()))?;

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).with_context(|| format!("failed to create {}", parent.display()))?;
        }

        fs::write(&output_path, emit_output.rust).with_context(|| format!("failed to write {}", output_path.display()))?;

        if rustfmt_available {
            run_rustfmt(&output_path)?;
        }
    }

    return Ok(());
}

fn transpile_file_with_rustfmt(
    input: &Path,
    output: &Path,
    config: &RustPlusConfig,
    rustfmt_available: bool,
) -> Result<()> {
    let source = fs::read_to_string(input).with_context(|| format!("failed to read {}", input.display()))?;
    return transpile_source_with_rustfmt(&source, output, config, None, rustfmt_available);
}

fn transpile_source_with_rustfmt(
    source: &str,
    output: &Path,
    config: &RustPlusConfig,
    project_context: Option<&SemanticContext>,
    rustfmt_available: bool,
) -> Result<()> {
    let transpiler = Transpiler::with_config(source.to_string(), config.clone());
    let generated = match project_context {
        Some(context) => transpiler.transpile_with_project_context(context)?,
        None => transpiler.transpile()?,
    };

    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent).with_context(|| format!("failed to create {}", parent.display()))?;
    }

    fs::write(output, generated).with_context(|| format!("failed to write {}", output.display()))?;

    if rustfmt_available {
        run_rustfmt(output)?;
    }

    return Ok(());
}

pub fn run_rustfmt_if_available(path: &Path) -> Result<()> {
    if !is_rustfmt_available() {
        return Ok(());
    }

    return run_rustfmt(path);
}

pub fn write_example_project(out: &Path) -> Result<()> {
    fs::create_dir_all(out.join("src")).with_context(|| format!("failed to create {}", out.display()))?;

    let rustplus_path = std::env::current_dir()
        .with_context(|| "failed to resolve current Rust Plus project directory")?
        .display()
        .to_string()
        .replace('\\', "\\\\");
    let cargo_toml = EXAMPLE_CARGO_TOML.replace("{RUSTPLUS_PATH}", &rustplus_path);

    fs::write(out.join("Cargo.toml"), cargo_toml)?;
    fs::write(out.join("rustplus.toml"), EXAMPLE_RUSTPLUS_TOML)?;
    fs::write(out.join("build.rs"), EXAMPLE_BUILD_RS)?;
    fs::write(out.join("src/main.rp"), EXAMPLE_MAIN_RP)?;
    fs::write(out.join("src/account.rp"), EXAMPLE_ACCOUNT_RP)?;

    return Ok(());
}

pub fn find_default_config(start: &Path) -> Option<PathBuf> {
    let mut current = if start.is_file() {
        start.parent()?.to_path_buf()
    } else {
        start.to_path_buf()
    };

    loop {
        let candidate = current.join("rustplus.toml");
        if candidate.is_file() {
            return Some(candidate);
        }

        if !current.pop() {
            return None;
        }
    }
}

fn is_rustfmt_available() -> bool {
    return Command::new("rustfmt").arg("--version").output().is_ok();
}

fn run_rustfmt(path: &Path) -> Result<()> {
    let status = Command::new("rustfmt")
        .arg(path)
        .status()
        .with_context(|| format!("failed to execute rustfmt for {}", path.display()))?;

    if !status.success() {
        return Err(anyhow!("rustfmt failed for {}", path.display()));
    }

    return Ok(());
}

const EXAMPLE_CARGO_TOML: &str = r#"[package]
name = "rustplus-example"
version = "0.1.0"
edition = "2021"
build = "build.rs"

[build-dependencies]
rustplus = { path = "{RUSTPLUS_PATH}" }

[[bin]]
name = "rustplus-example"
path = "src/main.rs"
"#;

const EXAMPLE_BUILD_RS: &str = r#"fn main() {
    rustplus::cargo_integration::compile_sibling_files("src")
        .expect("failed to generate Rust files from .rp files");
}
"#;

const EXAMPLE_RUSTPLUS_TOML: &str = r#"[features]
classes = true
interfaces = true
abstract_classes = true
multiple_bases = true
visibility_modifiers = true
this_receiver = true
composition_bases = true
generics = true
csharp_variable_declarations = false
new_expressions = false
interface_object_sugar = true
"#;

const EXAMPLE_MAIN_RP: &str = r#"mod account;

use account::{Account, IAccount};

fn main()
{
    let mut account: IAccount = Account::Heap("account-1");

    account.deposit(250);
    account.deposit(125);

    println!("{}", account.id());
    println!("balance = {}", account.balance());
}
"#;

const EXAMPLE_ACCOUNT_RP: &str = r#"pub interface IAccount
{
    fn id(&self) -> &str;
    fn deposit(&mut self, amount: i64);
    fn balance(&self) -> i64;
}

pub class Account : IAccount
{
    id: String;
    balance: i64;

    pub fn new(id: String) -> Self
    {
        return Self
        {
            id,
            balance: 0,
        };
    }

    pub fn deposit(&mut self, amount: i64)
    {
        self.balance += amount;
    }

    pub fn id(&self) -> &str
    {
        return &self.id;
    }

    pub fn balance(&self) -> i64
    {
        return self.balance;
    }
}
"#;
