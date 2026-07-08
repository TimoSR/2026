#include "DiagnosticsConfig.h"

namespace Config {

Diagnostics makeDefaultDiagnosticsConfig() {
    return Diagnostics{
        .level = DiagnosticsLevel::Full,
    };
}

} // namespace Config

const char* toString(Config::DiagnosticsLevel value) {
    switch (value) {
        case Config::DiagnosticsLevel::None: return "None";
        case Config::DiagnosticsLevel::Basic: return "Basic";
        case Config::DiagnosticsLevel::Full: return "Full";
    }
    return "Unknown";
}
