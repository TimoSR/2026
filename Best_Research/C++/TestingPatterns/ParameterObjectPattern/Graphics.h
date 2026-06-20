#pragma once

#include <unordered_map>

#include "ConfigTypes.h"

class Graphics {
public:
    static Graphics Create();
    static Graphics Create(Config::Graphics config);

    void UpdateConfig(Config::Graphics config);

    void LoadImage(const char* name, const char* filePath);
    void DrawText(const char* text, int x, int y) const;
    void DrawImage(const char* name, int x, int y) const;
    void Stop();

    void printSummary() const;

private:
    struct InternalRenderConfig {
        RenderBackend renderBackend = RenderBackend::Vulkan;
        Resolution resolution = Resolution::FullHD();
        ShadowQuality shadowQuality = ShadowQuality::High;
        AntiAliasing antiAliasing = AntiAliasing::TAA();
        Framerate frameRate = Framerate::from_fps(60);
        VSync vSync = VSync::Enabled;
        DiagnosticsLevel diagnosticsLevel = DiagnosticsLevel::Basic;
        int effectiveFramesPerSecond = 60;
    };

    explicit Graphics(InternalRenderConfig renderConfig);

    static InternalRenderConfig translate(Config::Graphics config);
    static void validate(const Config::Graphics& config);
    static int resolveEffectiveFPS(const Config::Graphics& config);
    void ensureRunning() const;

    InternalRenderConfig renderConfig_;
    bool running_ = false;
    std::unordered_map<std::string, std::string> images_;
};


