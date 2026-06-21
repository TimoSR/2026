# xmake consuming a CMake dependency

This example builds `cmake_math` with CMake, installs it under the local
`build/cmake_install` prefix, then links it into the xmake-owned application.

## Prerequisites

- xmake
- CMake 3.20+
- A native C++ compiler
- xmake package access (required only for the optional Vulkan project)

## Debug build and run

```powershell
cd C:\Users\timot\code\2026\Best_Research\Build Systems\XmakeCmakeDependencyExample
xmake f -m debug --toolchain=clang
xmake
xmake project -k compile_commands --lsp=cpptools
xmake run xmake_cmake_consumer
```

Expected output:

```text
CMake dependency: 7 + 35 = 42
```

## Release build and run

```powershell
xmake f -m release --toolchain=clang
xmake -r
xmake run xmake_cmake_consumer
```

## How it works

The `cmake_math` phony target runs CMake's configure, build, and install
steps. `xmake_cmake_consumer` declares `add_deps("cmake_math")`, so xmake
builds the CMake project first, then consumes its installed headers and static
library from `build/cmake_install`.

The CMake source header directory is also an xmake include directory. This
keeps editor diagnostics valid before the first build; compile the project with
`xmake` rather than invoking the C compiler directly so the CMake library is
built and linked.

For another CMake project, replace `cmake_math`, then update the include path,
library name, and any required CMake configuration options in `xmake.lua`.

## VS Code and Clang

The `.vscode` folder configures Clang for both the Microsoft C/C++ extension
and clangd. Run the **xmake: build debug (Clang)** task to configure xmake,
generate `compile_commands.json`, and build the CMake dependency with the same
Clang compiler.

For breakpoint debugging, install the Microsoft C/C++ extension, select
**Debug: xmake CMake consumer (Clang)** from the Run and Debug panel, and press
`F5`. The launch profile first runs the matching xmake build task, launches in
the integrated terminal, and uses the Windows debugger with symbols enabled.

Use **Debug: xmake Vulkan demo (Clang)** to build and debug the optional Vulkan
target. The debug target settings explicitly retain symbols and disable
optimization for reliable stepping and local-variable inspection.

## Optional Vulkan and C++ module project

The `vulkan_demo` directory is a CMake C++23 project. It builds an internal
`vulkan_demo.context` module and dynamically queries the Windows Vulkan loader,
so it does not need to link against a locally installed Vulkan SDK.

On its first configure, xmake downloads the pinned `vulkan-headers 1.4.335`
package from xmake-repo, then passes the resolved header path to CMake. CMake
does not fetch dependencies itself.

```powershell
xmake f -y -m debug --toolchain=clang --with_vulkan_demo=y
xmake -r xmake_vulkan_consumer
xmake run xmake_vulkan_consumer
```

The matching VS Code tasks are **xmake: build Vulkan demo (Clang)** and
**xmake: run Vulkan demo (Clang)**.
