# Assembly Subroutines in C++ with xmake

A dependency-free C++23 example that calls hand-written assembly subroutines. xmake selects the correct assembly source for the configured target, and the project includes a CodeLLDB setup, project-local runtime configuration, and folded-stack profiling output.

## Project structure

```
├── asm/
│   ├── x86_64_windows/add.asm   # MASM -- Windows, Microsoft x64 ABI
│   ├── x86_64/add.S             # GAS  -- Linux/macOS, System V AMD64 ABI
│   └── aarch64/add.S            # GAS  -- Linux/macOS, AAPCS64
├── include/
│   ├── assembly.hpp              # Type-safe C++ wrapper declaration
│   ├── env_config.hpp            # Small .env reader
│   ├── logger.hpp                # Dependency-free console logging
│   └── tracing.hpp               # Folded-stack profiling scopes
├── src/
│   ├── assembly.cpp              # extern "C" ABI boundary
│   ├── main.cpp                  # Application entry point
│   └── tracing.cpp               # Flamegraph recorder
├── .vscode/                      # CodeLLDB and xmake tasks
└── xmake.lua                     # Target-aware build configuration
```

## Target-aware assembly selection

`xmake.lua` chooses one source file at configuration time:

| xmake target | Assembly source | ABI |
| --- | --- | --- |
| `windows/x64` | `asm/x86_64_windows/add.asm` | Microsoft x64: `rcx`, `rdx`, `rax` |
| non-Windows `x86_64` | `asm/x86_64/add.S` | System V: `rdi`, `rsi`, `rax` |
| `arm64` | `asm/aarch64/add.S` | AAPCS64: `x0`, `x1`, `x0` |

The C++ wrapper declares the symbol as `extern "C"`, preventing C++ name mangling at the assembly boundary. The non-Windows sources use the `.S` extension so the compiler preprocesses them and emits the underscore-prefixed symbol required by macOS.

## Prerequisites

- [xmake](https://xmake.io/)
- A native C++ toolchain
  - Windows: Visual Studio Build Tools with the C++ workload and MASM, or a Visual Studio installation
  - Linux: GCC or Clang with the GNU assembler
  - macOS: Xcode Command Line Tools
- VS Code plus CodeLLDB for debugger integration (optional)
- [just](https://github.com/casey/just) for the optional recipe shortcuts
- Python and `inferno-flamegraph` only when rendering profiling output

## Build and run

```sh
xmake f -m debug
xmake build
xmake run assembly_subroutines
```

The included `justfile` provides the equivalent shortcuts on Windows (it uses PowerShell 7):

```sh
just run             # configure, build, and run the debug target
just release          # configure and build the release target
just flamegraph       # run and render a flamegraph
```

The program prints:

```
7 + 35 = 42
```

For an optimized binary that preserves source-level symbols:

```sh
xmake f -m release
xmake build
xmake run assembly_subroutines
```

## Debug in VS Code

Run the one-time workspace setup:

```sh
py scripts/bootstrap.py       # Windows
python3 scripts/bootstrap.py  # Linux/macOS
```

Before opening source files with clangd, generate the project-root compilation database once:

```sh
just configure
```

This writes an ignored `compile_commands.json` containing the selected MSVC/Clang/GCC include paths. The VS Code debug task refreshes it automatically on every F5 launch.

Then open this folder in VS Code, choose **Debug C++ (CodeLLDB)**, and press F5. Its pre-launch task configures an xmake debug build before starting the executable.

The provided launch paths cover Windows x64, Linux x86-64, and macOS arm64. If you configure another target architecture, update the corresponding `program` path in `.vscode/launch.json` to match xmake's `build/<platform>/<architecture>/debug` output directory.

## Runtime logging and flamegraphs

`.env` contains project-owned configuration. `APP_LOG=warn` is the default, while `APP_LOG=info` exposes the assembly-call diagnostics. The file deliberately takes precedence over shell environment variables so command-line and debugger runs agree.

Each `TraceSpan` writes folded-stack timing data to `build/tracing.folded`. To render it, install the renderer once and run either helper:

```sh
cargo install inferno
xmake run assembly_subroutines
python scripts/render_flamegraph.py
python scripts/render_flamechart.py
```

## Cross compilation

For example, with an AArch64 GNU toolchain installed:

```sh
xmake f -p cross -a arm64 --cross=aarch64-linux-gnu- -m release
xmake build
```

xmake selects `asm/aarch64/add.S` from the configured architecture.

## Add a subroutine

1. Add an assembly implementation for each supported ABI under `asm/`.
2. Declare the ABI-stable symbol with `extern "C"` in a C++ source file.
3. Expose a typed C++ wrapper in `include/`.
4. Update `xmake.lua` if the new files require a different target selection rule.
