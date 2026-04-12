#pragma once

#include <string>

namespace demo
{
    class FileSystem final
    {
    public:
        FileSystem() = default;

        [[nodiscard]] bool Exists(const std::string& path) const;
    };
}
