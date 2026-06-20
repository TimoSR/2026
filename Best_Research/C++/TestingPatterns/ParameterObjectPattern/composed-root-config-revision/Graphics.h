#pragma once

#include <string>
#include <unordered_map>

#include "App.h"

class Graphics {
public:
    static Graphics Create();
    static Graphics Create(const Config::App& config);
    static Graphics Create(Config::RenderSettingsView settings);

    void UpdateConfig(const Config::App& config);
    void UpdateConfig(Config::RenderSettingsView settings);

    void LoadImage(const char* name, const char* filePath);
    void DrawText(const char* text, int x, int y) const;
    void DrawImage(const char* name, int x, int y) const;
    void Stop();

    void printSummary() const;

private:
    struct InternalRenderConfig {
        Config::RenderBackend renderBackend = Config::RenderBackend::DirectX12;
        Config::Resolution resolution = Config::Resolution::FullHD();
        Config::Framerate refreshRate = Config::Framerate::from_fps(60);
        Config::VSync vSync = Config::VSync::Enabled;
        Config::ShadowQuality shadowQuality = Config::ShadowQuality::High;
        Config::AntiAliasing antiAliasing = Config::AntiAliasing::MSAAx4();
        Config::DiagnosticsLevel diagnosticsLevel = Config::DiagnosticsLevel::Basic;
    };

    explicit Graphics(InternalRenderConfig renderConfig);

    static InternalRenderConfig translate(Config::RenderSettingsView settings);
    static void validate(Config::RenderSettingsView settings);
    void ensureRunning() const;

    InternalRenderConfig renderConfig_;
    bool running_ = false;
    std::unordered_map<std::string, std::string> images_;
};
