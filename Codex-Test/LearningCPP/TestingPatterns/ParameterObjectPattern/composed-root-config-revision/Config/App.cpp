#include "../App.h"

#include <stdexcept>

namespace Config {

RenderSettingsView getRenderSettings(const App& config) {
    return RenderSettingsView{
        .display = config.display,
        .diagnostics = config.diagnostics,
        .render = config.render,
    };
}

AudioSettingsView getAudioSettings(const App& config) {
    return AudioSettingsView{
        .diagnostics = config.diagnostics,
        .audio = config.audio,
    };
}

WindowSettingsView getWindowSettings(const App& config) {
    return WindowSettingsView{
        .display = config.display,
        .window = config.window,
    };
}

App makeDefaultAppConfig() {
    return App{
        .display = makeDefaultDisplayConfig(),
        .diagnostics = makeDefaultDiagnosticsConfig(),
        .render = makeDefaultRenderConfig(),
        .audio = makeDefaultAudioConfig(),
        .window = makeDefaultWindowConfig(),
    };
}

void validateConfig(const App& config) {
    if (config.display.resolution.width == 0U || config.display.resolution.height == 0U) {
        throw std::invalid_argument("display resolution must be non-zero");
    }

    if (config.display.refreshRate.fps == 0U) {
        throw std::invalid_argument("display refresh rate must be non-zero");
    }

    if (config.audio.masterVolume.percent > 100U) {
        throw std::invalid_argument("masterVolume must be between 0 and 100");
    }

    if (config.audio.musicVolume.percent > 100U) {
        throw std::invalid_argument("musicVolume must be between 0 and 100");
    }

    if (config.audio.sfxVolume.percent > 100U) {
        throw std::invalid_argument("sfxVolume must be between 0 and 100");
    }

    if (config.audio.outputSampleRate.hz < 8000U) {
        throw std::invalid_argument("output sample rate is invalid");
    }

    if (config.window.title.empty()) {
        throw std::invalid_argument("window title must be non-empty");
    }

    if (config.render.antiAliasing.mode != AntiAliasing::Mode::MSAA &&
        config.render.antiAliasing.samples != 1U) {
        throw std::invalid_argument("antiAliasing samples must be 1 unless mode is MSAA");
    }

    if (config.render.antiAliasing.mode == AntiAliasing::Mode::MSAA) {
        const std::uint32_t samples = config.render.antiAliasing.samples;
        if (!(samples == 2U || samples == 4U || samples == 8U)) {
            throw std::invalid_argument("MSAA sample count must be 2, 4, or 8");
        }
    }
}

} // namespace Config
