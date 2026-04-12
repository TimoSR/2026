#pragma once

#include "engine/file_system.hpp"

namespace demo
{
    class OS final
    {
    public:
        [[nodiscard]] static auto FileSystem() -> demo::FileSystem;
    };
}
