# Hotgame Live — Windows hot-reload game runtime

This is a Windows-first Rust prototype for live game programming.

It is **not** an MCP server. It is a direct game-dev runtime:

- `host` keeps the window, input, persistent game state, frame capture ring, and debug timeline alive.
- `game_plugin` is a Rust `cdylib` that is rebuilt and hot-swapped while the host is running.
- `hot_api` is the stable ABI shared between the host and reloadable plugin.

## Quick start on Windows PowerShell

From the project root:

```powershell
.\scripts\doctor.ps1
.\scripts\setup.ps1
.\scripts\run.ps1
```

Fast path after setup:

```powershell
.\dev.ps1
```

Or double-click `dev.bat`.

Then edit and save:

```text
crates\game_plugin\src\lib.rs
```

The running window detects the change, rebuilds `game_plugin.dll`, copies it to `.hot\`, reloads it, and continues with the same host-owned `GameState`.

## Controls

| Key | Action |
|---|---|
| `Esc` | Exit |
| `Space` | Pause/resume simulation |
| `Right Arrow` | When paused, execute exactly one new frame using the currently loaded plugin |
| `Left Arrow` | When paused, scrub backward through captured frames |
| `Enter` | Jump back to latest captured frame |
| `F5` | Force rebuild/reload plugin |

## What to edit first

Open:

```text
crates\game_plugin\src\lib.rs
```

Try changing:

```rust
let speed = 260.0;
```

or the color values in the `ctx.emit_rect(...)` calls. Save the file while the host window is running.


## Article-informed design

This project follows the same low-level model as Robert Krahn's hot-reloading Rust write-up: keep a host executable running, rebuild a dynamic library, copy the DLL before loading it on Windows, keep signatures stable, and keep durable state outside the reloadable library.

It does **not** use `hot-lib-reloader`, Rhai, Bevy, cargo-watch, or macro-generated reload wrappers. The loader is explicit so the game runtime can control build failures, frame stepping, debug events, state ownership, and future bytecode/source-level debugging.

See:

```text
docs\hot-reload-design.md
```

## Debugging model

This prototype gives you a frame-step debugger at the runtime level:

- persistent state lives in the host
- each simulated frame records input, state snapshot, draw commands, and debug events
- pause the simulation
- edit plugin code
- reload it
- step one frame at a time
- inspect debug events in the console/title

Statement-level stepping through custom source code comes later when the reloadable side becomes a VM/bytecode language. This project gives you the correct host/plugin foundation first.

## Project layout

```text
crates/
  hot_api/       Stable C ABI and shared types
  game_plugin/   Hot-swappable gameplay/render code
  host/          Window, input, plugin loader, build watcher, frame debugger
scripts/
  doctor.ps1     Check Rust/Cargo, project shape, and workspace build
  setup.ps1      Build everything once
  run.ps1        Run the host
  build.ps1      Build all crates
  clean-hot.ps1  Remove copied hot DLLs/logs
```

## Notes

On Windows, loaded DLLs are usually locked. The host never loads `target\debug\game_plugin.dll` directly. It copies each successful build to a unique file under `.hot\` and loads that copy.
