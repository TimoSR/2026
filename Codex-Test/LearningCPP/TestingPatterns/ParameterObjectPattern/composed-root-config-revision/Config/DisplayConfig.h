#pragma once

#include <cstdint>

namespace Config {

enum class VSync {
    Disabled,
    Enabled,
};

struct Resolution {
    std::uint32_t width {1920};
    std::uint32_t height {1080};

    static constexpr Resolution UHD4K() noexcept {
        return Resolution{3840, 2160};
    }

    static constexpr Resolution QHD() noexcept {
        return Resolution{2560, 1440};
    }

    static constexpr Resolution FullHD() noexcept {
        return Resolution{1920, 1080};
    }
};

struct Framerate {
    std::uint32_t fps {60};

    static Framerate from_fps(std::uint32_t value);
};

struct Display {
    Resolution resolution {Resolution::FullHD()};
    Framerate refreshRate {Framerate::from_fps(60)};
    VSync vSync {VSync::Enabled};
};

Display makeDefaultDisplayConfig();

} // namespace Config

const char* toString(Config::VSync value);
