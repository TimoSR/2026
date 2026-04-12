#pragma once

#include <string>

#include "ConfigTypes.h"

class Window {
public:
    static Window Create();
    static Window Create(Config::Window config);

    void UpdateConfig(Config::Window config);
    void Close();

    void printSummary() const;

private:
    struct InternalWindowConfig {
        std::string title = "Parameter Object Pattern Demo";
        WindowMode windowMode = WindowMode::Windowed;
        Resolution resolution = Resolution::FullHD();
        CursorMode cursorMode = CursorMode::Visible;
        Resizable resizable = Resizable::Enabled;
    };

    explicit Window(InternalWindowConfig config);

    static InternalWindowConfig translate(Config::Window config);
    static void validate(const Config::Window& config);
    void ensureOpen() const;

    InternalWindowConfig config_;
    bool open_ = false;
};

