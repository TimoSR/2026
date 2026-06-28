set_project("Physics")
set_version("0.1.0")

set_languages("c++23")

add_rules("mode.debug", "mode.release")

local warning_flags = function()
    add_cxxflags("/W4", "/permissive-", {tools = "cl"})
    add_cxxflags("-Wall", "-Wextra", "-Wpedantic", {tools = {"clang", "gcc"}})
end

target("PhysicsDemo")
    set_kind("binary")
    set_default(true)
    set_rundir(os.projectdir())
    add_files("src/main.cpp")
    add_includedirs(".")
    add_deps("Physics")
    warning_flags()

target("Physics")
    set_kind("static")
    add_files("Physics/**.cpp")
    add_includedirs(".", {public = true})
    warning_flags()

target("PhysicsTests")
    set_kind("binary")
    set_default(false)
    set_rundir(os.projectdir())
    add_files("tests/test_physics.cpp")
    add_includedirs(".")
    add_deps("Physics")
    warning_flags()
