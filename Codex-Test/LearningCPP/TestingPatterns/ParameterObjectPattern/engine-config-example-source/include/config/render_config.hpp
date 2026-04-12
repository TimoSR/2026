#pragma once

#include "engine/types.hpp"

namespace demo::config::render
{
    struct Startup final
    {
        RenderBackend backend = RenderBackend::DirectX12;
        ShadowQuality shadowQuality = ShadowQuality::High;
        AntiAliasing antiAliasing = AntiAliasing::TAA();
        DiagnosticsLevel diagnosticsLevel = DiagnosticsLevel::ErrorsOnly;
    };

    struct Runtime final
    {
        ShadowQuality shadowQuality = ShadowQuality::High;
        AntiAliasing antiAliasing = AntiAliasing::TAA();
        DiagnosticsLevel diagnosticsLevel = DiagnosticsLevel::ErrorsOnly;
    };
}
