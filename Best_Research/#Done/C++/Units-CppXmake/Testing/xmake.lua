set_project("Testing")
set_version("0.1.0")

set_languages("c++23")

add_rules("mode.debug", "mode.release")

local testing_dir = os.scriptdir()
local workspace_dir = path.join(testing_dir, "..")

local warning_flags = function()
    add_cxxflags("/W4", "/permissive-", {tools = "cl"})
    add_cxxflags("-Wall", "-Wextra", "-Wpedantic", {tools = {"clang", "gcc"}})
end

target("Testing")
    set_kind("static")
    add_files(path.join(testing_dir, "**.cpp"))
    add_includedirs(workspace_dir, {public = true})
    warning_flags()
