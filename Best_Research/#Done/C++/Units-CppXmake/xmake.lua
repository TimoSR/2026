set_project("physics_demo")
set_version("0.1.0")

add_rules("mode.debug", "mode.release")

set_languages("c++23")
set_toolchains("clang")
set_policy("build.c++.modules", true)

local project_dir = os.projectdir()

includes("physics")
includes("aztro_physics")

target("physics_demo")
    set_kind("binary")
    set_default(true)
    set_rundir(project_dir)

    add_files("src/main.cpp")

    add_deps("physics")
    add_deps("aztro_physics")
