# Library Distribution Practical Chapter

This chapter is execution-first: pick a release mode, run the steps, publish artifacts.

All examples assume CMake and are runnable from a project root.

## 1. Pick a Release Mode Fast

Use this default decision flow:

- need fastest adoption and smallest setup: `header-only`
- need hidden implementation with simple linking: `static library`
- need runtime swap/patch without relinking app: `shared library`
- consumer must build from source in their own toolchain: `source release`
- company must hide source and control usage: `binary SDK release`
- many teams/projects must install by version: `package manager release`

## 2. Common Release Baseline (Do This For Every Mode)

1. Version your library (`major.minor.patch`).
2. Freeze public API for the release.
3. Build and test `Debug` + `Release`.
4. Create install layout (`include/`, `lib/`, `bin/`, `cmake/`, `licenses/`).
5. Generate changelog entry.
6. Tag release in git.

Recommended baseline commands:

```powershell
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build --config Release
ctest --test-dir build -C Release --output-on-failure
cmake --install build --prefix out/install
```

## 3. Mode A: Source Release (Header + Source)

Use when:
- consumers want full source visibility
- consumers use custom flags/toolchains

Ship:
- `include/`
- `src/`
- `CMakeLists.txt`
- docs + license

Producer steps:

1. Keep clean public headers under `include/<libname>/`.
2. Keep implementation in `src/`.
3. Export one target from CMake.

CMake shape:

```cmake
add_library(my_lib src/my_lib.cpp)
target_include_directories(my_lib
    PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
        $<INSTALL_INTERFACE:include>
)
install(TARGETS my_lib)
install(DIRECTORY include/ DESTINATION include)
```

Consumer use:

```cmake
add_subdirectory(external/my_lib)
target_link_libraries(app PRIVATE my_lib)
```

## 4. Mode B: Header-Only Release

Use when:
- library is small/templated
- no compiled binary is needed

Ship:
- headers only
- docs + license

Producer CMake:

```cmake
add_library(my_lib INTERFACE)
target_include_directories(my_lib
    INTERFACE
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
        $<INSTALL_INTERFACE:include>
)
install(TARGETS my_lib EXPORT my_lib_targets)
install(DIRECTORY include/ DESTINATION include)
```

Consumer use:

```cmake
target_link_libraries(app PRIVATE my_lib)
```

Risk:
- compile times can grow as headers grow.

## 5. Mode C: Static Library Release (`.lib` / `.a`)

Use when:
- you want hidden implementation
- you prefer no runtime DLL deployment

Ship:
- public headers
- static library binary
- import CMake config files

Producer CMake:

```cmake
add_library(my_lib STATIC src/my_lib.cpp)
set_target_properties(my_lib PROPERTIES OUTPUT_NAME "my_lib")
install(TARGETS my_lib EXPORT my_lib_targets ARCHIVE DESTINATION lib)
install(DIRECTORY include/ DESTINATION include)
```

Publish checklist:

1. Build per target compiler/architecture.
2. Publish one artifact per matrix entry.
3. Document CRT/runtime requirements.

## 6. Mode D: Shared Library Release (`.dll/.so/.dylib`)

Use when:
- you want smaller consumer binaries
- you need independent runtime upgrades

Ship:
- public headers
- shared library
- import library on Windows (`.lib`)
- runtime deployment docs

Producer CMake:

```cmake
add_library(my_lib SHARED src/my_lib.cpp)
install(TARGETS my_lib EXPORT my_lib_targets
    RUNTIME DESTINATION bin
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
)
install(DIRECTORY include/ DESTINATION include)
```

Windows symbol export pattern:

```cpp
#ifdef _WIN32
  #ifdef MY_LIB_BUILD
    #define MY_LIB_API __declspec(dllexport)
  #else
    #define MY_LIB_API __declspec(dllimport)
  #endif
#else
  #define MY_LIB_API
#endif
```

Critical release step:
- verify app launch with only shipped runtime files.

## 7. Mode E: Binary SDK Release (Hide Implementation)

Use when:
- company wants private implementation
- partners should consume API only

Ship:
- `include/` public API only
- `lib/` and optionally `bin/`
- integration guide
- version compatibility matrix

Hard requirements:

1. Do not leak private headers.
2. Use stable ABI strategy (or pin exact compiler/toolchain).
3. Keep exported API narrow.

Practical ABI safety:
- prefer PImpl for classes in public headers
- avoid exposing STL types across DLL boundaries unless policy is explicit

## 8. Mode F: Package Manager Release

### vcpkg

Use when:
- C++ users consume with vcpkg toolchain

Producer steps:

1. Ensure install layout is correct (`cmake --install`).
2. Create vcpkg port files (`portfile.cmake`, `vcpkg.json`).
3. Test install from a clean consumer app.

Consumer install:

```powershell
vcpkg install my-lib
```

### Conan

Use when:
- teams use Conan profiles/remote repos

Producer steps:

1. Write `conanfile.py` with settings/options.
2. Build + package with profile matrix.
3. Upload to remote.

Publish example:

```powershell
conan create . --version=1.2.0
conan upload my-lib/1.2.0 -r=myremote --all --confirm
```

## 9. Open Source vs Internal Company Release

### Open Source Release

Ship:
- source
- tests or test subset
- examples
- docs
- license

Practical flow:

1. run CI matrix
2. tag release
3. publish source + binaries on release page
4. update package manager metadata

### Internal Company Release

Ship:
- SDK (headers + binaries)
- compatibility table
- migration notes
- support policy

Practical flow:

1. publish to internal artifact repo
2. pin version in consuming repos
3. enforce deprecation windows
4. add rollback plan

## 10. Practical Artifact Layout (Recommended)

```text
my_lib_release/
  include/
    my_lib/*.h
  lib/
    my_lib.lib (or .a/.so/.dylib)
  bin/
    my_lib.dll (if shared)
  cmake/
    my_libConfig.cmake
    my_libTargets.cmake
  docs/
    QUICKSTART.md
    CHANGELOG.md
  licenses/
    LICENSE
```

## 11. Release Pipeline Template

Use this exact pipeline per release mode:

1. `configure`
2. `build`
3. `test`
4. `install-staging`
5. `package`
6. `verify-consumer-project`
7. `publish`
8. `tag`

Verification rule:
- always test consumption from a fresh sample app, not only from your main repo build.

## 12. Common Failures and Fixes

Failure: consumer cannot find headers.
- fix: wrong `target_include_directories` install interface.

Failure: shared lib works on dev machine but not consumer machine.
- fix: missing runtime DLL deployment and search path docs.

Failure: binary breaks after minor update.
- fix: ABI break; bump major version or restore compatibility.

Failure: package manager installs but link fails.
- fix: exported target names or transitive dependencies not declared.

## 13. Minimum Release Checklist

Before each release, confirm:

- tests pass in clean environment
- install tree is complete
- quickstart works in clean consumer project
- version + changelog updated
- artifact names include version and target info
- rollback plan exists
