mod cli;
mod commands;
mod error;
mod output;

use std::process::ExitCode;

use clap::Parser;
use cli::{Cli, Commands};

fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Run(args) => commands::run::execute(args),
        Commands::Benchmark(args) => commands::benchmark::execute(args),
        Commands::ValidateAssets(args) => commands::validate_assets::execute(args),
        Commands::HeadlessServer(args) => commands::headless_server::execute(args),
    };

    match result {
        Ok(response) => {
            output::emit(response);
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::from(err.code() as u8)
        }
    }
}
