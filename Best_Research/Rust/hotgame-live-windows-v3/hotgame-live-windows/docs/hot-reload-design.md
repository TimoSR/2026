# Hot-reload design notes

This project intentionally uses the same low-level technique described in Robert Krahn's hot-reloading Rust article, but without depending on `hot-lib-reloader`, Rhai, Bevy, cargo-watch, or macro-generated wrappers.

## What we borrow

The important ideas are:

1. Keep a normal Rust executable running.
2. Put reloadable behavior in a dynamic library.
3. Export unmangled functions from the dynamic library.
4. Keep the function signatures stable.
5. Copy the freshly built DLL before loading it on Windows.
6. Keep durable state outside the reloadable library.
7. Route renderer/framework state through the host instead of touching global renderer state from the reloadable DLL.

The host therefore owns the window, renderer, input, game state, frame history, debug timeline, and plugin build/reload loop.

The plugin owns only reloadable behavior:

```text
hot_update(ctx)
  read input
  mutate host-owned GameState
  emit draw commands
  emit debug events
```

## Why not use `hot-lib-reloader` directly?

`hot-lib-reloader` is useful, but it optimizes for convenience. This project optimizes for a game runtime/debugger boundary.

We avoid it because we want direct control over:

- when reloads are allowed
- how build failures are displayed
- how Windows DLL shadow-copying works
- where persistent state lives
- how frames are captured and scrubbed
- how future bytecode/source stepping will attach to the runtime

The current dependency set is deliberately small:

```text
anyhow      error handling
libloading  dynamic library loading
minifb      simple temporary window/pixel output
```

## Stable ABI rule

The boundary between host and plugin must stay stable while the host is running.

Safe to change live:

```text
crates/game_plugin/src/lib.rs
```

Restart the host after changing:

```text
crates/hot_api/src/lib.rs
crates/host/src/main.rs
crates/game_plugin/Cargo.toml
Cargo.toml
```

Technically the project can detect some of these changes, but changing shared type layouts while a host process is running is not a valid hot-reload operation. Restart instead.

## Why `cdylib` + `extern "C"`?

The article demonstrates Rust dynamic-library hot reloading with Rust function signatures. For this project, we use a stricter boundary:

```rust
#[repr(C)]
pub struct FrameContext { /* stable fields */ }

pub type HotUpdateFn = unsafe extern "C" fn(*mut FrameContext) -> PluginStatus;
```

That is more appropriate for a long-lived game runtime because it avoids relying on Rust ABI details at the plugin boundary.

## State ownership

Do not put durable state in the plugin.

Bad:

```rust
static mut WORLD: World = World::new();
```

Good:

```rust
let state = unsafe { &mut *ctx.state };
state.player_x += state.player_vx * input.dt_seconds;
```

When the DLL reloads, plugin globals are not the state model. The host-owned `GameState` is the state model.

## Rendering model

The plugin does not own GPU or window state. It emits commands:

```rust
ctx.emit_rect(x, y, w, h, r, g, b, a);
```

The host consumes the draw list and renders it. This keeps the graphics runtime stable across reloads and makes frame capture/debugging straightforward.

## Debug model

The plugin emits structured debug events:

```rust
ctx.emit_debug_f64(FILE_ID_GAME_PLUGIN, line!(), column!(), "player_x", player_x as f64);
```

The host captures:

```text
input
state snapshot
draw commands
debug events
plugin generation
```

That gives you frame-level stepping now. Statement-level stepping comes later when the reloadable side becomes a custom bytecode VM or interpreter.
