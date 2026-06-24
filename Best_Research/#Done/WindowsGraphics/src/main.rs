#![allow(clippy::needless_return)]

use std::{path::Path, time::Instant};
use diagnostics::performance_metrics;
use models::{cube, gltf};
use platform::window;
use windows::core::Result;

fn main() -> Result<()>
{
    // application constants
    const WINDOW_WIDTH: i32 = 1920;
    const WINDOW_HEIGHT: i32 = 1080;
    const MULTISAMPLE_ANTIALIASING_ENABLED: bool = false;
    const TEMPORAL_ANTIALIASING_ENABLED: bool = true;
    const EXAMPLE_GLTF_PATH: &str = "assets/example_cube.gltf";
    const EXAMPLE_GLTF_FIRST_OBJECT_IDENTIFIER: u64 = 1000;
    const EXAMPLE_GLB_FIRST_OBJECT_IDENTIFIER: u64 = 2000;
    const EXAMPLE_GLTF_ROTATION_RADIANS_PER_SECOND: [f32; 3] = [0.7, -1.0, 0.3];
    const EXAMPLE_GLB_ROTATION_RADIANS_PER_SECOND: [f32; 3] = [-0.5, 0.9, -0.2];
    const FIRST_CUBE_IDENTIFIER: u64 = 1;
    const SECOND_CUBE_IDENTIFIER: u64 = 2;
    const FIRST_CUBE_POSITION: [f32; 3] = [-1.3, 0.0, 5.0];
    const SECOND_CUBE_POSITION: [f32; 3] = [1.3, 0.0, 5.0];
    const FIRST_CUBE_ROTATION_RADIANS_PER_SECOND: [f32; 3] = [0.8, 1.2, 0.0];
    const SECOND_CUBE_ROTATION_RADIANS_PER_SECOND: [f32; 3] = [1.1, -0.7, 0.4];
    // application constants

    window::enable_per_monitor_dpi_awareness()?;
    let mut window = window::create_window(WINDOW_WIDTH, WINDOW_HEIGHT)?;

    let first_cube = cube::SpinningCube::new(
        FIRST_CUBE_IDENTIFIER,
        FIRST_CUBE_POSITION,
        FIRST_CUBE_ROTATION_RADIANS_PER_SECOND,
    );
    let second_cube = cube::SpinningCube::new(
        SECOND_CUBE_IDENTIFIER,
        SECOND_CUBE_POSITION,
        SECOND_CUBE_ROTATION_RADIANS_PER_SECOND,
    );
    let example_gltf_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(EXAMPLE_GLTF_PATH);
    let example_gltf_objects = gltf::load_objects(
        &example_gltf_path,
        EXAMPLE_GLTF_FIRST_OBJECT_IDENTIFIER,
    )?;
    let example_glb_path = Path::new(env!("OUT_DIR")).join("example_cube.glb");
    let example_glb_objects = gltf::load_objects(
        &example_glb_path,
        EXAMPLE_GLB_FIRST_OBJECT_IDENTIFIER,
    )?;

    let mut graphics = graphics::create_direct3d_graphics(
        window.handle(),
        WINDOW_WIDTH as u32,
        WINDOW_HEIGHT as u32,
    )?;

    graphics.set_multisample_antialiasing_enabled(MULTISAMPLE_ANTIALIASING_ENABLED)?;
    graphics.set_temporal_antialiasing_enabled(TEMPORAL_ANTIALIASING_ENABLED);
    debug_assert_eq!(
        graphics.is_multisample_antialiasing_enabled(),
        MULTISAMPLE_ANTIALIASING_ENABLED,
    );
    debug_assert_eq!(
        graphics.is_temporal_antialiasing_enabled(),
        TEMPORAL_ANTIALIASING_ENABLED,
    );

    graphics.add_object(first_cube)?;
    graphics.add_object(second_cube)?;

    for mut example_gltf_object in example_gltf_objects
    {
        example_gltf_object.set_rotation_radians_per_second(
            EXAMPLE_GLTF_ROTATION_RADIANS_PER_SECOND,
        );
        graphics.add_object(example_gltf_object)?;
    }

    for mut example_glb_object in example_glb_objects
    {
        example_glb_object.set_rotation_radians_per_second(
            EXAMPLE_GLB_ROTATION_RADIANS_PER_SECOND,
        );
        graphics.add_object(example_glb_object)?;
    }

    let started_at = Instant::now();
    let mut performance_metrics = performance_metrics::PerformanceMetrics::create()?;
    let mut are_metrics_visible = false;

    loop
    {
        let window_events = window.process_pending_messages();

        if window_events.should_close
        {
            return Ok(());
        }

        if window_events.was_tab_released
        {
            are_metrics_visible = !are_metrics_visible;
            graphics.set_metrics_visible(are_metrics_visible);
        }

        if window.is_minimized()
        {
            window.wait_for_message()?;
            continue;
        }

        let elapsed_seconds = started_at.elapsed().as_secs_f32();
        graphics.render(elapsed_seconds)?;
        performance_metrics.record_rendered_frame();

        if let Some(performance_sample) = performance_metrics.sample()?
        {
            if are_metrics_visible
            {
                let graphics_memory_metrics = graphics.graphics_memory_metrics();
                let metrics_text = create_metrics_text(
                    &performance_sample,
                    graphics_memory_metrics,
                    &graphics,
                );
                graphics.set_metrics_text(&metrics_text)?;
            }
        }
    }
}

fn create_metrics_text(
    performance_sample: &performance_metrics::PerformanceSample,
    graphics_memory_metrics: Option<graphics::GraphicsMemoryMetrics>,
    graphics: &graphics::Direct3DGraphics,
) -> String
{
    let antialiasing_text = format!(
        "MSAA: {} | TAA: {}",
        if graphics.is_multisample_antialiasing_enabled() { "On" } else { "Off" },
        if graphics.is_temporal_antialiasing_enabled() { "On" } else { "Off" },
    );
    let graphics_memory_text = match graphics_memory_metrics
    {
        Some(graphics_memory_metrics) => format!(
            "GPU memory: {:.0} / {:.0} MiB",
            graphics_memory_metrics.used_bytes as f64 / 1_048_576.0,
            graphics_memory_metrics.budget_bytes as f64 / 1_048_576.0,
        ),
        None => String::from("GPU memory: unavailable"),
    };
    let gpu_usage_text = match graphics.gpu_frame_time_in_milliseconds()
    {
        Some(gpu_frame_time_in_milliseconds) => format!(
            "GPU: {:.2} MS {:.0}% FRAME",
            gpu_frame_time_in_milliseconds,
            gpu_frame_time_in_milliseconds / performance_sample.frame_time_in_milliseconds * 100.0,
        ),
        None => String::from("GPU: collecting timing"),
    };

    return format!(
        "FPS: {:.1}\nFrame time: {:.2} ms\nProcess CPU: {:.1}%\n{}\n{}\n{}\nObjects: {}\nPress Tab to hide metrics",
        performance_sample.frames_per_second,
        performance_sample.frame_time_in_milliseconds,
        performance_sample.process_cpu_usage_percentage,
        gpu_usage_text,
        graphics_memory_text,
        antialiasing_text,
        graphics.loaded_object_count(),
    );
}
