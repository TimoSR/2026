#pragma once

#include <string>

#include "config/window_config.hpp"

namespace demo
{
    class Window final
    {
    public:
        [[nodiscard]] static Window Open(config::window::Startup config);

        Window(const Window&) = delete;
        Window& operator=(const Window&) = delete;
        Window(Window&&) noexcept = default;
        Window& operator=(Window&&) noexcept = default;

        void UpdateConfig(config::window::Runtime config);
        void Close();

        [[nodiscard]] const Resolution& GetResolution() const noexcept;
        [[nodiscard]] const std::string& GetTitle() const noexcept;

    private:
        explicit Window(config::window::Startup config);

        std::string title_;
        Resolution resolution_{};
        VSync vSync_ = VSync::Enabled;
        int targetFramesPerSecond_ = 60;
        bool open_ = true;
    };
}
