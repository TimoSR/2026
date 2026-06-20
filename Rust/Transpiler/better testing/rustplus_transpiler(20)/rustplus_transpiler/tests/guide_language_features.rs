use rustplus::{RustPlusConfig, Transpiler};

struct GuideExample {
    rust_plus_source: &'static str,
    generated_rust: String,
}

impl GuideExample {
    fn compile(rust_plus_source: &'static str) -> Self {
        let generated_rust = Transpiler::new(rust_plus_source.to_string())
            .transpile()
            .expect("the guide example should compile");

        return Self {
            rust_plus_source,
            generated_rust,
        };
    }

    fn compile_with_config(rust_plus_source: &'static str, config: RustPlusConfig) -> anyhow::Result<String> {
        return Transpiler::with_config(rust_plus_source.to_string(), config).transpile();
    }

    fn should_generate(self, expected_rust_snippet: &str) -> Self {
        assert!(
            self.generated_rust.contains(expected_rust_snippet),
            "Guide source:\n{}\n\nExpected generated Rust to contain:\n{}\n\nGenerated Rust:\n{}",
            self.rust_plus_source,
            expected_rust_snippet,
            self.generated_rust,
        );

        return self;
    }

    fn should_not_generate(self, unexpected_rust_snippet: &str) -> Self {
        assert!(
            !self.generated_rust.contains(unexpected_rust_snippet),
            "Guide source:\n{}\n\nDid not expect generated Rust to contain:\n{}\n\nGenerated Rust:\n{}",
            self.rust_plus_source,
            unexpected_rust_snippet,
            self.generated_rust,
        );

        return self;
    }
}

#[test]
fn guide_write_a_class_and_get_a_struct_with_private_fields_by_default() {
    GuideExample::compile(
        r#"
class Account
{
    id: String;
    pub display_name: String;
}
"#,
    )
    .should_generate("struct Account")
    .should_generate("    id: String,")
    .should_generate("    pub display_name: String,")
    .should_not_generate("private id");
}

#[test]
fn guide_write_an_interface_and_implement_it_with_a_class() {
    GuideExample::compile(
        r#"
interface IAccount
{
    fn balance(&self) -> i64;
}

class Account : IAccount
{
    balance: i64;

    pub fn balance(&self) -> i64
    {
        return self.balance;
    }
}
"#,
    )
    .should_generate("trait IAccount")
    .should_generate("struct Account")
    .should_generate("impl IAccount for Account")
    .should_generate("    fn balance(&self) -> i64")
    .should_not_generate(r#"impl IAccount for Account
{
    pub fn balance(&self) -> i64"#);
}

#[test]
fn guide_create_a_stack_value_with_stack_initializer() {
    GuideExample::compile(
        r#"
class Article
{
    pub headline: String;

    pub fn new(headline: &str) -> Self
    {
        return Self
        {
            headline: headline.to_string(),
        };
    }
}

fn main()
{
    let article = Article::Stack("Rust Plus");
}
"#,
    )
    .should_generate("let article: Article = Article::new(\"Rust Plus\");")
    .should_not_generate("pub fn Stack(");
}

#[test]
fn guide_create_a_boxed_trait_object_with_heap_initializer() {
    GuideExample::compile(
        r#"
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
"#,
    )
    .should_generate("let account: Box<dyn IAccount> = Box::new(Account::new(\"account-1\".to_string()));");
}

#[test]
fn guide_keep_rust_attributes_without_teaching_the_transpiler_each_attribute() {
    GuideExample::compile(
        r#"
#[derive(Debug, Clone, PartialEq, Eq)]
pub class Article
{
    #[doc = "A headline shown in the UI."]
    pub headline: String;

    #[inline]
    pub fn new(headline: &str) -> Self
    {
        return Self
        {
            headline: headline.to_string(),
        };
    }
}
"#,
    )
    .should_generate("#[derive(Debug, Clone, PartialEq, Eq)]\npub struct Article")
    .should_generate("    #[doc = \"A headline shown in the UI.\"]\n    pub headline: String,")
    .should_generate("    #[inline]\n    pub fn new(headline: &str) -> Self");
}

#[test]
fn guide_csharp_style_new_object_syntax_is_not_the_default_language() {
    let error = GuideExample::compile_with_config(
        r#"
interface IAccount
{
    fn id(&self) -> &str;
}

class Account : IAccount
{
    pub fn id(&self) -> &str
    {
        return "account";
    }
}

fn main()
{
    IAccount account = new Account();
}
"#,
        RustPlusConfig::default(),
    )
    .expect_err("C# style declarations are legacy syntax and disabled by default");

    assert!(error.to_string().contains("csharp_variable_declarations"));
}
