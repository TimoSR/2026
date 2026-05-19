# RUNME

## Prerequisites

- xmake
- Native C toolchain
  - Windows: MSVC Build Tools (+ MASM)
  - Linux/macOS: GCC or Clang

## Debug Build + Run

```sh
cd XmakeCExample
xmake f -m debug
xmake
xmake run assembly_subroutines_c_xmake
```

## Release Build + Run

```sh
cd XmakeCExample
xmake f -m release
xmake -r
xmake run assembly_subroutines_c_xmake
```
