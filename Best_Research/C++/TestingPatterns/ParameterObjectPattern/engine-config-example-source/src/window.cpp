#include "engine/window.hpp"

#include <iostream>

#include "engine/types.hpp"

namespace demo
{
    Window::Window(config::window::Startup config)
        : title_(std::move(config.title)),
          resolution_(config.resolution),
          vSync_(config.vSync),
          frameTarget_(config.frameTarget)
    {
        std::cout
            << "[window] open title=\"" << title_ << "\""
            << " resolution=" << ToString(resolution_)
            << " vSync=" << ToString(vSync_)
            << " targetFps=" << frameTarget_.FramesPerSecond()
            << '\n';
    }

    Window Window::Open(config::window::Startup config)
    {
        return Window(std::move(config));
    }

    void Window::UpdateConfig(config::window::Runtime config)
    {
        resolution_ = config.resolution;
        vSync_ = config.vSync;
        frameTarget_ = config.frameTarget;

        std::cout
            << "[window] update resolution=" << ToString(resolution_)
            << " vSync=" << ToString(vSync_)
            << " targetFps=" << frameTarget_.FramesPerSecond()
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
