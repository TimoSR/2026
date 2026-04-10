#include "engine/window.hpp"

#include <iostream>
#include <stdexcept>

#include "engine/types.hpp"

namespace demo
{
    Window::Window(config::window::Startup config)
        : title_(std::move(config.title)),
          resolution_(config.resolution),
          vSync_(config.vSync),
          targetFramesPerSecond_(config.targetFramesPerSecond)
    {
        if (targetFramesPerSecond_ <= 0)
        {
            throw std::invalid_argument("Window targetFramesPerSecond must be > 0.");
        }

        std::cout
            << "[window] open title=\"" << title_ << "\""
            << " resolution=" << ToString(resolution_)
            << " vSync=" << ToString(vSync_)
            << " targetFps=" << targetFramesPerSecond_
            << '\n';
    }

    Window Window::Open(config::window::Startup config)
    {
        return Window(std::move(config));
    }

    void Window::UpdateConfig(config::window::Runtime config)
    {
        if (config.targetFramesPerSecond <= 0)
        {
            throw std::invalid_argument("Window targetFramesPerSecond must be > 0.");
        }

        resolution_ = config.resolution;
        vSync_ = config.vSync;
        targetFramesPerSecond_ = config.targetFramesPerSecond;

        std::cout
            << "[window] update resolution=" << ToString(resolution_)
            << " vSync=" << ToString(vSync_)
            << " targetFps=" << targetFramesPerSecond_
            << '\n';
    }

    void Window::Close()
    {
        if (!open_)
        {
            return;
        }

        open_ = false;
        std::cout << "[window] close\n";
    }

    const Resolution& Window::GetResolution() const noexcept
    {
        return resolution_;
    }

    const std::string& Window::GetTitle() const noexcept
    {
        return title_;
    }
}
