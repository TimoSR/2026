#include "Audio.h"
#include "Graphics.h"
#include "Window.h"

int main() {
    Config::App appConfig{
        .display = {
            .resolution = Config::Resolution::UHD4K(),
            .refreshRate = Config::Framerate::from_fps(144),
            .vSync = Config::VSync::Enabled,
        },
        .diagnostics = {
            .level = Config::DiagnosticsLevel::Full,
        },
        .render = {
            .renderBackend = Config::RenderBackend::DirectX12,
            .shadowQuality = Config::ShadowQuality::High,
            .antiAliasing = Config::AntiAliasing::MSAAx4(),
        },
        .audio = {
            .audioBackend = Config::AudioBackend::WASAPI,
            .masterVolume = Config::Volume::from_percent(85),
            .musicVolume = Config::Volume::from_percent(70),
            .sfxVolume = Config::Volume::from_percent(90),
            .spatialAudio = Config::SpatialAudio::Enabled,
            .outputSampleRate = Config::SampleRate::from_hz(48000),
        },
        .window = {
            .title = "Parameter Object Pattern Demo",
            .windowMode = Config::WindowMode::Borderless,
            .cursorMode = Config::CursorMode::Visible,
            .resizable = Config::Resizable::Disabled,
        },
    };

    Config::validateConfig(appConfig);

    Graphics graphics = Graphics::Create(appConfig);
    Audio audio = Audio::Create(appConfig);
    Window window = Window::Create(appConfig);

    graphics.LoadImage("whale", "whale.png");
    audio.PlaySound("music.ogg");

    graphics.DrawText("Hello World!", 400, 300);
    graphics.DrawImage("whale", 300, 200);

    Config::App runtimeConfig = appConfig;
    runtimeConfig.display = {
        .resolution = Config::Resolution::QHD(),
        .refreshRate = Config::Framerate::from_fps(120),
        .vSync = Config::VSync::Disabled,
    };
    runtimeConfig.diagnostics.level = Config::DiagnosticsLevel::Basic;
    runtimeConfig.audio.musicVolume = Config::Volume::from_percent(60);
    runtimeConfig.audio.sfxVolume = Config::Volume::from_percent(80);
    runtimeConfig.window.title = "Runtime Window Profile";
    runtimeConfig.window.windowMode = Config::WindowMode::Windowed;
    runtimeConfig.window.cursorMode = Config::CursorMode::Captured;
    runtimeConfig.window.resizable = Config::Resizable::Enabled;

    Config::validateConfig(runtimeConfig);

    graphics.UpdateConfig(runtimeConfig);
    audio.UpdateConfig(runtimeConfig);
    window.UpdateConfig(runtimeConfig);

    graphics.DrawText("After runtime config update", 180, 120);
    audio.Stop();
    window.Close();
    graphics.Stop();

    Graphics defaultGraphics = Graphics::Create();
    Audio defaultAudio = Audio::Create();
    Window defaultWindow = Window::Create();

    defaultGraphics.DrawText("Default preset path", 64, 64);
    defaultAudio.PlaySound("default.ogg");

    defaultAudio.Stop();
    defaultWindow.Close();
    defaultGraphics.Stop();

    return 0;
}
