import vulkan_demo.context;

#include "vulkan_demo/vulkan_demo.hpp"

#include <format>

namespace vulkan_demo {

LoaderInfo inspect_loader() {
    const auto version = detail::query_loader_version();
    if (!version.available) {
        return {
            false,
            0,
            0,
            0,
            "Vulkan loader (vulkan-1.dll) is not available."
        };
    }

    return {
        true,
        version.major,
        version.minor,
        version.patch,
        std::format("Vulkan loader API: {}.{}.{}", version.major, version.minor, version.patch)
    };
}

} // namespace vulkan_demo
