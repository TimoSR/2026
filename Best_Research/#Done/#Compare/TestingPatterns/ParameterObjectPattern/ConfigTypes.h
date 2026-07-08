#pragma once

#include <string>

class Graphics;
class Audio;
class Window;

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

enum class AudioBackend {
    XAudio2,
    OpenAL,
    WASAPI,
};

enum class SpatialAudio {
    Disabled,
    Enabled,
};

enum class WindowMode {
    Windowed,
    Borderless,
    Fullscreen,
};

enum class Resizable {
    Disabled,
    Enabled,
};

enum class CursorMode {
    Visible,
    Hidden,
    Captured,
};

struct Framerate {
private:
    int fps_ = 60;

    explicit constexpr Framerate(int fps) noexcept
        : fps_(fps) {}

    friend class ::Graphics;

public:
    static Framerate from_fps(int value);
    Framerate() = delete;
};

struct Volume {
private:
    int percent_ = 100;

    explicit constexpr Volume(int percent) noexcept
        : percent_(percent) {}

    friend class ::Audio;

public:
    static Volume from_percent(int value);
    Volume() = delete;
};

struct SampleRate {
private:
    int hz_ = 48000;

    explicit constexpr SampleRate(int hz) noexcept
        : hz_(hz) {}

    friend class ::Audio;

public:
    static SampleRate from_hz(int value);
    SampleRate() = delete;
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

namespace Config {
    struct Graphics {
        RenderBackend renderBackend = RenderBackend::Vulkan;
        Resolution resolution = Resolution::FullHD();
        ShadowQuality shadowQuality = ShadowQuality::High;
        AntiAliasing antiAliasing = AntiAliasing::TAA();
        Framerate frameRate = Framerate::from_fps(60);
        VSync vSync = VSync::Enabled;
        DiagnosticsLevel diagnosticsLevel = DiagnosticsLevel::Basic;
    };

    struct Audio {
        AudioBackend audioBackend = AudioBackend::XAudio2;
        Volume masterVolume = Volume::from_percent(100);
        Volume musicVolume = Volume::from_percent(80);
        Volume sfxVolume = Volume::from_percent(90);
        SpatialAudio spatialAudio = SpatialAudio::Disabled;
        SampleRate outputSampleRate = SampleRate::from_hz(48000);
        DiagnosticsLevel diagnosticsLevel = DiagnosticsLevel::Basic;
    };

    struct Window {
        std::string title = "Parameter Object Pattern Demo";
        WindowMode windowMode = WindowMode::Windowed;
        Resolution resolution = Resolution::FullHD();
        CursorMode cursorMode = CursorMode::Visible;
        Resizable resizable = Resizable::Enabled;
    };
}

// Aggregate, so designated initializers stay available.


const char* toString(RenderBackend value);
const char* toString(ShadowQuality value);
const char* toString(DiagnosticsLevel value);
const char* toString(VSync value);
const char* toString(AudioBackend value);
const char* toString(SpatialAudio value);
const char* toString(WindowMode value);
const char* toString(Resizable value);
const char* toString(CursorMode value);
const char* toString(AntiAliasing::Mode value);

