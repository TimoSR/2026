target("aztro_physics")
    set_kind("static")

    set_languages("c++23")
    set_toolchains("clang")
    set_policy("build.c++.modules", true)

    add_files("Physics.cppm", { public = true })

    add_files("**.cpp")
    remove_files("tests/**.cpp")
    remove_files("**/tests.cpp")

target("aztro_physicsSmoke")
    set_kind("binary")
    set_default(false)
    set_rundir(os.projectdir())

    set_languages("c++23")
    set_toolchains("clang")

    add_files("tests/main.cpp")
    add_deps("aztro_physics")
