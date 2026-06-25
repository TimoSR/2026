use std::env;

use tracing::info;
use tracing_subscriber::{Layer, filter::EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

unsafe extern "C" {
    fn asm_add(a: i64, b: i64) -> i64;
}

fn main()
{
    dotenvy::from_filename_override(".env").expect("load .env");

    let _flame_guard = init_tracing();

    run();
}

#[tracing::instrument]
fn run()
{
    let a: i64 = 7;
    let b: i64 = 35;

    info!(a, b, "calling assembly addition");
    let result = assembly_add(a, b);
    info!(a, b, result, "assembly addition completed");

    println!("{a} + {b} = {result}");
}

#[tracing::instrument]
fn assembly_add(a: i64, b: i64) -> i64
{
    return unsafe { asm_add(a, b) };
}

const DEFAULT_LOG_FILTER: &str = "warn";
const FLAMEGRAPH_OUTPUT_PATH: &str = "target/tracing.folded";

pub fn init_tracing() -> impl Drop
{
    let rust_log = read_rust_log();
    let console_filter = EnvFilter::new(rust_log);

    let console_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .compact()
        .with_filter(console_filter);

    let (flame_layer, flame_guard) = tracing_flame::FlameLayer::with_file(FLAMEGRAPH_OUTPUT_PATH)
        .expect("failed to create flamegraph tracing output file");

    tracing_subscriber::registry()
        .with(console_layer)
        .with(flame_layer)
        .init();

    return flame_guard;
}

fn read_rust_log() -> String
{
    let rust_log_result = env::var("RUST_LOG");

    if rust_log_result.is_err()
    {
        return DEFAULT_LOG_FILTER.to_owned();
    }

    let rust_log = rust_log_result.unwrap();

    if rust_log.trim().is_empty()
    {
        return DEFAULT_LOG_FILTER.to_owned();
    }

    return rust_log;
}
