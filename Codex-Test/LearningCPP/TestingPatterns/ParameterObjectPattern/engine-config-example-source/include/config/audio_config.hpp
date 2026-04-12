#pragma once

namespace demo::config::audio
{
    struct Startup final
    {
        float masterVolume = 1.0f;
    };

    struct Playback final
    {
        bool loop = false;
        float volume = 1.0f;
    };
}
