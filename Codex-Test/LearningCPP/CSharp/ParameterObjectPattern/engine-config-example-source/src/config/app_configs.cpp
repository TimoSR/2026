#include "config/app_configs.hpp"

namespace demo::config::app
{
    render::Startup CreateRenderStartup()
    {
        return render::Startup{
            .backend = RenderBackend::DirectX12,
            .shadowQuality = ShadowQuality::High,
            .antiAliasing = AntiAliasing::TAA(),
            .diagnosticsLevel = DiagnosticsLevel::ErrorsOnly,
        };
    }

    render::Runtime CreateRenderRuntimeUpdate()
    {
        return render::Runtime{
            .shadowQuality = ShadowQuality::Ultra,
            .antiAliasing = AntiAliasing::FXAA(),
            .diagnosticsLevel = DiagnosticsLevel::Full,
        };
    }

    audio::Startup CreateAudioStartup()
    {
        return audio::Startup{
            .masterVolume = 0.80f,
        };
    }

    audio::Playback CreateMusicPlayback()
    {
        return audio::Playback{
            .loop = true,
            .volume = 0.35f,
        };
    }

    physics::Startup CreatePhysicsStartup()
    {
        return physics::Startup{
            .gravity = Vec2{ .x = 0.0f, .y = -9.81f },
        };
    }

    window::Startup CreateWindowStartup()
    {
        return window::Startup{
            .title = "Whale Demo",
            .resolution = Resolution::FHD(),
            .vSync = VSync::Enabled,
            .targetFramesPerSecond = 144,
        };
    }

    window::Runtime CreateWindowRuntimeUpdate()
    {
        return window::Runtime{
            .resolution = Resolution::UHD4K(),
            .vSync = VSync::Enabled,
            .targetFramesPerSecond = 144,
        };
    }
}
