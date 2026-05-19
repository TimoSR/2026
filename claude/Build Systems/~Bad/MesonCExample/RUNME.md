# RUNME

## Prerequisites

- Meson
- Ninja
- Native compiler/assembler toolchain
  - Windows: Visual Studio Build Tools (MSVC + MASM `ml64`)
  - Linux/macOS: GCC or Clang

## Debug Build + Run

```sh
cd MesonCExample
meson setup build
meson compile -C build
meson test -C build
./build/assembly_subroutines_c_meson
```

Windows run path:

```powershell
.\build\assembly_subroutines_c_meson.exe
```

## Release Build + Run

```sh
cd MesonCExample
meson setup build-release --buildtype=release
meson compile -C build-release
./build-release/assembly_subroutines_c_meson
```

Windows run path:

```powershell
.\build-release\assembly_subroutines_c_meson.exe
```
