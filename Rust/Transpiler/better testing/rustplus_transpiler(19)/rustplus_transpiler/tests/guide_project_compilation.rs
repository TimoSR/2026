use rustplus::{RustPlusCompiler, RustPlusConfig, RustPlusProgram};

#[test]
fn guide_compile_one_source_document_from_memory() {
    let rust_plus_source = r#"
class Account
{
    id: String;

    pub fn new(id: String) -> Self
    {
        return Self { id };
    }
}
"#;

    let compiler = RustPlusCompiler::default();
    let generated_rust = compiler
        .compile_source_text(rust_plus_source)
        .expect("the guide example should compile");

    assert!(generated_rust.code.contains("struct Account"));
    assert!(generated_rust.code.contains("id: String,"));
}

#[test]
fn guide_compile_a_project_and_read_the_project_report() {
    let rust_plus_source = r#"
class Account
{
    id: String;

    pub fn new(id: String) -> Self
    {
        return Self { id };
    }
}
"#;

    let mut program = RustPlusProgram::from_source(rust_plus_source.to_string(), RustPlusConfig::default())
        .expect("the project should load");
    let generated_file = program.emit_file(0, None).expect("the source file should emit Rust");
    let report = program.format_report(true);

    assert!(generated_file.rust.contains("struct Account"));
    assert!(report.contains("Rust Plus project report"));
    assert!(report.contains("Known classes"));
}
