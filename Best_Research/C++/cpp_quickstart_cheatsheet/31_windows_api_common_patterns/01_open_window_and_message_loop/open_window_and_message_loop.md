# open_window_and_message_loop.cpp

## InDepth: Real Flow, No Text-List Helpers

This example now focuses only on real Win32 calls:

- `RegisterClassW`
- `CreateWindowExW`
- `ShowWindow` / `UpdateWindow`
- `PeekMessageW` / `TranslateMessage` / `DispatchMessageW`
- `DestroyWindow` / `UnregisterClassW`

The message loop exists because Win32 is event-driven: the OS delivers `WM_*` messages and your window procedure reacts to them.
