# RUNME

## Prerequisites

- xmake
- A C++23 toolchain for the host platform
  - Windows: Visual Studio Build Tools with C++ and MASM
  - Linux/macOS: GCC or Clang toolchain

## Debug build and run

```sh
cd AssemblySubroutines-CppXmake
xmake f -m debug
xmake
xmake run assembly_subroutines
```

Or use the included recipe file:

```sh
just run
```

## Clangd includes

Generate clangd's project-root compilation database before opening C++ sources:

```sh
just configure
```

The VS Code debug task refreshes this file automatically.

## Debug in VS Code

Install the workspace extensions once:

```sh
py scripts/bootstrap.py       # Windows
python3 scripts/bootstrap.py  # Linux/macOS
```

Open the project folder, select **Debug C++ (CodeLLDB)** in Run and Debug, and press F5. The launch task configures and builds the debug target first.

## Release build

```sh
xmake f -m release
xmake
```

## Run the release binary

- Windows: `build\windows\x64\release\assembly_subroutines.exe`
- Linux: `build/linux/x86_64/release/assembly_subroutines`
- macOS (Apple Silicon): `build/macosx/arm64/release/assembly_subroutines`
