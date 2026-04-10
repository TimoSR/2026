#pragma once

enum class RenderBackend {
    DirectX12,
    Vulkan,
    Metal,
};

enum class ShadowQuality {
    Low,
    Medium,
    High,
    Ultra,
};

enum class DiagnosticsLevel {
    Off,
    Basic,
    Full,
};

enum class VSync {
    Disabled,
    Enabled,
};

struct Resolution {
    int width = 1920;
    int height = 1080;

    static constexpr Resolution HD() noexcept { return {1280, 720}; }
    static constexpr Resolution FullHD() noexcept { return {1920, 1080}; }
    static constexpr Resolution UHD4K() noexcept { return {3840, 2160}; }
};

struct AntiAliasing {
    enum class Mode {
        None,
        FXAA,
        TAA,
        MSAA,
    };

    Mode mode = Mode::None;
    int samples = 1;

    static constexpr AntiAliasing None() noexcept { return {Mode::None, 1}; }
    static constexpr AntiAliasing FXAA() noexcept { return {Mode::FXAA, 1}; }
    static constexpr AntiAliasing TAA() noexcept { return {Mode::TAA, 1}; }
    static constexpr AntiAliasing MSAAx4() noexcept { return {Mode::MSAA, 4}; }
};

// Aggregate, so designated initializers stay available.
struct RenderEngineConfiguration {
    RenderBackend renderBackend = RenderBackend::Vulkan;
    Resolution resolution = Resolution::FullHD();
    ShadowQuality shadowQuality = ShadowQuality::High;
    AntiAliasing antiAliasing = AntiAliasing::TAA();
    int targetFramesPerSecond = 60;
    VSync vSync = VSync::Enabled;
    DiagnosticsLevel diagnosticsLevel = DiagnosticsLevel::Basic;
};

const char* toString(RenderBackend value);
const char* toString(ShadowQuality value);
const char* toString(DiagnosticsLevel value);
const char* toString(VSync value);
const char* toString(AntiAliasing::Mode value);
