mod cli;

use std::process::ExitCode;

use game_client::{GameConfig, run};

fn main() -> ExitCode {
    match try_main() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("fatal error: {err}");
            ExitCode::FAILURE
        }
    }
}

fn try_main() -> engine_core::Result<()> {
    let max_frames = cli::parse_max_frames()?;
    run(GameConfig { max_frames })
}
