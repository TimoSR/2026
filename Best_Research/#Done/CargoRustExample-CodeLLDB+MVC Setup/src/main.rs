use tracing::info;
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

unsafe extern "C" {
    fn asm_add(a: i64, b: i64) -> i64;
}

fn main() {
    dotenvy::from_filename_override(".env").expect("load .env");

    let _flame_guard = init_tracing();

    run();
}

#[tracing::instrument]
fn run() {
    let a: i64 = 7;
    let b: i64 = 35;

    info!(a, b, "calling assembly addition");
    let result = assembly_add(a, b);
    info!(a, b, result, "assembly addition completed");

    println!("{a} + {b} = {result}");
}

#[tracing::instrument]
fn assembly_add(a: i64, b: i64) -> i64 {
    unsafe { asm_add(a, b) }
}

fn init_tracing() -> impl Drop {
    let console_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));
    let (flame_layer, guard) = tracing_flame::FlameLayer::with_file("target/tracing.folded")
        .expect("create flamegraph trace");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .compact()
                .with_filter(console_filter),
        )
        .with(flame_layer)
        .init();

    guard
}
