# win32_vs_winrt_mental_model.cpp

This file now includes real runtime checks:

- Win32 call: `GetCurrentProcessId`
- WinRT runtime probes: `RoInitialize` / `RoUninitialize` (loaded from `combase.dll`)

The code shows how to verify runtime capability first, then choose which API layer to use.

`chooseWindowsApiLayer(...)` models a practical policy:
- `win32` for low-level control
- `winrt` for projection-heavy APIs
- `hybrid` when both concerns are present
