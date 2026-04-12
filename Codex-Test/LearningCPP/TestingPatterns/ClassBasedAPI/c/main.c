#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef enum {
    RenderBackend_DirectX12,
    RenderBackend_Vulkan,
    RenderBackend_Metal,
    RenderBackend_OpenGL
} RenderBackend;

typedef enum {
    ShadowQuality_Off,
    ShadowQuality_Low,
    ShadowQuality_Medium,
    ShadowQuality_High,
    ShadowQuality_Ultra
} ShadowQuality;

typedef enum {
    AntiAliasingMode_None,
    AntiAliasingMode_FXAA,
    AntiAliasingMode_TAA,
    AntiAliasingMode_MSAA
} AntiAliasingMode;

typedef enum {
    DiagnosticsLevel_Off,
    DiagnosticsLevel_Overlay,
    DiagnosticsLevel_Validation,
    DiagnosticsLevel_Full
} DiagnosticsLevel;

typedef enum {
    RenderProfile_Balanced,
    RenderProfile_Performance,
    RenderProfile_Cinematic,
    RenderProfile_Development
} RenderProfile;

typedef struct {
    int width;
    int height;
} Resolution;

typedef struct {
    AntiAliasingMode mode;
    int msaaSamples;
} AntiAliasing;

#define MAX_METADATA_ENTRIES 16
#define MAX_KEY_LEN 63
#define MAX_VALUE_LEN 127

typedef struct {
    char key[MAX_KEY_LEN + 1];
    char value[MAX_VALUE_LEN + 1];
} MetadataEntry;

typedef struct {
    bool renderingEnabled;
    RenderBackend backend;
    int width;
    int height;
    bool vSyncEnabled;
    ShadowQuality shadowQuality;
    AntiAliasingMode antiAliasingMode;
    int msaaSamples;
    int targetFramesPerSecond;
    bool debugOverlayEnabled;
    bool gpuValidationEnabled;
    MetadataEntry metadata[MAX_METADATA_ENTRIES];
    size_t metadataCount;
} RenderOptions;

typedef struct {
    RenderProfile profile;
    const char* intendedUse;
    RenderOptions options;
} RenderProfileDescription;

typedef struct {
    RenderBackend renderBackend;
    Resolution resolution;
    ShadowQuality shadowQuality;
    AntiAliasing antiAliasing;
    int targetFramesPerSecond;
    bool vSyncEnabled;
    DiagnosticsLevel diagnosticsLevel;
    bool renderingEnabled;
    MetadataEntry metadata[MAX_METADATA_ENTRIES];
    size_t metadataCount;
} RenderEngineConfiguration;

typedef struct {
    RenderOptions options;
    bool started;
    bool disposed;
} RenderEngine;

static Resolution Resolution_HD(void) { return (Resolution){1280, 720}; }
static Resolution Resolution_FullHD(void) { return (Resolution){1920, 1080}; }
static Resolution Resolution_QHD(void) { return (Resolution){2560, 1440}; }
static Resolution Resolution_UHD4K(void) { return (Resolution){3840, 2160}; }

static AntiAliasing AntiAliasing_None(void) { return (AntiAliasing){AntiAliasingMode_None, 1}; }
static AntiAliasing AntiAliasing_FXAA(void) { return (AntiAliasing){AntiAliasingMode_FXAA, 1}; }
static AntiAliasing AntiAliasing_TAA(void) { return (AntiAliasing){AntiAliasingMode_TAA, 1}; }
static AntiAliasing AntiAliasing_MSAA(int samples) { return (AntiAliasing){AntiAliasingMode_MSAA, samples}; }

static const char* ToStringBackend(RenderBackend value) {
    switch (value) {
        case RenderBackend_DirectX12: return "DirectX12";
        case RenderBackend_Vulkan: return "Vulkan";
        case RenderBackend_Metal: return "Metal";
        case RenderBackend_OpenGL: return "OpenGL";
        default: return "Unknown";
    }
}

static const char* ToStringShadow(ShadowQuality value) {
    switch (value) {
        case ShadowQuality_Off: return "Off";
        case ShadowQuality_Low: return "Low";
        case ShadowQuality_Medium: return "Medium";
        case ShadowQuality_High: return "High";
        case ShadowQuality_Ultra: return "Ultra";
        default: return "Unknown";
    }
}

static const char* ToStringAA(AntiAliasingMode value) {
    switch (value) {
        case AntiAliasingMode_None: return "None";
        case AntiAliasingMode_FXAA: return "FXAA";
        case AntiAliasingMode_TAA: return "TAA";
        case AntiAliasingMode_MSAA: return "MSAA";
        default: return "Unknown";
    }
}

static const char* ToStringProfile(RenderProfile value) {
    switch (value) {
        case RenderProfile_Balanced: return "Balanced";
        case RenderProfile_Performance: return "Performance";
        case RenderProfile_Cinematic: return "Cinematic";
        case RenderProfile_Development: return "Development";
        default: return "Unknown";
    }
}

static bool ValidateResolution(Resolution resolution, char* error, size_t errorSize) {
    if (resolution.width <= 0) {
        snprintf(error, errorSize, "resolution.width must be greater than zero.");
        return false;
    }

    if (resolution.height <= 0) {
        snprintf(error, errorSize, "resolution.height must be greater than zero.");
        return false;
    }

    return true;
}

static bool ValidateAntiAliasing(AntiAliasing antiAliasing, char* error, size_t errorSize) {
    if (antiAliasing.mode == AntiAliasingMode_MSAA) {
        if (antiAliasing.msaaSamples == 2 || antiAliasing.msaaSamples == 4 || antiAliasing.msaaSamples == 8 ||
            antiAliasing.msaaSamples == 16) {
            return true;
        }

        snprintf(error, errorSize, "MSAA samples must be one of: 2, 4, 8, or 16.");
        return false;
    }

    if (antiAliasing.msaaSamples != 1) {
        snprintf(error, errorSize, "MSAA sample count can only be customized when mode is MSAA.");
        return false;
    }

    return true;
}

static bool BuildOptions(RenderEngineConfiguration configuration, RenderOptions* outOptions, char* error, size_t errorSize) {
    if (outOptions == NULL) {
        snprintf(error, errorSize, "outOptions must not be NULL.");
        return false;
    }

    if (!ValidateResolution(configuration.resolution, error, errorSize)) {
        return false;
    }

    if (!ValidateAntiAliasing(configuration.antiAliasing, error, errorSize)) {
        return false;
    }

    if (configuration.targetFramesPerSecond <= 0) {
        snprintf(error, errorSize, "targetFramesPerSecond must be greater than zero.");
        return false;
    }

    if (configuration.metadataCount > MAX_METADATA_ENTRIES) {
        snprintf(error, errorSize, "metadataCount exceeds max entries.");
        return false;
    }

    outOptions->renderingEnabled = configuration.renderingEnabled;
    outOptions->backend = configuration.renderBackend;
    outOptions->width = configuration.resolution.width;
    outOptions->height = configuration.resolution.height;
    outOptions->vSyncEnabled = configuration.vSyncEnabled;
    outOptions->shadowQuality = configuration.shadowQuality;
    outOptions->antiAliasingMode = configuration.antiAliasing.mode;
    outOptions->msaaSamples = configuration.antiAliasing.msaaSamples;
    outOptions->targetFramesPerSecond = configuration.targetFramesPerSecond;
    outOptions->debugOverlayEnabled =
        configuration.diagnosticsLevel == DiagnosticsLevel_Overlay || configuration.diagnosticsLevel == DiagnosticsLevel_Full;
    outOptions->gpuValidationEnabled =
        configuration.diagnosticsLevel == DiagnosticsLevel_Validation || configuration.diagnosticsLevel == DiagnosticsLevel_Full;
    outOptions->metadataCount = configuration.metadataCount;

    for (size_t i = 0; i < configuration.metadataCount; ++i) {
        outOptions->metadata[i] = configuration.metadata[i];
    }

    return true;
}

static RenderEngineConfiguration RenderEngineConfiguration_Init(
    RenderBackend renderBackend,
    Resolution resolution,
    ShadowQuality shadowQuality,
    AntiAliasing antiAliasing,
    int targetFramesPerSecond,
    bool vSyncEnabled,
    DiagnosticsLevel diagnosticsLevel,
    bool renderingEnabled) {
    RenderEngineConfiguration configuration;
    memset(&configuration, 0, sizeof(configuration));

    configuration.renderBackend = renderBackend;
    configuration.resolution = resolution;
    configuration.shadowQuality = shadowQuality;
    configuration.antiAliasing = antiAliasing;
    configuration.targetFramesPerSecond = targetFramesPerSecond;
    configuration.vSyncEnabled = vSyncEnabled;
    configuration.diagnosticsLevel = diagnosticsLevel;
    configuration.renderingEnabled = renderingEnabled;

    return configuration;
}

static bool RenderEngineConfiguration_AddMetadata(RenderEngineConfiguration* configuration, const char* key, const char* value) {
    if (configuration == NULL || key == NULL || value == NULL) {
        return false;
    }

    if (configuration->metadataCount >= MAX_METADATA_ENTRIES) {
        return false;
    }

    MetadataEntry* entry = &configuration->metadata[configuration->metadataCount++];
    snprintf(entry->key, sizeof(entry->key), "%s", key);
    snprintf(entry->value, sizeof(entry->value), "%s", value);
    return true;
}

static RenderProfileDescription gProfiles[4];
static bool gProfilesInitialized = false;

static void InitializeProfiles(void) {
    if (gProfilesInitialized) {
        return;
    }

    char error[256] = {0};

    RenderEngineConfiguration balanced = RenderEngineConfiguration_Init(
        RenderBackend_Vulkan, Resolution_FullHD(), ShadowQuality_Medium, AntiAliasing_TAA(), 60, true, DiagnosticsLevel_Off, true);
    BuildOptions(balanced, &gProfiles[0].options, error, sizeof(error));
    gProfiles[0].profile = RenderProfile_Balanced;
    gProfiles[0].intendedUse = "General gameplay across typical desktop hardware.";

    RenderEngineConfiguration performance = RenderEngineConfiguration_Init(
        RenderBackend_DirectX12, Resolution_FullHD(), ShadowQuality_Low, AntiAliasing_FXAA(), 144, false, DiagnosticsLevel_Off, true);
    BuildOptions(performance, &gProfiles[1].options, error, sizeof(error));
    gProfiles[1].profile = RenderProfile_Performance;
    gProfiles[1].intendedUse = "High FPS for competitive gameplay.";

    RenderEngineConfiguration cinematic = RenderEngineConfiguration_Init(
        RenderBackend_Vulkan, Resolution_QHD(), ShadowQuality_Ultra, AntiAliasing_TAA(), 60, true, DiagnosticsLevel_Off, true);
    BuildOptions(cinematic, &gProfiles[2].options, error, sizeof(error));
    gProfiles[2].profile = RenderProfile_Cinematic;
    gProfiles[2].intendedUse = "Visual quality focused scenes and demos.";

    RenderEngineConfiguration development = RenderEngineConfiguration_Init(
        RenderBackend_OpenGL, (Resolution){1600, 900}, ShadowQuality_Off, AntiAliasing_None(), 60, false, DiagnosticsLevel_Full, true);
    BuildOptions(development, &gProfiles[3].options, error, sizeof(error));
    gProfiles[3].profile = RenderProfile_Development;
    gProfiles[3].intendedUse = "Debugging and validation while building features.";

    gProfilesInitialized = true;
}

static const RenderOptions* GetProfileOptions(RenderProfile profile) {
    InitializeProfiles();
    for (size_t i = 0; i < 4; ++i) {
        if (gProfiles[i].profile == profile) {
            return &gProfiles[i].options;
        }
    }
    return NULL;
}

static RenderEngineConfiguration RenderEngineConfiguration_FromProfile(RenderProfile profile) {
    const RenderOptions* options = GetProfileOptions(profile);
    RenderEngineConfiguration configuration = RenderEngineConfiguration_Init(
        options->backend,
        (Resolution){options->width, options->height},
        options->shadowQuality,
        (AntiAliasing){options->antiAliasingMode, options->msaaSamples},
        options->targetFramesPerSecond,
        options->vSyncEnabled,
        options->debugOverlayEnabled && options->gpuValidationEnabled
            ? DiagnosticsLevel_Full
            : options->debugOverlayEnabled
                ? DiagnosticsLevel_Overlay
                : options->gpuValidationEnabled
                    ? DiagnosticsLevel_Validation
                    : DiagnosticsLevel_Off,
        options->renderingEnabled);

    configuration.metadataCount = options->metadataCount;
    for (size_t i = 0; i < options->metadataCount; ++i) {
        configuration.metadata[i] = options->metadata[i];
    }

    return configuration;
}

static const RenderProfileDescription* Graphics3D_Profiles(size_t* count) {
    InitializeProfiles();
    if (count != NULL) {
        *count = 4;
    }
    return gProfiles;
}

static RenderEngine Graphics3D_Default(void) {
    RenderEngine engine;
    memset(&engine, 0, sizeof(engine));
    const RenderOptions* options = GetProfileOptions(RenderProfile_Balanced);
    engine.options = *options;
    return engine;
}

static RenderEngine Graphics3D_DefaultProfile(RenderProfile profile) {
    RenderEngine engine;
    memset(&engine, 0, sizeof(engine));
    const RenderOptions* options = GetProfileOptions(profile);
    if (options != NULL) {
        engine.options = *options;
    }
    return engine;
}

static bool Graphics3D_Configure(RenderEngineConfiguration configuration, RenderEngine* outEngine, char* error, size_t errorSize) {
    if (outEngine == NULL) {
        snprintf(error, errorSize, "outEngine must not be NULL.");
        return false;
    }

    RenderOptions options;
    memset(&options, 0, sizeof(options));

    if (!BuildOptions(configuration, &options, error, errorSize)) {
        return false;
    }

    memset(outEngine, 0, sizeof(*outEngine));
    outEngine->options = options;
    return true;
}

static void RenderEngine_Start(RenderEngine* engine) {
    if (engine == NULL || engine->disposed) {
        fprintf(stderr, "Engine is null or disposed.\n");
        return;
    }

    if (engine->started) {
        fprintf(stderr, "The render engine has already been started.\n");
        return;
    }

    if (!engine->options.renderingEnabled) {
        fprintf(stderr, "Rendering is disabled in the configured options.\n");
        return;
    }

    engine->started = true;
    printf(
        "Renderer started. Backend=%s, Resolution=%dx%d, VSync=%s, Shadows=%s, AA=%s, MSAA=%d, TargetFPS=%d, DebugOverlay=%s, GpuValidation=%s\n",
        ToStringBackend(engine->options.backend),
        engine->options.width,
        engine->options.height,
        engine->options.vSyncEnabled ? "true" : "false",
        ToStringShadow(engine->options.shadowQuality),
        ToStringAA(engine->options.antiAliasingMode),
        engine->options.msaaSamples,
        engine->options.targetFramesPerSecond,
        engine->options.debugOverlayEnabled ? "true" : "false",
        engine->options.gpuValidationEnabled ? "true" : "false");
}

static void RenderEngine_Stop(RenderEngine* engine) {
    if (engine == NULL || engine->disposed || !engine->started) {
        return;
    }

    engine->started = false;
    printf("Renderer stopped.\n");
}

static void RenderEngine_Dispose(RenderEngine* engine) {
    if (engine == NULL || engine->disposed) {
        return;
    }

    RenderEngine_Stop(engine);
    engine->disposed = true;
}

int main(void) {
    size_t profileCount = 0;
    const RenderProfileDescription* profiles = Graphics3D_Profiles(&profileCount);
    for (size_t i = 0; i < profileCount; ++i) {
        printf("%s: %s\n", ToStringProfile(profiles[i].profile), profiles[i].intendedUse);
    }

    RenderEngine defaultEngine = Graphics3D_Default();
    RenderEngine_Start(&defaultEngine);
    RenderEngine_Stop(&defaultEngine);
    RenderEngine_Dispose(&defaultEngine);

    RenderEngineConfiguration customConfigDesignated = {
        .renderBackend = RenderBackend_DirectX12,
        .resolution = Resolution_UHD4K(),
        .shadowQuality = ShadowQuality_Ultra,
        .antiAliasing = AntiAliasing_FXAA(),
        .targetFramesPerSecond = 144,
        .vSyncEnabled = true,
        .diagnosticsLevel = DiagnosticsLevel_Full,
        .renderingEnabled = true
    };
    RenderEngineConfiguration_AddMetadata(&customConfigDesignated, "environment", "production");

    RenderEngine customEngineDesignated;
    char error[256] = {0};
    if (!Graphics3D_Configure(customConfigDesignated, &customEngineDesignated, error, sizeof(error))) {
        fprintf(stderr, "Configure failed: %s\n", error);
        return 1;
    }

    RenderEngine_Start(&customEngineDesignated);
    RenderEngine_Stop(&customEngineDesignated);
    RenderEngine_Dispose(&customEngineDesignated);

    RenderEngineConfiguration customConfigCtor = RenderEngineConfiguration_Init(
        RenderBackend_Vulkan,
        Resolution_QHD(),
        ShadowQuality_High,
        AntiAliasing_MSAA(4),
        144,
        true,
        DiagnosticsLevel_Full,
        true);
    RenderEngineConfiguration_AddMetadata(&customConfigCtor, "scene", "hangar");

    RenderEngine customEngineCtor;
    if (!Graphics3D_Configure(customConfigCtor, &customEngineCtor, error, sizeof(error))) {
        fprintf(stderr, "Configure failed: %s\n", error);
        return 1;
    }

    RenderEngine_Start(&customEngineCtor);
    RenderEngine_Stop(&customEngineCtor);
    RenderEngine_Dispose(&customEngineCtor);

    RenderEngine fromProfile = Graphics3D_DefaultProfile(RenderProfile_Development);
    RenderEngine_Start(&fromProfile);
    RenderEngine_Stop(&fromProfile);
    RenderEngine_Dispose(&fromProfile);

    RenderEngineConfiguration clonedProfileConfig = RenderEngineConfiguration_FromProfile(RenderProfile_Cinematic);
    RenderEngine clonedProfileEngine;
    if (Graphics3D_Configure(clonedProfileConfig, &clonedProfileEngine, error, sizeof(error))) {
        RenderEngine_Start(&clonedProfileEngine);
        RenderEngine_Stop(&clonedProfileEngine);
        RenderEngine_Dispose(&clonedProfileEngine);
    }

    return 0;
}
