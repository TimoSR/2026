#include "Graphics.h"
#include "Audio.h"
#include "Window.h"

int main() {
    Config::Graphics graphicsConfig{
        .renderBackend = RenderBackend::DirectX12,
        .resolution = Resolution::UHD4K(),
        .shadowQuality = ShadowQuality::High,
        .antiAliasing = AntiAliasing::MSAAx4(),
        .frameRate = Framerate::from_fps(144),
        .vSync = VSync::Enabled,
        .diagnosticsLevel = DiagnosticsLevel::Full
    };

    Config::Audio audioConfig{
        .audioBackend = AudioBackend::WASAPI,
        .masterVolume = Volume::from_percent(85),
        .musicVolume = Volume::from_percent(70),
        .sfxVolume = Volume::from_percent(90),
        .spatialAudio = SpatialAudio::Enabled,
        .outputSampleRate = SampleRate::from_hz(48000),
        .diagnosticsLevel = DiagnosticsLevel::Basic
    };

    Config::Window windowConfig{
        .title = "Parameter Object Pattern Demo",
        .windowMode = WindowMode::Borderless,
        .resolution = Resolution::UHD4K(),
        .cursorMode = CursorMode::Visible,
        .resizable = Resizable::Disabled
    };

    Graphics graphics = Graphics::Create(graphicsConfig);
    Audio audio = Audio::Create(audioConfig);
    Window window = Window::Create(windowConfig);

    graphics.LoadImage("whale", "whale.png");
    audio.PlaySound("music.ogg");

    graphics.DrawText("Hello World!", 400, 300);
    graphics.DrawImage("whale", 300, 200);

    Config::Audio runtimeAudioConfig{
        .audioBackend = AudioBackend::XAudio2,
        .masterVolume = Volume::from_percent(75),
        .musicVolume = Volume::from_percent(60),
        .sfxVolume = Volume::from_percent(80),
        .spatialAudio = SpatialAudio::Disabled,
        .outputSampleRate = SampleRate::from_hz(44100),
        .diagnosticsLevel = DiagnosticsLevel::Full
    };

    Config::Window runtimeWindowConfig{
        .title = "Runtime Window Profile",
        .windowMode = WindowMode::Windowed,
        .resolution = Resolution::FullHD(),
        .cursorMode = CursorMode::Captured,
        .resizable = Resizable::Enabled
    };

    audio.UpdateConfig(runtimeAudioConfig);
    window.UpdateConfig(runtimeWindowConfig);

    graphics.DrawText("After runtime config update", 180, 120);
    audio.Stop();
    window.Close();
    graphics.Stop();
    return 0;
}
