# Assembly Subroutines in Rust

A minimal example of calling hand-written assembly subroutines from Rust, with per-architecture assembly files selected at build time.

## Project Structure

```
├── asm/
│   ├── x86_64_windows/add.asm   # MASM — Windows, Microsoft x64 ABI
│   ├── x86_64/add.s             # GAS  — Linux/macOS, System V AMD64 ABI
│   └── aarch64/add.s            # GAS  — ARM64, AAPCS64
├── src/
│   └── main.rs                  # Rust caller
├── build.rs                     # Selects and assembles the right file
└── Cargo.toml
```

## How It Works

### 1. `build.rs` — architecture selection

Before the Rust crate compiles, Cargo runs `build.rs`. It reads three environment variables Cargo provides:

| Variable                  | Example values              |
|---------------------------|-----------------------------|
| `CARGO_CFG_TARGET_ARCH`   | `x86_64`, `aarch64`        |
| `CARGO_CFG_TARGET_OS`     | `windows`, `linux`, `macos` |
| `CARGO_CFG_TARGET_ENV`    | `msvc`, `gnu`, `musl`       |

Based on these, it picks the matching assembly file and passes it to the `cc` crate, which invokes the platform assembler:

- `.asm` → `ml64.exe` (MASM, Windows MSVC)
- `.s` → C compiler frontend (GAS via GCC or Clang)

The assembled object is bundled into a static library and linked into the binary automatically.

### 2. Calling conventions

Each architecture has its own register-based calling convention for passing arguments and returning values.

| Target                  | Arg 1 | Arg 2 | Return |
|-------------------------|-------|-------|--------|
| x86-64 Windows (MSVC)   | rcx   | rdx   | rax    |
| x86-64 Linux/macOS      | rdi   | rsi   | rax    |
| AArch64                 | x0    | x1    | x0     |

### 3. Linking into Rust

`main.rs` declares the symbol with `extern "C"` — Rust only needs the function signature. The linker resolves it from the assembled static library:

```rust
unsafe extern "C" {
    fn asm_add(a: i64, b: i64) -> i64;
}
```

## Build & Run

```sh
cargo run            # debug
cargo build --release  # optimized
```

### Cross-compile

`build.rs` picks the correct assembly file automatically based on the target triple:

```sh
cargo build --target aarch64-unknown-linux-gnu
```

## Runtime tracing

The example uses the maintained [`tracing`](https://github.com/tokio-rs/tracing)
ecosystem for structured application diagnostics. Normal runs emit `info`-level
events to standard error:

```sh
cargo run
```

Console logging defaults to `warn`, configured once in the project `.env` file.
The program loads that file before initializing tracing, overriding any
`RUST_LOG` inherited from a shell or debugger. The flamegraph still records
spans at that level because its layer has a separate filter.

### Flamegraphs

The `tracing-flame` layer records entered spans to `target/tracing.folded`.
Install the renderer once, run the application, then use Python to create an
SVG flamegraph:

```sh
cargo install inferno
cargo run
python scripts/render_flamegraph.py
```

To preserve span order in a flamechart instead of aggregating identical stacks:

```sh
python scripts/render_flamechart.py
```

## Adding a New Subroutine

1. Add an assembly file for each target in `asm/<arch>/`
2. Declare the function signature in `main.rs` with `unsafe extern "C"`
3. Update `build.rs` to pass the new file to `cc::Build`
