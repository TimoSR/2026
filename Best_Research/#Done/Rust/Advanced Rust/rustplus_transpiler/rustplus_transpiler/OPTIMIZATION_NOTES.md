# Rust Plus optimization notes

This version keeps the same language behavior, but reduces avoidable work in the transpiler pipeline.

## Applied optimizations

### 1. Parse class bodies once

Previously, class bodies were parsed in code generation and again inside feature validation. The transpiler now parses class bodies while building `SemanticContext` and reuses the parsed `ClassBody` everywhere.

Affected files:

- `src/transpiler.rs`
- `src/codegen.rs`
- `src/features/visibility.rs`

### 2. Remove method-declaration cloning during code generation

Code generation now groups borrowed `&MethodDeclaration` references instead of cloning `MethodDeclaration` values into temporary vectors.

Affected file:

- `src/codegen.rs`

### 3. Emit directly into the output buffer

`RustCodeGenerator` now supports `emit_item_into`, which writes directly into the final output `String`. This avoids creating a temporary `String` per top-level item.

Affected files:

- `src/codegen.rs`
- `src/transpiler.rs`

### 4. Preallocate output buffers

The transpiler now preallocates the generated output buffer based on source length. Generated file formatting also preallocates its output string.

Affected files:

- `src/transpiler.rs`
- `src/cargo_integration.rs`

### 5. Avoid checking `rustfmt --version` per generated file

Cargo integration now checks for `rustfmt` once per build-script run and reuses that result for every `.rp` file.

Affected files:

- `src/cargo_integration.rs`
- `src/project.rs`

### 6. Avoid scanning for `this` when not present

The `this` rewrite pass now returns immediately when the source does not contain `this`.

Affected file:

- `src/features/this_receiver.rs`

### 7. Remove heap allocation from the feature registry

The feature registry no longer allocates `Vec<Box<dyn LanguageFeature>>` just to validate features. It dispatches to zero-sized feature modules directly.

Affected file:

- `src/features/mod.rs`

### 8. Resolve generic base names semantically

Trait/composition classification now uses the base type name without generic arguments. For example, `class Repo : IRepository<Account>` resolves against the declared `IRepository` interface.

Affected files:

- `src/transpiler.rs`
- `src/codegen.rs`

## Still worth optimizing later

The scanner is still hand-written and byte-oriented. That is fine for the current ASCII-like syntax layer, but a later version should consider tokenizing once into a lightweight token stream and letting features consume tokens instead of scanning strings repeatedly.
