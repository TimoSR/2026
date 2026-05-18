use engine_core::{EngineError, Result};

pub(crate) fn parse_max_frames() -> Result<u64> {
    match std::env::args().nth(1) {
        Some(raw) => raw.parse::<u64>().map_err(|_| {
            EngineError::new(format!(
                "invalid frame count `{raw}` (expected an unsigned integer)"
            ))
        }),
        None => Ok(6),
    }
}
