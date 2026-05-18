pub mod benchmark;
pub mod headless_server;
pub mod run;
pub mod validate_assets;

use serde::Serialize;

use crate::error::AppError;

pub struct CommandResponse {
    pub json: bool,
    pub human_body: String,
    pub json_body: String,
}

impl CommandResponse {
    pub fn from_payload<T>(json: bool, human_body: String, payload: &T) -> Result<Self, AppError>
    where
        T: Serialize,
    {
        let json_body = serde_json::to_string_pretty(payload)
            .map_err(|err| AppError::runtime(format!("failed to serialize json output: {err}")))?;

        Ok(Self {
            json,
            human_body,
            json_body,
        })
    }
}
