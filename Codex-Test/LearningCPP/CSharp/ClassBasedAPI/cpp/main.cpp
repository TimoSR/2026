#include <iostream>
#include <map>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

namespace FluentGraphics3D {

enum class RenderBackend { DirectX12, Vulkan, Metal, OpenGL };
enum class ShadowQuality { Off, Low, Medium, High, Ultra };
enum class AntiAliasingMode { None, FXAA, TAA, MSAA };
enum class DiagnosticsLevel { Off, Overlay, Validation, Full };
enum class RenderProfile { Balanced, Performance, Cinematic, Development };

struct Resolution {
    int width {1920};
    int height {1080};

    static Resolution HD() { return {1280, 720}; }
    static Resolution FullHD() { return {1920, 1080}; }
    static Resolution QHD() { return {2560, 1440}; }
    static Resolution UHD4K() { return {3840, 2160}; }
};

struct AntiAliasing {
    AntiAliasingMode mode {AntiAliasingMode::None};
    int msaaSamples {1};

    static AntiAliasing None() { return {AntiAliasingMode::None, 1}; }
    static AntiAliasing FXAA() { return {AntiAliasingMode::FXAA, 1}; }
    static AntiAliasing TAA() { return {AntiAliasingMode::TAA, 1}; }
    static AntiAliasing MSAA(int samples) { return {AntiAliasingMode::MSAA, samples}; }
};

struct RenderOptions {
    bool renderingEnabled {};
    RenderBackend backend {};
    int width {};
    int height {};
    bool vSyncEnabled {};
    ShadowQuality shadowQuality {};
    AntiAliasingMode antiAliasingMode {};
    int msaaSamples {};
    int targetFramesPerSecond {};
    bool debugOverlayEnabled {};
    bool gpuValidationEnabled {};
    std::map<std::string, std::string> metadata;
};

struct RenderProfileDescription {
    RenderProfile profile {};
    std::string intendedUse;
    RenderOptions options;
};

struct RenderEngineConfiguration {
    RenderBackend renderBackend {RenderBackend::Vulkan};
    Resolution resolution {Resolution::FullHD()};
    ShadowQuality shadowQuality {ShadowQuality::Medium};
    AntiAliasing antiAliasing {AntiAliasing::None()};
    int targetFramesPerSecond {60};
    bool vSyncEnabled {false};
    DiagnosticsLevel diagnosticsLevel {DiagnosticsLevel::Off};
    bool renderingEnabled {true};
    std::map<std::string, std::string> metadata {};

    static RenderEngineConfiguration FromProfile(RenderProfile profile);
};

class RenderEngineConfigurationCtor {
public:
    explicit RenderEngineConfigurationCtor(
        RenderBackend renderBackend,
        Resolution resolution,
        ShadowQuality shadowQuality = ShadowQuality::Medium,
        AntiAliasing antiAliasing = AntiAliasing::None(),
        int targetFramesPerSecond = 60,
        bool vSyncEnabled = false,
        DiagnosticsLevel diagnosticsLevel = DiagnosticsLevel::Off,
        bool renderingEnabled = true,
        std::map<std::string, std::string> metadata = {});

    RenderEngineConfiguration ToConfiguration() const;

private:
    RenderEngineConfiguration value {};
};

namespace detail {

static const char* ToString(RenderBackend value) {
    switch (value) {
    case RenderBackend::DirectX12:
        return "DirectX12";
    case RenderBackend::Vulkan:
        return "Vulkan";
    case RenderBackend::Metal:
        return "Metal";
    case RenderBackend::OpenGL:
        return "OpenGL";
    }

    return "Unknown";
}

static const char* ToString(ShadowQuality value) {
    switch (value) {
    case ShadowQuality::Off:
        return "Off";
    case ShadowQuality::Low:
        return "Low";
    case ShadowQuality::Medium:
        return "Medium";
    case ShadowQuality::High:
        return "High";
    case ShadowQuality::Ultra:
        return "Ultra";
    }

    return "Unknown";
}

static const char* ToString(AntiAliasingMode value) {
    switch (value) {
    case AntiAliasingMode::None:
        return "None";
    case AntiAliasingMode::FXAA:
        return "FXAA";
    case AntiAliasingMode::TAA:
        return "TAA";
    case AntiAliasingMode::MSAA:
        return "MSAA";
    }

    return "Unknown";
}

static const char* ToString(RenderProfile value) {
    switch (value) {
    case RenderProfile::Balanced:
        return "Balanced";
    case RenderProfile::Performance:
        return "Performance";
    case RenderProfile::Cinematic:
        return "Cinematic";
    case RenderProfile::Development:
        return "Development";
    }

    return "Unknown";
}

template <typename TEnum>
static void ThrowInvalidEnum(const char* name) {
    throw std::out_of_range(std::string(name) + " is not a valid enum value.");
}

static void ValidateRenderBackend(RenderBackend value) {
    switch (value) {
    case RenderBackend::DirectX12:
    case RenderBackend::Vulkan:
    case RenderBackend::Metal:
    case RenderBackend::OpenGL:
        return;
    }

    ThrowInvalidEnum<RenderBackend>("renderBackend");
}

static void ValidateShadowQuality(ShadowQuality value) {
    switch (value) {
    case ShadowQuality::Off:
    case ShadowQuality::Low:
    case ShadowQuality::Medium:
    case ShadowQuality::High:
    case ShadowQuality::Ultra:
        return;
    }

    ThrowInvalidEnum<ShadowQuality>("shadowQuality");
}

static void ValidateAntiAliasingMode(AntiAliasingMode value) {
    switch (value) {
    case AntiAliasingMode::None:
    case AntiAliasingMode::FXAA:
    case AntiAliasingMode::TAA:
    case AntiAliasingMode::MSAA:
        return;
    }

    ThrowInvalidEnum<AntiAliasingMode>("antiAliasing.mode");
}

static void ValidateDiagnosticsLevel(DiagnosticsLevel value) {
    switch (value) {
    case DiagnosticsLevel::Off:
    case DiagnosticsLevel::Overlay:
    case DiagnosticsLevel::Validation:
    case DiagnosticsLevel::Full:
        return;
    }

    ThrowInvalidEnum<DiagnosticsLevel>("diagnosticsLevel");
}

static void ValidateResolution(const Resolution& resolution) {
    if (resolution.width <= 0) {
        throw std::out_of_range("resolution.width must be greater than zero.");
    }

    if (resolution.height <= 0) {
        throw std::out_of_range("resolution.height must be greater than zero.");
    }
}

static void ValidateAntiAliasing(const AntiAliasing& antiAliasing) {
    ValidateAntiAliasingMode(antiAliasing.mode);

    if (antiAliasing.mode == AntiAliasingMode::MSAA) {
        if (antiAliasing.msaaSamples == 2 || antiAliasing.msaaSamples == 4 || antiAliasing.msaaSamples == 8 ||
            antiAliasing.msaaSamples == 16) {
            return;
        }

        throw std::out_of_range("MSAA samples must be one of: 2, 4, 8, or 16.");
    }

    if (antiAliasing.msaaSamples != 1) {
        throw std::invalid_argument("MSAA sample count can only be customized when mode is MSAA.");
    }
}

static RenderOptions BuildOptions(const RenderEngineConfiguration& configuration) {
    ValidateRenderBackend(configuration.renderBackend);
    ValidateResolution(configuration.resolution);
    ValidateShadowQuality(configuration.shadowQuality);
    ValidateAntiAliasing(configuration.antiAliasing);
    ValidateDiagnosticsLevel(configuration.diagnosticsLevel);

    if (configuration.targetFramesPerSecond <= 0) {
        throw std::out_of_range("targetFramesPerSecond must be greater than zero.");
    }

    const bool debugOverlayEnabled =
        configuration.diagnosticsLevel == DiagnosticsLevel::Overlay || configuration.diagnosticsLevel == DiagnosticsLevel::Full;
    const bool gpuValidationEnabled =
        configuration.diagnosticsLevel == DiagnosticsLevel::Validation || configuration.diagnosticsLevel == DiagnosticsLevel::Full;

    return RenderOptions {
        configuration.renderingEnabled,
        configuration.renderBackend,
        configuration.resolution.width,
        configuration.resolution.height,
        configuration.vSyncEnabled,
        configuration.shadowQuality,
        configuration.antiAliasing.mode,
        configuration.antiAliasing.msaaSamples,
        configuration.targetFramesPerSecond,
        debugOverlayEnabled,
        gpuValidationEnabled,
        configuration.metadata
    };
}

static std::vector<RenderProfileDescription> BuildProfiles() {
    std::vector<RenderProfileDescription> profiles;
    profiles.push_back(RenderProfileDescription {
        RenderProfile::Balanced,
        "General gameplay across typical desktop hardware.",
        BuildOptions(RenderEngineConfiguration {
            .renderBackend = RenderBackend::Vulkan,
            .resolution = Resolution::FullHD(),
            .shadowQuality = ShadowQuality::Medium,
            .antiAliasing = AntiAliasing::TAA(),
            .targetFramesPerSecond = 60,
            .vSyncEnabled = true,
            .diagnosticsLevel = DiagnosticsLevel::Off
        })
    });
    profiles.push_back(RenderProfileDescription {
        RenderProfile::Performance,
        "High FPS for competitive gameplay.",
        BuildOptions(RenderEngineConfiguration {
            .renderBackend = RenderBackend::DirectX12,
            .resolution = Resolution::FullHD(),
            .shadowQuality = ShadowQuality::Low,
            .antiAliasing = AntiAliasing::FXAA(),
            .targetFramesPerSecond = 144,
            .vSyncEnabled = false,
            .diagnosticsLevel = DiagnosticsLevel::Off
        })
    });
    profiles.push_back(RenderProfileDescription {
        RenderProfile::Cinematic,
        "Visual quality focused scenes and demos.",
        BuildOptions(RenderEngineConfiguration {
            .renderBackend = RenderBackend::Vulkan,
            .resolution = Resolution::QHD(),
            .shadowQuality = ShadowQuality::Ultra,
            .antiAliasing = AntiAliasing::TAA(),
            .targetFramesPerSecond = 60,
            .vSyncEnabled = true,
            .diagnosticsLevel = DiagnosticsLevel::Off
        })
    });
    profiles.push_back(RenderProfileDescription {
        RenderProfile::Development,
        "Debugging and validation while building features.",
        BuildOptions(RenderEngineConfiguration {
            .renderBackend = RenderBackend::OpenGL,
            .resolution = {1600, 900},
            .shadowQuality = ShadowQuality::Off,
            .antiAliasing = AntiAliasing::None(),
            .targetFramesPerSecond = 60,
            .vSyncEnabled = false,
            .diagnosticsLevel = DiagnosticsLevel::Full
        })
    });

    return profiles;
}

static const std::vector<RenderProfileDescription>& Profiles() {
    static const std::vector<RenderProfileDescription> kProfiles = BuildProfiles();
    return kProfiles;
}

static const RenderOptions& GetProfileOptions(RenderProfile profile) {
    for (const RenderProfileDescription& description : Profiles()) {
        if (description.profile == profile) {
            return description.options;
        }
    }

    throw std::out_of_range("Unknown render profile.");
}

} // namespace detail

inline RenderEngineConfiguration RenderEngineConfiguration::FromProfile(RenderProfile profile) {
    const RenderOptions& options = detail::GetProfileOptions(profile);
    return RenderEngineConfiguration {
        .renderBackend = options.backend,
        .resolution = {options.width, options.height},
        .shadowQuality = options.shadowQuality,
        .antiAliasing = {options.antiAliasingMode, options.msaaSamples},
        .targetFramesPerSecond = options.targetFramesPerSecond,
        .vSyncEnabled = options.vSyncEnabled,
        .diagnosticsLevel = options.debugOverlayEnabled && options.gpuValidationEnabled
                                ? DiagnosticsLevel::Full
                                : options.debugOverlayEnabled
                                    ? DiagnosticsLevel::Overlay
                                    : options.gpuValidationEnabled
                                        ? DiagnosticsLevel::Validation
                                        : DiagnosticsLevel::Off,
        .renderingEnabled = options.renderingEnabled,
        .metadata = options.metadata
    };
}

inline RenderEngineConfigurationCtor::RenderEngineConfigurationCtor(
    RenderBackend renderBackend,
    Resolution resolution,
    ShadowQuality shadowQuality,
    AntiAliasing antiAliasing,
    int targetFramesPerSecond,
    bool vSyncEnabled,
    DiagnosticsLevel diagnosticsLevel,
    bool renderingEnabled,
    std::map<std::string, std::string> metadata) {
    value.renderBackend = renderBackend;
    value.resolution = resolution;
    value.shadowQuality = shadowQuality;
    value.antiAliasing = antiAliasing;
    value.targetFramesPerSecond = targetFramesPerSecond;
    value.vSyncEnabled = vSyncEnabled;
    value.diagnosticsLevel = diagnosticsLevel;
    value.renderingEnabled = renderingEnabled;
    value.metadata = std::move(metadata);
}

inline RenderEngineConfiguration RenderEngineConfigurationCtor::ToConfiguration() const {
    return value;
}

class RenderEngine {
public:
    explicit RenderEngine(RenderOptions options)
        : options_(std::move(options)) {}

    const RenderOptions& Options() const { return options_; }

    void Start() {
        if (started_) {
            throw std::logic_error("The render engine has already been started.");
        }

        if (!options_.renderingEnabled) {
            throw std::logic_error("Rendering is disabled in the configured options.");
        }

        started_ = true;

        std::cout << "Renderer started. "
                  << "Backend=" << detail::ToString(options_.backend)
                  << ", Resolution=" << options_.width << "x" << options_.height
                  << ", VSync=" << (options_.vSyncEnabled ? "true" : "false")
                  << ", Shadows=" << detail::ToString(options_.shadowQuality)
                  << ", AA=" << detail::ToString(options_.antiAliasingMode)
                  << ", MSAA=" << options_.msaaSamples
                  << ", TargetFPS=" << options_.targetFramesPerSecond
                  << ", DebugOverlay=" << (options_.debugOverlayEnabled ? "true" : "false")
                  << ", GpuValidation=" << (options_.gpuValidationEnabled ? "true" : "false")
                  << "\n";
    }

    void Stop() {
        if (!started_) {
            return;
        }

        started_ = false;
        std::cout << "Renderer stopped.\n";
    }

    ~RenderEngine() {
        try {
            Stop();
        } catch (...) {
            // Avoid throwing from destructor.
        }
    }

private:
    RenderOptions options_;
    bool started_ {false};
};

class Graphics3D {
public:
    static const std::vector<RenderProfileDescription>& Profiles() { return detail::Profiles(); }

    static RenderEngine Default() { return Default(RenderProfile::Balanced); }

    static RenderEngine Default(RenderProfile profile) { return RenderEngine(detail::GetProfileOptions(profile)); }

    static RenderEngine Configure(RenderEngineConfiguration configuration) {
        return RenderEngine(detail::BuildOptions(configuration));
    }

    static RenderEngine Configure(RenderEngineConfigurationCtor configuration) {
        return Configure(configuration.ToConfiguration());
    }
};

} // namespace FluentGraphics3D

int main() {
    
    using namespace FluentGraphics3D;

    RenderEngine defaultEngine = Graphics3D::Default();
    defaultEngine.Start();
    defaultEngine.Stop();
    

    RenderEngineConfiguration customConfig2 {
        .renderBackend = RenderBackend::DirectX12,
        .resolution = Resolution::UHD4K(),
        .shadowQuality = ShadowQuality::Ultra,
        .antiAliasing = AntiAliasing::FXAA(),
        .targetFramesPerSecond = 144,
        .vSyncEnabled = true,
        .diagnosticsLevel = DiagnosticsLevel::Full
    };

    RenderEngine customEngine = Graphics3D::Configure(customConfig2);
    customEngine.Start();
    customEngine.Stop();

    RenderEngineConfigurationCtor customConfigWithConstructor(
        RenderBackend::Vulkan,
        Resolution::QHD(),
        ShadowQuality::High,
        AntiAliasing::MSAA(4),
        144,
        true,
        DiagnosticsLevel::Full);

    RenderEngine customEngineFromCtor = Graphics3D::Configure(customConfigWithConstructor);
    customEngineFromCtor.Start();
    customEngineFromCtor.Stop();

    RenderEngine engine = Graphics3D::Configure(customConfigWithConstructor);
}
