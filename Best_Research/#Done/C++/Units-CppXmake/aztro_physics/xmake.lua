local aztro_physics_dir = os.scriptdir()
local workspace_dir = path.join(aztro_physics_dir, "..")

target("aztro_physics")
    set_kind("static")

    set_languages("c++23")
    set_toolchains("clang")
    set_policy("build.c++.modules", true)

    add_files(path.join(aztro_physics_dir, "Physics.cppm"), { public = true })

    add_files(path.join(aztro_physics_dir, "**.cpp"), { public = true })
    remove_files(path.join(aztro_physics_dir, "tests/**.cpp"))
    remove_files(path.join(aztro_physics_dir, "**/tests.cpp"))

target("AztroPhysicsTests")
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
