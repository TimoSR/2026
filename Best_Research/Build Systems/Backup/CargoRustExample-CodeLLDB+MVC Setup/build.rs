fn main() {
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let os   = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let env  = std::env::var("CARGO_CFG_TARGET_ENV").unwrap();

    let asm_file = match (arch.as_str(), os.as_str(), env.as_str()) {
        ("x86_64", "windows", "msvc") => "asm/x86_64_windows/add.asm",
        ("x86_64",  _,         _    ) => "asm/x86_64/add.s",
        ("aarch64", _,         _    ) => "asm/aarch64/add.s",
        _ => panic!("unsupported target: {arch} on {os}"),
    };

    let mut build = cc::Build::new();
    build.file(asm_file);

    // MASM needs CodeView line records for a debugger to map instructions to add.asm.
    if os == "windows" && env == "msvc" {
        build.flag("/Zi");
    }

    build.compile("asm_add");

    println!("cargo:rerun-if-changed={asm_file}");
}
