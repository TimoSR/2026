pub mod ast;
pub mod cargo_integration;
pub mod codegen;
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

pub use config::{FeatureFlags, RustPlusConfig};
pub use program::{EmitOutput, RustPlusProgram, SourceFile};
pub use transpiler::Transpiler;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpiles_multiple_modular_features() {
        let source = r#"
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

        let output = Transpiler::new(source.to_string()).transpile().unwrap();

        assert!(output.contains("trait IAccount"));
        assert!(output.contains("trait IPayable"));
        assert!(output.contains("struct UserAccount"));
        assert!(output.contains("id: AccountId,"));
        assert!(output.contains("balance: Money,"));
        assert!(output.contains("user: User,"));
        assert!(output.contains("impl IAccount for UserAccount"));
        assert!(output.contains("impl IPayable for UserAccount"));
        assert!(output.contains("fn deposit(mut self, amount: Money)"));
        assert!(output.contains("self.balance = self.balance.add(amount);"));
    }

    #[test]
    fn disabling_this_receiver_rejects_this_keyword() {
        let mut config = RustPlusConfig::default();
        config.features.this_receiver = false;

        let source = r#"
class UserAccount
{
    balance: Money;

    pub fn balance(this) -> Money
    {
        return this.balance;
    }
}
"#;

        let error = Transpiler::with_config(source.to_string(), config).transpile().unwrap_err();
        assert!(error.to_string().contains("this_receiver"));
    }

    #[test]
    fn disabling_multiple_bases_rejects_multiple_base_list() {
        let mut config = RustPlusConfig::default();
        config.features.multiple_bases = false;

        let source = r#"
interface A { fn a(&self); }
interface B { fn b(&self); }
class C : A, B { fn a(&self) {} fn b(&self) {} }
"#;

        let error = Transpiler::with_config(source.to_string(), config).transpile().unwrap_err();
        assert!(error.to_string().contains("multiple_bases"));
    }

    #[test]
    fn rejects_csharp_style_interface_variable_declaration_by_default() {
        let source = r#"
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

        let error = Transpiler::new(source.to_string()).transpile().unwrap_err();

        assert!(error.to_string().contains("csharp_variable_declarations"));
    }

    #[test]
    fn transpiles_typed_interface_heap_initializer_in_main() {
        let source = r#"
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

        let output = Transpiler::new(source.to_string()).transpile().unwrap();

        assert!(output.contains(
            "let account: Box<dyn IAccount> = Box::new(Account::new(\"account-1\".to_string()));"
        ));
        assert!(output.contains("id: String,"));
        assert!(!output.contains("private id"));
    }

    #[test]
    fn transpiles_stack_heap_initializers() {
        let source = r#"
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

        let output = Transpiler::new(source.to_string()).transpile().unwrap();

        assert!(output.contains("let article: Article = Article::new(\"My own new!\", \"Lol!\", \"stuff\");"));
        assert!(output.contains(
            "let boxed_article: Box<Article> = Box::new(Article::new(\"My own new!\", \"Lol!\", \"stuff\"));"
        ));
        assert!(!output.contains("pub new("));
        assert!(!output.contains("pub fn Stack("));
        assert!(!output.contains("pub fn Heap("));
    }

    #[test]
    fn stack_heap_initializers_require_pub_fn_new_for_argument_construction() {
        let source = r#"
class Article
{
    headline: String;
}

fn main()
{
    let article = Article::Stack("headline");
}
"#;

        let error = Transpiler::new(source.to_string()).transpile().unwrap_err();

        assert!(error.to_string().contains("matching new(...) constructor"));
    }
}

#[cfg(test)]
mod program_architecture_tests {
    use super::*;

    #[test]
    fn rustplus_program_emits_project_report() {
        let source = r#"
class Account
{
    id: String;

    pub fn new(id: String) -> Self
    {
        return Self { id };
    }
}
"#;

        let mut program = RustPlusProgram::from_source(source.to_string(), RustPlusConfig::default()).unwrap();
        let output = program.emit_file(0, None).unwrap();
        let report = program.format_report(true);

        assert!(output.rust.contains("struct Account"));
        assert!(report.contains("Rust Plus project report"));
        assert!(report.contains("Known classes"));
    }
}
