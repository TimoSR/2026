#pragma once

#include <string>

#include "engine/types.hpp"

namespace demo::config::window
{
    struct Startup final
    {
        std::string title = "Demo Window";
        Resolution resolution = Resolution::FHD();
        VSync vSync = VSync::Enabled;
        int targetFramesPerSecond = 60;
    };

    struct Runtime final
    {
        Resolution resolution = Resolution::FHD();
        VSync vSync = VSync::Enabled;
        int targetFramesPerSecond = 60;
    };
}
