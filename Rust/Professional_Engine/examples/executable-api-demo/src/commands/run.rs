use serde::{Deserialize, Serialize};

use std::io::Read;

use crate::cli::RunArgs;
use crate::commands::CommandResponse;
use crate::error::AppError;

#[derive(Debug, Deserialize)]
struct RunInputConfig {
    frames: Option<u64>,
    scene: Option<String>,
}

#[derive(Debug, Serialize)]
struct RunReport {
    command: &'static str,
    frames: u64,
    config_source: &'static str,
    scene: String,
    tick_preview: Vec<u64>,
    total_ticks: u64,
}

pub fn execute(args: RunArgs) -> Result<CommandResponse, AppError> {
    let mut frames = args.frames;
    let mut scene = String::from("default-scene");
    let mut config_source = "cli";

    if let Some(path) = args.config {
        let text = std::fs::read_to_string(&path).map_err(|err| {
            AppError::io(format!(
                "failed to read config file `{}`: {err}",
                path.display()
            ))
        })?;
        let config: RunInputConfig = serde_json::from_str(&text).map_err(|err| {
            AppError::cli(format!(
                "invalid json in config file `{}`: {err}",
                path.display()
            ))
        })?;

        if let Some(file_frames) = config.frames {
            frames = file_frames;
        }
        if let Some(file_scene) = config.scene {
            scene = file_scene;
        }

        config_source = "file";
    }

    if args.config_stdin {
        let mut input = String::new();
        std::io::stdin()
            .read_to_string(&mut input)
            .map_err(|err| AppError::io(format!("failed to read config from stdin: {err}")))?;

        let config: RunInputConfig = serde_json::from_str(&input)
            .map_err(|err| AppError::cli(format!("invalid json from stdin: {err}")))?;

        if let Some(stdin_frames) = config.frames {
            frames = stdin_frames;
        }
        if let Some(stdin_scene) = config.scene {
            scene = stdin_scene;
        }

        config_source = "stdin";
    }

    let tick_preview: Vec<u64> = (0..frames.min(12)).collect();

    let report = RunReport {
        command: "run",
        frames,
        config_source,
        scene,
        total_ticks: frames,
        tick_preview,
    };

    let human_body = format!(
        "[run] scene={} source={} total_ticks={} preview={:?}",
        report.scene, report.config_source, report.total_ticks, report.tick_preview
    );

    CommandResponse::from_payload(args.json, human_body, &report)
}
