# Rust Plus `.rp`

A tiny Rust-surface-syntax transpiler.

It lowers:

```text
interface X              -> trait X
abstract class X         -> trait X
class X                  -> struct X + impl X
class X : I              -> struct X + impl I for X
Type::Stack(args)        -> Type::new(args) with an explicit local type
Type::Heap(args)         -> Box::new(Type::new(args)) with an explicit Box local type
let x: Interface = Type::Heap(args) -> let x: Box<dyn Interface> = Box::new(Type::new(args))
Type::Stack()            -> Type::default()
Type::Heap()             -> Box::new(Type::default())
```

It deliberately keeps Rust ownership, lifetimes, modules, Cargo, rustfmt, Clippy, and normal Rust method receivers like `self`.

## Run

```bash
cargo run -- transpile examples/account.rp --out target/account.rs
rustfmt target/account.rs
rustc target/account.rs -o target/account
./target/account
```

## Example `.rp`

```rust
interface Drawable
{
    fn draw(&self);
}

class Sprite : Drawable
{
    x: f32;
    y: f32;

    fn new(x: f32, y: f32) -> Self
    {
        return Self { x, y };
    }

    fn draw(&self)
    {
        println!("Sprite at {}, {}", self.x, self.y);
    }
}
```

Generated Rust:

```rust
trait Drawable
{
    fn draw(&self);
}

struct Sprite
{
    x: f32,
    y: f32,
}

impl Sprite
{
    fn new(x: f32, y: f32) -> Self
    {
        return Self { x, y };
    }
}

impl Drawable for Sprite
{
    fn draw(&self)
    {
        println!("Sprite at {}, {}", self.x, self.y);
    }
}
```

## Stack / Heap initializers

Rust Plus treats `Stack` and `Heap` as initializer forms, not as generated Rust methods.

```rust
let article: Article = Article::Stack("My own new!", "Lol!", "stuff");
let boxed_article: Box<Article> = Article::Heap("My own new!", "Lol!", "stuff");
let account: IAccount = Account::Heap("account-1");
```

Generated Rust:

```rust
let article: Article = Article::new("My own new!", "Lol!", "stuff");
let boxed_article: Box<Article> = Box::new(Article::new("My own new!", "Lol!", "stuff"));
let account: Box<dyn IAccount> = Box::new(Account::new("account-1".to_string()));
```

Typed concrete construction should use Rust-style `let` syntax plus `Stack`:

```rust
let account: Account = Account::Stack("account-1");
```

Generated Rust:

```rust
let account: Account = Account::new("account-1".to_string());
```

Typed interface construction must use Rust-style `let` syntax plus `Heap`:

```rust
let account: IAccount = Account::Heap("account-1");
```

C#-style declarations are intentionally disabled by default and are not the project standard:

```rust
IAccount account = new Account("account-1");
```

The constructor declaration standard remains normal Rust-style syntax:

```rust
pub fn new(headline: &str, location: &str, artist: &str) -> Self
```

Rust Plus does not support `pub new(...)`.

Optional interop wrappers can be enabled with:

```toml
[features]
preserve_stack_heap_methods = true
```

By default, no non-snake-case Rust methods named `Stack` or `Heap` are generated.

## Attribute passthrough

Rust Plus supports one generic attribute passthrough feature instead of a separate feature for every Rust attribute. Outer Rust attributes before classes, interfaces, fields, and methods are preserved exactly and emitted above the generated Rust item.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub class Article
{
    #[doc = "Article headline displayed to users."]
    pub headline: String;

    #[inline]
    pub fn new(headline: &str) -> Self
    {
        return Self { headline: headline.to_string() };
    }
}
```

Generates the attributes on the generated `struct`, field, and method. The transpiler does not validate attribute semantics; `rustc` remains responsible for deciding whether an attribute is legal at its generated Rust location.

```toml
[features]
attribute_passthrough = true
```

Run the attribute example:

```bash
cargo run -- check examples/attributes_article.rp
```


## Common build error: Stack assigned to interface

This is invalid Rust Plus:

```rust
let account: IAccount = Account::Stack("account-1");
```

`Stack` creates a concrete owned value, so it must bind to the concrete type:

```rust
let account: Account = Account::Stack("account-1");
```

For interface/dynamic-dispatch binding, use `Heap`:

```rust
let account: IAccount = Account::Heap("account-1");
```

Generated Rust:

```rust
let account: Box<dyn IAccount> = Box::new(Account::new("account-1".to_string()));
```


## VS Code icons

The bundled VS Code extension now ships with the **Rust Plus File Icons** icon theme.

- Install the local extension from `tooling/vscode-rustplus`.
- In VS Code, run **Preferences: File Icon Theme** and choose **Rust Plus File Icons**.
- The workspace settings already prefer `workbench.iconTheme = "rustplus-file-icons"`.

The packaged icon files live under `tooling/vscode-rustplus/icons/`.

## TypeScript-inspired compiler architecture

Rust Plus now includes a project-level compiler object, `RustPlusProgram`, inspired by the useful parts of TypeScript's `Program` / `SourceFile` architecture.

Added pieces:

```text
src/program.rs        # project compiler object
src/host.rs           # file-system abstraction + memory host
src/diagnostics.rs    # structured diagnostics
src/line_map.rs       # approximate .rp -> .rs line maps
src/incremental.rs    # Cargo build cache
src/timing.rs         # parse/bind/validate/emit timings
ARCHITECTURE.md       # detailed architecture notes
```

Useful commands:

```bash
cargo run -- check examples/account.rp --diagnostics
cargo run -- check examples/account.rp --extended-diagnostics
```

Cargo-native builds now use an incremental cache under:

```text
target/rustplus/cache.toml
```

Generated sibling files also get `.rpmap` sidecar files, for example:

```text
src/main.rs.rpmap
src/account.rs.rpmap
```

See `ARCHITECTURE.md` for the full model.
