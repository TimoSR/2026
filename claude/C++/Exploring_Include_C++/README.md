# Exploring Include C++

This project demonstrates a small C++ program where implementation files are included once from `Main.cpp` instead of using headers.

## Project Files

- `Math.cpp` defines the `Math` class and its `Add` method.
- `Physics.cpp` defines `CalculateForce`.
- `Main.cpp` includes those implementation files and owns the single translation unit.
- `build-clang.ps1` builds the program with Clang.
- `compile_flags.txt` tells clangd to analyze the project as C++23.

## Unity Build Pattern

`Main.cpp` is the only file that should be compiled:

```cpp
#define UNITY_BUILD

#include "Math.cpp"
#include "Physics.cpp"
```

The included `.cpp` files are implementation fragments. If a fragment needs another fragment for clangd analysis, it can include that dependency only when it is not being pulled into the unity translation unit:

```cpp
#ifndef UNITY_BUILD
#include "Math.cpp"
#endif
```

This keeps the real build as one translation unit while preventing clangd from reporting false errors when it opens an individual implementation fragment by itself.

## Requirements

Install Clang and make sure `clang++` is available in PowerShell:

```powershell
clang++ --version
```

## Build

```powershell
clang++ -std=c++23 -Wall -Wextra -Wpedantic Main.cpp -o exploring_include.exe
```

```powershell
.\build-clang.ps1
```

Or use the included script:

```powershell
powershell -ExecutionPolicy Bypass -File .\build-clang.ps1
```

## Run

```powershell
.\exploring_include.exe
```

Expected output is:

```text
Hello C++23
8
8
10
```
