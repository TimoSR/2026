mod config;
mod systems;
pub mod types;
mod making_runs_for_main;

use crate::config::*;
use crate::systems::{Audio, Graphics, Window};

fn main() -> Result<(), String> {

    //Customized

    let app_config = AppConfig
    {
        display: DisplayConfig {
            resolution: Resolution::uhd4k(),
            frame_rate: Framerate::from_fps(144),
            v_sync: VSync::Enabled,
        },
        diagnostics: DiagnosticsConfig {
            diagnostics_level: DiagnosticsLevel::Full,
        },
        graphics: GraphicsConfig {
            render_backend: RenderBackend::DirectX12,
            shadow_quality: ShadowQuality::High,
            anti_aliasing: AntiAliasing::Msaa4,
        },
        audio: AudioConfig {
            audio_backend: AudioBackend::Wasapi,
            master_volume: Volume::from_percent(85),
            music_volume: Volume::from_percent(70),
            sfx_volume: Volume::from_percent(90),
            spatial_audio: SpatialAudio::Enabled,
            output_sample_rate: SampleRate::from_hz(48_000),
        },
        window: WindowConfig {
            title: "Parameter Object Pattern Demo",
            window_mode: WindowMode::Borderless,
            cursor_mode: CursorMode::Visible,
            resizable: Resizable::Disabled,
        },
    };

    validate_config(app_config)?;

    let mut custom_graphics = Graphics::new(app_config);
    let mut custom_audio = Audio::new(app_config);
    let mut custom_window = Window::new(app_config);

    custom_graphics.load_image("whale", "whale.png");
    custom_audio.play_sound("music.ogg");

    custom_graphics.draw_text("Hello World!", 400, 300);
    custom_graphics.draw_image("whale", 300, 200);

    let mut runtime_config = app_config.clone();

    runtime_config.display = DisplayConfig
    {
        resolution: Resolution::qhd(),
        frame_rate: Framerate::from_fps(120),
        v_sync: VSync::Disabled,
    };

    runtime_config.diagnostics.diagnostics_level = DiagnosticsLevel::Basic;
    runtime_config.audio.music_volume = Volume::from_percent(60);
    runtime_config.audio.sfx_volume = Volume::from_percent(80);
    runtime_config.window.title = "Runtime Window Profile";
    runtime_config.window.window_mode = WindowMode::Windowed;
    runtime_config.window.cursor_mode = CursorMode::Captured;
    runtime_config.window.resizable = Resizable::Enabled;

    validate_config(runtime_config)?;

    custom_graphics.update_config(runtime_config);
    custom_audio.update_config(runtime_config);
    custom_window.update_config(runtime_config);

    custom_graphics.draw_text("After runtime config update", 180, 120);
    custom_audio.stop();
    custom_window.close();
    custom_graphics.stop();

    // Default Setup

    let mut graphics = Graphics::default();
    let mut audio = Audio::default();
    let mut window = Window::default();

    graphics.draw_text("Default preset path", 64, 64);
    audio.play_sound("default.ogg");

    audio.stop();
    window.close();
    graphics.stop();

    return Ok(());
}
