# RUNME

## Prerequisites

- premake5
- A backend tool
  - Windows: Visual Studio (recommended)
  - Linux/macOS: GNU Make
- Native C toolchain

## Generate + Debug Build + Run (Windows/Visual Studio)

```powershell
cd PremakeCExample
premake5 vs2022
msbuild .\assembly_subroutines_c_premake.sln /p:Configuration=Debug
.\build\bin\Debug\assembly_subroutines_c_premake.exe
```

## One-Command Terminal Run (Windows)

```powershell
cd PremakeCExample
.\run.ps1
```

## Generate + Release Build + Run (Windows/Visual Studio)

```powershell
cd PremakeCExample
premake5 vs2022
msbuild .\assembly_subroutines_c_premake.sln /p:Configuration=Release
.\build\bin\Release\assembly_subroutines_c_premake.exe
```

Or:

```powershell
cd PremakeCExample
.\run.ps1 -Configuration Release
```

## Linux/macOS (GNU Make)

```sh
cd PremakeCExample
premake5 gmake2
make config=debug
./build/bin/Debug/assembly_subroutines_c_premake
make config=release
./build/bin/Release/assembly_subroutines_c_premake
```
