# RUNME

## Prerequisites

- Rust toolchain (`rustup`, `cargo`)
- A native toolchain for your platform
  - Windows: Visual Studio Build Tools (MSVC + assembler)
  - Linux/macOS: GCC or Clang toolchain

## Debug Build + Run

```sh
cd RustExample
cargo run
```

## Debug in VS Code (Windows, macOS, Linux)

Install the **CodeLLDB** extension, open the folder that contains `Cargo.toml`,
then choose **Debug Rust (CodeLLDB — Windows, macOS, Linux)** in Run and Debug
and press F5. The launch configuration asks Cargo for the executable, so it
uses the correct platform-specific path automatically (`.exe` on Windows and
no extension on macOS/Linux).

`Debug Rust (MSVC)` remains available as an optional Windows-only profile for
the Microsoft C++ debugger.

## Release Build

```sh
cd RustExample
cargo build --release
```

## Run Release Binary

- Windows:

```sh
.\target\release\assembly_subroutines.exe
```

- Linux/macOS:

```sh
./target/release/assembly_subroutines
```
