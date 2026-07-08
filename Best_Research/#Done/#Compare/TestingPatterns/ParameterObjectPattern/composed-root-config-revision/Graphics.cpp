#include "Graphics.h"

#include <iostream>
#include <stdexcept>
#include <utility>

Graphics Graphics::Create() {
    const Config::Display display = Config::makeDefaultDisplayConfig();
    const Config::Diagnostics diagnostics = Config::makeDefaultDiagnosticsConfig();
    const Config::RenderEngine render = Config::makeDefaultRenderConfig();
    return Create(Config::RenderSettingsView{
        .display = display,
        .diagnostics = diagnostics,
        .render = render,
    });
}

Graphics Graphics::Create(const Config::App& config) {
    Config::validateConfig(config);
    return Create(Config::getRenderSettings(config));
}

Graphics Graphics::Create(Config::RenderSettingsView settings) {
    Graphics engine(translate(settings));
    engine.running_ = true;
    std::cout << "Graphics created with composed render settings.\n";
    engine.printSummary();
    return engine;
}

void Graphics::UpdateConfig(const Config::App& config) {
    Config::validateConfig(config);
    UpdateConfig(Config::getRenderSettings(config));
}

void Graphics::UpdateConfig(Config::RenderSettingsView settings) {
    ensureRunning();
    renderConfig_ = translate(settings);
    std::cout << "Render configuration updated from shared app config.\n";
    printSummary();
}

void Graphics::LoadImage(const char* name, const char* filePath) {
    ensureRunning();
    if (name == nullptr || *name == '\0' || filePath == nullptr || *filePath == '\0') {
        throw std::invalid_argument("image name and filePath must be non-empty");
    }

    images_[name] = filePath;
    std::cout << "Loaded image '" << name << "' from " << filePath << '\n';
}

void Graphics::DrawText(const char* text, int x, int y) const {
    ensureRunning();
    if (text == nullptr) {
        throw std::invalid_argument("text cannot be null");
    }

    std::cout << "Draw text: '" << text << "' at (" << x << ", " << y << ")\n";
}

void Graphics::DrawImage(const char* name, int x, int y) const {
    ensureRunning();
    if (name == nullptr || *name == '\0') {
        throw std::invalid_argument("image name must be non-empty");
    }

    const auto it = images_.find(name);
    if (it == images_.end()) {
        throw std::runtime_error("image not loaded: " + std::string(name));
    }

    std::cout << "Draw image: '" << name << "' from " << it->second
              << " at (" << x << ", " << y << ")\n";
}

void Graphics::Stop() {
    ensureRunning();
    running_ = false;
    std::cout << "Graphics stopped.\n";
}

void Graphics::printSummary() const {
    std::cout << "Graphics\n";
    std::cout << "  backend: " << toString(renderConfig_.renderBackend) << '\n';
    std::cout << "  resolution: " << renderConfig_.resolution.width << "x" << renderConfig_.resolution.height << '\n';
    std::cout << "  refreshRate: " << renderConfig_.refreshRate.fps << " Hz\n";
    std::cout << "  vsync: " << toString(renderConfig_.vSync) << '\n';
    std::cout << "  shadows: " << toString(renderConfig_.shadowQuality) << '\n';
    std::cout << "  antiAliasing: " << toString(renderConfig_.antiAliasing.mode);
    if (renderConfig_.antiAliasing.mode == Config::AntiAliasing::Mode::MSAA) {
        std::cout << " x" << renderConfig_.antiAliasing.samples;
    }
    std::cout << '\n';
    std::cout << "  diagnostics: " << toString(renderConfig_.diagnosticsLevel) << '\n';
}

Graphics::Graphics(InternalRenderConfig renderConfig)
    : renderConfig_(std::move(renderConfig)) {}

Graphics::InternalRenderConfig Graphics::translate(Config::RenderSettingsView settings) {
    validate(settings);
    return InternalRenderConfig{
        .renderBackend = settings.render.renderBackend,
        .resolution = settings.display.resolution,
        .refreshRate = settings.display.refreshRate,
        .vSync = settings.display.vSync,
        .shadowQuality = settings.render.shadowQuality,
        .antiAliasing = settings.render.antiAliasing,
        .diagnosticsLevel = settings.diagnostics.level,
    };
}

void Graphics::validate(Config::RenderSettingsView settings) {
    if (settings.display.resolution.width == 0U || settings.display.resolution.height == 0U) {
        throw std::invalid_argument("resolution values must be > 0");
    }

    if (settings.display.refreshRate.fps == 0U) {
        throw std::invalid_argument("refreshRate must be > 0");
    }

    if (settings.render.antiAliasing.mode != Config::AntiAliasing::Mode::MSAA &&
        settings.render.antiAliasing.samples != 1U) {
        throw std::invalid_argument("antiAliasing samples must be 1 unless mode is MSAA");
    }

    if (settings.render.antiAliasing.mode == Config::AntiAliasing::Mode::MSAA) {
        const std::uint32_t samples = settings.render.antiAliasing.samples;
        if (!(samples == 2U || samples == 4U || samples == 8U)) {
            throw std::invalid_argument("MSAA sample count must be 2, 4, or 8");
        }
    }
}

void Graphics::ensureRunning() const {
    if (!running_) {
        throw std::runtime_error("engine is not running");
    }
}
