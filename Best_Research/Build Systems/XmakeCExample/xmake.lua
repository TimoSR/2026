set_project("assembly_subroutines_c_xmake")
set_version("0.1.0")
set_languages("c11")

-- xmake f -m release for release builds

target("assembly_subroutines_c_xmake")
    set_kind("binary")
    add_files("src/main.c")

    if is_plat("windows") and is_arch("x64", "x86_64") then
        add_files("asm/x86_64_windows/add.asm")
    elseif (is_plat("linux") or is_plat("macosx")) and is_arch("x64", "x86_64") then
        add_files("asm/x86_64/add.s")
    elseif is_arch("arm64", "aarch64") then
        add_files("asm/aarch64/add.s")
    end