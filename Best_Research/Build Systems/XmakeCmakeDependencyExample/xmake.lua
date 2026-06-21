set_project("xmake_cmake_dependency_example")
set_version("0.1.0")
set_languages("cxx20")

option("with_vulkan_demo")
    set_default(false)
    set_showmenu(true)
    set_description("Build the C++23 Vulkan and C++ module example")
option_end()

local cmake_source_dir = path.join(os.projectdir(), "cmake_math")
local cmake_build_root = path.join(os.projectdir(), "build", "cmake_math")
local cmake_install_dir = path.join(os.projectdir(), "build", "cmake_install")

if has_config("with_vulkan_demo") then
    add_requires("vulkan-headers 1.4.335")
end

local function cmake_configuration(target, source_dir, build_root, install_dir, extra_args)
    local build_type = is_mode("debug") and "Debug" or "Release"
    local compiler = target:tool("cxx")
    if target:is_plat("windows") and not compiler:endswith(".exe") then
        compiler = compiler .. ".exe"
    end

    local cmake_build_dir = path.join(build_root, "native")
    local cmake_args = {
        "-S", source_dir,
        "-B", cmake_build_dir,
        "-DCMAKE_BUILD_TYPE=" .. build_type,
        "-DCMAKE_EXPORT_COMPILE_COMMANDS=ON",
        "-DCMAKE_INSTALL_PREFIX=" .. install_dir
    }

    if compiler:lower():find("clang", 1, true) then
        cmake_build_dir = path.join(build_root, "clang")
        cmake_args = {
            "-S", source_dir,
            "-B", cmake_build_dir,
            "-G", "Ninja",
            "-DCMAKE_CXX_COMPILER=" .. compiler,
            "-DCMAKE_BUILD_TYPE=" .. build_type,
            "-DCMAKE_EXPORT_COMPILE_COMMANDS=ON",
            "-DCMAKE_INSTALL_PREFIX=" .. install_dir
        }
    end

    for _, argument in ipairs(extra_args or {}) do
        table.insert(cmake_args, argument)
    end

    return cmake_build_dir, build_type, cmake_args
end

target("cmake_math")
    set_kind("phony")
    set_policy("build.fence", true)

    on_build(function (target)
        local build_dir, build_type, cmake_args = cmake_configuration(
            target, cmake_source_dir, cmake_build_root, cmake_install_dir)
        os.vrunv("cmake", cmake_args)
        os.vrunv("cmake", {"--build", build_dir, "--config", build_type})
        os.vrunv("cmake", {"--install", build_dir, "--config", build_type})
    end)

target("xmake_cmake_consumer")
    set_kind("binary")
    if is_mode("debug") then
        set_symbols("debug")
        set_optimize("none")
    end
    add_deps("cmake_math")
    add_files("src/main.cpp")
    -- Source headers keep editor diagnostics valid before the CMake install step.
    add_includedirs(path.join(cmake_source_dir, "include"))
    add_includedirs(path.join(cmake_install_dir, "include"))
    add_linkdirs(path.join(cmake_install_dir, "lib"))
    add_links("cmake_math")

if has_config("with_vulkan_demo") then
    local vulkan_source_dir = path.join(os.projectdir(), "vulkan_demo")
    local vulkan_install_dir = path.join(os.projectdir(), "build", "vulkan_install")

    target("vulkan_demo")
        set_kind("phony")
        set_policy("build.fence", true)
        add_packages("vulkan-headers")

        on_build(function (target)
            local vulkan_headers = assert(target:pkg("vulkan-headers"))
            local includedir = path.join(assert(vulkan_headers:installdir()), "include")
            assert(os.isfile(path.join(includedir, "vulkan", "vulkan.h")))
            local build_dir, build_type, cmake_args = cmake_configuration(
                target, vulkan_source_dir, vulkan_build_root, vulkan_install_dir,
                {"-DVULKAN_HEADERS_INCLUDE_DIR=" .. includedir})
            os.vrunv("cmake", cmake_args)
            os.vrunv("cmake", {"--build", build_dir, "--config", build_type})
            os.vrunv("cmake", {"--install", build_dir, "--config", build_type})
        end)

    target("xmake_vulkan_consumer")
        set_kind("binary")
        set_languages("cxx23")
        if is_mode("debug") then
            set_symbols("debug")
            set_optimize("none")
        end
        add_deps("vulkan_demo")
        add_files("src/vulkan_main.cpp")
        add_includedirs(path.join(vulkan_source_dir, "include"))
        add_includedirs(path.join(vulkan_install_dir, "include"))
        add_linkdirs(path.join(vulkan_install_dir, "lib"))
        add_links("vulkan_demo")
end
