set_project("PhysicsDemo")
set_version("0.1.0")

set_languages("c++23")

add_rules("mode.debug", "mode.release")

includes("Physics")
includes("aztro_physics")

local warning_flags = function()
    add_cxxflags("/W4", "/permissive-", {tools = "cl"})
    add_cxxflags("-Wall", "-Wextra", "-Wpedantic", {tools = {"clang", "gcc"}})
end

target("PhysicsDemo")
    set_kind("binary")
    set_default(true)
    set_rundir(os.projectdir())
    set_toolchains("clang-cl")
    add_files("src/main.cpp")
    add_includedirs(".")
    add_deps("Physics", "AZTROPhysics")
    warning_flags()
