#pragma once

#include <cstdint>
#include <iomanip>
#include <sstream>
#include <string>

namespace demo
{
    struct Vec2 final
    {
        float x = 0.0f;
        float y = 0.0f;
    };

    inline std::string ToString(const Vec2& value)
    {
        std::ostringstream stream;
        stream << std::fixed << std::setprecision(2) << "(" << value.x << ", " << value.y << ")";
        return stream.str();
    }

    struct Resolution final
    {
        int width = 1280;
        int height = 720;

        static Resolution HD() noexcept
        {
            return Resolution{ .width = 1280, .height = 720 };
        }

        static Resolution FHD() noexcept
        {
            return Resolution{ .width = 1920, .height = 1080 };
        }

        static Resolution UHD4K() noexcept
        {
            return Resolution{ .width = 3840, .height = 2160 };
        }
    };

    inline std::string ToString(const Resolution& value)
    {
        std::ostringstream stream;
        stream << value.width << "x" << value.height;
        return stream.str();
    }

    enum class RenderBackend
    {
        DirectX12,
        Vulkan,
        Metal
    };

    inline std::string ToString(RenderBackend value)
    {
        switch (value)
        {
            case RenderBackend::DirectX12: return "DirectX12";
            case RenderBackend::Vulkan: return "Vulkan";
            case RenderBackend::Metal: return "Metal";
        }

        return "Unknown";
    }

    enum class ShadowQuality
    {
        Low,
        Medium,
        High,
        Ultra
    };

    inline std::string ToString(ShadowQuality value)
    {
        switch (value)
        {
            case ShadowQuality::Low: return "Low";
            case ShadowQuality::Medium: return "Medium";
            case ShadowQuality::High: return "High";
            case ShadowQuality::Ultra: return "Ultra";
        }

        return "Unknown";
    }

    class AntiAliasing final
    {
    public:
        static AntiAliasing None() noexcept
        {
            return AntiAliasing("None");
        }

        static AntiAliasing FXAA() noexcept
        {
            return AntiAliasing("FXAA");
        }

        static AntiAliasing TAA() noexcept
        {
            return AntiAliasing("TAA");
        }

        const std::string& Name() const noexcept
        {
            return name_;
        }

    private:
        explicit AntiAliasing(std::string name)
            : name_(std::move(name))
        {
        }

        std::string name_;
    };

    inline std::string ToString(const AntiAliasing& value)
    {
        return value.Name();
    }

    enum class DiagnosticsLevel
    {
        Off,
        ErrorsOnly,
        Full
    };

    inline std::string ToString(DiagnosticsLevel value)
    {
        switch (value)
        {
            case DiagnosticsLevel::Off: return "Off";
            case DiagnosticsLevel::ErrorsOnly: return "ErrorsOnly";
            case DiagnosticsLevel::Full: return "Full";
        }

        return "Unknown";
    }

    enum class VSync
    {
        Disabled,
        Enabled
    };

    inline std::string ToString(VSync value)
    {
        switch (value)
        {
            case VSync::Disabled: return "Disabled";
            case VSync::Enabled: return "Enabled";
        }

        return "Unknown";
    }

    struct ImageId final
    {
        std::string value;
    };

    struct SoundId final
    {
        std::string value;
    };

    using PlaybackId = std::uint64_t;
}
