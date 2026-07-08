#pragma once

#include <cstdint>

namespace Config {

enum class RenderBackend {
    DirectX12,
    Vulkan,
    OpenGL,
};

enum class ShadowQuality {
    Low,
    Medium,
    High,
    Ultra,
};

struct AntiAliasing {
    enum class Mode {
        None,
        FXAA,
        TAA,
        MSAA,
    };

    Mode mode {Mode::None};
    std::uint32_t samples {1};

    static constexpr AntiAliasing None() noexcept {
        return AntiAliasing{Mode::None, 1};
    }

    static constexpr AntiAliasing FXAA() noexcept {
        return AntiAliasing{Mode::FXAA, 1};
    }

    static constexpr AntiAliasing TAA() noexcept {
        return AntiAliasing{Mode::TAA, 1};
    }

    static constexpr AntiAliasing MSAAx2() noexcept {
        return AntiAliasing{Mode::MSAA, 2};
    }

    static constexpr AntiAliasing MSAAx4() noexcept {
        return AntiAliasing{Mode::MSAA, 4};
    }

    static constexpr AntiAliasing MSAAx8() noexcept {
        return AntiAliasing{Mode::MSAA, 8};
    }
};

struct RenderEngine {
    RenderBackend renderBackend {RenderBackend::DirectX12};
    ShadowQuality shadowQuality {ShadowQuality::High};
    AntiAliasing antiAliasing {AntiAliasing::MSAAx4()};
};

RenderEngine makeDefaultRenderConfig();

} // namespace Config

const char* toString(Config::RenderBackend value);
const char* toString(Config::ShadowQuality value);
const char* toString(Config::AntiAliasing::Mode value);
