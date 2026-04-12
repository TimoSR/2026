#pragma once

#include <string>

#include "Config/AppViews.h"

class Window {
public:
    static Window Create();
    static Window Create(const Config::App& config);
    static Window Create(Config::WindowSettingsView settings);

    void UpdateConfig(const Config::App& config);
    void UpdateConfig(Config::WindowSettingsView settings);

    void Close();

    void printSummary() const;

private:
    struct InternalWindowConfig {
        std::string title = "Parameter Object Pattern Demo";
        Config::WindowMode windowMode = Config::WindowMode::Windowed;
        Config::Resolution resolution = Config::Resolution::FullHD();
        Config::Framerate refreshRate = Config::Framerate::from_fps(60);
        Config::VSync vSync = Config::VSync::Enabled;
        Config::CursorMode cursorMode = Config::CursorMode::Visible;
        Config::Resizable resizable = Config::Resizable::Enabled;
    };

    explicit Window(InternalWindowConfig config);

    static InternalWindowConfig translate(Config::WindowSettingsView settings);
    static void validate(Config::WindowSettingsView settings);
    void ensureOpen() const;

    InternalWindowConfig config_;
    bool open_ = false;
};
