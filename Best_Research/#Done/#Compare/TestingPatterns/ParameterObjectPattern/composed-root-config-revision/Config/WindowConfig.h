#pragma once

#include <string>

namespace Config {

enum class WindowMode {
    Windowed,
    Borderless,
    Fullscreen,
};

enum class CursorMode {
    Hidden,
    Visible,
    Captured,
};

enum class Resizable {
    Disabled,
    Enabled,
};

struct Window {
    std::string title {"Parameter Object Pattern Demo"};
    WindowMode windowMode {WindowMode::Windowed};
    CursorMode cursorMode {CursorMode::Visible};
    Resizable resizable {Resizable::Enabled};
};

Window makeDefaultWindowConfig();

} // namespace Config

const char* toString(Config::WindowMode value);
const char* toString(Config::CursorMode value);
const char* toString(Config::Resizable value);
