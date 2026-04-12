#include "engine/render_engine.hpp"

#include <iostream>
#include <stdexcept>

#include "engine/types.hpp"
#include "engine/window.hpp"

namespace demo
{
    RenderEngine::Frame::Frame(RenderEngine& renderEngine, Window& window)
        : renderEngine_(&renderEngine)
    {
        renderEngine_->BeginFrameInternal(window);
    }

    RenderEngine::Frame::~Frame() noexcept
    {
        if (renderEngine_ != nullptr)
        {
            try
            {
                renderEngine_->EndFrameInternal();
            }
            catch (...)
            {
            }
        }
    }

    RenderEngine::Frame::Frame(Frame&& other) noexcept
        : renderEngine_(other.renderEngine_)
    {
        other.renderEngine_ = nullptr;
    }

    void RenderEngine::Frame::DrawText(std::string_view text, int x, int y)
    {
        if (renderEngine_ == nullptr)
        {
            throw std::logic_error("RenderEngine::Frame is not active.");
        }

        renderEngine_->DrawText(text, x, y);
    }

    void RenderEngine::Frame::DrawImage(const ImageId& imageId, int x, int y)
    {
        if (renderEngine_ == nullptr)
        {
            throw std::logic_error("RenderEngine::Frame is not active.");
        }

        renderEngine_->DrawImage(imageId, x, y);
    }

    RenderEngine::RenderEngine(config::render::Startup config)
        : backend_(config.backend),
          shadowQuality_(config.shadowQuality),
          antiAliasing_(std::move(config.antiAliasing)),
          diagnosticsLevel_(config.diagnosticsLevel)
    {
        std::cout
            << "[render] create backend=" << ToString(backend_)
            << " shadowQuality=" << ToString(shadowQuality_)
            << " antiAliasing=" << ToString(antiAliasing_)
            << " diagnostics=" << ToString(diagnosticsLevel_)
            << '\n';
    }

    RenderEngine RenderEngine::Create(config::render::Startup config)
    {
        return RenderEngine(std::move(config));
    }

    RenderEngine::Frame RenderEngine::BeginFrame(Window& window)
    {
        return Frame(*this, window);
    }

    void RenderEngine::DrawText(std::string_view text, int x, int y)
    {
        RequireActiveFrame();

        std::ostringstream stream;
        stream << "  drawText(\"" << text << "\", x=" << x << ", y=" << y << ")";
        commandBuffer_.push_back(stream.str());
    }

    void RenderEngine::DrawImage(const ImageId& imageId, int x, int y)
    {
        RequireActiveFrame();

        std::ostringstream stream;
        stream << "  drawImage(id=\"" << imageId.value << "\", x=" << x << ", y=" << y << ")";
        commandBuffer_.push_back(stream.str());
    }

    void RenderEngine::UpdateConfig(config::render::Runtime config)
    {
        shadowQuality_ = config.shadowQuality;
        antiAliasing_ = std::move(config.antiAliasing);
        diagnosticsLevel_ = config.diagnosticsLevel;

        std::cout
            << "[render] update shadowQuality=" << ToString(shadowQuality_)
            << " antiAliasing=" << ToString(antiAliasing_)
            << " diagnostics=" << ToString(diagnosticsLevel_)
            << '\n';
    }

    void RenderEngine::Stop()
    {
        if (!running_)
        {
            return;
        }

        if (frameOpen_)
        {
            throw std::logic_error("RenderEngine::Stop called while a frame is still open.");
        }

        running_ = false;
        std::cout << "[render] stop\n";
    }

    void RenderEngine::BeginFrameInternal(Window& window)
    {
        if (!running_)
        {
            throw std::logic_error("RenderEngine::BeginFrame called after Stop.");
        }

        if (frameOpen_)
        {
            throw std::logic_error("RenderEngine::BeginFrame called while another frame is open.");
        }

        frameOpen_ = true;
        commandBuffer_.clear();

        std::ostringstream header;
        header
            << "[frame] begin title=\"" << window.GetTitle() << "\""
            << " resolution=" << ToString(window.GetResolution())
            << " backend=" << ToString(backend_)
            << " shadowQuality=" << ToString(shadowQuality_)
            << " antiAliasing=" << ToString(antiAliasing_)
            << " diagnostics=" << ToString(diagnosticsLevel_);

        commandBuffer_.push_back(header.str());
    }

    void RenderEngine::EndFrameInternal()
    {
        RequireActiveFrame();
        commandBuffer_.push_back("[frame] end");

        for (const std::string& command : commandBuffer_)
        {
            std::cout << command << '\n';
        }

        commandBuffer_.clear();
        frameOpen_ = false;
    }

    void RenderEngine::RequireActiveFrame() const
    {
        if (!frameOpen_)
        {
            throw std::logic_error("RenderEngine draw call issued outside an active frame.");
        }
    }
}
