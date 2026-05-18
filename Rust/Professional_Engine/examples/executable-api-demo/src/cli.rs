use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "executable-api-demo")]
#[command(about = "Demonstrates process-style APIs for compiled executables")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Run(RunArgs),
    Benchmark(BenchmarkArgs),
    ValidateAssets(ValidateAssetsArgs),
    HeadlessServer(HeadlessServerArgs),
}

#[derive(Debug, Args)]
pub struct RunArgs {
    #[arg(long, default_value_t = 6)]
    pub frames: u64,
    #[arg(long, conflicts_with = "config_stdin")]
    pub config: Option<PathBuf>,
    #[arg(long, conflicts_with = "config")]
    pub config_stdin: bool,
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct BenchmarkArgs {
    #[arg(long, default_value_t = 200_000)]
    pub iterations: u64,
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct ValidateAssetsArgs {
    #[arg(long)]
    pub manifest: PathBuf,
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct HeadlessServerArgs {
    #[arg(long, default_value = "127.0.0.1")]
    pub bind: String,
    #[arg(long, default_value_t = 5001)]
    pub port: u16,
    #[arg(long, default_value_t = 1)]
    pub max_clients: u32,
    #[arg(long)]
    pub live: bool,
    #[arg(long)]
    pub json: bool,
}
