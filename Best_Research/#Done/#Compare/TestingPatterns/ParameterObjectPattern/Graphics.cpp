#include "Graphics.h"

#include <algorithm>
#include <iostream>
#include <stdexcept>
#include <utility>

Graphics Graphics::Create() {
    Graphics engine(translate(Config::Graphics{}));
    engine.running_ = true;
    std::cout << "Graphics created with default profile.\n";
    engine.printSummary();
    return engine;
}

Graphics Graphics::Create(Config::Graphics config) {
    Graphics engine(translate(std::move(config)));
    engine.running_ = true;
    std::cout << "Graphics created with custom profile.\n";
    engine.printSummary();
    return engine;
}

void Graphics::UpdateConfig(Config::Graphics config) {
    ensureRunning();
    renderConfig_ = translate(std::move(config));
    std::cout << "Render configuration updated at runtime.\n";
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
    std::cout << "  shadows: " << toString(renderConfig_.shadowQuality) << '\n';
    std::cout << "  antiAliasing: " << toString(renderConfig_.antiAliasing.mode);
    if (renderConfig_.antiAliasing.mode == AntiAliasing::Mode::MSAA) {
        std::cout << " x" << renderConfig_.antiAliasing.samples;
    }
    std::cout << '\n';
    std::cout << "  target FPS: " << renderConfig_.frameRate.fps_ << '\n';
    std::cout << "  vsync: " << toString(renderConfig_.vSync) << '\n';
    std::cout << "  diagnostics: " << toString(renderConfig_.diagnosticsLevel) << '\n';
    std::cout << "  effective FPS: " << renderConfig_.effectiveFramesPerSecond << '\n';
}

Graphics::Graphics(InternalRenderConfig renderConfig)
    : renderConfig_(std::move(renderConfig)) {}

Graphics::InternalRenderConfig Graphics::translate(Config::Graphics config) {
    validate(config);
    return InternalRenderConfig{
        .renderBackend = config.renderBackend,
        .resolution = config.resolution,
        .shadowQuality = config.shadowQuality,
        .antiAliasing = config.antiAliasing,
        .frameRate = config.frameRate,
        .vSync = config.vSync,
        .diagnosticsLevel = config.diagnosticsLevel,
        .effectiveFramesPerSecond = resolveEffectiveFPS(config),
    };
}

void Graphics::validate(const Config::Graphics& config) {
    if (config.resolution.width <= 0 || config.resolution.height <= 0) {
        throw std::invalid_argument("resolution values must be > 0");
    }
    if (config.frameRate.fps_ <= 0) {
        throw std::invalid_argument("frameRate must be > 0");
    }
}

int Graphics::resolveEffectiveFPS(const Config::Graphics& config) {
    if (config.vSync == VSync::Disabled) {
        return config.frameRate.fps_;
    }

    constexpr int monitorRefreshRate = 60;
    return std::min(config.frameRate.fps_, monitorRefreshRate);
}

void Graphics::ensureRunning() const {
    if (!running_) {
        throw std::runtime_error("engine is not running");
    }
}

