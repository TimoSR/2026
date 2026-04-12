#pragma once

#include "Config/AppViews.h"

namespace Config {

struct App {
    Display display {};
    Diagnostics diagnostics {};
    RenderEngine render {};
    Audio audio {};
    Window window {};
};

App makeDefaultAppConfig();

} // namespace Config
