#pragma once

#include <cstdint>
#include <string>

namespace vulkan_demo {

struct LoaderInfo {
    bool available{};
    std::uint32_t major{};
    std::uint32_t minor{};
    std::uint32_t patch{};
    std::string message;
};

[[nodiscard]] LoaderInfo inspect_loader();

} // namespace vulkan_demo
