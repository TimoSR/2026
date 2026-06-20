# Testing Style

Rust Plus tests are written as executable documentation.

The reader should be able to open a test file and learn how to use the language or the public library API without first understanding compiler internals.

## Rule

Every new test should follow this shape:

```rust
#[test]
fn guide_create_a_stack_value_with_stack_initializer()
{
    GuideExample::compile(r#"
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
"#)
    .should_generate("let article: Article = Article::new(\"Rust Plus\");")
    .should_not_generate("pub fn Stack(");
}
```

## Naming convention

Use guide-style names:

```text
guide_write_a_class_and_get_a_struct
guide_create_a_boxed_trait_object_with_heap_initializer
guide_attributes_are_preserved_on_generated_items
```

Avoid implementation-shaped names unless the test is purely internal:

```text
parse_top_level_items_handles_attributes
rewrites_heap_initializer_with_arguments
emits_outer_attributes_on_generated_class_fields_and_methods
```

The preferred test names describe what a Rust Plus user can do, not how the compiler implementation performs the work.

## Test vocabulary

Use human-facing variables:

```rust
let rust_plus_source = r#"..."#;
let generated_rust = compile_guide_snippet(rust_plus_source);
```

Avoid compiler-expert names in guide tests:

```rust
let ast = ...;
let token_stream = ...;
let semantic_context = ...;
```

Internal tests may still use lower-level names when they verify a private scanner/parser helper, but they should keep a short guide-oriented test name and a direct input/output example.

## Assertion style

Prefer assertions that show the Rust Plus snippet and generated Rust when they fail. This makes tests useful as debugging documentation.

```rust
assert!(
    generated_rust.contains(expected_snippet),
    "Guide source:\n{rust_plus_source}\n\nExpected generated Rust to contain:\n{expected_snippet}\n\nGenerated Rust:\n{generated_rust}",
);
```

## Rust generation facts documented by guide tests

Guide tests should document Rust Plus behavior and the Rust shape it lowers into. For trait/interface implementations, generated Rust must not put `pub` on methods inside `impl Trait for Type`, because Rust does not allow visibility modifiers there. Visibility belongs to the trait item itself.

```rust
interface IAccount
{
    fn balance(&self) -> i64;
}

class Account : IAccount
{
    pub fn balance(&self) -> i64
    {
        return 0;
    }
}
```

The generated trait implementation should contain:

```rust
impl IAccount for Account
{
    fn balance(&self) -> i64
    {
        return 0;
    }
}
```

Not:

```rust
impl IAccount for Account
{
    pub fn balance(&self) -> i64
    {
        return 0;
    }
}
```

## Test layers

Rust Plus has three test layers:

1. **Guide tests** in `tests/` document the public language and library API.
2. **Feature tests** near `src/features/*` document small language transformations.
3. **Infrastructure tests** near compiler/build modules document generated file markers, line maps, cache behavior, and reports.

Guide tests are the primary style. Internal tests are allowed, but they should still read like examples.
