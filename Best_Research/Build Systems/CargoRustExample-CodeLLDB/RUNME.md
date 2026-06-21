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
