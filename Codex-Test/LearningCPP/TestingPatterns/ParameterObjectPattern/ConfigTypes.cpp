#include "ConfigTypes.h"

#include <stdexcept>

Framerate Framerate::from_fps(int value) {
    if (value <= 0) {
        throw std::invalid_argument("fps must be > 0");
    }
    return Framerate(value);
}

Volume Volume::from_percent(int value) {
    if (value < 0 || value > 100) {
        throw std::invalid_argument("volume percent must be between 0 and 100");
    }
    return Volume(value);
}

SampleRate SampleRate::from_hz(int value) {
    if (value <= 0) {
        throw std::invalid_argument("sample rate must be > 0");
    }
    return SampleRate(value);
}

const char* toString(RenderBackend value) {
    switch (value) {
        case RenderBackend::DirectX12: return "DirectX12";
        case RenderBackend::Vulkan: return "Vulkan";
        case RenderBackend::Metal: return "Metal";
    }
    return "Unknown";
}

const char* toString(ShadowQuality value) {
    switch (value) {
        case ShadowQuality::Low: return "Low";
        case ShadowQuality::Medium: return "Medium";
        case ShadowQuality::High: return "High";
        case ShadowQuality::Ultra: return "Ultra";
    }
    return "Unknown";
}

const char* toString(DiagnosticsLevel value) {
    switch (value) {
        case DiagnosticsLevel::Off: return "Off";
        case DiagnosticsLevel::Basic: return "Basic";
        case DiagnosticsLevel::Full: return "Full";
    }
    return "Unknown";
}

const char* toString(VSync value) {
    switch (value) {
        case VSync::Disabled: return "Disabled";
        case VSync::Enabled: return "Enabled";
    }
    return "Unknown";
}

const char* toString(AudioBackend value) {
    switch (value) {
        case AudioBackend::XAudio2: return "XAudio2";
        case AudioBackend::OpenAL: return "OpenAL";
        case AudioBackend::WASAPI: return "WASAPI";
    }
    return "Unknown";
}

const char* toString(SpatialAudio value) {
    switch (value) {
        case SpatialAudio::Disabled: return "Disabled";
        case SpatialAudio::Enabled: return "Enabled";
    }
    return "Unknown";
}

const char* toString(WindowMode value) {
    switch (value) {
        case WindowMode::Windowed: return "Windowed";
        case WindowMode::Borderless: return "Borderless";
        case WindowMode::Fullscreen: return "Fullscreen";
    }
    return "Unknown";
}

const char* toString(Resizable value) {
    switch (value) {
        case Resizable::Disabled: return "Disabled";
        case Resizable::Enabled: return "Enabled";
    }
    return "Unknown";
}

const char* toString(CursorMode value) {
    switch (value) {
        case CursorMode::Visible: return "Visible";
        case CursorMode::Hidden: return "Hidden";
        case CursorMode::Captured: return "Captured";
    }
    return "Unknown";
}

const char* toString(AntiAliasing::Mode value) {
    switch (value) {
        case AntiAliasing::Mode::None: return "None";
        case AntiAliasing::Mode::FXAA: return "FXAA";
        case AntiAliasing::Mode::TAA: return "TAA";
        case AntiAliasing::Mode::MSAA: return "MSAA";
    }
    return "Unknown";
}

