use serde::Serialize;

use crate::cli::ValidateAssetsArgs;
use crate::commands::CommandResponse;
use crate::error::AppError;

const ALLOWED_EXTENSIONS: [&str; 3] = [".png", ".ogg", ".json"];

#[derive(Debug, Serialize)]
struct ValidateAssetsReport {
    command: &'static str,
    manifest: String,
    assets_count: usize,
    allowed_extensions: &'static [&'static str],
}

pub fn execute(args: ValidateAssetsArgs) -> Result<CommandResponse, AppError> {
    let text = std::fs::read_to_string(&args.manifest).map_err(|err| {
        AppError::io(format!(
            "failed to read manifest `{}`: {err}",
            args.manifest.display()
        ))
    })?;

    let mut assets = Vec::new();
    let mut invalid = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if ALLOWED_EXTENSIONS.iter().any(|ext| trimmed.ends_with(ext)) {
            assets.push(trimmed.to_owned());
        } else {
            invalid.push(trimmed.to_owned());
        }
    }

    if assets.is_empty() {
        return Err(AppError::validation(format!(
            "no assets found in manifest `{}`",
            args.manifest.display()
        )));
    }

    if !invalid.is_empty() {
        return Err(AppError::validation(format!(
            "invalid manifest entries in `{}`: {}",
            args.manifest.display(),
            invalid.join(", ")
        )));
    }

    let report = ValidateAssetsReport {
        command: "validate-assets",
        manifest: args.manifest.display().to_string(),
        assets_count: assets.len(),
        allowed_extensions: &ALLOWED_EXTENSIONS,
    };

    let human_body = format!(
        "[validate-assets] manifest={} assets_count={}",
        report.manifest, report.assets_count
    );

    CommandResponse::from_payload(args.json, human_body, &report)
}
