set_project("physics")
set_version("0.1.0")

set_languages("c++23")

add_rules("mode.debug", "mode.release")

local physics_dir = os.scriptdir()
local workspace_dir = path.join(physics_dir, "..")

target("physics")
    set_kind("static")
    add_files(path.join(physics_dir, "**.cpp"))
    remove_files(path.join(physics_dir, "tests/**.cpp"))
    remove_files(path.join(physics_dir, "**/tests.cpp"))
    add_includedirs(workspace_dir, {public = true})

if path.absolute(os.projectdir()) == path.absolute(physics_dir) then
    includes(path.join(workspace_dir, "Testing"))

    target("PhysicsTests")
        set_kind("binary")
        set_default(false)
        set_rundir(os.projectdir())
        add_files(path.join(physics_dir, "tests/main.cpp"))
        add_files(path.join(physics_dir, "**/tests.cpp"))
        add_includedirs(workspace_dir)
        add_deps("physics", "Testing")
        warning_flags()
end
