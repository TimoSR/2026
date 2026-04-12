#include "engine/file_system.hpp"
#include "engine/os.hpp"

#include <iostream>

namespace demo
{
    bool FileSystem::Exists(const std::string& path) const
    {
        return !path.empty();
    }

    auto OS::FileSystem() -> demo::FileSystem
    {
        std::cout << "[os] acquire file system\n";
        return demo::FileSystem{};
    }
}
