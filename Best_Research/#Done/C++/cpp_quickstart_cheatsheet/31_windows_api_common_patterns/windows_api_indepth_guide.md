# Windows API InDepth Guide

This guide is explanation-first.  
Use it as the primary reference, and use the code files only as runnable anchor examples.

## 1. Mental Model: Windows API Surface

Windows development is not one API. It is a stack of API families.

- Win32: broad desktop/system API surface, low-level control, long compatibility history.
- COM-based APIs: many subsystems (audio, shell, multimedia) use COM interfaces.
- WinRT: modern runtime API model with language projections and app-model-oriented capabilities.
- Driver/vendor SDKs: required for some hardware-specific telemetry and controls.

Practical takeaway:
- Choose by capability and constraints, not by ideology.
- Expect mixed usage in real apps.

## 2. Opening a Window (Win32 Thinking)

Core flow:
1. Define and register a window class.
2. Create a window instance.
3. Show/update it.
4. Run the message loop.

Why this matters:
- Windows desktop UI is event/message driven.
- Your app reacts to `WM_*` messages.
- This model remains fundamental even when layered frameworks are used.

Important messages:
- `WM_CREATE`: initialization after window creation.
- `WM_SIZE`: resize handling.
- `WM_PAINT`: drawing trigger.
- `WM_CLOSE` / `WM_DESTROY`: shutdown flow.

Design advice:
- Keep window procedure thin.
- Dispatch to your own app services/use-cases instead of placing business logic directly in message handler branches.

## 3. Filesystem and Process APIs

Common filesystem primitives:
- `CreateFileW`, `ReadFile`, `WriteFile`
- `FindFirstFileW`, `FindNextFileW`
- `GetFileAttributesExW`
- `CreateDirectoryW`

Common process primitives:
- `CreateProcessW`
- `OpenProcess`
- `WaitForSingleObject`
- `TerminateProcess` (careful usage)

Important ownership rule:
- handle-returning APIs require deterministic cleanup (`CloseHandle`).
- Wrap handles in small RAII classes in production code.

## 4. System and Device Information

Common system info APIs:
- `GetSystemInfo`
- `GlobalMemoryStatusEx`
- `GetLogicalProcessorInformationEx`
- `GetTickCount64`

Device/power/telemetry APIs often used:
- `GetSystemPowerStatus`
- power management APIs (power schemes, battery data)
- PDH / performance counters
- WMI queries

Reality check:
- not all metrics are universally available.
- some detailed hardware metrics depend on drivers/vendor tooling.

## 5. Audio APIs

Typical options:
- WASAPI (`IAudioClient`, `IAudioRenderClient`) for low-level/control-heavy audio.
- MMDevice API for endpoint discovery and selection.
- XAudio2 for engine/game-style audio abstractions.

Pattern:
- separate device enumeration from playback pipeline.
- hide COM lifetime/error details behind internal wrappers.

## 6. GPU APIs

Core graphics stack on Windows:
- D3D11 / D3D12 for rendering.
- DXGI for adapters/outputs/swap chains.

Telemetry caveat:
- adapter-level information is broadly available.
- deep temperature/power/frequency often requires vendor-specific SDKs and permissions.

## 7. Process Introspection

Typical goals:
- list running processes
- inspect memory/CPU usage
- detect parent/child processes

Operational concerns:
- permissions and integrity levels can block access.
- anti-cheat/security tooling may restrict process introspection.

## 8. Win32 vs WinRT: Why WinRT Did Not Replace Win32

Short answer:
- WinRT was not a full Win32 replacement strategy.

Why:
- Win32 had and still has broad ecosystem/compatibility requirements.
- many low-level/legacy/enterprise scenarios are Win32-first.
- WinRT addressed modern API projection and app model concerns.
- both stacks are used together in practice.

How to think about it:
- Win32: foundational desktop/system/control layer.
- WinRT: modern projected API layer for selected domains and app models.
- combine where needed.

## 9. Practical Selection Strategy (Project Level)

Use Win32 when:
- deep desktop integration is required
- low-level process/thread/handle control is required
- legacy compatibility is critical

Use WinRT when:
- projected modern APIs improve productivity
- app model/platform constraints align with WinRT capabilities
- async/data binding style integrates better with your chosen UI stack

Use both when:
- one layer gives low-level control and the other gives modern convenience.

## 10. Architecture Pattern for Maintainable Windows Integration

Recommended layering:
1. `platform adapters` (Win32/WinRT/COM/vendor SDK calls)
2. `project services` (stable app-facing interfaces)
3. `application logic` (no direct platform API calls)

Benefits:
- easier testing (mock adapters)
- easier migration (swap adapter implementation)
- reduced platform API spread across business code

## 11. Common Mistakes and Fixes

Mistake: spreading raw API calls across many modules.  
Fix: centralize in adapter services.

Mistake: leaking HANDLE/COM ownership rules into business logic.  
Fix: RAII wrappers + narrow service interfaces.

Mistake: assuming all telemetry is portable and always available.  
Fix: capability detection + graceful fallback paths.

Mistake: debating Win32 vs WinRT as mutually exclusive.  
Fix: evaluate capability by use case and combine intentionally.

## 12. Suggested Learning Sequence

1. Window + message loop basics
2. Filesystem/process/system-info wrappers
3. Audio/GPU intro layer
4. Win32 + WinRT mixed integration patterns
5. Capability probing + fallback design
