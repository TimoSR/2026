#pragma once

#include <memory>
#include <string>
#include <string_view>
#include <vector>

#include "config/render_config.hpp"
#include "engine/types.hpp"

namespace demo
{
    class Window;

    class RenderEngine final
    {
    public:
        class Frame final
        {
        public:
            Frame(RenderEngine& renderEngine, Window& window);
            ~Frame() noexcept;

            Frame(const Frame&) = delete;
            Frame& operator=(const Frame&) = delete;
            Frame(Frame&& other) noexcept;
            Frame& operator=(Frame&& other) = delete;

            void DrawText(std::string_view text, int x, int y);
            void DrawImage(const ImageId& imageId, int x, int y);

        private:
            RenderEngine* renderEngine_ = nullptr;
        };

        [[nodiscard]] static RenderEngine Create(config::render::Startup config);

        RenderEngine(const RenderEngine&) = delete;
        RenderEngine& operator=(const RenderEngine&) = delete;
        RenderEngine(RenderEngine&&) noexcept = default;
        RenderEngine& operator=(RenderEngine&&) noexcept = default;

        [[nodiscard]] Frame BeginFrame(Window& window);
        void DrawText(std::string_view text, int x, int y);
        void DrawImage(const ImageId& imageId, int x, int y);
        void UpdateConfig(config::render::Runtime config);
        void Stop();

    private:
        explicit RenderEngine(config::render::Startup config);

        void BeginFrameInternal(Window& window);
        void EndFrameInternal();
        void RequireActiveFrame() const;

        friend class Frame;

        RenderBackend backend_ = RenderBackend::DirectX12;
        ShadowQuality shadowQuality_ = ShadowQuality::High;
        AntiAliasing antiAliasing_ = AntiAliasing::TAA();
        DiagnosticsLevel diagnosticsLevel_ = DiagnosticsLevel::ErrorsOnly;
        bool running_ = true;
        bool frameOpen_ = false;
        std::vector<std::string> commandBuffer_;
    };
}
