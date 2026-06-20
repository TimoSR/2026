#include "Window.h"

#include <iostream>
#include <stdexcept>
#include <utility>

Window Window::Create() {
    const Config::Display display = Config::makeDefaultDisplayConfig();
    const Config::Window windowConfig = Config::makeDefaultWindowConfig();
    return Create(Config::WindowSettingsView{
        .display = display,
        .window = windowConfig,
    });
}

Window Window::Create(const Config::App& config) {
    Config::validateConfig(config);
    return Create(Config::getWindowSettings(config));
}

Window Window::Create(Config::WindowSettingsView settings) {
    Window window(translate(settings));
    window.open_ = true;
    std::cout << "Window created with composed window settings.\n";
    window.printSummary();
    return window;
}

void Window::UpdateConfig(const Config::App& config) {
    Config::validateConfig(config);
    UpdateConfig(Config::getWindowSettings(config));
}

void Window::UpdateConfig(Config::WindowSettingsView settings) {
    ensureOpen();
    config_ = translate(settings);
    std::cout << "Window configuration updated from shared app config.\n";
    printSummary();
}

void Window::Close() {
    ensureOpen();
    open_ = false;
    std::cout << "Window closed.\n";
}

void Window::printSummary() const {
    std::cout << "Window\n";
    std::cout << "  title: " << config_.title << '\n';
    std::cout << "  mode: " << toString(config_.windowMode) << '\n';
    std::cout << "  resolution: " << config_.resolution.width << "x" << config_.resolution.height << '\n';
    std::cout << "  refreshRate: " << config_.refreshRate.fps << " Hz\n";
    std::cout << "  vsync: " << toString(config_.vSync) << '\n';
    std::cout << "  cursor: " << toString(config_.cursorMode) << '\n';
    std::cout << "  resizable: " << toString(config_.resizable) << '\n';
}

Window::Window(InternalWindowConfig config)
    : config_(std::move(config)) {}

Window::InternalWindowConfig Window::translate(Config::WindowSettingsView settings) {
    validate(settings);
    return InternalWindowConfig{
        .title = settings.window.title,
        .windowMode = settings.window.windowMode,
        .resolution = settings.display.resolution,
        .refreshRate = settings.display.refreshRate,
        .vSync = settings.display.vSync,
        .cursorMode = settings.window.cursorMode,
        .resizable = settings.window.resizable,
    };
}

void Window::validate(Config::WindowSettingsView settings) {
    if (settings.window.title.empty()) {
        throw std::invalid_argument("title must be non-empty");
    }

    if (settings.display.resolution.width == 0U || settings.display.resolution.height == 0U) {
        throw std::invalid_argument("window resolution values must be > 0");
    }

    if (settings.display.refreshRate.fps == 0U) {
        throw std::invalid_argument("window refreshRate must be > 0");
    }
}

void Window::ensureOpen() const {
    if (!open_) {
        throw std::runtime_error("window is not open");
    }
}
