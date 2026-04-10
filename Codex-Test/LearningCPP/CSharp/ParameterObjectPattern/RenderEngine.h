#pragma once

#include <string>
#include <unordered_map>

#include "RenderTypes.h"

class RenderEngine {
public:
    static RenderEngine Run();
    static RenderEngine Run(RenderEngineConfiguration config);

    void UpdateConfiguration(RenderEngineConfiguration config);

    void LoadImage(const char* name, const char* filePath);
    void PlaySound(const char* filePath) const;
    void DrawText(const char* text, int x, int y) const;
    void DrawImage(const char* name, int x, int y) const;
    void Stop();

    void printSummary() const;

private:
    struct InternalConfig {
        RenderBackend renderBackend = RenderBackend::Vulkan;
        Resolution resolution = Resolution::FullHD();
        ShadowQuality shadowQuality = ShadowQuality::High;
        AntiAliasing antiAliasing = AntiAliasing::TAA();
        int targetFramesPerSecond = 60;
        VSync vSync = VSync::Enabled;
        DiagnosticsLevel diagnosticsLevel = DiagnosticsLevel::Basic;
        int effectiveFramesPerSecond = 60;
    };

    explicit RenderEngine(InternalConfig config);

    static InternalConfig translate(RenderEngineConfiguration config);
    static void validate(const RenderEngineConfiguration& config);
    static int resolveEffectiveFPS(const RenderEngineConfiguration& config);
    void ensureRunning() const;

    InternalConfig config_;
    bool running_ = false;
    std::unordered_map<std::string, std::string> images_;
};
