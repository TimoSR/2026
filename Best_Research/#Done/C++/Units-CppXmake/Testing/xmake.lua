local testing_dir = os.scriptdir()
local workspace_dir = path.join(testing_dir, "..")

target("Testing")
    set_kind("static")
    set_default(false)
    set_languages("c++23")

    add_files(path.join(testing_dir, "*.cpp"))
    add_includedirs(workspace_dir, { public = true })
