#include "RenderEngine.h"

#include <algorithm>
#include <iostream>
#include <stdexcept>
#include <utility>

RenderEngine RenderEngine::Run() {
    RenderEngine engine(translate(RenderEngineConfiguration{}));
    engine.running_ = true;
    std::cout << "Engine running with default profile.\n";
    engine.printSummary();
    return engine;
}

RenderEngine RenderEngine::Run(RenderEngineConfiguration config) {
    RenderEngine engine(translate(std::move(config)));
    engine.running_ = true;
    std::cout << "Engine running with custom profile.\n";
    engine.printSummary();
    return engine;
}

void RenderEngine::UpdateConfiguration(RenderEngineConfiguration config) {
    ensureRunning();
    config_ = translate(std::move(config));
    std::cout << "Configuration updated at runtime.\n";
    printSummary();
}

void RenderEngine::LoadImage(const char* name, const char* filePath) {
    ensureRunning();
    if (name == nullptr || *name == '\0' || filePath == nullptr || *filePath == '\0') {
        throw std::invalid_argument("image name and filePath must be non-empty");
    }

    images_[name] = filePath;
    std::cout << "Loaded image '" << name << "' from " << filePath << '\n';
}

void RenderEngine::PlaySound(const char* filePath) const {
    ensureRunning();
    if (filePath == nullptr || *filePath == '\0') {
        throw std::invalid_argument("sound filePath must be non-empty");
    }

    std::cout << "Playing sound stream: " << filePath << '\n';
}

void RenderEngine::DrawText(const char* text, int x, int y) const {
    ensureRunning();
    if (text == nullptr) {
        throw std::invalid_argument("text cannot be null");
    }

    std::cout << "Draw text: '" << text << "' at (" << x << ", " << y << ")\n";
}

void RenderEngine::DrawImage(const char* name, int x, int y) const {
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

void RenderEngine::Stop() {
    ensureRunning();
    running_ = false;
    std::cout << "Engine stopped.\n";
}

void RenderEngine::printSummary() const {
    std::cout << "RenderEngine\n";
    std::cout << "  backend: " << toString(config_.renderBackend) << '\n';
    std::cout << "  resolution: " << config_.resolution.width << "x" << config_.resolution.height << '\n';
    std::cout << "  shadows: " << toString(config_.shadowQuality) << '\n';
    std::cout << "  antiAliasing: " << toString(config_.antiAliasing.mode);
    if (config_.antiAliasing.mode == AntiAliasing::Mode::MSAA) {
        std::cout << " x" << config_.antiAliasing.samples;
    }
    std::cout << '\n';
    std::cout << "  target FPS: " << config_.targetFramesPerSecond << '\n';
    std::cout << "  vsync: " << toString(config_.vSync) << '\n';
    std::cout << "  diagnostics: " << toString(config_.diagnosticsLevel) << '\n';
    std::cout << "  effective FPS: " << config_.effectiveFramesPerSecond << '\n';
}

RenderEngine::RenderEngine(InternalConfig config)
    : config_(std::move(config)) {}

RenderEngine::InternalConfig RenderEngine::translate(RenderEngineConfiguration config) {
    validate(config);
    return InternalConfig{
        .renderBackend = config.renderBackend,
        .resolution = config.resolution,
        .shadowQuality = config.shadowQuality,
        .antiAliasing = config.antiAliasing,
        .targetFramesPerSecond = config.targetFramesPerSecond,
        .vSync = config.vSync,
        .diagnosticsLevel = config.diagnosticsLevel,
        .effectiveFramesPerSecond = resolveEffectiveFPS(config),
    };
}

void RenderEngine::validate(const RenderEngineConfiguration& config) {
    if (config.resolution.width <= 0 || config.resolution.height <= 0) {
        throw std::invalid_argument("resolution values must be > 0");
    }
    if (config.targetFramesPerSecond <= 0) {
        throw std::invalid_argument("targetFramesPerSecond must be > 0");
    }
}

int RenderEngine::resolveEffectiveFPS(const RenderEngineConfiguration& config) {
    if (config.vSync == VSync::Disabled) {
        return config.targetFramesPerSecond;
    }

    constexpr int monitorRefreshRate = 60;
    return std::min(config.targetFramesPerSecond, monitorRefreshRate);
}

void RenderEngine::ensureRunning() const {
    if (!running_) {
        throw std::runtime_error("engine is not running");
    }
}
