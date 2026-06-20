# Engine Config Example

This example keeps the API surface close to your preferred style while moving all config types and config values into a dedicated `config/` area.

## Structure

- `include/config/` contains the config types and the functions that provide the app's chosen config values.
- `include/engine/` contains the engine-facing API.
- `src/config/app_configs.cpp` contains the concrete startup/runtime config objects used by `main.cpp`.
- `src/main.cpp` stays focused on flow instead of inline config construction.

## Build

### PowerShell (works with or without CMake)

```powershell
.\build.ps1 -Run
```

`build.ps1` uses CMake when available, and otherwise falls back to `clang++`.

### Manual CMake

```bash
cmake -S . -B build
cmake --build build
./build/engine_config_example
```
