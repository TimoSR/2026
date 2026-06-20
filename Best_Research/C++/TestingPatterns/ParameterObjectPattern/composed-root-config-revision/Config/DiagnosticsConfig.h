#pragma once

namespace Config {

enum class DiagnosticsLevel {
    None,
    Basic,
    Full,
};

struct Diagnostics {
    DiagnosticsLevel level {DiagnosticsLevel::Basic};
};

Diagnostics makeDefaultDiagnosticsConfig();

} // namespace Config

const char* toString(Config::DiagnosticsLevel value);
