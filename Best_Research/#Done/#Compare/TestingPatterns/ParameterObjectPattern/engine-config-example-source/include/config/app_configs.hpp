#pragma once

#include "config/audio_config.hpp"
#include "config/physics_config.hpp"
#include "config/render_config.hpp"
#include "config/window_config.hpp"

namespace demo::config::app
{
    render::Startup CreateRenderStartup();
    render::Runtime CreateRenderRuntimeUpdate();
    audio::Startup CreateAudioStartup();
    audio::Playback CreateMusicPlayback();
    physics::Startup CreatePhysicsStartup();
    window::Startup CreateWindowStartup();
    window::Runtime CreateWindowRuntimeUpdate();
}
