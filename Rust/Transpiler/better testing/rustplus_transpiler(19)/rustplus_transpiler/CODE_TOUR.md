# Rust Plus Code Tour

This file explains the codebase using product language, not compiler jargon.

## Mental model

Rust Plus is a small source-to-source layer on top of Rust.

```text
User writes .rp
    ↓
Rust Plus reads source documents
    ↓
Rust Plus collects project symbols
    ↓
Rust Plus validates enabled language features
    ↓
Rust Plus emits normal .rs files
    ↓
Cargo/rustc does the real Rust compilation
```

Rust Plus should not become a second Rust compiler. It only lowers readable Rust Plus syntax into ordinary Rust.

## Human-readable concepts

| Concept | Code type | Meaning |
|---|---|---|
| Rust Plus compiler | `RustPlusCompiler` | High-level API for compiling source text, one document, or a project. |
| Source document | `SourceDocument` / `SourceFile` | One `.rp` file. |
| Project symbols | `ProjectSymbols` | Known classes, interfaces, constructors, default initializers, and method ownership. |
| Feature pipeline | `FeaturePipeline` | Ordered validation for optional language features. |
| Code generator | `RustCodeGenerator` | Emits Rust text from parsed Rust Plus items. |
| Statement-level features | `apply_statement_level_features` | Rewrites expression sugar such as `Account::Heap(...)`. |
| Sibling file build | `SiblingFileBuild` | Cargo `build.rs` flow that turns `src/foo.rp` into `src/foo.rs`. |
| Generated Rust | `GeneratedRust` / `EmitOutput` | The `.rs` code plus source mapping metadata. |

## Where to start

Start with these files in this order:

1. `src/compiler.rs`
   - human-friendly public API
   - best file for understanding the intended workflow

2. `src/program.rs`
   - project-level pipeline
   - parse -> collect symbols -> validate features -> emit Rust

3. `src/transpiler.rs`
   - `ProjectSymbols`
   - project facts needed by features and codegen

4. `src/features/mod.rs`
   - `FeaturePipeline`
   - ordered list of language features

5. `src/codegen.rs`
   - class/interface lowering
   - statement-level sugar lowering

6. `src/cargo_integration.rs`
   - normal Cargo workflow
   - sibling `.rp` -> `.rs` generation

## Feature implementation rule

A language feature should answer these questions:

```text
1. What syntax does this feature recognize?
2. How does the feature validate disabled/enabled behavior?
3. Does the feature need project symbols?
4. Does the feature lower AST items or statement-level text?
5. What examples prove it works?
```

Do not hide feature order behind macros. The explicit `FeaturePipeline` is easier to read, debug, and disable.

## Naming rules

Prefer product names over compiler names:

```text
Use ProjectSymbols instead of SemanticContext in new code.
Use SourceDocument instead of tuple `(PathBuf, String)`.
Use GeneratedRust instead of raw `String` when returning compiled output.
Use FeaturePipeline instead of registry/factory terminology.
Use SiblingFileBuild instead of build-script plumbing.
```

The old names still exist as compatibility aliases where needed.

## Good module shape

A readable module should look like this:

```rust
pub struct FeatureName;

impl LanguageFeature for FeatureName
{
    fn name(&self) -> &'static str { ... }
    fn enabled(&self, flags: &FeatureFlags) -> bool { ... }
    fn validate(...) -> Result<()> { ... }
}

pub fn rewrite_feature_syntax(...) -> Result<String>
{
    // Small, named workflow steps.
}
```

Avoid one very large function that parses, validates, transforms, and emits at once.

## Current debt

The two largest files are still:

```text
src/features/stack_heap_initializers.rs
src/features/csharp_variable_declarations.rs
```

They work, but they should eventually be split into smaller modules:

```text
features/stack_heap_initializers/
├── mod.rs
├── syntax.rs
├── validation.rs
├── lowering.rs
└── diagnostics.rs
```

and:

```text
features/interface_objects/
├── mod.rs
├── typed_let.rs
├── heap_lowering.rs
└── diagnostics.rs
```

Do this after the syntax stabilizes so the split does not create churn.

## Tests are executable guide snippets

The preferred test files are in `tests/`:

```text
tests/guide_language_features.rs
tests/guide_project_compilation.rs
```

These tests are written for humans first. They show:

```text
Rust Plus input -> generated Rust output
```

Feature-level tests under `src/features/*` should use the same style even when testing a small private transformation helper.

See `TESTING.md` for the project convention.
