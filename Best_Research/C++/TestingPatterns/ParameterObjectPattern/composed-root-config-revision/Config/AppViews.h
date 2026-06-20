#pragma once

#include "AudioConfig.h"
#include "DiagnosticsConfig.h"
#include "DisplayConfig.h"
#include "RenderConfig.h"
#include "WindowConfig.h"

namespace Config {

struct App;

struct RenderSettingsView {
    const Display& display;
    const Diagnostics& diagnostics;
    const RenderEngine& render;
};

struct AudioSettingsView {
    const Diagnostics& diagnostics;
    const Audio& audio;
};

struct WindowSettingsView {
    const Display& display;
    const Window& window;
};

RenderSettingsView getRenderSettings(const App& config);
AudioSettingsView getAudioSettings(const App& config);
WindowSettingsView getWindowSettings(const App& config);

void validateConfig(const App& config);

} // namespace Config
