use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use rustplus::config::RustPlusConfig;
use rustplus::features::FeaturePipeline;
use rustplus::program::RustPlusProgram;
use rustplus::project::{find_default_config, transpile_directory, transpile_file, write_example_project};
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        CommandKind::Transpile {
            input,
            out,
            config,
            enable_feature,
            disable_feature,
            diagnostics,
            extended_diagnostics,
        } => {
            let config = load_config(config.or_else(|| find_default_config(&input)), enable_feature, disable_feature)?;
            transpile_file(&input, &out, config.clone())?;
            println!("generated {}", out.display());

            if diagnostics || extended_diagnostics {
                let source = fs::read_to_string(&input).with_context(|| format!("failed to read {}", input.display()))?;
                let mut program = RustPlusProgram::from_named_source(input, source, config)?;
                let _ = program.emit_file(0, Some(out))?;
                eprintln!("{}", program.format_report(extended_diagnostics));
            }
        }
        CommandKind::TranspileDir {
            input_dir,
            out_dir,
            config,
            enable_feature,
            disable_feature,
        } => {
            let config = load_config(config.or_else(|| find_default_config(&input_dir)), enable_feature, disable_feature)?;
            transpile_directory(&input_dir, &out_dir, config)?;
            println!("generated .rs files in {}", out_dir.display());
        }
        CommandKind::Check {
            input,
            config,
            enable_feature,
            disable_feature,
            diagnostics,
            extended_diagnostics,
        } => {
            let config = load_config(config.or_else(|| find_default_config(&input)), enable_feature, disable_feature)?;
            let source = fs::read_to_string(&input).with_context(|| format!("failed to read {}", input.display()))?;
            let mut program = RustPlusProgram::from_named_source(input.clone(), source, config)?;
            let emit_output = program.emit_file(0, None)?;
            println!("{}", emit_output.rust);

            if diagnostics || extended_diagnostics {
                eprintln!("{}", program.format_report(extended_diagnostics));
            }
        }
        CommandKind::Example { out } => {
            write_example_project(&out)?;
            println!("wrote example project to {}", out.display());
        }
        CommandKind::Features => {
            let feature_pipeline = FeaturePipeline::default();
            for name in feature_pipeline.names() {
                println!("{}", name);
            }
        }
    }

    return Ok(());
}

#[derive(Parser, Debug)]
#[command(name = "rustplus")]
#[command(about = "Modular .rp Rust Plus transpiler", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: CommandKind,
}

#[derive(Subcommand, Debug)]
enum CommandKind {
    /// Transpile one .rp file into one .rs file.
    Transpile {
        input: PathBuf,

        #[arg(short, long)]
        out: PathBuf,

        #[arg(long)]
        config: Option<PathBuf>,

        #[arg(long = "enable-feature")]
        enable_feature: Vec<String>,

        #[arg(long = "disable-feature")]
        disable_feature: Vec<String>,

        #[arg(long)]
        diagnostics: bool,

        #[arg(long = "extended-diagnostics")]
        extended_diagnostics: bool,
    },

    /// Transpile every .rp file in a directory tree into .rs files.
    TranspileDir {
        input_dir: PathBuf,

        #[arg(short, long)]
        out_dir: PathBuf,

        #[arg(long)]
        config: Option<PathBuf>,

        #[arg(long = "enable-feature")]
        enable_feature: Vec<String>,

        #[arg(long = "disable-feature")]
        disable_feature: Vec<String>,
    },

    /// Print transpiled Rust to stdout without writing a file.
    Check {
        input: PathBuf,

        #[arg(long)]
        config: Option<PathBuf>,

        #[arg(long = "enable-feature")]
        enable_feature: Vec<String>,

        #[arg(long = "disable-feature")]
        disable_feature: Vec<String>,

        #[arg(long)]
        diagnostics: bool,

        #[arg(long = "extended-diagnostics")]
        extended_diagnostics: bool,
    },

    /// Create a tiny example Rust Plus project.
    Example {
        #[arg(short, long, default_value = "rustplus-example")]
        out: PathBuf,
    },

    /// List available language feature flags.
    Features,
}

fn load_config(
    config_path: Option<PathBuf>,
    enable_features: Vec<String>,
    disable_features: Vec<String>,
) -> Result<RustPlusConfig> {
    let mut config = RustPlusConfig::load_optional(config_path.as_deref())?;

    for feature_name in disable_features {
        config.features.set_by_name(&feature_name, false)?;
    }

    for feature_name in enable_features {
        config.features.set_by_name(&feature_name, true)?;
    }

    return Ok(config);
}
