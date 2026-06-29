local aztro_physics_dir = os.scriptdir()
local workspace_dir = path.join(aztro_physics_dir, "..")
local is_standalone = path.absolute(os.projectdir()) == path.absolute(aztro_physics_dir)

if is_standalone then
    set_project("aztro_physics")
    set_version("0.1.0")

    add_rules("mode.debug", "mode.release")
end

target("aztro_physics")
    set_kind("static")

    set_languages("c++23")
    set_toolchains("clang")
    set_policy("build.c++.modules", true)

    add_files(path.join(aztro_physics_dir, "Physics.cppm"), { public = true })

    add_files(path.join(aztro_physics_dir, "**.cpp"), { public = true })
    remove_files(path.join(aztro_physics_dir, "tests/**.cpp"))
    remove_files(path.join(aztro_physics_dir, "**/tests.cpp"))

target("aztro_physics_tests")
    set_kind("binary")
    set_default(false)
    set_rundir(os.projectdir())

    set_languages("c++23")
    set_toolchains("clang")
    set_policy("build.c++.modules", true)

    add_files(path.join(aztro_physics_dir, "tests/main.cpp"))
    add_files(path.join(aztro_physics_dir, "**/tests.cpp"))
    add_files(path.join(workspace_dir, "Testing", "*.cpp"))
    add_includedirs(workspace_dir)
    add_deps("aztro_physics")
