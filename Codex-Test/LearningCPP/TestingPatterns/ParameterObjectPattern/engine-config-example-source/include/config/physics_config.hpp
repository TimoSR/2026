#pragma once

#include "engine/types.hpp"

namespace demo::config::physics
{
    struct Startup final
    {
        Vec2 gravity{ .x = 0.0f, .y = -9.81f };
    };
}
