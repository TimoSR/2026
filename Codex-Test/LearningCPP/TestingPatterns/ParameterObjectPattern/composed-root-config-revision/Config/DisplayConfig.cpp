#include "DisplayConfig.h"

#include <stdexcept>

namespace Config {

Framerate Framerate::from_fps(std::uint32_t value) {
    if (value == 0U) {
        throw std::invalid_argument("fps must be > 0");
    }
    return Framerate{value};
}

Display makeDefaultDisplayConfig() {
    return Display{
        .resolution = Resolution::UHD4K(),
        .refreshRate = Framerate::from_fps(144),
        .vSync = VSync::Enabled,
    };
}

} // namespace Config

const char* toString(Config::VSync value) {
    switch (value) {
        case Config::VSync::Disabled: return "Disabled";
        case Config::VSync::Enabled: return "Enabled";
    }
    return "Unknown";
}
