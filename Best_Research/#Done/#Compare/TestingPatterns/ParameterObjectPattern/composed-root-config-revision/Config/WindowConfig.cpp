#include "WindowConfig.h"

namespace Config {

Window makeDefaultWindowConfig() {
    return Window{
        .title = "Parameter Object Pattern Demo",
        .windowMode = WindowMode::Borderless,
        .cursorMode = CursorMode::Visible,
        .resizable = Resizable::Disabled,
    };
}

} // namespace Config

const char* toString(Config::WindowMode value) {
    switch (value) {
        case Config::WindowMode::Windowed: return "Windowed";
        case Config::WindowMode::Borderless: return "Borderless";
        case Config::WindowMode::Fullscreen: return "Fullscreen";
    }
    return "Unknown";
}

const char* toString(Config::CursorMode value) {
    switch (value) {
        case Config::CursorMode::Hidden: return "Hidden";
        case Config::CursorMode::Visible: return "Visible";
        case Config::CursorMode::Captured: return "Captured";
    }
    return "Unknown";
}

const char* toString(Config::Resizable value) {
    switch (value) {
        case Config::Resizable::Disabled: return "Disabled";
        case Config::Resizable::Enabled: return "Enabled";
    }
    return "Unknown";
}
