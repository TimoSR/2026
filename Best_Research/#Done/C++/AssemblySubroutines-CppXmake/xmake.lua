set_project("assembly_subroutines_cpp")
set_version("0.1.0")

set_languages("c++23")

add_rules("mode.debug", "mode.release")

target("assembly_subroutines")
    set_kind("binary")
    set_rundir(os.projectdir())
    add_files("src/**.cpp")
    add_includedirs("include")

    on_load(function (target)
        local plat = target:plat()
        local arch = target:arch()

        if plat == "windows" and arch == "x64" then
            -- MASM uses CodeView records so CodeLLDB can map assembly instructions.
            target:add("files", "asm/x86_64_windows/add.asm")
        elseif arch == "x86_64" then
            target:add("files", "asm/x86_64/add.S")
        elseif arch == "arm64" or arch == "arm64-v8a" then
            target:add("files", "asm/aarch64/add.S")
        else
            raise("unsupported target: " .. plat .. "/" .. arch)
        end
    end)

    add_cxxflags("/W4", "/permissive-", {tools = "cl"})
    add_cxxflags("-Wall", "-Wextra", "-Wpedantic", {tools = {"clang", "gcc"}})

    -- Keep source-level debug information in both debug and release artifacts.
    set_symbols("debug")
