# Professional Engine Workspace Example

This repository demonstrates a thin `main` and crate-based composition for a larger Rust engine/game project.

## Workspace layout

```text
crates/
  engine-core/
    src/
      context.rs
      error.rs
      event.rs
      plugin.rs
      lib.rs
  engine-runtime/
    src/
      config.rs
      runtime.rs
      lib.rs
  engine-render/
    src/
      plugin.rs
      lib.rs
  engine-audio/
    src/
      plugin.rs
      lib.rs
  game-client/
    src/
      config.rs
      app.rs
      plugins/
        gameplay.rs
        mod.rs
      lib.rs
  launcher/
    src/
      cli.rs
      main.rs
scripts/
  call-launcher.ps1
```

## Design rule shown in code

- `main` only parses CLI args, builds config, and calls `game_client::run(...)`.
- `game-client` owns composition by constructing `Runtime` and registering plugins.
- `engine-runtime` owns startup/update/shutdown lifecycle.
- subsystem crates are reusable libraries and do not depend on `main`.

## Run directly with Cargo

```powershell
cargo run -p launcher
cargo run -p launcher -- 12
```

The optional argument is `max_frames`.

## How the executable is built

Manual build of the binary artifact:

```powershell
cargo build -p launcher
cargo build -p launcher --release
```

Output artifacts:
- debug build: `target\debug\launcher.exe`
- release build: `target\release\launcher.exe`

The executable comes from the binary crate in `crates/launcher` (entrypoint: `src/main.rs`).

## Compiled executable called by another script

This demonstrates the exact pattern you asked for: build a binary artifact and call it from a separate script.

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\call-launcher.ps1 -Release -Frames 8
```

What this script does:
- compiles `launcher` (`cargo build -p launcher --release`)
- resolves `target\release\launcher.exe`
- executes the compiled `.exe` with your frame count
- returns the executable exit code

Optional machine-readable output:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\call-launcher.ps1 -Release -Frames 8 -AsJson
```
