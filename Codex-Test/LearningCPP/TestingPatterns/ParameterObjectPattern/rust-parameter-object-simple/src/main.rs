mod config;
mod systems;

use crate::config::*;

use crate::systems::{Audio, Graphics, Window};

fn main() -> Result<(), String> {

    //Customized

    let app_config = AppConfig 
    {
        display: DisplayConfig {
            resolution: Resolution::uhd4k(),
            refresh_rate: Framerate::from_fps(144),
            v_sync: VSync::Enabled,
        },
        diagnostics: DiagnosticsConfig {
            level: DiagnosticsLevel::Full,
        },
        render: RenderConfig {
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
            title: "Parameter Object Pattern Demo".to_string(),
            window_mode: WindowMode::Borderless,
            cursor_mode: CursorMode::Visible,
            resizable: Resizable::Disabled,
        },
    };

    validate_config(&app_config)?;

    let mut graphics = Graphics::create(&app_config);
    let mut audio = Audio::create(&app_config);
    let mut window = Window::create(&app_config);

    graphics.load_image("whale", "whale.png");
    audio.play_sound("music.ogg");

    graphics.draw_text("Hello World!", 400, 300);
    graphics.draw_image("whale", 300, 200);

    let mut runtime_config = app_config.clone();
    
    runtime_config.display = DisplayConfig 
    {
        resolution: Resolution::qhd(),
        refresh_rate: Framerate::from_fps(120),
        v_sync: VSync::Disabled,
    };

    runtime_config.diagnostics.level = DiagnosticsLevel::Basic;
    runtime_config.audio.music_volume = Volume::from_percent(60);
    runtime_config.audio.sfx_volume = Volume::from_percent(80);
    runtime_config.window.title = "Runtime Window Profile".to_string();
    runtime_config.window.window_mode = WindowMode::Windowed;
    runtime_config.window.cursor_mode = CursorMode::Captured;
    runtime_config.window.resizable = Resizable::Enabled;

    validate_config(&runtime_config)?;

    graphics.update_config(&runtime_config);
    audio.update_config(&runtime_config);
    window.update_config(&runtime_config);

    graphics.draw_text("After runtime config update", 180, 120);
    audio.stop();
    window.close();
    graphics.stop();

    // Default Setup

    let mut default_graphics = Graphics::create_default();
    let mut default_audio = Audio::create_default();
    let mut default_window = Window::create_default();

    default_graphics.draw_text("Default preset path", 64, 64);
    default_audio.play_sound("default.ogg");

    default_audio.stop();
    default_window.close();
    default_graphics.stop();

    Ok(())
}
