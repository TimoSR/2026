set_project("aztro_physics")
set_version("0.1.0")

set_languages("c++23")

add_rules("mode.debug", "mode.release")
set_policy("build.c++.modules", true)

local aztro_physics_dir = os.scriptdir()

local warning_flags = function()
    add_cxxflags("/W4", "/permissive-", {tools = "cl"})
    add_cxxflags("-Wall", "-Wextra", "-Wpedantic", {tools = {"clang", "gcc"}})
end

target("aztro_physics")
    set_kind("static")
    set_toolchains("clang-cl")
    add_files(path.join(aztro_physics_dir, "physics.cppm"), {public = true})
    add_files(path.join(aztro_physics_dir, "**.cpp"))
    remove_files(path.join(aztro_physics_dir, "tests/**.cpp"))
    remove_files(path.join(aztro_physics_dir, "**/tests.cpp"))
    warning_flags()

target("AZTROPhysicsSmoke")
    set_kind("binary")
    set_default(false)
    set_rundir(os.projectdir())
    set_toolchains("clang-cl")
    add_files(path.join(aztro_physics_dir, "tests/main.cpp"))
    add_deps("aztro_physics")
    warning_flags()
