#include "RenderConfig.h"

namespace Config {

RenderEngine makeDefaultRenderConfig() {
    return RenderEngine{
        .renderBackend = RenderBackend::DirectX12,
        .shadowQuality = ShadowQuality::Ultra,
        .antiAliasing = AntiAliasing::MSAAx8(),
    };
}

} // namespace Config

const char* toString(Config::RenderBackend value) {
    switch (value) {
        case Config::RenderBackend::DirectX12: return "DirectX12";
        case Config::RenderBackend::Vulkan: return "Vulkan";
        case Config::RenderBackend::OpenGL: return "OpenGL";
    }
    return "Unknown";
}

const char* toString(Config::ShadowQuality value) {
    switch (value) {
        case Config::ShadowQuality::Low: return "Low";
        case Config::ShadowQuality::Medium: return "Medium";
        case Config::ShadowQuality::High: return "High";
        case Config::ShadowQuality::Ultra: return "Ultra";
    }
    return "Unknown";
}

const char* toString(Config::AntiAliasing::Mode value) {
    switch (value) {
        case Config::AntiAliasing::Mode::None: return "None";
        case Config::AntiAliasing::Mode::FXAA: return "FXAA";
        case Config::AntiAliasing::Mode::TAA: return "TAA";
        case Config::AntiAliasing::Mode::MSAA: return "MSAA";
    }
    return "Unknown";
}
