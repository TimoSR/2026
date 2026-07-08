# RUNME

## Prerequisites

- CMake 3.20+
- C++ compiler + assembler
  - Windows: Visual Studio Build Tools (MSVC + MASM)
  - Linux/macOS: GCC or Clang

## Debug Build + Run

```sh
cd CppExample
cmake -S . -B build
cmake --build build
```

Run binary:

- Windows:

```sh
.\build\Debug\assembly_subroutines_cpp.exe
```

- Linux/macOS (single-config generators):

```sh
./build/assembly_subroutines_cpp
```

## Release Build

- Multi-config generators (Visual Studio):

```sh
cd CppExample
cmake -S . -B build
cmake --build build --config Release
```

- Single-config generators (Ninja/Unix Makefiles):

```sh
cd CppExample
cmake -S . -B build-release -DCMAKE_BUILD_TYPE=Release
cmake --build build-release
```

Run release binary:

- Windows:

```sh
.\build\Release\assembly_subroutines_cpp.exe
```

- Linux/macOS:

```sh
./build-release/assembly_subroutines_cpp
```
