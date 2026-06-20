# Rust Plus Compiler Architecture

## Conclusion

Rust Plus now follows a TypeScript-inspired project compiler shape instead of only acting like a single-file text converter.

The compiler pipeline is:

```text
RustPlusProgram
  -> SourceFile collection
  -> parse top-level items
  -> bind SemanticContext
  -> validate feature modules
  -> emit Rust
  -> generate approximate .rp -> .rs line maps
  -> use Cargo build incremental cache
```

## Core model

### `RustPlusProgram`

`RustPlusProgram` is the project-level compiler object.

It owns:

```text
config
source_files
semantic context
diagnostics
timings
```

Use it when the compiler needs to understand more than one `.rp` file at once.

This matters for features such as:

```text
- interface object lowering
- constructor argument conversion
- Stack/Heap initializer validation
- class/interface lookup across files
- Cargo sibling-file generation
```

### `SourceFile`

A source file contains:

```text
path
source text
parsed top-level items
```

This is the Rust Plus equivalent of TypeScript's `SourceFile` concept.

### `SemanticContext`

`SemanticContext` remains the lightweight binder. It records:

```text
known classes
known interfaces / abstract classes
interface method names
class bodies
constructor parameters
default initializers
declaration kinds
```

The important design constraint is that Rust Plus does not become a full Rust type checker. `rustc` still owns real Rust semantic validation.

## Host abstraction

`src/host.rs` adds a file-system abstraction:

```text
RustPlusHost
RealFileSystemHost
MemoryHost
```

This makes compiler tests easier because future tests can run with in-memory files instead of touching disk.

## Structured diagnostics

`src/diagnostics.rs` adds structured compiler diagnostics:

```text
Diagnostic
DiagnosticSeverity
SourceSpan
```

Current errors can still use `anyhow`, but user-facing compiler errors can now move toward:

```text
error[RP0042]: cannot lower initializer
  --> src/main.rp:7:25
   = hint: use Account::Heap(...) for boxed construction
```

## Line maps

`src/line_map.rs` adds an approximate generated-line to source-line mapping.

Cargo sibling generation writes sidecar files such as:

```text
src/main.rs.rpmap
src/account.rs.rpmap
```

Generated `.rs` files also include a compact line-map comment.

This is intentionally lightweight. It is not a full source-map format yet.

## Incremental cache

`src/incremental.rs` adds Cargo-build cache support.

The cache lives under:

```text
target/rustplus/cache.toml
```

The cache fingerprints:

```text
source text
Rust Plus config / feature flags
generated output path
generated output
```

During `build.rs`, unchanged `.rp` files are skipped if their generated `.rs` twin already exists.

## Diagnostics CLI

`check` and `transpile` now support:

```bash
cargo run -- check examples/account.rp --diagnostics
cargo run -- check examples/account.rp --extended-diagnostics
```

Example output:

```text
Rust Plus project report
  Files:           1
  Known classes:   1
  Known traits:    0
  Timings:         parse=... bind=... validate=... emit=... total=...
```

## Files added

```text
src/program.rs
src/host.rs
src/diagnostics.rs
src/line_map.rs
src/incremental.rs
src/timing.rs
ARCHITECTURE.md
```

## Boundary

Rust Plus should remain a surface-syntax compiler.

It should validate Rust Plus features, but it should not replace:

```text
rustc
Cargo
rustfmt
Clippy
rust-analyzer
```

