use serde::Serialize;

use std::time::Instant;

use crate::cli::BenchmarkArgs;
use crate::commands::CommandResponse;
use crate::error::AppError;

#[derive(Debug, Serialize)]
struct BenchmarkReport {
    command: &'static str,
    iterations: u64,
    checksum: u64,
    elapsed_ms: f64,
}

pub fn execute(args: BenchmarkArgs) -> Result<CommandResponse, AppError> {
    let start = Instant::now();
    let mut checksum = 0_u64;

    for i in 0..args.iterations {
        let rotated = i.rotate_left((i % 31) as u32);
        checksum = checksum.wrapping_add(rotated ^ 0x9E37_79B9_u64);
    }

    let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;

    let report = BenchmarkReport {
        command: "benchmark",
        iterations: args.iterations,
        checksum,
        elapsed_ms,
    };

    let human_body = format!(
        "[benchmark] iterations={} checksum={} elapsed_ms={:.3}",
        report.iterations, report.checksum, report.elapsed_ms
    );

    CommandResponse::from_payload(args.json, human_body, &report)
}
