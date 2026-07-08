#include "Window.h"

#include <iostream>
#include <stdexcept>
#include <utility>

Window Window::Create() {
    Window window(translate(Config::Window{}));
    window.open_ = true;
    std::cout << "Window created with default profile.\n";
    window.printSummary();
    return window;
}

Window Window::Create(Config::Window config) {
    Window window(translate(std::move(config)));
    window.open_ = true;
    std::cout << "Window created with custom profile.\n";
    window.printSummary();
    return window;
}

void Window::UpdateConfig(Config::Window config) {
    ensureOpen();
    config_ = translate(std::move(config));
    std::cout << "Window configuration updated at runtime.\n";
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
    std::cout << "  cursor: " << toString(config_.cursorMode) << '\n';
    std::cout << "  resizable: " << toString(config_.resizable) << '\n';
}

Window::Window(InternalWindowConfig config)
    : config_(std::move(config)) {}

Window::InternalWindowConfig Window::translate(Config::Window config) {
    validate(config);
    return InternalWindowConfig{
        .title = std::move(config.title),
        .windowMode = config.windowMode,
        .resolution = config.resolution,
        .cursorMode = config.cursorMode,
        .resizable = config.resizable,
    };
}

void Window::validate(const Config::Window& config) {
    if (config.title.empty()) {
        throw std::invalid_argument("title must be non-empty");
    }
    if (config.resolution.width <= 0 || config.resolution.height <= 0) {
        throw std::invalid_argument("window resolution values must be > 0");
    }
}

void Window::ensureOpen() const {
    if (!open_) {
        throw std::runtime_error("window is not open");
    }
}
