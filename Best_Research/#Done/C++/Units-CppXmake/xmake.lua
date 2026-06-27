set_project("units_cpp")
set_version("0.1.0")

set_languages("c++23")

add_rules("mode.debug", "mode.release")

local warning_flags = function()
    add_cxxflags("/W4", "/permissive-", {tools = "cl"})
    add_cxxflags("-Wall", "-Wextra", "-Wpedantic", {tools = {"clang", "gcc"}})
end

target("units")
    set_kind("binary")
    set_rundir(os.projectdir())
    add_files("src/main.cpp")
    add_includedirs("include")
    add_deps("units_core")
    warning_flags()

target("units_core")
    set_kind("static")
    add_files("src/units/**.cpp")
    add_includedirs("include", {public = true})
    warning_flags()

target("units_tests")
    set_kind("binary")
    set_rundir(os.projectdir())
    add_files("tests/test_units.cpp")
    add_includedirs("include")
    add_deps("units_core")
    warning_flags()
