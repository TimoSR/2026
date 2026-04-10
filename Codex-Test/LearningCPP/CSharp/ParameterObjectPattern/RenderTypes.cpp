#include "RenderTypes.h"

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

const char* toString(AntiAliasing::Mode value) {
    switch (value) {
        case AntiAliasing::Mode::None: return "None";
        case AntiAliasing::Mode::FXAA: return "FXAA";
        case AntiAliasing::Mode::TAA: return "TAA";
        case AntiAliasing::Mode::MSAA: return "MSAA";
    }
    return "Unknown";
}
