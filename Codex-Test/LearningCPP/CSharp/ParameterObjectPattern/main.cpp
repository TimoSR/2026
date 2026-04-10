#include <algorithm>
#include <iostream>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <utility>

enum class RenderBackend {
    DirectX12,
    Vulkan,
    Metal,
};

enum class ShadowQuality {
    Low,
    Medium,
    High,
    Ultra,
};

enum class DiagnosticsLevel {
    Off,
    Basic,
    Full,
};

enum class VSync {
    Disabled,
    Enabled,
};

struct Resolution {
    int width = 1920;
    int height = 1080;

    static constexpr Resolution HD() noexcept { return {1280, 720}; }
    static constexpr Resolution FullHD() noexcept { return {1920, 1080}; }
    static constexpr Resolution UHD4K() noexcept { return {3840, 2160}; }
};

struct AntiAliasing {
    enum class Mode {
        None,
        FXAA,
        TAA,
        MSAA,
    };

    Mode mode = Mode::None;
    int samples = 1;

    static constexpr AntiAliasing None() noexcept { return {Mode::None, 1}; }
    static constexpr AntiAliasing FXAA() noexcept { return {Mode::FXAA, 1}; }
    static constexpr AntiAliasing TAA() noexcept { return {Mode::TAA, 1}; }
    static constexpr AntiAliasing MSAAx4() noexcept { return {Mode::MSAA, 4}; }
};

// Aggregate, so designated initializers stay available.
struct RenderEngineConfiguration {
    RenderBackend renderBackend = RenderBackend::Vulkan;
    Resolution resolution = Resolution::FullHD();
    ShadowQuality shadowQuality = ShadowQuality::High;
    AntiAliasing antiAliasing = AntiAliasing::TAA();
    int targetFramesPerSecond = 60;
    VSync vSync = VSync::Enabled;
    DiagnosticsLevel diagnosticsLevel = DiagnosticsLevel::Basic;
};

static const char* toString(RenderBackend value) {
    switch (value) {
        case RenderBackend::DirectX12: return "DirectX12";
        case RenderBackend::Vulkan: return "Vulkan";
        case RenderBackend::Metal: return "Metal";
    }
    return "Unknown";
}

static const char* toString(ShadowQuality value) {
    switch (value) {
        case ShadowQuality::Low: return "Low";
        case ShadowQuality::Medium: return "Medium";
        case ShadowQuality::High: return "High";
        case ShadowQuality::Ultra: return "Ultra";
    }
    return "Unknown";
}

static const char* toString(DiagnosticsLevel value) {
    switch (value) {
        case DiagnosticsLevel::Off: return "Off";
        case DiagnosticsLevel::Basic: return "Basic";
        case DiagnosticsLevel::Full: return "Full";
    }
    return "Unknown";
}

static const char* toString(VSync value) {
    switch (value) {
        case VSync::Disabled: return "Disabled";
        case VSync::Enabled: return "Enabled";
    }
    return "Unknown";
}

static const char* toString(AntiAliasing::Mode value) {
    switch (value) {
        case AntiAliasing::Mode::None: return "None";
        case AntiAliasing::Mode::FXAA: return "FXAA";
        case AntiAliasing::Mode::TAA: return "TAA";
        case AntiAliasing::Mode::MSAA: return "MSAA";
    }
    return "Unknown";
}

class RenderEngine {
public:
    // Starts the engine with a default profile in one call.
    static RenderEngine Run() {
        RenderEngine engine(translate(RenderEngineConfiguration{}));
        engine.running_ = true;
        std::cout << "Engine running with default profile.\n";
        engine.printSummary();
        return engine;
    }

    // Optional path if you want a custom profile at startup.
    static RenderEngine Run(RenderEngineConfiguration config) {
        RenderEngine engine(translate(std::move(config)));
        engine.running_ = true;
        std::cout << "Engine running with custom profile.\n";
        engine.printSummary();
        return engine;
    }

    // Mutable runtime API: apply a new profile while running.
    void UpdateConfiguration(RenderEngineConfiguration config) {
        ensureRunning();
        config_ = translate(std::move(config));
        std::cout << "Configuration updated at runtime.\n";
        printSummary();
    }

    void LoadImage(const char* name, const char* filePath) {
        ensureRunning();
        if (name == nullptr || *name == '\0' || filePath == nullptr || *filePath == '\0') {
            throw std::invalid_argument("image name and filePath must be non-empty");
        }

        images_[name] = filePath;
        std::cout << "Loaded image '" << name << "' from " << filePath << '\n';
    }

    void PlaySound(const char* filePath) const {
        ensureRunning();
        if (filePath == nullptr || *filePath == '\0') {
            throw std::invalid_argument("sound filePath must be non-empty");
        }

        std::cout << "Playing sound stream: " << filePath << '\n';
    }

    void DrawText(const char* text, int x, int y) const {
        ensureRunning();
        if (text == nullptr) {
            throw std::invalid_argument("text cannot be null");
        }

        std::cout << "Draw text: '" << text << "' at (" << x << ", " << y << ")\n";
    }

    void DrawImage(const char* name, int x, int y) const {
        ensureRunning();
        if (name == nullptr || *name == '\0') {
            throw std::invalid_argument("image name must be non-empty");
        }

        const auto it = images_.find(name);
        if (it == images_.end()) {
            throw std::runtime_error("image not loaded: " + std::string(name));
        }

        std::cout << "Draw image: '" << name << "' from " << it->second
                  << " at (" << x << ", " << y << ")\n";
    }

    void Stop() {
        ensureRunning();
        running_ = false;
        std::cout << "Engine stopped.\n";
    }

    void printSummary() const {
        std::cout << "RenderEngine\n";
        std::cout << "  backend: " << toString(config_.renderBackend) << '\n';
        std::cout << "  resolution: " << config_.resolution.width << "x" << config_.resolution.height << '\n';
        std::cout << "  shadows: " << toString(config_.shadowQuality) << '\n';
        std::cout << "  antiAliasing: " << toString(config_.antiAliasing.mode);
        if (config_.antiAliasing.mode == AntiAliasing::Mode::MSAA) {
            std::cout << " x" << config_.antiAliasing.samples;
        }
        std::cout << '\n';
        std::cout << "  target FPS: " << config_.targetFramesPerSecond << '\n';
        std::cout << "  vsync: " << toString(config_.vSync) << '\n';
        std::cout << "  diagnostics: " << toString(config_.diagnosticsLevel) << '\n';
        std::cout << "  effective FPS: " << config_.effectiveFramesPerSecond << '\n';
    }

private:
    struct InternalConfig {
        RenderBackend renderBackend = RenderBackend::Vulkan;
        Resolution resolution = Resolution::FullHD();
        ShadowQuality shadowQuality = ShadowQuality::High;
        AntiAliasing antiAliasing = AntiAliasing::TAA();
        int targetFramesPerSecond = 60;
        VSync vSync = VSync::Enabled;
        DiagnosticsLevel diagnosticsLevel = DiagnosticsLevel::Basic;
        int effectiveFramesPerSecond = 60;
    };

    explicit RenderEngine(InternalConfig config)
        : config_(std::move(config)) {}

    static InternalConfig translate(RenderEngineConfiguration config) {
        validate(config);
        return InternalConfig{
            .renderBackend = config.renderBackend,
            .resolution = config.resolution,
            .shadowQuality = config.shadowQuality,
            .antiAliasing = config.antiAliasing,
            .targetFramesPerSecond = config.targetFramesPerSecond,
            .vSync = config.vSync,
            .diagnosticsLevel = config.diagnosticsLevel,
            .effectiveFramesPerSecond = resolveEffectiveFPS(config),
        };
    }

    static void validate(const RenderEngineConfiguration& config) {
        if (config.resolution.width <= 0 || config.resolution.height <= 0) {
            throw std::invalid_argument("resolution values must be > 0");
        }
        if (config.targetFramesPerSecond <= 0) {
            throw std::invalid_argument("targetFramesPerSecond must be > 0");
        }
    }

    static int resolveEffectiveFPS(const RenderEngineConfiguration& config) {
        if (config.vSync == VSync::Disabled) {
            return config.targetFramesPerSecond;
        }

        constexpr int monitorRefreshRate = 60;
        return std::min(config.targetFramesPerSecond, monitorRefreshRate);
    }

    void ensureRunning() const {
        if (!running_) {
            throw std::runtime_error("engine is not running");
        }
    }

    InternalConfig config_;
    bool running_ = false;
    std::unordered_map<std::string, std::string> images_;
};

int main() {
    RenderEngine engine = RenderEngine::Run();

    engine.LoadImage("whale", "whale.png");
    engine.PlaySound("music.ogg");

    engine.DrawText("Hello World!", 400, 300);
    engine.DrawImage("whale", 300, 200);

    engine.UpdateConfiguration({
        .renderBackend = RenderBackend::DirectX12,
        .resolution = Resolution::UHD4K(),
        .shadowQuality = ShadowQuality::Ultra,
        .antiAliasing = AntiAliasing::FXAA(),
        .targetFramesPerSecond = 144,
        .vSync = VSync::Enabled,
        .diagnosticsLevel = DiagnosticsLevel::Full,
    });

    engine.DrawText("After runtime config update", 180, 120);
    engine.Stop();
    return 0;
}
