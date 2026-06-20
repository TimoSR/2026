pub mod ast;
pub mod cargo_integration;
pub mod codegen;
pub mod compiler;
pub mod config;
pub mod diagnostics;
pub mod features;
pub mod host;
pub mod incremental;
pub mod line_map;
pub mod parser;
pub mod project;
pub mod program;
pub mod scanner;
pub mod transpiler;
pub mod timing;

pub use compiler::{GeneratedRust, RustPlusCompilation, RustPlusCompiler, SourceDocument};
pub use config::{FeatureFlags, RustPlusConfig};
pub use program::{EmitOutput, RustPlusProgram, SourceFile};
pub use transpiler::{ProjectSymbols, SemanticContext, Transpiler};


#[cfg(test)]
mod guide_tests {
    use super::*;

    fn compile_guide_snippet(rust_plus_source: &str) -> String {
        return Transpiler::new(rust_plus_source.to_string()).transpile().unwrap();
    }

    fn compile_guide_snippet_with_config(rust_plus_source: &str, config: RustPlusConfig) -> anyhow::Result<String> {
        return Transpiler::with_config(rust_plus_source.to_string(), config).transpile();
    }

    fn generated_rust_should_contain(generated_rust: &str, expected_snippet: &str) {
        assert!(
            generated_rust.contains(expected_snippet),
            "expected generated Rust to contain:\n{expected_snippet}\n\nGenerated Rust was:\n{generated_rust}"
        );
    }

    #[test]
    fn guide_a_class_can_implement_multiple_interfaces_and_embed_a_base_class() {
        let rust_plus_source = r#"
interface IAccount
{
    fn balance(self) -> Money;
}

interface IPayable
{
    fn deposit(mut self, amount: Money);
}

class User
{
    name: String;
}

class UserAccount : IAccount, IPayable, User
{
    id: AccountId;
    balance: Money;

    pub fn deposit(mut this, amount: Money)
    {
        this.balance = this.balance.add(amount);
    }

    pub fn balance(this) -> Money
    {
        return this.balance;
    }
}
"#;

        let generated_rust = compile_guide_snippet(rust_plus_source);

        generated_rust_should_contain(&generated_rust, "trait IAccount");
        generated_rust_should_contain(&generated_rust, "trait IPayable");
        generated_rust_should_contain(&generated_rust, "struct UserAccount");
        generated_rust_should_contain(&generated_rust, "id: AccountId,");
        generated_rust_should_contain(&generated_rust, "balance: Money,");
        generated_rust_should_contain(&generated_rust, "user: User,");
        generated_rust_should_contain(&generated_rust, "impl IAccount for UserAccount");
        generated_rust_should_contain(&generated_rust, "impl IPayable for UserAccount");
        generated_rust_should_contain(&generated_rust, "fn deposit(mut self, amount: Money)");
        generated_rust_should_contain(&generated_rust, "self.balance = self.balance.add(amount);");
    }

    #[test]
    fn guide_disabling_this_receiver_makes_this_an_error() {
        let rust_plus_source = r#"
class UserAccount
{
    balance: Money;

    pub fn balance(this) -> Money
    {
        return this.balance;
    }
}
"#;
        let mut config = RustPlusConfig::default();
        config.features.this_receiver = false;

        let error = compile_guide_snippet_with_config(rust_plus_source, config).unwrap_err();

        assert!(error.to_string().contains("this_receiver"));
    }

    #[test]
    fn guide_disabling_multiple_bases_makes_multiple_base_lists_an_error() {
        let rust_plus_source = r#"
interface A { fn a(&self); }
interface B { fn b(&self); }
class C : A, B { fn a(&self) {} fn b(&self) {} }
"#;
        let mut config = RustPlusConfig::default();
        config.features.multiple_bases = false;

        let error = compile_guide_snippet_with_config(rust_plus_source, config).unwrap_err();

        assert!(error.to_string().contains("multiple_bases"));
    }

    #[test]
    fn guide_csharp_style_object_creation_is_not_the_standard_language() {
        let rust_plus_source = r#"
interface IAccount
{
    fn deposit(&mut self, amount: i64);
    fn balance(&self) -> i64;
}

class Account : IAccount
{
    balance: i64;

    pub fn new() -> Self
    {
        return Self
        {
            balance: 0,
        };
    }

    pub fn deposit(&mut self, amount: i64)
    {
        self.balance += amount;
    }

    pub fn balance(&self) -> i64
    {
        return self.balance;
    }
}

fn main()
{
    IAccount account = new Account();
    account.deposit(10);
}
"#;

        let error = Transpiler::new(rust_plus_source.to_string()).transpile().unwrap_err();

        assert!(error.to_string().contains("csharp_variable_declarations"));
    }

    #[test]
    fn guide_interface_variables_use_rust_let_with_heap_initializer() {
        let rust_plus_source = r#"
interface IAccount
{
    fn id(&self) -> &str;
}

class Account : IAccount
{
    id: String;

    pub fn new(id: String) -> Self
    {
        return Self { id };
    }

    pub fn id(&self) -> &str
    {
        return &self.id;
    }
}

fn main()
{
    let account: IAccount = Account::Heap("account-1");
}
"#;

        let generated_rust = compile_guide_snippet(rust_plus_source);

        generated_rust_should_contain(
            &generated_rust,
            "let account: Box<dyn IAccount> = Box::new(Account::new(\"account-1\".to_string()));",
        );
        generated_rust_should_contain(&generated_rust, "id: String,");
        assert!(!generated_rust.contains("private id"));
    }

    #[test]
    fn guide_stack_and_heap_initializers_create_owned_or_boxed_values() {
        let rust_plus_source = r#"
class Article
{
    pub headline: String;
    pub location: String;
    pub artist: String;

    pub fn new(headline: &str, location: &str, artist: &str) -> Self
    {
        return Self
        {
            headline: headline.to_string(),
            location: location.to_string(),
            artist: artist.to_string(),
        };
    }
}

fn main()
{
    let article = Article::Stack("My own new!", "Lol!", "stuff");
    let boxed_article = Article::Heap("My own new!", "Lol!", "stuff");
}
"#;

        let generated_rust = compile_guide_snippet(rust_plus_source);

        generated_rust_should_contain(
            &generated_rust,
            "let article: Article = Article::new(\"My own new!\", \"Lol!\", \"stuff\");",
        );
        generated_rust_should_contain(
            &generated_rust,
            "let boxed_article: Box<Article> = Box::new(Article::new(\"My own new!\", \"Lol!\", \"stuff\"));",
        );
        assert!(!generated_rust.contains("pub new("));
        assert!(!generated_rust.contains("pub fn Stack("));
        assert!(!generated_rust.contains("pub fn Heap("));
    }

    #[test]
    fn guide_stack_and_heap_with_arguments_require_pub_fn_new() {
        let rust_plus_source = r#"
class Article
{
    headline: String;
}

fn main()
{
    let article = Article::Stack("headline");
}
"#;

        let error = Transpiler::new(rust_plus_source.to_string()).transpile().unwrap_err();

        assert!(error.to_string().contains("matching new(...) constructor"));
    }

    #[test]
    fn guide_rustplus_compiler_facade_compiles_one_source_document() {
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
        let generated = compiler.compile_source_text(rust_plus_source).unwrap();

        generated_rust_should_contain(&generated.code, "struct Account");
        generated_rust_should_contain(&generated.code, "id: String,");
    }
}

#[cfg(test)]
mod guide_project_tests {
    use super::*;

    #[test]
    fn guide_a_program_report_explains_what_the_project_knows() {
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

        let mut program = RustPlusProgram::from_source(rust_plus_source.to_string(), RustPlusConfig::default()).unwrap();
        let output = program.emit_file(0, None).unwrap();
        let report = program.format_report(true);

        assert!(output.rust.contains("struct Account"));
        assert!(report.contains("Rust Plus project report"));
        assert!(report.contains("Known classes"));
    }
}
