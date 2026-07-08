# Composed Root Config Revision (C++)

This variant demonstrates a composed root config (`Config::App`) with shared sub-objects:

- `display` (shared by renderer + window)
- `diagnostics` (shared policy)
- `render` (render-only)
- `audio` (audio-only)
- `window` (window-only)

No-argument constructors (`Graphics::Create()`, `Audio::Create()`, `Window::Create()`) use
`Config::makeDefaultAppConfig()`, which is a high-end preset profile.

Config types are split into focused headers under `Config/` (`DisplayConfig.h`,
`RenderConfig.h`, `AudioConfig.h`, `WindowConfig.h`, `DiagnosticsConfig.h`) and composed in
`Config/App.h`.
Implementations are split the same way (`Config/App.cpp`, `Config/DisplayConfig.cpp`,
`Config/RenderConfig.cpp`, `Config/AudioConfig.cpp`, `Config/WindowConfig.cpp`,
`Config/DiagnosticsConfig.cpp`), with no central `ConfigTypes.cpp`.

Run from this folder in PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -File .\run.ps1
```
